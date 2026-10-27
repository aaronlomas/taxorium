import { describe, it, expect } from 'vitest';
import {
	buildVoucherData,
	generateVoucherPdf,
	generateVoucherXml,
	estadoPagoDesdeTipoPago,
	tipoDocumentoSunat
} from '../services/voucherGenerator';
import type { VoucherConfig } from '../state/voucherContext';

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
			tieneIcbper: false,
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
			tieneIcbper: false,
			moneda: 'PEN'
		}
	];
}

function itemBolsa(cantidad: number) {
	return {
		id: 99,
		productoId: '99',
		descripcion: 'Bolsa plástica',
		unidad: 'NIU',
		cantidad,
		precioUnitario: 0.2,
		subtotal: cantidad * 0.2,
		total: cantidad * 0.2,
		afectacion: '20',
		tieneIcbper: true,
		moneda: 'PEN'
	};
}

describe('voucherGenerator', () => {
	it('buildVoucherData calcula totales y numeración', () => {
		const data = buildVoucherData(sampleConfig(), null, sampleItems(), null);
		expect(data.total).toBe(118);
		expect(data.opExoneradas).toBe(18);
		expect(data.opGravadas).toBe(84.75);
		expect(data.igv).toBe(15.25);
		expect(data.vuelto).toBe(2);
		expect(data.numeroCompleto).toMatch(/^B001-\d{8}$/);
	});

	it('genera un PDF con cabecera válida', () => {
		const data = buildVoucherData(sampleConfig(), null, sampleItems(), null);
		const pdf = generateVoucherPdf(data);
		expect(pdf).toBeInstanceOf(Uint8Array);
		const head = new TextDecoder().decode(pdf.slice(0, 8));
		expect(head).toBe('%PDF-1.4');
	});

	it('el ICBPER se suma aparte con la tarifa configurada', () => {
		const data = buildVoucherData(sampleConfig(), null, [itemBolsa(3)], null, undefined, 0.5);
		// 3 bolsas x S/0.20 = S/0.60 de producto + 3 x S/0.50 de ICBPER.
		expect(data.icbper).toBe(1.5);
		expect(data.total).toBe(2.1);
	});

	it('mapea las etiquetas de documento a códigos SUNAT', () => {
		expect(tipoDocumentoSunat('RUC')).toBe('6');
		expect(tipoDocumentoSunat('DNI')).toBe('1');
		expect(tipoDocumentoSunat('CE')).toBe('4');
		expect(tipoDocumentoSunat('Carné de extranjería')).toBe('4');
		expect(tipoDocumentoSunat('PASAPORTE')).toBe('7');
		expect(tipoDocumentoSunat('6')).toBe('6');
	});

	it('normaliza el tipo de documento del cliente al catálogo 06 de SUNAT', () => {
		const cliente = {
			id: 1,
			tipo_documento: 'RUC',
			numero_documento: '20123456789',
			nombre: 'CLIENTE DE PRUEBA SRL',
			activo: true
		};

		const data = buildVoucherData(
			sampleConfig({ tipoComprobante: '01', serie: 'F001' }),
			null,
			sampleItems(),
			cliente
		);

		expect(data.cliente?.tipoDocumento).toBe('6');
		expect(generateVoucherXml(data)).toContain('schemeID="6"');
	});

	it('incluye el código de establecimiento en el XML', () => {
		const data = buildVoucherData(
			sampleConfig({ establecimiento: '0001' }),
			null,
			sampleItems(),
			null
		);

		expect(data.establecimiento).toBe('0001');

		const xml = generateVoucherXml(data);
		const supplier = xml
			.split('<cac:AccountingSupplierParty>')[1]
			.split('</cac:AccountingSupplierParty>')[0];
		const legalEntity = supplier
			.split('<cac:PartyLegalEntity>')[1]
			.split('</cac:PartyLegalEntity>')[0];
		const partyTaxScheme = supplier
			.split('<cac:PartyTaxScheme>')[1]
			.split('</cac:PartyTaxScheme>')[0];

		expect(legalEntity).toContain('<cbc:AddressTypeCode>0001</cbc:AddressTypeCode>');
		expect(partyTaxScheme).not.toContain('AddressTypeCode');
	});

	it('genera XML UBL 2.1 bien formado', () => {
		const data = buildVoucherData(sampleConfig(), null, sampleItems(), null);
		const xml = generateVoucherXml(data);
		expect(xml).toContain('<Invoice');
		expect(xml).toContain('urn:oasis:names:specification:ubl:schema:xsd:Invoice-2');
		expect(xml).toContain(data.numeroCompleto);
		expect(xml).toContain('<cbc:IssueDate>2026-09-04</cbc:IssueDate>');
	});

	it('incluye el tipo de cambio en el XML cuando la moneda es USD', () => {
		const data = buildVoucherData(
			sampleConfig({ moneda: 'USD', tipoCambio: '3.44' }),
			null,
			sampleItems(),
			null
		);
		expect(data.tipoCambio).toBe(3.44);

		const xml = generateVoucherXml(data);
		expect(xml).toContain('<cac:PricingExchangeRate>');
		expect(xml).toContain('<cbc:SourceCurrencyCode>USD</cbc:SourceCurrencyCode>');
		expect(xml).toContain('<cbc:TargetCurrencyCode>PEN</cbc:TargetCurrencyCode>');
		expect(xml).toContain('<cbc:CalculationRate>3.44</cbc:CalculationRate>');
	});

	it('omite el tipo de cambio cuando la moneda es PEN', () => {
		const data = buildVoucherData(
			sampleConfig({ moneda: 'PEN', tipoCambio: '3.44' }),
			null,
			sampleItems(),
			null
		);
		expect(data.tipoCambio).toBe(0);
		expect(generateVoucherXml(data)).not.toContain('<cac:PricingExchangeRate>');
	});

	it('incluye cac:PaymentTerms al contado por defecto', () => {
		const data = buildVoucherData(sampleConfig({ tipoPago: 'Contado' }), null, sampleItems(), null);
		const xml = generateVoucherXml(data);
		expect(xml).toContain('<cac:PaymentTerms>');
		expect(xml).toContain('<cbc:ID>FormaPago</cbc:ID>');
		expect(xml).toContain('<cbc:PaymentMeansID>Contado</cbc:PaymentMeansID>');
	});

	it('incluye cac:PaymentTerms al crédito con monto pendiente y cuota', () => {
		const data = buildVoucherData(
			sampleConfig({ tipoPago: 'Credito', fechaVencimiento: '15/10/2026' }),
			null,
			sampleItems(),
			null
		);
		const xml = generateVoucherXml(data);
		expect(xml).toContain('<cac:PaymentTerms>');
		expect(xml).toContain('<cbc:PaymentMeansID>Credito</cbc:PaymentMeansID>');
		expect(xml).toContain('<cbc:PaymentMeansID>Cuota001</cbc:PaymentMeansID>');
		expect(xml).toContain('<cbc:PaymentDueDate>2026-10-15</cbc:PaymentDueDate>');
		expect(xml).toContain(`<cbc:Amount currencyID="PEN">${data.total.toFixed(2)}</cbc:Amount>`);
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
