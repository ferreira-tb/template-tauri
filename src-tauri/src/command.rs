#[tauri::command]
pub async fn version(app: tauri::AppHandle) -> String {
  app.config().version.clone().unwrap()
}
