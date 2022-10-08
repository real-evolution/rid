use crate::error::DataResult;

#[async_trait::async_trait]
pub trait Repo<Entity, Key> {
    async fn get(id: Key) -> DataResult<Entity>;
}
