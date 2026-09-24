use crate::AppState;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

use super::series::is_valid_serie;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Voucher {
    pub id: i64,
    pub fecha_de_emision: String,
    pub cliente: String,
    pub numero_comprobante: String,
    pub estado_validez: String,
    pub estado_pago: String,
    pub moneda: String,
    pub gravado: f64,
    pub igv: f64,
    pub total: f64,
    pub estado: bool,
    pub hash_cpe: Option<String>,
    pub estado_sunat: i64,
    pub codigo_cdr: Option<String>,
    pub descripcion_cdr: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVoucherPayload {
    pub fecha_de_emision: String,
    pub cliente: String,
    pub numero_comprobante: String,
    pub serie: String,
    pub correlativo: i64,
    pub tipo_comprobante: String,
    pub moneda: String,
    pub gravado: f64,
    pub igv: f64,
    pub total: f64,
    pub estado_pago: Option<String>,
}

// --- CORE LOGIC ---

/// Devuelve el siguiente correlativo disponible para una serie, tomado de la
/// tabla `series.numero_actual` (fuente de verdad para la numeración).
pub fn core_next_correlativo(db: &Connection, serie: &str) -> Result<i64, String> {
    db.query_row(
        "SELECT numero_actual FROM series WHERE codigo = ?1 AND activo = 1",
        params![serie.to_uppercase()],
        |row| row.get(0),
    )
    .map_err(|e| format!("La serie {serie} no existe en la base de datos: {e}"))
}

pub fn core_get_vouchers(db: &Connection) -> Result<Vec<Voucher>, String> {
    let mut stmt = db
        .prepare(
            "SELECT id, fecha_de_emision, cliente, numero_comprobante, estado_validez, estado_pago, moneda, gravado, igv, total, estado, hash_cpe, estado_sunat, codigo_cdr, descripcion_cdr FROM comprobantes WHERE estado = 1 ORDER BY numero_comprobante DESC",
        )
        .map_err(|e| e.to_string())?;

    let voucher_iter = stmt
        .query_map([], |row| {
            Ok(Voucher {
                id: row.get(0)?,
                fecha_de_emision: row.get(1)?,
                cliente: row.get(2)?,
                numero_comprobante: row.get(3)?,
                estado_validez: row.get(4)?,
                estado_pago: row.get(5)?,
                moneda: row.get(6)?,
                gravado: row.get(7)?,
                igv: row.get(8)?,
                total: row.get(9)?,
                estado: row.get(10)?,
                hash_cpe: row.get(11)?,
                estado_sunat: row.get(12)?,
                codigo_cdr: row.get(13)?,
                descripcion_cdr: row.get(14)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut vouchers = Vec::new();
    for voucher in voucher_iter {
        vouchers.push(voucher.map_err(|e| e.to_string())?);
    }
    Ok(vouchers)
}

/// Inserta un comprobante en la tabla `comprobantes` y avanza el correlativo de
/// su serie de forma atómica (transacción).
pub fn core_create_voucher(db: &mut Connection, payload: CreateVoucherPayload) -> Result<Voucher, String> {
    if !is_valid_serie(&payload.serie, &payload.tipo_comprobante) {
        return Err(format!(
            "La serie {} no es válida para el tipo de comprobante {}.",
            payload.serie, payload.tipo_comprobante
        ));
    }

    let tx = db.transaction().map_err(|e| e.to_string())?;

    let estado_pago = payload.estado_pago.unwrap_or_else(|| "pendiente".to_string());
    if !matches!(estado_pago.as_str(), "pendiente" | "pagado") {
        return Err(format!("Estado de pago inválido: {estado_pago}"));
    }

    tx.execute(
        "INSERT INTO comprobantes (fecha_de_emision, cliente, numero_comprobante, estado_pago, moneda, gravado, igv, total) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            payload.fecha_de_emision,
            payload.cliente,
            payload.numero_comprobante,
            estado_pago,
            payload.moneda,
            payload.gravado,
            payload.igv,
            payload.total
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = tx.last_insert_rowid();

    // Avanza el número actual de la serie si el correlativo emitido lo supera.
    tx.execute(
        "UPDATE series SET numero_actual = ?1 WHERE codigo = ?2 AND numero_actual <= ?1",
        params![payload.correlativo + 1, payload.serie.to_uppercase()],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok(Voucher {
        id,
        fecha_de_emision: payload.fecha_de_emision,
        cliente: payload.cliente,
        numero_comprobante: payload.numero_comprobante,
        estado_validez: "registrado".to_string(),
        estado_pago,
        moneda: payload.moneda,
        gravado: payload.gravado,
        igv: payload.igv,
        total: payload.total,
        estado: true,
        hash_cpe: None,
        estado_sunat: 0,
        codigo_cdr: None,
        descripcion_cdr: None,
    })
}

// --- TAURI COMMANDS ---

#[tauri::command]
pub fn get_next_correlativo(serie: String, state: State<'_, AppState>) -> Result<i64, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_next_correlativo(db, &serie)
}

#[tauri::command]
pub fn get_vouchers(state: State<'_, AppState>) -> Result<Vec<Voucher>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_vouchers(db)
}

#[tauri::command]
pub fn create_voucher(
    payload: CreateVoucherPayload,
    state: State<'_, AppState>,
) -> Result<Voucher, String> {
    let mut db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_mut().ok_or("Database not initialized.")?;
    core_create_voucher(db, payload)
}