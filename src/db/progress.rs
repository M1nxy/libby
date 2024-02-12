use chrono::{DateTime, Utc};
use sqlx::{mysql::MySqlQueryResult, query, query_as, FromRow, MySql, Transaction};

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct Progress {
  pub id: u64,
  pub user_id: u8,
  pub book_id: u64,
  pub current_page: u16,
  pub date_added: Option<DateTime<Utc>>,
  pub date_last_updated: Option<DateTime<Utc>>,
}

impl Progress {
  pub async fn fetch_one<'a>(tx: &mut Transaction<'a, MySql>, user_id: u8, book_id: u64) -> Result<Progress, sqlx::Error> {
    query_as::<MySql, Progress>(
      r#"SELECT * FROM `progress`
      WHERE `user_id`= ? AND `book_id` = ?"#,
    )
    .bind(user_id)
    .bind(book_id)
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn fetch_last<'a>(tx: &mut Transaction<'a, MySql>) -> Result<Progress, sqlx::Error> {
    query_as::<MySql, Progress>(
      r#"SELECT * FROM `progress`
      WHERE `id` = LAST_INSERT_ID();"#,
    )
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn create<'a>(tx: &mut Transaction<'a, MySql>, user_id: u8, book_id: u64, current_page: u16) -> Result<Progress, sqlx::Error> {
    query(
      r#"INSERT INTO `progress` (`user_id`, `book_id`, `current_page`)
      VALUES (?, ?, ?)"#,
    )
    .bind(current_page)
    .bind(user_id)
    .bind(book_id)
    .fetch_one(&mut **tx)
    .await?;

    Progress::fetch_last(tx).await
  }

  pub async fn update<'a>(tx: &mut Transaction<'a, MySql>, user_id: u8, book_id: u64, current_page: u16) -> Result<Progress, sqlx::Error> {
    query_as::<MySql, Progress>(
      r#"UPDATE `progress`
      SET `current_page` = ?
      WHERE `user_id`= ? AND `book_id` = ?"#,
    )
    .bind(current_page)
    .bind(user_id)
    .bind(book_id)
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn delete<'a>(tx: &mut Transaction<'a, MySql>, user_id: u8, book_id: u64) -> Result<MySqlQueryResult, sqlx::Error> {
    query(
      r#"DELETE FROM `progress`
      WHERE `user_id`= ? AND `book_id` = ?"#,
    )
    .bind(user_id)
    .bind(book_id)
    .execute(&mut **tx)
    .await
  }
}
