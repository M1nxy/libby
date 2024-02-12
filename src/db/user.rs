use sqlx::{mysql::MySqlQueryResult, query, query_as, FromRow, MySql, Transaction};

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct User {
  pub id: u8,
  pub name: String,
}

impl User {
  pub async fn fetch_one<'a>(tx: &mut Transaction<'a, MySql>, user_id: u8) -> Result<User, sqlx::Error> {
    query_as::<MySql, User>(
      r#"SELECT * FROM `user`
      WHERE `user_id`= ?"#,
    )
    .bind(user_id)
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn fetch_last<'a>(tx: &mut Transaction<'a, MySql>) -> Result<User, sqlx::Error> {
    query_as::<MySql, User>(
      r#"SELECT * FROM `user` 
      WHERE `id` = LAST_INSERT_ID();"#,
    )
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn create<'a>(tx: &mut Transaction<'a, MySql>, user_id: u8, user_name: String) -> Result<User, sqlx::Error> {
    query(
      r#"INSERT INTO `user` (`id`, `name`)
      VALUES (?, ?)"#,
    )
    .bind(user_id)
    .bind(user_name)
    .fetch_one(&mut **tx)
    .await?;

    User::fetch_last(tx).await
  }

  pub async fn update<'a>(tx: &mut Transaction<'a, MySql>, user_id: u8, user_name: String) -> Result<User, sqlx::Error> {
    // some logic here for partial user with merge fn when/if i add user prefs
    query_as::<MySql, User>(
      r#"UPDATE `user` 
      SET `name` = ?
      WHERE `user_id`= ?"#,
    )
    .bind(user_name)
    .bind(user_id)
    .fetch_one(&mut **tx)
    .await
  }

  pub async fn delete<'a>(tx: &mut Transaction<'a, MySql>, user_id: u8) -> Result<MySqlQueryResult, sqlx::Error> {
    query(
      r#"DELETE FROM `user` 
      WHERE `user_id`= ?"#,
    )
    .bind(user_id)
    .execute(&mut **tx)
    .await
  }
}
