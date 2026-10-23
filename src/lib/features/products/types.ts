export interface Product {
	id: number;
	codigo_interno?: string;
	codigo_unidad: string;
	codigo_unidad_compra: string;
	factor_a_unidad_base: number;
	nombre: string;
	codigo_sunat?: string;
	codigo_gsl?: string;
	moneda: string;
	precio_unitario_venta: number;
	precio_unitario_compra: number;
	stock_minimo: number;
	afectacion_venta: string;
	afectacion_compra: string;
	tiene_icbper: boolean;
	marca?: string;
	categoria?: string;
	codigo_sede?: string;
	activo: boolean;
}

export interface CreateProductPayload {
	codigo_interno?: string;
	codigo_unidad: string;
	codigo_unidad_compra?: string;
	factor_a_unidad_base?: number;
	nombre: string;
	codigo_sunat?: string;
	codigo_gsl?: string;
	moneda: string;
	precio_unitario_venta: number;
	precio_unitario_compra: number;
	stock_minimo: number;
	afectacion_venta: string;
	afectacion_compra: string;
	tiene_icbper: boolean;
	marca?: string;
	categoria?: string;
	codigo_sede?: string;
}
