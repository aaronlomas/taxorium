import { writable } from 'svelte/store';

/**
 * Contexto compartido de una venta en curso.
 * `SalesPanel` publica aquí la configuración seleccionada (tipo de comprobante,
 * cliente, serie, moneda, etc.) y `VoucherModal` la consume para generar el
 * comprobante (factura/boleta).
 */
export interface VoucherConfig {
	tipoComprobante: string;
	clienteId: number | '';
	serie: string;
	establecimiento: string;
	tipoOperacion: string;
	moneda: string;
	tipoPago: string;
	medioPago: string;
	medioPagoLabel: string;
	fechaEmision: string;
	fechaVencimiento: string;
	infoAdicional: string;
	ordenCompra: string;
	tipoCambio: string;
	montoRecibido: string;
}

export const DEFAULT_VOUCHER_CONFIG: VoucherConfig = {
	tipoComprobante: '01',
	clienteId: '',
	serie: 'F001',
	establecimiento: '0000',
	tipoOperacion: '0101',
	moneda: 'PEN',
	tipoPago: 'Contado',
	medioPago: '008',
	medioPagoLabel: 'Efectivo',
	fechaEmision: '',
	fechaVencimiento: '',
	infoAdicional: '',
	ordenCompra: '',
	tipoCambio: '',
	montoRecibido: '0'
};

function createVoucherConfigStore() {
	const { subscribe, set, update } = writable<VoucherConfig>({ ...DEFAULT_VOUCHER_CONFIG });

	return {
		subscribe,
		set,
		update,
		updateField: <K extends keyof VoucherConfig>(key: K, value: VoucherConfig[K]) =>
			update((c) => ({ ...c, [key]: value })),
		reset: () => set({ ...DEFAULT_VOUCHER_CONFIG })
	};
}

export const voucherConfigStore = createVoucherConfigStore();

/**
 * Correlativos de sesión por serie (B001, F001...).
 * La numeración principal proviene de la tabla `series` de la BD (visible en
 * el VoucherModal); aquí solo se mantiene como respaldo por si la BD no está
 * disponible.
 */
const correlativos = new Map<string, number>();

export function nextCorrelativo(serie: string): number {
	const siguiente = (correlativos.get(serie) ?? 0) + 1;
	correlativos.set(serie, siguiente);
	return siguiente;
}

/** Formatea la fecha ISO o DD/MM/AAAA a DD/MM/AAAA para impresión. */
export function formatFecha(value?: string): string {
	if (!value) return '';
	if (/^\d{2}\/\d{2}\/\d{4}$/.test(value)) return value;
	const date = new Date(value);
	if (isNaN(date.getTime())) return value;
	const dd = String(date.getDate()).padStart(2, '0');
	const mm = String(date.getMonth() + 1).padStart(2, '0');
	return `${dd}/${mm}/${date.getFullYear()}`;
}

/** Parsea una fecha ISO o DD/MM/AAAA a Date (o null si no es válida). */
export function parseFecha(value?: string): Date | null {
	if (!value) return null;
	const slash = /^(\d{1,2})\/(\d{1,2})\/(\d{4})$/.exec(value);
	if (slash) {
		const [_, d, m, y] = slash;
		return new Date(+y, +m - 1, +d);
	}
	const date = new Date(value);
	return isNaN(date.getTime()) ? null : date;
}
