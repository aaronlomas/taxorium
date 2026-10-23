use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

impl Default for Monto {
    fn default() -> Self {
        Monto(0.0)
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
/// Redondea a 2 decimales.
///
/// Sumar y restar `f64` reintroduce el residuo binario (`21.239999999999997`) que
/// el XSD de SUNAT no admite en los campos tipados como `xs:decimal`.
pub fn redondear(valor: f64) -> f64 {
    (valor * 100.0).round() / 100.0
}
