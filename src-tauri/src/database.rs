// pub mod entities;
pub mod prelude;

// pub use entities::prelude::*;
// use migration::{Migrator, MigratorTrait};
use crate::error::Result;
use crate::APP_DATA_DIR;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::thread;

pub fn connect() -> Result<DatabaseConnection> {
  let handle = thread::spawn(move || {
    tauri::async_runtime::block_on(async {
      let path = APP_DATA_DIR.get().unwrap().join("RENAME_THIS.db");
      let url = format!("sqlite://{}?mode=rwc", path.to_str().unwrap());

      let options = ConnectOptions::new(url);
      let conn = Database::connect(options).await?;

      // Migrator::up(&conn, None).await?;

      Ok(conn)
    })
  });

  handle.join().unwrap()
}
