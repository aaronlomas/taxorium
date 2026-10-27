export interface Vendor {
	id: number;
	tipo_documento: string;
	numero_documento: string;
	nombre: string;
	nombre_comercial?: string;
	pais?: string;
	departamento?: string;
	provincia?: string;
	distrito?: string;
	direccion?: string;
	telefono?: string;
	correo?: string;
	contacto?: string;
	cuenta_bancaria?: string;
	observaciones?: string;
	activo: boolean;
}

export interface CreateVendorPayload {
	tipo_documento: string;
	numero_documento: string;
	nombre: string;
	nombre_comercial?: string;
	pais?: string;
	departamento?: string;
	provincia?: string;
	distrito?: string;
	direccion?: string;
	telefono?: string;
	correo?: string;
	contacto?: string;
	cuenta_bancaria?: string;
	observaciones?: string;
}
