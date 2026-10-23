use base64::{engine::general_purpose, Engine as _};
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read, Write};
use tauri::State;
use zip::CompressionMethod;
use zip::ZipArchive;
use zip::ZipWriter;

// =============================================================================
// MÓDULO 1: EMPAQUETADO ZIP
// =============================================================================

/// Empaqueta el XML en un archivo ZIP en memoria, usando la nomenclatura exacta que exige SUNAT.
/// Nomenclatura: {RUC}-{TipoDoc}-{Serie}-{Correlativo}.zip
/// El ZIP contendrá un archivo XML con el mismo nombre base.
///
/// Retorna una tupla con (nombre_del_zip, contenido_binario_zip).
pub fn empaquetar_xml_sunat(
    ruc: &str,
    tipo_doc: &str,
    serie: &str,
    correlativo: &str,
    xml_content: &str,
) -> Result<(String, Vec<u8>), String> {
    let base_name = format!("{}-{}-{}-{}", ruc, tipo_doc, serie, correlativo);
    let xml_filename = format!("{}.xml", base_name);
    let zip_filename = format!("{}.zip", base_name);

    let mut buffer = Vec::new();

    {
        let mut zip = ZipWriter::new(Cursor::new(&mut buffer));
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated);

        zip.start_file(xml_filename, options)
            .map_err(|e| format!("Error al iniciar archivo zip: {}", e))?;
        zip.write_all(xml_content.as_bytes())
            .map_err(|e| format!("Error al escribir xml en zip: {}", e))?;
        zip.finish()
            .map_err(|e| format!("Error al finalizar zip: {}", e))?;
    }

    Ok((zip_filename, buffer))
}

// =============================================================================
// MÓDULO 1-bis: NÚMERO DEL COMPROBANTE
// =============================================================================

/// Extrae el `cbc:ID` del comprobante, que es la única fuente de verdad del número.
///
/// El nombre del ZIP tiene que coincidir carácter por carácter con este valor, o
/// SUNAT responde con la observación 1036: *"Número de documento en el nombre del
/// archivo no coincide con el consignado en el contenido del XML"*. Por eso no se
/// puede armar desde la columna `numero_comprobante`: esa la genera el frontend con
/// otro relleno de ceros (`padStart(7)`) que el que usa el backend al firmar el XML
/// (`{:08}`), así que ambos producían `B001-0000003` y `B001-00000003`.
pub fn extraer_id_comprobante(xml: &str) -> Result<String, String> {
    const APERTURA: &str = "<cbc:ID>";
    const CIERRE: &str = "</cbc:ID>";

    let inicio = xml.find(APERTURA).ok_or_else(|| {
        "El XML no contiene el elemento cbc:ID, no se puede nombrar el archivo.".to_string()
    })? + APERTURA.len();
    let fin = xml[inicio..]
        .find(CIERRE)
        .ok_or_else(|| "El cbc:ID del XML está mal formado.".to_string())?
        + inicio;

    let id = xml[inicio..fin].trim();
    if id.is_empty() {
        return Err("El cbc:ID del XML está vacío.".to_string());
    }
    Ok(id.to_string())
}

/// Divide un `cbc:ID` en (serie, tipo de comprobante, correlativo).
///
/// Valida el formato `F001-00000001` / `B001-00000001` que exige SUNAT, en vez de
/// deducir el tipo por el prefijo sin comprobar nada.
pub fn partir_id_comprobante(id: &str) -> Result<(String, String, String), String> {
    let mut partes = id.splitn(2, '-');
    let serie = partes.next().unwrap_or_default().to_string();
    let correlativo = partes.next().unwrap_or_default().to_string();

    if partes.next().is_some() || serie.len() != 4 || correlativo.is_empty() {
        return Err(format!(
            "El número de comprobante '{id}' no tiene el formato F001-00000001."
        ));
    }

    let tipo_doc = match serie.as_bytes()[0] {
        b'F' => "01",
        b'B' => "03",
        otro => {
            return Err(format!(
                "La serie '{}' no corresponde a una factura (F) ni a una boleta (B).",
                otro as char
            ))
        }
    };

    if !serie[1..].chars().all(|c| c.is_ascii_digit())
        || !correlativo.chars().all(|c| c.is_ascii_digit())
    {
        return Err(format!(
            "El número de comprobante '{id}' contiene caracteres no válidos."
        ));
    }

    Ok((serie, tipo_doc.to_string(), correlativo))
}

// =============================================================================
// MÓDULO 2: CLIENTE REST SUNAT
// =============================================================================

/// Entorno de conexión a SUNAT
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AmbienteSunat {
    Beta,
    Produccion,
}

impl AmbienteSunat {
    /// URL base del endpoint SOAP CPE
    fn cpe_url(&self) -> &'static str {
        match self {
            AmbienteSunat::Beta => "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService",
            AmbienteSunat::Produccion => {
                "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService"
            }
        }
    }
}

/// Credenciales del emisor para autenticarse con SUNAT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredencialesSunat {
    pub ruc: String,
    pub usuario_sol: String,
    pub clave_sol: String,
    pub client_id: String, // Ya no se usa para SOAP, pero lo mantenemos por compatibilidad de struct
    pub client_secret: String,
}

/// Resultado del CDR (Constancia de Recepción) devuelto por SUNAT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CdrSunat {
    /// Código de respuesta. "0" = aceptado, otro = observado/rechazado
    pub codigo: String,
    /// Descripción humana de la respuesta de SUNAT
    pub descripcion: String,
    /// Contenido XML del CDR (para guardarlo en BD)
    pub xml_cdr: String,
}

