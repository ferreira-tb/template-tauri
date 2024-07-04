// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(let_chains, try_blocks)]

mod command;
mod database;
mod error;
mod prelude;
mod state;
mod utils;
mod window;

use error::BoxResult;
use state::AppState;
use tauri::{App, Manager};

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_manatsu::init())
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(plugin::prevent_default())
    .plugin(plugin::single_instance())
    .plugin(plugin::window_state())
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

  window::app::open(handle)?;

  Ok(())
}

mod plugin {
  use crate::window::{WindowExt, WindowManager};
  use tauri::plugin::TauriPlugin;
  use tauri::Wry;

  #[cfg(debug_assertions)]
  pub fn prevent_default() -> TauriPlugin<Wry> {
    use tauri_plugin_prevent_default::Flags;

    tauri_plugin_prevent_default::Builder::new()
      .with_flags(Flags::all().difference(Flags::RELOAD))
      .build()
  }

  #[cfg(not(debug_assertions))]
  pub fn prevent_default() -> TauriPlugin<Wry> {
    tauri_plugin_prevent_default::Builder::new().build()
  }

  pub fn single_instance() -> TauriPlugin<Wry> {
    tauri_plugin_single_instance::init(|app, _, _| {
      let _ = app.main_window().set_foreground_focus();
    })
  }

  pub fn window_state() -> TauriPlugin<Wry> {
    use tauri_plugin_window_state::StateFlags as Flags;

    tauri_plugin_window_state::Builder::new()
      .with_state_flags(Flags::MAXIMIZED | Flags::POSITION | Flags::SIZE)
      .build()
  }
}
