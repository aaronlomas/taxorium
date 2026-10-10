use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

/// Dirección real (`ip:puerto`) en la que este nodo servidor atiende la red local.
#[tauri::command]
pub fn get_local_ip() -> Result<String, String> {
    // El connect de un UDP socket no envía paquetes: solo hace que el sistema
    // resuelva la interfaz de la ruta por defecto.
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")
        .map_err(|e| format!("No se pudo abrir el socket: {e}"))?;
    socket
        .connect("8.8.8.8:80")
        .map_err(|e| format!("No se pudo determinar la IP local: {e}"))?;
    let ip = socket
        .local_addr()
        .map_err(|e| format!("No se pudo leer la dirección local: {e}"))?
        .ip();
    Ok(format!("{}:{}", ip, crate::api::API_PORT))
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

// ─── Configuración local del dispositivo ────────────────────────────────────
//
// A diferencia de NodeConfig (que describe el rol en la red), estos valores son
// intrínsecos de la máquina: la ruta del .p12 y su contraseña solo significan algo
// en el equipo donde está instalado el archivo. Por eso viven en un JSON local y
// se acceden por Tauri IPC, nunca por HTTP — de otro modo un nodo cliente
// recibiría la ruta del certificado del servidor, que no existe en su disco.

fn get_device_config_path(app: &AppHandle) -> PathBuf {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");
    if !app_dir.exists() {
        if let Err(e) = fs::create_dir_all(&app_dir) {
            log::error!("No se pudo crear el directorio de datos de la app: {}", e);
        }
    }
    app_dir.join("device_config.json")
}

pub fn load_device_config(app: &AppHandle) -> HashMap<String, String> {
    let path = get_device_config_path(app);
    if !path.exists() {
        return HashMap::new();
    }
    match fs::read_to_string(&path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(config) => config,
            Err(e) => {
                let msg =
                    format!("device_config.json tiene formato inválido y no se pudo leer: {e}");
                emit::error(app, "config", &msg);
                log::error!("{}", msg);
                HashMap::new()
            }
        },
        Err(e) => {
            let msg = format!("No se pudo leer device_config.json: {e}");
            emit::error(app, "config", &msg);
            log::error!("{}", msg);
            HashMap::new()
        }
    }
}

fn save_device_config(app: &AppHandle, config: &HashMap<String, String>) -> Result<(), String> {
    let path = get_device_config_path(app);
    let contents = serde_json::to_string_pretty(config).map_err(|e| {
        let msg = format!("Error al serializar device_config.json: {e}");
        emit::error(app, "config", &msg);
        msg
    })?;
    fs::write(&path, contents).map_err(|e| {
        let msg = format!("Error al guardar device_config.json en {:?}: {e}", path);
        emit::error(app, "config", &msg);
        msg
    })
}

/// Devuelve toda la configuración local del dispositivo como clave-valor.
#[tauri::command]
pub fn get_device_config(app: AppHandle) -> HashMap<String, String> {
    load_device_config(&app)
}

/// Guarda o actualiza un valor de la configuración local del dispositivo.
#[tauri::command]
pub fn set_device_config(app: AppHandle, clave: String, valor: String) -> Result<(), String> {
    let mut config = load_device_config(&app);
    config.insert(clave, valor);
    save_device_config(&app, &config)
}
