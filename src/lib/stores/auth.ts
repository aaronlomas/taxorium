import { writable, derived } from 'svelte/store';
import { supabase } from '$lib/supabase';
import type { User, Session } from '@supabase/supabase-js';

// ─── Estado ────────────────────────────────────────────────────────────────
interface AuthState {
	user: User | null;
	session: Session | null;
	loading: boolean;
	initialized: boolean;
}

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>({
		user: null,
		session: null,
		loading: true,
		initialized: false
	});

	return {
		subscribe,

		/** Inicializar: restaurar sesión existente y escuchar cambios */
		async init() {
			// Restaurar sesión guardada
			const {
				data: { session }
			} = await supabase.auth.getSession();

			set({
				user: session?.user ?? null,
				session,
				loading: false,
				initialized: true
			});

			// Escuchar cambios de sesión en tiempo real
			supabase.auth.onAuthStateChange((_event, session) => {
				update((state) => ({
					...state,
					user: session?.user ?? null,
					session,
					loading: false
				}));
			});
		},

		/** Login con email + password */
		async signIn(email: string, password: string) {
			update((s) => ({ ...s, loading: true }));
			const { data, error } = await supabase.auth.signInWithPassword({ email, password });
			update((s) => ({ ...s, loading: false }));
			if (error) throw error;
			return data;
		},

		/** Registro (solo para crear cuenta root inicial) */
		async signUp(email: string, password: string) {
			update((s) => ({ ...s, loading: true }));
			const { data, error } = await supabase.auth.signUp({ email, password });
			update((s) => ({ ...s, loading: false }));
			if (error) throw error;
			return data;
		},

		/** Cerrar sesión */
		async signOut() {
			await supabase.auth.signOut();
			set({ user: null, session: null, loading: false, initialized: true });
		}
	};
}

export const auth = createAuthStore();

// Derivados útiles
export const isAuthenticated = derived(auth, ($auth) => !!$auth.user && !$auth.loading);
export const isLoading = derived(auth, ($auth) => $auth.loading);
export const currentUser = derived(auth, ($auth) => $auth.user);
