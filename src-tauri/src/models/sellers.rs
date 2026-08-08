use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

use boveda_core::crypto::{decrypt_with_password, encrypt_with_password, verify_with_password};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Seller {
    pub id: i64,
    pub nombres: Option<String>,
    pub apellidos: Option<String>,
    pub usuario: String,
    // Note: We deliberately don't send the clave_cifrada back to the client
    pub accesos: Option<String>,
    pub dominio: Option<String>,
    pub activo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSellerPayload {
    pub nombres: Option<String>,
    pub apellidos: Option<String>,
    pub usuario: String,
    pub clave_plana: String,
    pub accesos: Option<String>,
    pub dominio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSellerPayload {
    pub usuario: String,
    pub clave_plana: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSellerPayload {
    pub nombres: Option<String>,
    pub apellidos: Option<String>,
    pub usuario: String,
    pub clave_plana: Option<String>,
    pub accesos: Option<String>,
    pub dominio: Option<String>,
}

// Domain salt: deterministic, scoped to seller password encryption.
const SELLER_SALT: &[u8] = b"taxorium::sellers::v1";

// ─── Core logic ─────────────────────────────────────────────────────────────

pub fn core_get_sellers(db: &Connection) -> Result<Vec<Seller>, String> {
    let mut stmt = db
        .prepare("SELECT id, nombres, apellidos, usuario, accesos, dominio, activo FROM vendedores WHERE activo = 1")
        .map_err(|e| e.to_string())?;

    let seller_iter = stmt
        .query_map([], |row| {
            Ok(Seller {
                id: row.get(0)?,
                nombres: row.get(1)?,
                apellidos: row.get(2)?,
                usuario: row.get(3)?,
                accesos: row.get(4)?,
                dominio: row.get(5)?,
                activo: row.get(6)?,
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
    let password_encrypted = encrypt_with_password(&payload.clave_plana, device_id, SELLER_SALT)
        .map_err(|e| e.to_string())?;

    db.execute(
        "INSERT INTO vendedores (nombres, apellidos, usuario, clave_cifrada, accesos, dominio) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![payload.nombres, payload.apellidos, payload.usuario, password_encrypted, payload.accesos, payload.dominio],
    )
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("UNIQUE constraint failed") {
            format!("El usuario '{}' ya existe. Por favor elige un nombre de usuario diferente.", payload.usuario)
        } else {
            msg
        }
    })?;

    let id = db.last_insert_rowid();

    Ok(Seller {
        id,
        nombres: payload.nombres,
        apellidos: payload.apellidos,
        usuario: payload.usuario,
        accesos: payload.accesos,
        dominio: payload.dominio,
        activo: true,
    })
}

pub fn core_update_seller(
    db: &Connection,
    id: i64,
    payload: UpdateSellerPayload,
    device_id: &str,
) -> Result<Seller, String> {
    if let Some(pwd) = &payload.clave_plana {
        let password_encrypted =
            encrypt_with_password(pwd, device_id, SELLER_SALT).map_err(|e| e.to_string())?;
        db.execute(
            "UPDATE vendedores SET nombres = ?1, apellidos = ?2, usuario = ?3, clave_cifrada = ?4, accesos = ?5, dominio = ?6 WHERE id = ?7",
            params![payload.nombres, payload.apellidos, payload.usuario, password_encrypted, payload.accesos, payload.dominio, id],
        )
        .map_err(|e| e.to_string())?;
    } else {
        db.execute(
            "UPDATE vendedores SET nombres = ?1, apellidos = ?2, usuario = ?3, accesos = ?4, dominio = ?5 WHERE id = ?6",
            params![payload.nombres, payload.apellidos, payload.usuario, payload.accesos, payload.dominio, id],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(Seller {
        id,
        nombres: payload.nombres,
        apellidos: payload.apellidos,
        usuario: payload.usuario,
        accesos: payload.accesos,
        dominio: payload.dominio,
        activo: true,
    })
}

pub fn core_delete_seller(db: &Connection, id: i64) -> Result<(), String> {
    db.execute(
        "UPDATE vendedores SET activo = 0 WHERE id = ?1",
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
    // Fetch the seller row by usuario only — never by hash in plaintext comparison
    let mut stmt = db
        .prepare("SELECT id, nombres, apellidos, usuario, clave_cifrada, accesos, dominio, activo FROM vendedores WHERE usuario = ?1 AND activo = 1")
        .map_err(|e| e.to_string())?;

    let result = stmt.query_row(params![payload.usuario], |row| {
        Ok((
            Seller {
                id: row.get(0)?,
                nombres: row.get(1)?,
                apellidos: row.get(2)?,
                usuario: row.get(3)?,
                accesos: row.get(5)?,
                dominio: row.get(6)?,
                activo: row.get(7)?,
            },
            row.get::<_, String>(4)?, // clave_cifrada (now stores ciphertext)
        ))
    });

    match result {
        Ok((seller, ciphertext)) => {
            if verify_with_password(&payload.clave_plana, &ciphertext, device_id, SELLER_SALT) {
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
            "SELECT clave_cifrada FROM vendedores WHERE id = ?1 AND activo = 1",
            params![seller_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    decrypt_with_password(&ciphertext, &get_device_id(), SELLER_SALT).map_err(|e| e.to_string())
}
