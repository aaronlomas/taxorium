pub struct Unit {
    pub codigo: &'static str,
    pub descripcion: &'static str,
    pub simbolo: &'static str,
}

pub const UNITS: &[Unit] = &[
    Unit {
        codigo: "4A",
        descripcion: "BOBINAS",
        simbolo: "BOB",
    },
    Unit {
        codigo: "AV",
        descripcion: "CÁPSULA",
        simbolo: "CAPS",
    },
    Unit {
        codigo: "BE",
        descripcion: "FARDO",
        simbolo: "FARD",
    },
    Unit {
        codigo: "BG",
        descripcion: "BOLSA",
        simbolo: "BOLS",
    },
    Unit {
        codigo: "BJ",
        descripcion: "BALDE",
        simbolo: "BALD",
    },
    Unit {
        codigo: "BLL",
        descripcion: "BARRILES",
        simbolo: "BRL",
    },
    Unit {
        codigo: "BO",
        descripcion: "BOTELLAS",
        simbolo: "BOT",
    },
    Unit {
        codigo: "BT",
        descripcion: "TORNILLO",
        simbolo: "TORN",
    },
    Unit {
        codigo: "BX",
        descripcion: "CAJA",
        simbolo: "CAJ",
    },
    Unit {
        codigo: "C62",
        descripcion: "PIEZAS",
        simbolo: "PZ",
    },
    Unit {
        codigo: "CA",
        descripcion: "LATAS",
        simbolo: "LAT",
    },
    Unit {
        codigo: "CEN",
        descripcion: "CIENTO DE UNIDADES (CENTENA)",
        simbolo: "CTO",
    },
    Unit {
        codigo: "CH",
        descripcion: "ENVASE",
        simbolo: "ENV",
    },
    Unit {
        codigo: "CJ",
        descripcion: "CONOS",
        simbolo: "CN",
    },
    Unit {
        codigo: "CMK",
        descripcion: "CENTÍMETRO CUADRADO",
        simbolo: "CM2",
    },
    Unit {
        codigo: "CMQ",
        descripcion: "CENTÍMETRO CÚBICO",
        simbolo: "CM3",
    },
    Unit {
        codigo: "CMT",
        descripcion: "CENTÍMETRO LINEAL",
        simbolo: "CM",
    },
    Unit {
        codigo: "CT",
        descripcion: "CARTONES",
        simbolo: "CTON",
    },
    Unit {
        codigo: "CY",
        descripcion: "CILINDRO",
        simbolo: "CIL",
    },
    Unit {
        codigo: "DR",
        descripcion: "TAMBOR",
        simbolo: "TAMB",
    },
    Unit {
        codigo: "DZN",
        descripcion: "DOCENA",
        simbolo: "DOC",
    },
    Unit {
        codigo: "DZP",
        descripcion: "DOCENA POR 10**6",
        simbolo: "DOC2",
    },
    Unit {
        codigo: "FOT",
        descripcion: "PIES",
        simbolo: "PIE",
    },
    Unit {
        codigo: "FTK",
        descripcion: "PIES CUADRADOS",
        simbolo: "PIE2",
    },
    Unit {
        codigo: "FTQ",
        descripcion: "PIES CÚBICOS",
        simbolo: "PIE3",
    },
    Unit {
        codigo: "GLI",
        descripcion: "GALÓN INGLÉS (4,545956 L)",
        simbolo: "GL",
    },
    Unit {
        codigo: "GLL",
        descripcion: "US GALLON (3,7854 L)",
        simbolo: "GL",
    },
    Unit {
        codigo: "GRM",
        descripcion: "GRAMO",
        simbolo: "GR",
    },
    Unit {
        codigo: "GRO",
        descripcion: "GRUESA",
        simbolo: "DOC2",
    },
    Unit {
        codigo: "HD",
        descripcion: "MEDIA DOCENA",
        simbolo: "1/2 DOC",
    },
    Unit {
        codigo: "HLT",
        descripcion: "HECTOLITRO",
        simbolo: "HL",
    },
    Unit {
        codigo: "HT",
        descripcion: "MEDIA HORA",
        simbolo: "1/2 H",
    },
    Unit {
        codigo: "HUR",
        descripcion: "HORA",
        simbolo: "HR",
    },
    Unit {
        codigo: "INH",
        descripcion: "PULGADAS",
        simbolo: "INCH",
    },
    Unit {
        codigo: "JG",
        descripcion: "JARRA",
        simbolo: "JARR",
    },
    Unit {
        codigo: "JR",
        descripcion: "FRASCO",
        simbolo: "FCO",
    },
    Unit {
        codigo: "KGM",
        descripcion: "KILOGRAMO",
        simbolo: "KG",
    },
    Unit {
        codigo: "KT",
        descripcion: "KIT",
        simbolo: "KIT",
    },
    Unit {
        codigo: "KTM",
        descripcion: "KILÓMETRO",
        simbolo: "KM",
    },
    Unit {
        codigo: "KWH",
        descripcion: "KILOVATIO HORA",
        simbolo: "KWxH",
    },
    Unit {
        codigo: "LBR",
        descripcion: "LIBRAS",
        simbolo: "LB",
    },
    Unit {
        codigo: "LEF",
        descripcion: "HOJA",
        simbolo: "HOJA",
    },
    Unit {
        codigo: "LTR",
        descripcion: "LITRO",
        simbolo: "LT",
    },
    Unit {
        codigo: "MGM",
        descripcion: "MILIGRAMO",
        simbolo: "MG",
    },
    Unit {
        codigo: "MLT",
        descripcion: "MILILITRO",
        simbolo: "ML",
    },
    Unit {
        codigo: "MMK",
        descripcion: "MILÍMETRO CUADRADO",
        simbolo: "MM2",
    },
    Unit {
        codigo: "MMQ",
        descripcion: "MILÍMETRO CÚBICO",
        simbolo: "MM3",
    },
    Unit {
        codigo: "MMT",
        descripcion: "MILÍMETRO",
        simbolo: "MM",
    },
    Unit {
        codigo: "MTK",
        descripcion: "METRO CUADRADO",
        simbolo: "M2",
    },
    Unit {
        codigo: "MTQ",
        descripcion: "METRO CÚBICO",
        simbolo: "M3",
    },
    Unit {
        codigo: "MTR",
        descripcion: "METRO",
        simbolo: "M",
    },
    Unit {
        codigo: "MWH",
        descripcion: "MEGAVATIO HORA",
        simbolo: "MWxH",
    },
    Unit {
        codigo: "NIU",
        descripcion: "UNIDAD",
        simbolo: "UND",
    },
    Unit {
        codigo: "ONZ",
        descripcion: "ONZAS",
        simbolo: "ONZ",
    },
    Unit {
        codigo: "PF",
        descripcion: "PALETAS",
        simbolo: "PAL",
    },
    Unit {
        codigo: "PG",
        descripcion: "PLACAS",
        simbolo: "PLAC",
    },
    Unit {
        codigo: "PK",
        descripcion: "PAQUETE",
        simbolo: "PQT",
    },
    Unit {
        codigo: "PR",
        descripcion: "PAR",
        simbolo: "PAR",
    },
    Unit {
        codigo: "QD",
        descripcion: "CUARTO DE DOCENA",
        simbolo: "1/4 DOC",
    },
    Unit {
        codigo: "RD",
        descripcion: "VARILLA",
        simbolo: "VAR",
    },
    Unit {
        codigo: "RL",
        descripcion: "CARRETE",
        simbolo: "CRR",
    },
    Unit {
        codigo: "RM",
        descripcion: "RESMA",
        simbolo: "RESM",
    },
    Unit {
        codigo: "SA",
        descripcion: "SACO",
        simbolo: "SCO",
    },
    Unit {
        codigo: "SEC",
        descripcion: "SEGUNDO",
        simbolo: "SEG",
    },
    Unit {
        codigo: "SET",
        descripcion: "JUEGO",
        simbolo: "JGO",
    },
    Unit {
        codigo: "ST",
        descripcion: "PLIEGO",
        simbolo: "PLGO",
    },
    Unit {
        codigo: "STN",
        descripcion: "TONELADA CORTA",
        simbolo: "TON",
    },
    Unit {
        codigo: "TNE",
        descripcion: "TONELADAS",
        simbolo: "T",
    },
    Unit {
        codigo: "TU",
        descripcion: "TUBOS",
        simbolo: "TB",
    },
    Unit {
        codigo: "U2",
        descripcion: "TABLETA O BLISTER",
        simbolo: "BLIS",
    },
    Unit {
        codigo: "UM",
        descripcion: "MILLÓN DE UNIDADES",
        simbolo: "MILL",
    },
    Unit {
        codigo: "YDK",
        descripcion: "YARDA CUADRADA",
        simbolo: "YD2",
    },
    Unit {
        codigo: "YRD",
        descripcion: "YARDA",
        simbolo: "YD",
    },
    Unit {
        codigo: "ZZ",
        descripcion: "UNIDAD (SERVICIOS)",
        simbolo: "SERV",
    },
];
