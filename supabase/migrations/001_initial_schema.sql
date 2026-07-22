-- ============================================================
-- TAXOR — Schema inicial de Supabase
-- Ejecutar en: Supabase Dashboard > SQL Editor
-- ============================================================

-- Extensiones necesarias
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================
-- TABLA: tenants (empresas registradas en Taxor)
-- ============================================================
CREATE TABLE public.tenants (
  id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id       UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
  ruc           VARCHAR(11) NOT NULL UNIQUE,
  razon_social  TEXT NOT NULL,
  nombre_comercial TEXT,
  direccion     TEXT NOT NULL,
  ubigeo        VARCHAR(6),
  departamento  TEXT,
  provincia     TEXT,
  distrito      TEXT,
  telefono      VARCHAR(20),
  email         TEXT,
  -- Series por defecto
  serie_boleta  VARCHAR(4) NOT NULL DEFAULT 'B001',
  serie_factura VARCHAR(4) NOT NULL DEFAULT 'F001',
  -- Control de correlativos
  correlativo_boleta   INTEGER NOT NULL DEFAULT 0,
  correlativo_factura  INTEGER NOT NULL DEFAULT 0,
  -- Estado
  configurado   BOOLEAN NOT NULL DEFAULT FALSE,
  activo        BOOLEAN NOT NULL DEFAULT TRUE,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- TABLA: licenses (control de activación por dispositivo)
-- ============================================================
CREATE TABLE public.licenses (
  id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  tenant_id     UUID NOT NULL REFERENCES public.tenants(id) ON DELETE CASCADE,
  license_key   VARCHAR(36) NOT NULL UNIQUE,
  device_id     TEXT,
  device_name   TEXT,
  activa        BOOLEAN NOT NULL DEFAULT FALSE,
  activada_at   TIMESTAMPTZ,
  expiry_at     TIMESTAMPTZ,
  last_heartbeat_at TIMESTAMPTZ,
  revocada      BOOLEAN NOT NULL DEFAULT FALSE,
  revocada_reason TEXT,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- TABLA: invoices (comprobantes emitidos)
-- ============================================================
CREATE TABLE public.invoices (
  id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  tenant_id       UUID NOT NULL REFERENCES public.tenants(id) ON DELETE CASCADE,
  tipo_comprobante VARCHAR(2) NOT NULL CHECK (tipo_comprobante IN ('01','03')),
  serie           VARCHAR(4) NOT NULL,
  correlativo     INTEGER NOT NULL,
  numero_completo TEXT GENERATED ALWAYS AS (serie || '-' || LPAD(correlativo::TEXT, 8, '0')) STORED,
  -- Cliente
  tipo_doc_cliente VARCHAR(2),
  num_doc_cliente  TEXT,
  nombre_cliente   TEXT NOT NULL,
  direccion_cliente TEXT,
  -- Montos
  subtotal        NUMERIC(12,2) NOT NULL,
  igv             NUMERIC(12,2) NOT NULL DEFAULT 0,
  total           NUMERIC(12,2) NOT NULL,
  moneda          VARCHAR(3) NOT NULL DEFAULT 'PEN',
  -- Items (JSON)
  items           JSONB NOT NULL DEFAULT '[]',
  -- Estado SUNAT
  estado          TEXT NOT NULL DEFAULT 'BORRADOR'
                  CHECK (estado IN ('BORRADOR','ENVIANDO','ACEPTADO','RECHAZADO','ANULADO')),
  codigo_sunat    TEXT,
  mensaje_sunat   TEXT,
  -- Archivos
  xml_content     TEXT,
  cdr_content     TEXT,
  pdf_url         TEXT,
  -- Metadata
  fecha_emision   DATE NOT NULL DEFAULT CURRENT_DATE,
  hora_emision    TIME NOT NULL DEFAULT CURRENT_TIME,
  device_id       TEXT,
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (tenant_id, tipo_comprobante, serie, correlativo)
);

-- ============================================================
-- TABLA: customers (clientes del tenant)
-- ============================================================
CREATE TABLE public.customers (
  id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  tenant_id     UUID NOT NULL REFERENCES public.tenants(id) ON DELETE CASCADE,
  tipo_doc      VARCHAR(2) NOT NULL DEFAULT '1',
  num_doc       TEXT NOT NULL,
  nombre        TEXT NOT NULL,
  direccion     TEXT,
  email         TEXT,
  telefono      TEXT,
  activo        BOOLEAN NOT NULL DEFAULT TRUE,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (tenant_id, tipo_doc, num_doc)
);

-- ============================================================
-- TABLA: products (catálogo de productos/servicios)
-- ============================================================
CREATE TABLE public.products (
  id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  tenant_id       UUID NOT NULL REFERENCES public.tenants(id) ON DELETE CASCADE,
  codigo          TEXT,
  nombre          TEXT NOT NULL,
  descripcion     TEXT,
  unidad          VARCHAR(10) NOT NULL DEFAULT 'NIU',
  precio          NUMERIC(12,4) NOT NULL,
  afecto_igv      BOOLEAN NOT NULL DEFAULT TRUE,
  activo          BOOLEAN NOT NULL DEFAULT TRUE,
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (tenant_id, codigo)
);

-- ============================================================
-- ÍNDICES
-- ============================================================
CREATE INDEX idx_tenants_user_id ON public.tenants(user_id);
CREATE INDEX idx_tenants_ruc ON public.tenants(ruc);
CREATE INDEX idx_licenses_tenant_id ON public.licenses(tenant_id);
CREATE INDEX idx_licenses_device_id ON public.licenses(device_id);
CREATE INDEX idx_invoices_tenant_id ON public.invoices(tenant_id);
CREATE INDEX idx_invoices_estado ON public.invoices(estado);
CREATE INDEX idx_invoices_fecha ON public.invoices(fecha_emision);
CREATE INDEX idx_customers_tenant_id ON public.customers(tenant_id);
CREATE INDEX idx_products_tenant_id ON public.products(tenant_id);

-- ============================================================
-- TRIGGERS: updated_at automático
-- ============================================================
CREATE OR REPLACE FUNCTION public.set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_tenants_updated_at
  BEFORE UPDATE ON public.tenants
  FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();

CREATE TRIGGER trg_invoices_updated_at
  BEFORE UPDATE ON public.invoices
  FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();

-- ============================================================
-- ROW LEVEL SECURITY (RLS)
-- ============================================================
ALTER TABLE public.tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.licenses ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.products ENABLE ROW LEVEL SECURITY;

CREATE POLICY "tenants_owner_policy" ON public.tenants
  FOR ALL USING (auth.uid() = user_id);

CREATE POLICY "licenses_owner_policy" ON public.licenses
  FOR ALL USING (
    tenant_id IN (SELECT id FROM public.tenants WHERE user_id = auth.uid())
  );

CREATE POLICY "invoices_owner_policy" ON public.invoices
  FOR ALL USING (
    tenant_id IN (SELECT id FROM public.tenants WHERE user_id = auth.uid())
  );

CREATE POLICY "customers_owner_policy" ON public.customers
  FOR ALL USING (
    tenant_id IN (SELECT id FROM public.tenants WHERE user_id = auth.uid())
  );

CREATE POLICY "products_owner_policy" ON public.products
  FOR ALL USING (
    tenant_id IN (SELECT id FROM public.tenants WHERE user_id = auth.uid())
  );

-- ============================================================
-- FUNCIÓN: obtener siguiente correlativo (atómico)
-- ============================================================
CREATE OR REPLACE FUNCTION public.get_next_correlativo(
  p_tenant_id UUID,
  p_tipo VARCHAR(2)
)
RETURNS INTEGER AS $$
DECLARE
  v_correlativo INTEGER;
BEGIN
  IF p_tipo = '03' THEN
    UPDATE public.tenants
    SET correlativo_boleta = correlativo_boleta + 1
    WHERE id = p_tenant_id AND user_id = auth.uid()
    RETURNING correlativo_boleta INTO v_correlativo;
  ELSE
    UPDATE public.tenants
    SET correlativo_factura = correlativo_factura + 1
    WHERE id = p_tenant_id AND user_id = auth.uid()
    RETURNING correlativo_factura INTO v_correlativo;
  END IF;

  IF v_correlativo IS NULL THEN
    RAISE EXCEPTION 'Tenant no encontrado o sin permisos';
  END IF;

  RETURN v_correlativo;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
