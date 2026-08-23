use crate::AppState;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CatalogoMoneda {
    pub codigo: String,
    pub descripcion: String,
    pub simbolo: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CatalogoUnidad {
    pub codigo: String,
    pub descripcion: String,
    pub simbolo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CatalogoSede {
    pub codigo: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CatalogoAfectacion {
    pub codigo: String,
    pub descripcion: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CatalogoTipoOperacion {
    pub codigo: String,
    pub descripcion: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CatalogoTipoPago {
    pub codigo: String,
    pub descripcion: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CatalogoTipoComprobante {
    pub codigo: String,
    pub descripcion: String,
}

// --- CONSTANTES PARA DATOS INICIALES ---

pub const TIPOS_OPERACION: &[(&str, &str)] = &[
    ("0101", "Venta Interna"),
    ("0102", "Venta Interna - Anticipos"),
];

pub const TIPOS_PAGO: &[(&str, &str)] = &[
    ("Contado", "Contado"),
    ("Credito", "Crédito"),
];

pub const TIPOS_COMPROBANTE: &[(&str, &str)] = &[
    ("01", "Factura Electrónica"),
    ("03", "Boleta de Venta Electrónica"),
];

// --- CORE LOGIC ---

pub fn core_get_monedas(db: &Connection) -> Result<Vec<CatalogoMoneda>, String> {
    let mut stmt = db
        .prepare("SELECT codigo, descripcion, simbolo FROM monedas WHERE activo = 1 ORDER BY codigo")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CatalogoMoneda {
                codigo: row.get(0)?,
                descripcion: row.get(1)?,
                simbolo: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut monedas = Vec::new();
    for row in rows {
        monedas.push(row.map_err(|e| e.to_string())?);
    }
    Ok(monedas)
}

pub fn core_get_unidades(db: &Connection) -> Result<Vec<CatalogoUnidad>, String> {
    let mut stmt = db
        .prepare("SELECT codigo, descripcion, simbolo FROM unidades ORDER BY codigo")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CatalogoUnidad {
                codigo: row.get(0)?,
                descripcion: row.get(1)?,
                simbolo: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut unidades = Vec::new();
    for row in rows {
        unidades.push(row.map_err(|e| e.to_string())?);
    }
    Ok(unidades)
}

pub fn core_get_sedes(db: &Connection) -> Result<Vec<CatalogoSede>, String> {
    let mut stmt = db
        .prepare("SELECT codigo, label FROM sedes WHERE activo = 1 ORDER BY codigo")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CatalogoSede {
                codigo: row.get(0)?,
                label: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut sedes = Vec::new();
    for row in rows {
        sedes.push(row.map_err(|e| e.to_string())?);
    }
    Ok(sedes)
}

pub fn core_get_afectaciones_venta(db: &Connection) -> Result<Vec<CatalogoAfectacion>, String> {
    let mut stmt = db
        .prepare("SELECT codigo, descripcion FROM afectaciones_venta ORDER BY codigo")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CatalogoAfectacion {
                codigo: row.get(0)?,
                descripcion: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| e.to_string())?);
    }
    Ok(items)
}

pub fn core_get_afectaciones_compra(db: &Connection) -> Result<Vec<CatalogoAfectacion>, String> {
    let mut stmt = db
        .prepare("SELECT codigo, descripcion FROM afectaciones_compra ORDER BY codigo")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CatalogoAfectacion {
                codigo: row.get(0)?,
                descripcion: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| e.to_string())?);
    }
    Ok(items)
}

// --- TAURI COMMANDS ---

#[tauri::command]
pub fn get_monedas(state: State<'_, AppState>) -> Result<Vec<CatalogoMoneda>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_monedas(db)
}

#[tauri::command]
pub fn get_unidades(state: State<'_, AppState>) -> Result<Vec<CatalogoUnidad>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_unidades(db)
}

#[tauri::command]
pub fn get_sedes(state: State<'_, AppState>) -> Result<Vec<CatalogoSede>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_sedes(db)
}

#[tauri::command]
pub fn get_afectaciones_venta(state: State<'_, AppState>) -> Result<Vec<CatalogoAfectacion>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_afectaciones_venta(db)
}

#[tauri::command]
pub fn get_afectaciones_compra(
    state: State<'_, AppState>,
) -> Result<Vec<CatalogoAfectacion>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_afectaciones_compra(db)
}

pub fn core_get_tipos_operacion(db: &Connection) -> Result<Vec<CatalogoTipoOperacion>, String> {
    let mut stmt = db
        .prepare("SELECT codigo, descripcion FROM tipos_operacion ORDER BY codigo")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CatalogoTipoOperacion {
                codigo: row.get(0)?,
                descripcion: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| e.to_string())?);
    }
    Ok(items)
}

pub fn core_get_tipos_pago(db: &Connection) -> Result<Vec<CatalogoTipoPago>, String> {
    let mut stmt = db
        .prepare("SELECT codigo, descripcion FROM tipos_pago ORDER BY codigo")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CatalogoTipoPago {
                codigo: row.get(0)?,
                descripcion: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| e.to_string())?);
    }
    Ok(items)
}

pub fn core_get_tipos_comprobante(db: &Connection) -> Result<Vec<CatalogoTipoComprobante>, String> {
    let mut stmt = db
        .prepare("SELECT codigo, descripcion FROM tipos_comprobante ORDER BY codigo")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CatalogoTipoComprobante {
                codigo: row.get(0)?,
                descripcion: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| e.to_string())?);
    }
    Ok(items)
}

#[tauri::command]
pub fn get_tipos_operacion(state: State<'_, AppState>) -> Result<Vec<CatalogoTipoOperacion>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_tipos_operacion(db)
}

#[tauri::command]
pub fn get_tipos_pago(state: State<'_, AppState>) -> Result<Vec<CatalogoTipoPago>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_tipos_pago(db)
}

#[tauri::command]
pub fn get_tipos_comprobante(state: State<'_, AppState>) -> Result<Vec<CatalogoTipoComprobante>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_tipos_comprobante(db)
}
