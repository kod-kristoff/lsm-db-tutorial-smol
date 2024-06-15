use std::path::Path;

mod log;
mod memtable;
mod queryable;

use crate::Error;
pub use queryable::Queryable;

use self::{log::{Log}, memtable::Memtable};

pub struct Db {
    log: Log,
    memtable: Memtable,
}

impl Db {
    pub async fn new(db_dir: impl AsRef<Path>) -> Result<Db, Error> {
        if !db_dir.as_ref().exists() {
            smol::fs::create_dir_all(&db_dir).await?;
        }
        let log = Log::open(db_dir.as_ref().join("log.jsonl")).await?;
        let memtable = Memtable::hydrate(&log).await?;
        Ok(Db { log, memtable })
    }

    pub async fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        self.log.put(key, value).await?;
        self.memtable.put(key.into(),value.into() );

        Ok(())
    }

    pub async fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        // self.log.get(key).await
        Ok(self.memtable.get(key).await?)
    }
}
