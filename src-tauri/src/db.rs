use rusqlite::{params, Connection, Result};
use tauri::{AppHandle, Manager};
use std::fs;
use crate::units::UNITS;

pub fn init_db(app: &AppHandle, password: &str) -> Result<Connection> {
    // Determinar la ruta de la base de datos. Usualmente en el directorio de datos de la app.
    let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
    }
    
    let db_path = app_dir.join("taxorium.db");
    
    let conn = Connection::open(&db_path)?;
    
    // Establecer la clave de encriptación primero
    conn.pragma_update(None, "key", &password)?;

    // Habilitar el modo WAL para mejor concurrencia y confiabilidad
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = NORMAL;"
    )?;

    // Probar la clave de encriptación leyendo el esquema (fallará si la clave es incorrecta)
    conn.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(()))?;

    run_migrations(&conn)?;
    seed_units(&conn)?;

    Ok(conn)
}

fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS series (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code TEXT NOT NULL UNIQUE,
            document_type TEXT NOT NULL,
            current_number INTEGER NOT NULL DEFAULT 1,
            is_active BOOLEAN NOT NULL DEFAULT 1
        );

        CREATE TABLE IF NOT EXISTS units (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code TEXT NOT NULL UNIQUE,
            description TEXT NOT NULL,
            symbol TEXT
        );

        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sku TEXT UNIQUE,
            name TEXT NOT NULL,
            unit_code TEXT NOT NULL,
            price REAL NOT NULL,
            currency TEXT NOT NULL DEFAULT 'PEN',
            is_active BOOLEAN NOT NULL DEFAULT 1,
            FOREIGN KEY(unit_code) REFERENCES units(code)
        );

        CREATE TABLE IF NOT EXISTS customers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            document_type TEXT NOT NULL,
            document_number TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            address TEXT,
            email TEXT,
            is_active BOOLEAN NOT NULL DEFAULT 1
        );
        "
    )?;

    Ok(())
}

fn seed_units(conn: &Connection) -> Result<()> {
    // Verificar si las unidades ya fueron sembradas para evitar trabajo innecesario
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM units", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    let mut stmt = conn.prepare("INSERT INTO units (code, description, symbol) VALUES (?1, ?2, ?3)")?;
    for unit in UNITS {
        stmt.execute(params![unit.code, unit.description, unit.symbol])?;
    }
    
    log::info!("Seeded {} units", UNITS.len());
    Ok(())
}
