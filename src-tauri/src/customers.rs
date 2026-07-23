use rusqlite::params;
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

#[tauri::command]
pub fn get_customers(state: State<'_, AppState>) -> Result<Vec<Customer>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
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

#[tauri::command]
pub fn create_customer(payload: CreateCustomerPayload, state: State<'_, AppState>) -> Result<Customer, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
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

#[tauri::command]
pub fn update_customer(id: i64, payload: CreateCustomerPayload, state: State<'_, AppState>) -> Result<Customer, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
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

#[tauri::command]
pub fn delete_customer(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    // Eliminación lógica
    db.execute(
        "UPDATE customers SET is_active = 0 WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}
