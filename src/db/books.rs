use chrono::{DateTime, Utc};
use sqlx::{mysql::MySqlQueryResult, query, query_as, FromRow, MySql, Transaction};

use super::{authors::Authors, publisher::Publisher};

type Books = Vec<Book>;

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
  pub publisher_id: Option<u16>,
  pub date_published: Option<DateTime<Utc>>,
  pub date_added: Option<DateTime<Utc>>,
  pub date_last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct PartialBook {
  pub isbn: Option<String>,
  pub name: Option<String>,
  pub description: Option<String>,
  pub language: Option<String>,
  pub nsfw: Option<bool>,
  pub num_pages: Option<u16>,
  pub image_formatted: Option<bool>,
  pub publisher_id: Option<u16>,
  pub date_published: Option<DateTime<Utc>>,
}

impl Book {
  fn merge(mut self, partial: PartialBook) -> Self {
    if let Some(isbn) = partial.isbn {
      self.isbn = isbn;
    }
    if let Some(name) = partial.name {
      self.name = name;
    }
    self.description = partial.description;
    self.language = partial.language;
    if let Some(nsfw) = partial.nsfw {
      self.nsfw = nsfw;
    }
    if let Some(num_pages) = partial.num_pages {
      self.num_pages = num_pages;
    }
    if let Some(image_formatted) = partial.image_formatted {
      self.image_formatted = image_formatted;
    }
    self.publisher_id = partial.publisher_id;
    self.date_published = partial.date_published;
    self
  }

  pub fn fetch_authors() -> Result<Authors, sqlx::Error> {
    todo!("Unimplemented"); // TODO:
  }

  pub async fn fetch_publisher<'a>(&self, tx: &mut Transaction<'a, MySql>) -> Result<Publisher, sqlx::Error> {
    query_as::<MySql, Publisher>(
      r#"SELECT * FROM `book`
      WHERE `publisher_id`= ?"#,
    )
    .bind(self.publisher_id)
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn fetch_books_by_publisher<'a>(tx: &mut Transaction<'a, MySql>, publisher_id: u16) -> Result<Books, sqlx::Error> {
    query_as::<MySql, Book>(
      r#"SELECT * FROM `book`
      WHERE `publisher_id`= ?"#,
    )
    .bind(publisher_id)
    .fetch_all(&mut **tx)
    .await
  }

  pub async fn fetch_books_by_author<'a>(tx: &mut Transaction<'a, MySql>, author_id: u64) -> Result<Book, sqlx::Error> {
    todo!("Unimplemented"); // TODO:
  }

  pub async fn fetch_one<'a>(tx: &mut Transaction<'a, MySql>, book_id: u64) -> Result<Book, sqlx::Error> {
    query_as::<MySql, Book>(
      r#"SELECT * FROM `book`
      WHERE `id`= ?"#,
    )
    .bind(book_id)
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn fetch_all<'a>(tx: &mut Transaction<'a, MySql>) -> Result<Books, sqlx::Error> {
    query_as::<MySql, Book>(r#"SELECT * FROM `book`"#).fetch_all(&mut **tx).await
  }

  pub async fn fetch_last<'a>(tx: &mut Transaction<'a, MySql>) -> Result<Book, sqlx::Error> {
    query_as::<MySql, Book>(
      r#"SELECT * FROM `book`
      WHERE `id` = LAST_INSERT_ID();"#,
    )
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn create<'a>(
    tx: &mut Transaction<'a, MySql>,
    isbn: String,
    name: String,
    description: String,
    language: String,
    nsfw: bool,
    num_pages: u16,
    image_formatted: bool,
    publisher_id: u16,
    date_published: DateTime<Utc>,
  ) -> Result<Book, sqlx::Error> {
    query(
      r#"INSERT INTO `book` (`isbn`, `name`, `description`, `language`, `nsfw`, `num_pages`, `image_formatted`, `publisher_id`, `date_published`)
      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(isbn)
    .bind(name)
    .bind(description)
    .bind(language)
    .bind(nsfw)
    .bind(num_pages)
    .bind(image_formatted)
    .bind(publisher_id)
    .bind(date_published)
    .fetch_one(&mut **tx)
    .await?;

    Book::fetch_last(tx).await
  }

  pub async fn update<'a>(tx: &mut Transaction<'a, MySql>, book_id: u64, partial: PartialBook) -> Result<Book, sqlx::Error> {
    let old_book = Book::fetch_one(tx, book_id).await?;
    let updated_book = old_book.merge(partial);

    query(
      r#"UPDATE `book`
      SET `isbn` = ?, `name` = ?, `description`= ?, `language`= ?, `nsfw`= ?, `num_pages`= ?, `image_formatted`= ?, `publisher_id`= ?, `date_published`= ?
      WHERE `id` = ?"#,
    )
    .bind(updated_book.isbn)
    .bind(updated_book.name)
    .bind(updated_book.description)
    .bind(updated_book.language)
    .bind(updated_book.nsfw)
    .bind(updated_book.num_pages)
    .bind(updated_book.image_formatted)
    .bind(updated_book.publisher_id)
    .bind(updated_book.date_published)
    .bind(book_id)
    .execute(&mut **tx)
    .await?;

    Book::fetch_one(tx, book_id).await
  }

  pub async fn delete<'a>(tx: &mut Transaction<'a, MySql>, book_id: u64) -> Result<MySqlQueryResult, sqlx::Error> {
    query(
      r#"DELETE FROM `book`
      WHERE `id`= ?"#,
    )
    .bind(book_id)
    .execute(&mut **tx)
    .await
  }
}
