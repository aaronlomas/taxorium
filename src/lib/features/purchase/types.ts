export interface PurchaseItem {
	id: number;
	productoId: string | number;
	descripcion: string;
	/** Código de la unidad en la que se compra (productos.codigo_unidad_compra). */
	unidadCompra: string;
	/** Código de la unidad base/stock (productos.codigo_unidad). */
	unidadBase: string;
	/** Cuántas unidades base contiene 1 unidad de compra. */
	factorConversion: number;
	/** Cantidad tal como se ingresa en la unidad de compra. */
	cantidadCompra: number;
	/** Cantidad convertida a unidad base (cantidadCompra * factorConversion). */
	cantidadBase: number;
	/** Costo por unidad de compra. */
	costoUnitarioCompra: number;
	/** Costo por unidad base, para el kardex valorizado (costoUnitarioCompra / factor). */
	costoUnitarioBase: number;
	subtotal: number;
	afectacion: string;
	moneda: string;
}
