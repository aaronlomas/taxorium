import { ApiClient } from '../apiClient';

export interface Voucher {
	id: number;
	fecha_de_emision: string;
	cliente: string;
	numero_comprobante: string;
	estado_validez: 'registrado' | 'rechazado' | 'aceptado';
	estado_pago: 'pendiente' | 'pagado';
	moneda: string;
	gravado: number;
	igv: number;
	total: number;
	estado: boolean;
	hash_cpe: string | null;
	estado_sunat: number;
	codigo_cdr: string | null;
	descripcion_cdr: string | null;
}

export interface CreateVoucherPayload {
	fecha_de_emision: string;
	cliente: string;
	numero_comprobante: string;
	serie: string;
	correlativo: number;
	tipo_comprobante: string;
	moneda: string;
	gravado: number;
	igv: number;
	total: number;
	estado_pago?: 'pendiente' | 'pagado';
}

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
