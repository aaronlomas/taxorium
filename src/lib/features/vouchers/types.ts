export interface Voucher {
	id: number;
	fecha_de_emision: string;
	cliente: string;
	numero_comprobante: string;
	estado_validez: 'registrado' | 'rechazado' | 'aceptado' | 'anulado';
	estado_pago: 'pendiente' | 'pagado';
	moneda: string;
	/** Tipo de cambio a soles; `null` para comprobantes emitidos en soles. */
	tipo_cambio: number | null;
	gravado: number;
	igv: number;
	total: number;
	estado: boolean;
	hash_cpe: string | null;
	estado_sunat: number;
	codigo_cdr: string | null;
	descripcion_cdr: string | null;
	forma_pago?: string;
	fecha_vencimiento?: string | null;
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
	/** El ítem es una bolsa plástica sujeta al ICBPER (impuesto fijo por unidad). */
	tiene_icbper?: boolean;
	descripcion: string;
}

export interface CreateVoucherPayload {
	fecha_de_emision: string;
	cliente: string;
	numero_comprobante: string;
	serie: string;
	correlativo: number;
	/** Código de local anexo del emisor (catálogo 22 de SUNAT, ej. "0000"). */
	establecimiento?: string;
	tipo_comprobante: string;
	moneda: string;
	/** Tipo de cambio a soles (obligatorio cuando la moneda no es PEN). */
	tipo_cambio?: number;
	estado_pago?: 'pendiente' | 'pagado';
	/** Forma de pago SUNAT: 'Contado' o 'Credito'. */
	forma_pago?: string;
	/** Fecha de vencimiento si la venta es al crédito. */
	fecha_vencimiento?: string;
	emisor_ruc: string;
	emisor_razon_social: string;
	emisor_ubigeo: string;
	emisor_direccion: string;
	receptor_tipo_doc: string;
	receptor_num_doc: string;
	tipo_operacion: string;
	p12_bytes: number[];
	p12_password: string;
	/** Tarifa ICBPER por bolsa vigente (S/ 0.50 por defecto). */
	tasa_icbper: number;
	items: VoucherItem[];
}
