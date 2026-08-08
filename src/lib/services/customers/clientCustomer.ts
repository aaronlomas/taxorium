import { ApiClient } from '../apiClient';

export interface Customer {
	id: number;
	tipo_documento: string;
	numero_documento: string;
	nombre: string;
	direccion?: string;
	correo?: string;
	activo: boolean;
}

export interface CreateCustomerPayload {
	tipo_documento: string;
	numero_documento: string;
	nombre: string;
	direccion?: string;
	correo?: string;
}

class CustomerClient extends ApiClient {
	async getCustomers(): Promise<Customer[]> {
		return this.request<Customer[]>('api/customers', 'get_customers');
	}

	async createCustomer(payload: CreateCustomerPayload): Promise<Customer> {
		return this.request<Customer>('api/customers', 'create_customer', { method: 'POST' }, payload);
	}

	async updateCustomer(id: number, payload: CreateCustomerPayload): Promise<Customer> {
		return this.request<Customer>(
			`api/customers/${id}`,
			'update_customer',
			{ method: 'PUT' },
			payload,
			{ id }
		);
	}

	async deleteCustomer(id: number): Promise<void> {
		return this.request<void>(
			`api/customers/${id}`,
			'delete_customer',
			{ method: 'DELETE' },
			undefined,
			{ id }
		);
	}
}

export const customerClient = new CustomerClient();
