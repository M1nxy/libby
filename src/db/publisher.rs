use chrono::{DateTime, Utc};
use sqlx::{query_as, FromRow, MySql, Transaction};

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct Publisher {
  id: u16,
  name: String,
  description: String,
  city: String,
  pub date_added: Option<DateTime<Utc>>,
  pub date_last_updated: Option<DateTime<Utc>>,
}

impl Publisher {
  pub async fn fetch_one<'a>(tx: &mut Transaction<'a, MySql>, publisher_id: u16) -> Result<Publisher, sqlx::Error> {
    query_as::<MySql, Publisher>(
      r#"SELECT * FROM `publisher`
      WHERE `id`= ?"#,
    )
    .bind(publisher_id)
    .fetch_one(&mut **tx)
    .await
  }
}
