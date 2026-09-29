use std::fmt;

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};
use tera::{Context, Tera};

use crate::c14n;

/// `Id` de `ds:SignedInfo` en las plantillas. Lo exige SUNAT y permite localizar el
/// elemento al canonizar sin depender de la posición en el texto.
pub const SIGNED_INFO_ID: &str = "SignedInfo";

/// Importe que siempre se serializa con 2 decimales.
///
/// Los importes se manejan como `f64`, y Tera los renderiza con `Display`, que puede
/// emitir cosas como `21.239999999999997` o `15.450000000000001`. El XSD de SUNAT
/// tipa esos campos como `xs:decimal`, que no admite esa representación, así que el
/// redondeo tiene que ocurrir antes de llegar a la plantilla y no confiar en lo que
/// envíe el frontend (que solo redondea el IGV, no la base gravada).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Monto(pub f64);

impl Monto {
    pub fn new(valor: f64) -> Self {
        Monto(valor)
    }
}

impl From<f64> for Monto {
    fn from(valor: f64) -> Self {
        Monto(valor)
    }
}

impl fmt::Display for Monto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

impl Serialize for Monto {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{:.2}", self.0))
    }
}

/// Acepta cualquier forma numérica que pueda emitir el deserializador, no solo `f64`:
/// `serde_json` enruta un entero positivo como `visit_u64` y uno negativo como
/// `visit_i64`, así que con solo esos dos casos un payload con `"cantidad": 8` fallaba
/// con `invalid type: integer 8, expected un número o una cadena numérica`.
macro_rules! visitor_num {
    ($($metodo:ident : $tipo:ident),* $(,)?) => {
        $(
            fn $metodo<E: serde::de::Error>(self, v: $tipo) -> Result<Monto, E> {
                Ok(Monto(v as f64))
            }
        )*
    };
}

impl<'de> Deserialize<'de> for Monto {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'d> serde::de::Visitor<'d> for Visitor {
            type Value = Monto;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("un número o una cadena numérica")
            }

