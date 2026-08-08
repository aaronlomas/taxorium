import { writable } from 'svelte/store';
import { sellerClient, type Seller } from '$lib/services/sellers/clientSellers';

function createSellersStore() {
	const { subscribe, set, update } = writable<Seller[]>([]);

	return {
		subscribe,
		load: async () => {
			try {
				const data = await sellerClient.getSellers();
				set(data);
			} catch (e) {
				console.error('Error loading sellers:', e);
			}
		}
	};
}

export const sellersStore = createSellersStore();
export const editingSeller = writable<Seller | null>(null);

/**
 * In-memory cache of plain-text passwords for sellers created/updated
 * in the current session. Passwords are never stored on disk.
 */
export const sellerPasswordCache = writable<Record<number, string>>({});
