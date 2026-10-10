use crate::validaciones::{parse_fecha_emision_iso, validar_comprobante, DatosValidacion};
use crate::xml_builder::{
    canonicalize_signed_info, canonicalize_xml, generate_xml, hash_xml_sha256, items_de_venta,
    remove_signature, totales_de, FacturaPayload, ItemVenta,
};
use crate::AppState;
use base64::{engine::general_purpose, Engine as _};
use boveda_core::{parse_pkcs12, sign_hash_with_key};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

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
    pub estado_pago: Option<String>,
    // Campos para validaciones SUNAT
    pub emisor_ruc: String,
    pub emisor_razon_social: String,
    pub emisor_ubigeo: String,
    pub emisor_direccion: String,
    pub receptor_tipo_doc: String,
    pub receptor_num_doc: String,
    pub tipo_operacion: String,
    pub p12_bytes: Vec<u8>,
    pub p12_password: String,
    /// Tarifa vigente del ICBPER por bolsa. La envía el frontend desde Ajustes para
    /// que el importe que vio el usuario sea el mismo que se declara ante SUNAT; si
    /// no viene (payloads antiguos) se usa S/ 0.50.
    #[serde(default = "tasa_icbper_default")]
    pub tasa_icbper: f64,
    /// Las líneas tal como las capturó el usuario: precios con IGV incluido y la
    /// afectación de cada ítem. Los totales del comprobante no se reciben, se derivan
    /// de aquí para que el frontend no pueda declarar una base que no cuadre con sus
    /// propias líneas.
    pub items: Vec<ItemVenta>,
}

