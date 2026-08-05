import { invoke } from '@tauri-apps/api/core';
import { get } from 'svelte/store';
import { configStore } from '$lib/stores/config';

// Re-export types from backend logic if needed, or define them here
export interface Customer {
	id: number;
	document_type: string;
	document_number: string;
	name: string;
	address?: string;
	email?: string;
	is_active: boolean;
}

export interface CreateCustomerPayload {
	document_type: string;
	document_number: string;
	name: string;
	address?: string;
	email?: string;
}

class ApiClient {
	private async getBaseUrl(): Promise<string> {
		const config = get(configStore);
		if (!config.server_ip) {
			throw new Error('Server IP not configured for client node.');
		}
		// Ensure protocol is present
		let ip = config.server_ip;
		if (!ip.startsWith('http://') && !ip.startsWith('https://')) {
			ip = `http://${ip}`;
		}
		return ip;
	}

	private async request<T>(
		endpoint: string,
		tauriCommand: string,
		options: RequestInit = {},
		payload?: any
	): Promise<T> {
		const config = get(configStore);

		if (config.role === 'server') {
			// Local execution via Tauri IPC
			if (payload) {
				return invoke<T>(tauriCommand, { payload });
			} else {
				return invoke<T>(tauriCommand);
			}
		} else if (config.role === 'client') {
			// Network execution via HTTP (Axum)
			const baseUrl = await this.getBaseUrl();
			
			// Format URL correctly to avoid double slashes
			const cleanEndpoint = endpoint.startsWith('/') ? endpoint.slice(1) : endpoint;
			const url = `${baseUrl}/${cleanEndpoint}`;

			const headers = new Headers(options.headers);
			headers.set('Content-Type', 'application/json');

			const fetchOptions: RequestInit = {
				...options,
				headers,
			};

			if (payload) {
				fetchOptions.body = JSON.stringify(payload);
			}

			const response = await fetch(url, fetchOptions);

			if (!response.ok) {
				const text = await response.text();
				throw new Error(`API Error (${response.status}): ${text}`);
			}
			
			// If it's a DELETE or 204 No Content, return null as T
			if (response.status === 204) {
				return null as T;
			}

			return response.json();
		} else {
			throw new Error('Node role not configured.');
		}
	}

	// --- Customers API ---
	
	async getCustomers(): Promise<Customer[]> {
		return this.request<Customer[]>('api/customers', 'get_customers');
	}

	async createCustomer(payload: CreateCustomerPayload): Promise<Customer> {
		return this.request<Customer>('api/customers', 'create_customer', { method: 'POST' }, payload);
	}

	async updateCustomer(id: number, payload: CreateCustomerPayload): Promise<Customer> {
		// En el frontend, el id se suele pasar aparte en el comando tauri
		const config = get(configStore);
		if (config.role === 'server') {
			return invoke<Customer>('update_customer', { id, payload });
		} else {
			return this.request<Customer>(`api/customers/${id}`, '', { method: 'PUT' }, payload);
		}
	}

	async deleteCustomer(id: number): Promise<void> {
		const config = get(configStore);
		if (config.role === 'server') {
			return invoke<void>('delete_customer', { id });
		} else {
			return this.request<void>(`api/customers/${id}`, '', { method: 'DELETE' });
		}
	}
}

export const apiClient = new ApiClient();
