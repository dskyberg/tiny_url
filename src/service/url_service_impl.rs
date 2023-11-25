use async_trait::async_trait;
use dyn_clone::DynClone;
use std::fmt::Debug;
use validator::Validate;

use super::{Result, ServiceError, UrlService};
use crate::{
    model::{TinyUrl, UrlRequest},
    repository::UrlRepository,
};

#[derive(Clone, Debug)]
pub struct UrlServiceImpl<R: UrlRepository + Send + Sync> {
    repository: R,
}
impl<R: UrlRepository + Send + Sync + DynClone + Debug> UrlServiceImpl<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: UrlRepository + Send + Sync + std::clone::Clone + Debug> UrlService for UrlServiceImpl<R> {
    async fn all(&self) -> Result<Vec<TinyUrl>> {
        self.repository
            .all()
            .await
            .map_err(ServiceError::RepositoryError)
    }

    async fn get(&self, id: &str) -> Result<TinyUrl> {
        self.repository
            .get(id)
            .await
            .map_err(ServiceError::RepositoryError)
    }

    async fn create(&self, req: &UrlRequest) -> Result<TinyUrl> {
        req.validate().map_err(ServiceError::ValidationError)?;

        let tiny = TinyUrl::create(&req.url);
        self.repository
            .create(&tiny)
            .await
            .map_err(ServiceError::RepositoryError)
    }

    async fn update(&self, id: &str, req: &UrlRequest) -> Result<TinyUrl> {
        req.validate().map_err(ServiceError::ValidationError)?;

        self.repository
            .update(id, &req.url)
            .await
            .map_err(ServiceError::RepositoryError)
    }
    async fn delete(&self, id: &str) -> Result<()> {
        self.repository
            .delete(id)
            .await
            .map_err(ServiceError::RepositoryError)
    }
}
