pub mod model;
pub mod store;

use std::sync::Mutex;

use model::{ServiceInstance, ServiceInstanceDraft};
use store::Store;
use tauri::{Manager, State};

#[tauri::command]
fn list_instances(store: State<'_, Mutex<Store>>) -> Result<Vec<ServiceInstance>, String> {
    store
        .lock()
        .map_err(|_| "数据库状态不可用".to_owned())?
        .list_instances()
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn save_instance(
    store: State<'_, Mutex<Store>>,
    instance: ServiceInstanceDraft,
) -> Result<(), String> {
    store
        .lock()
        .map_err(|_| "数据库状态不可用".to_owned())?
        .register_instance(instance)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let directory = app.path().app_data_dir()?;
            std::fs::create_dir_all(&directory)?;
            let store = store::open(&directory.join("devdesk.sqlite"))?;
            app.manage(Mutex::new(store));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![list_instances, save_instance])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
