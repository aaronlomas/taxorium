pub struct Moneda {
    pub codigo: &'static str,
    pub descripcion: &'static str,
    pub simbolo: &'static str,
}

pub const TIPO_MONEDA: &[Moneda] = &[
    Moneda {
        codigo: "PEN",
        descripcion: "Soles",
        simbolo: "S/",
    },
    Moneda {
        codigo: "USD",
        descripcion: "Dólares",
        simbolo: "$",
    },
];
