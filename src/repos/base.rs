use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::error::Error;

pub trait IRepo<T, TDto> {
    fn new(db: Arc<Surreal<Client>>) -> Self;
    async fn create(&self, input: TDto) -> Result<Option<T>, Error>;
    async fn read(&self) -> Result<Vec<T>, Error>;
    async fn update(&self, id: String, input: TDto) -> Result<Option<T>, Error>;
    async fn delete(&self, id: String) -> Result<Option<T>, Error>;
    async fn get_by_id(&self, id: String) -> Result<Option<T>, Error>;
}