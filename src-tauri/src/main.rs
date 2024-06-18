// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(try_blocks)]

mod command;
pub mod database;
mod error;
mod prelude;
pub mod state;
mod utils;

use error::BoxResult;
use state::AppState;
use tauri::{App, Manager};

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_manatsu::init())
    .setup(setup)
    .invoke_handler(tauri::generate_handler![
      command::close_window,
      command::show_window
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn setup(app: &mut App) -> BoxResult<()> {
  let handle = app.handle();

  #[cfg(debug_assertions)]
  utils::log::setup_tracing(handle);

  let db = database::connect(handle)?;
  app.manage(AppState { db });

  Ok(())
}
