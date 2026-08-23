use crate::AppState;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Serie {
    pub id: i64,
    pub codigo: String,
    pub tipo_documento: String,
    pub numero_actual: i64,
    pub activo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSeriePayload {
    pub codigo: String,
    pub tipo_documento: String,
    pub numero_actual: i64,
}

// --- CONSTANTES PARA DATOS INICIALES ---
pub const SERIES_DEFECTO: &[(&str, &str)] = &[
    ("F001", "01"), // Factura
    ("B001", "03"), // Boleta
];

/// Valida la serie según las reglas de SUNAT (ver apoyo/seriesDeComprobantes.png)
pub fn is_valid_serie(codigo: &str, tipo_documento: &str) -> bool {
    let codigo = codigo.to_uppercase();
    if codigo.len() != 4 {
        return false;
    }

    match tipo_documento {
        "01" => codigo.starts_with('F') || codigo == "E001", // Factura
        "03" => codigo.starts_with('B') || codigo == "EB01", // Boleta de Venta
        "09" => codigo.starts_with('T') || codigo == "EG07", // Guía Remitente
        "31" => codigo.starts_with('V') || codigo == "EG03", // Guía Transportista
        _ => {
            // Guía por evento usa EG05, pero dependerá del código que se asigne (no estándar para emisión normal)
            if codigo == "EG05" {
                return true;
            }
            false
        }
    }
}

pub fn core_get_series(db: &Connection) -> Result<Vec<Serie>, String> {
    let mut stmt = db
        .prepare("SELECT id, codigo, tipo_documento, numero_actual, activo FROM series WHERE activo = 1 ORDER BY id DESC")
        .map_err(|e| e.to_string())?;

    let series_iter = stmt
        .query_map([], |row| {
            Ok(Serie {
                id: row.get(0)?,
                codigo: row.get(1)?,
                tipo_documento: row.get(2)?,
                numero_actual: row.get(3)?,
                activo: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut series = Vec::new();
    for s in series_iter {
        series.push(s.map_err(|e| e.to_string())?);
    }
    Ok(series)
}

pub fn core_create_serie(db: &Connection, payload: CreateSeriePayload) -> Result<Serie, String> {
    if !is_valid_serie(&payload.codigo, &payload.tipo_documento) {
        return Err(format!(
            "La serie {} no es válida para el tipo de documento {}. Revisa los prefijos permitidos (F, B, T, V, E001, etc.)",
            payload.codigo, payload.tipo_documento
        ));
    }

    db.execute(
        "INSERT INTO series (codigo, tipo_documento, numero_actual) VALUES (?1, ?2, ?3)",
        params![
            payload.codigo.to_uppercase(),
            payload.tipo_documento,
            payload.numero_actual
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();
    Ok(Serie {
        id,
        codigo: payload.codigo.to_uppercase(),
        tipo_documento: payload.tipo_documento,
        numero_actual: payload.numero_actual,
        activo: true,
    })
}

pub fn core_update_serie(db: &Connection, id: i64, payload: CreateSeriePayload) -> Result<Serie, String> {
    if !is_valid_serie(&payload.codigo, &payload.tipo_documento) {
        return Err(format!(
            "La serie {} no es válida para el tipo de documento {}. Revisa los prefijos permitidos.",
            payload.codigo, payload.tipo_documento
        ));
    }

    db.execute(
        "UPDATE series SET codigo = ?1, tipo_documento = ?2, numero_actual = ?3 WHERE id = ?4",
        params![
            payload.codigo.to_uppercase(),
            payload.tipo_documento,
            payload.numero_actual,
            id
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(Serie {
        id,
        codigo: payload.codigo.to_uppercase(),
        tipo_documento: payload.tipo_documento,
        numero_actual: payload.numero_actual,
        activo: true,
    })
}

pub fn core_delete_serie(db: &Connection, id: i64) -> Result<(), String> {
    db.execute("UPDATE series SET activo = 0 WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// --- TAURI COMMANDS ---

#[tauri::command]
pub fn get_series(state: State<'_, AppState>) -> Result<Vec<Serie>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_series(db)
}

#[tauri::command]
pub fn create_serie(payload: CreateSeriePayload, state: State<'_, AppState>) -> Result<Serie, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_create_serie(db, payload)
}

#[tauri::command]
pub fn update_serie(id: i64, payload: CreateSeriePayload, state: State<'_, AppState>) -> Result<Serie, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_update_serie(db, id, payload)
}

#[tauri::command]
pub fn delete_serie(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_delete_serie(db, id)
}
