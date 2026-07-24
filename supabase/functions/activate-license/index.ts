import { createClient } from 'jsr:@supabase/supabase-js@2';

const LEASE_DURATION_DAYS = 7;

const CORS = {
  'Access-Control-Allow-Origin': '*',
  'Access-Control-Allow-Methods': 'POST, OPTIONS',
  'Access-Control-Allow-Headers': 'Content-Type, Authorization',
};

function json(data: unknown, status = 200) {
  return new Response(JSON.stringify(data), {
    status,
    headers: { 'Content-Type': 'application/json', ...CORS },
  });
}

Deno.serve(async (req: Request) => {
  if (req.method === 'OPTIONS') {
    return new Response(null, { status: 204, headers: CORS });
  }

  if (req.method !== 'POST') {
    return json({ error: 'Método no permitido' }, 405);
  }

  let body: { license_key?: string; device_id?: string; device_name?: string };
  try {
    body = await req.json();
  } catch {
    return json({ error: 'JSON inválido' }, 400);
  }

  const { license_key, device_id, device_name } = body;

  if (!license_key || !device_id) {
    return json({ error: 'license_key y device_id son requeridos' }, 400);
  }

  const supabase = createClient(
    Deno.env.get('SUPABASE_URL')!,
    Deno.env.get('SUPABASE_SERVICE_ROLE_KEY')!
  );

  // Buscar la licencia
  const { data: license, error: licenseError } = await supabase
    .from('licenses')
    .select('id, tenant_id, activa, revocada, expiry_at, device_id')
    .eq('license_key', license_key)
    .maybeSingle();

  if (licenseError || !license) {
    return json({ error: 'Licencia no encontrada' }, 404);
  }

  if (license.revocada) {
    return json({ error: 'Esta licencia ha sido revocada. Contacta al soporte.' }, 403);
  }

  if (license.expiry_at && new Date(license.expiry_at) < new Date()) {
    return json({ error: 'Esta licencia ha vencido. Renueva tu suscripción.' }, 403);
  }

  if (license.activa && license.device_id && license.device_id !== device_id) {
    return json({ error: 'Esta licencia ya está activada en otro dispositivo.' }, 409);
  }

  // Marcar licencia como activa y registrar dispositivo
  const { error: updateError } = await supabase
    .from('licenses')
    .update({
      activa: true,
      device_id,
      device_name: device_name ?? null,
      activada_at: new Date().toISOString(),
      last_heartbeat_at: new Date().toISOString(),
    })
    .eq('id', license.id);

  if (updateError) {
    return json({ error: 'Error al activar la licencia' }, 500);
  }

  // Crear registro de lease
  const expiresAt = new Date();
  expiresAt.setDate(expiresAt.getDate() + LEASE_DURATION_DAYS);

  const { error: leaseError } = await supabase
    .from('license_leases')
    .insert({ license_id: license.id, device_id, expires_at: expiresAt.toISOString() });

  if (leaseError) {
    console.error('Error al crear lease:', leaseError.message);
  }

  const leaseToken = btoa(JSON.stringify({
    license_id: license.id,
    tenant_id: license.tenant_id,
    device_id,
    expires_at: expiresAt.toISOString(),
    issued_at: new Date().toISOString(),
  }));

  return json({
    success: true,
    tenant_id: license.tenant_id,
    lease_token: leaseToken,
    expires_at: expiresAt.toISOString(),
    message: `Licencia activada. Válida por ${LEASE_DURATION_DAYS} días sin internet.`,
  });
});
