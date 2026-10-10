import { writable } from 'svelte/store';
import { sellerClient } from './api';
import type { Seller } from './types';

function createSellersStore() {
	const { subscribe, set } = writable<Seller[]>([]);

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
export const sellerPasswordCache = writable<Record<number, string>>({});
