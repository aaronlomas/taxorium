import { derived, get, writable } from 'svelte/store';
import {
	catalogoClient,
	type Afectacion,
	type Moneda,
	type Sede,
	type Unidad,
	type TipoOperacion,
	type TipoPago,
	type TipoComprobante,
	type Serie
} from '$lib/services/catalogos/clientCatalogos';

interface CatalogoState {
	monedas: Moneda[];
	unidades: Unidad[];
	sedes: Sede[];
	afectacionesVenta: Afectacion[];
	afectacionesCompra: Afectacion[];
	tiposOperacion: TipoOperacion[];
	tiposPago: TipoPago[];
	tiposComprobante: TipoComprobante[];
	series: Serie[];
	loaded: boolean;
	loading: boolean;
}

const initialState: CatalogoState = {
	monedas: [],
	unidades: [],
	sedes: [],
	afectacionesVenta: [],
	afectacionesCompra: [],
	tiposOperacion: [],
	tiposPago: [],
	tiposComprobante: [],
	series: [],
	loaded: false,
	loading: false
};

function createCatalogoStore() {
	const { subscribe, set, update } = writable<CatalogoState>(initialState);

	return {
		subscribe,
		load: async () => {
			if (get(catalogoStore).loaded || get(catalogoStore).loading) return;

			update((s) => ({ ...s, loading: true }));

			try {
				const [
					monedas,
					unidades,
					sedes,
					afectacionesVenta,
					afectacionesCompra,
					tiposOperacion,
					tiposPago,
					tiposComprobante,
					series
				] = await Promise.all([
					catalogoClient.getMonedas(),
					catalogoClient.getUnidades(),
					catalogoClient.getSedes(),
					catalogoClient.getAfectacionesVenta(),
					catalogoClient.getAfectacionesCompra(),
					catalogoClient.getTiposOperacion(),
					catalogoClient.getTiposPago(),
					catalogoClient.getTiposComprobante(),
					catalogoClient.getSeries()
				]);

				set({
					monedas,
					unidades,
					sedes,
					afectacionesVenta,
					afectacionesCompra,
					tiposOperacion,
					tiposPago,
					tiposComprobante,
					series,
					loaded: true,
					loading: false
				});
			} catch (e) {
				console.error('Error cargando catálogos:', e);
				update((s) => ({ ...s, loading: false }));
			}
		}
	};
}

export const catalogoStore = createCatalogoStore();

// Opciones listas para el componente Select ({ value, label })
export const monedasOptions = derived(catalogoStore, ($c) =>
	$c.monedas.map((m) => ({ value: m.codigo, label: m.descripcion }))
);

export const unidadesOptions = derived(catalogoStore, ($c) =>
	$c.unidades.map((u) => ({
		value: u.codigo,
		label: u.simbolo ? `${u.simbolo} / ${u.descripcion}` : u.descripcion
	}))
);

export const sedesOptions = derived(catalogoStore, ($c) =>
	$c.sedes.map((s) => ({ value: s.codigo, label: s.label }))
);

export const afectacionesVentaOptions = derived(catalogoStore, ($c) =>
	$c.afectacionesVenta.map((a) => ({ value: a.codigo, label: a.descripcion }))
);

export const afectacionesCompraOptions = derived(catalogoStore, ($c) =>
	$c.afectacionesCompra.map((a) => ({ value: a.codigo, label: a.descripcion }))
);

export const tiposOperacionOptions = derived(catalogoStore, ($c) =>
	$c.tiposOperacion.map((t) => ({ value: t.codigo, label: t.descripcion }))
);

export const tiposPagoOptions = derived(catalogoStore, ($c) =>
	$c.tiposPago.map((t) => ({ value: t.codigo, label: t.descripcion }))
);

export const tiposComprobanteOptions = derived(catalogoStore, ($c) =>
	$c.tiposComprobante.map((t) => ({ value: t.codigo, label: t.descripcion }))
);

export const seriesOptions = derived(catalogoStore, ($c) =>
	$c.series.map((s) => ({ value: s.codigo, label: `${s.codigo} (${s.tipo_documento === '01' ? 'Factura' : s.tipo_documento === '03' ? 'Boleta' : 'Guía'})` }))
);

// Utilidad para el display de unidades en tablas
export function getUnitDisplay(code: string): string {
	const state = get(catalogoStore);
	const found = state.unidades.find((u) => u.codigo === code);
	return found ? `${found.codigo} (${found.simbolo || found.codigo})` : code;
}
