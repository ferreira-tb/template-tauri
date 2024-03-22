use sea_orm::DatabaseConnection;

pub struct AppState {
  pub database: DatabaseConnection,
}

pub type State<'a> = tauri::State<'a, AppState>;
