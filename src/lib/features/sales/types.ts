export interface SaleItem {
	id: number;
	productoId: string | number;
	descripcion: string;
	unidad: string;
	cantidad: number;
	precioUnitario: number;
	subtotal: number;
	total: number;
	afectacion: string;
	/** El producto es una bolsa plástica sujeta al ICBPER (impuesto fijo por unidad). */
	tieneIcbper: boolean;
	moneda: string;
}
