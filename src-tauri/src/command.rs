#[tauri::command]
pub async fn hello_manatsu() -> String {
  String::from("Hello, Manatsu!")
}
