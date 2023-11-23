use async_trait::async_trait;
use dyn_clone::DynClone;
use std::fmt::Debug;

use crate::{errors::Result, models::TinyUrl};

/// UrlRepository is a simple abstraction for the database implementation
/// Note:  Since a trait cannot directly rely on Clone, and this trait is
/// used in [crate::app_state::AppState], the dyn_clone crate is used.
#[async_trait]
pub trait UrlRepository: DynClone + Debug + Send {
    async fn new() -> Self
    where
        Self: Sized;
    async fn all(&self) -> Result<Vec<TinyUrl>>;
    async fn get(&self, id: &str) -> Result<TinyUrl>;
    async fn create(&self, student: &TinyUrl) -> Result<TinyUrl>;
    async fn update(&self, id: &str, src_url: &str) -> Result<TinyUrl>;
    async fn delete(&self, id: &str) -> Result<()>;
}

dyn_clone::clone_trait_object!(UrlRepository);
