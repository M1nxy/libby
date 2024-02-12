use sqlx::{mysql::MySqlPoolOptions, query, Error, MySqlPool};

pub mod authors;
pub mod books;
pub mod progress;
pub mod user;

pub struct Db {
  pub conn: MySqlPool,
}

#[allow(dead_code)]
impl Db {
  pub async fn new(url: &str) -> Result<Db, Error> {
    let conn = MySqlPoolOptions::new().max_connections(5).connect(url).await?;

    let db = Db { conn };
    db.migrate().await?;
    Ok(db)
  }

  pub async fn new_with_max(&self, url: &str, max: u32) -> Result<Db, Error> {
    let conn = MySqlPoolOptions::new().max_connections(max).connect(url).await?;

    let db = Db { conn };
    db.migrate().await?;
    Ok(db)
  }

  pub async fn migrate(&self) -> Result<(), Error> {
    self.migrate_v1().await
  }

  pub async fn migrate_v1(&self) -> Result<(), Error> {
    let mut tx = self.conn.begin().await?;

    sqlx::query!(
      r#"
        CREATE TABLE IF NOT EXISTS `author` (
          `id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
          `name` TEXT NOT NULL,
          `description` TEXT,
          `birth` DATE,
          `date_added` TIMESTAMP DEFAULT NOW(),
          `date_last_updated` TIMESTAMP ON UPDATE NOW()
        );
      "#
    )
    .execute(&mut *tx)
    .await?;

    query!(
      r#"
        CREATE TABLE IF NOT EXISTS `book` (
          `id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
          `isbn` TEXT NOT NULL,
          `name` TEXT NOT NULL,
          `description` TEXT,
          `language` TEXT,
          `nsfw` BOOL,
          `num_pages` SMALLINT UNSIGNED NOT NULL,
          `image_formatted` BOOL,
          `date_published` TIMESTAMP,
          `date_added` TIMESTAMP DEFAULT NOW(),
          `date_last_updated` TIMESTAMP ON UPDATE NOW()
        );
      "#
    )
    .execute(&mut *tx)
    .await?;

    query!(
      r#"
        CREATE TABLE IF NOT EXISTS `user` (
          `id` TINYINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
          `name` TEXT,
          `date_added` TIMESTAMP DEFAULT NOW(),
          `date_last_updated` TIMESTAMP ON UPDATE NOW()
        );
      "#
    )
    .execute(&mut *tx)
    .await?;

    query!(
      r#"
        CREATE TABLE IF NOT EXISTS `progress` (
          `id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
          `user_id` TINYINT UNSIGNED,
          `book_id` BIGINT UNSIGNED,
          `current_page` SMALLINT UNSIGNED,
          `date_added` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
          `date_last_updated` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
          CONSTRAINT fk_user_id FOREIGN KEY (`user_id`) REFERENCES `user`(`id`),
          CONSTRAINT fk_book_id FOREIGN KEY (`book_id`) REFERENCES `book`(`id`)
        );
      "#,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await
  }
}
