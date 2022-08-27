use anyhow::{anyhow, Context, Result};
use futures::{StreamExt, TryStreamExt};
use md5::{Digest, Md5};
use rusoto_core::{ByteStream, Region};
use rusoto_s3::*;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

use super::gen_website::LANGUAGE_MAP;
use crate::msg::MsgEntry;

#[derive(Serialize, Clone)]
struct OutputTocEntry<'a, 'b> {
    title: &'a str,
    path: &'b str,
}

struct TocEntry {
    title: Vec<String>,
    path: String,
}

pub struct Toc {
    entries: Vec<TocEntry>,
}

impl Toc {
    pub fn new() -> Toc {
        Toc { entries: vec![] }
    }

    pub fn finalize(self, sink: &impl Sink) -> Result<()> {
        let languages = self
            .entries
            .iter()
            .map(|e| e.title.len())
            .max()
            .unwrap_or(0);
        let mut toc_by_language = vec![vec![]; languages];
        for entry in &self.entries {
            for (i, title) in entry.title.iter().enumerate() {
                let title = title.trim();
                if title.is_empty() {
                    continue;
                }

                toc_by_language[i].push(OutputTocEntry {
                    title,
                    path: &entry.path,
                });
            }
        }

        for (i, toc) in toc_by_language.into_iter().enumerate() {
            let language_code = if let Some(&Some((_, code))) = LANGUAGE_MAP.get(i) {
                code
            } else {
                continue;
            };
            serde_json::to_writer(sink.create(&format!("{language_code}.json"))?, &toc)?;
        }

        Ok(())
    }
}

pub struct TocSink<'a> {
    toc: &'a mut Toc,
    path: String,
    title: Vec<String>, // For each language
}

impl<'a> TocSink<'a> {
    pub fn add(&mut self, title: &MsgEntry) {
        if self.title.len() < title.content.len() {
            self.title.resize_with(title.content.len(), String::default);
        }

        // When adding multiple language, join them for each language
        for (i, t) in title.content.iter().enumerate() {
            let t = t.trim();
            if t.is_empty() {
                continue;
            }
            if self.title[i].is_empty() {
                self.title[i] = t.to_string();
            } else {
                self.title[i] += ", ";
                self.title[i] += t;
            }
        }
    }
}

impl<'a> Drop for TocSink<'a> {
    fn drop(&mut self) {
        self.toc.entries.push(TocEntry {
            title: std::mem::take(&mut self.title),
            path: std::mem::take(&mut self.path),
        });
    }
}

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

    fn create_html_with_toc<'toc>(
        &self,
        name: &str,
        toc: &'toc mut Toc,
    ) -> Result<(Self::File, TocSink<'toc>)> {
        let file = self.create_html(name)?;
        let toc_path = self.toc_path() + name;
        Ok((
            file,
            TocSink {
                toc,
                path: toc_path,
                title: vec![],
            },
        ))
    }

    fn finalize(self) -> Result<()>;
    fn toc_path(&self) -> String;
}

pub struct NullSink;

impl Sink for NullSink {
    type File = std::io::Sink;

    fn create(&self, _name: &str) -> Result<Self::File> {
        Ok(std::io::sink())
    }

    fn sub_sink(&self, _name: &str) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(NullSink)
    }

    fn finalize(self) -> Result<()> {
        Ok(())
    }

    fn toc_path(&self) -> String {
        "".to_string()
    }
}

pub struct DiskSink {
    root: PathBuf,
    toc_path: String,
}

impl DiskSink {
    pub fn init(root: &Path) -> Result<Self> {
        let root = PathBuf::from(root);
        let toc_path = "/".to_string();
        if root.exists() {
            eprintln!(
                "Warning: output folder {} already exists",
                root.as_os_str().to_string_lossy()
            )
        } else {
            fs::create_dir(&root)?;
        }
        Ok(DiskSink { root, toc_path })
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
        let toc_path = self.toc_path.clone() + name + "/";
        fs::create_dir(&path)?;
        Ok(DiskSink {
            root: path,
            toc_path,
        })
    }

    fn finalize(self) -> Result<()> {
        Ok(())
    }

    fn toc_path(&self) -> String {
        self.toc_path.clone()
    }
}

struct S3SinkInner {
    sender: futures::channel::mpsc::Sender<Message>,
    uploader: Option<JoinHandle<()>>,
}

impl S3SinkInner {
    fn init(bucket: String, error: Arc<Mutex<Option<anyhow::Error>>>) -> Result<S3SinkInner> {
        let (sender, reciver) = futures::channel::mpsc::channel(10);
        let uploader = Some(spawn(move || {
            use tokio::runtime::Runtime;
            let rt = Runtime::new().unwrap();

            let result = rt.block_on(async move {
                let client = S3Client::new(Region::UsEast1);

                let mut existing_objects = HashMap::new();
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

                    existing_objects.extend(
                        result
                            .contents
                            .into_iter()
                            .flatten()
                            .flat_map(|o| o.key.map(|key| (key, o.e_tag))),
                    );

                    if result.is_truncated.unwrap_or(false) {
                        continuation_token = result.next_continuation_token;
                    } else {
                        break;
                    }
                }
                eprintln!("Finished listing existing files.");

                reciver
                    .take_while(|message| std::future::ready(!matches!(message, Message::Finalize)))
                    .filter_map(|message| {
                        let (name, data) = if let Message::Request { name, data } = message {
                            (name, data)
                        } else {
                            unreachable!()
                        };

                        if let Some(etag) = existing_objects.remove(&name).flatten() {
                            let md5: [u8; 16] = Md5::digest(&data).try_into().unwrap();
                            let md5_tag: String = md5
                                .into_iter()
                                .map(|b| format!("{b:02x}"))
                                .fold("\"".to_owned(), |a, b| a + &b)
                                + "\"";
                            let etag = etag.to_ascii_lowercase();
                            if md5_tag == etag {
                                eprintln!("Skipped unchanged {name}");
                                return std::future::ready(None);
                            }
                        }
                        std::future::ready(Some((name, data)))
                    })
                    .map(|(name, data)| {
                        eprintln!("Uploading {name}...");

                        let mime = match name.split('.').last() {
                            Some("html") => "text/html",
                            Some("css") => "text/css",
                            Some("js") => "text/javascript",
                            Some("png") => "image/png",
                            Some("json") => "application/json",
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

                let mut objects = existing_objects.into_keys();

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
                error.lock().unwrap().get_or_insert(e);
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
    error: Arc<Mutex<Option<anyhow::Error>>>,
}

pub struct S3File {
    path: String,
    buffer: Vec<u8>,
    sender: futures::channel::mpsc::Sender<Message>,
    error: Arc<Mutex<Option<anyhow::Error>>>,
}

enum Message {
    Request { name: String, data: Vec<u8> },
    Finalize,
}

impl S3Sink {
    pub fn init(bucket: String) -> Result<S3Sink> {
        let error = Arc::new(Mutex::new(None));
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
        if let Some(e) = self.error.lock().unwrap().take() {
            return Err(e.context("S3Sink detected error from previous operation"));
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
        if let Some(e) = self.error.lock().unwrap().take() {
            return Err(e.context("S3Sink detected error from previous operation"));
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
        if let Some(e) = error.lock().unwrap().take() {
            return Err(e.context("S3Sink detected error from previous operation"));
        }
        Ok(())
    }

    fn toc_path(&self) -> String {
        "/".to_string() + &self.path
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
            eprintln!("Failed to send file because {e}");
            self.error.lock().unwrap().get_or_insert(anyhow!("{e}"));
        }
    }
}
