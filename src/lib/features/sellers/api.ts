import { ApiClient } from '$lib/core/http';
import type { CreateSellerPayload, LoginSellerPayload, Seller, UpdateSellerPayload } from './types';

class SellerClient extends ApiClient {
	async getSellers(): Promise<Seller[]> {
		return this.request<Seller[]>('api/sellers', 'get_sellers');
	}

	async createSeller(payload: CreateSellerPayload): Promise<Seller> {
		return this.request<Seller>('api/sellers', 'create_seller', { method: 'POST' }, payload);
	}

	async updateSeller(id: number, payload: UpdateSellerPayload): Promise<Seller> {
		return this.request<Seller>(`api/sellers/${id}`, 'update_seller', { method: 'PUT' }, payload, {
			id
		});
	}

	async deleteSeller(id: number): Promise<void> {
		return this.request<void>(
			`api/sellers/${id}`,
			'delete_seller',
			{ method: 'DELETE' },
			undefined,
			{ id }
		);
	}

	async loginSeller(payload: LoginSellerPayload): Promise<Seller> {
		return this.request<Seller>('api/sellers/login', 'login_seller', { method: 'POST' }, payload);
	}
}

export const sellerClient = new SellerClient();