            visitor_num! {
                visit_f32: f32,
                visit_f64: f64,
                visit_i8: i8,
                visit_i16: i16,
                visit_i32: i32,
                visit_i64: i64,
                visit_i128: i128,
                visit_u8: u8,
                visit_u16: u16,
                visit_u32: u32,
                visit_u64: u64,
                visit_u128: u128,
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Monto, E> {
                v.trim()
                    .parse()
                    .map(Monto)
                    .map_err(|_| E::custom(format!("importe inválido: {v}")))
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

/// Línea del comprobante ya normalizada, tal como la consume la plantilla.
///
/// `subtotal` es la base imponible (sin IGV) e `igv` el tributo de esa línea; ambos
/// salen de [`ItemVenta::into_item`], que garantiza que `subtotal + igv` devuelve
/// exactamente el importe que capturó el usuario.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FacturaItem {
    pub unidad: String,
    pub cantidad: Monto,
    pub valor_unitario: Monto,
    pub precio_unitario: Monto,
    pub subtotal: Monto,
    pub igv: Monto,
    pub afectacion: String,
    pub descripcion: String,
}

/// Redondea a 2 decimales.
///
/// Sumar y restar `f64` reintroduce el residuo binario (`21.239999999999997`) que
/// el XSD de SUNAT no admite en los campos tipados como `xs:decimal`.
pub fn redondear(valor: f64) -> f64 {
    (valor * 100.0).round() / 100.0
}
#[derive(Debug, Clone, PartialEq)]
pub struct Tributo {
    pub porcentaje: &'static str,
    pub esquema_id: &'static str,
    pub esquema_nombre: &'static str,
    pub tipo_impuesto: &'static str,
    /// Catálogo N° 16 de SUNAT: código de categoría del tributo.
    /// "S" = gravado con IGV, "E" = exonerado, "O" = inafecto, "K" = ISC.
    pub categoria_id: &'static str,
}

impl Tributo {
    /// La tasa del tributo en tanto por uno (`0.18` para el IGV).
    pub fn tasa(&self) -> f64 {
        self.porcentaje.parse().unwrap_or(0.0) / 100.0
    }

    /// Si el tributo grava de verdad el importe de la línea. Un tributo al 0% o el
    /// ISC del bolsón se declaran con monto 0.00 sin que SUNAT lo rechace.
    pub fn grava(&self) -> bool {
        self.tasa() > 0.0
    }
}

/// Resuelve el tributo que corresponde a una afectación del catálogo 07.
///
/// SUNAT exige esquemas distintos según el tipo de operación (Catálogo N° 05):
///   - IGV gravado (10-17): esquema 1000, nombre "IGV", categoría "S"
///   - Exonerado (20-29):   esquema 9997, nombre "EXO", categoría "E"
///   - Inafecto  (30-49):   esquema 9998, nombre "INA", categoría "O"
///   - ICBPer   (71-76):    esquema 7152, nombre "ICBPER", categoría "K"
///
/// Usar 1000 (IGV) con TaxAmount 0.00 para exonerados/inafectos es exactamente
/// lo que provoca la observación 3111 de SUNAT.
pub fn tributo_de(afectacion: &str) -> Tributo {
    const IGV_18: Tributo = Tributo {
        porcentaje: "18.00",
        esquema_id: "1000",
        esquema_nombre: "IGV",
        tipo_impuesto: "VAT",
        categoria_id: "S",
    };
    const EXONERADO: Tributo = Tributo {
        porcentaje: "0.00",
        esquema_id: "9997",
        esquema_nombre: "EXO",
        tipo_impuesto: "VAT",
        categoria_id: "E",
    };
    const INAFECTO: Tributo = Tributo {
        porcentaje: "0.00",
        esquema_id: "9998",
        esquema_nombre: "INA",
        tipo_impuesto: "FRE",
        categoria_id: "O",
    };
    const ICBPER: Tributo = Tributo {
        porcentaje: "0.00",
        esquema_id: "7152",
        esquema_nombre: "ICBPER",
        tipo_impuesto: "OTH",
        categoria_id: "K",
    };

    match afectacion {
        // Gravado - Operación Onerosa y sus variantes onerosas.
        "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" => IGV_18,
        // Exonerado (20-29): Catálogo 07, esquema 9997.
        "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" => EXONERADO,
        _ if afectacion.starts_with('2') => EXONERADO,
        // Inafecto (30-49): Catálogo 07, esquema 9998.
        "30" | "31" | "32" | "33" | "34" | "35" | "36" | "37" => INAFECTO,
        _ if afectacion.starts_with('3') => INAFECTO,
        _ if afectacion.starts_with('4') => INAFECTO,
        // ICBPer: impuesto al bolsón plástico, esquema 7152.
        "71" | "72" | "73" | "74" | "75" | "76" => ICBPER,
        // Código desconocido → tratar como gravado (más seguro que declarar 0% erróneamente).
        _ => IGV_18,
    }
}

/// `cbc:TaxExemptionReasonCode` es obligatorio, así que una afectación vacía o con
/// formato inesperado se rechaza acá en vez de emitir un XML que SUNAT no valida.
pub fn es_afectacion_valida(afectacion: &str) -> bool {
    let codigo = afectacion.trim();
    codigo.len() == 2 && codigo.chars().all(|c| c.is_ascii_digit())
}

/// Ítem tal como lo captura el usuario en la pantalla de ventas.
///
/// El precio se ingresa final, con IGV incluido, así que el frontend solo tiene que
/// mandar el importe de la línea: la base imponible y el IGV los reparte el backend
/// con el tributo de la afectación (ver [`ItemVenta::into_item`]).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemVenta {
    pub unidad: String,
    pub cantidad: Monto,
    /// Precio unitario final (con IGV), tal como se capturó.
    pub precio_unitario: Monto,
    /// Importe total de la línea con IGV incluido.
    pub total: Monto,
    pub afectacion: String,
    pub descripcion: String,
}

impl ItemVenta {
    /// Convierte el ítem del frontend en la línea del XML, repartiendo el importe
    /// total entre base imponible e IGV.
    ///
    /// Que el reparto viva acá y no en el frontend es lo que evita la observación
    /// 3111: antes el cliente decidía "es gravado" comparando `afectacion === '10'`
    /// mientras el backend declaraba IGV al 18% para toda la familia `10`-`17`, así
    /// que una línea con afectación `11` (o con la afectación vacía, que el
    /// frontend convertía en `10` al enviar) quedaba con Tributo 1000 y
    /// `cbc:TaxAmount` en 0.00.
    pub fn into_item(self) -> Result<FacturaItem, String> {
        let ItemVenta {
            unidad,
            cantidad,
            precio_unitario,
            total,
            afectacion,
            descripcion,
        } = self;

        let afectacion = afectacion.trim().to_string();
        if !es_afectacion_valida(&afectacion) {
            return Err(format!(
                "El ítem '{descripcion}' no tiene una afectación del IGV válida ('{afectacion}'). \
                 Debe ser un código de 2 dígitos del catálogo 07 de SUNAT."
            ));
        }
        if total.0 <= 0.0 {
            return Err(format!(
                "El ítem '{descripcion}' tiene un importe de 0.00. \
                 Una línea sin valor no se puede declarar ante SUNAT."
            ));
        }

        let tributo = tributo_de(&afectacion);
        // Se redondea la base primero y el IGV es la diferencia: de ese modo
        // `base + IGV` devuelve exactamente el total capturado, sin el centavo de
        // descuadre que aparecería al redondear el IGV por separado.
        let base = if tributo.grava() {
            redondear(total.0 / (1.0 + tributo.tasa()))
        } else {
            redondear(total.0)
        };
        let igv = redondear(total.0 - base);
        let valor_unitario = if cantidad.0 > 0.0 {
            redondear(base / cantidad.0)
        } else {
            base
        };

        Ok(FacturaItem {
            unidad,
            cantidad,
            valor_unitario: valor_unitario.into(),
            precio_unitario,
            subtotal: base.into(),
            igv: igv.into(),
            afectacion,
            descripcion,
        })
    }
}

/// Normaliza todos los ítems del frontend, en el mismo orden en que llegaron.
pub fn items_de_venta(items: Vec<ItemVenta>) -> Result<Vec<FacturaItem>, String> {
    items.into_iter().map(ItemVenta::into_item).collect()
}

/// Totales del documento deducidos de las líneas ya normalizadas.
#[derive(Debug, Clone, Copy)]
pub struct TotalesDocumento {
    /// Suma de las bases de todas las líneas. Es lo que va a
    /// `cac:LegalMonetaryTotal/cbc:LineExtensionAmount`, que en UBL incluye también
    /// las bases exoneradas e inafectas.
    pub base_total: f64,
    /// Base imponible de las líneas con IGV, la única contra la que se valida el IGV.
    pub base_gravada: f64,
    /// Bases exoneradas, inafectas y de ICBPer: no generan IGV pero sí suman al total.
    pub base_no_gravada: f64,
    pub igv: f64,
    pub total_pagar: f64,
    /// Cuántas de esas líneas pagan IGV, para dimensionar la tolerancia con la que
    /// se valida que el IGV total sea el 18% de la base.
    pub lineas_gravadas: usize,
}

/// Suma las líneas para obtener los totales del comprobante.
///
/// Salen de las líneas y no de los campos que envía el frontend: antes `gravado`
/// viajaba como suma de importes con IGV incluido, y `LineExtensionAmount` no
/// cuadraba con la suma de las bases de las líneas.
pub fn totales_de(items: &[FacturaItem]) -> TotalesDocumento {
    let mut totales = TotalesDocumento {
        base_total: 0.0,
        base_gravada: 0.0,
        base_no_gravada: 0.0,
        igv: 0.0,
        total_pagar: 0.0,
        lineas_gravadas: 0,
    };
    for item in items {
        let base = item.subtotal.0;
        if tributo_de(&item.afectacion).grava() {
            totales.base_gravada += base;
            totales.lineas_gravadas += 1;
        } else {
            totales.base_no_gravada += base;
        }
        totales.base_total += base;
        totales.igv += item.igv.0;
        totales.total_pagar += base + item.igv.0;
    }
    TotalesDocumento {
        base_total: redondear(totales.base_total),
        base_gravada: redondear(totales.base_gravada),
        base_no_gravada: redondear(totales.base_no_gravada),
        igv: redondear(totales.igv),
        total_pagar: redondear(totales.total_pagar),
        lineas_gravadas: totales.lineas_gravadas,
    }
}

/// Documento completo listo para firmar. Los totales se calculan con
/// [`totales_de`] a partir de las líneas, nunca los envía el frontend.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FacturaPayload {
    pub hash_xml: String,
    pub firma: String,
    pub certificado: String,
    pub serie: String,
    pub correlativo: String,
    pub fecha_emision: String,
    pub hora_emision: String,
    pub tipo_comprobante: String,
    pub moneda: String,
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
}

