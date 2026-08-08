import { ApiClient } from '../apiClient';

export interface Seller {
	id: number;
	nombres?: string;
	apellidos?: string;
	usuario: string;
	accesos?: string;
	dominio?: string;
	activo: boolean;
}

export interface CreateSellerPayload {
	nombres?: string;
	apellidos?: string;
	usuario: string;
	clave_plana: string;
	accesos?: string;
	dominio?: string;
}

export interface UpdateSellerPayload {
	nombres?: string;
	apellidos?: string;
	usuario: string;
	clave_plana?: string;
	accesos?: string;
	dominio?: string;
}

export interface LoginSellerPayload {
	usuario: string;
	clave_plana: string;
}

class SellerClient extends ApiClient {
	async getSellers(): Promise<Seller[]> {
		return this.request<Seller[]>('api/sellers', 'get_sellers');
	}

	async createSeller(payload: CreateSellerPayload): Promise<Seller> {
		return this.request<Seller>('api/sellers', 'create_seller', { method: 'POST' }, payload);
	}

	async updateSeller(id: number, payload: UpdateSellerPayload): Promise<Seller> {
		return this.request<Seller>(
			`api/sellers/${id}`,
			'update_seller',
			{ method: 'PUT' },
			payload,
			{ id }
		);
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
