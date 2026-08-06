mod units;
mod db;
mod customers;
mod config;
mod api;
mod sellers;
mod emit;

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
    let device_id = machine_uid::get().unwrap_or_else(|_| "default_secure_password_123!".to_string());
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
    .invoke_handler(tauri::generate_handler![
        get_device_id,
        config::get_node_config,
        config::set_node_config,
        init_server_db,
        customers::get_customers,
        customers::create_customer,
        customers::update_customer,
        customers::delete_customer,
        sellers::get_sellers,
        sellers::create_seller,
        sellers::update_seller,
        sellers::delete_seller,
        sellers::login_seller,
        sellers::get_seller_password
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
                  emit::info(app.handle(), "db", "Base de datos inicializada correctamente al arrancar.");
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
          emit::warn(app.handle(), "config", "Rol de nodo no configurado. Ve a Configuración de Red.");
          log::warn!("Node role not configured — redirecting to setup.");
      }

      app.manage(AppState {
          db: Mutex::new(db_conn)
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
