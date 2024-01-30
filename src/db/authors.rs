use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{prelude::FromRow, query, query_as, MySql, MySqlPool};

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Author {
  pub id: u64,
  pub name: String,
  pub description: Option<String>,
  pub birth: Option<NaiveDate>,
  pub date_added: Option<DateTime<Utc>>,
  pub date_last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PartialAuthor {
  pub name: Option<String>,
  pub description: Option<String>,
  pub birth: Option<NaiveDate>,
}

#[allow(dead_code)]
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

  pub async fn fetch_one(conn: &MySqlPool, id: u64) -> Result<Author, sqlx::Error> {
    query_as::<MySql, Author>(r#"SELECT * FROM `authors` WHERE `id`= ?"#)
      .bind(id)
      .fetch_one(conn)
      .await
  }

  pub async fn fetch_all(conn: &MySqlPool) -> Result<Vec<Author>, sqlx::Error> {
    query_as::<MySql, Author>(r#"SELECT * FROM `authors`"#)
      .fetch_all(conn)
      .await
  }

  pub async fn create(conn: &MySqlPool, partial: PartialAuthor) -> Result<Author, sqlx::Error> {
    let mut tx = conn.begin().await?;

    query(
      r#"INSERT INTO `authors` (`name`, `description`, `birth`)
      VALUES (?, ?, ?)
      "#,
    )
    .bind(partial.name)
    .bind(partial.description)
    .bind(partial.birth)
    .execute(&mut *tx)
    .await?;

    let author: Author = query_as(r#"SELECT * FROM `authors` WHERE `id`= LAST_INSERT_ID()"#)
      .fetch_one(&mut *tx)
      .await?;

    tx.commit().await?;
    Ok(author)
  }

  pub async fn update(
    conn: &MySqlPool,
    id: u64,
    partial: PartialAuthor,
  ) -> Result<Author, sqlx::Error> {
    let mut tx = conn.begin().await?;

    let old_author = query_as::<MySql, Author>(r#"SELECT * FROM `authors` WHERE `id`= ?"#)
      .bind(id)
      .fetch_one(conn)
      .await?;

    let updated_author = old_author.merge(partial);

    query(
      r#"UPDATE `authors` 
      SET `name` = ?, `description` = ?, `birth` = ?
      WHERE `id` = ?
      "#,
    )
    .bind(updated_author.name)
    .bind(updated_author.description)
    .bind(updated_author.birth)
    .bind(id)
    .execute(&mut *tx)
    .await?;

    let author: Author = query_as(r#"SELECT * FROM `authors` WHERE `id` = ?"#)
      .bind(id)
      .fetch_one(&mut *tx)
      .await?;

    tx.commit().await?;
    Ok(author)
  }

  pub async fn delete(conn: &MySqlPool, id: u64) -> Result<(), sqlx::Error> {
    let mut tx = conn.begin().await?;

    query(r#"DELETE FROM `authors` WHERE `id` = ?"#)
      .bind(id)
      .execute(&mut *tx)
      .await?;

    tx.commit().await
  }
}
