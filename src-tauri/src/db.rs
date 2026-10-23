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
    Migrations::new(vec![
        M::up(include_str!("../migrations/01_esquema_inicial.sql")),
        // Migración 2: Amplía el CHECK constraint de estado_validez para incluir
        // 'anulado'. SQLite no permite ALTER COLUMN, por lo que se usa el patrón
        // estándar: renombrar → recrear → copiar → eliminar temporal.
        M::up(
            "PRAGMA foreign_keys = OFF;
             ALTER TABLE comprobantes RENAME TO _comprobantes_old;
             CREATE TABLE comprobantes (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 fecha_de_emision TEXT NOT NULL,
                 cliente TEXT NOT NULL,
                 numero_comprobante TEXT NOT NULL UNIQUE,
                 estado_validez TEXT NOT NULL DEFAULT 'registrado'
                     CHECK (estado_validez IN ('registrado', 'rechazado', 'aceptado', 'anulado')),
                 estado_pago TEXT NOT NULL DEFAULT 'pendiente'
                     CHECK (estado_pago IN ('pendiente', 'pagado')),
                 xml_firmado TEXT,
                 moneda TEXT NOT NULL DEFAULT 'PEN',
                 tipo_cambio REAL,
                 gravado REAL NOT NULL DEFAULT 0,
                 igv REAL NOT NULL DEFAULT 0,
                 total REAL NOT NULL DEFAULT 0,
                 forma_pago TEXT NOT NULL DEFAULT 'Contado' REFERENCES tipos_pago(codigo),
                 monto_neto_pendiente REAL,
                 fecha_vencimiento TEXT,
                 estado BOOLEAN NOT NULL DEFAULT 1,
                 hash_cpe TEXT,
                 estado_sunat INTEGER NOT NULL DEFAULT 0,
                 codigo_cdr TEXT,
                 descripcion_cdr TEXT,
                 xml_cdr TEXT
             );
             INSERT INTO comprobantes SELECT * FROM _comprobantes_old;
             DROP TABLE _comprobantes_old;
             CREATE INDEX IF NOT EXISTS idx_comprobantes_numero ON comprobantes(numero_comprobante);
             CREATE INDEX IF NOT EXISTS idx_comprobantes_estado_validez ON comprobantes(estado_validez);
             PRAGMA foreign_keys = ON;",
        ),
        // Migración 3: Tabla anulaciones para almacenar historial, tickets y
        // CDRs devueltos por SUNAT al procesar una Comunicación de Baja o anulación.
        M::up(
            "DROP TABLE IF EXISTS anulaciones;
             CREATE TABLE anulaciones (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 comprobante_id INTEGER NOT NULL REFERENCES comprobantes(id),
                 tipo TEXT NOT NULL CHECK (tipo IN ('baja', 'nota_credito', 'local')),
                 motivo TEXT NOT NULL,
                 id_baja TEXT,
                 ticket TEXT,
                 codigo_cdr TEXT,
                 descripcion_cdr TEXT,
                 xml_cdr TEXT,
                 xml_baja_firmado TEXT,
                 fecha_anulacion TEXT NOT NULL DEFAULT (datetime('now'))
             );
             CREATE INDEX IF NOT EXISTS idx_anulaciones_comprobante ON anulaciones(comprobante_id);
             CREATE INDEX IF NOT EXISTS idx_anulaciones_ticket ON anulaciones(ticket);",
        ),
    ])
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

    #[test]
    fn tabla_anulaciones_funciona() {
        let migraciones = obtener_migraciones();
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        migraciones.to_latest(&mut conn).unwrap();

        // Poblar catálogo requerido por FK de comprobantes
        conn.execute(
            "INSERT INTO tipos_pago (codigo, descripcion) VALUES ('Contado', 'Pago al contado')",
            [],
        )
        .unwrap();

        // Insertar comprobante de prueba
        conn.execute(
            "INSERT INTO comprobantes (fecha_de_emision, cliente, numero_comprobante, gravado, igv, total)
             VALUES ('2026-10-22', 'CLIENTE DE PRUEBA', 'F001-00000001', 100.0, 18.0, 118.0)",
            [],
        )
        .unwrap();
        let voucher_id = conn.last_insert_rowid();

        // Insertar anulación con CDR
        conn.execute(
            "INSERT INTO anulaciones (
                comprobante_id, tipo, motivo, id_baja, ticket,
                codigo_cdr, descripcion_cdr, xml_cdr, xml_baja_firmado
            ) VALUES (?1, 'baja', 'Error en el RUC', 'RA-20261022-1', '1234567890', '0', 'La baja ha sido aceptada', '<xml_cdr/>', '<xml_baja/>')",
            rusqlite::params![voucher_id],
        )
        .unwrap();

        // Verificar consulta
        let (tipo, motivo, ticket, codigo_cdr): (String, String, Option<String>, Option<String>) = conn
            .query_row(
                "SELECT tipo, motivo, ticket, codigo_cdr FROM anulaciones WHERE comprobante_id = ?1",
                rusqlite::params![voucher_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .unwrap();

        assert_eq!(tipo, "baja");
        assert_eq!(motivo, "Error en el RUC");
        assert_eq!(ticket.as_deref(), Some("1234567890"));
        assert_eq!(codigo_cdr.as_deref(), Some("0"));
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

    // Bases creadas con la primera versión del esquema no tienen esta columna.
    // `rusqlite_migration` ya marcó la migración inicial como aplicada, así que se
    // agrega de forma idempotente para no obligar a recrear la base.
    let ya_existe: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('comprobantes') WHERE name = 'tipo_cambio'",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map(|n| n > 0)
        .map_err(|e| e.to_string())?;

    if !ya_existe {
        conn.execute("ALTER TABLE comprobantes ADD COLUMN tipo_cambio REAL", [])
            .map_err(|e| e.to_string())?;
    }

    let tiene_forma_pago: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('comprobantes') WHERE name = 'forma_pago'",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map(|n| n > 0)
        .map_err(|e| e.to_string())?;

    if !tiene_forma_pago {
        conn.execute(
            "ALTER TABLE comprobantes ADD COLUMN forma_pago TEXT NOT NULL DEFAULT 'Contado'",
            [],
        )
        .map_err(|e| e.to_string())?;
    }

    let tiene_monto_pendiente: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('comprobantes') WHERE name = 'monto_neto_pendiente'",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map(|n| n > 0)
        .map_err(|e| e.to_string())?;

    if !tiene_monto_pendiente {
        conn.execute("ALTER TABLE comprobantes ADD COLUMN monto_neto_pendiente REAL", [])
            .map_err(|e| e.to_string())?;
    }

    let tiene_fecha_venc: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('comprobantes') WHERE name = 'fecha_vencimiento'",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map(|n| n > 0)
        .map_err(|e| e.to_string())?;

    if !tiene_fecha_venc {
        conn.execute("ALTER TABLE comprobantes ADD COLUMN fecha_vencimiento TEXT", [])
            .map_err(|e| e.to_string())?;
    }

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
    // Se siembra con `INSERT OR IGNORE` y sin guarda de "tabla vacía" para que los
    // códigos añadidos después lleguen también a las bases de datos ya creadas, sin
    // duplicar los existentes.
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    {
        // 1. Sembrar Afectaciones de Venta
        let mut stmt_v = tx
            .prepare(
                "INSERT OR IGNORE INTO afectaciones_venta (codigo, descripcion) VALUES (?1, ?2)",
            )
            .map_err(|e| e.to_string())?;

        for item in TIPOS_AFECTACION_VENTAS {
            stmt_v
                .execute(params![item.codigo, item.descripcion])
                .map_err(|e| e.to_string())?;
        }

        // 2. Sembrar Afectaciones de Compra
        let mut stmt_c = tx
            .prepare(
                "INSERT OR IGNORE INTO afectaciones_compra (codigo, descripcion) VALUES (?1, ?2)",
            )
            .map_err(|e| e.to_string())?;

        for item in DESTINO_AFECTACION_COMPRAS {
            stmt_c
                .execute(params![item.codigo, item.descripcion])
                .map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;

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
