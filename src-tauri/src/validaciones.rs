/// Validaciones de reglas de negocio SUNAT para comprobantes de pago electrónico.
/// Todas las funciones retornan `Ok(())` si la regla se cumple, o `Err(String)` con
/// el mensaje de error que se mostrará al usuario.
use chrono::NaiveDate;

/// Payload unificado que reciben todas las validaciones.
#[derive(Debug, Clone)]
pub struct DatosValidacion<'a> {
    /// "01" = Factura, "03" = Boleta
    pub tipo_comprobante: &'a str,
    /// RUC del emisor (11 dígitos)
    pub emisor_ruc: &'a str,
    /// SchemeID del documento del receptor: "6" = RUC, "1" = DNI, "4" = CE, "7" = PASAPORTE, "0" = Sin doc.
    pub receptor_tipo_doc: &'a str,
    /// Número del documento del receptor (puede estar vacío para consumidor final en boleta)
    pub receptor_num_doc: &'a str,
    /// Base imponible SOLO gravada (sin exoneradas ni inafectas) — para calcular y validar el IGV
    pub total_gravado: f64,
    /// IGV calculado
    pub total_igv: f64,
    /// Total a pagar (gravado_real + exonerado + igv)
    pub total_pagar: f64,
    /// Monto de operaciones exoneradas
    pub op_exoneradas: f64,
    /// Fecha de emisión en formato "YYYY-MM-DD"
    pub fecha_emision: &'a str,
    /// Tipo de operación (ej: "0101" = Venta interna)
    pub tipo_operacion: &'a str,
    /// Cuántas líneas con IGV componen el documento. El margen de la regla del IGV
    /// crece con esta cantidad, porque el reparto base/impuesto redondea por línea.
    pub lineas_gravadas: usize,
}

/// Ejecuta TODAS las validaciones de SUNAT sobre el payload.
/// Devuelve el primer error encontrado o `Ok(())` si todo es correcto.
pub fn validar_comprobante(datos: &DatosValidacion) -> Result<(), String> {
    validar_ruc_emisor(datos.emisor_ruc)?;
    validar_receptor(datos)?;
    validar_igv_con_tolerancia(
        datos.total_gravado,
        datos.total_igv,
        TOLERANCIA_IGV_BASE + TOLERANCIA_IGV_POR_LINEA * datos.lineas_gravadas as f64,
    )?;
    validar_total(
        datos.total_gravado,
        datos.total_igv,
        datos.op_exoneradas,
        datos.total_pagar,
    )?;
    validar_fecha_emision(datos.fecha_emision)?;
    validar_tipo_operacion(datos.tipo_comprobante, datos.tipo_operacion)?;
    Ok(())
}

// =============================================================================
// REGLA 1: RUC del emisor — Módulo 11
// =============================================================================

/// Valida que el RUC del emisor tenga 11 dígitos y pase el algoritmo Módulo 11
/// usado por SUNAT para verificar dígitos de control.
pub fn validar_ruc_emisor(ruc: &str) -> Result<(), String> {
    let digits: Vec<u32> = ruc
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "El RUC del emisor solo debe contener dígitos.".to_string())?;

    if digits.len() != 11 {
        return Err(format!(
            "El RUC del emisor debe tener exactamente 11 dígitos (tiene {}).",
            digits.len()
        ));
    }

    if digits[0] != 1 && digits[0] != 2 {
        return Err(
            "El RUC del emisor debe comenzar con 10 (persona natural) o 20 (empresa).".to_string(),
        );
    }

    // Algoritmo Módulo 11 de SUNAT
    let factores = [5u32, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let suma: u32 = digits[..10]
        .iter()
        .zip(factores.iter())
        .map(|(d, f)| d * f)
        .sum();
    let residuo = 11 - (suma % 11);
    let digito_control = match residuo {
        10 => 0,
        11 => 1,
        r => r,
    };

    if digits[10] != digito_control {
        return Err(format!(
            "El RUC '{}' no es válido (dígito de control incorrecto según Módulo 11).",
            ruc
        ));
    }

    Ok(())
}

// =============================================================================
// REGLA 2: Receptor según tipo de comprobante
// =============================================================================

