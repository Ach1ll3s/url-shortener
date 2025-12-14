use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct UrlEntry {
    pub id: i64,
    pub code: String,
    pub original_url: String,
    pub created_at: String,
    pub clicks: i64,
}