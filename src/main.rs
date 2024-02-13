use {db::Db, dotenv::dotenv, sqlx::Error, std::process::exit};

pub mod db;
pub mod test;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  dotenv().ok();

  let conn_str = std::env::var("DATABASE_URL").expect("DATABASE URL NOT PRESENT IN ENVIRONMENT");

  let database = Db::new(&conn_str).await?;

  tokio::spawn(async move {
    tokio::signal::ctrl_c().await.unwrap();
    database.conn.close().await;
    exit(0);
  });

  loop {}
}
