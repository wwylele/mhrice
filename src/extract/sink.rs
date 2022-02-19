use anyhow::{anyhow, bail, Context, Result};
use futures::{StreamExt, TryStreamExt};
use once_cell::sync::OnceCell;
use rusoto_core::{ByteStream, Region};
use rusoto_s3::*;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub trait Sink: Sync {
    type File: Write;
    fn create(&self, name: &str) -> Result<Self::File>;
    fn sub_sink(&self, name: &str) -> Result<Self>
    where
        Self: Sized;

    fn create_html(&self, name: &str) -> Result<Self::File> {
        let mut file = self.create(name)?;
        file.write_all(b"<!DOCTYPE html>\n")?;
        Ok(file)
    }

    fn finalize(self) -> Result<()>;
}

pub struct DiskSink {
    root: PathBuf,
}

impl DiskSink {
    pub fn init(root: &Path) -> Result<Self> {
        let root = PathBuf::from(root);
        if root.exists() {
            fs::remove_dir_all(&root)?;
        }
        fs::create_dir(&root)?;
        Ok(DiskSink { root })
    }
}

impl Sink for DiskSink {
    type File = fs::File;

    fn create(&self, name: &str) -> Result<Self::File> {
        let path = self.root.join(name);
        let file = fs::File::create(path)?;
        Ok(file)
    }

    fn sub_sink(&self, name: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let path = self.root.join(name);
        fs::create_dir(&path)?;
        Ok(DiskSink { root: path })
    }

    fn finalize(self) -> Result<()> {
        Ok(())
    }
}

struct S3SinkInner {
    sender: futures::channel::mpsc::Sender<Message>,
    uploader: Option<JoinHandle<()>>,
}

impl S3SinkInner {
    fn init(bucket: String, error: Arc<OnceCell<anyhow::Error>>) -> Result<S3SinkInner> {
        let (sender, reciver) = futures::channel::mpsc::channel(10);
        let uploader = Some(spawn(move || {
            use tokio::runtime::Runtime;
            let rt = Runtime::new().unwrap();

            let result = rt.block_on(async move {
                let client = S3Client::new(Region::UsEast1);

                let mut existing_objects = HashSet::new();
                let mut continuation_token = None;
                loop {
                    eprintln!("Listing existing files in the bucket..");
                    let request = ListObjectsV2Request {
                        bucket: bucket.clone(),
                        continuation_token: continuation_token.take(),
                        delimiter: None,
                        encoding_type: None,
                        expected_bucket_owner: None,
                        fetch_owner: None,
                        max_keys: None,
                        prefix: None,
                        request_payer: None,
                        start_after: None,
                    };
                    let result = client.list_objects_v2(request).await?;

                    existing_objects
                        .extend(result.contents.into_iter().flatten().flat_map(|o| o.key));

                    if result.is_truncated.unwrap_or(false) {
                        continuation_token = result.next_continuation_token;
                    } else {
                        break;
                    }
                }
                eprintln!("Finished listing existing files.");

                reciver
                    .take_while(|message| std::future::ready(!matches!(message, Message::Finalize)))
                    .map(|message| {
                        let (name, data) = if let Message::Request { name, data } = message {
                            (name, data)
                        } else {
                            unreachable!()
                        };

                        existing_objects.remove(&name);
                        eprintln!("Uploading {name}...");

                        let mime = match name.split('.').last() {
                            Some("html") => "text/html",
                            Some("css") => "text/css",
                            Some("js") => "text/javascript",
                            Some("png") => "image/png",
                            _ => panic!("Unknown extension"),
                        };
                        let content_length = Some(i64::try_from(data.len()).unwrap());

                        let request = PutObjectRequest {
                            bucket: bucket.clone(),
                            key: name,
                            body: Some(ByteStream::from(data)),
                            content_length,
                            content_type: Some(mime.to_owned()),
                            ..PutObjectRequest::default()
                        };

                        let future = client.put_object(request);

                        async { future.await.map(|_| ()).context("Failed to upload object") }
                    })
                    .buffer_unordered(10)
                    .try_collect::<()>()
                    .await?;

                eprintln!("Finished uploading");

                let mut objects = existing_objects.into_iter();

                loop {
                    let batch: Vec<_> = objects
                        .by_ref()
                        .take(1000)
                        .map(|key| {
                            eprintln!("Deleting {key}...");
                            ObjectIdentifier {
                                key,
                                version_id: None,
                            }
                        })
                        .collect();
                    if batch.is_empty() {
                        break;
                    }
                    let request = DeleteObjectsRequest {
                        bucket: bucket.clone(),
                        bypass_governance_retention: None,
                        delete: Delete {
                            objects: batch,
                            quiet: Some(true),
                        },
                        expected_bucket_owner: None,
                        mfa: None,
                        request_payer: None,
                    };

                    client.delete_objects(request).await?;
                }

                Ok(())
            });

            if let Err(e) = result {
                let _ = error.set(e);
            }
        }));

        Ok(S3SinkInner { sender, uploader })
    }
}