/// Envía un comprobante a SUNAT via SOAP y retorna el CDR parseado.
pub async fn enviar_comprobante_sunat(
    creds: &CredencialesSunat,
    ambiente: AmbienteSunat,
    ruc: &str,
    tipo_doc: &str,
    serie: &str,
    correlativo: &str,
    xml_firmado: &str,
) -> Result<CdrSunat, String> {
    // 1. Empaquetar en ZIP
    let (zip_filename, zip_bytes) =
        empaquetar_xml_sunat(ruc, tipo_doc, serie, correlativo, xml_firmado)?;

    // 2. ZIP → Base64
    let zip_b64 = general_purpose::STANDARD.encode(&zip_bytes);

    // 3. Preparar sobre SOAP
    let username = format!("{}{}", creds.ruc, creds.usuario_sol);
    let soap_request = format!(
        r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ser="http://service.sunat.gob.pe" xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd">
   <soapenv:Header>
      <wsse:Security>
         <wsse:UsernameToken>
            <wsse:Username>{}</wsse:Username>
            <wsse:Password>{}</wsse:Password>
         </wsse:UsernameToken>
      </wsse:Security>
   </soapenv:Header>
   <soapenv:Body>
      <ser:sendBill>
         <fileName>{}</fileName>
         <contentFile>{}</contentFile>
      </ser:sendBill>
   </soapenv:Body>
</soapenv:Envelope>"#,
        username, creds.clave_sol, zip_filename, zip_b64
    );

    let endpoint = ambiente.cpe_url();
    let client = reqwest::Client::new();
    let response = client
        .post(endpoint)
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(soap_request)
        .send()
        .await
        .map_err(|e| format!("Error de red al enviar a SUNAT: {}", e))?;

    let response_xml = response
        .text()
        .await
        .map_err(|e| format!("Error leyendo respuesta SOAP: {}", e))?;

    // 4. Parsear respuesta SOAP
    if let Some(fault_idx) = response_xml.find("<faultstring>") {
        let end_idx = response_xml[fault_idx..]
            .find("</faultstring>")
            .unwrap_or(0);
        let error_msg = &response_xml[fault_idx + 13..fault_idx + end_idx];
        return Err(format!("Error de SUNAT (Fault): {}", error_msg));
    }

    let cdr_start = response_xml
        .find("<applicationResponse>")
        .ok_or("No se encontró applicationResponse en la respuesta de SUNAT")?;
    let cdr_end = response_xml[cdr_start..]
        .find("</applicationResponse>")
        .ok_or("Formato de applicationResponse inválido")?;

    let cdr_b64 = &response_xml[cdr_start + 21..cdr_start + cdr_end];

    // 5. Decodificar CDR Base64 → ZIP → XML
    let cdr_zip_bytes = general_purpose::STANDARD
        .decode(cdr_b64)
        .map_err(|e| format!("Error al decodificar CDR base64: {}", e))?;

    let cdr_xml = extraer_xml_de_zip(&cdr_zip_bytes)?;

    // 6. Parsear XML del CDR
    parsear_cdr(&cdr_xml)
}

/// Extrae el primer archivo .xml de un ZIP en memoria.
fn extraer_xml_de_zip(zip_bytes: &[u8]) -> Result<String, String> {
    let mut archive = ZipArchive::new(Cursor::new(zip_bytes))
        .map_err(|e| format!("Error al abrir ZIP del CDR: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Error al leer archivo #{} del ZIP: {}", i, e))?;

        if file.name().ends_with(".xml") {
            let mut contenido = String::new();
            file.read_to_string(&mut contenido)
                .map_err(|e| format!("Error al leer XML del CDR: {}", e))?;
            return Ok(contenido);
        }
    }

    Err("El ZIP del CDR no contiene ningún archivo .xml".to_string())
}

/// Parsea el XML del CDR de SUNAT para extraer código y descripción.
/// Busca `<cbc:ResponseCode>` y `<cbc:Description>` sin depender de un parser completo.
pub fn parsear_cdr(cdr_xml: &str) -> Result<CdrSunat, String> {
    let codigo = extraer_texto_etiqueta(cdr_xml, "ResponseCode")
        .ok_or("CDR no contiene ResponseCode")?
        .trim()
        .to_string();

    let descripcion = extraer_texto_etiqueta(cdr_xml, "Description")
        .ok_or("CDR no contiene Description")?
        .trim()
        .to_string();

    Ok(CdrSunat {
        codigo,
        descripcion,
        xml_cdr: cdr_xml.to_string(),
    })
}

/// Extrae el texto de una etiqueta XML por su nombre local, ignorando el prefijo de namespace.
/// Ej: encuentra el contenido de `<cbc:ResponseCode>0</cbc:ResponseCode>` buscando "ResponseCode".
fn extraer_texto_etiqueta(xml: &str, local_name: &str) -> Option<String> {
    // --- APERTURA: buscar `:LocalName>` (con prefijo) o `<LocalName>` (sin prefijo) ---
    let with_ns = format!(":{}>", local_name);
    let without_ns = format!("<{}>", local_name);

    let start = xml
        .find(&with_ns)
        .map(|i| i + with_ns.len())
        .or_else(|| xml.find(&without_ns).map(|i| i + without_ns.len()))?;

    let tail = &xml[start..];

    // --- CIERRE: la etiqueta de cierre `</cbc:LocalName>` también contiene `:LocalName>`.
    //     Buscamos esa subcadena en tail y retrocedemos hasta el `<` más cercano.
    //     Fallback: buscar `</LocalName>` sin prefijo. ---
    let close_ns_marker = format!(":{}>", local_name); // presente en `</cbc:LocalName>`
    let close_simple = format!("</{}>", local_name); // sin namespace

    let content_end = if let Some(i) = tail.find(&close_ns_marker) {
        // Retroceder desde la posición de `:LocalName>` hasta encontrar `</`
        tail[..i].rfind('<').unwrap_or(i)
    } else if let Some(i) = tail.find(&close_simple) {
        i
    } else {
        return None;
    };

    Some(xml[start..start + content_end].to_string())
}

// =============================================================================
// COMANDO TAURI: enviar_a_sunat
// =============================================================================

/// Payload recibido desde el frontend para enviar un comprobante a SUNAT
#[derive(Debug, Deserialize)]
pub struct EnviarSunatPayload {
    pub ruc: String,
    pub usuario_sol: String,
    pub clave_sol: String,
    pub client_id: String,
    pub client_secret: String,
    /// "beta" | "produccion"
    pub ambiente: String,
    pub voucher_id: i64,
}

