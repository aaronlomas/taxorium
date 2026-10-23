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
    -- Unidad en la que se compra el producto (FK a unidades). Puede diferir de
    -- la unidad base/venta; por defecto igual a la unidad base.
    codigo_unidad_compra TEXT NOT NULL DEFAULT 'NIU',
    -- Cuántas unidades base (codigo_unidad) contiene 1 unidad de compra.
    -- Debe ser > 0 y valer 1 cuando la unidad de compra es la unidad base.
    factor_a_unidad_base REAL NOT NULL DEFAULT 1.0 CHECK (factor_a_unidad_base > 0),
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
    FOREIGN KEY(codigo_unidad_compra) REFERENCES unidades(codigo),
    FOREIGN KEY(codigo_sede) REFERENCES sedes(codigo),
    FOREIGN KEY(moneda) REFERENCES monedas(codigo),
    FOREIGN KEY(afectacion_venta) REFERENCES afectaciones_venta(codigo),
    FOREIGN KEY(afectacion_compra) REFERENCES afectaciones_compra(codigo),
    -- Si la unidad de compra es la unidad base, el factor debe ser exactamente 1.
    CHECK (codigo_unidad_compra <> codigo_unidad OR factor_a_unidad_base = 1.0)
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
    estado_validez TEXT NOT NULL DEFAULT 'registrado' CHECK (estado_validez IN ('registrado', 'rechazado', 'aceptado', 'anulado')),
    estado_pago TEXT NOT NULL DEFAULT 'pendiente' CHECK (estado_pago IN ('pendiente', 'pagado')),
    xml_firmado TEXT,
    moneda TEXT NOT NULL DEFAULT 'PEN',
    tipo_cambio REAL,
    gravado REAL NOT NULL DEFAULT 0,
    igv REAL NOT NULL DEFAULT 0,
    total REAL NOT NULL DEFAULT 0,
    forma_pago TEXT NOT NULL DEFAULT 'Contado' REFERENCES tipos_pago(codigo),
    monto_neto_pendiente REAL,
    fecha_vencimiento TEXT,
    estado BOOLEAN NOT NULL DEFAULT 1,
    hash_cpe TEXT,
    estado_sunat INTEGER NOT NULL DEFAULT 0,
    codigo_cdr TEXT,
    descripcion_cdr TEXT,
    xml_cdr TEXT
);

CREATE INDEX IF NOT EXISTS idx_comprobantes_numero ON comprobantes(numero_comprobante);
CREATE INDEX IF NOT EXISTS idx_comprobantes_estado_validez ON comprobantes(estado_validez);

-- TABLA DE ANULACIONES / COMUNICACIONES DE BAJA
CREATE TABLE IF NOT EXISTS anulaciones (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    comprobante_id INTEGER NOT NULL REFERENCES comprobantes(id),
    tipo TEXT NOT NULL CHECK (tipo IN ('baja', 'nota_credito', 'local')),
    motivo TEXT NOT NULL,
    id_baja TEXT,
    ticket TEXT,
    codigo_cdr TEXT,
    descripcion_cdr TEXT,
    xml_cdr TEXT,
    xml_baja_firmado TEXT,
    fecha_anulacion TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_anulaciones_comprobante ON anulaciones(comprobante_id);
CREATE INDEX IF NOT EXISTS idx_anulaciones_ticket ON anulaciones(ticket);

-- ============================================================================
-- INVENTARIO (CORE)
-- Motor genérico de existencias. TODO movimiento se registra en la unidad base
-- del producto (productos.codigo_unidad). La conversión desde la unidad de
-- compra se aplica UNA sola vez al registrar el movimiento:
--     cantidad_base = cantidad_compra * productos.factor_a_unidad_base
-- Los addons (mesas, despiece, etc.) consumen este motor y NUNCA llevan su
-- propio inventario.
-- ============================================================================
CREATE TABLE IF NOT EXISTS movimientos_inventario (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    producto_id INTEGER NOT NULL,
    codigo_sede TEXT NOT NULL DEFAULT '0000',
    -- Clasificación: 'entrada' (compra/reposición), 'salida' (venta/consumo),
    -- 'ajuste' (merma, corrección, inventario inicial).
    tipo TEXT NOT NULL CHECK (tipo IN ('entrada', 'salida', 'ajuste')),
    -- Delta firmado sobre el stock, siempre en unidad base.
    -- entrada > 0, salida < 0, ajuste puede ser positivo o negativo.
    cantidad_base REAL NOT NULL,
    -- Costo por unidad base al momento del movimiento (kardex valorizado).
    costo_unitario_base REAL NOT NULL DEFAULT 0,
    -- Origen del movimiento para trazabilidad (ej. 'compra', 'venta', 'ajuste').
    referencia_tipo TEXT,
    referencia_id INTEGER,
    descripcion TEXT,
    fecha TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY(producto_id) REFERENCES productos(id),
    FOREIGN KEY(codigo_sede) REFERENCES sedes(codigo),
    CHECK (
        (tipo = 'entrada' AND cantidad_base > 0) OR
        (tipo = 'salida'  AND cantidad_base < 0) OR
        (tipo = 'ajuste')
    )
);

CREATE INDEX IF NOT EXISTS idx_mov_inv_producto_sede ON movimientos_inventario(producto_id, codigo_sede);
CREATE INDEX IF NOT EXISTS idx_mov_inv_referencia ON movimientos_inventario(referencia_tipo, referencia_id);
CREATE INDEX IF NOT EXISTS idx_mov_inv_fecha ON movimientos_inventario(fecha);

-- Stock derivado. Se puede materializar más adelante si el volumen lo exige
-- sin cambiar el modelo de datos.
CREATE VIEW IF NOT EXISTS stock_actual AS
SELECT producto_id, codigo_sede, SUM(cantidad_base) AS cantidad_base
FROM movimientos_inventario
GROUP BY producto_id, codigo_sede;

-- Configuración local del dispositivo.
-- Pares clave-valor para datos específicos de esta instalación que NO deben
-- vivir en la nube: ruta al certificado .p12, contraseña, etc.
-- Permite que la app funcione sin conexión a internet una vez configurada.
CREATE TABLE IF NOT EXISTS config_local (
    clave   TEXT PRIMARY KEY,
    valor   TEXT
);