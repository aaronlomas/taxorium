use super::items::FacturaItem;
use super::monto::redondear;
use super::tributos::tributo_de;

/// Totales del documento deducidos de las líneas ya normalizadas.
#[derive(Debug, Clone, Copy)]
pub struct TotalesDocumento {
    /// Suma de las bases de todas las líneas. Es lo que va a
    /// `cac:LegalMonetaryTotal/cbc:LineExtensionAmount`, que en UBL incluye también
    /// las bases exoneradas e inafectas.
    pub base_total: f64,
    /// Base imponible de las líneas con IGV, la única contra la que se valida el IGV.
    pub base_gravada: f64,
    /// Bases exoneradas e inafectas: no generan IGV pero sí suman al total.
    pub base_no_gravada: f64,
    pub igv: f64,
    /// Impuesto a las bolsas plásticas (ICBPER) del documento. No forma parte de
    /// ninguna base: se suma al total por separado.
    pub icbper: f64,
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
        icbper: 0.0,
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
        totales.icbper += item.icbper.0;
        totales.total_pagar += base + item.igv.0 + item.icbper.0;
    }
    TotalesDocumento {
        base_total: redondear(totales.base_total),
        base_gravada: redondear(totales.base_gravada),
        base_no_gravada: redondear(totales.base_no_gravada),
        igv: redondear(totales.igv),
        icbper: redondear(totales.icbper),
        total_pagar: redondear(totales.total_pagar),
        lineas_gravadas: totales.lineas_gravadas,
    }
}