/// Comando Tauri que el frontend invoca para enviar un comprobante a SUNAT.
/// Retorna el CDR con código, descripción y XML completo.
#[tauri::command]
pub async fn enviar_a_sunat(
    payload: EnviarSunatPayload,
    db_state: State<'_, crate::AppState>,
) -> Result<CdrSunat, String> {
    let ambiente = if payload.ambiente == "produccion" {
        AmbienteSunat::Produccion
    } else {
        AmbienteSunat::Beta
    };

    let creds = CredencialesSunat {
        ruc: payload.ruc.clone(),
        usuario_sol: payload.usuario_sol,
        clave_sol: payload.clave_sol,
        client_id: payload.client_id,
        client_secret: payload.client_secret,
    };

    let xml_firmado: String = {
        let mut db_guard = db_state
            .db
            .lock()
            .map_err(|_| "Error al bloquear la base de datos".to_string())?;
        let db = db_guard
            .as_mut()
            .ok_or("Base de datos no inicializada".to_string())?;

        let mut stmt = db
            .prepare("SELECT xml_firmado FROM comprobantes WHERE id = ?1")
            .map_err(|e| e.to_string())?;

        let mut rows = stmt
            .query([payload.voucher_id])
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let xml: Option<String> = row.get(0).map_err(|e| e.to_string())?;

            if let Some(xml_str) = xml {
                xml_str
            } else {
                return Err("El comprobante no tiene XML firmado.".to_string());
            }
        } else {
            return Err("Comprobante no encontrado.".to_string());
        }
    };

    // El nombre del ZIP sale del `cbc:ID` del propio XML y no de la columna
    // `numero_comprobante`: es lo que SUNAT contrasta contra el contenido, y ambas
    // cosas se rellenaban con distinta cantidad de ceros (7 en el frontend, 8 al
    // firmar), lo que disparaba la observación 1036.
    let (serie, tipo_doc, correlativo) =
        partir_id_comprobante(&extraer_id_comprobante(&xml_firmado)?)?;

    let cdr = enviar_comprobante_sunat(
        &creds,
        ambiente,
        &payload.ruc,
        &tipo_doc,
        &serie,
        &correlativo,
        &xml_firmado,
    )
    .await?;

    // Actualizamos la base de datos con el resultado
    {
        let mut db_guard = db_state
            .db
            .lock()
            .map_err(|_| "Error al bloquear la base de datos".to_string())?;
        let db = db_guard
            .as_mut()
            .ok_or("Base de datos no inicializada".to_string())?;

        let estado_validez = if cdr.codigo == "0" {
            "aceptado"
        } else {
            "rechazado"
        };
        let estado_sunat = if cdr.codigo == "0" { 1 } else { 2 };

        db.execute(
            "UPDATE comprobantes SET estado_validez = ?1, estado_sunat = ?2, codigo_cdr = ?3, descripcion_cdr = ?4, xml_cdr = ?5 WHERE id = ?6",
            rusqlite::params![estado_validez, estado_sunat, cdr.codigo, cdr.descripcion, cdr.xml_cdr, payload.voucher_id],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(cdr)
}

/// Información de la anulación o Comunicación de Baja registrada para un comprobante
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Anulacion {
    pub id: i64,
    pub comprobante_id: i64,
    pub tipo: String, // "baja", "nota_credito", "local"
    pub motivo: String,
    pub id_baja: Option<String>,
    pub ticket: Option<String>,
    pub codigo_cdr: Option<String>,
    pub descripcion_cdr: Option<String>,
    pub xml_cdr: Option<String>,
    pub xml_baja_firmado: Option<String>,
    pub fecha_anulacion: String,
}

#[tauri::command]
pub fn descargar_documento_sunat(
    voucher_id: i64,
    tipo: String, // "XML", "CDR", "XML_BAJA", "CDR_BAJA"
    db_state: State<'_, crate::AppState>,
) -> Result<Vec<u8>, String> {
    let mut db_guard = db_state
        .db
        .lock()
        .map_err(|_| "Error al bloquear la base de datos".to_string())?;
    let db = db_guard
        .as_mut()
        .ok_or("Base de datos no inicializada".to_string())?;

    match tipo.as_str() {
        "XML" => {
            let mut stmt = db
                .prepare("SELECT xml_firmado FROM comprobantes WHERE id = ?1")
                .map_err(|e| e.to_string())?;
            let content: Option<String> = stmt
                .query_row([voucher_id], |row| row.get(0))
                .optional()
                .map_err(|e| e.to_string())?
                .flatten();
            content
                .map(|s| s.into_bytes())
                .ok_or_else(|| "El comprobante no tiene XML disponible.".to_string())
        }
        "CDR" => {
            // Intenta obtener el CDR del comprobante
            let mut stmt = db
                .prepare("SELECT xml_cdr FROM comprobantes WHERE id = ?1")
                .map_err(|e| e.to_string())?;
            let content: Option<String> = stmt
                .query_row([voucher_id], |row| row.get(0))
                .optional()
                .map_err(|e| e.to_string())?
                .flatten();

            if let Some(xml_str) = content {
                Ok(xml_str.into_bytes())
            } else {
                // Fallback: si no tiene en comprobantes, revisa en anulaciones
                let mut stmt_anul = db
                    .prepare(
                        "SELECT xml_cdr FROM anulaciones WHERE comprobante_id = ?1 AND xml_cdr IS NOT NULL ORDER BY id DESC LIMIT 1",
                    )
                    .map_err(|e| e.to_string())?;
                let content_anul: Option<String> = stmt_anul
                    .query_row([voucher_id], |row| row.get(0))
                    .optional()
                    .map_err(|e| e.to_string())?
                    .flatten();
                content_anul
                    .map(|s| s.into_bytes())
                    .ok_or_else(|| "El comprobante no tiene CDR disponible.".to_string())
            }
        }
        "CDR_BAJA" => {
            let mut stmt = db
                .prepare(
                    "SELECT xml_cdr FROM anulaciones WHERE comprobante_id = ?1 AND xml_cdr IS NOT NULL ORDER BY id DESC LIMIT 1",
                )
                .map_err(|e| e.to_string())?;
            let content: Option<String> = stmt
                .query_row([voucher_id], |row| row.get(0))
                .optional()
                .map_err(|e| e.to_string())?
                .flatten();
            content
                .map(|s| s.into_bytes())
                .ok_or_else(|| "No se encontró CDR de baja/anulación para este comprobante.".to_string())
        }
        "XML_BAJA" => {
            let mut stmt = db
                .prepare(
                    "SELECT xml_baja_firmado FROM anulaciones WHERE comprobante_id = ?1 AND xml_baja_firmado IS NOT NULL ORDER BY id DESC LIMIT 1",
                )
                .map_err(|e| e.to_string())?;
            let content: Option<String> = stmt
                .query_row([voucher_id], |row| row.get(0))
                .optional()
                .map_err(|e| e.to_string())?
                .flatten();
            content
                .map(|s| s.into_bytes())
                .ok_or_else(|| "No se encontró XML firmado de la baja para este comprobante.".to_string())
        }
        _ => Err(format!("Tipo de documento desconocido: {}", tipo)),
    }
}

/// Obtiene los datos de anulación (o Comunicación de Baja) asociados a un comprobante
#[tauri::command]
pub fn obtener_anulacion_voucher(
    voucher_id: i64,
    db_state: State<'_, crate::AppState>,
) -> Result<Option<Anulacion>, String> {
    let mut db_guard = db_state
        .db
        .lock()
        .map_err(|_| "Error al bloquear la base de datos".to_string())?;
    let db = db_guard
        .as_mut()
        .ok_or("Base de datos no inicializada".to_string())?;

    let mut stmt = db
        .prepare(
            "SELECT id, comprobante_id, tipo, motivo, id_baja, ticket, codigo_cdr, descripcion_cdr, xml_cdr, xml_baja_firmado, fecha_anulacion
             FROM anulaciones
             WHERE comprobante_id = ?1
             ORDER BY id DESC
             LIMIT 1",
        )
        .map_err(|e| e.to_string())?;

    let anulacion = stmt
        .query_row([voucher_id], |row| {
            Ok(Anulacion {
                id: row.get(0)?,
                comprobante_id: row.get(1)?,
                tipo: row.get(2)?,
                motivo: row.get(3)?,
                id_baja: row.get(4)?,
                ticket: row.get(5)?,
                codigo_cdr: row.get(6)?,
                descripcion_cdr: row.get(7)?,
                xml_cdr: row.get(8)?,
                xml_baja_firmado: row.get(9)?,
                fecha_anulacion: row.get(10)?,
            })
        })
        .optional()
        .map_err(|e| e.to_string())?;

    Ok(anulacion)
}

// =============================================================================
// MÓDULO 4: COMUNICACIÓN DE BAJA / REVERSIÓN
// =============================================================================

lazy_static::lazy_static! {
    static ref TERA_BAJA: tera::Tera = {
        let mut t = tera::Tera::default();
        let tpl = include_str!("../templates/baja.xml");
        t.add_raw_template("baja.xml", tpl)
            .expect("Fallo al cargar plantilla baja.xml");
        t
    };
}

/// Un comprobante a incluir en la Comunicación de Baja.
#[derive(Debug, Deserialize, Serialize)]
pub struct DocBaja {
    /// Tipo de comprobante SUNAT: "01" = Factura, "03" = Boleta.
    pub tipo: String,
    /// Serie (ej. "F001", "B001").
    pub serie: String,
    /// Correlativo numérico (ej. "00000001").
    pub correlativo: String,
    /// Motivo de anulación que ingresó el usuario.
    pub motivo: String,
}

/// Payload para el comando `enviar_baja_sunat`.
#[derive(Debug, Deserialize)]
pub struct EnviarBajaPayload {
    pub ruc: String,
    pub usuario_sol: String,
    pub clave_sol: String,
    pub client_id: String,
    pub client_secret: String,
    /// "beta" | "produccion"
    pub ambiente: String,
    /// Credenciales del certificado .p12 para firmar.
    pub p12_bytes: Vec<u8>,
    pub p12_password: String,
    /// ID del comprobante en la BD local.
    pub voucher_id: i64,
    /// Motivo libre de anulación.
    pub motivo: String,
    /// Nombre de la empresa emisora.
    pub emisor_razon_social: String,
    /// Correlativo del resumen (se incrementa por día; el frontend lo gestiona).
    pub correlativo_baja: u32,
}

/// Respuesta devuelta al frontend tras enviar la baja.
#[derive(Debug, Serialize)]
pub struct ResultadoBaja {
    /// Ticket SUNAT para consultar el estado de forma asíncrona.
    pub ticket: String,
    /// Código de CDR una vez consultado el ticket (si está disponible).
    pub codigo: String,
    /// Descripción del CDR.
    pub descripcion: String,
}

impl AmbienteSunat {
    /// URL del endpoint `sendSummary` (Comunicaciones de Baja y Resúmenes).
    fn summary_url(&self) -> &'static str {
        match self {
            AmbienteSunat::Beta => {
                "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService"
            }
            AmbienteSunat::Produccion => {
                "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService"
            }
        }
    }
}

