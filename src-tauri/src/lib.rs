mod hardware;
mod software;

use hardware::HardwareInfo;
use software::SoftwareInfo;

#[tauri::command]
fn get_hardware_info() -> HardwareInfo {
    hardware::collect()
}

#[tauri::command]
fn get_hardware_detail() -> serde_json::Value {
    hardware::collect_detail()
}

#[tauri::command]
fn get_installed_software() -> Vec<SoftwareInfo> {
    software::collect()
}

#[tauri::command]
fn launch_app(app: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        std::process::Command::new("cmd")
            .args(["/c", "start", "", &app])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_hardware_info,
            get_hardware_detail,
            get_installed_software,
            launch_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
