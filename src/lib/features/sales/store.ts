import { writable } from 'svelte/store';
import type { SaleItem } from './types';

function createSalesStore() {
	const { subscribe, set, update } = writable<SaleItem[]>([]);

	return {
		subscribe,
		add: (item: Omit<SaleItem, 'id'>) => update((items) => [...items, { ...item, id: Date.now() }]),
		remove: (id: number) => update((items) => items.filter((i) => i.id !== id)),
		clear: () => set([])
	};
}

export const salesStore = createSalesStore();
