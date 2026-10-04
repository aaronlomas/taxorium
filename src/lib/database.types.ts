export type Json = string | number | boolean | null | { [key: string]: Json | undefined } | Json[];

export interface TenantRow {
	id: string;
	user_id: string;
	ruc: string;
	razon_social: string;
	nombre_comercial: string | null;
	direccion: string;
	ubigeo: string | null;
	departamento: string | null;
	provincia: string | null;
	distrito: string | null;
	telefono: string | null;
	email: string | null;
	usuario_sol: string | null;
	clave_sol: string | null;
	certificado_path: string | null;
	/** Contraseña del certificado .p12. Solo texto plano en dev; cifrar en prod. */
	clave_cert: string | null;
	configurado: boolean;
	activo: boolean;
	created_at: string;
	updated_at: string;
}

export interface LicenseRow {
	id: string;
	tenant_id: string;
	license_key: string;
	device_id: string | null;
	device_name: string | null;
	activa: boolean;
	activada_at: string | null;
	expiry_at: string | null;
	last_heartbeat_at: string | null;
	revocada: boolean;
	revocada_reason: string | null;
	created_at: string;
}

export interface InvoiceRow {
	id: string;
	tenant_id: string;
	tipo_comprobante: '01' | '03';
	serie: string;
	correlativo: number;
	numero_completo: string;
	tipo_doc_cliente: string | null;
	num_doc_cliente: string | null;
	nombre_cliente: string;
	direccion_cliente: string | null;
	subtotal: number;
	igv: number;
	total: number;
	moneda: string;
	items: Json;
	estado: 'BORRADOR' | 'ENVIANDO' | 'ACEPTADO' | 'RECHAZADO' | 'ANULADO';
	codigo_sunat: string | null;
	mensaje_sunat: string | null;
	xml_content: string | null;
	cdr_content: string | null;
	pdf_url: string | null;
	fecha_emision: string;
	hora_emision: string;
	device_id: string | null;
	created_at: string;
	updated_at: string;
}

export interface CustomerRow {
	id: string;
	tenant_id: string;
	tipo_doc: string;
	num_doc: string;
	nombre: string;
	direccion: string | null;
	email: string | null;
	telefono: string | null;
	activo: boolean;
	created_at: string;
}

export interface ProductRow {
	id: string;
	tenant_id: string;
	codigo: string | null;
	nombre: string;
	descripcion: string | null;
	unidad: string;
	precio: number;
	afecto_igv: boolean;
	activo: boolean;
	created_at: string;
}

export interface Database {
	public: {
		Tables: {
			tenants: {
				Row: TenantRow;
				Insert: Partial<TenantRow>;
				Update: Partial<TenantRow>;
				Relationships: [];
			};
			licenses: {
				Row: LicenseRow;
				Insert: Partial<LicenseRow>;
				Update: Partial<LicenseRow>;
				Relationships: [];
			};
			invoices: {
				Row: InvoiceRow;
				Insert: Partial<InvoiceRow>;
				Update: Partial<InvoiceRow>;
				Relationships: [];
			};
			customers: {
				Row: CustomerRow;
				Insert: Partial<CustomerRow>;
				Update: Partial<CustomerRow>;
				Relationships: [];
			};
			products: {
				Row: ProductRow;
				Insert: Partial<ProductRow>;
				Update: Partial<ProductRow>;
				Relationships: [];
			};
		};
		Functions: {
			get_next_correlativo: {
				Args: { p_tenant_id: string; p_tipo: string };
				Returns: number;
			};
		};
	};
}
