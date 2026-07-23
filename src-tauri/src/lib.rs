mod units;
mod db;
mod customers;

use std::sync::Mutex;
use tauri::Manager;

pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
}

#[tauri::command]
fn get_device_id() -> Result<String, String> {
    machine_uid::get().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        get_device_id,
        customers::get_customers,
        customers::create_customer,
        customers::update_customer,
        customers::delete_customer
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      
      let device_id = machine_uid::get().unwrap_or_else(|_| "default_secure_password_123!".to_string());
      
      match db::init_db(app.handle(), &device_id) {
          Ok(conn) => {
              log::info!("Database initialized successfully!");
              app.manage(AppState {
                  db: Mutex::new(conn)
              });
          },
          Err(e) => log::error!("Failed to initialize database: {}", e),
      }

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
