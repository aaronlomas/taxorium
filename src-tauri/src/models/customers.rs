use crate::AppState;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: i64,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre: String,
    pub direccion: Option<String>,
    pub correo: Option<String>,
    pub activo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomerPayload {
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre: String,
    pub direccion: Option<String>,
    pub correo: Option<String>,
}

// --- CORE LOGIC ---

pub fn core_get_customers(db: &Connection) -> Result<Vec<Customer>, String> {
    let mut stmt = db
        .prepare("SELECT id, tipo_documento, numero_documento, nombre, direccion, correo, activo FROM clientes WHERE activo = 1")
        .map_err(|e| e.to_string())?;

    let customer_iter = stmt
        .query_map([], |row| {
            Ok(Customer {
                id: row.get(0)?,
                tipo_documento: row.get(1)?,
                numero_documento: row.get(2)?,
                nombre: row.get(3)?,
                direccion: row.get(4)?,
                correo: row.get(5)?,
                activo: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut customers = Vec::new();
    for customer in customer_iter {
        customers.push(customer.map_err(|e| e.to_string())?);
    }
    Ok(customers)
}

pub fn core_create_customer(
    db: &Connection,
    payload: CreateCustomerPayload,
) -> Result<Customer, String> {
    db.execute(
        "INSERT INTO clientes (tipo_documento, numero_documento, nombre, direccion, correo) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![payload.tipo_documento, payload.numero_documento, payload.nombre, payload.direccion, payload.correo],
    ).map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();

    Ok(Customer {
        id,
        tipo_documento: payload.tipo_documento,
        numero_documento: payload.numero_documento,
        nombre: payload.nombre,
        direccion: payload.direccion,
        correo: payload.correo,
        activo: true,
    })
}

pub fn core_update_customer(
    db: &Connection,
    id: i64,
    payload: CreateCustomerPayload,
) -> Result<Customer, String> {
    db.execute(
        "UPDATE clientes SET tipo_documento = ?1, numero_documento = ?2, nombre = ?3, direccion = ?4, correo = ?5 WHERE id = ?6",
        params![payload.tipo_documento, payload.numero_documento, payload.nombre, payload.direccion, payload.correo, id],
    ).map_err(|e| e.to_string())?;

    Ok(Customer {
        id,
        tipo_documento: payload.tipo_documento,
        numero_documento: payload.numero_documento,
        nombre: payload.nombre,
        direccion: payload.direccion,
        correo: payload.correo,
        activo: true,
    })
}

pub fn core_delete_customer(db: &Connection, id: i64) -> Result<(), String> {
    db.execute(
        "UPDATE clientes SET activo = 0 WHERE id = ?1",
        params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

// --- TAURI COMMANDS ---

#[tauri::command]
pub fn get_customers(state: State<'_, AppState>) -> Result<Vec<Customer>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_customers(db)
}

#[tauri::command]
pub fn create_customer(
    payload: CreateCustomerPayload,
    state: State<'_, AppState>,
) -> Result<Customer, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_create_customer(db, payload)
}

#[tauri::command]
pub fn update_customer(
    id: i64,
    payload: CreateCustomerPayload,
    state: State<'_, AppState>,
) -> Result<Customer, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_update_customer(db, id, payload)
}

#[tauri::command]
pub fn delete_customer(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_delete_customer(db, id)
}
