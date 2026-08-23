import { ApiClient } from '../apiClient';

export interface Product {
	id: number;
	codigo_interno?: string;
	codigo_unidad: string;
	nombre: string;
	codigo_sunat?: string;
	codigo_gsl?: string;
	moneda: string;
	precio_unitario_venta: number;
	precio_unitario_compra: number;
	stock_minimo: number;
	afectacion_venta: string;
	afectacion_compra: string;
	tiene_icbper: boolean;
	marca?: string;
	categoria?: string;
	codigo_sede?: string;
	activo: boolean;
}

export interface CreateProductPayload {
	codigo_interno?: string;
	codigo_unidad: string;
	nombre: string;
	codigo_sunat?: string;
	codigo_gsl?: string;
	moneda: string;
	precio_unitario_venta: number;
	precio_unitario_compra: number;
	stock_minimo: number;
	afectacion_venta: string;
	afectacion_compra: string;
	tiene_icbper: boolean;
	marca?: string;
	categoria?: string;
	codigo_sede?: string;
}

class ProductClient extends ApiClient {
	async getProducts(): Promise<Product[]> {
		return this.request<Product[]>('api/products', 'get_products');
	}

	async createProduct(payload: CreateProductPayload): Promise<Product> {
		return this.request<Product>('api/products', 'create_product', { method: 'POST' }, payload);
	}

	async updateProduct(id: number, payload: CreateProductPayload): Promise<Product> {
		return this.request<Product>(
			`api/products/${id}`,
			'update_product',
			{ method: 'PUT' },
			payload,
			{ id }
		);
	}

	async deleteProduct(id: number): Promise<void> {
		return this.request<void>(
			`api/products/${id}`,
			'delete_product',
			{ method: 'DELETE' },
			undefined,
			{ id }
		);
	}
}

export const productClient = new ProductClient();