/// Payload con los tributos derivados de cada ítem y los subtotales del documento
/// agrupados por tributo. `generate_xml` siempre trabaja sobre esta vista, así que
/// no se puede renderizar un documento con un `cac:TaxSubtotal` incoherente.
#[derive(Debug, Serialize)]
struct PayloadParaRender<'a> {
    #[serde(flatten)]
    payload: &'a FacturaPayload,
    items: Vec<ItemConTributo>,
    subtotales_impuestos: Vec<SubtotalImpuesto>,
    total_impuestos: Monto,
}

lazy_static::lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = Tera::default();
        // Cargamos la plantilla cruda (en runtime, esto asume que la app la encuentra,
        // pero podemos compilarla en el binario usando macros como include_str! para simplificar el build)
        let factura_str = include_str!("../templates/factura.xml");
        tera.add_raw_template("factura.xml", factura_str).expect("Fallo al cargar plantilla factura");
        let boleta_str = include_str!("../templates/boleta.xml");
        tera.add_raw_template("boleta.xml", boleta_str).expect("Fallo al cargar plantilla boleta");
        tera
    };
}

/// Genera el XML usando la plantilla y los datos
pub fn generate_xml(payload: &FacturaPayload) -> Result<String, String> {
    let para_render = payload_para_render(payload)?;
    let context = Context::from_serialize(para_render).map_err(|e| e.to_string())?;
    let template_name = if payload.tipo_comprobante == "03" {
        "boleta.xml"
    } else {
        "factura.xml"
    };
    // Re-render con las plantillas corregidas para firma
    TERA.render(template_name, &context)
        .map_err(|e| format!("No se pudo generar el XML de {template_name}: {e}"))
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
fn payload_para_render(payload: &FacturaPayload) -> Result<PayloadParaRender<'_>, String> {
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

    let total_impuestos = Monto(redondear(subtotales.iter().map(|s| s.impuesto.0).sum()));

    Ok(PayloadParaRender {
        payload,
        items,
        subtotales_impuestos: subtotales,
        total_impuestos,
    })
}

/// Canonización C14N 1.0 (`xml-c14n-20010315`, sin comentarios) del documento completo.
pub fn canonicalize_xml(xml: &str) -> Result<String, String> {
    c14n::canonicalize(xml).map_err(|e| format!("Error al canonizar el XML: {e}"))
}

/// Canonización C14N 1.0 de un único elemento (`ds:SignedInfo`), que es el
/// pre-imagen que firma el XMLDSIG. C14N inclusivo exige declarar en ese elemento
/// todos los espacios de nombres heredados, así que hay que canonizar sobre el
/// documento completo y no sobre el fragmento recortado.
pub fn canonicalize_signed_info(xml: &str) -> Result<String, String> {
    c14n::canonicalize_element_by_id(xml, SIGNED_INFO_ID)
        .map_err(|e| format!("Error al canonizar ds:SignedInfo: {e}"))
}

