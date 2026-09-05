import { writable } from 'svelte/store';

export interface SaleItem {
	id: number;
	productoId: string | number;
	descripcion: string;
	unidad: string;
	cantidad: number;
	precioUnitario: number;
	subtotal: number;
	total: number;
	afectacion: string;
	moneda: string;
}

function createSalesStore() {
	const { subscribe, set, update } = writable<SaleItem[]>([]);

	return {
		subscribe,
		add: (item: Omit<SaleItem, 'id'>) =>
			update((items) => [...items, { ...item, id: Date.now() }]),
		remove: (id: number) => update((items) => items.filter((i) => i.id !== id)),
		clear: () => set([])
	};
}

export const salesStore = createSalesStore();
