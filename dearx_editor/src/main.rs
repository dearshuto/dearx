#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dearx_edit_model::DearxProject;
use dearx_editor::MainWindowViewModel;
use dearx_workspace::{DocumentInfo, Workspace};
use std::sync::{Arc, Mutex};
use tauri::Manager;

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JS!");
}

#[tauri::command]
fn on_value_changed(value: f64) {
    println!("{}", value);
}

#[tokio::main]
async fn main() {
    let mut workspace = Workspace::new();
    let id = workspace.add_document(&DocumentInfo {
        content: Arc::new(DearxProject::new()),
    });
    workspace.update_current_project(&id.clone(), |x| x); // テストコード

    // ワークスペースへの変更を監視するテストコード
    workspace
        .observe_project()
        .lock()
        .unwrap()
        .subscribe(move |project| {
            if let Some(document) = project.documents.get(&id) {
                println!("Value: {}", document.content.value);
            }
        });

    let workspace = Arc::new(Mutex::new(workspace));
    let builder = tauri::Builder::default();
    let main_window_view_model = MainWindowViewModel::new(workspace.clone());

    builder
        .setup(move |app| {
            let app_handle = app.app_handle();
            std::thread::spawn(move || loop {
                app_handle
                    .emit_all("back-to-front", "ping frontend".to_string())
                    .unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1))
            });
            app.manage(id.clone());
            app.manage(workspace);

            main_window_view_model.listen(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            my_custom_command,
            on_value_changed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
