use async_trait::async_trait;
use chrono::Utc;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use super::url_repository::UrlRepository;
use crate::{errors::Result, models::TinyUrl};

#[derive(Clone, Debug)]
pub struct PostgresUrlRepository {
    pub pool: Pool<Postgres>,
}

impl PostgresUrlRepository {}

#[async_trait]
impl UrlRepository for PostgresUrlRepository {
    async fn new() -> Self {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => {
                log::error!("DATABASE_URL is not established");
                std::process::exit(1);
            }
        };
        let result = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await;
        match result {
            Ok(pool) => Self { pool },
            Err(err) => {
                log::error!("Failed to create conneection pool: {}", err.to_string());
                std::process::exit(1);
            }
        }
    }

    async fn all(&self) -> Result<Vec<TinyUrl>> {
        let recs = sqlx::query_as!(TinyUrl, "SELECT * FROM urls")
            .fetch_all(&self.pool)
            .await?;
        Ok(recs)
    }

    async fn get(&self, url: &str) -> Result<TinyUrl> {
        let rec = sqlx::query_as!(TinyUrl, "SELECT * FROM urls WHERE url = $1", url)
            .fetch_one(&self.pool)
            .await?;

        log::info!("Fetched: {:?}", &rec);
        Ok(rec)
    }

    async fn create(&self, tiny: &TinyUrl) -> Result<TinyUrl> {
        let row =
            sqlx::query_as!(TinyUrl,"INSERT INTO urls ( url, src_url, created_at, updated_at ) VALUES ($1, $2, $3, $4) RETURNING * ",
                &tiny.url,
                &tiny.src_url,
                tiny.created_at,
                tiny.updated_at)
                .fetch_one(&self.pool)
                .await?;

        log::info!("Inserted: {:?}", row);
        Ok(row)
    }

    async fn update(&self, tiny_url: &str, src_url: &str) -> Result<TinyUrl> {
        let now = Utc::now();
        let rec = sqlx::query_as!(
            TinyUrl,
            "UPDATE urls SET src_url = $1, updated_at = $2 WHERE url = $3 RETURNING * ",
            src_url,
            now,
            tiny_url
        )
        .fetch_one(&self.pool)
        .await?;

        log::info!("Updated: {:?}", &rec);
        Ok(rec)
    }

    async fn delete(&self, url: &str) -> Result<()> {
        sqlx::query!("DELETE FROM urls WHERE url = $1", url)
            .execute(&self.pool)
            .await?;

        log::info!("Deleted: {}", url);
        Ok(())
    }
}
