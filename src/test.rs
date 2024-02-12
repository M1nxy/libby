#[cfg(test)]
use crate::db::authors;
use crate::Db;
use dotenv::dotenv;
use sqlx::{MySql, Transaction};

#[allow(dead_code)]
async fn create_tx() -> Transaction<'static, MySql> {
  dotenv().ok();
  let conn_str = std::env::var("DATABASE_URL").expect("DATABASE URL NOT PRESENT IN ENVIRONMENT");
  let database = Db::new(&conn_str).await.expect("Failed to connect to DB");
  database.conn.begin().await.expect("Failed to create transaction")
}

#[tokio::test]
async fn authors_create() -> Result<(), sqlx::Error> {
  let mut tx = create_tx().await;

  let first_author = authors::PartialAuthor {
    name: Some(String::from("TEST AUTHOR")),
    description: Some(String::from("TEST DESCRIPTION")),
    birth: chrono::NaiveDate::from_ymd_opt(0000, 12, 12),
  };

  let second_author = authors::PartialAuthor {
    name: Some(String::from("TEST AUTHOR 2")),
    description: Some(String::from("TEST DESCRIPTION 2")),
    birth: chrono::NaiveDate::from_ymd_opt(0000, 12, 12),
  };

  // Create author 1 and check that it exists
  authors::Author::create(&mut tx, first_author.clone()).await?;
  let first_author_query = authors::Author::fetch_last(&mut tx).await?;
  assert_eq!(first_author.name, Some(first_author_query.name));

  // Create author 2 and check that it exists
  authors::Author::create(&mut tx, second_author.clone()).await?;
  let second_author_query = authors::Author::fetch_last(&mut tx).await?;
  assert_eq!(second_author.name, Some(second_author_query.name));

  Ok(())
}

#[tokio::test]
async fn authors_update() -> Result<(), sqlx::Error> {
  let mut tx = create_tx().await;

  let author_initial = authors::PartialAuthor {
    name: Some(String::from("TEST AUTHOR")),
    description: Some(String::from("TEST DESCRIPTION")),
    birth: chrono::NaiveDate::from_ymd_opt(0000, 12, 12),
  };

  let author_changes = authors::PartialAuthor {
    name: Some(String::from("Updated Name")),
    description: Some(String::from("Updated Desc")),
    birth: None,
  };

  // Create author, update some fields and compare a / b.
  authors::Author::create(&mut tx, author_initial.clone()).await?;
  let author_before = authors::Author::fetch_last(&mut tx).await?;
  let author_after = authors::Author::update(&mut tx, author_before.id, author_changes).await?;
  assert_ne!(author_before, author_after);

  Ok(())
}

#[tokio::test]
async fn authors_delete() -> Result<(), sqlx::Error> {
  let mut tx = create_tx().await;

  let author = authors::PartialAuthor {
    name: Some(String::from("TEST AUTHOR")),
    description: Some(String::from("TEST DESCRIPTION")),
    birth: chrono::NaiveDate::from_ymd_opt(0000, 12, 12),
  };

  // Create author and verify
  authors::Author::create(&mut tx, author.clone()).await?;
  let author_created = authors::Author::fetch_last(&mut tx).await?;
  assert_eq!(author.name, Some(author_created.name.clone()));

  // Delete author and verify
  authors::Author::delete(&mut tx, author_created.id.clone()).await?;
  let result = authors::Author::fetch_one(&mut tx, author_created.id).await;
  assert_eq!(result.is_err(), true);

  Ok(())
}
