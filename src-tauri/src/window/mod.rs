pub mod app;

use crate::prelude::*;
use std::path::PathBuf;
use strum::{Display, EnumIs};
use tauri::{EventTarget, WebviewUrl};

#[derive(Debug, Display, EnumIs)]
#[strum(serialize_all = "kebab-case")]
pub enum WindowKind {
  Main,
}

impl WindowKind {
  pub fn label(&self) -> String {
    self.to_string()
  }

  fn data_dir(&self, app: &AppHandle) -> Result<PathBuf> {
    let label = self.label();
    app
      .path()
      .app_local_data_dir()
      .map(|it| it.join(format!("windows/{label}")))
      .map_err(Into::into)
  }

  fn url(&self) -> WebviewUrl {
    let name = match self {
      Self::Main => "main",
    };

    WebviewUrl::App(format!("src/windows/{name}/index.html",).into())
  }
}

impl From<WindowKind> for EventTarget {
  fn from(kind: WindowKind) -> Self {
    let label = kind.label();
    EventTarget::WebviewWindow { label }
  }
}

pub trait WindowManager: Manager<Wry> {
  fn main_window(&self) -> WebviewWindow {
    let label = WindowKind::Main.label();
    self.get_webview_window(&label).unwrap()
  }
}

impl WindowManager for AppHandle {}

pub trait WindowExt {
  /// Like [`WebviewWindow::set_focus`], but unminimize the window before focusing.
  fn set_foreground_focus(&self) -> Result<()>;
}

impl WindowExt for WebviewWindow {
  fn set_foreground_focus(&self) -> Result<()> {
    if self.is_minimized()? {
      self.unminimize()?;
    }

    self.set_focus().map_err(Into::into)
  }
}
