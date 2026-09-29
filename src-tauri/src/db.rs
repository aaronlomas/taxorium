use crate::emit;
use crate::models::afectaciones::{DESTINO_AFECTACION_COMPRAS, TIPOS_AFECTACION_VENTAS};
use crate::models::branch::SEDES;
use crate::models::catalogos::{TIPOS_COMPROBANTE, TIPOS_OPERACION, TIPOS_PAGO};
use crate::models::currency::TIPO_MONEDA;
use crate::models::series::SERIES_DEFECTO;
use crate::models::units::UNITS;
use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};
use std::fs;
use tauri::{AppHandle, Manager};

fn obtener_migraciones() -> Migrations<'static> {
    Migrations::new(vec![M::up(include_str!(
        "../migrations/01_esquema_inicial.sql"
    ))])
}

#[cfg(test)]
mod tests {
    use super::obtener_migraciones;

    #[test]
    fn migraciones_validas() {
        let migraciones = obtener_migraciones();
        assert!(migraciones.validate().is_ok());
    }

    #[test]
    fn migraciones_aplican_en_memoria() {
        let migraciones = obtener_migraciones();
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        migraciones.to_latest(&mut conn).unwrap();
    }
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

    conn.pragma_update(None, "key", &password)
        .map_err(|e| e.to_string())?;

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
    seed_afectaciones(&mut conn, app)?;
    seed_tipos_operacion(&mut conn, app)?;
    seed_tipos_pago(&mut conn, app)?;
    seed_tipos_comprobante(&mut conn, app)?;
    seed_series(&mut conn, app)?;

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

    emit::info(
        app,
        "db",
        format!("Se sembraron {} monedas", TIPO_MONEDA.len()),
    );
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

fn seed_afectaciones(conn: &mut Connection, app: &AppHandle) -> Result<(), String> {
    // 1. Sembrar Afectaciones de Venta
    let count_v: i64 = conn
        .query_row("SELECT COUNT(*) FROM afectaciones_venta", [], |row| {
            row.get(0)
        })
        .unwrap_or(0);

    if count_v == 0 {
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        {
            let mut stmt = tx
                .prepare("INSERT INTO afectaciones_venta (codigo, descripcion) VALUES (?1, ?2)")
                .map_err(|e| e.to_string())?;

            for item in TIPOS_AFECTACION_VENTAS {
                stmt.execute(params![item.codigo, item.descripcion])
                    .map_err(|e| e.to_string())?;
            }
        }
        tx.commit().map_err(|e| e.to_string())?;
    }

    // 2. Sembrar Afectaciones de Compra
    let count_c: i64 = conn
        .query_row("SELECT COUNT(*) FROM afectaciones_compra", [], |row| {
            row.get(0)
        })
        .unwrap_or(0);

    if count_c == 0 {
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        {
            let mut stmt = tx
                .prepare("INSERT INTO afectaciones_compra (codigo, descripcion) VALUES (?1, ?2)")
                .map_err(|e| e.to_string())?;

            for item in DESTINO_AFECTACION_COMPRAS {
                stmt.execute(params![item.codigo, item.descripcion])
                    .map_err(|e| e.to_string())?;
            }
        }
        tx.commit().map_err(|e| e.to_string())?;
    }

    emit::info(
        app,
        "db",
        "Se sembraron los catálogos de afectación de ventas y compras SUNAT".to_string(),
    );
    Ok(())
}

fn seed_tipos_operacion(conn: &mut Connection, app: &AppHandle) -> Result<(), String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM tipos_operacion", [], |row| row.get(0))
        .unwrap_or(0);

    if count > 0 {
        return Ok(());
    }

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    {
        let mut stmt = tx
            .prepare("INSERT INTO tipos_operacion (codigo, descripcion) VALUES (?1, ?2)")
            .map_err(|e| e.to_string())?;

        for item in TIPOS_OPERACION {
            stmt.execute(params![item.0, item.1])
                .map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;

    emit::info(
        app,
        "db",
        format!("Se sembraron {} tipos de operación", TIPOS_OPERACION.len()),
    );
    Ok(())
}

fn seed_tipos_pago(conn: &mut Connection, app: &AppHandle) -> Result<(), String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM tipos_pago", [], |row| row.get(0))
        .unwrap_or(0);

    if count > 0 {
        return Ok(());
    }

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    {
        let mut stmt = tx
            .prepare("INSERT INTO tipos_pago (codigo, descripcion) VALUES (?1, ?2)")
            .map_err(|e| e.to_string())?;

        for item in TIPOS_PAGO {
            stmt.execute(params![item.0, item.1])
                .map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;

    emit::info(
        app,
        "db",
        format!("Se sembraron {} tipos de pago", TIPOS_PAGO.len()),
    );
    Ok(())
}

fn seed_tipos_comprobante(conn: &mut Connection, app: &AppHandle) -> Result<(), String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM tipos_comprobante", [], |row| {
            row.get(0)
        })
        .unwrap_or(0);

    if count > 0 {
        return Ok(());
    }

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    {
        let mut stmt = tx
            .prepare("INSERT INTO tipos_comprobante (codigo, descripcion) VALUES (?1, ?2)")
            .map_err(|e| e.to_string())?;

        for item in TIPOS_COMPROBANTE {
            stmt.execute(params![item.0, item.1])
                .map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;

    emit::info(
        app,
        "db",
        format!(
            "Se sembraron {} tipos de comprobante",
            TIPOS_COMPROBANTE.len()
        ),
    );
    Ok(())
}

fn seed_series(conn: &mut Connection, app: &AppHandle) -> Result<(), String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM series", [], |row| row.get(0))
        .unwrap_or(0);

    if count > 0 {
        return Ok(());
    }

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    {
        let mut stmt = tx
            .prepare(
                "INSERT INTO series (codigo, tipo_documento, numero_actual) VALUES (?1, ?2, ?3)",
            )
            .map_err(|e| e.to_string())?;

        for item in SERIES_DEFECTO {
            stmt.execute(params![item.0, item.1, 1])
                .map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;

    emit::info(
        app,
        "db",
        format!("Se sembraron {} series por defecto", SERIES_DEFECTO.len()),
    );
    Ok(())
}
