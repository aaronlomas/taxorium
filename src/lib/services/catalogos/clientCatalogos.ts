import { ApiClient } from '../apiClient';

export interface Moneda {
	codigo: string;
	descripcion: string;
	simbolo: string;
}

export interface Unidad {
	codigo: string;
	descripcion: string;
	simbolo: string | null;
}

export interface Sede {
	codigo: string;
	label: string;
}

export interface Afectacion {
	codigo: string;
	descripcion: string;
}

export interface TipoOperacion {
	codigo: string;
	descripcion: string;
}

export interface TipoPago {
	codigo: string;
	descripcion: string;
}

export interface TipoComprobante {
	codigo: string;
	descripcion: string;
}

export interface Serie {
	id: number;
	codigo: string;
	tipo_documento: string;
	numero_actual: number;
	activo: boolean;
}

class CatalogoClient extends ApiClient {
	async getMonedas(): Promise<Moneda[]> {
		return this.request<Moneda[]>('api/monedas', 'get_monedas');
	}

	async getUnidades(): Promise<Unidad[]> {
		return this.request<Unidad[]>('api/unidades', 'get_unidades');
	}

	async getSedes(): Promise<Sede[]> {
		return this.request<Sede[]>('api/sedes', 'get_sedes');
	}

	async getAfectacionesVenta(): Promise<Afectacion[]> {
		return this.request<Afectacion[]>('api/afectaciones/venta', 'get_afectaciones_venta');
	}

	async getAfectacionesCompra(): Promise<Afectacion[]> {
		return this.request<Afectacion[]>('api/afectaciones/compra', 'get_afectaciones_compra');
	}

	async getTiposOperacion(): Promise<TipoOperacion[]> {
		return this.request<TipoOperacion[]>('api/tipos_operacion', 'get_tipos_operacion');
	}

	async getTiposPago(): Promise<TipoPago[]> {
		return this.request<TipoPago[]>('api/tipos_pago', 'get_tipos_pago');
	}

	async getTiposComprobante(): Promise<TipoComprobante[]> {
		return this.request<TipoComprobante[]>('api/tipos_comprobante', 'get_tipos_comprobante');
	}

	async getSeries(): Promise<Serie[]> {
		return this.request<Serie[]>('api/series', 'get_series');
	}
}

export const catalogoClient = new CatalogoClient();
