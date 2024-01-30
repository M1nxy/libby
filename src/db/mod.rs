use sqlx::{mysql::MySqlPoolOptions, Error, MySqlPool};

pub mod authors;

pub struct Db {
  pub conn: MySqlPool,
}

#[allow(dead_code)]
impl Db {
  pub async fn new(url: &str) -> Result<Db, Error> {
    let conn = MySqlPoolOptions::new()
      .max_connections(5)
      .connect(url)
      .await?;

    let db = Db { conn };
    db.migrate().await?;
    Ok(db)
  }

  pub async fn new_with_max(&self, url: &str, max: u32) -> Result<Db, Error> {
    let conn = MySqlPoolOptions::new()
      .max_connections(max)
      .connect(url)
      .await?;

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
      "
        CREATE TABLE IF NOT EXISTS `authors` (
          `id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
          `name` TEXT NOT NULL,
          `description` TEXT,
          `birth` DATE,
          `date_added` TIMESTAMP DEFAULT NOW(),
          `date_last_updated` TIMESTAMP ON UPDATE NOW()
        );
      "
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await
  }
}
