export interface Moneda {
	codigo: string;
	descripcion: string;
	simbolo: string;
}

export interface Unidad {
	codigo: string;
	descripcion: string;
	simbolo: string | null;
}

export interface Sede {
	codigo: string;
	label: string;
}

export interface Afectacion {
	codigo: string;
	descripcion: string;
}

export interface TipoOperacion {
	codigo: string;
	descripcion: string;
}

export interface TipoPago {
	codigo: string;
	descripcion: string;
}

export interface TipoComprobante {
	codigo: string;
	descripcion: string;
}

export interface Serie {
	id: number;
	codigo: string;
	tipo_documento: string;
	numero_actual: number;
	activo: boolean;
}
