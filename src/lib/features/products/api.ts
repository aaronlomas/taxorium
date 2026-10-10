import { ApiClient } from '$lib/core/http';
import type { CreateProductPayload, Product } from './types';

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
