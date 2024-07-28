use super::WindowKind;
use crate::error::Result;
use tauri::{AppHandle, WebviewWindowBuilder};

pub fn open(app: &AppHandle) -> Result<()> {
  let kind = WindowKind::Main;
  WebviewWindowBuilder::new(app, kind.label(), kind.url())
    .data_directory(kind.data_dir(app)?)
    .title("Tauri Template")
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(false)
    .build()?;

  #[cfg(feature = "open-devtools")]
  window.open_devtools();

  Ok(())
}
