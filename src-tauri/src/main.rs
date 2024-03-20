// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(async_fn_in_trait)]

mod command;
pub mod database;
mod error;
pub mod prelude;

use std::sync::Arc;
use std::thread;
use tauri::async_runtime::block_on;
use tauri::Manager;

pub type State<'a> = tauri::State<'a, AppState>;

pub struct AppState {
  pub db: sea_orm::DatabaseConnection,
}

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_manatsu::init())
    .setup(|app| {
      let config = Arc::clone(&app.config());
      let handle = thread::spawn(move || block_on(database::connect(config)).unwrap());

      let state = AppState {
        db: handle.join().unwrap(),
      };

      app.manage(state);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![command::version])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
