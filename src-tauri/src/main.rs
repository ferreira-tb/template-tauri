// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(async_fn_in_trait)]

mod command;
pub mod database;
mod error;
pub mod prelude;

use std::path::PathBuf;
use std::sync::OnceLock;
use tauri::api::path::{app_cache_dir, app_data_dir};
use tauri::Manager;

pub struct AppState {
  pub database: sea_orm::DatabaseConnection,
}

pub type State<'a> = tauri::State<'a, AppState>;

pub static APP_CACHE_DIR: OnceLock<PathBuf> = OnceLock::new();
pub static APP_DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_manatsu::init())
    .setup(|app| {
      APP_CACHE_DIR
        .set(app_cache_dir(&app.config()).unwrap())
        .unwrap();

      APP_DATA_DIR
        .set(app_data_dir(&app.config()).unwrap())
        .unwrap();

      let state = AppState {
        database: database::connect().unwrap(),
      };

      app.manage(state);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![command::version])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
