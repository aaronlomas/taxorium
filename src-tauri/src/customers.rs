use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: i64,
    pub document_type: String,
    pub document_number: String,
    pub name: String,
    pub address: Option<String>,
    pub email: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomerPayload {
    pub document_type: String,
    pub document_number: String,
    pub name: String,
    pub address: Option<String>,
    pub email: Option<String>,
}

// --- CORE LOGIC ---

pub fn core_get_customers(db: &Connection) -> Result<Vec<Customer>, String> {
    let mut stmt = db
        .prepare("SELECT id, document_type, document_number, name, address, email, is_active FROM customers WHERE is_active = 1")
        .map_err(|e| e.to_string())?;
        
    let customer_iter = stmt.query_map([], |row| {
        Ok(Customer {
            id: row.get(0)?,
            document_type: row.get(1)?,
            document_number: row.get(2)?,
            name: row.get(3)?,
            address: row.get(4)?,
            email: row.get(5)?,
            is_active: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut customers = Vec::new();
    for customer in customer_iter {
        customers.push(customer.map_err(|e| e.to_string())?);
    }
    Ok(customers)
}

pub fn core_create_customer(db: &Connection, payload: CreateCustomerPayload) -> Result<Customer, String> {
    db.execute(
        "INSERT INTO customers (document_type, document_number, name, address, email) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![payload.document_type, payload.document_number, payload.name, payload.address, payload.email],
    ).map_err(|e| e.to_string())?;
    
    let id = db.last_insert_rowid();
    
    Ok(Customer {
        id,
        document_type: payload.document_type,
        document_number: payload.document_number,
        name: payload.name,
        address: payload.address,
        email: payload.email,
        is_active: true,
    })
}

pub fn core_update_customer(db: &Connection, id: i64, payload: CreateCustomerPayload) -> Result<Customer, String> {
    db.execute(
        "UPDATE customers SET document_type = ?1, document_number = ?2, name = ?3, address = ?4, email = ?5 WHERE id = ?6",
        params![payload.document_type, payload.document_number, payload.name, payload.address, payload.email, id],
    ).map_err(|e| e.to_string())?;
    
    Ok(Customer {
        id,
        document_type: payload.document_type,
        document_number: payload.document_number,
        name: payload.name,
        address: payload.address,
        email: payload.email,
        is_active: true,
    })
}

pub fn core_delete_customer(db: &Connection, id: i64) -> Result<(), String> {
    db.execute(
        "UPDATE customers SET is_active = 0 WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
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
pub fn create_customer(payload: CreateCustomerPayload, state: State<'_, AppState>) -> Result<Customer, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_create_customer(db, payload)
}

#[tauri::command]
pub fn update_customer(id: i64, payload: CreateCustomerPayload, state: State<'_, AppState>) -> Result<Customer, String> {
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
