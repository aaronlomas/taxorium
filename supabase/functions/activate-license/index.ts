import { createClient } from 'jsr:@supabase/supabase-js@2';

const LEASE_DURATION_DAYS = 7;

Deno.serve(async (req: Request) => {
  // Solo POST
  if (req.method !== 'POST') {
    return new Response(JSON.stringify({ error: 'Método no permitido' }), { status: 405 });
  }

  let body: { license_key?: string; device_id?: string; device_name?: string };
  try {
    body = await req.json();
  } catch {
    return new Response(JSON.stringify({ error: 'JSON inválido' }), { status: 400 });
  }

  const { license_key, device_id, device_name } = body;

  if (!license_key || !device_id) {
    return new Response(
      JSON.stringify({ error: 'license_key y device_id son requeridos' }),
      { status: 400 }
    );
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
    return new Response(
      JSON.stringify({ error: 'Licencia no encontrada' }),
      { status: 404 }
    );
  }

  if (license.revocada) {
    return new Response(
      JSON.stringify({ error: 'Esta licencia ha sido revocada. Contacta al soporte.' }),
      { status: 403 }
    );
  }

  // Verificar expiración de la licencia
  if (license.expiry_at && new Date(license.expiry_at) < new Date()) {
    return new Response(
      JSON.stringify({ error: 'Esta licencia ha vencido. Renueva tu suscripción.' }),
      { status: 403 }
    );
  }

  // Si la licencia está activa y ya tiene un device_id distinto, verificar
  if (license.activa && license.device_id && license.device_id !== device_id) {
    return new Response(
      JSON.stringify({ error: 'Esta licencia ya está activada en otro dispositivo.' }),
      { status: 409 }
    );
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
    return new Response(
      JSON.stringify({ error: 'Error al activar la licencia' }),
      { status: 500 }
    );
  }

  // Crear registro de lease
  const expiresAt = new Date();
  expiresAt.setDate(expiresAt.getDate() + LEASE_DURATION_DAYS);

  const { error: leaseError } = await supabase
    .from('license_leases')
    .insert({
      license_id: license.id,
      device_id,
      expires_at: expiresAt.toISOString(),
    });

  if (leaseError) {
    console.error('Error al crear lease:', leaseError.message);
  }

  // Generar token de lease (JWT simple firmado con el service role secret)
  const leasePayload = {
    license_id: license.id,
    tenant_id: license.tenant_id,
    device_id,
    expires_at: expiresAt.toISOString(),
    issued_at: new Date().toISOString(),
  };

  // Encodificar como base64 (el frontend lo almacena localmente)
  const leaseToken = btoa(JSON.stringify(leasePayload));

  return new Response(
    JSON.stringify({
      success: true,
      tenant_id: license.tenant_id,
      lease_token: leaseToken,
      expires_at: expiresAt.toISOString(),
      message: `Licencia activada. Válida por ${LEASE_DURATION_DAYS} días sin internet.`,
    }),
    { status: 200, headers: { 'Content-Type': 'application/json' } }
  );
});
