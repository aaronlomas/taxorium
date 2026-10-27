import { describe, it, expect } from 'vitest';
import { calculateSalesTaxes, clasificarAfectacion } from './salesTaxes';
import type { SaleItem } from '$lib/features/sales';
import type { Afectacion } from '$lib/features/catalogos';

const catalogoMock: Afectacion[] = [
	{ codigo: '10', descripcion: 'Gravado - Op. Onerosa (IGV 18%)' },
	{ codigo: '20', descripcion: 'Exonerado - Op. Onerosa (IGV 0%)' },
	{ codigo: '30', descripcion: 'Inafecto - Op. Onerosa (IGV 0%)' },
	{ codigo: '11', descripcion: 'Gravado - Retiro (Transferencia Gratuita)' },
	{ codigo: '31', descripcion: 'Inafecto - Retiro (Transferencia Gratuita)' }
];

describe('salesTaxes calculation', () => {
	it('clasifica correctamente los diferentes tipos de afectación', () => {
		expect(clasificarAfectacion('10', 'Gravado - Op. Onerosa').categoria).toBe('gravado');
		expect(clasificarAfectacion('20', 'Exonerado - Op. Onerosa').categoria).toBe('exonerado');
		expect(clasificarAfectacion('30', 'Inafecto - Op. Onerosa').categoria).toBe('inafecto');
		expect(clasificarAfectacion('11', 'Gravado - Retiro (Transferencia Gratuita)').categoria).toBe(
			'gratuito'
		);
		expect(clasificarAfectacion('40', 'Exportación').categoria).toBe('exportacion');
	});

	it('calcula correctamente mezcla de gravado, exonerado, inafecto, gratuito e ICBPER', () => {
		const items: SaleItem[] = [
			// Gravado S/ 59.00 -> Base 50.00, IGV 9.00
			{
				id: 1,
				productoId: 1,
				descripcion: 'Producto Gravado',
				unidad: 'NIU',
				cantidad: 1,
				precioUnitario: 59,
				subtotal: 59,
				total: 59,
				afectacion: '10',
				tieneIcbper: false,
				moneda: 'PEN'
			},
			// Exonerado S/ 20.00
			{
				id: 2,
				productoId: 2,
				descripcion: 'Producto Exonerado',
				unidad: 'NIU',
				cantidad: 2,
				precioUnitario: 10,
				subtotal: 20,
				total: 20,
				afectacion: '20',
				tieneIcbper: false,
				moneda: 'PEN'
			},
			// Inafecto S/ 15.00
			{
				id: 3,
				productoId: 3,
				descripcion: 'Producto Inafecto',
				unidad: 'NIU',
				cantidad: 1,
				precioUnitario: 15,
				subtotal: 15,
				total: 15,
				afectacion: '30',
				tieneIcbper: false,
				moneda: 'PEN'
			},
			// Gratuito S/ 30.00 (Valor referencial, no debe sumar al total pagable)
			{
				id: 4,
				productoId: 4,
				descripcion: 'Muestra médica gratuita',
				unidad: 'NIU',
				cantidad: 1,
				precioUnitario: 30,
				subtotal: 30,
				total: 30,
				afectacion: '11',
				tieneIcbper: false,
				moneda: 'PEN'
			},
			// Bolsa con ICBPER (2 bolsas a S/ 0.20 = S/ 0.40 total + 2 x 0.50 ICBPER = 1.00)
			{
				id: 5,
				productoId: 5,
				descripcion: 'Bolsa plástica',
				unidad: 'NIU',
				cantidad: 2,
				precioUnitario: 0.2,
				subtotal: 0.4,
				total: 0.4,
				afectacion: '20',
				tieneIcbper: true,
				moneda: 'PEN'
			}
		];

		const summary = calculateSalesTaxes(items, catalogoMock, 0.5);

		expect(summary.opGravadas).toBe(50);
		expect(summary.igv).toBe(9);
		expect(summary.opExoneradas).toBe(20.4);
		expect(summary.opInafectas).toBe(15);
		expect(summary.opGratuitas).toBe(30);
		expect(summary.icbper).toBe(1);
		expect(summary.bolsasIcbper).toBe(2);
		// Total pagable = 59 (gravado) + 20.40 (exonerado) + 15 (inafecto) + 1.00 (icbper) = 95.40
		expect(summary.totalPagable).toBe(95.4);
		expect(summary.porAfectacion).toHaveLength(4); // 10, 20, 30, 11
	});
});
