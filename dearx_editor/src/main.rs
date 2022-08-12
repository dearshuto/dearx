#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

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
    .setup(|app| {
        let app_handle = app.app_handle();
        std::thread::spawn(move || loop {
            app_handle
                .emit_all("back-to-front", "ping frontend".to_string())
                .unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1))
        });
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![my_custom_command, on_value_changed])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
