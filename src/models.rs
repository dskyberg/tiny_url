use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct UrlRequest {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TinyUrl {
    pub url: String,
    pub src_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TinyUrl {
    pub fn create(src_url: &str) -> Self {
        let now = Utc::now();
        let url = nanoid!(10);

        Self {
            url,
            src_url: src_url.to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}
