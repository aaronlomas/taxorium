use serde::{Deserialize, Serialize};

use super::items::FacturaItem;
use super::monto::{redondear, Monto};
use super::tributos::tributo_de;

/// Documento completo listo para firmar. Los totales se calculan con
/// [`crate::xml_builder::totales_de`] a partir de las líneas, nunca los envía el frontend.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FacturaPayload {
    pub hash_xml: String,
    pub firma: String,
    pub certificado: String,
    pub serie: String,
    pub correlativo: String,
    /// Código de local anexo del emisor (catálogo 22, ej. "0000").
    pub establecimiento: String,
    pub fecha_emision: String,
    pub hora_emision: String,
    /// Forma de pago SUNAT: "Contado" o "Credito".
    #[serde(default = "forma_pago_default")]
    pub forma_pago: String,
    /// Fecha de vencimiento ("YYYY-MM-DD") para operaciones al crédito.
    #[serde(default)]
    pub fecha_vencimiento: Option<String>,
    /// Monto neto pendiente para operaciones al crédito.
    #[serde(default)]
    pub monto_neto_pendiente: Option<Monto>,
    pub tipo_comprobante: String,
    /// Tipo de operación SUNAT (catálogo 51, ej. "0101" = venta interna).
    pub tipo_operacion: String,
    pub moneda: String,
    /// Tipo de cambio a soles. Solo se emite en el XML cuando la moneda no es PEN.
    pub tipo_cambio: Option<Monto>,
    pub emisor_ruc: String,
    pub emisor_razon_social: String,
    pub emisor_ubigeo: String,
    pub emisor_direccion: String,
    pub cliente_tipo_doc: String,
    pub cliente_num_doc: String,
    pub cliente_nombre: String,
    /// Suma de las bases de todas las líneas; alimenta
    /// `cac:LegalMonetaryTotal/cbc:LineExtensionAmount`.
    pub total_gravado: Monto,
    pub total_igv: Monto,
    pub total_pagar: Monto,
    pub items: Vec<FacturaItem>,
}

/// Un `cac:TaxSubtotal` del documento: los ítems con el mismo tributo se agrupan.
#[derive(Debug, Serialize, Clone)]
pub struct SubtotalImpuesto {
    pub base: Monto,
    pub impuesto: Monto,
    pub porcentaje: String,
    pub afectacion: String,
    pub esquema_id: String,
    pub esquema_nombre: String,
    pub tipo_impuesto: String,
    pub categoria_id: String,
}

/// Vista de un ítem con su tributo ya resuelto, tal como la consume la plantilla.
#[derive(Debug, Serialize)]
struct ItemConTributo {
    #[serde(flatten)]
    item: FacturaItem,
    porcentaje: String,
    esquema_id: String,
    esquema_nombre: String,
    tipo_impuesto: String,
    categoria_id: String,
    /// Si la línea lleva ICBPER, para que la plantilla emita el `cac:TaxSubtotal`
    /// adicional con el esquema 7152.
    tiene_icbper: bool,
    /// Cantidad de bolsas como entero (el `cbc:BaseUnitMeasure` del ICBPER es
    /// `xs:decimal` de tipo entero; SUNAT rechaza valores con decimales).
    bolsas: String,
    /// Tarifa por unidad del ICBPER (`cbc:PerUnitAmount`).
    tasa_icbper: Monto,
    /// Suma de todos los tributos de la línea (IGV + ICBPER) para el
    /// `cbc:TaxAmount` del `cac:TaxTotal` de la línea.
    impuesto_total: Monto,
}

/// Convierte una cantidad en el entero de bolsas que espera SUNAT, conservando
/// decimales solo si la cantidad no es entera.
fn formato_bolsas(cantidad: f64) -> String {
    if (cantidad.fract()).abs() < 1e-9 {
        format!("{}", cantidad.round() as i64)
    } else {
        format!("{cantidad:.2}")
    }
}

