export interface Customer {
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
	activo: boolean;
}

export interface CreateCustomerPayload {
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
}
