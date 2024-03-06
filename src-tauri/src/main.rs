// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(async_fn_in_trait)]

mod command;
mod database;
mod error;

pub use error::{Error, Result};
use sea_orm::prelude::DatabaseConnection;

pub struct AppState {
  pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
  let db = database::connect().await;
  let state = AppState { db };

  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_manatsu::init())
    .manage(state)
    .invoke_handler(tauri::generate_handler![command::version])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
