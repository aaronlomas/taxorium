import { createClient } from 'jsr:@supabase/supabase-js@2';

const LEASE_DURATION_DAYS = 7;

const CORS = {
	'Access-Control-Allow-Origin': '*',
	'Access-Control-Allow-Methods': 'POST, OPTIONS',
	'Access-Control-Allow-Headers': 'Content-Type, Authorization'
};

function json(data: unknown, status = 200) {
	return new Response(JSON.stringify(data), {
		status,
		headers: { 'Content-Type': 'application/json', ...CORS }
	});
}

Deno.serve(async (req: Request) => {
	if (req.method === 'OPTIONS') {
		return new Response(null, { status: 204, headers: CORS });
	}

	if (req.method !== 'POST') {
		return json({ error: 'Método no permitido' }, 405);
	}

	let body: { lease_token?: string; device_id?: string };
	try {
		body = await req.json();
	} catch {
		return json({ error: 'JSON inválido' }, 400);
	}

	const { lease_token, device_id } = body;

	if (!lease_token || !device_id) {
		return json({ error: 'lease_token y device_id son requeridos' }, 400);
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
		return json({ error: 'Token inválido' }, 400);
	}

	if (leasePayload.device_id !== device_id) {
		return json({ error: 'Token no corresponde a este dispositivo' }, 403);
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
		return json({ error: 'Licencia no encontrada para este dispositivo' }, 404);
	}

	if (license.revocada || !license.activa) {
		return json({ error: 'Licencia revocada o desactivada. Contacta al soporte.' }, 403);
	}

	if (license.expiry_at && new Date(license.expiry_at) < new Date()) {
		return json({ error: 'Licencia vencida. Renueva tu suscripción.' }, 403);
	}

	// Actualizar heartbeat
	await supabase
		.from('licenses')
		.update({ last_heartbeat_at: new Date().toISOString() })
		.eq('id', license.id);

	// Emitir nuevo lease token con +7 días
	const newExpiresAt = new Date();
	newExpiresAt.setDate(newExpiresAt.getDate() + LEASE_DURATION_DAYS);

	const newLeaseToken = btoa(
		JSON.stringify({
			license_id: leasePayload.license_id,
			tenant_id: leasePayload.tenant_id,
			device_id,
			expires_at: newExpiresAt.toISOString(),
			issued_at: new Date().toISOString()
		})
	);

	return json({
		success: true,
		lease_token: newLeaseToken,
		expires_at: newExpiresAt.toISOString()
	});
});