/// Renderiza la plantilla `baja.xml` con los datos del comprobante y estado de firma.
fn render_xml_baja(
    id_baja: &str,
    fecha_referencia: &str, // fecha de emisión del comprobante (YYYY-MM-DD)
    fecha_emision: &str,    // hoy (YYYY-MM-DD)
    emisor_ruc: &str,
    emisor_razon_social: &str,
    documentos: &[DocBaja],
    hash_xml: &str,
    firma: &str,
    certificado: &str,
) -> Result<String, String> {
    use tera::Context;

    let mut ctx = Context::new();
    ctx.insert("id_baja", id_baja);
    ctx.insert("fecha_referencia", fecha_referencia);
    ctx.insert("fecha_emision", fecha_emision);
    ctx.insert("emisor_ruc", emisor_ruc);
    ctx.insert("emisor_razon_social", emisor_razon_social);
    ctx.insert("documentos", documentos);
    ctx.insert("hash_xml", hash_xml);
    ctx.insert("firma", firma);
    ctx.insert("certificado", certificado);

    TERA_BAJA
        .render("baja.xml", &ctx)
        .map_err(|e| format!("Error al renderizar baja.xml: {e}"))
}

/// Firma el XML de baja con el certificado .p12 siguiendo el estándar Enveloped Signature de SUNAT.
fn firmar_xml_baja(
    id_baja: &str,
    fecha_referencia: &str,
    fecha_emision: &str,
    emisor_ruc: &str,
    emisor_razon_social: &str,
    documentos: &[DocBaja],
    p12_bytes: &[u8],
    p12_password: &str,
) -> Result<String, String> {
    use criptor::{parse_pkcs12, sign_hash_with_key};

    // 1. Renderizar XML sin firma (firma = "") para calcular digest del documento
    let raw_xml = render_xml_baja(
        id_baja,
        fecha_referencia,
        fecha_emision,
        emisor_ruc,
        emisor_razon_social,
        documentos,
        "",
        "",
        "",
    )?;

    // 2. Canonizar el XML sin firma y calcular digest SHA-256
    let c14n_xml = crate::xml_builder::canonicalize_xml(&raw_xml)?;
    let hash_b64 = crate::xml_builder::hash_xml_sha256(&c14n_xml);

    let (cert_info, private_key) =
        parse_pkcs12(p12_bytes, p12_password).map_err(|e| e.to_string())?;

    if !cert_info.is_valid_now() {
        return Err("El certificado digital se encuentra vencido.".into());
    }

    // 3. Renderizar XML temporal con hash real, certificado y firma dummy ("DUMMY")
    // Al tener firma no vacía, baja.xml renderiza <ds:Signature> con su <ds:SignedInfo>
    let temp_xml = render_xml_baja(
        id_baja,
        fecha_referencia,
        fecha_emision,
        emisor_ruc,
        emisor_razon_social,
        documentos,
        &hash_b64,
        "DUMMY",
        &cert_info.x509_base64,
    )?;

    // 4. Canonizar únicamente el elemento <ds:SignedInfo>
    let c14n_signed_info = crate::xml_builder::canonicalize_signed_info(&temp_xml)?;
    let hash_signed_info_b64 = crate::xml_builder::hash_xml_sha256(&c14n_signed_info);

    let hash_array: [u8; 32] = general_purpose::STANDARD
        .decode(&hash_signed_info_b64)
        .map_err(|_| "Error decodificando hash base64 de SignedInfo".to_string())?
        .try_into()
        .map_err(|_| "Hash debe ser de 32 bytes".to_string())?;

    // 5. Firmar hash_array con la clave privada RSA
    let firma_b64 = sign_hash_with_key(&private_key, &hash_array)
        .map_err(|e| format!("Error firmando XML de baja: {e}"))?;

    // 6. Renderizar XML final con la firma real
    let xml_firmado = render_xml_baja(
        id_baja,
        fecha_referencia,
        fecha_emision,
        emisor_ruc,
        emisor_razon_social,
        documentos,
        &hash_b64,
        &firma_b64,
        &cert_info.x509_base64,
    )?;

    Ok(xml_firmado)
}