/// Valida que el receptor cumpla las reglas según el tipo de comprobante:
/// - Factura (01): el receptor DEBE ser una empresa con RUC (tipo_doc = "6")
/// - Boleta (03): si el monto total es > S/.700, el receptor DEBE tener documento
/// - Boleta (03): si el monto total es ≤ S/.700, el receptor puede ser anónimo (tipo "0")
pub fn validar_receptor(datos: &DatosValidacion) -> Result<(), String> {
    match datos.tipo_comprobante {
        "01" => {
            // Factura → receptor debe ser RUC
            if datos.receptor_tipo_doc != "6" {
                return Err(
                    "La Factura solo puede emitirse a un receptor con RUC (tipo de documento '6'). \
                     Para consumidores con DNI use Boleta de Venta."
                    .to_string(),
                );
            }
            if datos.receptor_num_doc.len() != 11
                || !datos.receptor_num_doc.chars().all(|c| c.is_ascii_digit())
            {
                return Err(
                    "El RUC del receptor en una Factura debe tener 11 dígitos numéricos."
                        .to_string(),
                );
            }
        }
        "03" => {
            // Boleta → si el total supera S/.700, el receptor no puede ser anónimo
            if datos.total_pagar > 700.0 {
                if datos.receptor_tipo_doc == "0" || datos.receptor_num_doc.is_empty() {
                    return Err(format!(
                        "Las Boletas con un importe mayor a S/.700 (importe actual: S/.{:.2}) \
                         deben incluir el número de documento del comprador.",
                        datos.total_pagar
                    ));
                }
            }
        }
        tipo => {
            return Err(format!(
                "Tipo de comprobante '{}' no soportado. Use '01' (Factura) o '03' (Boleta).",
                tipo
            ));
        }
    }

    Ok(())
}

// =============================================================================
// REGLA 3: IGV debe ser 18% del valor gravado
// =============================================================================

/// Margen con el que se acepta el IGV declarado.
///
/// El IGV ya no lo declara el frontend: sale de repartir cada línea entre base e
/// impuesto, y ese reparto redondea a dos decimales por línea. Sumar esos redondeos
/// puede apartarse del 18% de la base en hasta un centavo por línea, así que el
/// margen crece con la cantidad de líneas gravadas; con un margen fijo, una factura
/// de 60 ítems sería rechazada por el error de un céntimo que el propio sistema
/// acaba de producir.
const TOLERANCIA_IGV_BASE: f64 = 0.05;
const TOLERANCIA_IGV_POR_LINEA: f64 = 0.01;

/// Valida que el IGV sea el 18% del total gravado con una tolerancia de ±0.05 soles
/// para cubrir redondeos por ítem.
pub fn validar_igv(total_gravado: f64, total_igv: f64) -> Result<(), String> {
    validar_igv_con_tolerancia(total_gravado, total_igv, TOLERANCIA_IGV_BASE)
}

/// Igual que [`validar_igv`] pero con el margen dado, para quien sepa cuántas líneas
/// gravadas componen el comprobante.
pub fn validar_igv_con_tolerancia(
    total_gravado: f64,
    total_igv: f64,
    tolerancia: f64,
) -> Result<(), String> {
    if total_gravado <= 0.0 && total_igv == 0.0 {
        // Comprobante sin gravado (exonerado/inafecto) — IGV en cero es válido
        return Ok(());
    }

    let igv_esperado = (total_gravado * 0.18 * 100.0).round() / 100.0;
    let diferencia = (total_igv - igv_esperado).abs();

    if diferencia > tolerancia {
        return Err(format!(
            "El IGV declarado (S/.{:.2}) no corresponde al 18% del total gravado \
             (S/.{:.2}). IGV esperado: S/.{:.2} (diferencia: S/.{:.2}).",
            total_igv, total_gravado, igv_esperado, diferencia
        ));
    }

    Ok(())
}

// =============================================================================
// REGLA 4: Total = Gravado + IGV (coherencia interna)
// =============================================================================

/// Valida que el total a pagar sea la suma de base_gravada + exoneradas + IGV con tolerancia ±0.05.
pub fn validar_total(
    base_gravada: f64,
    total_igv: f64,
    op_exoneradas: f64,
    total_pagar: f64,
) -> Result<(), String> {
    let total_esperado = ((base_gravada + op_exoneradas + total_igv) * 100.0).round() / 100.0;
    let diferencia = (total_pagar - total_esperado).abs();

    if diferencia > 0.05 {
        return Err(format!(
            "El total a pagar (S/.{:.2}) no coincide con la suma esperada \
             (gravado S/.{:.2} + exonerado S/.{:.2} + IGV S/.{:.2} = S/.{:.2}).",
            total_pagar, base_gravada, op_exoneradas, total_igv, total_esperado
        ));
    }

    Ok(())
}

