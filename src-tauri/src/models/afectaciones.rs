// Catálogo Nro 7 de SUNAT: tipo de afectación en ventas (IGV)
pub struct AfectacionVenta {
    pub codigo: &'static str,
    pub descripcion: &'static str,
}

pub const TIPOS_AFECTACION_VENTAS: &[AfectacionVenta] = &[
    AfectacionVenta {
        codigo: "10",
        descripcion: "Gravado - Op. Onerosa (IGV 18%)",
    },
    AfectacionVenta {
        codigo: "20",
        descripcion: "Exonerado - Op. Onerosa (IGV 0%)",
    },
    AfectacionVenta {
        codigo: "30",
        descripcion: "Inafecto - Op. Onerosa (IGV 0%)",
    },
    AfectacionVenta {
        codigo: "11",
        descripcion: "Gravado - Retiro (Transferencia Gratuita)",
    },
    AfectacionVenta {
        codigo: "31",
        descripcion: "Inafecto - Retiro (Transferencia Gratuita)",
    },
];

// Clasificación de destino de compras (SIRE - RCE):
// determina si el IGV de la compra va a Crédito Fiscal, Prorrata, Costo/Gasto o No Gravado.
pub struct AfectacionCompra {
    pub codigo: &'static str,
    pub descripcion: &'static str,
}

pub const DESTINO_AFECTACION_COMPRAS: &[AfectacionCompra] = &[
    AfectacionCompra {
        codigo: "GRAVADO_V_GRAVADO",
        descripcion: "Adq. Gravada - Destinada a Ventas Gravadas (Crédito Fiscal)",
    },
    AfectacionCompra {
        codigo: "GRAVADO_V_MIXTO",
        descripcion: "Adq. Gravada - Destinada a Ventas Gravadas y No Gravadas",
    },
    AfectacionCompra {
        codigo: "GRAVADO_V_NOGRAVADO",
        descripcion: "Adq. Gravada - Destinada a Ventas No Gravadas (Costo/Gasto)",
    },
    AfectacionCompra {
        codigo: "NO_GRAVADO",
        descripcion: "Adq. No Gravada - (Exonerada / Inafecta / Sin IGV)",
    },
];