/// Envía un resumen/baja a SUNAT via `sendSummary` y devuelve el ticket.
async fn enviar_summary_sunat(
    creds: &CredencialesSunat,
    ambiente: AmbienteSunat,
    ruc: &str,
    id_baja: &str,
    xml_firmado: &str,
) -> Result<String, String> {
    // El nombre del ZIP para la baja es: {RUC}-{ID_BAJA}.zip
    // donde ID_BAJA = "RA-YYYYMMDD-N" o "RC-YYYYMMDD-N"
    let zip_filename = format!("{}-{}.zip", ruc, id_baja);

    // Empaquetar XML en ZIP
    let zip_bytes = {
        use std::io::{Cursor, Write};
        use zip::CompressionMethod;
        let mut buffer = Vec::new();
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut buffer));
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated);
        let xml_filename = format!("{}-{}.xml", ruc, id_baja);
        zip.start_file(xml_filename, opts)
            .map_err(|e| format!("Error al crear ZIP de baja: {e}"))?;
        zip.write_all(xml_firmado.as_bytes())
            .map_err(|e| format!("Error al escribir XML en ZIP de baja: {e}"))?;
        zip.finish()
            .map_err(|e| format!("Error al finalizar ZIP de baja: {e}"))?;
        buffer
    };

    let zip_b64 = general_purpose::STANDARD.encode(&zip_bytes);
    let username = format!("{}{}", creds.ruc, creds.usuario_sol);

    let soap_request = format!(
        r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ser="http://service.sunat.gob.pe" xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd">
   <soapenv:Header>
      <wsse:Security>
         <wsse:UsernameToken>
            <wsse:Username>{}</wsse:Username>
            <wsse:Password>{}</wsse:Password>
         </wsse:UsernameToken>
      </wsse:Security>
   </soapenv:Header>
   <soapenv:Body>
      <ser:sendSummary>
         <fileName>{}</fileName>
         <contentFile>{}</contentFile>
      </ser:sendSummary>
   </soapenv:Body>
</soapenv:Envelope>"#,
        username, creds.clave_sol, zip_filename, zip_b64
    );

    let client = reqwest::Client::new();
    let resp = client
        .post(ambiente.summary_url())
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(soap_request)
        .send()
        .await
        .map_err(|e| format!("Error de red enviando baja a SUNAT: {e}"))?;

    let resp_xml = resp
        .text()
        .await
        .map_err(|e| format!("Error leyendo respuesta sendSummary: {e}"))?;

    // Verificar Fault
    if let Some(fi) = resp_xml.find("<faultstring>") {
        let ei = resp_xml[fi..].find("</faultstring>").unwrap_or(0);
        return Err(format!(
            "Error SUNAT (Fault): {}",
            &resp_xml[fi + 13..fi + ei]
        ));
    }

    // Extraer ticket
    let ticket = extraer_texto_etiqueta(&resp_xml, "ticket")
        .ok_or("SUNAT no devolvió un ticket en la respuesta de sendSummary")?;

    Ok(ticket)
}

