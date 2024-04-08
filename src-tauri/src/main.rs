// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
pub mod database;
mod error;
pub mod prelude;

use error::BoxResult;
use sea_orm::DatabaseConnection;
use tauri::{App, Manager};

pub struct AppState {
  pub db: DatabaseConnection,
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_manatsu::init())
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .setup(setup)
    .invoke_handler(tauri::generate_handler![command::hello_tauri])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn setup(app: &mut App) -> BoxResult<()> {
  let handle = app.handle();
  let state = AppState {
    db: database::connect(handle).unwrap(),
  };

  app.manage(state);

  Ok(())
}
