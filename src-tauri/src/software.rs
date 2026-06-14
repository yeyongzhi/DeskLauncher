use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;
use std::collections::HashSet;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Serialize, Debug, Clone)]
pub struct SoftwareInfo {
    pub name: String,
    pub version: String,
    pub publisher: String,
    pub install_location: String,
    pub install_date: String,
    pub size_kb: u64,
    pub icon: String,
}

pub fn collect() -> Vec<SoftwareInfo> {
    let mut apps: Vec<SoftwareInfo> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    read_uninstall_key(
        &hklm,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        &mut seen,
        &mut apps,
    );
    read_uninstall_key(
        &hklm,
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
        &mut seen,
        &mut apps,
    );
    read_uninstall_key(
        &hkcu,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        &mut seen,
        &mut apps,
    );

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn read_uninstall_key(
    hive: &RegKey,
    path: &str,
    seen: &mut HashSet<String>,
    apps: &mut Vec<SoftwareInfo>,
) {
    let Ok(uninstall_key) = hive.open_subkey(path) else {
        return;
    };

    for key_name in uninstall_key.enum_keys().flatten() {
        let Ok(subkey) = uninstall_key.open_subkey(&key_name) else {
            continue;
        };

        let name: String = subkey.get_value("DisplayName").unwrap_or_default();
        if name.is_empty() || seen.contains(&name) {
            continue;
        }

        // Skip system components and updates
        let system_component: u32 = subkey.get_value("SystemComponent").unwrap_or(0);
        if system_component == 1 {
            continue;
        }

        seen.insert(name.clone());

        let version: String = subkey.get_value("DisplayVersion").unwrap_or_default();
        let publisher: String = subkey.get_value("Publisher").unwrap_or_default();
        let install_location: String = subkey.get_value("InstallLocation").unwrap_or_default();
        let install_date_raw: String = subkey.get_value("InstallDate").unwrap_or_default();
        let size_kb: u32 = subkey.get_value("EstimatedSize").unwrap_or(0);
        let icon_raw: String = subkey.get_value("DisplayIcon").unwrap_or_default();
        let icon = extract_icon(&icon_raw);

        apps.push(SoftwareInfo {
            name,
            version,
            publisher,
            install_location,
            install_date: format_date(&install_date_raw),
            size_kb: size_kb as u64,
            icon,
        });
    }
}

fn expand_env_vars(s: &str) -> String {
    let mut result = s.to_string();
    loop {
        let Some(start) = result.find('%') else { break };
        let Some(end) = result[start + 1..].find('%') else { break };
        let var_name = result[start + 1..start + 1 + end].to_string();
        let Ok(val) = std::env::var(&var_name) else { break };
        result = format!("{}{}{}", &result[..start], val, &result[start + 1 + end + 1..]);
    }
    result
}

fn extract_icon(raw: &str) -> String {
    if raw.is_empty() {
        return String::new();
    }

    // Strip surrounding quotes then split off optional ",index" suffix
    let raw = raw.trim().trim_matches('"');
    let path_str = match raw.rfind(',') {
        Some(idx) => {
            let after = raw[idx + 1..].trim();
            if after.chars().all(|c| c.is_ascii_digit() || c == '-') {
                raw[..idx].trim()
            } else {
                raw
            }
        }
        None => raw,
    };

    let path_str = expand_env_vars(path_str);

    if path_str.to_lowercase().ends_with(".ico") {
        if let Ok(bytes) = std::fs::read(&path_str) {
            let encoded = general_purpose::STANDARD.encode(&bytes);
            return format!("data:image/x-icon;base64,{}", encoded);
        }
    }

    String::new()
}

fn format_date(raw: &str) -> String {
    if raw.len() == 8 && raw.chars().all(|c| c.is_ascii_digit()) {
        format!("{}-{}-{}", &raw[0..4], &raw[4..6], &raw[6..8])
    } else {
        raw.to_string()
    }
}
