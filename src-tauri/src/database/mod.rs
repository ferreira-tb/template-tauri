// pub mod entities;
pub mod prelude;

// pub use entities::prelude::*;
// use migration::{Migrator, MigratorTrait};
use crate::prelude::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub fn connect<M, R>(app: &M) -> Result<DatabaseConnection>
where
  R: Runtime,
  M: Manager<R>,
{
  let resolver = app.path();
  let path = resolver.app_data_dir().unwrap().join("RENAME.db");
  let url = format!("sqlite://{}?mode=rwc", path.to_str().unwrap());

  let handle = thread::spawn(move || {
    async_runtime::block_on(async {
      let options = ConnectOptions::new(url);
      let conn = Database::connect(options).await?;

      // Migrator::up(&conn, None).await?;

      Ok(conn)
    })
  });

  handle.join().unwrap()
}
