use chrono::{DateTime, Utc};
use sqlx::{mysql::MySqlQueryResult, query, query_as, FromRow, MySql, Transaction};

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct Publisher {
  id: u16,
  name: String,
  description: String,
  city: Option<String>,
  pub date_added: Option<DateTime<Utc>>,
  pub date_last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct PartialPublisher {
  name: Option<String>,
  description: Option<String>,
  city: Option<String>,
}

impl Publisher {
  fn merge(mut self, partial: PartialPublisher) -> Self {
    if let Some(name) = partial.name {
      self.name = name;
    }
    if let Some(description) = partial.description {
      self.description = description;
    }
    self.city = partial.city;
    self
  }

  pub async fn fetch_one<'a>(tx: &mut Transaction<'a, MySql>, publisher_id: u16) -> Result<Publisher, sqlx::Error> {
    query_as::<MySql, Publisher>(
      r#"SELECT * FROM `publisher`
      WHERE `id`= ?"#,
    )
    .bind(publisher_id)
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn fetch_last<'a>(tx: &mut Transaction<'a, MySql>) -> Result<Publisher, sqlx::Error> {
    query_as::<MySql, Publisher>(
      r#"SELECT * FROM `publisher`
      WHERE `id` = LAST_INSERT_ID();"#,
    )
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn create<'a>(tx: &mut Transaction<'a, MySql>, name: String, description: String, city: Option<String>) -> Result<Publisher, sqlx::Error> {
    query(
      r#"INSERT INTO `publisher` (`name`, `description`, `city`)
      VALUES (?, ?, ?)"#,
    )
    .bind(name)
    .bind(description)
    .bind(city)
    .fetch_one(&mut **tx)
    .await?;

    Publisher::fetch_last(tx).await
  }

  pub async fn update<'a>(tx: &mut Transaction<'a, MySql>, id: u16, partial: PartialPublisher) -> Result<Publisher, sqlx::Error> {
    let old_publisher = Publisher::fetch_one(tx, id).await?;
    let updated_publisher = old_publisher.merge(partial);

    query_as::<MySql, Publisher>(
      r#"UPDATE `publisher`
      SET `name` = ?, `description` = ?, `city` = ?
      WHERE `id` = ?"#,
    )
    .bind(updated_publisher.name)
    .bind(updated_publisher.description)
    .bind(updated_publisher.city)
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn delete<'a>(tx: &mut Transaction<'a, MySql>, id: u16) -> Result<MySqlQueryResult, sqlx::Error> {
    query(
      r#"DELETE FROM `publisher`
      WHERE `id` = ?"#,
    )
    .bind(id)
    .execute(&mut **tx)
    .await
  }
}
