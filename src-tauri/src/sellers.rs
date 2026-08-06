use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

use boveda_core::crypto::{decrypt_with_password, encrypt_with_password, verify_with_password};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Seller {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    // Note: We deliberately don't send the password_hash back to the client
    pub accesses: Option<String>,
    pub domain: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSellerPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub password_plain: String,
    pub accesses: Option<String>,
    pub domain: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSellerPayload {
    pub username: String,
    pub password_plain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSellerPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub password_plain: Option<String>,
    pub accesses: Option<String>,
    pub domain: Option<String>,
}

// Domain salt: deterministic, scoped to seller password encryption.
const SELLER_SALT: &[u8] = b"taxorium::sellers::v1";

// ─── Core logic ─────────────────────────────────────────────────────────────

pub fn core_get_sellers(db: &Connection) -> Result<Vec<Seller>, String> {
    let mut stmt = db
        .prepare("SELECT id, first_name, last_name, username, accesses, domain, is_active FROM sellers WHERE is_active = 1")
        .map_err(|e| e.to_string())?;

    let seller_iter = stmt
        .query_map([], |row| {
            Ok(Seller {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                username: row.get(3)?,
                accesses: row.get(4)?,
                domain: row.get(5)?,
                is_active: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut sellers = Vec::new();
    for seller in seller_iter {
        sellers.push(seller.map_err(|e| e.to_string())?);
    }
    Ok(sellers)
}

pub fn core_create_seller(
    db: &Connection,
    payload: CreateSellerPayload,
    device_id: &str,
) -> Result<Seller, String> {
    let password_encrypted = encrypt_with_password(&payload.password_plain, device_id, SELLER_SALT)
        .map_err(|e| e.to_string())?;

    db.execute(
        "INSERT INTO sellers (first_name, last_name, username, password_hash, accesses, domain) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![payload.first_name, payload.last_name, payload.username, password_encrypted, payload.accesses, payload.domain],
    )
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("UNIQUE constraint failed") {
            format!("El usuario '{}' ya existe. Por favor elige un nombre de usuario diferente.", payload.username)
        } else {
            msg
        }
    })?;

    let id = db.last_insert_rowid();

    Ok(Seller {
        id,
        first_name: payload.first_name,
        last_name: payload.last_name,
        username: payload.username,
        accesses: payload.accesses,
        domain: payload.domain,
        is_active: true,
    })
}

pub fn core_update_seller(
    db: &Connection,
    id: i64,
    payload: UpdateSellerPayload,
    device_id: &str,
) -> Result<Seller, String> {
    if let Some(pwd) = &payload.password_plain {
        let password_encrypted =
            encrypt_with_password(pwd, device_id, SELLER_SALT).map_err(|e| e.to_string())?;
        db.execute(
            "UPDATE sellers SET first_name = ?1, last_name = ?2, username = ?3, password_hash = ?4, accesses = ?5, domain = ?6 WHERE id = ?7",
            params![payload.first_name, payload.last_name, payload.username, password_encrypted, payload.accesses, payload.domain, id],
        )
        .map_err(|e| e.to_string())?;
    } else {
        db.execute(
            "UPDATE sellers SET first_name = ?1, last_name = ?2, username = ?3, accesses = ?4, domain = ?5 WHERE id = ?6",
            params![payload.first_name, payload.last_name, payload.username, payload.accesses, payload.domain, id],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(Seller {
        id,
        first_name: payload.first_name,
        last_name: payload.last_name,
        username: payload.username,
        accesses: payload.accesses,
        domain: payload.domain,
        is_active: true,
    })
}

pub fn core_delete_seller(db: &Connection, id: i64) -> Result<(), String> {
    db.execute(
        "UPDATE sellers SET is_active = 0 WHERE id = ?1",
        params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn core_login_seller(
    db: &Connection,
    payload: LoginSellerPayload,
    device_id: &str,
) -> Result<Seller, String> {
    // Fetch the seller row by username only — never by hash in plaintext comparison
    let mut stmt = db
        .prepare("SELECT id, first_name, last_name, username, password_hash, accesses, domain, is_active FROM sellers WHERE username = ?1 AND is_active = 1")
        .map_err(|e| e.to_string())?;

    let result = stmt.query_row(params![payload.username], |row| {
        Ok((
            Seller {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                username: row.get(3)?,
                accesses: row.get(5)?,
                domain: row.get(6)?,
                is_active: row.get(7)?,
            },
            row.get::<_, String>(4)?, // password_hash (now stores ciphertext)
        ))
    });

    match result {
        Ok((seller, ciphertext)) => {
            if verify_with_password(&payload.password_plain, &ciphertext, device_id, SELLER_SALT) {
                Ok(seller)
            } else {
                Err("Invalid credentials".into())
            }
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Err("Invalid credentials".into()),
        Err(e) => Err(e.to_string()),
    }
}

// ─── Tauri commands ──────────────────────────────────────────────────────────

fn get_device_id() -> String {
    machine_uid::get().unwrap_or_else(|_| "default_secure_password_123!".to_string())
}

#[tauri::command]
pub fn get_sellers(state: State<'_, AppState>) -> Result<Vec<Seller>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_sellers(db)
}

#[tauri::command]
pub fn create_seller(
    payload: CreateSellerPayload,
    state: State<'_, AppState>,
) -> Result<Seller, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_create_seller(db, payload, &get_device_id())
}

#[tauri::command]
pub fn update_seller(
    id: i64,
    payload: UpdateSellerPayload,
    state: State<'_, AppState>,
) -> Result<Seller, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_update_seller(db, id, payload, &get_device_id())
}

#[tauri::command]
pub fn delete_seller(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_delete_seller(db, id)
}

#[tauri::command]
pub fn login_seller(
    payload: LoginSellerPayload,
    state: State<'_, AppState>,
) -> Result<Seller, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_login_seller(db, payload, &get_device_id())
}

/// Exposes the decrypted password for a specific seller ID.
/// Used by the frontend to populate the password cache on startup.
#[tauri::command]
pub fn get_seller_password(seller_id: i64, state: State<'_, AppState>) -> Result<String, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;

    let ciphertext: String = db
        .query_row(
            "SELECT password_hash FROM sellers WHERE id = ?1 AND is_active = 1",
            params![seller_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    decrypt_with_password(&ciphertext, &get_device_id(), SELLER_SALT).map_err(|e| e.to_string())
}
