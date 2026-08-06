use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::emit;

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
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");
    if !app_dir.exists() {
        if let Err(e) = fs::create_dir_all(&app_dir) {
            log::error!("No se pudo crear el directorio de datos de la app: {}", e);
        }
    }
    app_dir.join("node_config.json")
}

pub fn load_config(app: &AppHandle) -> NodeConfig {
    let path = get_config_path(app);
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(config) => return config,
                Err(e) => {
                    let msg =
                        format!("node_config.json tiene formato inválido y no se pudo leer: {e}");
                    emit::error(app, "config", &msg);
                    log::error!("{}", msg);
                }
            },
            Err(e) => {
                let msg = format!("No se pudo leer node_config.json: {e}");
                emit::error(app, "config", &msg);
                log::error!("{}", msg);
            }
        }
    }
    NodeConfig::default()
}

pub fn save_config(app: &AppHandle, config: &NodeConfig) -> Result<(), String> {
    let path = get_config_path(app);
    let contents = serde_json::to_string_pretty(config).map_err(|e| {
        let msg = format!("Error al serializar la configuración: {e}");
        emit::error(app, "config", &msg);
        msg
    })?;
    fs::write(&path, contents).map_err(|e| {
        let msg = format!("Error al guardar node_config.json en {:?}: {e}", path);
        emit::error(app, "config", &msg);
        msg
    })?;
    emit::info(
        app,
        "config",
        format!("Configuración guardada: rol = {:?}", config.role),
    );
    Ok(())
}

#[tauri::command]
pub fn get_node_config(app: AppHandle) -> NodeConfig {
    load_config(&app)
}

#[tauri::command]
pub fn set_node_config(
    role: String,
    server_ip: Option<String>,
    app: AppHandle,
) -> Result<(), String> {
    let config = NodeConfig {
        role: Some(role),
        server_ip,
    };
    save_config(&app, &config)
}
