use crate::emit;
use crate::monedas::TIPO_MONEDA;
use crate::sedes::SEDES;
use crate::units::UNITS;
use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};
use std::fs;
use tauri::{AppHandle, Manager};

fn obtener_migraciones() -> Migrations<'static> {
    Migrations::new(vec![
        M::up(include_str!("../migrations/01_esquema_inicial.sql")),
    ])
}

pub fn init_db(app: &AppHandle, password: &str) -> Result<Connection, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| {
        let msg = format!("No se pudo obtener el directorio de datos de la app: {e}");
        emit::error(app, "db", &msg);
        msg
    })?;

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }

    let db_path = app_dir.join("taxorium.db");
    let mut conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    conn.pragma_update(None, "key", &password).map_err(|e| e.to_string())?;

    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = NORMAL;
         PRAGMA foreign_keys = ON;",
    )
    .map_err(|e| e.to_string())?;

    // Validar encriptación
    conn.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(()))
        .map_err(|_| "Clave de encriptación incorrecta".to_string())?;

    // Ejecutar migraciones y sembrados
    run_migrations(&mut conn, app)?;
    seed_monedas(&mut conn, app)?;
    seed_units(&mut conn, app)?;
    seed_sedes(&mut conn, app)?;

    Ok(conn)
}

fn run_migrations(conn: &mut Connection, app: &AppHandle) -> Result<(), String> {
    let migraciones = obtener_migraciones();
    migraciones.to_latest(conn).map_err(|e| {
        let msg = format!("Error ejecutando migraciones iniciales: {e}");
        emit::error(app, "db", &msg);
        msg
    })?;
    Ok(())
}

fn seed_monedas(conn: &mut Connection, app: &AppHandle) -> Result<(), String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM monedas", [], |row| row.get(0))
        .unwrap_or(0);

    if count > 0 {
        return Ok(());
    }

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    {
        let mut stmt = tx
            .prepare("INSERT INTO monedas (codigo, descripcion, simbolo) VALUES (?1, ?2, ?3)")
            .map_err(|e| e.to_string())?;

        for moneda in TIPO_MONEDA {
            stmt.execute(params![moneda.codigo, moneda.descripcion, moneda.simbolo])
                .map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;

    emit::info(app, "db", format!("Se sembraron {} monedas", TIPO_MONEDA.len()));
    Ok(())
}

fn seed_units(conn: &mut Connection, app: &AppHandle) -> Result<(), String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM unidades", [], |row| row.get(0))
        .unwrap_or(0);

    if count > 0 {
        return Ok(());
    }

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    {
        let mut stmt = tx
            .prepare("INSERT INTO unidades (codigo, descripcion, simbolo) VALUES (?1, ?2, ?3)")
            .map_err(|e| e.to_string())?;

        for unit in UNITS {
            stmt.execute(params![unit.codigo, unit.descripcion, unit.simbolo])
                .map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;

    emit::info(app, "db", format!("Se sembraron {} unidades", UNITS.len()));
    Ok(())
}

fn seed_sedes(conn: &mut Connection, app: &AppHandle) -> Result<(), String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM sedes", [], |row| row.get(0))
        .unwrap_or(0);

    if count > 0 {
        return Ok(());
    }

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    {
        let mut stmt = tx
            .prepare("INSERT INTO sedes (codigo, label) VALUES (?1, ?2)")
            .map_err(|e| e.to_string())?;

        for sede in SEDES {
            stmt.execute(params![sede.codigo, sede.label])
                .map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;

    emit::info(app, "db", format!("Se sembraron {} sedes", SEDES.len()));
    Ok(())
}