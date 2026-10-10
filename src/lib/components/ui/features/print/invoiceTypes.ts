export interface PrintCompanyData {
	ruc: string;
	razon_social: string;
	direccion: string;
	ubigeo: string;
	telefono?: string;
	email?: string;
	logo_url?: string;
}

export interface PrintCustomerData {
	tipo_doc: string; // Ej: 'DNI', 'RUC'
	num_doc: string;
	nombre_o_razon_social: string;
	direccion?: string;
}

export interface PrintItemData {
	cantidad: number;
	unidad: string; // Ej: 'NIU', 'ZZ'
	descripcion: string;
	precio_unitario: number;
	total: number;
}

export interface PrintTotalsData {
	moneda: string; // Ej: 'PEN', 'USD'
	gravado: number;
	igv: number;
	exonerado?: number;
	inafecto?: number;
	gratuito?: number;
	descuento?: number;
	total: number;
	total_letras?: string; // Ej: "CIEN Y 00/100 SOLES"
}

export interface PrintableInvoiceData {
	// Metadatos del comprobante
	formato?: 'A4' | 'TICKET'; // Permite al template saber si debe renderizar estilo ancho o angosto
	tipo_comprobante: string; // 'Factura Electrónica' o 'Boleta de Venta Electrónica'
	serie_correlativo: string; // Ej: 'F001-00000123'
	fecha_emision: string; // Ej: '2023-10-25 14:30:00'

	// Entidades
	empresa: PrintCompanyData;
	cliente: PrintCustomerData;

	// Detalle
	items: PrintItemData[];
	totales: PrintTotalsData;

	// Pago
	/** Forma de pago legible, p. ej. 'Efectivo - Contado' */
	forma_pago?: string;
	/** Estado de pago: 'Pagado' | 'Pendiente' */
	estado_pago?: string;

	// Extra SUNAT
	hash_cpe?: string;
	qr_code_data?: string; // Cadena para generar el QR si lo necesitan
	resolucion_sunat?: string; // Texto legal requerido
}