/// Quita el elemento `<ds:Signature>` completo del XML firmado.
///
/// Es la transformación `enveloped-signature` que declara el `SignedInfo`: lo que
/// queda es exactamente el documento al que se le calcula el `DigestValue`.
pub fn remove_signature(xml: &str) -> Result<String, String> {
    let start = xml
        .find("<ds:Signature")
        .ok_or("El XML no contiene el elemento ds:Signature")?;
    let end = xml[start..]
        .find("</ds:Signature>")
        .ok_or("El XML tiene un ds:Signature sin cerrar")?
        + start
        + "</ds:Signature>".len();
    let mut out = String::with_capacity(xml.len());
    out.push_str(&xml[..start]);
    out.push_str(&xml[end..]);
    Ok(out)
}

/// Calcula el Hash SHA-256 del XML canonizado y lo retorna en Base64
pub fn hash_xml_sha256(canonical_xml: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(canonical_xml.as_bytes());
    let result = hasher.finalize();
    general_purpose::STANDARD.encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload_prueba() -> FacturaPayload {
        FacturaPayload {
            hash_xml: String::new(),
            firma: String::new(),
            certificado: String::new(),
            serie: "F001".to_string(),
            correlativo: "00000001".to_string(),
            fecha_emision: "2023-10-24".to_string(),
            hora_emision: "15:30:00".to_string(),
            tipo_comprobante: "01".to_string(),
            moneda: "PEN".to_string(),
            emisor_ruc: "20123456789".to_string(),
            emisor_razon_social: "EMPRESA DE PRUEBA SAC".to_string(),
            emisor_ubigeo: "150101".to_string(),
            emisor_direccion: "AV LIMA 123".to_string(),
            cliente_tipo_doc: "6".to_string(),
            cliente_num_doc: "20987654321".to_string(),
            cliente_nombre: "CLIENTE DE PRUEBA SRL".to_string(),
            total_gravado: 100.0.into(),
            total_igv: 18.0.into(),
            total_pagar: 118.0.into(),
            items: vec![FacturaItem {
                unidad: "NIU".to_string(),
                cantidad: 1.0.into(),
                valor_unitario: 100.0.into(),
                precio_unitario: 118.0.into(),
                subtotal: 100.0.into(),
                igv: 18.0.into(),
                afectacion: "10".to_string(),
                descripcion: "PRODUCTO X".to_string(),
            }],
        }
    }

    /// El `DigestValue` tiene que ser reproducible por cualquier validador XMLDSIG
    /// (SUNAT recalcula el digest por su cuenta). Los valores esperados se generaron
    /// con `lxml.etree.tostring(method="c14n", exclusive=False, with_comments=False)`
    /// sobre el mismo documento de prueba.
    const EXPECTED_DIGEST: &str = "X2bdcarDsVxzGlm5IARjCwE5rHeB62xBARWEkcUy5Hc=";
    const EXPECTED_SIGNED_INFO_DIGEST: &str = "wMYbu6wsfdv1pBs3l+Ow7I5hrXzPf1+/bJDk4nn0o30=";

    fn payload_firmado() -> FacturaPayload {
        let mut payload = payload_prueba();
        payload.hash_xml = EXPECTED_DIGEST.to_string();
        payload.firma = "FIRMA".to_string();
        payload.certificado = "Q0VSVA==".to_string();
        payload
    }

    #[test]
    fn digest_coincide_con_una_implementacion_de_referencia() {
        let firmado = generate_xml(&payload_firmado()).unwrap();
        let sin_firma = remove_signature(&firmado).unwrap();
        let digest = hash_xml_sha256(&canonicalize_xml(&sin_firma).unwrap());

        assert_eq!(digest, EXPECTED_DIGEST);
    }

    /// Lo mismo para el pre-imagen que se firma: si difiere, SUNAT rechaza la
    /// firma aunque el `DigestValue` sea correcto.
    #[test]
    fn pre_imagen_de_la_firma_coincide_con_una_implementacion_de_referencia() {
        let firmado = generate_xml(&payload_firmado()).unwrap();
        let digest = hash_xml_sha256(&canonicalize_signed_info(&firmado).unwrap());
        assert_eq!(digest, EXPECTED_SIGNED_INFO_DIGEST);
    }

    #[test]
    fn quitar_la_firma_no_altera_el_resto_del_documento() {
        let firmado = generate_xml(&payload_firmado()).unwrap();
        let sin_firma = remove_signature(&firmado).unwrap();
        assert!(!sin_firma.contains("ds:Signature"));
        assert!(sin_firma.contains("F001-00000001"));
        assert_eq!(
            sin_firma.matches("<ext:UBLExtensions>").count(),
            firmado.matches("<ext:UBLExtensions>").count()
        );
    }

    /// C14N inclusivo: `ds:SignedInfo` debe declarar los espacios de nombres
    /// heredados de `<Invoice>`, si no la firma no valida en SUNAT.
    #[test]
    fn signed_info_arrastra_los_namespaces_heredados() {
        let firmado = generate_xml(&payload_firmado()).unwrap();
        let canonico = canonicalize_signed_info(&firmado).unwrap();

        assert!(canonico.starts_with("<ds:SignedInfo "));
        assert!(canonico.contains("xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\""));
        assert!(
            canonico.contains("xmlns=\"urn:oasis:names:specification:ubl:schema:xsd:Invoice-2\"")
        );
        assert!(canonico.contains(
            "xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\""
        ));
        assert!(canonico.ends_with("</ds:SignedInfo>"));
    }

    #[test]
    fn generate_xml_incluye_la_razon_social_y_el_id() {
        let raw_xml = generate_xml(&payload_prueba()).unwrap();
        assert!(raw_xml.contains("F001-00000001"));
        assert!(raw_xml.contains("EMPRESA DE PRUEBA SAC"));
    }

    /// Los importes se tipan como `xs:decimal`. Renderizar un `f64` con `Display`
    /// produce `21.239999999999997`, que no es un decimal válido y hace que SUNAT
    /// rechace el documento completo.
    #[test]
    fn los_importes_se_emiten_con_dos_decimales() {
        let mut payload = payload_prueba();
        // Valores típicos de la aritmética de coma flotante del frontend.
        payload.total_gravado = (10.30f64 * 1.5).into();
        payload.total_igv = (2.727).into();
        payload.total_pagar = 0.1f64.into();
        payload.items[0].cantidad = 1.5.into();
        payload.items[0].subtotal = (10.30f64 * 1.5).into();
        payload.items[0].igv = 2.727.into();

        let xml = generate_xml(&payload).unwrap();

        assert_eq!(valor_de(&xml, "cbc:PayableAmount"), "0.10");
        assert_eq!(valor_de(&xml, "cbc:LineExtensionAmount"), "15.45");
        assert_eq!(valor_de(&xml, "cbc:InvoicedQuantity"), "1.50");
        assert_eq!(valor_de(&xml, "cbc:TaxAmount"), "2.73");

        // Ningún importe puede llevar notación exponente ni residuo de coma flotante.
        assert!(!xml.contains("999999"), "hay un residuo binario en el XML");
        assert!(
            !xml.contains("e0") && !xml.contains("E0"),
            "notación exponente en el XML"
        );
    }

    /// `cbc:IssueDate` es `xs:date` y `cbc:IssueTime` es `xs:time`. El frontend
    /// entrega la fecha como `DD/MM/YYYY HH:MM`, así que tiene que normalizarse antes
    /// de llegar a la plantilla; si no, SUNAT responde
    /// `IssueDate value '27/09/2026 00:00' is not a valid instance of type ...date`.
    #[test]
    fn issue_date_y_issue_time_cumplen_el_xsd() {
        use crate::validaciones::parse_fecha_emision_iso;
        use chrono::Local;

        // Exactamente lo que arma `VoucherModal.svelte` a partir del calendario.
        let fecha_ui = Local::now().format("%d/%m/%Y").to_string();
        let mut payload = payload_prueba();
        payload.fecha_emision = parse_fecha_emision_iso(&format!("{fecha_ui} 00:00")).unwrap();
        payload.hora_emision = Local::now().format("%H:%M:%S").to_string();

        let xml = generate_xml(&payload).unwrap();

        let issue_date = valor_de(&xml, "cbc:IssueDate");
        assert!(
            chrono::NaiveDate::parse_from_str(issue_date, "%Y-%m-%d").is_ok(),
            "cbc:IssueDate '{issue_date}' no es un xs:date válido"
        );

        let issue_time = valor_de(&xml, "cbc:IssueTime");
        assert!(
            chrono::NaiveTime::parse_from_str(issue_time, "%H:%M:%S").is_ok(),
            "cbc:IssueTime '{issue_time}' no es un xs:time válido"
        );
    }

    /// El frontend manda los importes como JSON crudo, así que llegan como enteros
    /// cuando no tienen parte decimal (`"cantidad": 8`). `serde_json` enruta eso a
    /// `visit_u64`, y si el visitor no lo contemplaba la creación del comprobante
    /// moría con `invalid type: integer 8, expected un número o una cadena numérica`.
    #[test]
    fn monto_acepta_todas_las_formas_numericas_de_json() {
        assert_eq!(serde_json::from_str::<Monto>("8").unwrap().0, 8.0);
        assert_eq!(serde_json::from_str::<Monto>("0").unwrap().0, 0.0);
        assert_eq!(serde_json::from_str::<Monto>("-3").unwrap().0, -3.0);
        assert_eq!(serde_json::from_str::<Monto>("1.5").unwrap().0, 1.5);
        assert_eq!(serde_json::from_str::<Monto>("-0.25").unwrap().0, -0.25);
        assert_eq!(serde_json::from_str::<Monto>("\"8.75\"").unwrap().0, 8.75);
        assert_eq!(serde_json::from_str::<Monto>("\" 12 \"").unwrap().0, 12.0);
        assert_eq!(serde_json::from_str::<Monto>("1e3").unwrap().0, 1000.0);

        // Un precio sin decimales llega como entero, no como cadena.
        assert!(serde_json::from_str::<Monto>("\"abc\"").is_err());
        assert!(serde_json::from_str::<Monto>("true").is_err());
        assert!(serde_json::from_str::<Monto>("null").is_err());
    }

    /// El payload real de un ítem tal como lo manda `VoucherModal.svelte`.
    #[test]
    fn deserialize_de_una_factura_con_importes_enteros() {
        let json = r#"{
            "unidad": "NIU", "cantidad": 8, "valor_unitario": 8,
            "precio_unitario": 9.44, "subtotal": 8, "igv": 1.44,
            "afectacion": "10", "descripcion": "PRODUCTO"
        }"#;

        let item: FacturaItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.cantidad.0, 8.0);
        assert_eq!(item.subtotal.0, 8.0);
        // Y al reemitir, el XML lleva los 2 decimales que exige el XSD.
        assert_eq!(item.cantidad.to_string(), "8.00");
        assert_eq!(item.subtotal.to_string(), "8.00");
    }

    /// Ítem de prueba tal como lo envía el frontend: precio con IGV incluido.
    fn item_venta(afectacion: &str, cantidad: f64, unitario: f64) -> ItemVenta {
        ItemVenta {
            unidad: "NIU".to_string(),
            cantidad: cantidad.into(),
            precio_unitario: unitario.into(),
            total: (cantidad * unitario).into(),
            afectacion: afectacion.to_string(),
            descripcion: format!("ITEM {afectacion}"),
        }
    }

    /// Ítem ya normalizado por el backend, como llega a la plantilla.
    fn item(afectacion: &str, cantidad: f64, unitario: f64) -> FacturaItem {
        item_venta(afectacion, cantidad, unitario)
            .into_item()
            .unwrap()
    }

    fn payload_con_items(items: Vec<FacturaItem>) -> FacturaPayload {
        let mut payload = payload_prueba();
        let totales = totales_de(&items);
        payload.total_gravado = totales.base_total.into();
        payload.total_igv = totales.igv.into();
        payload.total_pagar = totales.total_pagar.into();
        payload.items = items;
        payload
    }

    fn payload_con_items_venta(items: Vec<ItemVenta>) -> FacturaPayload {
        let normalizados: Vec<FacturaItem> =
            items.into_iter().map(|i| i.into_item().unwrap()).collect();
        payload_con_items(normalizados)
    }

    /// Devuelve el texto de todos los `cac:TaxSubtotal` de un elemento, para poder
    /// comprobar bloque por bloque lo que se declara.
    fn subtotales_de(xml: &str) -> Vec<String> {
        let mut bloques = Vec::new();
        let mut resto = xml;
        while let Some(inicio) = resto.find("<cac:TaxSubtotal>") {
            let desde = inicio + "<cac:TaxSubtotal>".len();
            let fin = resto[desde..]
                .find("</cac:TaxSubtotal>")
                .expect("TaxSubtotal sin cerrar")
                + desde;
            bloques.push(resto[desde..fin].to_string());
            resto = &resto[fin..];
        }
        bloques
    }

    /// Un ítem gravado siempre lleva IGV positivo, y un exonerado declara 0% en vez
    /// de mentir con un 18% que no existe.
    #[test]
    fn el_tributo_se_deriva_de_la_afectacion() {
        assert_eq!(
            tributo_de("10"),
            Tributo {
                porcentaje: "18.00",
                esquema_id: "1000",
                esquema_nombre: "IGV",
                tipo_impuesto: "VAT",
                categoria_id: "S",
            }
        );
        assert_eq!(tributo_de("20").porcentaje, "0.00");
        // Exonerado usa esquema 9997, NO 1000 (observación 3111 de SUNAT).
        assert_eq!(tributo_de("20").esquema_id, "9997");
        assert_eq!(tributo_de("20").categoria_id, "E");
        // Inafecto usa esquema 9998.
        assert_eq!(tributo_de("30").esquema_id, "9998");
        assert_eq!(tributo_de("30").categoria_id, "O");
        assert_eq!(tributo_de("31").porcentaje, "0.00");
        assert_eq!(tributo_de("40").porcentaje, "0.00");
        // ICBPer va con el esquema 7152, no con IGV.
        assert_eq!(tributo_de("71").esquema_id, "7152");
        assert_eq!(tributo_de("72").esquema_id, "7152");
        // Un código desconocido se trata como gravado, que es lo que no rechaza SUNAT.
        assert_eq!(tributo_de("99").porcentaje, "18.00");
    }

    /// Regresión de la observación 3111: la plantilla fijaba 18% de IGV para todas
    /// las líneas, y un ítem exonerado quedaba con monto 0.00 y Tributo 1000.
    #[test]
    fn un_item_exonerado_no_declara_igv_al_18() {
        let payload = payload_con_items(vec![item("20", 1.0, 100.0)]);
        let xml = generate_xml(&payload).unwrap();

        for bloque in subtotales_de(&xml) {
            assert!(
                !bloque.contains("<cbc:Percent>18.00</cbc:Percent>"),
                "{bloque}"
            );
            assert!(
                bloque.contains("<cbc:Percent>0.00</cbc:Percent>"),
                "{bloque}"
            );
            assert!(
                bloque.contains(">20</cbc:TaxExemptionReasonCode>"),
                "{bloque}"
            );
        }
        assert!(xml.contains("<cbc:TaxAmount currencyID=\"PEN\">0.00</cbc:TaxAmount>"));
    }

    /// Un ítem gravado debe llevar un IGV distinto de cero (observación 3111).
    #[test]
    fn un_item_gravado_lleva_igv_distinto_de_cero() {
        let payload = payload_con_items(vec![item("10", 1.0, 118.0)]);
        let xml = generate_xml(&payload).unwrap();

        for bloque in subtotales_de(&xml) {
            assert!(
                bloque.contains("<cbc:Percent>18.00</cbc:Percent>"),
                "{bloque}"
            );
            assert!(!bloque.contains(">0.00</cbc:TaxAmount>"), "{bloque}");
        }
        assert!(xml.contains("<cbc:TaxAmount currencyID=\"PEN\">18.00</cbc:TaxAmount>"));
    }

    /// Regresión del reporte de SUNAT: `Error en la linea: 1 Tributo: 1000: 3111
    /// (nodo: "cac:TaxSubtotal/cbc:TaxAmount" valor: "0.00")`.
    ///
    /// El frontend decidía el IGV con `afectacion === '10'` y mandaba 0 para todo lo
    /// demás, mientras el backend declaraba IGV al 18% para toda la familia `10`-`17`.
    /// Con la afectación `11` (Gravado - Retiro) la línea salía con Tributo 1000 y
    /// monto 0.00. Ahora el reparto sale del tributo, así que da igual el código.
    #[test]
    fn una_afectacion_gravada_distinta_de_10_tambien_lleva_igv() {
        for afectacion in ["10", "11", "12", "13", "14", "15", "16", "17"] {
            let payload = payload_con_items_venta(vec![item_venta(afectacion, 2.0, 118.0)]);
            let item = &payload.items[0];
            assert_eq!(item.subtotal.0, 200.0, "afectación {afectacion}");
            assert_eq!(item.igv.0, 36.0, "afectación {afectacion}");

            let xml = generate_xml(&payload).unwrap();
            for bloque in subtotales_de(&xml) {
                assert!(
                    !bloque.contains(">0.00</cbc:TaxAmount>"),
                    "afectación {afectacion}: {bloque}"
                );
                assert!(
                    bloque.contains(
                        "<cbc:ID schemeID=\"UN/ECE 5153\" schemeAgencyID=\"6\">1000</cbc:ID>"
                    ),
                    "afectación {afectacion}: {bloque}"
                );
            }
        }
    }

    /// El reparto base/IGV es responsabilidad del backend, no del cliente.
    #[test]
    fn el_importe_de_la_linea_se_reparte_segun_su_tributo() {
        // 118.00 con IGV incluido -> base 100.00 + IGV 18.00
        let gravado = item("10", 1.0, 118.0);
        assert_eq!(gravado.subtotal.0, 100.0);
        assert_eq!(gravado.igv.0, 18.0);
        assert_eq!(gravado.valor_unitario.0, 100.0);

        // Exonerado, inafecto y el resto de gravados no pagan IGV.
        for afectacion in ["20", "21", "30", "31", "40"] {
            let item = item(afectacion, 1.0, 118.0);
            assert_eq!(item.subtotal.0, 118.0, "afectación {afectacion}");
            assert_eq!(item.igv.0, 0.0, "afectación {afectacion}");
            assert_eq!(item.valor_unitario.0, 118.0, "afectación {afectacion}");
        }
    }

    /// Base e IGV se redondean de forma que la línea siempre vuelve a sumar el
    /// importe que capturó el usuario, aunque el reparto no sea exacto.
    #[test]
    fn la_linia_siempre_devuelve_el_importe_capturado() {
        for (cantidad, unitario) in [(1.0, 10.30), (3.0, 10.30), (7.0, 0.85), (2.0, 33.33)] {
            let venta = item_venta("10", cantidad, unitario);
            let total = venta.total.0;
            let item = venta.into_item().unwrap();
            assert_eq!(
                redondear(item.subtotal.0 + item.igv.0),
                redondear(total),
                "{cantidad} x {unitario}"
            );
        }
    }

    /// Una afectación vacía se reporta con el detalle del ítem en vez de emitir un
    /// `cbc:TaxExemptionReasonCode` en blanco que SUNAT no valida.
    #[test]
    fn una_afectacion_vacia_o_invalida_se_rechaza() {
        for afectacion in ["", "  ", "1", "ABC", "100"] {
            let error = item_venta(afectacion, 1.0, 118.0)
                .into_item()
                .expect_err("debería rechazar la afectación");
            assert!(
                error.contains("ITEM") && error.contains("afectación del IGV válida"),
                "{afectacion}: {error}"
            );
        }
        assert!(es_afectacion_valida("10"));
        assert!(es_afectacion_valida(" 20 "));
        assert!(!es_afectacion_valida(""));
    }

    /// Una línea de importe cero tampoco es declarable.
    #[test]
    fn una_linea_en_cero_se_rechaza() {
        let error = item_venta("10", 0.0, 0.0)
            .into_item()
            .expect_err("debería rechazar el importe en cero");
        assert!(error.contains("0.00"), "{error}");
    }

    /// Última red antes de gastar un ticket en SUNAT: si una línea llega ya
    /// normalizada pero con Tributo 1000 y monto 0.00, se corta con el detalle.
    #[test]
    fn una_linea_gravada_sin_igv_no_llega_al_xml() {
        let mut roto = item("10", 1.0, 118.0);
        roto.igv = 0.0.into();
        let payload = payload_con_items(vec![roto]);

        let error = generate_xml(&payload).expect_err("debería cortar antes de firmar");
        assert!(error.contains("3111"), "{error}");
        assert!(error.contains("Línea 1"), "{error}");
    }

    /// Los totales del comprobante salen de las líneas, no de lo que declare el
    /// cliente: `LineExtensionAmount` es la suma de las bases de todas ellas.
    #[test]
    fn los_totales_suman_las_lineas() {
        let items = vec![
            item("10", 2.0, 118.0), // base 200.00, igv 36.00
            item("20", 1.0, 50.0),  // base  50.00, igv  0.00
            item("10", 1.0, 59.0),  // base  50.00, igv  9.00
        ];
        let totales = totales_de(&items);

        assert_eq!(totales.base_gravada, 250.0);
        assert_eq!(totales.base_no_gravada, 50.0);
        assert_eq!(totales.base_total, 300.0);
        assert_eq!(totales.igv, 45.0);
        assert_eq!(totales.total_pagar, 345.0);

        // Y el documento firmado lleva esos mismos importes.
        let payload = payload_con_items(items);
        let xml = generate_xml(&payload).unwrap();
        assert!(xml.contains(
            "<cbc:LineExtensionAmount currencyID=\"PEN\">300.00</cbc:LineExtensionAmount>"
        ));
        assert!(xml.contains("<cbc:PayableAmount currencyID=\"PEN\">345.00</cbc:PayableAmount>"));
    }

    /// Una venta mixta necesita un `cac:TaxSubtotal` por grupo, no uno solo.
    #[test]
    fn una_venta_mixta_agrupa_los_subtotales_por_tributo() {
        let payload = payload_con_items(vec![
            item("10", 2.0, 118.0), // gravado  -> base 200.00, igv 36.00
            item("20", 1.0, 50.0),  // exonerado-> base  50.00, igv  0.00
            item("10", 1.0, 59.0),  // gravado  -> base  50.00, igv  9.00
        ]);
        let xml = generate_xml(&payload).unwrap();

        let bloques = subtotales_de(&xml);
        // 2 en la cabecera (por grupo) + 1 por cada una de las 3 líneas.
        assert_eq!(bloques.len(), 5, "{}", xml);

        let cabecera = &bloques[0..2];
        let mut bases: Vec<f64> = Vec::new();
        let mut impuestos: Vec<f64> = Vec::new();
        for bloque in cabecera {
            bases.push(
                valor_de(bloque, "cbc:TaxableAmount")
                    .parse::<f64>()
                    .unwrap(),
            );
            impuestos.push(valor_de(bloque, "cbc:TaxAmount").parse::<f64>().unwrap());
        }

        // Los gravados se suman en un solo subtotal.
        let mut suma = 0.0;
        for (base, impuesto) in bases.iter().zip(impuestos.iter()) {
            if *impuesto > 0.0 {
                assert!((base - impuesto / 0.18).abs() < 0.01);
                suma += *impuesto;
            }
        }
        assert!((suma - 45.00).abs() < 0.001, "IGV total {suma}");
        assert_eq!(bases.iter().sum::<f64>().round(), 300.0);

        // El total de impuestos de la cabecera cuadra con la suma de los grupos.
        let cabecera_tax = valor_de(&xml, "cac:TaxTotal").trim_start();
        assert!(
            cabecera_tax.starts_with("<cbc:TaxAmount currencyID=\"PEN\">45.00</cbc:TaxAmount>"),
            "{cabecera_tax}"
        );
    }

    /// El total del documento debe ser la suma de las líneas, sin residuo de coma
    /// flotante aunque las bases se sumen en cascada.
    #[test]
    fn los_subtotales_agrupados_no_arrastran_residuo_binario() {
        let payload = payload_con_items((0..6).map(|_| item("10", 3.0, 10.30)).collect());
        let xml = generate_xml(&payload).unwrap();

        assert!(!xml.contains("999999"), "hay un residuo binario en el XML");
        // 6 lineas x 3 x 10.30 = 185.40 con IGV incluido. Cada línea declara una base
        // de 26.19 y un IGV de 4.71, así que la cabecera suma lo que realmente dice el
        // documento (6 x 26.19 = 157.14), no el importe sin redondear (157.12), que
        // no aparece en ninguna línea.
        assert_eq!(valor_de(&xml, "cbc:TaxableAmount"), "157.14");
        assert_eq!(valor_de(&xml, "cbc:TaxAmount"), "28.26");
        assert!(xml.contains(">157.14</cbc:TaxableAmount>"));
        // Y la base más el IGV siguen dando el total cobrado.
        assert!(xml.contains("<cbc:PayableAmount currencyID=\"PEN\">185.40</cbc:PayableAmount>"));
    }

    /// Devuelve el texto del primer elemento `<nombre ...>`, tolerando atributos.
    fn valor_de<'a>(xml: &'a str, nombre: &str) -> &'a str {
        let apertura = xml
            .find(&format!("<{nombre} "))
            .or_else(|| xml.find(&format!("<{nombre}>")))
            .unwrap_or_else(|| panic!("no se encontró <{nombre}>"));
        let ini = xml[apertura..].find('>').expect("etiqueta sin cerrar") + apertura + 1;
        let fin = ini + xml[ini..].find(&format!("</{nombre}>")).expect(nombre);
        &xml[ini..fin]
    }
}
