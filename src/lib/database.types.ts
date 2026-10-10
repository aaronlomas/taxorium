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
	correlativo_boleta: number;
	correlativo_factura: number;
	configurado: boolean;
	activo: boolean;
	created_at: string;
	updated_at: string;
	usuario_sol: string | null;
	clave_sol: string | null;
	certificado_path: string | null;
	clave_cert: string | null;
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

export interface LicenseLeaseRow {
	id: string;
	license_id: string;
	device_id: string;
	issued_at: string;
	expires_at: string;
	revocado: boolean;
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
			license_leases: {
				Row: LicenseLeaseRow;
				Insert: Partial<LicenseLeaseRow>;
				Update: Partial<LicenseLeaseRow>;
				Relationships: [];
			};
		};
		Views: Record<string, never>;
		Functions: Record<string, never>;
		Enums: Record<string, never>;
		CompositeTypes: Record<string, never>;
	};
}
