import { writable } from 'svelte/store';

/**
 * Contexto compartido de una compra en curso.
 * `PurchasePanel` publica aquí la configuración seleccionada (tipo de
 * documento, proveedor, serie, moneda, etc.) y `PurchaseDetails` la consume
 * al registrar la compra.
 */
export interface PurchaseConfig {
	tipoDocumento: string;
	proveedorId: number | '';
	serie: string;
	numero: string;
	establecimiento: string;
	moneda: string;
	formaPago: string;
	fechaEmision: string;
	fechaVencimiento: string;
	infoAdicional: string;
	tipoCambio: string;
	afectacionCompra: string;
	montoPagado: string;
}

export const DEFAULT_PURCHASE_CONFIG: PurchaseConfig = {
	tipoDocumento: '01',
	proveedorId: '',
	serie: 'F001',
	numero: '',
	establecimiento: '0000',
	moneda: 'PEN',
	formaPago: 'Contado',
	fechaEmision: '',
	fechaVencimiento: '',
	infoAdicional: '',
	tipoCambio: '',
	afectacionCompra: 'NO_GRAVADO',
	montoPagado: '0'
};

function createPurchaseConfigStore() {
	const { subscribe, set, update } = writable<PurchaseConfig>({ ...DEFAULT_PURCHASE_CONFIG });

	return {
		subscribe,
		set,
		update,
		updateField: <K extends keyof PurchaseConfig>(key: K, value: PurchaseConfig[K]) =>
			update((c) => ({ ...c, [key]: value })),
		reset: () => set({ ...DEFAULT_PURCHASE_CONFIG })
	};
}

export const purchaseConfigStore = createPurchaseConfigStore();
