mod api;
mod config;
mod db;
mod emit;
mod models;

use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

pub struct AppState {
    pub db: Mutex<Option<rusqlite::Connection>>,
}

#[tauri::command]
fn get_device_id() -> Result<String, String> {
    machine_uid::get().map_err(|e| e.to_string())
}

#[tauri::command]
fn init_server_db(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let device_id =
        machine_uid::get().unwrap_or_else(|_| "default_secure_password_123!".to_string());
    match db::init_db(&app, &device_id) {
        Ok(conn) => {
            emit::info(&app, "db", "Base de datos inicializada correctamente.");
            log::info!("Database initialized successfully on demand!");
            let mut db_guard = state.db.lock().map_err(|e| {
                let msg = format!("No se pudo adquirir el lock de la DB: {e}");
                emit::error(&app, "db", &msg);
                e.to_string()
            })?;
            *db_guard = Some(conn);
            api::spawn_server(app.clone());
            Ok(())
        }
        Err(e) => {
            let msg = format!("Error al inicializar la base de datos: {e}");
            emit::error(&app, "db", &msg);
            log::error!("Failed to initialize database: {}", e);
            Err(e.to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_device_id,
            config::get_node_config,
            config::set_node_config,
            init_server_db,
            models::customers::get_customers,
            models::customers::create_customer,
            models::customers::update_customer,
            models::customers::delete_customer,
            models::sellers::get_sellers,
            models::sellers::create_seller,
            models::sellers::update_seller,
            models::sellers::delete_seller,
            models::sellers::login_seller,
            models::sellers::get_seller_password,
            models::products::get_products,
            models::products::create_product,
            models::products::update_product,
            models::products::delete_product,
            models::catalogos::get_monedas,
            models::catalogos::get_unidades,
            models::catalogos::get_sedes,
            models::catalogos::get_afectaciones_venta,
            models::catalogos::get_afectaciones_compra,
            models::catalogos::get_tipos_operacion,
            models::catalogos::get_tipos_pago,
            models::catalogos::get_tipos_comprobante,
            models::series::get_series,
            models::series::create_serie,
            models::series::update_serie,
            models::series::delete_serie
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let node_config = config::load_config(app.handle());

            let mut db_conn = None;
            if node_config.role.as_deref() == Some("server") {
                let device_id = machine_uid::get()
                    .unwrap_or_else(|_| "default_secure_password_123!".to_string());
                match db::init_db(app.handle(), &device_id) {
                    Ok(conn) => {
                        emit::info(
                            app.handle(),
                            "db",
                            "Base de datos inicializada correctamente al arrancar.",
                        );
                        log::info!("Database initialized successfully!");
                        db_conn = Some(conn);
                        api::spawn_server(app.handle().clone());
                    }
                    Err(e) => {
                        let msg = format!("Error al inicializar la base de datos al arrancar: {e}");
                        emit::error(app.handle(), "db", &msg);
                        log::error!("Failed to initialize database: {}", e);
                    }
                }
            } else if node_config.role.is_none() {
                emit::warn(
                    app.handle(),
                    "config",
                    "Rol de nodo no configurado. Ve a Configuración de Red.",
                );
                log::warn!("Node role not configured — redirecting to setup.");
            }

            app.manage(AppState {
                db: Mutex::new(db_conn),
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
