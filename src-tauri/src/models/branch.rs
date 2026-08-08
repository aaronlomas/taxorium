pub struct Sede {
    pub codigo: &'static str,
    pub label: &'static str,
}

pub const SEDES: &[Sede] = &[
    Sede {
        codigo: "0000",
        label: "Oficina Principal",
    },
    Sede {
        codigo: "0001",
        label: "Sede 01 (Sucursal)",
    },
    Sede {
        codigo: "0002",
        label: "Sede 02 (Agencia)",
    },
    Sede {
        codigo: "0003",
        label: "Almacén / Depósito",
    },
];