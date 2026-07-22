import { writable, derived } from 'svelte/store';
import { supabase } from '$lib/supabase';
import type { Database } from '$lib/database.types';

type Tenant = Database['public']['Tables']['tenants']['Row'];

interface TenantState {
	tenant: Tenant | null;
	loading: boolean;
	error: string | null;
}

function createTenantStore() {
	const { subscribe, set, update } = writable<TenantState>({
		tenant: null,
		loading: false,
		error: null
	});

	return {
		subscribe,

		/** Cargar el tenant del usuario autenticado */
		async load() {
			update((s) => ({ ...s, loading: true, error: null }));
			const { data, error } = await (supabase.from('tenants') as any)
				.select('*')
				.eq('activo', true)
				.maybeSingle();

			if (error) {
				update((s) => ({ ...s, loading: false, error: error.message }));
				return null;
			}
			update((s) => ({ ...s, tenant: data, loading: false }));
			return data;
		},

		/** Crear tenant nuevo (onboarding inicial) */
		async create(userId: string, data: Partial<Tenant>) {
			update((s) => ({ ...s, loading: true, error: null }));
			const { data: created, error } = await (supabase.from('tenants') as any)
				.insert({ ...data, user_id: userId })
				.select()
				.single();

			if (error) {
				update((s) => ({ ...s, loading: false, error: error.message }));
				throw error;
			}
			update((s) => ({ ...s, tenant: created, loading: false }));
			return created;
		},

		/** Marcar tenant como completamente configurado */
		async markConfigured(tenantId: string) {
			const { error } = await (supabase.from('tenants') as any)
				.update({ configurado: true })
				.eq('id', tenantId);
			if (error) throw error;
			update((s) => ({
				...s,
				tenant: s.tenant ? { ...s.tenant, configurado: true } : null
			}));
		},

		clear() {
			set({ tenant: null, loading: false, error: null });
		}
	};
}

export const tenantStore = createTenantStore();

// Derivados
export const currentTenant = derived(tenantStore, ($t) => $t.tenant);
export const isConfigured = derived(tenantStore, ($t) => $t.tenant?.configurado ?? false);
export const tenantLoading = derived(tenantStore, ($t) => $t.loading);
