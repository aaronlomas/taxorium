import { ApiClient } from '$lib/core/http';
import type { CreateVoucherPayload, Voucher } from './types';

class VoucherClient extends ApiClient {
	async getVouchers(): Promise<Voucher[]> {
		return this.request<Voucher[]>('api/vouchers', 'get_vouchers');
	}

	async getNextCorrelativo(serie: string): Promise<number> {
		return this.request<number>(
			`api/vouchers/next_correlativo/${serie}`,
			'get_next_correlativo',
			{},
			undefined,
			{ serie }
		);
	}

	async createVoucher(payload: CreateVoucherPayload): Promise<Voucher> {
		return this.request<Voucher>('api/vouchers', 'create_voucher', { method: 'POST' }, payload);
	}
}

export const voucherClient = new VoucherClient();
