use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeConfig {
    pub role: Option<String>,      // "server" or "client"
    pub server_ip: Option<String>, // e.g., "192.168.1.100:3000"
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            role: None,
            server_ip: None,
        }
    }
}

fn get_config_path(app: &AppHandle) -> PathBuf {
    let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
    }
    app_dir.join("node_config.json")
}

pub fn load_config(app: &AppHandle) -> NodeConfig {
    let path = get_config_path(app);
    if path.exists() {
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&contents) {
                return config;
            }
        }
    }
    NodeConfig::default()
}

pub fn save_config(app: &AppHandle, config: &NodeConfig) -> Result<(), String> {
    let path = get_config_path(app);
    let contents = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, contents).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_node_config(app: AppHandle) -> NodeConfig {
    load_config(&app)
}

#[tauri::command]
pub fn set_node_config(role: String, server_ip: Option<String>, app: AppHandle) -> Result<(), String> {
    let config = NodeConfig {
        role: Some(role),
        server_ip,
    };
    save_config(&app, &config)
}
