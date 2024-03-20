// pub mod entities;
pub mod prelude;

// pub use entities::prelude::*;
// use migration::{Migrator, MigratorTrait};
use crate::error::Result;
use sea_orm::prelude::DatabaseConnection;
use sea_orm::Database;
use std::sync::Arc;
use tauri::api::path::app_data_dir;
use tauri::Config;

pub async fn connect(config: Arc<Config>) -> Result<DatabaseConnection> {
  let path = app_data_dir(&config).unwrap().join("RENAME_THIS.db");
  let url = format!("sqlite://{}?mode=rwc", path.to_str().unwrap());
  let conn = Database::connect(&url).await?;

  // Migrator::up(&conn, None).await?;

  Ok(conn)
}
