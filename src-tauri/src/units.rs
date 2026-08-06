pub struct Unit {
    pub code: &'static str,
    pub description: &'static str,
    pub symbol: &'static str,
}

pub const UNITS: &[Unit] = &[
    Unit {
        code: "4A",
        description: "BOBINAS",
        symbol: "BOB",
    },
    Unit {
        code: "AV",
        description: "CÁPSULA",
        symbol: "CAPS",
    },
    Unit {
        code: "BE",
        description: "FARDO",
        symbol: "FARD",
    },
    Unit {
        code: "BG",
        description: "BOLSA",
        symbol: "BOLS",
    },
    Unit {
        code: "BJ",
        description: "BALDE",
        symbol: "BALD",
    },
    Unit {
        code: "BLL",
        description: "BARRILES",
        symbol: "BRL",
    },
    Unit {
        code: "BO",
        description: "BOTELLAS",
        symbol: "BOT",
    },
    Unit {
        code: "BT",
        description: "TORNILLO",
        symbol: "TORN",
    },
    Unit {
        code: "BX",
        description: "CAJA",
        symbol: "CAJ",
    },
    Unit {
        code: "C62",
        description: "PIEZAS",
        symbol: "PZ",
    },
    Unit {
        code: "CA",
        description: "LATAS",
        symbol: "LAT",
    },
    Unit {
        code: "CEN",
        description: "CIENTO DE UNIDADES (CENTENA)",
        symbol: "CTO",
    },
    Unit {
        code: "CH",
        description: "ENVASE",
        symbol: "ENV",
    },
    Unit {
        code: "CJ",
        description: "CONOS",
        symbol: "CN",
    },
    Unit {
        code: "CMK",
        description: "CENTÍMETRO CUADRADO",
        symbol: "CM2",
    },
    Unit {
        code: "CMQ",
        description: "CENTÍMETRO CÚBICO",
        symbol: "CM3",
    },
    Unit {
        code: "CMT",
        description: "CENTÍMETRO LINEAL",
        symbol: "CM",
    },
    Unit {
        code: "CT",
        description: "CARTONES",
        symbol: "CTON",
    },
    Unit {
        code: "CY",
        description: "CILINDRO",
        symbol: "CIL",
    },
    Unit {
        code: "DR",
        description: "TAMBOR",
        symbol: "TAMB",
    },
    Unit {
        code: "DZN",
        description: "DOCENA",
        symbol: "DOC",
    },
    Unit {
        code: "DZP",
        description: "DOCENA POR 10**6",
        symbol: "DOC2",
    },
    Unit {
        code: "FOT",
        description: "PIES",
        symbol: "PIE",
    },
    Unit {
        code: "FTK",
        description: "PIES CUADRADOS",
        symbol: "PIE2",
    },
    Unit {
        code: "FTQ",
        description: "PIES CÚBICOS",
        symbol: "PIE3",
    },
    Unit {
        code: "GLI",
        description: "GALÓN INGLÉS (4,545956 L)",
        symbol: "GL",
    },
    Unit {
        code: "GLL",
        description: "US GALLON (3,7854 L)",
        symbol: "GL",
    },
    Unit {
        code: "GRM",
        description: "GRAMO",
        symbol: "GR",
    },
    Unit {
        code: "GRO",
        description: "GRUESA",
        symbol: "DOC2",
    },
    Unit {
        code: "HD",
        description: "MEDIA DOCENA",
        symbol: "1/2 DOC",
    },
    Unit {
        code: "HLT",
        description: "HECTOLITRO",
        symbol: "HL",
    },
    Unit {
        code: "HT",
        description: "MEDIA HORA",
        symbol: "1/2 H",
    },
    Unit {
        code: "HUR",
        description: "HORA",
        symbol: "HR",
    },
    Unit {
        code: "INH",
        description: "PULGADAS",
        symbol: "INCH",
    },
    Unit {
        code: "JG",
        description: "JARRA",
        symbol: "JARR",
    },
    Unit {
        code: "JR",
        description: "FRASCO",
        symbol: "FCO",
    },
    Unit {
        code: "KGM",
        description: "KILOGRAMO",
        symbol: "KG",
    },
    Unit {
        code: "KT",
        description: "KIT",
        symbol: "KIT",
    },
    Unit {
        code: "KTM",
        description: "KILÓMETRO",
        symbol: "KM",
    },
    Unit {
        code: "KWH",
        description: "KILOVATIO HORA",
        symbol: "KWxH",
    },
    Unit {
        code: "LBR",
        description: "LIBRAS",
        symbol: "LB",
    },
    Unit {
        code: "LEF",
        description: "HOJA",
        symbol: "HOJA",
    },
    Unit {
        code: "LTR",
        description: "LITRO",
        symbol: "LT",
    },
    Unit {
        code: "MGM",
        description: "MILIGRAMO",
        symbol: "MG",
    },
    Unit {
        code: "MLT",
        description: "MILILITRO",
        symbol: "ML",
    },
    Unit {
        code: "MMK",
        description: "MILÍMETRO CUADRADO",
        symbol: "MM2",
    },
    Unit {
        code: "MMQ",
        description: "MILÍMETRO CÚBICO",
        symbol: "MM3",
    },
    Unit {
        code: "MMT",
        description: "MILÍMETRO",
        symbol: "MM",
    },
    Unit {
        code: "MTK",
        description: "METRO CUADRADO",
        symbol: "M2",
    },
    Unit {
        code: "MTQ",
        description: "METRO CÚBICO",
        symbol: "M3",
    },
    Unit {
        code: "MTR",
        description: "METRO",
        symbol: "M",
    },
    Unit {
        code: "MWH",
        description: "MEGAVATIO HORA",
        symbol: "MWxH",
    },
    Unit {
        code: "NIU",
        description: "UNIDAD (BIENES)",
        symbol: "UND",
    },
    Unit {
        code: "ONZ",
        description: "ONZAS",
        symbol: "ONZ",
    },
    Unit {
        code: "PF",
        description: "PALETAS",
        symbol: "PAL",
    },
    Unit {
        code: "PG",
        description: "PLACAS",
        symbol: "PLAC",
    },
    Unit {
        code: "PK",
        description: "PAQUETE",
        symbol: "PQT",
    },
    Unit {
        code: "PR",
        description: "PAR",
        symbol: "PAR",
    },
    Unit {
        code: "QD",
        description: "CUARTO DE DOCENA",
        symbol: "1/4 DOC",
    },
    Unit {
        code: "RD",
        description: "VARILLA",
        symbol: "VAR",
    },
    Unit {
        code: "RL",
        description: "CARRETE",
        symbol: "CRR",
    },
    Unit {
        code: "RM",
        description: "RESMA",
        symbol: "RESM",
    },
    Unit {
        code: "SA",
        description: "SACO",
        symbol: "SCO",
    },
    Unit {
        code: "SEC",
        description: "SEGUNDO",
        symbol: "SEG",
    },
    Unit {
        code: "SET",
        description: "JUEGO",
        symbol: "JGO",
    },
    Unit {
        code: "ST",
        description: "PLIEGO",
        symbol: "PLGO",
    },
    Unit {
        code: "STN",
        description: "TONELADA CORTA",
        symbol: "TON",
    },
    Unit {
        code: "TNE",
        description: "TONELADAS",
        symbol: "T",
    },
    Unit {
        code: "TU",
        description: "TUBOS",
        symbol: "TB",
    },
    Unit {
        code: "U2",
        description: "TABLETA O BLISTER",
        symbol: "BLIS",
    },
    Unit {
        code: "UM",
        description: "MILLÓN DE UNIDADES",
        symbol: "MILL",
    },
    Unit {
        code: "YDK",
        description: "YARDA CUADRADA",
        symbol: "YD2",
    },
    Unit {
        code: "YRD",
        description: "YARDA",
        symbol: "YD",
    },
    Unit {
        code: "ZZ",
        description: "UNIDAD (SERVICIOS)",
        symbol: "SERV",
    },
];
