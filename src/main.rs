use {db::Db, dotenv::dotenv, sqlx::Error, std::process::exit};

mod db;

#[tokio::main]
async fn main() -> Result<(), Error> {
  dotenv().ok();

  let conn_str = std::env::var("DATABASE_URL").expect("DATABASE URL NOT PRESENT IN ENVIRONMENT");

  let database = Db::new(&conn_str).await?;

  database.migrate_v1().await?;

  let test = db::authors::Author::fetch_all(&database.conn).await?;
  println!("all fetched before update:{:?}", test);

  let res = db::authors::Author::fetch_one(&database.conn, 1).await?;
  println!("fetched before update: {:?}", res);

  let test = db::authors::PartialAuthor {
    name: Some(String::from("Updated Name")),
    description: Some(String::from("Updated Desc")),
    birth: None,
  };

  let res = db::authors::Author::update(&database.conn, 1, test).await?;
  println!("returned from update: {:?}", res);

  let res = db::authors::Author::fetch_one(&database.conn, 1).await?;
  println!("fetched after update: {:?}", res);

  let del = db::authors::Author::delete(&database.conn, 1).await?;
  println!("deleted: {:?}", del);

  let abc = db::authors::Author::fetch_all(&database.conn).await?;
  println!("all fetched after delete:{:?}", abc);

  tokio::spawn(async move {
    tokio::signal::ctrl_c().await.unwrap();
    database.conn.close().await;
    exit(0);
  });

  loop {}
}
