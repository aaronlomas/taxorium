import { writable } from 'svelte/store';
import { vendorClient } from './api';
import type { Vendor } from './types';

function createVendorsStore() {
	const { subscribe, set } = writable<Vendor[]>([]);

	return {
		subscribe,
		load: async () => {
			try {
				const data = await vendorClient.getVendors();
				set(data);
			} catch (e) {
				console.error('Error loading vendors:', e);
			}
		}
	};
}

export const vendorsStore = createVendorsStore();
export const editingVendor = writable<Vendor | null>(null);
