import { writable } from 'svelte/store';
import { productClient } from './api';
import type { Product } from './types';

function createProductsStore() {
	const { subscribe, set } = writable<Product[]>([]);

	return {
		subscribe,
		load: async () => {
			try {
				const data = await productClient.getProducts();
				set(data);
			} catch (e) {
				console.error('Error loading products:', e);
			}
		}
	};
}

export const productsStore = createProductsStore();
export const editingProduct = writable<Product | null>(null);
