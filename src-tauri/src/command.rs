#[tauri::command]
pub async fn hello_tauri() -> String {
  String::from("Hello, Tauri!")
}
