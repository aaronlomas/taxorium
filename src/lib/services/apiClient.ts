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

// --- Seller Types ---

export interface Seller {
	id: number;
	first_name?: string;
	last_name?: string;
	username: string;
	accesses?: string;
	domain?: string;
	is_active: boolean;
}

export interface CreateSellerPayload {
	first_name?: string;
	last_name?: string;
	username: string;
	password_plain: string;
	accesses?: string;
	domain?: string;
}

export interface UpdateSellerPayload {
	first_name?: string;
	last_name?: string;
	username: string;
	password_plain?: string;
	accesses?: string;
	domain?: string;
}

export interface LoginSellerPayload {
	username: string;
	password_plain: string;
}

// --- Product Types ---

export interface Product {
	id: number;
	internal_code?: string;
	unit_code: string;
	name: string;
	sunat_code?: string;
	gsl_code?: string;
	currency: string;
	price_sale: number;
	price_purchase: number;
	stock_minimo: number;
	afectacion_venta: string;
	afectacion_compra: string;
	has_icbper: boolean;
	brand?: string;
	category?: string;
	branch?: string;
	stock_local: number;
	is_active: boolean;
}

export interface CreateProductPayload {
	internal_code?: string;
	unit_code: string;
	name: string;
	sunat_code?: string;
	gsl_code?: string;
	currency: string;
	price_sale: number;
	price_purchase: number;
	stock_minimo: number;
	afectacion_venta: string;
	afectacion_compra: string;
	has_icbper: boolean;
	brand?: string;
	category?: string;
	branch?: string;
	stock_local: number;
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
		let config = get(configStore);

		// If role hasn't loaded yet (race condition on app start), force a re-init
		if (!config.role) {
			await configStore.init();
			config = get(configStore);
		}

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

	// --- Sellers API ---

	async getSellers(): Promise<Seller[]> {
		return this.request<Seller[]>('api/sellers', 'get_sellers');
	}

	async createSeller(payload: CreateSellerPayload): Promise<Seller> {
		return this.request<Seller>('api/sellers', 'create_seller', { method: 'POST' }, payload);
	}

	async updateSeller(id: number, payload: UpdateSellerPayload): Promise<Seller> {
		const config = get(configStore);
		if (config.role === 'server') {
			return invoke<Seller>('update_seller', { id, payload });
		} else {
			return this.request<Seller>(`api/sellers/${id}`, '', { method: 'PUT' }, payload);
		}
	}

	async deleteSeller(id: number): Promise<void> {
		const config = get(configStore);
		if (config.role === 'server') {
			return invoke<void>('delete_seller', { id });
		} else {
			return this.request<void>(`api/sellers/${id}`, '', { method: 'DELETE' });
		}
	}

	async loginSeller(payload: LoginSellerPayload): Promise<Seller> {
		const config = get(configStore);
		if (config.role === 'server') {
			return invoke<Seller>('login_seller', { payload });
		} else {
			return this.request<Seller>('api/sellers/login', '', { method: 'POST' }, payload);
		}
	}

	// --- Products API ---

	async getProducts(): Promise<Product[]> {
		return this.request<Product[]>('api/products', 'get_products');
	}

	async createProduct(payload: CreateProductPayload): Promise<Product> {
		return this.request<Product>('api/products', 'create_product', { method: 'POST' }, payload);
	}

	async updateProduct(id: number, payload: CreateProductPayload): Promise<Product> {
		const config = get(configStore);
		if (config.role === 'server') {
			return invoke<Product>('update_product', { id, payload });
		} else {
			return this.request<Product>(`api/products/${id}`, '', { method: 'PUT' }, payload);
		}
	}

	async deleteProduct(id: number): Promise<void> {
		const config = get(configStore);
		if (config.role === 'server') {
			return invoke<void>('delete_product', { id });
		} else {
			return this.request<void>(`api/products/${id}`, '', { method: 'DELETE' });
		}
	}
}

export const apiClient = new ApiClient();
