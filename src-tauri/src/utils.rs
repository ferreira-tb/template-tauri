#[cfg(debug_assertions)]
pub mod log {
  use tauri::{AppHandle, Manager};
  use tracing_appender::non_blocking::WorkerGuard;
  use tracing_appender::rolling;
  use tracing_subscriber::fmt::time::ChronoLocal;
  use tracing_subscriber::fmt::writer::MakeWriterExt;
  use tracing_subscriber::fmt::Layer;
  use tracing_subscriber::layer::SubscriberExt;
  use tracing_subscriber::{EnvFilter, Registry};

  /// <https://docs.rs/chrono/latest/chrono/format/strftime/index.html>
  const TIMESTAMP: &str = "%F %T%.3f %:z";

  struct TracingGuard {
    _guard: WorkerGuard,
  }

  pub fn setup_tracing(app: &AppHandle) {
    let filter = EnvFilter::builder()
      .from_env()
      .unwrap()
      .add_directive("tauri_template=trace".parse().unwrap())
      .add_directive("tauri_plugin_manatsu=trace".parse().unwrap());

    let appender = rolling::never("../", "app.log");
    let (writer, guard) = tracing_appender::non_blocking(appender);
    app.manage(TracingGuard { _guard: guard });

    let file = Layer::default()
      .with_ansi(false)
      .with_timer(ChronoLocal::new(TIMESTAMP.into()))
      .with_writer(writer.with_max_level(tracing::Level::TRACE))
      .pretty();

    let stderr = Layer::default()
      .with_ansi(true)
      .with_timer(ChronoLocal::new(TIMESTAMP.into()))
      .with_writer(std::io::stderr)
      .pretty();

    let subscriber = Registry::default()
      .with(file)
      .with(stderr)
      .with(filter);

    tracing::subscriber::set_global_default(subscriber).unwrap();
  }
}
