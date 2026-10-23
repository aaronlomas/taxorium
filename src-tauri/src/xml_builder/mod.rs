mod firma;
mod items;
mod monto;
mod payload;
mod render;
mod totales;
mod tributos;

#[cfg(test)]
mod tests;

pub use firma::{
    canonicalize_signed_info, canonicalize_xml, hash_xml_sha256, remove_signature, SIGNED_INFO_ID,
};
pub use items::{items_de_venta, FacturaItem, ItemVenta};
pub use monto::{redondear, Monto};
pub use payload::{FacturaPayload, SubtotalImpuesto};
pub use render::{generate_xml, TERA};
pub use totales::{totales_de, TotalesDocumento};
pub use tributos::{es_afectacion_valida, tributo_de, Tributo};
