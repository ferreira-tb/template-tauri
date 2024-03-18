// pub mod entities;
pub mod prelude;

// pub use entities::prelude::*;
use sea_orm::prelude::DatabaseConnection;
use sea_orm::Database;

pub async fn connect() -> DatabaseConnection {
  const DATABASE_URL: &str = "sqlite::memory:";
  Database::connect(DATABASE_URL).await.unwrap()
}
