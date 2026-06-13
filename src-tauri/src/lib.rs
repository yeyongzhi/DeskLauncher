mod hardware;

use hardware::HardwareInfo;

#[tauri::command]
fn get_hardware_info() -> HardwareInfo {
    hardware::collect()
}

#[tauri::command]
fn get_hardware_detail() -> serde_json::Value {
    hardware::collect_detail()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_hardware_info, get_hardware_detail])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
