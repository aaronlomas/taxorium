CREATE TABLE IF NOT EXISTS monedas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE,       -- PEN, USD
    descripcion TEXT NOT NULL,         -- Soles, Dólares
    simbolo TEXT NOT NULL,             -- S/, $
    activo BOOLEAN NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS sedes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE,
    label TEXT NOT NULL,
    activo BOOLEAN NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS series (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE,
    tipo_documento TEXT NOT NULL,
    numero_actual INTEGER NOT NULL DEFAULT 1,
    activo BOOLEAN NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS unidades (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE,
    descripcion TEXT NOT NULL,
    simbolo TEXT
);
-- TABLAS DE CATÁLOGOS TRIBUTARIOS
CREATE TABLE IF NOT EXISTS afectaciones_venta (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE,       -- '10', '20', etc.
    descripcion TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS afectaciones_compra (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE,       -- 'GRAVADO_V_GRAVADO', etc.
    descripcion TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS productos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo_interno TEXT,
    codigo_unidad TEXT NOT NULL DEFAULT 'NIU',
    nombre TEXT NOT NULL,
    codigo_sunat TEXT,
    codigo_gsl TEXT,
    moneda TEXT NOT NULL DEFAULT 'PEN',
    precio_unitario_venta REAL NOT NULL DEFAULT 0,
    precio_unitario_compra REAL NOT NULL DEFAULT 0,
    stock_minimo REAL NOT NULL DEFAULT 1,
    afectacion_venta TEXT NOT NULL DEFAULT '20',
    afectacion_compra TEXT NOT NULL DEFAULT 'NO_GRAVADO',
    tiene_icbper BOOLEAN NOT NULL DEFAULT 0,
    marca TEXT,
    categoria TEXT,
    codigo_sede TEXT DEFAULT '0000',
    activo BOOLEAN NOT NULL DEFAULT 1,
    FOREIGN KEY(codigo_unidad) REFERENCES unidades(codigo),
    FOREIGN KEY(codigo_sede) REFERENCES sedes(codigo),
    FOREIGN KEY(moneda) REFERENCES monedas(codigo),
    FOREIGN KEY(afectacion_venta) REFERENCES afectaciones_venta(codigo),
    FOREIGN KEY(afectacion_compra) REFERENCES afectaciones_compra(codigo)
);

CREATE TABLE IF NOT EXISTS clientes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo_documento TEXT NOT NULL,
    numero_documento TEXT NOT NULL UNIQUE,
    nombre TEXT NOT NULL,
    nombre_comercial TEXT,
    pais TEXT NOT NULL DEFAULT 'PERU',
    departamento TEXT,
    provincia TEXT,
    distrito TEXT,
    direccion TEXT,
    telefono TEXT,
    correo TEXT,
    activo BOOLEAN NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS vendedores (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombres TEXT,
    apellidos TEXT,
    usuario TEXT NOT NULL UNIQUE,
    clave_cifrada TEXT NOT NULL,
    accesos TEXT,
    dominio TEXT,
    activo BOOLEAN NOT NULL DEFAULT 1
);

-- ÍNDICES DE RENDIMIENTO
CREATE INDEX IF NOT EXISTS idx_productos_codigo_interno ON productos(codigo_interno);
CREATE INDEX IF NOT EXISTS idx_productos_sede ON productos(codigo_sede);
CREATE INDEX IF NOT EXISTS idx_clientes_num_doc ON clientes(numero_documento);

CREATE TABLE IF NOT EXISTS tipos_operacion (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE,
    descripcion TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tipos_pago (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE,
    descripcion TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tipos_comprobante (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE,
    descripcion TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS comprobantes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fecha_de_emision TEXT NOT NULL,
    cliente TEXT NOT NULL,
    numero_comprobante TEXT NOT NULL UNIQUE,
    estado_validez TEXT NOT NULL DEFAULT 'registrado' CHECK (estado_validez IN ('registrado', 'rechazado', 'aceptado')),
    estado_pago TEXT NOT NULL DEFAULT 'pendiente' CHECK (estado_pago IN ('pendiente', 'pagado')),
    moneda TEXT NOT NULL DEFAULT 'PEN',
    gravado REAL NOT NULL DEFAULT 0,
    igv REAL NOT NULL DEFAULT 0,
    total REAL NOT NULL DEFAULT 0,
    estado BOOLEAN NOT NULL DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_comprobantes_numero ON comprobantes(numero_comprobante);
CREATE INDEX IF NOT EXISTS idx_comprobantes_estado_validez ON comprobantes(estado_validez);

CREATE TABLE IF NOT EXISTS comprobantes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fecha_de_emision TEXT NOT NULL,
    cliente TEXT NOT NULL,
    numero_comprobante TEXT NOT NULL UNIQUE,
    estado_validez TEXT NOT NULL DEFAULT 'registrado' CHECK (estado_validez IN ('registrado', 'rechazado', 'aceptado')),
    estado_pago TEXT NOT NULL DEFAULT 'pendiente' CHECK (estado_pago IN ('pendiente', 'pagado')),
    moneda TEXT NOT NULL DEFAULT 'PEN',
    gravado REAL NOT NULL DEFAULT 0,
    igv REAL NOT NULL DEFAULT 0,
    total REAL NOT NULL DEFAULT 0,
    estado BOOLEAN NOT NULL DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_comprobantes_numero ON comprobantes(numero_comprobante);
CREATE INDEX IF NOT EXISTS idx_comprobantes_estado_validez ON comprobantes(estado_validez);