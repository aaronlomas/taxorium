import { writable } from 'svelte/store';
import { apiClient, type Product } from '$lib/services/apiClient';

function createProductsStore() {
	const { subscribe, set, update } = writable<Product[]>([]);

	return {
		subscribe,
		load: async () => {
			try {
				const data = await apiClient.getProducts();
				set(data);
			} catch (e) {
				console.error('Error loading products:', e);
			}
		}
	};
}

export const productsStore = createProductsStore();
export const editingProduct = writable<Product | null>(null);
