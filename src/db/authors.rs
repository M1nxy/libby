use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{mysql::MySqlQueryResult, query, query_as, FromRow, MySql, Transaction};

pub type Authors = Vec<Author>;

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct Author {
  pub id: u64,
  pub name: String,
  pub description: Option<String>,
  pub birth: Option<NaiveDate>,
  pub date_added: Option<DateTime<Utc>>,
  pub date_last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct PartialAuthor {
  pub name: Option<String>,
  pub description: Option<String>,
  pub birth: Option<NaiveDate>,
}

impl Author {
  fn merge(mut self, partial: PartialAuthor) -> Self {
    if let Some(name) = partial.name {
      self.name = name;
    }
    if let Some(description) = partial.description {
      self.description = Some(description);
    }
    if let Some(birth) = partial.birth {
      self.birth = Some(birth);
    }
    self
  }

  pub async fn fetch_one<'a>(tx: &mut Transaction<'a, MySql>, author_id: u64) -> Result<Author, sqlx::Error> {
    query_as::<MySql, Author>(
      r#"SELECT * FROM `author`
      WHERE `id`= ?"#,
    )
    .bind(author_id)
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn fetch_all<'a>(tx: &mut Transaction<'a, MySql>) -> Result<Authors, sqlx::Error> {
    query_as::<MySql, Author>(r#"SELECT * FROM `author`"#).fetch_all(&mut **tx).await
  }

  pub async fn fetch_last<'a>(tx: &mut Transaction<'a, MySql>) -> Result<Author, sqlx::Error> {
    query_as::<MySql, Author>(
      r#"SELECT * FROM `author`
      WHERE `id` = LAST_INSERT_ID();"#,
    )
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn create<'a>(tx: &mut Transaction<'a, MySql>, partial: PartialAuthor) -> Result<Author, sqlx::Error> {
    query(
      r#"INSERT INTO `author` (`name`, `description`, `birth`)
      VALUES (?, ?, ?)"#,
    )
    .bind(partial.name)
    .bind(partial.description)
    .bind(partial.birth)
    .execute(&mut **tx)
    .await?;

    Author::fetch_last(tx).await
  }

  pub async fn update<'a>(tx: &mut Transaction<'a, MySql>, author_id: u64, partial: PartialAuthor) -> Result<Author, sqlx::Error> {
    let old_author = Author::fetch_one(tx, author_id).await?;
    let updated_author = old_author.merge(partial);

    query(
      r#"UPDATE `author`
      SET `name` = ?, `description` = ?, `birth` = ?
      WHERE `id` = ?"#,
    )
    .bind(updated_author.name)
    .bind(updated_author.description)
    .bind(updated_author.birth)
    .bind(author_id)
    .execute(&mut **tx)
    .await?;

    Author::fetch_one(tx, author_id).await
  }

  pub async fn delete<'a>(tx: &mut Transaction<'a, MySql>, author_id: u64) -> Result<MySqlQueryResult, sqlx::Error> {
    query(
      r#"DELETE FROM `author`
      WHERE `id` = ?"#,
    )
    .bind(author_id)
    .execute(&mut **tx)
    .await
  }
}
