use crate::prelude::*;
use crate::window::WindowExt;

#[tauri::command]
pub async fn close_window(window: WebviewWindow) -> Result<()> {
  window.close().map_err(Into::into)
}

#[tauri::command]
pub async fn focus_window(window: WebviewWindow) -> Result<()> {
  window.set_foreground_focus().map_err(Into::into)
}

#[tauri::command]
pub async fn show_window(window: WebviewWindow) -> Result<()> {
  window.show().map_err(Into::into)
}
