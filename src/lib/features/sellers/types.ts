export interface Seller {
	id: number;
	nombres?: string;
	apellidos?: string;
	usuario: string;
	accesos?: string;
	dominio?: string;
	activo: boolean;
}

export interface CreateSellerPayload {
	nombres?: string;
	apellidos?: string;
	usuario: string;
	clave_plana: string;
	accesos?: string;
	dominio?: string;
}

export interface UpdateSellerPayload {
	nombres?: string;
	apellidos?: string;
	usuario: string;
	clave_plana?: string;
	accesos?: string;
	dominio?: string;
}

export interface LoginSellerPayload {
	usuario: string;
	clave_plana: string;
}
