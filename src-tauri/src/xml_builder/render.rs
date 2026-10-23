use tera::{Context, Tera};

use super::payload::{payload_para_render, FacturaPayload};

lazy_static::lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = Tera::default();
        // Cargamos la plantilla cruda (en runtime, esto asume que la app la encuentra,
        // pero podemos compilarla en el binario usando macros como include_str! para simplificar el build)
        let factura_str = include_str!("../../templates/factura.xml");
        tera.add_raw_template("factura.xml", factura_str).expect("Fallo al cargar plantilla factura");
        let boleta_str = include_str!("../../templates/boleta.xml");
        tera.add_raw_template("boleta.xml", boleta_str).expect("Fallo al cargar plantilla boleta");
        let nc_str = include_str!("../../templates/nota_credito.xml");
        tera.add_raw_template("nota_credito.xml", nc_str).expect("Fallo al cargar plantilla nota_credito");
        let nd_str = include_str!("../../templates/nota_debito.xml");
        tera.add_raw_template("nota_debito.xml", nd_str).expect("Fallo al cargar plantilla nota_debito");
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
