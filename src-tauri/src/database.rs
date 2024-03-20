// pub mod entities;
pub mod prelude;

// pub use entities::prelude::*;
// use migration::{Migrator, MigratorTrait};
use crate::error::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::thread;
use tauri::api::path::app_data_dir;
use tauri::Config;

pub fn connect(config: &Config) -> Result<DatabaseConnection> {
  let path = app_data_dir(config).unwrap().join("RENAME_THIS.db");
  let url = format!("sqlite://{}?mode=rwc", path.to_str().unwrap());

  let handle = thread::spawn(move || {
    tauri::async_runtime::block_on(async {
      let options = ConnectOptions::new(url);
      let conn = Database::connect(options).await?;

      // Migrator::up(&conn, None).await?;

      Ok(conn)
    })
  });

  handle.join().unwrap()
}
