-- ============================================================
-- TAXORIUM — Migración 002: Sistema de Licencias
-- ============================================================

-- Corregir política RLS de tenants (faltaba WITH CHECK para INSERT)
DROP POLICY IF EXISTS "tenants_owner_policy" ON public.tenants;
CREATE POLICY "tenants_owner_policy" ON public.tenants
  FOR ALL
  USING (auth.uid() = user_id)
  WITH CHECK (auth.uid() = user_id);

-- Política correcta para licenses (solo el dueño del tenant puede ver sus licencias)
DROP POLICY IF EXISTS "licenses_owner_policy" ON public.licenses;
CREATE POLICY "licenses_owner_policy" ON public.licenses
  FOR ALL
  USING (
    tenant_id IN (SELECT id FROM public.tenants WHERE user_id = auth.uid())
  )
  WITH CHECK (
    tenant_id IN (SELECT id FROM public.tenants WHERE user_id = auth.uid())
  );

-- ============================================================
-- TABLA: license_leases (tokens de lease locales rastreados)
-- Permite saber qué dispositivos tienen lease activo
-- ============================================================
CREATE TABLE IF NOT EXISTS public.license_leases (
  id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  license_id   UUID NOT NULL REFERENCES public.licenses(id) ON DELETE CASCADE,
  device_id    TEXT NOT NULL,
  issued_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  expires_at   TIMESTAMPTZ NOT NULL,
  revocado     BOOLEAN NOT NULL DEFAULT FALSE
);

ALTER TABLE public.license_leases ENABLE ROW LEVEL SECURITY;

-- Los leases los escribe solo la Edge Function (service_role), no el usuario directo
CREATE POLICY "leases_service_only" ON public.license_leases
  FOR ALL USING (FALSE);

-- ============================================================
-- FUNCIÓN: generate_license_key
-- Uso: SELECT generate_license_key('tenant-uuid-aqui', '2026-12-31');
-- La ejecutas tú desde Supabase Dashboard para crear keys para tus clientes
-- ============================================================
CREATE OR REPLACE FUNCTION public.generate_license_key(
  p_tenant_id UUID,
  p_expiry_at TIMESTAMPTZ DEFAULT NULL
)
RETURNS TEXT AS $$
DECLARE
  v_key TEXT;
  v_segment TEXT;
  i INTEGER;
BEGIN
  -- Generar key en formato TAXO-XXXX-XXXX-XXXX (letras mayúsculas + números)
  v_key := 'TAXO';
  FOR i IN 1..3 LOOP
    v_segment := upper(substr(encode(gen_random_bytes(3), 'hex'), 1, 4));
    v_key := v_key || '-' || v_segment;
  END LOOP;

  -- Insertar en la tabla licenses
  INSERT INTO public.licenses (tenant_id, license_key, expiry_at)
  VALUES (p_tenant_id, v_key, p_expiry_at);

  RETURN v_key;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- ============================================================
-- FUNCIÓN: revoke_license
-- Uso: SELECT revoke_license('TAXO-XXXX-XXXX-XXXX', 'No pagó');
-- ============================================================
CREATE OR REPLACE FUNCTION public.revoke_license(
  p_license_key TEXT,
  p_reason TEXT DEFAULT NULL
)
RETURNS BOOLEAN AS $$
DECLARE
  v_updated INTEGER;
BEGIN
  UPDATE public.licenses
  SET revocada = TRUE,
      revocada_reason = p_reason,
      activa = FALSE
  WHERE license_key = p_license_key;

  GET DIAGNOSTICS v_updated = ROW_COUNT;

  -- Revocar también todos los leases activos de ese dispositivo
  UPDATE public.license_leases
  SET revocado = TRUE
  WHERE license_id IN (
    SELECT id FROM public.licenses WHERE license_key = p_license_key
  );

  RETURN v_updated > 0;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