impl S3SinkInner {
    fn finalize(&mut self) -> Result<()> {
        use futures::sink::SinkExt;
        let uploader = std::mem::take(&mut self.uploader);
        futures::executor::block_on(self.sender.send(Message::Finalize))?;
        uploader
            .unwrap()
            .join()
            .map_err(|_| anyhow!("uploader thread panic"))?;
        Ok(())
    }
}

impl Drop for S3SinkInner {
    fn drop(&mut self) {
        if self.uploader.is_some() {
            eprintln!("S3SinkInner dropped without finalize")
        }
    }
}

pub struct S3Sink {
    path: String,
    inner: Arc<S3SinkInner>,
    error: Arc<OnceCell<anyhow::Error>>,
}

pub struct S3File {
    path: String,
    buffer: Vec<u8>,
    sender: futures::channel::mpsc::Sender<Message>,
    error: Arc<OnceCell<anyhow::Error>>,
}

enum Message {
    Request { name: String, data: Vec<u8> },
    Finalize,
}

impl S3Sink {
    pub fn init(bucket: String) -> Result<S3Sink> {
        let error = Arc::new(OnceCell::new());
        let inner = Arc::new(S3SinkInner::init(bucket, error.clone())?);
        Ok(S3Sink {
            inner,
            path: String::new(),
            error,
        })
    }
}

impl Sink for S3Sink {
    type File = S3File;

    fn create(&self, name: &str) -> Result<Self::File> {
        if let Some(e) = self.error.get() {
            bail!("S3Sink detected error from previous operation: {e}");
        }
        let path = self.path.clone() + name;
        let inner = self.inner.clone();
        Ok(S3File {
            path,
            buffer: vec![],
            sender: inner.sender.clone(),
            error: self.error.clone(),
        })
    }

    fn sub_sink(&self, name: &str) -> Result<Self>
    where
        Self: Sized,
    {
        if let Some(e) = self.error.get() {
            bail!("S3Sink detected error from previous operation: {e}");
        }
        let path = self.path.clone() + name + "/";
        let inner = self.inner.clone();
        Ok(S3Sink {
            path,
            inner,
            error: self.error.clone(),
        })
    }

    fn finalize(self) -> Result<()> {
        Arc::try_unwrap(self.inner)
            .map_err(|e| {
                std::mem::forget(e);
                anyhow!("S3Sink finalized with sub sink still open")
            })?
            .finalize()?;

        let error = Arc::try_unwrap(self.error).map_err(|e| {
            std::mem::forget(e);
            anyhow!("S3Sink finalized with files still open")
        })?;
        if let Some(e) = error.get() {
            bail!("S3Sink detected error from previous operation: {e}");
        }
        Ok(())
    }
}

impl Write for S3File {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Drop for S3File {
    fn drop(&mut self) {
        use futures::sink::SinkExt;
        if let Err(e) = futures::executor::block_on(self.sender.send(Message::Request {
            name: std::mem::take(&mut self.path),
            data: std::mem::take(&mut self.buffer),
        })) {
            println!("Failed to send file because {e}");
            let _ = self.error.set(anyhow!("{e}"));
        }
    }
}
