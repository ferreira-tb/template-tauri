use sea_orm::DatabaseConnection;

pub struct AppState {
  // REMOVE ME
  #[allow(dead_code)]
  pub db: DatabaseConnection,
}
