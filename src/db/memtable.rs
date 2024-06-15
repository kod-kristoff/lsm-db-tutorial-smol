use std::collections::BTreeMap;

use nanoserde::DeJson;
use smol::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
    stream::StreamExt,
};

use super::{
    log::{Log, Put},
    Queryable,
};

#[derive(Debug, Clone, Default)]
pub struct Memtable {
    data: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Memtable {
    pub async fn hydrate(log: &Log) -> Result<Memtable, crate::Error> {
        let mut data = BTreeMap::new();
        let reader = File::open(&log.path).await?;
        let reader = BufReader::new(reader);
        let mut lines = reader.lines();
        while let Some(line) = lines.next().await {
            let put = Put::deserialize_json(&line?)?;
            data.insert(put.key, put.value);
        }
        Ok(Memtable { data })
    }
}
impl Memtable {
    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.data.insert(key, value);
    }
}

#[async_trait::async_trait]
impl Queryable for Memtable {
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, crate::Error> {
        Ok(self.data.get(key).cloned())
    }
}