// =============================================================================
// REGLA 5: Fecha de emisión máximo 3 días hacia atrás
// =============================================================================

/// Formato que exige el XSD de SUNAT para `cbc:IssueDate` (`xs:date`).
const FORMATO_ISO_FECHA: &str = "%Y-%m-%d";

const FORMATOS_FECHA_ENTRADA: &[&str] = &["%Y-%m-%d", "%d/%m/%Y", "%d-%m-%Y"];

/// Interpreta la fecha de emisión que llega del frontend, sea cual sea su formato,
/// y devuelve la fecha ya normalizada a `YYYY-MM-DD`, lista para `cbc:IssueDate`.
///
/// El frontend usa `DD/MM/YYYY` (ver `InputDate.svelte`) y lo concatena con la hora,
/// así que llega como `DD/MM/YYYY HH:MM`. Ese valor no puede ir directo a
/// `cbc:IssueDate`: SUNAT lo rechaza con
/// `element ...IssueDate value '27/09/2026 00:00' is not a valid instance of type
/// {http://www.w3.org/2001/XMLSchema}date`.
pub fn parse_fecha_emision_iso(fecha_emision: &str) -> Result<String, String> {
    Ok(parse_fecha_emision(fecha_emision)?
        .format(FORMATO_ISO_FECHA)
        .to_string())
}

/// Igual que [`parse_fecha_emision_iso`] pero devuelve la fecha sin formatear.
pub fn parse_fecha_emision(fecha_emision: &str) -> Result<NaiveDate, String> {
    // Descartamos la parte horaria: `cbc:IssueDate` es `xs:date` y la hora va aparte
    // en `cbc:IssueTime`.
    let fecha_str = fecha_emision
        .split_whitespace()
        .next()
        .unwrap_or(fecha_emision);

    FORMATOS_FECHA_ENTRADA
        .iter()
        .find_map(|formato| NaiveDate::parse_from_str(fecha_str, formato).ok())
        .ok_or_else(|| {
            format!(
                "Formato de fecha inválido: '{fecha_emision}'. Se esperaba YYYY-MM-DD o DD/MM/YYYY."
            )
        })
}

/// Valida que la fecha de emisión no sea anterior a más de 3 días calendarios
/// desde hoy (regla de SUNAT para la ventana de emisión).
pub fn validar_fecha_emision(fecha_emision: &str) -> Result<(), String> {
    use chrono::{Duration, Local};

    let fecha = parse_fecha_emision(fecha_emision)?;
    let fecha_str = fecha.format("%d/%m/%Y").to_string();

    let hoy = Local::now().date_naive();
    let limite_atras = hoy - Duration::days(3);
    let limite_adelante = hoy + Duration::days(1); // No permitir fechas futuras

    if fecha < limite_atras {
        return Err(format!(
            "La fecha de emisión ({}) es anterior al límite permitido por SUNAT. \
             Solo se pueden emitir comprobantes con fecha de hasta 3 días atrás (desde {}).",
            fecha_str,
            limite_atras.format("%d/%m/%Y")
        ));
    }

    if fecha >= limite_adelante {
        return Err(format!(
            "La fecha de emisión ({}) no puede ser una fecha futura.",
            fecha_str
        ));
    }

    Ok(())
}

// =============================================================================
// REGLA 6: Tipo de operación coherente con el tipo de comprobante
// =============================================================================

