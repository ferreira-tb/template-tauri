use serde::ser::Serializer;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Database(#[from] sea_orm::error::DbErr),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
  #[error(transparent)]
  Tauri(#[from] tauri::Error),
  #[error(transparent)]
  Unknown(#[from] anyhow::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
