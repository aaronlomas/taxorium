import { createClient } from 'jsr:@supabase/supabase-js@2';

const LEASE_DURATION_DAYS = 7;

Deno.serve(async (req: Request) => {
  if (req.method !== 'POST') {
    return new Response(JSON.stringify({ error: 'Método no permitido' }), { status: 405 });
  }

  let body: { lease_token?: string; device_id?: string };
  try {
    body = await req.json();
  } catch {
    return new Response(JSON.stringify({ error: 'JSON inválido' }), { status: 400 });
  }

  const { lease_token, device_id } = body;

  if (!lease_token || !device_id) {
    return new Response(
      JSON.stringify({ error: 'lease_token y device_id son requeridos' }),
      { status: 400 }
    );
  }

  // Decodificar el lease token
  let leasePayload: {
    license_id: string;
    tenant_id: string;
    device_id: string;
    expires_at: string;
  };

  try {
    leasePayload = JSON.parse(atob(lease_token));
  } catch {
    return new Response(JSON.stringify({ error: 'Token inválido' }), { status: 400 });
  }

  // Verificar que el device_id coincide
  if (leasePayload.device_id !== device_id) {
    return new Response(
      JSON.stringify({ error: 'Token no corresponde a este dispositivo' }),
      { status: 403 }
    );
  }

  const supabase = createClient(
    Deno.env.get('SUPABASE_URL')!,
    Deno.env.get('SUPABASE_SERVICE_ROLE_KEY')!
  );

  // Verificar que la licencia sigue activa y no revocada
  const { data: license, error: licenseError } = await supabase
    .from('licenses')
    .select('id, activa, revocada, expiry_at')
    .eq('id', leasePayload.license_id)
    .eq('device_id', device_id)
    .maybeSingle();

  if (licenseError || !license) {
    return new Response(
      JSON.stringify({ error: 'Licencia no encontrada para este dispositivo' }),
      { status: 404 }
    );
  }

  if (license.revocada || !license.activa) {
    return new Response(
      JSON.stringify({ error: 'Licencia revocada o desactivada. Contacta al soporte.' }),
      { status: 403 }
    );
  }

  if (license.expiry_at && new Date(license.expiry_at) < new Date()) {
    return new Response(
      JSON.stringify({ error: 'Licencia vencida. Renueva tu suscripción.' }),
      { status: 403 }
    );
  }

  // Actualizar heartbeat
  await supabase
    .from('licenses')
    .update({ last_heartbeat_at: new Date().toISOString() })
    .eq('id', license.id);

  // Emitir nuevo lease token con +7 días
  const newExpiresAt = new Date();
  newExpiresAt.setDate(newExpiresAt.getDate() + LEASE_DURATION_DAYS);

  const newLeasePayload = {
    license_id: leasePayload.license_id,
    tenant_id: leasePayload.tenant_id,
    device_id,
    expires_at: newExpiresAt.toISOString(),
    issued_at: new Date().toISOString(),
  };

  const newLeaseToken = btoa(JSON.stringify(newLeasePayload));

  return new Response(
    JSON.stringify({
      success: true,
      lease_token: newLeaseToken,
      expires_at: newExpiresAt.toISOString(),
    }),
    { status: 200, headers: { 'Content-Type': 'application/json' } }
  );
});
