import { writable } from 'svelte/store';
import { apiClient, type Seller, type LoginSellerPayload } from '$lib/services/apiClient';

function createSellerAuth() {
	// Initialize from localStorage if running in browser
	const isBrowser = typeof window !== 'undefined';
	const storedSeller = isBrowser ? localStorage.getItem('activeSeller') : null;
	const initialSeller = storedSeller ? JSON.parse(storedSeller) : null;

	const { subscribe, set } = writable<Seller | null>(initialSeller);

	return {
		subscribe,
		login: async (payload: LoginSellerPayload) => {
			try {
				const seller = await apiClient.loginSeller(payload);
				set(seller);
				if (isBrowser) {
					localStorage.setItem('activeSeller', JSON.stringify(seller));
				}
				return seller;
			} catch (e) {
				console.error('Login failed:', e);
				throw e;
			}
		},
		logout: () => {
			set(null);
			if (isBrowser) {
				localStorage.removeItem('activeSeller');
			}
		}
	};
}

export const sellerAuth = createSellerAuth();
