/**
 * Cliente para la API de configuración local (SQLite).
 *
 * A diferencia del resto de clientes, este siempre habla con el servidor Axum
 * local en localhost:3000, independientemente del rol del nodo.
 * Los datos de configuración (certificado_path, clave_cert) son siempre locales.
 */

const BASE = 'http://localhost:3000';

export interface LocalConfig {
	certificado_path?: string;
	clave_cert?: string;
	[key: string]: string | undefined;
}

async function request<T>(path: string, init?: RequestInit, body?: unknown): Promise<T> {
	const res = await fetch(`${BASE}/${path}`, {
		...init,
		headers: { 'Content-Type': 'application/json', ...(init?.headers ?? {}) },
		body: body !== undefined ? JSON.stringify(body) : undefined
	});
	if (!res.ok) {
		const text = await res.text().catch(() => `HTTP ${res.status}`);
		throw new Error(`config_local: ${text}`);
	}
	if (res.status === 200 && res.headers.get('content-type')?.includes('json')) {
		return res.json();
	}
	return undefined as T;
}

export const configLocalClient = {
	/** Obtiene toda la configuración local como un objeto clave-valor. */
	getAll(): Promise<LocalConfig> {
		return request<LocalConfig>('api/config_local');
	},

	/** Guarda o actualiza un valor en la configuración local. */
	set(clave: string, valor: string): Promise<void> {
		return request<void>(`api/config_local/${encodeURIComponent(clave)}`, { method: 'PUT' }, { valor });
	}
};