/// Tarifa por defecto del ICBPER (S/ 0.50 por bolsa), usada si el payload no la trae.
fn tasa_icbper_default() -> f64 {
    0.5
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
pub fn core_create_voucher(
    db: &mut Connection,
    payload: CreateVoucherPayload,
) -> Result<Voucher, String> {
    if !is_valid_serie(&payload.serie, &payload.tipo_comprobante) {
        return Err(format!(
            "La serie {} no es válida para el tipo de comprobante {}.",
            payload.serie, payload.tipo_comprobante
        ));
    }

    // --- Validaciones de Reglas de Negocio SUNAT ---
    // Las líneas se normalizan primero: de ahí salen la base imponible, el IGV de
    // cada ítem y los totales, todos con el mismo criterio (`tributo_de`).
    let items = items_de_venta(payload.items, payload.tasa_icbper)?;
    if items.is_empty() {
        return Err("Un comprobante no puede emitirse sin ítems.".into());
    }
    let totales = totales_de(&items);

    let datos = DatosValidacion {
        tipo_comprobante: &payload.tipo_comprobante,
        emisor_ruc: &payload.emisor_ruc,
        receptor_tipo_doc: &payload.receptor_tipo_doc,
        receptor_num_doc: &payload.receptor_num_doc,
        total_gravado: totales.base_gravada,
        total_igv: totales.igv,
        total_pagar: totales.total_pagar,
        op_exoneradas: totales.base_no_gravada,
        op_icbper: totales.icbper,
        fecha_emision: &payload.fecha_de_emision,
        tipo_operacion: &payload.tipo_operacion,
        lineas_gravadas: totales.lineas_gravadas,
    };
    validar_comprobante(&datos)?;
    // ------------------------------------------------

    // --- ORQUESTACIÓN DE FIRMA XML ---
    // `cbc:IssueDate` es de tipo `xs:date`, así que debe ir siempre en `YYYY-MM-DD`.
    // El frontend manda `DD/MM/YYYY HH:MM`; sin normalizar, SUNAT rechaza el XML.
    let fecha_emision = parse_fecha_emision_iso(&payload.fecha_de_emision)?;
    let mut f_payload = FacturaPayload {
        hash_xml: String::new(),
        firma: String::new(),
        certificado: String::new(),
        serie: payload.serie.clone(),
        correlativo: format!("{:08}", payload.correlativo),
        fecha_emision,
        hora_emision: chrono::Local::now().format("%H:%M:%S").to_string(),
        tipo_comprobante: payload.tipo_comprobante.clone(),
        moneda: payload.moneda.clone(),
        emisor_ruc: payload.emisor_ruc.clone(),
        emisor_razon_social: payload.emisor_razon_social.clone(),
        emisor_ubigeo: payload.emisor_ubigeo.clone(),
        emisor_direccion: payload.emisor_direccion.clone(),
        cliente_tipo_doc: payload.receptor_tipo_doc.clone(),
        cliente_num_doc: payload.receptor_num_doc.clone(),
        cliente_nombre: payload.cliente.clone(),
        // `LineExtensionAmount` es la suma de las bases de todas las líneas, con
        // IGV incluido el de las exoneradas e inafectas.
        total_gravado: totales.base_total.into(),
        total_igv: totales.igv.into(),
        total_pagar: totales.total_pagar.into(),
        items,
    };

    let raw_xml = generate_xml(&f_payload)?;
    let c14n_xml = canonicalize_xml(&raw_xml)?;
    let hash_b64 = hash_xml_sha256(&c14n_xml);

    let (cert_info, private_key) = parse_pkcs12(&payload.p12_bytes, &payload.p12_password)
        .map_err(|e| format!("Error procesando el certificado: {}", e))?;

    if !cert_info.is_valid_now() {
        return Err("El certificado se encuentra vencido.".into());
    }

    f_payload.hash_xml = hash_b64.clone();
    f_payload.firma = "DUMMY".to_string();
    f_payload.certificado = cert_info.x509_base64.clone();

    let temp_xml = generate_xml(&f_payload)?;

    let c14n_signed_info = canonicalize_signed_info(&temp_xml)?;
    let hash_signed_info_b64 = hash_xml_sha256(&c14n_signed_info);

    let hash_array: [u8; 32] = general_purpose::STANDARD
        .decode(&hash_signed_info_b64)
        .map_err(|_| "Error decodificando hash base64".to_string())?
        .try_into()
        .map_err(|_| "Hash debe ser de 32 bytes".to_string())?;

    let firma_b64 = sign_hash_with_key(&private_key, &hash_array)
        .map_err(|e| format!("Error firmando XML: {}", e))?;

    f_payload.firma = firma_b64;

    let xml_firmado = generate_xml(&f_payload)?;

    // El digest se calculó sobre el documento sin `ds:Signature`. Recomputarlo sobre
    // el XML que realmente se envía detecta cualquier divergencia entre la
    // plantilla y el cálculo; sin esto, SUNAT respondería "Incorrect reference
    // digest value" sin más detalle.
    let digest_recalculado = hash_xml_sha256(&canonicalize_xml(&remove_signature(&xml_firmado)?)?);
    if digest_recalculado != hash_b64 {
        return Err(format!(
            "El digest del comprobante no es consistente con el XML firmado (esperado {hash_b64}, obtenido {digest_recalculado}). No se enviará a SUNAT."
        ));
    }

    let tx = db.transaction().map_err(|e| e.to_string())?;

    let estado_pago = payload
        .estado_pago
        .unwrap_or_else(|| "pendiente".to_string());
    if !matches!(estado_pago.as_str(), "pendiente" | "pagado") {
        return Err(format!("Estado de pago inválido: {estado_pago}"));
    }

    tx.execute(
        "INSERT INTO comprobantes (fecha_de_emision, cliente, numero_comprobante, estado_pago, moneda, gravado, igv, total, hash_cpe, xml_firmado, estado_sunat) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            payload.fecha_de_emision,
            payload.cliente,
            payload.numero_comprobante,
            estado_pago,
            payload.moneda,
            totales.base_gravada,
            totales.igv,
            totales.total_pagar,
            hash_b64,
            xml_firmado,
            0
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
        gravado: totales.base_gravada,
        igv: totales.igv,
        total: totales.total_pagar,
        estado: true,
        hash_cpe: Some(hash_b64),
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

/// Registra el comprobante desde el frontend.
///
/// `core_create_voucher` es trabajo pesado y bloqueante: parsea el `.p12`,
/// canonicaliza el XML tres veces y firma con RSA. Como comando **síncrono**
/// Tauri lo ejecutaba en el hilo principal, que es el mismo que despacha la
/// respuesta del `invoke`; si ese hilo quedaba bloqueado, el frontend nunca
/// recibía respuesta y el botón se quedaba en «Generando...» para siempre.
///
/// Por eso el comando es `async` y delega en `spawn_blocking`, igual que hace
/// `api::run_db_task_mut` en la ruta HTTP: el hilo principal queda libre y el
/// invoke siempre resuelve (con `Ok` o con `Err`).
#[tauri::command]
pub async fn create_voucher(
    app: AppHandle,
    payload: CreateVoucherPayload,
) -> Result<Voucher, String> {
    let numero = payload.numero_comprobante.clone();
    log::info!("vouchers: create_voucher iniciado para {numero}");

    let resultado = tauri::async_runtime::spawn_blocking(move || {
        let app_state = app.state::<AppState>();
        let mut db_guard = app_state.db.lock().map_err(|e| e.to_string())?;
        let db = db_guard.as_mut().ok_or("Database not initialized.")?;
        core_create_voucher(db, payload)
    })
    .await
    .map_err(|e| format!("La tarea de registro no se pudo ejecutar: {e}"))?;

    match &resultado {
        Ok(v) => log::info!(
            "vouchers: {} registrado con id {}",
            v.numero_comprobante,
            v.id
        ),
        Err(e) => log::error!("vouchers: create_voucher fallo para {numero}: {e}"),
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::CreateVoucherPayload;
    use crate::xml_builder::items_de_venta;

    /// Reproduce el JSON que manda `VoucherModal.svelte`. Los importes viajan como
    /// números JS, así que un precio sin decimales llega como entero (`8`, no `8.0`)
    /// y antes fallaba con `invalid type: integer 8, expected un número o una cadena
    /// numérica`, abortando la creación del comprobante.
    ///
    /// El payload ya no lleva los totales: el backend los deduce de las líneas, que
    /// llegan con el importe con IGV incluido.
    #[test]
    fn el_payload_del_frontend_se_deserializa() {
        let json = r#"{
            "fecha_de_emision": "27/09/2026 00:00",
            "cliente": "CLIENTE DE PRUEBA SRL",
            "numero_comprobante": "F001-00000001",
            "serie": "F001",
            "correlativo": 1,
            "tipo_comprobante": "01",
            "moneda": "PEN",
            "estado_pago": "Pagado",
            "emisor_ruc": "20123456789",
            "emisor_razon_social": "EMPRESA DE PRUEBA SAC",
            "emisor_ubigeo": "150101",
            "emisor_direccion": "AV LIMA 123",
            "receptor_tipo_doc": "6",
            "receptor_num_doc": "20987654321",
            "tipo_operacion": "0101",
            "p12_bytes": [],
            "p12_password": "",
            "items": [
                { "unidad": "NIU", "cantidad": 8, "precio_unitario": 9.44, "total": 75.52,
                  "afectacion": "10", "descripcion": "PRODUCTO" }
            ]
        }"#;

        let payload: CreateVoucherPayload = serde_json::from_str(json).unwrap();

        assert_eq!(payload.fecha_de_emision, "27/09/2026 00:00");
        assert_eq!(payload.items[0].cantidad.0, 8.0);
        assert_eq!(payload.items[0].total.0, 75.52);

        // 75.52 con IGV incluido son 64.00 de base y 11.52 de IGV: el reparto lo hace
        // el backend a partir de la afectación, no el frontend.
        let items = items_de_venta(payload.items, 0.5).unwrap();
        assert_eq!(items[0].subtotal.to_string(), "64.00");
        assert_eq!(items[0].igv.to_string(), "11.52");
        assert_eq!(items[0].valor_unitario.to_string(), "8.00");
    }
}
