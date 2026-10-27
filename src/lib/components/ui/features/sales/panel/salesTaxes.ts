import type { SaleItem } from '$lib/features/sales';
import type { Afectacion } from '$lib/features/catalogos';

export type CategoriaAfectacion =
	| 'gravado'
	| 'exonerado'
	| 'inafecto'
	| 'gratuito'
	| 'exportacion'
	| 'otro';

export interface AfectacionBreakdown {
	codigo: string;
	descripcion: string;
	categoria: CategoriaAfectacion;
	categoriaLabel: string;
	cantidadItems: number;
	unidadesTotales: number;
	baseImponible: number;
	igv: number;
	total: number;
	esGratuita: boolean;
}

export interface SalesTaxesSummary {
	opGravadas: number;
	igv: number;
	opExoneradas: number;
	opInafectas: number;
	opGratuitas: number;
	opExportacion: number;
	icbper: number;
	bolsasIcbper: number;
	baseImponibleTotal: number;
	impuestosTotal: number;
	totalPagable: number;
	porAfectacion: AfectacionBreakdown[];
}

/**
 * Clasifica cualquier código o descripción de afectación de la tabla `afectaciones_venta`
 * según el Catálogo 07 de SUNAT.
 */
export function clasificarAfectacion(
	codigo: string,
	descripcion: string = ''
): { categoria: CategoriaAfectacion; label: string } {
	const desc = descripcion.toLowerCase();

	// Retiro o Transferencia Gratuita (códigos 11-17, 21, 31-36)
	if (
		desc.includes('gratuita') ||
		desc.includes('retiro') ||
		['11', '12', '13', '14', '15', '16', '17', '21', '31', '32', '33', '34', '35', '36'].includes(
			codigo
		)
	) {
		return { categoria: 'gratuito', label: 'Gratuita' };
	}

	// Gravado Oneroso (código 10 o descripción que indica gravado)
	if (codigo === '10' || (desc.includes('gravado') && !desc.includes('gratuita'))) {
		return { categoria: 'gravado', label: 'Gravada' };
	}

	// Exonerado Oneroso (código 20 o descripción que indica exonerado)
	if (codigo.startsWith('2') || desc.includes('exonerado')) {
		return { categoria: 'exonerado', label: 'Exonerada' };
	}

	// Inafecto Oneroso (código 30 o descripción que indica inafecto)
	if (codigo.startsWith('3') || desc.includes('inafecto')) {
		return { categoria: 'inafecto', label: 'Inafecta' };
	}

	// Exportación (código 40 o descripción que indica exportación)
	if (codigo.startsWith('4') || desc.includes('export')) {
		return { categoria: 'exportacion', label: 'Exportación' };
	}

	return { categoria: 'otro', label: 'Otros' };
}

/**
 * Calcula de forma dinámica el desglose completo de impuestos y afectaciones
 * a partir de los ítems en venta y el catálogo de afectaciones.
 */
export function calculateSalesTaxes(
	items: SaleItem[],
	catalogoAfectaciones: Afectacion[] = [],
	tasaIcbper = 0.5
): SalesTaxesSummary {
	const map = new Map<string, AfectacionBreakdown>();

	let opGravadasConIgv = 0;
	let opExoneradas = 0;
	let opInafectas = 0;
	let opGratuitas = 0;
	let opExportacion = 0;
	let bolsasIcbper = 0;

	for (const item of items) {
		const codigo = item.afectacion || '20';
		const afCat = catalogoAfectaciones.find((a) => a.codigo === codigo);
		const descripcion = afCat?.descripcion ?? `Afectación ${codigo}`;
		const { categoria, label: categoriaLabel } = clasificarAfectacion(codigo, descripcion);

		if (item.tieneIcbper) {
			bolsasIcbper += item.cantidad;
		}

		let entry = map.get(codigo);
		if (!entry) {
			entry = {
				codigo,
				descripcion,
				categoria,
				categoriaLabel,
				cantidadItems: 0,
				unidadesTotales: 0,
				baseImponible: 0,
				igv: 0,
				total: 0,
				esGratuita: categoria === 'gratuito'
			};
			map.set(codigo, entry);
		}

		entry.cantidadItems += 1;
		entry.unidadesTotales += item.cantidad;

		if (categoria === 'gravado') {
			opGravadasConIgv += item.total;
			const igvItem = Math.round(item.total * (0.18 / 1.18) * 100) / 100;
			const baseItem = Math.round((item.total - igvItem) * 100) / 100;
			entry.baseImponible = Math.round((entry.baseImponible + baseItem) * 100) / 100;
			entry.igv = Math.round((entry.igv + igvItem) * 100) / 100;
			entry.total = Math.round((entry.total + item.total) * 100) / 100;
		} else if (categoria === 'gratuito') {
			opGratuitas += item.total;
			entry.baseImponible = Math.round((entry.baseImponible + item.total) * 100) / 100;
			entry.igv = 0;
			entry.total = 0; // Transferencia gratuita no suma al total por cobrar
		} else if (categoria === 'exonerado') {
			opExoneradas += item.total;
			entry.baseImponible = Math.round((entry.baseImponible + item.total) * 100) / 100;
			entry.igv = 0;
			entry.total = Math.round((entry.total + item.total) * 100) / 100;
		} else if (categoria === 'inafecto') {
			opInafectas += item.total;
			entry.baseImponible = Math.round((entry.baseImponible + item.total) * 100) / 100;
			entry.igv = 0;
			entry.total = Math.round((entry.total + item.total) * 100) / 100;
		} else if (categoria === 'exportacion') {
			opExportacion += item.total;
			entry.baseImponible = Math.round((entry.baseImponible + item.total) * 100) / 100;
			entry.igv = 0;
			entry.total = Math.round((entry.total + item.total) * 100) / 100;
		} else {
			opExoneradas += item.total;
			entry.baseImponible = Math.round((entry.baseImponible + item.total) * 100) / 100;
			entry.igv = 0;
			entry.total = Math.round((entry.total + item.total) * 100) / 100;
		}
	}

	// IGV y Base Gravada calculados sobre el total con IGV acumulado
	const igv = Math.round(opGravadasConIgv * (0.18 / 1.18) * 100) / 100;
	const opGravadas = Math.round((opGravadasConIgv - igv) * 100) / 100;
	const icbper = Math.round(bolsasIcbper * tasaIcbper * 100) / 100;

	// Total a pagar: operaciones onerosas + icbper
	const totalOneroso = opGravadasConIgv + opExoneradas + opInafectas + opExportacion;
	const totalPagable = Math.round((totalOneroso + icbper) * 100) / 100;

	const baseImponibleTotal =
		Math.round((opGravadas + opExoneradas + opInafectas + opExportacion) * 100) / 100;
	const impuestosTotal = Math.round((igv + icbper) * 100) / 100;

	const porAfectacion = Array.from(map.values()).sort((a, b) => a.codigo.localeCompare(b.codigo));

	return {
		opGravadas,
		igv,
		opExoneradas: Math.round(opExoneradas * 100) / 100,
		opInafectas: Math.round(opInafectas * 100) / 100,
		opGratuitas: Math.round(opGratuitas * 100) / 100,
		opExportacion: Math.round(opExportacion * 100) / 100,
		icbper,
		bolsasIcbper,
		baseImponibleTotal,
		impuestosTotal,
		totalPagable,
		porAfectacion
	};
}
