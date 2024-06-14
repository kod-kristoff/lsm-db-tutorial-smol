use nanoserde::{DeJson, SerJson};
use smol::{
    fs::{File, OpenOptions},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    stream::StreamExt,
};
use std::path::{Path, PathBuf};

use crate::Error;

pub struct Log {
    path: PathBuf,
    log: File,
}

impl Log {
    pub async fn open(path: impl AsRef<Path>) -> Result<Log, Error> {
        let log = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)
            .await?;
        Ok(Log {
            path: path.as_ref().to_path_buf(),
            log,
        })
    }
}

#[derive(nanoserde::DeJson, nanoserde::SerJson)]
struct Put {
    key: Vec<u8>,
    value: Vec<u8>,
}

impl Log {
    pub async fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        let put = Put {
            key: key.into(),
            value: value.into(),
        };

        let serialized = SerJson::serialize_json(&put);
        self.log.write_all(serialized.as_bytes()).await?;
        self.log.write_all(b"\n").await?;
        self.log.sync_all().await?;

        Ok(())
    }
}

#[async_trait::async_trait]
pub trait Queryable {
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error>;
}

#[async_trait::async_trait]
impl Queryable for Log {
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        let reader = File::open(&self.path).await?;
        let reader = BufReader::new(reader);
        let mut lines = reader.lines();
        let mut result = None;
        while let Some(line) = lines.next().await {
            let put = Put::deserialize_json(&line?)?;
            if put.key == key {
                result = Some(put.value);
            }
        }

        Ok(result)
    }
}
