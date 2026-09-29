use crate::AppState;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i64,
    pub codigo_interno: Option<String>,
    pub codigo_unidad: String,
    pub nombre: String,
    pub codigo_sunat: Option<String>,
    pub codigo_gsl: Option<String>,
    pub moneda: String,
    pub precio_unitario_venta: f64,
    pub precio_unitario_compra: f64,
    pub stock_minimo: f64,
    pub afectacion_venta: String,
    pub afectacion_compra: String,
    pub tiene_icbper: bool,
    pub marca: Option<String>,
    pub categoria: Option<String>,
    pub codigo_sede: Option<String>,
    pub activo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductPayload {
    pub codigo_interno: Option<String>,
    pub codigo_unidad: String,
    pub nombre: String,
    pub codigo_sunat: Option<String>,
    pub codigo_gsl: Option<String>,
    pub moneda: String,
    pub precio_unitario_venta: f64,
    pub precio_unitario_compra: f64,
    pub stock_minimo: f64,
    pub afectacion_venta: String,
    pub afectacion_compra: String,
    pub tiene_icbper: bool,
    pub marca: Option<String>,
    pub categoria: Option<String>,
    pub codigo_sede: Option<String>,
}

// --- CORE LOGIC ---

pub fn core_get_products(db: &Connection) -> Result<Vec<Product>, String> {
    let mut stmt = db
        .prepare(
            "SELECT id, codigo_interno, codigo_unidad, nombre, codigo_sunat, codigo_gsl, moneda, \
             precio_unitario_venta, precio_unitario_compra, stock_minimo, afectacion_venta, afectacion_compra, \
             tiene_icbper, marca, categoria, codigo_sede, activo \
             FROM productos WHERE activo = 1 ORDER BY id DESC",
        )
        .map_err(|e| e.to_string())?;

    let product_iter = stmt
        .query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                codigo_interno: row.get(1)?,
                codigo_unidad: row.get(2)?,
                nombre: row.get(3)?,
                codigo_sunat: row.get(4)?,
                codigo_gsl: row.get(5)?,
                moneda: row.get(6)?,
                precio_unitario_venta: row.get(7)?,
                precio_unitario_compra: row.get(8)?,
                stock_minimo: row.get(9)?,
                afectacion_venta: row.get(10)?,
                afectacion_compra: row.get(11)?,
                tiene_icbper: row.get(12)?,
                marca: row.get(13)?,
                categoria: row.get(14)?,
                codigo_sede: row.get(15)?,
                activo: row.get(16)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut products = Vec::new();
    for product in product_iter {
        products.push(product.map_err(|e| e.to_string())?);
    }
    Ok(products)
}

pub fn core_create_product(
    db: &Connection,
    payload: CreateProductPayload,
) -> Result<Product, String> {
    db.execute(
        "INSERT INTO productos (codigo_interno, codigo_unidad, nombre, codigo_sunat, codigo_gsl, moneda, \
         precio_unitario_venta, precio_unitario_compra, stock_minimo, afectacion_venta, afectacion_compra, \
         tiene_icbper, marca, categoria, codigo_sede) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        params![
            payload.codigo_interno,
            payload.codigo_unidad,
            payload.nombre,
            payload.codigo_sunat,
            payload.codigo_gsl,
            payload.moneda,
            payload.precio_unitario_venta,
            payload.precio_unitario_compra,
            payload.stock_minimo,
            payload.afectacion_venta,
            payload.afectacion_compra,
            payload.tiene_icbper,
            payload.marca,
            payload.categoria,
            payload.codigo_sede,
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();

    Ok(Product {
        id,
        codigo_interno: payload.codigo_interno,
        codigo_unidad: payload.codigo_unidad,
        nombre: payload.nombre,
        codigo_sunat: payload.codigo_sunat,
        codigo_gsl: payload.codigo_gsl,
        moneda: payload.moneda,
        precio_unitario_venta: payload.precio_unitario_venta,
        precio_unitario_compra: payload.precio_unitario_compra,
        stock_minimo: payload.stock_minimo,
        afectacion_venta: payload.afectacion_venta,
        afectacion_compra: payload.afectacion_compra,
        tiene_icbper: payload.tiene_icbper,
        marca: payload.marca,
        categoria: payload.categoria,
        codigo_sede: payload.codigo_sede,
        activo: true,
    })
}

pub fn core_update_product(
    db: &Connection,
    id: i64,
    payload: CreateProductPayload,
) -> Result<Product, String> {
    db.execute(
        "UPDATE productos SET codigo_interno = ?1, codigo_unidad = ?2, nombre = ?3, codigo_sunat = ?4, \
         codigo_gsl = ?5, moneda = ?6, precio_unitario_venta = ?7, precio_unitario_compra = ?8, stock_minimo = ?9, \
         afectacion_venta = ?10, afectacion_compra = ?11, tiene_icbper = ?12, marca = ?13, \
         categoria = ?14, codigo_sede = ?15 WHERE id = ?16",
        params![
            payload.codigo_interno,
            payload.codigo_unidad,
            payload.nombre,
            payload.codigo_sunat,
            payload.codigo_gsl,
            payload.moneda,
            payload.precio_unitario_venta,
            payload.precio_unitario_compra,
            payload.stock_minimo,
            payload.afectacion_venta,
            payload.afectacion_compra,
            payload.tiene_icbper,
            payload.marca,
            payload.categoria,
            payload.codigo_sede,
            id,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(Product {
        id,
        codigo_interno: payload.codigo_interno,
        codigo_unidad: payload.codigo_unidad,
        nombre: payload.nombre,
        codigo_sunat: payload.codigo_sunat,
        codigo_gsl: payload.codigo_gsl,
        moneda: payload.moneda,
        precio_unitario_venta: payload.precio_unitario_venta,
        precio_unitario_compra: payload.precio_unitario_compra,
        stock_minimo: payload.stock_minimo,
        afectacion_venta: payload.afectacion_venta,
        afectacion_compra: payload.afectacion_compra,
        tiene_icbper: payload.tiene_icbper,
        marca: payload.marca,
        categoria: payload.categoria,
        codigo_sede: payload.codigo_sede,
        activo: true,
    })
}

pub fn core_delete_product(db: &Connection, id: i64) -> Result<(), String> {
    db.execute("UPDATE productos SET activo = 0 WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// --- TAURI COMMANDS ---

#[tauri::command]
pub fn get_products(state: State<'_, AppState>) -> Result<Vec<Product>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_products(db)
}

#[tauri::command]
pub fn create_product(
    payload: CreateProductPayload,
    state: State<'_, AppState>,
) -> Result<Product, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_create_product(db, payload)
}

#[tauri::command]
pub fn update_product(
    id: i64,
    payload: CreateProductPayload,
    state: State<'_, AppState>,
) -> Result<Product, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_update_product(db, id, payload)
}

#[tauri::command]
pub fn delete_product(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_delete_product(db, id)
}