/// Payload con los tributos derivados de cada ítem y los subtotales del documento
/// agrupados por tributo. `generate_xml` siempre trabaja sobre esta vista, así que
/// no se puede renderizar un documento con un `cac:TaxSubtotal` incoherente.
#[derive(Debug, Serialize)]
pub(super) struct PayloadParaRender<'a> {
    #[serde(flatten)]
    payload: &'a FacturaPayload,
    items: Vec<ItemConTributo>,
    subtotales_impuestos: Vec<SubtotalImpuesto>,
    /// ICBPER total del documento; `None` cuando no hay bolsas, para omitir el
    /// `cac:TaxSubtotal` de ICBPER en la cabecera.
    subtotal_icbper: Option<Monto>,
    total_impuestos: Monto,
}
/// Resuelve el tributo de cada ítem y agrupa los subtotales del documento.
///
/// El UBL exige un `cac:TaxSubtotal` por combinación de tributo (esquema, categoría
/// y porcentaje). Con una venta mixta hay que emitir uno por cada grupo, no un único
/// bloque que declare IGV 18% para todo.
///
/// Devuelve `Err` si alguna línea declara un tributo que no corresponde con su
/// importe. Es la red de seguridad de la observación 3111: aunque una línea llegue ya
/// normalizada, una base con IGV 18% y monto de impuesto 0.00 se detecta aquí, con el
/// detalle de la línea, en vez de gastar un ticket de SUNAT para enterarse.
pub(super) fn payload_para_render(
    payload: &FacturaPayload,
) -> Result<PayloadParaRender<'_>, String> {
    let items: Vec<ItemConTributo> = payload
        .items
        .iter()
        .map(|item| {
            let tributo = tributo_de(&item.afectacion);
            ItemConTributo {
                item: item.clone(),
                porcentaje: tributo.porcentaje.to_string(),
                esquema_id: tributo.esquema_id.to_string(),
                esquema_nombre: tributo.esquema_nombre.to_string(),
                tipo_impuesto: tributo.tipo_impuesto.to_string(),
                categoria_id: tributo.categoria_id.to_string(),
                tiene_icbper: item.icbper.0 > 0.0,
                bolsas: formato_bolsas(item.cantidad.0),
                tasa_icbper: Monto(if item.cantidad.0 > 0.0 {
                    redondear(item.icbper.0 / item.cantidad.0)
                } else {
                    0.0
                }),
                impuesto_total: Monto(redondear(item.igv.0 + item.icbper.0)),
            }
        })
        .collect();

    // Una línea con tributo positivo y monto de impuesto 0.00 es exactamente la
    // observación 3111 ("El monto de afectación de IGV por línea debe ser diferente
    // a 0.00"), así que se corta acá nombrando la línea y el descuadre detectado.
    for (indice, item) in payload.items.iter().enumerate() {
        let tributo = tributo_de(&item.afectacion);
        if tributo.grava() && item.subtotal.0 > 0.0 && item.igv.0 == 0.0 {
            return Err(format!(
                "Línea {} ('{}'): declara {} al {}% sobre una base de S/.{:.2} pero el \
                 monto del impuesto es 0.00. SUNAT lo rechaza con la observación 3111.",
                indice + 1,
                item.descripcion,
                tributo.esquema_nombre,
                tributo.porcentaje,
                item.subtotal.0
            ));
        }
    }

    // Un subtotal por afectación distinta, conservando el orden de aparición.
    let mut subtotales: Vec<SubtotalImpuesto> = Vec::new();
    for (item, _) in payload.items.iter().zip(items.iter()) {
        let tributo = tributo_de(&item.afectacion);
        match subtotales
            .iter_mut()
            .find(|s| s.afectacion == item.afectacion && s.esquema_id == tributo.esquema_id)
        {
            Some(existente) => {
                existente.base = Monto(redondear(existente.base.0 + item.subtotal.0));
                existente.impuesto = Monto(redondear(existente.impuesto.0 + item.igv.0));
            }
            None => subtotales.push(SubtotalImpuesto {
                base: item.subtotal,
                impuesto: item.igv,
                porcentaje: tributo.porcentaje.to_string(),
                afectacion: item.afectacion.clone(),
                esquema_id: tributo.esquema_id.to_string(),
                esquema_nombre: tributo.esquema_nombre.to_string(),
                tipo_impuesto: tributo.tipo_impuesto.to_string(),
                categoria_id: tributo.categoria_id.to_string(),
            }),
        }
    }

    // El ICBPER no se agrupa por afectación: es un tributo propio del documento.
    let total_icbper = redondear(payload.items.iter().map(|i| i.icbper.0).sum());
    let subtotal_icbper = if total_icbper > 0.0 {
        Some(Monto(total_icbper))
    } else {
        None
    };

    let total_impuestos = Monto(redondear(
        subtotales.iter().map(|s| s.impuesto.0).sum::<f64>() + total_icbper,
    ));

    Ok(PayloadParaRender {
        payload,
        items,
        subtotales_impuestos: subtotales,
        subtotal_icbper,
        total_impuestos,
    })
}

fn forma_pago_default() -> String {
    "Contado".to_string()
}

