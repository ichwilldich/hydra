use centaurus::db::init::Connection;

mod config;
mod invalid_jwt;
mod key;
mod user;

pub trait DBTrait {
  fn key(&self) -> key::KeyTable<'_>;
  fn user(&self) -> user::UserTable<'_>;
  fn invalid_jwt(&self) -> invalid_jwt::InvalidJwtTable<'_>;
  fn config(&self) -> config::ConfigTable<'_>;
}

impl DBTrait for Connection {
  fn key(&self) -> key::KeyTable<'_> {
    key::KeyTable::new(&self.0)
  }

  fn user(&self) -> user::UserTable<'_> {
    user::UserTable::new(&self.0)
  }

  fn invalid_jwt(&self) -> invalid_jwt::InvalidJwtTable<'_> {
    invalid_jwt::InvalidJwtTable::new(&self.0)
  }

  fn config(&self) -> config::ConfigTable<'_> {
    config::ConfigTable::new(&self.0)
  }
}

#[cfg(test)]
pub mod test {
  use migration::MigratorTrait;
  use sea_orm::Database;

  use super::*;

  pub async fn test_db() -> Connection {
    let conn = Database::connect("sqlite::memory:")
      .await
      .expect("Failed to connect to database");
    migration::Migrator::up(&conn, None)
      .await
      .expect("Failed to run database migrations");

    Connection(conn)
  }

  #[tokio::test]
  async fn test_connection() {
    let _ = test_db().await;
  }
}
