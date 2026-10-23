use serde::{Deserialize, Serialize};

use super::monto::{redondear, Monto};
use super::tributos::{es_afectacion_valida, tributo_de};

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
    /// ICBPER de la línea: impuesto fijo por bolsa plástica, se suma al IGV para
    /// formar el `cbc:TaxAmount` del `cac:TaxTotal` de la línea.
    #[serde(default)]
    pub icbper: Monto,
    pub afectacion: String,
    pub descripcion: String,
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
    /// Si la línea es una bolsa plástica sujeta al ICBPER. El impuesto lo calcula
    /// el backend con la tasa vigente (no viaja como importe desde el frontend).
    #[serde(default)]
    pub tiene_icbper: bool,
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
    pub fn into_item(self, tasa_icbper: f64) -> Result<FacturaItem, String> {
        let ItemVenta {
            unidad,
            cantidad,
            precio_unitario,
            total,
            afectacion,
            descripcion,
            tiene_icbper,
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

        // El ICBPER es un impuesto fijo por unidad (bolsa), no un porcentaje: se
        // calcula sobre la cantidad y se suma aparte del IGV. Una tasa negativa se
        // ignora para no restar del total.
        let icbper = if tiene_icbper {
            redondear(tasa_icbper.max(0.0) * cantidad.0)
        } else {
            0.0
        };

        Ok(FacturaItem {
            unidad,
            cantidad,
            valor_unitario: valor_unitario.into(),
            precio_unitario,
            subtotal: base.into(),
            igv: igv.into(),
            icbper: icbper.into(),
            afectacion,
            descripcion,
        })
    }
}

/// Normaliza todos los ítems del frontend, en el mismo orden en que llegaron.
///
/// `tasa_icbper` es el impuesto fijo por bolsa vigente (S/ 0.50 por defecto) y se
/// aplica a las líneas marcadas con `tiene_icbper`.
pub fn items_de_venta(items: Vec<ItemVenta>, tasa_icbper: f64) -> Result<Vec<FacturaItem>, String> {
    items
        .into_iter()
        .map(|item| item.into_item(tasa_icbper))
        .collect()
}
