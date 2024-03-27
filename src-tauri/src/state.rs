use sea_orm::DatabaseConnection;

pub mod prelude {
  pub use super::{AppState, State};
}

pub struct AppState {
  pub database: DatabaseConnection,
}

pub type State<'a> = tauri::State<'a, AppState>;