/// Consulta el estado de un ticket `sendSummary` usando `getStatus`.
async fn consultar_ticket_sunat(
    creds: &CredencialesSunat,
    ambiente: AmbienteSunat,
    ticket: &str,
) -> Result<CdrSunat, String> {
    let username = format!("{}{}", creds.ruc, creds.usuario_sol);
    let soap_request = format!(
        r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ser="http://service.sunat.gob.pe" xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd">
   <soapenv:Header>
      <wsse:Security>
         <wsse:UsernameToken>
            <wsse:Username>{}</wsse:Username>
            <wsse:Password>{}</wsse:Password>
         </wsse:UsernameToken>
      </wsse:Security>
   </soapenv:Header>
   <soapenv:Body>
      <ser:getStatus>
         <ticket>{}</ticket>
      </ser:getStatus>
   </soapenv:Body>
</soapenv:Envelope>"#,
        username, creds.clave_sol, ticket
    );

    let client = reqwest::Client::new();
    let resp = client
        .post(ambiente.summary_url())
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(soap_request)
        .send()
        .await
        .map_err(|e| format!("Error de red consultando ticket: {e}"))?;

    let resp_xml = resp
        .text()
        .await
        .map_err(|e| format!("Error leyendo respuesta getStatus: {e}"))?;

    if let Some(fi) = resp_xml.find("<faultstring>") {
        let ei = resp_xml[fi..].find("</faultstring>").unwrap_or(0);
        return Err(format!("Error SUNAT (Fault): {}", &resp_xml[fi + 13..fi + ei]));
    }

    // statusCode "0" = procesado, "98" = en proceso (retry), "99" = error
    let status_code = extraer_texto_etiqueta(&resp_xml, "statusCode")
        .unwrap_or_else(|| "99".to_string());

    if status_code == "98" {
        return Err("SUNAT aún está procesando la baja (código 98). Intente consultar más tarde.".to_string());
    }

    let cdr_start = resp_xml
        .find("<content>")
        .ok_or("No se encontró <content> en la respuesta de getStatus")?;
    let cdr_end = resp_xml[cdr_start..]
        .find("</content>")
        .ok_or("Respuesta de getStatus mal formada")?;
    let cdr_b64 = &resp_xml[cdr_start + 9..cdr_start + cdr_end];

    let cdr_zip_bytes = general_purpose::STANDARD
        .decode(cdr_b64)
        .map_err(|e| format!("Error decodificando CDR de baja: {e}"))?;

    let cdr_xml = extraer_xml_de_zip(&cdr_zip_bytes)?;
    parsear_cdr(&cdr_xml)
}

/// Comando Tauri: envía una Comunicación de Baja (RA / RC) a SUNAT,
/// espera el ticket y actualiza el estado del comprobante en la BD.
#[tauri::command]
pub async fn enviar_baja_sunat(
    payload: EnviarBajaPayload,
    db_state: State<'_, crate::AppState>,
) -> Result<ResultadoBaja, String> {
    use chrono::Local;

    if payload.motivo.trim().is_empty() {
        return Err("El motivo de anulación no puede estar vacío.".to_string());
    }

    let ambiente = if payload.ambiente == "produccion" {
        AmbienteSunat::Produccion
    } else {
        AmbienteSunat::Beta
    };

    let creds = CredencialesSunat {
        ruc: payload.ruc.clone(),
        usuario_sol: payload.usuario_sol.clone(),
        clave_sol: payload.clave_sol.clone(),
        client_id: payload.client_id.clone(),
        client_secret: payload.client_secret.clone(),
    };

    // 1. Leer el comprobante de la BD
    let (numero_comprobante, fecha_emision_str) = {
        let mut db_guard = db_state
            .db
            .lock()
            .map_err(|_| "Error al bloquear la base de datos".to_string())?;
        let db = db_guard
            .as_mut()
            .ok_or("Base de datos no inicializada".to_string())?;

        let mut stmt = db
            .prepare(
                "SELECT numero_comprobante, fecha_de_emision
                 FROM comprobantes
                 WHERE id = ?1 AND (estado_sunat = 1 OR (estado_sunat = 3 AND EXISTS (SELECT 1 FROM anulaciones WHERE comprobante_id = ?1 AND tipo = 'local')))",
            )
            .map_err(|e| e.to_string())?;

        let mut rows = stmt.query([payload.voucher_id]).map_err(|e| e.to_string())?;
        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let nc: String = row.get(0).map_err(|e| e.to_string())?;
            let fe: String = row.get(1).map_err(|e| e.to_string())?;
            (nc, fe)
        } else {
            return Err("Comprobante no encontrado o no está en un estado válido para anular ante SUNAT.".to_string());
        }
    };

    // 2. Descomponer el número de comprobante
    let (serie, tipo_doc, correlativo) =
        partir_id_comprobante(&numero_comprobante)
            .or_else(|_| partir_id_comprobante(&extraer_id_comprobante(&numero_comprobante).unwrap_or(numero_comprobante.clone())))?;

    // 3. Calcular fechas
    let hoy = Local::now().format("%Y-%m-%d").to_string();
    let hoy_compact = Local::now().format("%Y%m%d").to_string();

    // Prefijo: RA para facturas/boletas dentro del plazo de 7 días
    // RC para la Comunicación de Reversión de boletas (misma estructura)
    let prefix = if tipo_doc == "01" { "RA" } else { "RA" };

    // Si no se proporcionó correlativo positivo, calcular el siguiente del día
    let correlativo_final = if payload.correlativo_baja > 0 {
        payload.correlativo_baja as i64
    } else {
        let mut db_guard = db_state
            .db
            .lock()
            .map_err(|_| "Error al bloquear la base de datos".to_string())?;
        let db = db_guard
            .as_mut()
            .ok_or("Base de datos no inicializada".to_string())?;

        let patron = format!("{}-{}%", prefix, hoy_compact);
        let count: i64 = db
            .query_row(
                "SELECT COUNT(*) FROM anulaciones WHERE id_baja LIKE ?1",
                [&patron],
                |r| r.get(0),
            )
            .unwrap_or(0);
        count + 1
    };
    let id_baja = format!("{}-{}-{}", prefix, hoy_compact, correlativo_final);

    // fecha_referencia = fecha de emisión del comprobante (formato YYYY-MM-DD para UBL)
    let fecha_ref = crate::validaciones::parse_fecha_emision_iso(&fecha_emision_str)
        .unwrap_or_else(|_| hoy.clone());

    let doc_baja = DocBaja {
        tipo: tipo_doc.clone(),
        serie: serie.clone(),
        correlativo: correlativo.clone(),
        motivo: payload.motivo.clone(),
    };

    // 4. Firmar el XML con el certificado digital .p12
    let xml_firmado = firmar_xml_baja(
        &id_baja,
        &fecha_ref,
        &hoy,
        &payload.ruc,
        &payload.emisor_razon_social,
        &[doc_baja],
        &payload.p12_bytes,
        &payload.p12_password,
    )?;

    // 5. Enviar via sendSummary → obtener ticket
    let ticket = enviar_summary_sunat(&creds, ambiente, &payload.ruc, &id_baja, &xml_firmado).await?;

    // 6. Esperar y consultar el ticket (hasta 5 intentos con 3s de pausa)
    let mut cdr_result: Option<CdrSunat> = None;
    for intento in 0..5u32 {
        tokio::time::sleep(tokio::time::Duration::from_secs(3 + intento as u64 * 2)).await;
        match consultar_ticket_sunat(&creds, ambiente, &ticket).await {
            Ok(cdr) => {
                cdr_result = Some(cdr);
                break;
            }
            Err(e) if e.contains("código 98") => continue, // aún procesando en SUNAT
            Err(e) => return Err(e),
        }
    }

    let cdr = cdr_result.ok_or_else(|| {
        format!("SUNAT tardó demasiado en procesar la baja. Ticket: {ticket}. Consulte más tarde.")
    })?;

    // 7. Actualizar comprobante y registrar anulación con CDR en la tabla anulaciones
    let estado_validez = if cdr.codigo == "0" { "anulado" } else { "rechazado" };
    // estado_sunat: 3 = anulado confirmado, 2 = rechazado
    let nuevo_estado_sunat = if cdr.codigo == "0" { 3i64 } else { 2i64 };

    {
        let mut db_guard = db_state
            .db
            .lock()
            .map_err(|_| "Error al bloquear la base de datos".to_string())?;
        let db = db_guard
            .as_mut()
            .ok_or("Base de datos no inicializada".to_string())?;

        // Actualizamos estado en comprobantes
        db.execute(
            "UPDATE comprobantes SET estado_validez = ?1, estado_sunat = ?2 WHERE id = ?3",
            rusqlite::params![estado_validez, nuevo_estado_sunat, payload.voucher_id],
        )
        .map_err(|e| e.to_string())?;

        // Si existía un registro local previo, actualizarlo con los datos del CDR y ticket
        let updated = db.execute(
            "UPDATE anulaciones SET
                tipo = 'baja', motivo = ?1, id_baja = ?2, ticket = ?3,
                codigo_cdr = ?4, descripcion_cdr = ?5, xml_cdr = ?6, xml_baja_firmado = ?7,
                fecha_anulacion = datetime('now')
             WHERE comprobante_id = ?8 AND tipo = 'local'",
            rusqlite::params![
                payload.motivo,
                id_baja,
                ticket,
                cdr.codigo,
                cdr.descripcion,
                cdr.xml_cdr,
                xml_firmado,
                payload.voucher_id,
            ],
        ).unwrap_or(0);

        if updated == 0 {
            db.execute(
                "INSERT INTO anulaciones (
                    comprobante_id, tipo, motivo, id_baja, ticket,
                    codigo_cdr, descripcion_cdr, xml_cdr, xml_baja_firmado, fecha_anulacion
                ) VALUES (?1, 'baja', ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'))",
                rusqlite::params![
                    payload.voucher_id,
                    payload.motivo,
                    id_baja,
                    ticket,
                    cdr.codigo,
                    cdr.descripcion,
                    cdr.xml_cdr,
                    xml_firmado,
                ],
            )
            .map_err(|e| format!("Error al registrar anulación en tabla anulaciones: {e}"))?;
        }
    }

    Ok(ResultadoBaja {
        ticket,
        codigo: cdr.codigo,
        descripcion: cdr.descripcion,
    })
}

