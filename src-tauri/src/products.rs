use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i64,
    pub internal_code: Option<String>,
    pub unit_code: String,
    pub name: String,
    pub sunat_code: Option<String>,
    pub gsl_code: Option<String>,
    pub currency: String,
    pub price_sale: f64,
    pub price_purchase: f64,
    pub stock_minimo: f64,
    pub afectacion_venta: String,
    pub afectacion_compra: String,
    pub has_icbper: bool,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub branch: Option<String>,
    pub stock_local: f64,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductPayload {
    pub internal_code: Option<String>,
    pub unit_code: String,
    pub name: String,
    pub sunat_code: Option<String>,
    pub gsl_code: Option<String>,
    pub currency: String,
    pub price_sale: f64,
    pub price_purchase: f64,
    pub stock_minimo: f64,
    pub afectacion_venta: String,
    pub afectacion_compra: String,
    pub has_icbper: bool,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub branch: Option<String>,
    pub stock_local: f64,
}

// --- CORE LOGIC ---

pub fn core_get_products(db: &Connection) -> Result<Vec<Product>, String> {
    let mut stmt = db
        .prepare(
            "SELECT id, internal_code, unit_code, name, sunat_code, gsl_code, currency, \
             price_sale, price_purchase, stock_minimo, afectacion_venta, afectacion_compra, \
             has_icbper, brand, category, branch, stock_local, is_active \
             FROM products WHERE is_active = 1 ORDER BY id DESC",
        )
        .map_err(|e| e.to_string())?;

    let product_iter = stmt
        .query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                internal_code: row.get(1)?,
                unit_code: row.get(2)?,
                name: row.get(3)?,
                sunat_code: row.get(4)?,
                gsl_code: row.get(5)?,
                currency: row.get(6)?,
                price_sale: row.get(7)?,
                price_purchase: row.get(8)?,
                stock_minimo: row.get(9)?,
                afectacion_venta: row.get(10)?,
                afectacion_compra: row.get(11)?,
                has_icbper: row.get(12)?,
                brand: row.get(13)?,
                category: row.get(14)?,
                branch: row.get(15)?,
                stock_local: row.get(16)?,
                is_active: row.get(17)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut products = Vec::new();
    for product in product_iter {
        products.push(product.map_err(|e| e.to_string())?);
    }
    Ok(products)
}

pub fn core_create_product(db: &Connection, payload: CreateProductPayload) -> Result<Product, String> {
    db.execute(
        "INSERT INTO products (internal_code, unit_code, name, sunat_code, gsl_code, currency, \
         price_sale, price_purchase, stock_minimo, afectacion_venta, afectacion_compra, \
         has_icbper, brand, category, branch, stock_local) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
        params![
            payload.internal_code,
            payload.unit_code,
            payload.name,
            payload.sunat_code,
            payload.gsl_code,
            payload.currency,
            payload.price_sale,
            payload.price_purchase,
            payload.stock_minimo,
            payload.afectacion_venta,
            payload.afectacion_compra,
            payload.has_icbper,
            payload.brand,
            payload.category,
            payload.branch,
            payload.stock_local,
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();

    Ok(Product {
        id,
        internal_code: payload.internal_code,
        unit_code: payload.unit_code,
        name: payload.name,
        sunat_code: payload.sunat_code,
        gsl_code: payload.gsl_code,
        currency: payload.currency,
        price_sale: payload.price_sale,
        price_purchase: payload.price_purchase,
        stock_minimo: payload.stock_minimo,
        afectacion_venta: payload.afectacion_venta,
        afectacion_compra: payload.afectacion_compra,
        has_icbper: payload.has_icbper,
        brand: payload.brand,
        category: payload.category,
        branch: payload.branch,
        stock_local: payload.stock_local,
        is_active: true,
    })
}

pub fn core_update_product(
    db: &Connection,
    id: i64,
    payload: CreateProductPayload,
) -> Result<Product, String> {
    db.execute(
        "UPDATE products SET internal_code = ?1, unit_code = ?2, name = ?3, sunat_code = ?4, \
         gsl_code = ?5, currency = ?6, price_sale = ?7, price_purchase = ?8, stock_minimo = ?9, \
         afectacion_venta = ?10, afectacion_compra = ?11, has_icbper = ?12, brand = ?13, \
         category = ?14, branch = ?15, stock_local = ?16 WHERE id = ?17",
        params![
            payload.internal_code,
            payload.unit_code,
            payload.name,
            payload.sunat_code,
            payload.gsl_code,
            payload.currency,
            payload.price_sale,
            payload.price_purchase,
            payload.stock_minimo,
            payload.afectacion_venta,
            payload.afectacion_compra,
            payload.has_icbper,
            payload.brand,
            payload.category,
            payload.branch,
            payload.stock_local,
            id,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(Product {
        id,
        internal_code: payload.internal_code,
        unit_code: payload.unit_code,
        name: payload.name,
        sunat_code: payload.sunat_code,
        gsl_code: payload.gsl_code,
        currency: payload.currency,
        price_sale: payload.price_sale,
        price_purchase: payload.price_purchase,
        stock_minimo: payload.stock_minimo,
        afectacion_venta: payload.afectacion_venta,
        afectacion_compra: payload.afectacion_compra,
        has_icbper: payload.has_icbper,
        brand: payload.brand,
        category: payload.category,
        branch: payload.branch,
        stock_local: payload.stock_local,
        is_active: true,
    })
}

pub fn core_delete_product(db: &Connection, id: i64) -> Result<(), String> {
    db.execute("UPDATE products SET is_active = 0 WHERE id = ?1", params![id])
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
