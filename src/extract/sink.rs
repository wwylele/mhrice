use anyhow::Result;
use futures::StreamExt;
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
}

struct S3SinkInner {
    sender: futures::channel::mpsc::Sender<Message>,
    uploader: Option<JoinHandle<()>>,
}

impl S3SinkInner {
    fn init(bucket: String) -> Result<S3SinkInner> {
        let (sender, reciver) = futures::channel::mpsc::channel(10);
        let uploader = Some(spawn(move || {
            use tokio::runtime::Runtime;
            let rt = Runtime::new().unwrap();

            rt.block_on(async move {
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
                    let result = client
                        .list_objects_v2(request)
                        .await
                        .expect("Error listing files");

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

                        client.put_object(request)
                    })
                    .buffer_unordered(10)
                    .for_each(|result| {
                        if let Err(e) = result {
                            panic!("Failed to upload object: {e}")
                        }
                        std::future::ready(())
                    })
                    .await;

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

                    client.delete_objects(request).await.unwrap();
                }
            })
        }));

        Ok(S3SinkInner { sender, uploader })
    }
}

impl Drop for S3SinkInner {
    fn drop(&mut self) {
        use futures::sink::SinkExt;
        futures::executor::block_on(self.sender.send(Message::Finalize)).unwrap();
        std::mem::take(&mut self.uploader).unwrap().join().unwrap();
    }
}

pub struct S3Sink {
    path: String,
    inner: Arc<S3SinkInner>,
}

pub struct S3File {
    path: String,
    buffer: Vec<u8>,
    sender: futures::channel::mpsc::Sender<Message>,
}

enum Message {
    Request { name: String, data: Vec<u8> },
    Finalize,
}

impl S3Sink {
    pub fn init(bucket: String) -> Result<S3Sink> {
        let inner = Arc::new(S3SinkInner::init(bucket)?);
        Ok(S3Sink {
            inner,
            path: String::new(),
        })
    }
}

impl Sink for S3Sink {
    type File = S3File;

    fn create(&self, name: &str) -> Result<Self::File> {
        let path = self.path.clone() + name;
        let inner = self.inner.clone();
        Ok(S3File {
            path,
            buffer: vec![],
            sender: inner.sender.clone(),
        })
    }

    fn sub_sink(&self, name: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let path = self.path.clone() + name + "/";
        let inner = self.inner.clone();
        Ok(S3Sink { path, inner })
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
        futures::executor::block_on(self.sender.send(Message::Request {
            name: std::mem::take(&mut self.path),
            data: std::mem::take(&mut self.buffer),
        }))
        .unwrap()
    }
}