/// Anulación local de último recurso (sin conexión a SUNAT).
/// Solo marca el comprobante como anulado en la BD local.
/// Úsese cuando se requiere anular por Nota de Crédito (fuera de plazo)
/// o cuando no hay conectividad y se hará la baja manualmente.
#[tauri::command]
pub fn anular_voucher(
    payload: AnularVoucherPayload,
    db_state: State<'_, crate::AppState>,
) -> Result<(), String> {
    if payload.motivo.trim().is_empty() {
        return Err("El motivo de anulación no puede estar vacío.".to_string());
    }

    let mut db_guard = db_state
        .db
        .lock()
        .map_err(|_| "Error al bloquear la base de datos".to_string())?;
    let db = db_guard
        .as_mut()
        .ok_or("Base de datos no inicializada".to_string())?;

    let rows_affected = db
        .execute(
            "UPDATE comprobantes SET estado_validez = 'anulado', estado_sunat = 3 WHERE id = ?1 AND estado_sunat = 1",
            rusqlite::params![payload.voucher_id],
        )
        .map_err(|e| e.to_string())?;

    if rows_affected == 0 {
        return Err(
            "No se pudo anular el comprobante. Verifique que esté aceptado por SUNAT.".to_string(),
        );
    }

    // Registrar anulación local en la tabla anulaciones
    db.execute(
        "INSERT INTO anulaciones (
            comprobante_id, tipo, motivo, fecha_anulacion
        ) VALUES (?1, 'local', ?2, datetime('now'))",
        rusqlite::params![payload.voucher_id, payload.motivo],
    )
    .map_err(|e| format!("Error al registrar anulación en tabla anulaciones: {e}"))?;

    Ok(())
}

/// Payload para anular un comprobante localmente en la base de datos.
#[derive(Debug, Deserialize)]
pub struct AnularVoucherPayload {
    pub voucher_id: i64,
    pub motivo: String,
}

