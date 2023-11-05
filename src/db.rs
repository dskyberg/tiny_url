use chrono::Utc;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{errors::Result, models::TinyUrl};

#[derive(Clone)]
pub struct DB {
    pub pool: Pool<Postgres>,
}

impl DB {
    pub async fn new() -> Self {
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

    pub async fn get_url(&self, url: &str) -> Result<TinyUrl> {
        let rec = sqlx::query!("SELECT * FROM urls WHERE url = $1", url)
            .fetch_one(&self.pool)
            .await?;

        let tiny = TinyUrl {
            url: rec.url,
            src_url: rec.src_url,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
        };

        log::info!("Fetched: {:?}", &tiny);
        Ok(tiny)
    }

    pub async fn put_url(&self, tiny: &TinyUrl) -> Result<TinyUrl> {
        let row =
            sqlx::query!("INSERT INTO urls ( url, src_url, created_at, updated_at ) VALUES ($1, $2, $3, $4) RETURNING * ",
                &tiny.url,
                &tiny.src_url,
                tiny.created_at,
                tiny.updated_at)
                .fetch_one(&self.pool)
                .await?;

        log::info!("Inserted: {:?}", row);
        Ok(tiny.clone())
    }

    pub async fn update_url(&self, tiny_url: &str, src_url: &str) -> Result<TinyUrl> {
        let now = Utc::now();
        let rec = sqlx::query!(
            "UPDATE urls SET src_url = $1, updated_at = $2 WHERE url = $3 RETURNING * ",
            src_url,
            now,
            tiny_url
        )
        .fetch_one(&self.pool)
        .await?;

        let tiny = TinyUrl {
            url: rec.url,
            src_url: rec.src_url,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
        };

        log::info!("Updated: {:?}", &tiny);
        Ok(tiny)
    }

    pub async fn delete_url(&self, url: &str) -> Result<()> {
        sqlx::query!("DELETE FROM urls WHERE url = $1", url)
            .execute(&self.pool)
            .await?;

        log::info!("Deleted: {}", url);
        Ok(())
    }
}
