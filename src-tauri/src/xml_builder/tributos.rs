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
