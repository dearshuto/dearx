#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dearx_edit_model::{DearxProject, GameObject, GameObjectId};
use dearx_tool_windows::{MainWindowViewModel, ServiceProvider};
use dearx_workspace::{DocumentInfo, Workspace};
use im::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let (mut workspace, id) = {
        let mut workspace = Workspace::<DearxProject>::new();
        let mut game_object_map = HashMap::new();
        game_object_map.insert(GameObjectId::new(), GameObject::new());
        game_object_map.insert(GameObjectId::new(), GameObject::new());

        let content = DearxProject::new().with_game_object(Arc::new(game_object_map));
        let id = workspace.add_document(&DocumentInfo { content });
        (workspace, id)
    };
    // workspace.mutable_instance.active_document_manager.active_id = Some(id);

    let workspace = Arc::new(Mutex::new(workspace));
    let mut main_window_view_model = MainWindowViewModel::new(workspace.clone());
    tauri::Builder::default()
        .setup(move |app| {
            // main_window_view_model.listen(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
