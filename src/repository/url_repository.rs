use async_trait::async_trait;

use crate::{errors::Result, models::TinyUrl};

#[async_trait]
pub trait UrlRepository: Clone {
    async fn all(&self) -> Result<Vec<TinyUrl>>;
    async fn get(&self, id: &str) -> Result<TinyUrl>;
    async fn create(&self, student: &TinyUrl) -> Result<TinyUrl>;
    async fn update(&self, id: &str, src_url: &str) -> Result<TinyUrl>;
    async fn delete(&self, id: &str) -> Result<()>;
}
