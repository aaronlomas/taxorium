use crate::AppState;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vendor {
    pub id: i64,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre: String,
    pub nombre_comercial: Option<String>,
    pub pais: Option<String>,
    pub departamento: Option<String>,
    pub provincia: Option<String>,
    pub distrito: Option<String>,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub contacto: Option<String>,
    pub cuenta_bancaria: Option<String>,
    pub observaciones: Option<String>,
    pub activo: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateVendorPayload {
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre: String,
    pub nombre_comercial: Option<String>,
    pub pais: Option<String>,
    pub departamento: Option<String>,
    pub provincia: Option<String>,
    pub distrito: Option<String>,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub contacto: Option<String>,
    pub cuenta_bancaria: Option<String>,
    pub observaciones: Option<String>,
}

// --- CORE LOGIC ---

pub fn core_get_vendors(db: &Connection) -> Result<Vec<Vendor>, String> {
    let mut stmt = db
        .prepare(
            "SELECT id, tipo_documento, numero_documento, nombre, nombre_comercial, pais, departamento, provincia, distrito, direccion, telefono, correo, contacto, cuenta_bancaria, observaciones, activo FROM proveedores WHERE activo = 1",
        )
        .map_err(|e| e.to_string())?;

    let vendor_iter = stmt
        .query_map([], |row| {
            Ok(Vendor {
                id: row.get(0)?,
                tipo_documento: row.get(1)?,
                numero_documento: row.get(2)?,
                nombre: row.get(3)?,
                nombre_comercial: row.get(4)?,
                pais: row.get(5)?,
                departamento: row.get(6)?,
                provincia: row.get(7)?,
                distrito: row.get(8)?,
                direccion: row.get(9)?,
                telefono: row.get(10)?,
                correo: row.get(11)?,
                contacto: row.get(12)?,
                cuenta_bancaria: row.get(13)?,
                observaciones: row.get(14)?,
                activo: row.get(15)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut vendors = Vec::new();
    for vendor in vendor_iter {
        vendors.push(vendor.map_err(|e| e.to_string())?);
    }
    Ok(vendors)
}

pub fn core_create_vendor(db: &Connection, payload: CreateVendorPayload) -> Result<Vendor, String> {
    let existing = db
        .query_row(
            "SELECT id, activo FROM proveedores WHERE numero_documento = ?1 LIMIT 1",
            params![payload.numero_documento],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, bool>(1)?)),
        )
        .optional()
        .map_err(|e| e.to_string())?;

    if let Some((id, activo)) = existing {
        if activo {
            return Err(format!(
                "Ya existe un proveedor activo con el documento {}",
                payload.numero_documento
            ));
        }

        db.execute(
            "UPDATE proveedores SET tipo_documento = ?1, numero_documento = ?2, nombre = ?3, nombre_comercial = ?4, pais = ?5, departamento = ?6, provincia = ?7, distrito = ?8, direccion = ?9, telefono = ?10, correo = ?11, contacto = ?12, cuenta_bancaria = ?13, observaciones = ?14, activo = 1 WHERE id = ?15",
            params![
                payload.tipo_documento,
                payload.numero_documento,
                payload.nombre,
                payload.nombre_comercial,
                payload.pais,
                payload.departamento,
                payload.provincia,
                payload.distrito,
                payload.direccion,
                payload.telefono,
                payload.correo,
                payload.contacto,
                payload.cuenta_bancaria,
                payload.observaciones,
                id
            ],
        )
        .map_err(|e| e.to_string())?;

        return Ok(Vendor {
            id,
            tipo_documento: payload.tipo_documento,
            numero_documento: payload.numero_documento,
            nombre: payload.nombre,
            nombre_comercial: payload.nombre_comercial,
            pais: payload.pais,
            departamento: payload.departamento,
            provincia: payload.provincia,
            distrito: payload.distrito,
            direccion: payload.direccion,
            telefono: payload.telefono,
            correo: payload.correo,
            contacto: payload.contacto,
            cuenta_bancaria: payload.cuenta_bancaria,
            observaciones: payload.observaciones,
            activo: true,
        });
    }

    db.execute(
        "INSERT INTO proveedores (tipo_documento, numero_documento, nombre, nombre_comercial, pais, departamento, provincia, distrito, direccion, telefono, correo, contacto, cuenta_bancaria, observaciones) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            payload.tipo_documento,
            payload.numero_documento,
            payload.nombre,
            payload.nombre_comercial,
            payload.pais,
            payload.departamento,
            payload.provincia,
            payload.distrito,
            payload.direccion,
            payload.telefono,
            payload.correo,
            payload.contacto,
            payload.cuenta_bancaria,
            payload.observaciones
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();

    Ok(Vendor {
        id,
        tipo_documento: payload.tipo_documento,
        numero_documento: payload.numero_documento,
        nombre: payload.nombre,
        nombre_comercial: payload.nombre_comercial,
        pais: payload.pais,
        departamento: payload.departamento,
        provincia: payload.provincia,
        distrito: payload.distrito,
        direccion: payload.direccion,
        telefono: payload.telefono,
        correo: payload.correo,
        contacto: payload.contacto,
        cuenta_bancaria: payload.cuenta_bancaria,
        observaciones: payload.observaciones,
        activo: true,
    })
}

pub fn core_update_vendor(
    db: &Connection,
    id: i64,
    payload: CreateVendorPayload,
) -> Result<Vendor, String> {
    let conflict = db
        .query_row(
            "SELECT id FROM proveedores WHERE numero_documento = ?1 AND id != ?2 LIMIT 1",
            params![payload.numero_documento, id],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;

    if conflict.is_some() {
        return Err(format!(
            "Ya existe otro proveedor con el documento {}",
            payload.numero_documento
        ));
    }

    db.execute(
        "UPDATE proveedores SET tipo_documento = ?1, numero_documento = ?2, nombre = ?3, nombre_comercial = ?4, pais = ?5, departamento = ?6, provincia = ?7, distrito = ?8, direccion = ?9, telefono = ?10, correo = ?11, contacto = ?12, cuenta_bancaria = ?13, observaciones = ?14 WHERE id = ?15",
        params![
            payload.tipo_documento,
            payload.numero_documento,
            payload.nombre,
            payload.nombre_comercial,
            payload.pais,
            payload.departamento,
            payload.provincia,
            payload.distrito,
            payload.direccion,
            payload.telefono,
            payload.correo,
            payload.contacto,
            payload.cuenta_bancaria,
            payload.observaciones,
            id
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(Vendor {
        id,
        tipo_documento: payload.tipo_documento,
        numero_documento: payload.numero_documento,
        nombre: payload.nombre,
        nombre_comercial: payload.nombre_comercial,
        pais: payload.pais,
        departamento: payload.departamento,
        provincia: payload.provincia,
        distrito: payload.distrito,
        direccion: payload.direccion,
        telefono: payload.telefono,
        correo: payload.correo,
        contacto: payload.contacto,
        cuenta_bancaria: payload.cuenta_bancaria,
        observaciones: payload.observaciones,
        activo: true,
    })
}

pub fn core_delete_vendor(db: &Connection, id: i64) -> Result<(), String> {
    db.execute("UPDATE proveedores SET activo = 0 WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// --- TAURI COMMANDS ---

#[tauri::command]
pub fn get_vendors(state: State<'_, AppState>) -> Result<Vec<Vendor>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_vendors(db)
}

#[tauri::command]
pub fn create_vendor(payload: CreateVendorPayload, state: State<'_, AppState>) -> Result<Vendor, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_create_vendor(db, payload)
}

#[tauri::command]
pub fn update_vendor(
    id: i64,
    payload: CreateVendorPayload,
    state: State<'_, AppState>,
) -> Result<Vendor, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_update_vendor(db, id, payload)
}

#[tauri::command]
pub fn delete_vendor(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_delete_vendor(db, id)
}
