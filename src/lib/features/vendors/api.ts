import { ApiClient } from '$lib/core/http';
import type { CreateVendorPayload, Vendor } from './types';

class VendorClient extends ApiClient {
	async getVendors(): Promise<Vendor[]> {
		return this.request<Vendor[]>('api/vendors', 'get_vendors');
	}

	async createVendor(payload: CreateVendorPayload): Promise<Vendor> {
		return this.request<Vendor>('api/vendors', 'create_vendor', { method: 'POST' }, payload);
	}

	async updateVendor(id: number, payload: CreateVendorPayload): Promise<Vendor> {
		return this.request<Vendor>(
			`api/vendors/${id}`,
			'update_vendor',
			{ method: 'PUT' },
			payload,
			{ id }
		);
	}

	async deleteVendor(id: number): Promise<void> {
		return this.request<void>(
			`api/vendors/${id}`,
			'delete_vendor',
			{ method: 'DELETE' },
			undefined,
			{ id }
		);
	}
}

export const vendorClient = new VendorClient();
