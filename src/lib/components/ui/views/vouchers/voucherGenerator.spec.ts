import { describe, it, expect } from 'vitest';
import {
	buildVoucherData,
	generateVoucherPdf,
	generateVoucherXml,
	estadoPagoDesdeTipoPago
} from './voucherGenerator';
import type { VoucherConfig } from './voucherContext';

function sampleConfig(overrides: Partial<VoucherConfig> = {}): VoucherConfig {
	return {
		tipoComprobante: '03',
		clienteId: 1,
		serie: 'B001',
		establecimiento: '0000',
		tipoOperacion: '0101',
		moneda: 'PEN',
		tipoPago: 'Contado',
		medioPago: '008',
		medioPagoLabel: 'Efectivo',
		fechaEmision: '04/09/2026',
		fechaVencimiento: '',
		infoAdicional: '',
		ordenCompra: '',
		tipoCambio: '',
		montoRecibido: '120',
		...overrides
	};
}

function sampleItems() {
	return [
		{
			id: 1,
			productoId: '1',
			descripcion: 'Teclado USB, color negro, texto más largo para probar wrapping de linea',
			unidad: 'NIU',
			cantidad: 2,
			precioUnitario: 50,
			subtotal: 100,
			total: 100,
			afectacion: '10',
			moneda: 'PEN'
		},
		{
			id: 2,
			productoId: '2',
			descripcion: 'Mouse',
			unidad: 'NIU',
			cantidad: 1,
			precioUnitario: 18,
			subtotal: 18,
			total: 18,
			afectacion: '20',
			moneda: 'PEN'
		}
	];
}

describe('voucherGenerator', () => {
	it('buildVoucherData calcula totales y numeración', () => {
		const data = buildVoucherData(sampleConfig(), null, sampleItems(), null);
		expect(data.total).toBe(118);
		expect(data.opExoneradas).toBe(18);
		expect(data.opGravadas).toBe(100);
		expect(data.vuelto).toBe(2);
		expect(data.numeroCompleto).toMatch(/^B001-\d{7}$/);
	});

	it('genera un PDF con cabecera válida', () => {
		const data = buildVoucherData(sampleConfig(), null, sampleItems(), null);
		const pdf = generateVoucherPdf(data);
		expect(pdf).toBeInstanceOf(Uint8Array);
		const head = new TextDecoder().decode(pdf.slice(0, 8));
		expect(head).toBe('%PDF-1.4');
	});

	it('genera XML UBL 2.1 bien formado', () => {
		const data = buildVoucherData(sampleConfig(), null, sampleItems(), null);
		const xml = generateVoucherXml(data);
		expect(xml).toContain('<Invoice');
		expect(xml).toContain('urn:oasis:names:specification:ubl:schema:xsd:Invoice-2');
		expect(xml).toContain(data.numeroCompleto);
		expect(xml).toContain('<cbc:IssueDate>2026-09-04</cbc:IssueDate>');
	});
});

describe('estadoPagoDesdeTipoPago', () => {
	it('marca pagado el contado, con o sin monto recibido', () => {
		expect(estadoPagoDesdeTipoPago('Contado')).toBe('pagado');

		const sinMonto = buildVoucherData(
			sampleConfig({ montoRecibido: '0' }),
			null,
			sampleItems(),
			null
		);
		expect(sinMonto.estadoPago).toBe('pagado');

		const pagoExacto = buildVoucherData(
			sampleConfig({ montoRecibido: '118' }),
			null,
			sampleItems(),
			null
		);
		expect(pagoExacto.estadoPago).toBe('pagado');
	});

	it('marca pendiente el crédito aunque se registre un monto', () => {
		expect(estadoPagoDesdeTipoPago('Credito')).toBe('pendiente');

		const conMonto = buildVoucherData(
			sampleConfig({ tipoPago: 'Credito', montoRecibido: '118' }),
			null,
			sampleItems(),
			null
		);
		expect(conMonto.estadoPago).toBe('pendiente');
	});
});
