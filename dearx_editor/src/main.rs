#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JS!");
}

#[tauri::command]
fn on_value_changed(value: f64) {
    println!("Value Changed!: {}", value);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command, on_value_changed])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
