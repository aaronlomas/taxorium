import { writable } from 'svelte/store';
import type { PurchaseItem } from './types';

function createPurchaseStore() {
	const { subscribe, set, update } = writable<PurchaseItem[]>([]);

	return {
		subscribe,
		add: (item: Omit<PurchaseItem, 'id'>) =>
			update((items) => [...items, { ...item, id: Date.now() }]),
		remove: (id: number) => update((items) => items.filter((i) => i.id !== id)),
		clear: () => set([])
	};
}

export const purchaseStore = createPurchaseStore();
