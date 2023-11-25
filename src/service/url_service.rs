use async_trait::async_trait;
use dyn_clone::DynClone;
use std::fmt::Debug;

use super::error::Result;
use crate::model::{TinyUrl, UrlRequest};

#[async_trait]
pub trait UrlService: Sync + Send + DynClone + Debug {
    async fn all(&self) -> Result<Vec<TinyUrl>>;
    async fn get(&self, id: &str) -> Result<TinyUrl>;
    async fn create(&self, url: &UrlRequest) -> Result<TinyUrl>;
    async fn update(&self, id: &str, student: &UrlRequest) -> Result<TinyUrl>;
    async fn delete(&self, id: &str) -> Result<()>;
}

dyn_clone::clone_trait_object!(UrlService);
