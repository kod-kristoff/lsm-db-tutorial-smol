use crate::Error;

#[async_trait::async_trait]
pub trait Queryable {
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error>;
}
