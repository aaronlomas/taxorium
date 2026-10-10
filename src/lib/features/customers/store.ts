import { writable } from 'svelte/store';
import { customerClient } from './api';
import type { Customer } from './types';

function createCustomersStore() {
	const { subscribe, set } = writable<Customer[]>([]);

	return {
		subscribe,
		load: async () => {
			try {
				const data = await customerClient.getCustomers();
				set(data);
			} catch (e) {
				console.error('Error loading customers:', e);
			}
		}
	};
}

export const customersStore = createCustomersStore();
export const editingCustomer = writable<Customer | null>(null);
