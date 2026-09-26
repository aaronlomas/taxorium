import { writable } from 'svelte/store';
import { customerClient, type Customer } from '$lib/services/customers/clientCustomer';

function createCustomersStore() {
	const { subscribe, set, update } = writable<Customer[]>([]);

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
