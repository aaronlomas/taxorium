import { writable, derived } from 'svelte/store';
import { supabase } from '$lib/integrations/supabase/client';
import type { User, Session } from '@supabase/supabase-js';

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

		async init() {
			const {
				data: { session }
			} = await supabase.auth.getSession();

			set({
				user: session?.user ?? null,
				session,
				loading: false,
				initialized: true
			});

			supabase.auth.onAuthStateChange((_event, session) => {
				update((state) => ({
					...state,
					user: session?.user ?? null,
					session,
					loading: false
				}));
			});
		},

		async signIn(email: string, password: string) {
			update((s) => ({ ...s, loading: true }));
			const { data, error } = await supabase.auth.signInWithPassword({ email, password });
			update((s) => ({ ...s, loading: false }));
			if (error) throw error;
			return data;
		},

		async signUp(email: string, password: string) {
			update((s) => ({ ...s, loading: true }));
			const { data, error } = await supabase.auth.signUp({ email, password });
			update((s) => ({ ...s, loading: false }));
			if (error) throw error;
			return data;
		},

		async signOut() {
			await supabase.auth.signOut();
			set({ user: null, session: null, loading: false, initialized: true });
		}
	};
}

export const auth = createAuthStore();
export const isAuthenticated = derived(auth, ($auth) => !!$auth.user);
export const isLoading = derived(auth, ($auth) => $auth.loading);
export const currentUser = derived(auth, ($auth) => $auth.user);
