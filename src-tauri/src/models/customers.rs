use crate::AppState;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: i64,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre: String,
    pub nombre_comercial: Option<String>,
    pub pais: Option<String>,
    pub departamento: Option<String>,
    pub provincia: Option<String>,
    pub distrito: Option<String>,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub correo: Option<String>,
    pub activo: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCustomerPayload {
    pub tipo_documento: String,
    pub numero_documento: String,
    pub nombre: String,
    pub nombre_comercial: Option<String>,
    pub pais: Option<String>,
    pub departamento: Option<String>,
    pub provincia: Option<String>,
    pub distrito: Option<String>,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub correo: Option<String>,
}

// --- CORE LOGIC ---

pub fn core_get_customers(db: &Connection) -> Result<Vec<Customer>, String> {
    let mut stmt = db.prepare(
        "SELECT id, tipo_documento, numero_documento, nombre, nombre_comercial, pais, departamento, provincia, distrito, direccion, telefono, correo, activo FROM clientes WHERE activo = 1",
    )
    .map_err(|e| e.to_string())?;

    let customer_iter = stmt
        .query_map([], |row| {
            Ok(Customer {
                id: row.get(0)?,
                tipo_documento: row.get(1)?,
                numero_documento: row.get(2)?,
                nombre: row.get(3)?,
                nombre_comercial: row.get(4)?,
                pais: row.get(5)?,
                departamento: row.get(6)?,
                provincia: row.get(7)?,
                distrito: row.get(8)?,
                direccion: row.get(9)?,
                telefono: row.get(10)?,
                correo: row.get(11)?,
                activo: row.get(12)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut customers = Vec::new();
    for customer in customer_iter {
        customers.push(customer.map_err(|e| e.to_string())?);
    }
    Ok(customers)
}

pub fn core_create_customer(
    db: &Connection,
    payload: CreateCustomerPayload,
) -> Result<Customer, String> {
    let existing = db
        .query_row(
            "SELECT id, activo FROM clientes WHERE numero_documento = ?1 LIMIT 1",
            params![payload.numero_documento],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, bool>(1)?)),
        )
        .optional()
        .map_err(|e| e.to_string())?;

    if let Some((id, activo)) = existing {
        if activo {
            return Err(format!(
                "Ya existe un cliente activo con el documento {}",
                payload.numero_documento
            ));
        }

        db.execute(
            "UPDATE clientes SET tipo_documento = ?1, numero_documento = ?2, nombre = ?3, nombre_comercial = ?4, pais = ?5, departamento = ?6, provincia = ?7, distrito = ?8, direccion = ?9, telefono = ?10, correo = ?11, activo = 1 WHERE id = ?12",
            params![
                payload.tipo_documento,
                payload.numero_documento,
                payload.nombre,
                payload.nombre_comercial,
                payload.pais,
                payload.departamento,
                payload.provincia,
                payload.distrito,
                payload.direccion,
                payload.telefono,
                payload.correo,
                id
            ],
        ).map_err(|e| e.to_string())?;

        return Ok(Customer {
            id,
            tipo_documento: payload.tipo_documento,
            numero_documento: payload.numero_documento,
            nombre: payload.nombre,
            nombre_comercial: payload.nombre_comercial,
            pais: payload.pais,
            departamento: payload.departamento,
            provincia: payload.provincia,
            distrito: payload.distrito,
            direccion: payload.direccion,
            telefono: payload.telefono,
            correo: payload.correo,
            activo: true,
        });
    }

    db.execute(
        "INSERT INTO clientes (tipo_documento, numero_documento, nombre, nombre_comercial, pais, departamento, provincia, distrito, direccion, telefono, correo) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            payload.tipo_documento,
            payload.numero_documento,
            payload.nombre,
            payload.nombre_comercial,
            payload.pais,
            payload.departamento,
            payload.provincia,
            payload.distrito,
            payload.direccion,
            payload.telefono,
            payload.correo
        ],
    ).map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();

    Ok(Customer {
        id,
        tipo_documento: payload.tipo_documento,
        numero_documento: payload.numero_documento,
        nombre: payload.nombre,
        nombre_comercial: payload.nombre_comercial,
        pais: payload.pais,
        departamento: payload.departamento,
        provincia: payload.provincia,
        distrito: payload.distrito,
        direccion: payload.direccion,
        telefono: payload.telefono,
        correo: payload.correo,
        activo: true,
    })
}

pub fn core_update_customer(
    db: &Connection,
    id: i64,
    payload: CreateCustomerPayload,
) -> Result<Customer, String> {
    let conflict = db
        .query_row(
            "SELECT id FROM clientes WHERE numero_documento = ?1 AND id != ?2 LIMIT 1",
            params![payload.numero_documento, id],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;

    if conflict.is_some() {
        return Err(format!(
            "Ya existe otro cliente con el documento {}",
            payload.numero_documento
        ));
    }

    db.execute(
        "UPDATE clientes SET tipo_documento = ?1, numero_documento = ?2, nombre = ?3, nombre_comercial = ?4, pais = ?5, departamento = ?6, provincia = ?7, distrito = ?8, direccion = ?9, telefono = ?10, correo = ?11 WHERE id = ?12",
        params![
            payload.tipo_documento,
            payload.numero_documento,
            payload.nombre,
            payload.nombre_comercial,
            payload.pais,
            payload.departamento,
            payload.provincia,
            payload.distrito,
            payload.direccion,
            payload.telefono,
            payload.correo,
            id
        ],
    ).map_err(|e| e.to_string())?;

    Ok(Customer {
        id,
        tipo_documento: payload.tipo_documento,
        numero_documento: payload.numero_documento,
        nombre: payload.nombre,
        nombre_comercial: payload.nombre_comercial,
        pais: payload.pais,
        departamento: payload.departamento,
        provincia: payload.provincia,
        distrito: payload.distrito,
        direccion: payload.direccion,
        telefono: payload.telefono,
        correo: payload.correo,
        activo: true,
    })
}

pub fn core_delete_customer(db: &Connection, id: i64) -> Result<(), String> {
    db.execute("UPDATE clientes SET activo = 0 WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// --- TAURI COMMANDS ---

#[tauri::command]
pub fn get_customers(state: State<'_, AppState>) -> Result<Vec<Customer>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_get_customers(db)
}

#[tauri::command]
pub fn create_customer(
    payload: CreateCustomerPayload,
    state: State<'_, AppState>,
) -> Result<Customer, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_create_customer(db, payload)
}

#[tauri::command]
pub fn update_customer(
    id: i64,
    payload: CreateCustomerPayload,
    state: State<'_, AppState>,
) -> Result<Customer, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_update_customer(db, id, payload)
}

#[tauri::command]
pub fn delete_customer(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized.")?;
    core_delete_customer(db, id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> Connection {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch(
            r#"
            CREATE TABLE clientes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tipo_documento TEXT NOT NULL,
                numero_documento TEXT NOT NULL UNIQUE,
                nombre TEXT NOT NULL,
                nombre_comercial TEXT,
                pais TEXT NOT NULL DEFAULT 'PERU',
                departamento TEXT,
                provincia TEXT,
                distrito TEXT,
                direccion TEXT,
                telefono TEXT,
                correo TEXT,
                activo BOOLEAN NOT NULL DEFAULT 1
            );
            "#,
        )
        .unwrap();
        db
    }

    #[test]
    fn reactivates_inactive_duplicate_document() {
        let db = create_test_db();
        let payload = CreateCustomerPayload {
            tipo_documento: "RUC".to_string(),
            numero_documento: "20123456789".to_string(),
            nombre: "Cliente viejo".to_string(),
            nombre_comercial: None,
            pais: Some("PERU".to_string()),
            departamento: None,
            provincia: None,
            distrito: None,
            direccion: None,
            telefono: None,
            correo: None,
        };

        let created = core_create_customer(&db, payload.clone()).unwrap();
        assert!(created.activo);

        core_delete_customer(&db, created.id).unwrap();
        assert_eq!(core_get_customers(&db).unwrap().len(), 0);

        let reactivated = core_create_customer(&db, payload).unwrap();
        assert_eq!(reactivated.id, created.id);
        assert!(reactivated.activo);
        assert_eq!(core_get_customers(&db).unwrap().len(), 1);
    }

    #[test]
    fn rejects_active_duplicate_document() {
        let db = create_test_db();
        let payload = CreateCustomerPayload {
            tipo_documento: "DNI".to_string(),
            numero_documento: "12345678".to_string(),
            nombre: "Cliente activo".to_string(),
            nombre_comercial: None,
            pais: Some("PERU".to_string()),
            departamento: None,
            provincia: None,
            distrito: None,
            direccion: None,
            telefono: None,
            correo: None,
        };

        core_create_customer(&db, payload.clone()).unwrap();
        let result = core_create_customer(&db, payload);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_document_on_update() {
        let db = create_test_db();
        let payload1 = CreateCustomerPayload {
            tipo_documento: "DNI".to_string(),
            numero_documento: "11111111".to_string(),
            nombre: "Cliente 1".to_string(),
            nombre_comercial: None,
            pais: Some("PERU".to_string()),
            departamento: None,
            provincia: None,
            distrito: None,
            direccion: None,
            telefono: None,
            correo: None,
        };
        let payload2 = CreateCustomerPayload {
            tipo_documento: "DNI".to_string(),
            numero_documento: "22222222".to_string(),
            nombre: "Cliente 2".to_string(),
            nombre_comercial: None,
            pais: Some("PERU".to_string()),
            departamento: None,
            provincia: None,
            distrito: None,
            direccion: None,
            telefono: None,
            correo: None,
        };

        let c1 = core_create_customer(&db, payload1).unwrap();
        let _c2 = core_create_customer(&db, payload2.clone()).unwrap();

        // Updating c1 to have c2's document should fail
        let update_payload = CreateCustomerPayload {
            numero_documento: "22222222".to_string(),
            ..payload2
        };
        let result = core_update_customer(&db, c1.id, update_payload);
        assert!(result.is_err());
    }
}
