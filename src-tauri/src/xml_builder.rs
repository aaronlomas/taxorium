use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tera::{Context, Tera};

#[derive(Debug, Serialize, Deserialize)]
pub struct FacturaItem {
    pub unidad: String,
    pub cantidad: f64,
    pub valor_unitario: f64,
    pub precio_unitario: f64,
    pub subtotal: f64,
    pub igv: f64,
    pub afectacion: String,
    pub descripcion: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub total_gravado: f64,
    pub total_igv: f64,
    pub total_pagar: f64,
    pub items: Vec<FacturaItem>,
}

lazy_static::lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = Tera::default();
        // Cargamos la plantilla cruda (en runtime, esto asume que la app la encuentra,
        // pero podemos compilarla en el binario usando macros como include_str! para simplificar el build)
        let template_str = include_str!("../templates/factura.xml");
        tera.add_raw_template("factura.xml", template_str).expect("Fallo al cargar plantilla");
        tera
    };
}

/// Genera el XML usando la plantilla y los datos
pub fn generate_xml(payload: &FacturaPayload) -> Result<String, String> {
    let context = Context::from_serialize(payload).map_err(|e| e.to_string())?;
    TERA.render("factura.xml", &context).map_err(|e| e.to_string())
}

/// Canonización simple para SUNAT (elimina espacios y saltos de línea entre etiquetas XML)
pub fn canonicalize_xml(xml: &str) -> String {
    let mut result = String::with_capacity(xml.len());
    let mut in_tag = false;
    let mut in_cdata = false;
    let chars: Vec<char> = xml.chars().collect();
    
    let mut i = 0;
    while i < chars.len() {
        // Detectar CDATA
        if i + 8 < chars.len() && chars[i..i+9].iter().collect::<String>() == "<![CDATA[" {
            in_cdata = true;
            result.push_str("<![CDATA[");
            i += 9;
            continue;
        }
        if in_cdata && i + 2 < chars.len() && chars[i..i+3].iter().collect::<String>() == "]]>" {
            in_cdata = false;
            result.push_str("]]>");
            i += 3;
            continue;
        }

        let c = chars[i];
        if in_cdata {
            result.push(c);
        } else {
            if c == '<' {
                in_tag = true;
                result.push(c);
            } else if c == '>' {
                in_tag = false;
                result.push(c);
            } else if in_tag {
                result.push(c);
            } else {
                // Afuera de las etiquetas (text nodes), omitimos si solo hay espacios o saltos
                // Sin embargo, SUNAT espera que no haya NADA de espacios entre `> <`
                if !c.is_whitespace() {
                    result.push(c);
                }
            }
        }
        i += 1;
    }
    result
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

    #[test]
    fn test_canonicalize_and_hash() {
        let payload = FacturaPayload {
            hash_xml: "".to_string(),
            firma: "".to_string(),
            certificado: "".to_string(),
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
            total_gravado: 100.0,
            total_igv: 18.0,
            total_pagar: 118.0,
            items: vec![FacturaItem {
                unidad: "NIU".to_string(),
                cantidad: 1.0,
                valor_unitario: 100.0,
                precio_unitario: 118.0,
                subtotal: 100.0,
                igv: 18.0,
                afectacion: "10".to_string(),
                descripcion: "PRODUCTO X".to_string(),
            }],
        };

        let raw_xml = generate_xml(&payload).unwrap();
        assert!(raw_xml.contains("F001-00000001"));
        
        let c14n_xml = canonicalize_xml(&raw_xml);
        // Asegurar que no hay "\n" ni espacios entre etiquetas XML, salvo dentro de CDATA
        assert!(!c14n_xml.contains(">\n<"));
        assert!(!c14n_xml.contains("> <"));

        let base64_hash = hash_xml_sha256(&c14n_xml);
        assert!(!base64_hash.is_empty());
        println!("Hash generado: {}", base64_hash);
    }
}
