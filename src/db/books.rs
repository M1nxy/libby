use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Book {
  pub id: u64,
  pub isbn: u64, // TODO: Sanity check this is large enough
  pub name: String,
  pub description: Option<String>,
  pub num_pages: u16, // 65k. should be ok u32 (4b) seems wildly excessive
  pub date_added: Option<DateTime<Utc>>,
  pub date_last_updated: Option<DateTime<Utc>>,
}