/// Comando Tauri para firmar un XML (o su hash) con el certificado .p12
#[tauri::command]
pub fn firmar_factura_sunat(
    p12_bytes: Vec<u8>,
    p12_password: String,
    hash_xml: Vec<u8>,
) -> Result<(String, String), String> {
    use criptor::{parse_pkcs12, sign_hash_with_key};

    let (cert_info, private_key) =
        parse_pkcs12(&p12_bytes, &p12_password).map_err(|e| e.to_string())?;

    if !cert_info.is_valid_now() {
        return Err("Certificado vencido".into());
    }

    let hash_array: [u8; 32] = hash_xml
        .try_into()
        .map_err(|_| "El hash debe ser exactamente 32 bytes".to_string())?;

    let firma = sign_hash_with_key(&private_key, &hash_array).map_err(|e| e.to_string())?;

    Ok((firma, cert_info.x509_base64))
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// El nombre del ZIP tiene que salir del `cbc:ID` del XML, no de otra fuente.
    /// Con el desfase anterior (`numero_comprobante` a 7 dígitos, XML a 8) SUNAT
    /// devolvía la observación 1036.
    #[test]
    fn el_nombre_del_zip_coincide_con_el_cbc_id() {
        for (serie, tipo, xml_id) in [
            ("B001", "03", "B001-00000003"),
            ("F001", "01", "F001-00000001"),
            ("B999", "03", "B999-00012345"),
        ] {
            let xml = format!("<cbc:ID>{xml_id}</cbc:ID>");
            let (serie_extraida, tipo_extraido, correlativo) =
                partir_id_comprobante(&extraer_id_comprobante(&xml).unwrap()).unwrap();

            assert_eq!(serie_extraida, serie);
            assert_eq!(tipo_extraido, tipo);
            // El número que SUNAT extrae del nombre (la parte tras el tipo de
            // comprobante) debe ser idéntico al `cbc:ID`.
            assert_eq!(format!("{}-{}", serie_extraida, correlativo), xml_id);

            // Y el nombre final del ZIP contiene exactamente el cbc:ID.
            let (zip, _bytes) = empaquetar_xml_sunat(
                "1790550077682",
                &tipo_extraido,
                &serie_extraida,
                &correlativo,
                &xml,
            )
            .unwrap();
            assert!(zip.contains(xml_id), "{zip} no contiene {xml_id}");
            assert!(zip.ends_with(".zip"));
        }
    }

    #[test]
    fn extraer_id_comprobante_tolera_espacios() {
        let xml = "<cbc:ID>\n  F001-00000009\n</cbc:ID>";
        assert_eq!(extraer_id_comprobante(xml).unwrap(), "F001-00000009");
    }

    #[test]
    fn extraer_id_comprobante_falla_si_no_hay_id() {
        assert!(extraer_id_comprobante("<Invoice><cbc:IDPago/></Invoice>").is_err());
        assert!(extraer_id_comprobante("<cbc:ID></cbc:ID>").is_err());
        assert!(extraer_id_comprobante("no hay xml aqui").is_err());
        // El ID de la línea no debe confundirse con el del comprobante.
        let con_dos = "<cbc:ID>B001-00000003</cbc:ID>...<cbc:ID>1</cbc:ID>";
        assert_eq!(extraer_id_comprobante(con_dos).unwrap(), "B001-00000003");
    }

    #[test]
    fn partir_id_comprobante_rechaza_numeros_malos() {
        // Sin correlativo, serie corta, letras donde van dígitos, serie inválida.
        for malo in [
            "B001",
            "B001-",
            "B01-00000001",
            "B001-0000A001",
            "X001-00000001",
            "B00A-00000001",
            "B001-00000001-extra",
            "",
        ] {
            assert!(
                partir_id_comprobante(malo).is_err(),
                "{malo} debió ser rechazado"
            );
        }
    }

    #[test]
    fn test_empaquetar_xml_sunat() {
        let xml_mock = "<Invoice>Contenido de prueba</Invoice>";
        let (filename, zip_bytes) =
            empaquetar_xml_sunat("20123456789", "01", "F001", "00000001", xml_mock).unwrap();

        assert_eq!(filename, "20123456789-01-F001-00000001.zip");
        assert!(zip_bytes.len() > 20);

        let mut archive = ZipArchive::new(Cursor::new(zip_bytes)).unwrap();
        assert_eq!(archive.len(), 1);

        let mut file = archive.by_index(0).unwrap();
        assert_eq!(file.name(), "20123456789-01-F001-00000001.xml");

        let mut extracted = String::new();
        file.read_to_string(&mut extracted).unwrap();
        assert_eq!(extracted, xml_mock);
    }

    #[test]
    fn test_nombre_archivo_boleta() {
        let (filename, _) =
            empaquetar_xml_sunat("20123456789", "03", "B001", "00000042", "<Invoice/>").unwrap();
        assert_eq!(filename, "20123456789-03-B001-00000042.zip");
    }

    #[test]
    fn test_parsear_cdr_aceptado() {
        let cdr_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <ApplicationResponse>
            <cbc:ResponseCode>0</cbc:ResponseCode>
            <cbc:Description>La Factura numero F001-00000001, ha sido aceptada</cbc:Description>
        </ApplicationResponse>"#;

        let cdr = parsear_cdr(cdr_xml).unwrap();
        assert_eq!(cdr.codigo, "0");
        assert!(cdr.descripcion.contains("aceptada"));
    }

    #[test]
    fn test_parsear_cdr_rechazado() {
        let cdr_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <ApplicationResponse>
            <cbc:ResponseCode>2335</cbc:ResponseCode>
            <cbc:Description>El valor del campo firma digital es invalido</cbc:Description>
        </ApplicationResponse>"#;

        let cdr = parsear_cdr(cdr_xml).unwrap();
        assert_eq!(cdr.codigo, "2335");
        assert!(cdr.descripcion.contains("firma digital"));
    }

    #[test]
    fn test_render_xml_baja_tags_namespace() {
        use super::DocBaja;

        let doc = DocBaja {
            tipo: "01".to_string(),
            serie: "F001".to_string(),
            correlativo: "00000002".to_string(),
            motivo: "ERROR EN RUC".to_string(),
        };

        let xml = super::render_xml_baja(
            "RA-20261022-1",
            "2026-10-22",
            "2026-10-22",
            "20123456789",
            "EMPRESA DE PRUEBA SAC",
            &[doc],
            "",
            "",
            "",
        )
        .unwrap();

        assert!(xml.contains("<sac:VoidedDocumentsLine>"));
        assert!(xml.contains("<cbc:DocumentTypeCode>01</cbc:DocumentTypeCode>"));
        assert!(xml.contains("<sac:DocumentSerialID>F001</sac:DocumentSerialID>"));
        assert!(xml.contains("<sac:DocumentNumberID>00000002</sac:DocumentNumberID>"));
        assert!(xml.contains("<sac:VoidReasonDescription><![CDATA[ERROR EN RUC]]></sac:VoidReasonDescription>"));
        assert!(!xml.contains("<cbc:DocumentSerialID>"));
        assert!(!xml.contains("<cbc:DocumentNumberID>"));
    }
}
