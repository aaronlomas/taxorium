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

export interface VoucherItem {
	unidad: string;
	cantidad: number;
	/** Precio unitario final, con IGV incluido. */
	precio_unitario: number;
	/** Importe total de la línea con IGV incluido. */
	total: number;
	/** Código de afectación del catálogo 07 de SUNAT. */
	afectacion: string;
	descripcion: string;
}

export interface CreateVoucherPayload {
	fecha_de_emision: string;
	cliente: string;
	numero_comprobante: string;
	serie: string;
	correlativo: number;
	tipo_comprobante: string;
	moneda: string;
	estado_pago?: 'pendiente' | 'pagado';
	emisor_ruc: string;
	emisor_razon_social: string;
	emisor_ubigeo: string;
	emisor_direccion: string;
	receptor_tipo_doc: string;
	receptor_num_doc: string;
	tipo_operacion: string;
	p12_bytes: number[];
	p12_password: string;
	items: VoucherItem[];
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
