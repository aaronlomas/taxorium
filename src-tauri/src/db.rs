use rusqlite::{params, Connection};
use tauri::{AppHandle, Manager};
use std::fs;
use crate::units::UNITS;
use crate::emit;

pub fn init_db(app: &AppHandle, password: &str) -> Result<Connection, String> {
    // Determinar la ruta de la base de datos. Usualmente en el directorio de datos de la app.
    let app_dir = match app.path().app_data_dir() {
        Ok(dir) => dir,
        Err(e) => {
            let msg = format!("No se pudo obtener el directorio de datos de la app: {e}");
            emit::error(app, "db", &msg);
            return Err(msg);
        }
    };

    if !app_dir.exists() {
        if let Err(e) = fs::create_dir_all(&app_dir) {
            let msg = format!("No se pudo crear el directorio de datos de la app: {e}");
            emit::error(app, "db", &msg);
            return Err(msg);
        }
    }
    
    let db_path = app_dir.join("taxorium.db");
    
    let conn = Connection::open(&db_path).map_err(|e| {
        let msg = format!("No se pudo abrir la conexión a la base de datos: {e}");
        emit::error(app, "db", &msg);
        msg
    })?;
    
    // Establecer la clave de encriptación primero
    conn.pragma_update(None, "key", &password).map_err(|e| {
        let msg = format!("Error al establecer clave de encriptación: {e}");
        emit::error(app, "db", &msg);
        msg
    })?;

    // Habilitar el modo WAL para mejor concurrencia y confiabilidad
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = NORMAL;"
    ).map_err(|e| {
        let msg = format!("Error configurando pragmas: {e}");
        emit::error(app, "db", &msg);
        msg
    })?;

    // Probar la clave de encriptación leyendo el esquema (fallará si la clave es incorrecta)
    conn.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(())).map_err(|e| {
        let msg = format!("Fallo comprobando clave de encriptación (esquema inaccesible): {e}");
        emit::error(app, "db", &msg);
        msg
    })?;

    run_migrations(&conn, app)?;
    seed_units(&conn, app)?;

    Ok(conn)
}

fn run_migrations(conn: &Connection, app: &AppHandle) -> Result<(), String> {
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

        CREATE TABLE IF NOT EXISTS sellers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT,
            last_name TEXT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            accesses TEXT,
            domain TEXT,
            is_active BOOLEAN NOT NULL DEFAULT 1
        );
        "
    ).map_err(|e| {
        let msg = format!("Error ejecutando migraciones iniciales: {e}");
        emit::error(app, "db", &msg);
        msg
    })?;

    // Safe migration: add columns if table already existed before this update
    let _ = conn.execute("ALTER TABLE sellers ADD COLUMN first_name TEXT", []);
    let _ = conn.execute("ALTER TABLE sellers ADD COLUMN last_name TEXT", []);

    Ok(())
}

fn seed_units(conn: &Connection, app: &AppHandle) -> Result<(), String> {
    // Verificar si las unidades ya fueron sembradas para evitar trabajo innecesario
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM units", [], |row| row.get(0)).unwrap_or(0);
    if count > 0 {
        return Ok(());
    }

    let mut stmt = conn.prepare("INSERT INTO units (code, description, symbol) VALUES (?1, ?2, ?3)").map_err(|e| {
         let msg = format!("Error preparando inserción de unidades: {e}");
         emit::error(app, "db", &msg);
         msg
    })?;
    
    for unit in UNITS {
        if let Err(e) = stmt.execute(params![unit.code, unit.description, unit.symbol]) {
             let msg = format!("Error insertando unidad {}: {e}", unit.code);
             emit::error(app, "db", &msg);
             return Err(msg);
        }
    }
    
    emit::info(app, "db", format!("Se sembraron {} unidades", UNITS.len()));
    log::info!("Seeded {} units", UNITS.len());
    Ok(())
}
