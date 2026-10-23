use base64::{engine::general_purpose, Engine as _};
use sha2::{Digest, Sha256};

use crate::c14n;

/// `Id` de `ds:SignedInfo` en las plantillas. Lo exige SUNAT y permite localizar el
/// elemento al canonizar sin depender de la posición en el texto.
pub const SIGNED_INFO_ID: &str = "SignedInfo";
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