/// Valida que el tipo de operación sea compatible con el tipo de comprobante.
/// Las facturas aceptan operaciones de venta a empresas (01xx).
/// Las boletas aceptan operaciones de venta a consumidor final (01xx) y algunas especiales.
/// Fuente: Catálogo 17 de SUNAT.
pub fn validar_tipo_operacion(tipo_comprobante: &str, tipo_operacion: &str) -> Result<(), String> {
    // Tipos de operación válidos para Factura (01)
    const OPERACIONES_FACTURA: &[&str] = &[
        "0101", // Venta interna
        "0112", // Venta interna — sustenta gastos deducibles persona natural
        "0113", // Venta interna — NRUS
        "1001", // Exportación de bienes
        "1002", // Exportación de servicios — Hospedaje no domiciliado
        "1003", // Exportación de servicios — Transporte de navieras
        "1004", // Exportación de servicios — Servicios prestados al íntegro desde el país
        "2001", // Operaciones sujetas al SPOT (detracciones)
    ];

    // Tipos de operación válidos para Boleta (03)
    const OPERACIONES_BOLETA: &[&str] = &[
        "0101", // Venta interna
        "0112", // Venta interna — sustenta gastos deducibles persona natural
        "0113", // Venta interna — NRUS
    ];

    let validos = match tipo_comprobante {
        "01" => OPERACIONES_FACTURA,
        "03" => OPERACIONES_BOLETA,
        _ => return Ok(()), // Tipos desconocidos se validan en otro lugar
    };

    if !validos.contains(&tipo_operacion) {
        let lista = validos.join(", ");
        return Err(format!(
            "El tipo de operación '{}' no es válido para el tipo de comprobante '{}'. \
             Tipos permitidos: {}.",
            tipo_operacion, tipo_comprobante, lista
        ));
    }

    Ok(())
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml_builder::redondear;
    use chrono::Local;

    // ---------- RUC Módulo 11 ----------

    #[test]
    fn test_ruc_valido_empresa() {
        // RUC de empresa conocido y válido (RUC de SUNAT)
        assert!(validar_ruc_emisor("20131312955").is_ok());
    }

    #[test]
    fn test_ruc_invalido_digito_control() {
        // Mismo RUC pero con último dígito incorrecto
        assert!(validar_ruc_emisor("20131312950").is_err());
    }

    #[test]
    fn test_ruc_longitud_incorrecta() {
        assert!(validar_ruc_emisor("2012345678").is_err()); // 10 dígitos
    }

    #[test]
    fn test_ruc_con_letras() {
        assert!(validar_ruc_emisor("2012345678A").is_err());
    }

    // ---------- Receptor ----------

    #[test]
    fn test_factura_requiere_ruc() {
        let datos = DatosValidacion {
            tipo_comprobante: "01",
            emisor_ruc: "20123456789",
            receptor_tipo_doc: "1", // DNI — inválido para factura
            receptor_num_doc: "12345678",
            total_gravado: 100.0,
            total_igv: 18.0,
            total_pagar: 118.0,
            op_exoneradas: 0.0,
            fecha_emision: "2026-09-27",
            tipo_operacion: "0101",
            lineas_gravadas: 1,
        };
        assert!(validar_receptor(&datos).is_err());
    }

    #[test]
    fn test_boleta_anonima_permitida_bajo_700() {
        let datos = DatosValidacion {
            tipo_comprobante: "03",
            emisor_ruc: "20123456789",
            receptor_tipo_doc: "0",
            receptor_num_doc: "",
            total_gravado: 593.22,
            total_igv: 106.78,
            total_pagar: 700.0,
            op_exoneradas: 0.0,
            fecha_emision: "2026-09-27",
            tipo_operacion: "0101",
            lineas_gravadas: 1,
        };
        assert!(validar_receptor(&datos).is_ok());
    }

    #[test]
    fn test_boleta_anonima_rechazada_sobre_700() {
        let datos = DatosValidacion {
            tipo_comprobante: "03",
            emisor_ruc: "20123456789",
            receptor_tipo_doc: "0",
            receptor_num_doc: "",
            total_gravado: 593.23,
            total_igv: 106.78,
            total_pagar: 700.01,
            op_exoneradas: 0.0,
            fecha_emision: "2026-09-27",
            tipo_operacion: "0101",
            lineas_gravadas: 1,
        };
        assert!(validar_receptor(&datos).is_err());
    }

    // ---------- IGV ----------

    #[test]
    fn test_igv_correcto() {
        assert!(validar_igv(100.0, 18.0).is_ok());
    }

    #[test]
    fn test_igv_con_redondeo_permitido() {
        // Diferencia de S/.0.03 — dentro de la tolerancia de ±0.05
        assert!(validar_igv(100.17, 18.03).is_ok());
    }

    #[test]
    fn test_igv_incorrecto() {
        // IGV de 10% — claramente incorrecto
        assert!(validar_igv(100.0, 10.0).is_err());
    }

    /// El IGV de cada línea se redondea a dos decimales, así que en una venta con
    /// muchas líneas la suma puede apartarse un centavo por ítem del 18% exacto. Con
    /// el margen fijo de ±0.05 el sistema rechazaría comprobantes que él mismo generó.
    #[test]
    fn test_igv_de_un_documento_con_muchas_lineas() {
        let lineas = 60;
        let base_linea = redondear(1.00 / 1.18);
        let igv_linea = redondear(1.00 - base_linea);
        let total_gravado = redondear(base_linea * lineas as f64);
        let total_igv = redondear(igv_linea * lineas as f64);

        assert!(total_igv.abs() > 0.0);
        assert!(
            validar_comprobante(&DatosValidacion {
                tipo_comprobante: "03",
                emisor_ruc: "20131312955",
                receptor_tipo_doc: "1",
                receptor_num_doc: "12345678",
                total_gravado,
                total_igv,
                total_pagar: redondear(total_gravado + total_igv),
                op_exoneradas: 0.0,
                fecha_emision: &Local::now().format("%Y-%m-%d").to_string(),
                tipo_operacion: "0101",
                lineas_gravadas: lineas,
            })
            .is_ok(),
            "base {total_gravado} / igv {total_igv}"
        );
    }

    // ---------- Total ----------

    #[test]
    fn test_total_correcto() {
        assert!(validar_total(100.0, 18.0, 0.0, 118.0).is_ok());
    }

    #[test]
    fn test_total_correcto_con_exonerados() {
        // 50 gravado + 9 IGV + 50 exonerado = 109
        assert!(validar_total(50.0, 9.0, 50.0, 109.0).is_ok());
    }

    #[test]
    fn test_total_incorrecto() {
        assert!(validar_total(100.0, 18.0, 0.0, 120.0).is_err());
    }

    // ---------- Fecha ----------

    #[test]
    fn test_fecha_formato_invalido() {
        // Separador no soportado
        assert!(validar_fecha_emision("2026/09/04").is_err());
        // No es una fecha en absoluto
        assert!(validar_fecha_emision("ayer").is_err());
        // Día/mes fuera de rango
        assert!(validar_fecha_emision("2026-13-45").is_err());
    }

    #[test]
    fn test_fecha_formatos_aceptados() {
        // DD/MM/YYYY es válido: la app lo envía en ese formato desde el frontend
        assert!(validar_fecha_emision(&Local::now().format("%d/%m/%Y").to_string()).is_ok());
        assert!(
            validar_fecha_emision(&format!("{} 10:30", Local::now().format("%d/%m/%Y"))).is_ok()
        );
        assert!(validar_fecha_emision(&Local::now().format("%Y-%m-%d").to_string()).is_ok());
    }

    #[test]
    fn test_fecha_muy_antigua() {
        assert!(validar_fecha_emision("2020-01-01").is_err());
    }

    // ---------- Normalización de fecha para el XML ----------

    #[test]
    fn test_parse_fecha_emision_normaliza_a_iso() {
        // Formato que produce `InputDate.svelte` concatenado con la hora.
        assert_eq!(
            parse_fecha_emision_iso("27/09/2026 00:00").unwrap(),
            "2026-09-27"
        );
        assert_eq!(
            parse_fecha_emision_iso("04/09/2026 10:30").unwrap(),
            "2026-09-04"
        );
        // Ya ISO: se deja igual.
        assert_eq!(parse_fecha_emision_iso("2026-09-04").unwrap(), "2026-09-04");
        // Separador de guiones.
        assert_eq!(parse_fecha_emision_iso("04-09-2026").unwrap(), "2026-09-04");
        // Sin hora.
        assert_eq!(parse_fecha_emision_iso("04/09/2026").unwrap(), "2026-09-04");
    }

    #[test]
    fn test_parse_fecha_emision_rechaza_basura() {
        // "2026/09/04" es ambiguo: no se puede distinguir de un formato no soportado.
        assert!(parse_fecha_emision_iso("2026/09/04").is_err());
        assert!(parse_fecha_emision_iso("ayer").is_err());
        assert!(parse_fecha_emision_iso("2026-13-45").is_err());
        assert!(parse_fecha_emision_iso("").is_err());
    }

    // ---------- Tipo de operación ----------

    #[test]
    fn test_operacion_valida_factura() {
        assert!(validar_tipo_operacion("01", "0101").is_ok());
    }

    #[test]
    fn test_operacion_invalida_para_boleta() {
        // Exportación no aplica a boleta
        assert!(validar_tipo_operacion("03", "1001").is_err());
    }
}
