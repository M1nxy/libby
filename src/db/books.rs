use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct Book {
  pub id: u64,
  pub isbn: String,
  pub name: String,
  pub description: Option<String>,
  pub language: Option<String>,
  pub nsfw: bool,
  pub num_pages: u16,
  pub image_formatted: bool,
  pub date_published: Option<DateTime<Utc>>,
  pub date_added: Option<DateTime<Utc>>,
  pub date_last_updated: Option<DateTime<Utc>>,
}
