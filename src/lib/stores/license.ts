import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

const SUPABASE_URL = import.meta.env.VITE_SUPABASE_URL as string;
const LEASE_KEY = 'taxorium_lease';

interface LeasePayload {
	license_id: string;
	tenant_id: string;
	device_id: string;
	expires_at: string;
	issued_at: string;
}

interface LicenseState {
	valid: boolean;
	expiresAt: Date | null;
	tenantId: string | null;
	leaseToken: string | null;
	activating: boolean;
	error: string | null;
}

function createLicenseStore() {
	const { subscribe, set, update } = writable<LicenseState>({
		valid: false,
		expiresAt: null,
		tenantId: null,
		leaseToken: null,
		activating: false,
		error: null
	});

	function leerLeaseLocal(): { token: string; payload: LeasePayload } | null {
		try {
			const raw = localStorage.getItem(LEASE_KEY);
			if (!raw) return null;
			const payload: LeasePayload = JSON.parse(atob(raw));
			return { token: raw, payload };
		} catch {
			return null;
		}
	}

	function guardarLeaseLocal(token: string) {
		localStorage.setItem(LEASE_KEY, token);
	}

	function borrarLeaseLocal() {
		localStorage.removeItem(LEASE_KEY);
	}

	return {
		subscribe,

		/** Inicializar: lee el lease local y verifica si sigue vigente */
		init() {
			const lease = leerLeaseLocal();
			if (!lease) {
				set({ valid: false, expiresAt: null, tenantId: null, leaseToken: null, activating: false, error: null });
				return;
			}

			const expiresAt = new Date(lease.payload.expires_at);
			const valid = expiresAt > new Date();

			set({
				valid,
				expiresAt,
				tenantId: lease.payload.tenant_id,
				leaseToken: lease.token,
				activating: false,
				error: valid ? null : 'Licencia expirada. Conecta a internet para renovar.'
			});

			// Intentar renovar silenciosamente si hay internet
			if (valid) {
				this.renovarSilencioso(lease.token);
			}
		},

		/** Activar con una license key nueva */
		async activar(licenseKey: string): Promise<boolean> {
			update((s) => ({ ...s, activating: true, error: null }));

			let deviceId: string;
			try {
				deviceId = await invoke<string>('get_device_id');
			} catch {
				update((s) => ({ ...s, activating: false, error: 'No se pudo obtener el ID del dispositivo.' }));
				return false;
			}

			try {
				const res = await fetch(`${SUPABASE_URL}/functions/v1/activate-license`, {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({
						license_key: licenseKey.trim().toUpperCase(),
						device_id: deviceId,
						device_name: navigator.userAgent.slice(0, 100)
					})
				});

				const json = await res.json();

				if (!res.ok) {
					update((s) => ({ ...s, activating: false, error: json.error ?? 'Error al activar' }));
					return false;
				}

				guardarLeaseLocal(json.lease_token);

				const expiresAt = new Date(json.expires_at);
				set({
					valid: true,
					expiresAt,
					tenantId: json.tenant_id,
					leaseToken: json.lease_token,
					activating: false,
					error: null
				});

				return true;
			} catch {
				update((s) => ({ ...s, activating: false, error: 'Sin conexión. Verifica tu internet e intenta de nuevo.' }));
				return false;
			}
		},

		/** Renovar el lease silenciosamente en background */
		async renovarSilencioso(token: string) {
			let deviceId: string;
			try {
				deviceId = await invoke<string>('get_device_id');
			} catch {
				return;
			}

			try {
				const res = await fetch(`${SUPABASE_URL}/functions/v1/renew-lease`, {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({ lease_token: token, device_id: deviceId })
				});

				if (!res.ok) {
					const json = await res.json();
					// Si la licencia fue revocada, invalidar inmediatamente
					if (res.status === 403) {
						borrarLeaseLocal();
						update((s) => ({ ...s, valid: false, error: json.error }));
					}
					return;
				}

				const json = await res.json();
				guardarLeaseLocal(json.lease_token);

				const expiresAt = new Date(json.expires_at);
				update((s) => ({ ...s, expiresAt, leaseToken: json.lease_token }));
			} catch {
				// Sin internet — no hacer nada, el lease local sigue vigente
			}
		},

		/** Limpiar la licencia (cierre de sesión o desactivación) */
		limpiar() {
			borrarLeaseLocal();
			set({ valid: false, expiresAt: null, tenantId: null, leaseToken: null, activating: false, error: null });
		}
	};
}

export const licenseStore = createLicenseStore();
export const licenseValid = derived(licenseStore, ($l) => $l.valid);
export const licenseExpiry = derived(licenseStore, ($l) => $l.expiresAt);
