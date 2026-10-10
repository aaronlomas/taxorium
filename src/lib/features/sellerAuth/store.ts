import { writable } from 'svelte/store';
import { sellerClient, type LoginSellerPayload, type Seller } from '$lib/features/sellers';

function createSellerAuth() {
	const isBrowser = typeof window !== 'undefined';
	const storedSeller = isBrowser ? localStorage.getItem('activeSeller') : null;
	const initialSeller = storedSeller ? JSON.parse(storedSeller) : null;

	const { subscribe, set } = writable<Seller | null>(initialSeller);

	return {
		subscribe,
		login: async (payload: LoginSellerPayload) => {
			try {
				const seller = await sellerClient.loginSeller(payload);
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
