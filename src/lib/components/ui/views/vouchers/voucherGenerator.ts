import type { SaleItem } from '$lib/stores/sales';
import type { VoucherConfig } from './voucherContext';
import { formatFecha, nextCorrelativo, parseFecha } from './voucherContext';
import type { TenantRow } from '$lib/database.types';
import type { Customer } from '$lib/services/customers/clientCustomer';

/** Formatos que SUNAT consume: PDF para entregar al cliente y XML (UBL 2.1) para el envío electrónico. */
export type VoucherFormat = 'pdf' | 'xml' | 'ambos';

export interface VoucherData {
	tipoComprobante: string;
	tipoComprobanteLabel: string;
	serie: string;
	correlativo: number;
	numeroCompleto: string;
	moneda: string;
	monedaSimbolo: string;
	fechaEmision: string;
	horaEmision: string;
	condicionPago: string;
	metodoPago: string;
	infoAdicional: string;
	empresa: {
		ruc: string;
		razonSocial: string;
		nombreComercial: string | null;
		direccion: string;
		telefono: string | null;
		email: string | null;
	};
	cliente: {
		tipoDocumento: string;
		numeroDocumento: string;
		nombre: string;
		direccion: string;
	} | null;
	items: SaleItem[];
	opGravadas: number;
	opExoneradas: number;
	igv: number;
	icbper: number;
	total: number;
	montoPagado: number;
	vuelto: number;
	estadoPago: EstadoPago;
}

export type EstadoPago = 'pagado' | 'pendiente';

const CURRENCY_SYMBOL: Record<string, string> = {
	PEN: 'S/',
	USD: '$',
	EUR: 'EUR'
};

const TIPO_COMPROBANTE_LABELS: Record<string, string> = {
	'01': 'FACTURA ELECTRÓNICA',
	'03': 'BOLETA DE VENTA ELECTRÓNICA'
};

export function tipoComprobanteLabel(codigo: string): string {
	return TIPO_COMPROBANTE_LABELS[codigo] ?? 'COMPROBANTE';
}

export function formatoMetodoPago(codigo: string): string {
	const metodos: Record<string, string> = {
		'008': 'Efectivo',
		'006': 'Tarjeta Visa (Crédito)',
		'005': 'Tarjeta Visa (Débito)',
		'003': 'Transferencia de Fondos (Yape/Plin)'
	};
	return metodos[codigo] ?? codigo;
}

/**
 * El estado de pago lo determina la condición de pago, no el monto recibido.
 * Una venta al contado se cobra en el momento (queda pagada aunque el usuario
 * no digite el monto, p. ej. pago exacto con tarjeta o transferencia); una
 * venta a crédito queda pendiente hasta su cancelación.
 */
export function estadoPagoDesdeTipoPago(tipoPago: string): EstadoPago {
	return tipoPago === 'Contado' ? 'pagado' : 'pendiente';
}

/**
 * Construye la estructura del comprobante a partir de la configuración de la
 * venta, los ítems agregados y los datos de la empresa (tenant).
 * Mantiene los mismos totales que muestra la pantalla de ventas y separa el
 * IGV asumiendo precios finales (IGV incluido) para las operaciones gravadas.
 *
 * El correlativo, si se pasa (`correlativo`), proviene de la tabla `series` de
 * la base de datos; si no, se consulta el correlativo de sesión (solo respaldo).
 */
export function buildVoucherData(
	config: VoucherConfig,
	tenant: TenantRow | null,
	items: SaleItem[],
	cliente: Customer | null,
	correlativo?: number
): VoucherData {
	const serie = config.serie || (config.tipoComprobante === '03' ? 'B001' : 'F001');
	const correlativoUsado = correlativo ?? nextCorrelativo(serie);

	const exoneradoCodigos = ['20', '21', '30'];
	const icbperCodigos = ['71', '72'];

	const total = items.reduce((sum, i) => sum + i.total, 0);
	const opExoneradas = items
		.filter((i) => exoneradoCodigos.includes(i.afectacion))
		.reduce((sum, i) => sum + i.total, 0);
	const icbper = items
		.filter((i) => icbperCodigos.includes(i.afectacion))
		.reduce((sum, i) => sum + i.total, 0);

	const opGravadas = Math.max(0, total - opExoneradas - icbper);
	const igv = total > 0 ? Math.round(opGravadas * (0.18 / 1.18) * 100) / 100 : 0;

	const moneda = config.moneda || 'PEN';
	const montoPagado = Math.max(0, Number(config.montoRecibido) || 0);
	const vuelto = montoPagado > 0 ? montoPagado - total : 0;
	const estadoPago = estadoPagoDesdeTipoPago(config.tipoPago);

	const fecha = parseFecha(config.fechaEmision) ?? new Date();
	const fechaEmision = config.fechaEmision
		? formatFecha(config.fechaEmision)
		: formatFecha(fecha.toISOString());
	const horaEmision = `${String(fecha.getHours()).padStart(2, '0')}:${String(fecha.getMinutes()).padStart(2, '0')}`;

	return {
		tipoComprobante: config.tipoComprobante,
		tipoComprobanteLabel: tipoComprobanteLabel(config.tipoComprobante),
		serie,
		correlativo: correlativoUsado,
		// 8 dígitos, igual que el backend al firmar el XML (`{:08}`). Con 7 el
		// número guardado en BD no coincidía con el `cbc:ID` del XML firmado y
		// SUNAT rechazaba el ZIP con la observación 1036.
		numeroCompleto: `${serie}-${String(correlativoUsado).padStart(8, '0')}`,
		moneda,
		monedaSimbolo: CURRENCY_SYMBOL[moneda] ?? 'S/',
		fechaEmision,
		horaEmision,
		condicionPago: config.tipoPago,
		metodoPago: formatoMetodoPago(config.medioPago),
		infoAdicional: config.infoAdicional,
		empresa: {
			ruc: tenant?.ruc ?? '00000000000',
			razonSocial: tenant?.razon_social ?? '',
			nombreComercial: tenant?.nombre_comercial ?? null,
			direccion: tenant?.direccion ?? '',
			telefono: tenant?.telefono ?? null,
			email: tenant?.email ?? null
		},
		cliente: cliente
			? {
					tipoDocumento: cliente.tipo_documento,
					numeroDocumento: cliente.numero_documento,
					nombre: cliente.nombre,
					direccion: cliente.direccion ?? ''
				}
			: null,
		items,
		opGravadas,
		opExoneradas,
		igv,
		icbper,
		total,
		montoPagado,
		vuelto,
		estadoPago
	};
}

/* -------------------------------------------------------------------------- */
/*  GENERACIÓN DE PDF (sin dependencias externas)                             */
/* -------------------------------------------------------------------------- */

interface PdfTextLine {
	x: number;
	y: number;
	size: number;
	text: string;
}

const PAGE_WIDTH = 226.77; // 80mm en puntos
const MARGIN = 8;
const MAX_W = PAGE_WIDTH - MARGIN * 2;
const LINE_FACTOR = 0.52;

function normalizeText(input: string): string {
	return input
		.normalize('NFD')
		.replace(/[\u0300-\u036f]/g, '')
		.replace(/ñ/gi, 'n')
		.replace(/[''`]/g, "'")
		.replace(/["""]/g, '"')
		.replace(/[–—]/g, '-')
		.replace(/€/g, 'EUR')
		.replace(/\u00a0/g, ' ')
		.replace(/[^\x00-\x7f]/g, '?');
}

function escapePdf(input: string): string {
	return normalizeText(input).replace(/\\/g, '\\\\').replace(/\(/g, '\\(').replace(/\)/g, '\\)');
}

function textWidth(text: string, size: number): number {
	return text.length * size * LINE_FACTOR;
}

function wrapText(text: string, size: number, maxWidth: number = MAX_W): string[] {
	const maxChars = Math.max(1, Math.floor(maxWidth / (size * LINE_FACTOR)));
	if (text.length <= maxChars) return [text];
	const words = text.split(' ');
	const lines: string[] = [];
	let current = '';
	for (const word of words) {
		const candidate = current ? `${current} ${word}` : word;
		if (candidate.length <= maxChars) {
			current = candidate;
		} else {
			if (current) lines.push(current);
			current = word;
		}
	}
	if (current) lines.push(current);
	return lines;
}

interface LayoutResult {
	width: number;
	height: number;
	lines: PdfTextLine[];
}

function layoutVoucher(data: VoucherData): LayoutResult {
	const lines: PdfTextLine[] = [];
	let y = 12;

	const lineHeight = (size: number) => size + 3;

	function push(text: string, size: number, opts: { align?: 'left' | 'center' | 'right' } = {}) {
		const width = textWidth(text, size);
		let x = MARGIN;
		if (opts.align === 'center') x = (PAGE_WIDTH - width) / 2;
		if (opts.align === 'right') x = PAGE_WIDTH - MARGIN - width;
		lines.push({ x: Math.max(0, x), y, size, text: normalizeText(text) });
		y += lineHeight(size);
	}

	function spacer(size = 4) {
		y += size;
	}

	function divider() {
		push('-'.repeat(Math.floor(MAX_W / (8 * LINE_FACTOR))), 7);
		y -= 2;
	}

	const { empresa } = data;
	const rucLine = `RUC: ${empresa.ruc}`;
	push(rucLine, 8, { align: 'center' });
	push(empresa.nombreComercial || empresa.razonSocial, 11, { align: 'center' });
	if (empresa.razonSocial && empresa.nombreComercial) {
		push(empresa.razonSocial, 8, { align: 'center' });
	}
	if (empresa.direccion) push(empresa.direccion, 8, { align: 'center' });
	if (empresa.telefono || empresa.email) {
		const line = [empresa.telefono && `Telf: ${empresa.telefono}`, empresa.email]
			.filter(Boolean)
			.join('  ');
		push(line, 7, { align: 'center' });
	}

	divider();
	spacer(2);

	push(data.tipoComprobanteLabel, 10, { align: 'center' });
	push(data.numeroCompleto, 10, { align: 'center' });
	spacer();
	push(`F. Emisión: ${data.fechaEmision} ${data.horaEmision}`, 8);
	if (data.condicionPago) push(`Condición de Pago: ${data.condicionPago}`, 8);

	divider();

	const cliente = data.cliente;
	if (cliente) {
		const nombreCliente = wrapText(cliente.nombre, 9);
		push(`Cliente: ${nombreCliente[0]}`, 9);
		for (const extra of nombreCliente.slice(1)) push(extra, 9);
		if (cliente.numeroDocumento) {
			const docLabel =
				cliente.tipoDocumento === '6'
					? 'RUC'
					: cliente.tipoDocumento === '4'
						? 'RUC (ext.)'
						: 'DNI';
			push(`${docLabel}: ${cliente.numeroDocumento}`, 8);
		}
		if (cliente.direccion) {
			const direccionCliente = wrapText(cliente.direccion, 8);
			push(direccionCliente[0], 8);
			for (const extra of direccionCliente.slice(1)) push(extra, 8);
		}
	} else {
		push('Cliente: Clientes Varios', 9);
	}

	divider();
	spacer(2);

	push('CANT.  DESCRIPCIÓN', 8);
	lines.push({
		x: PAGE_WIDTH - MARGIN - textWidth('IMPORTE', 8),
		y: y - lineHeight(8),
		size: 8,
		text: 'IMPORTE'
	});
	const importeWidth = textWidth('S/ 99999.99', 8);
	for (const item of data.items) {
		const cantidad = String(item.cantidad);
		const importe = `${data.monedaSimbolo} ${item.total.toFixed(2)}`;
		const descLines = wrapText(item.descripcion, 8, MAX_W - importeWidth - 6);
		const descripcion1 = descLines[0] ?? '';

		push(`${cantidad}  ${descripcion1}`, 8);
		lines.push({
			x: PAGE_WIDTH - MARGIN - textWidth(importe, 8),
			y: y - lineHeight(8),
			size: 8,
			text: importe
		});

		for (const extra of descLines.slice(1)) {
			push(`       ${extra}`, 8);
		}
	}

	spacer(2);
	divider();
	spacer(2);

	const money = (value: number) => `${data.monedaSimbolo} ${value.toFixed(2)}`;
	const rowLabel = (label: string, value: string, bold = false) => {
		const size = bold ? 10 : 8;
		push(label, size);
		const valueWidth = textWidth(value, size);
		lines.push({ x: PAGE_WIDTH - MARGIN - valueWidth, y: y - lineHeight(size), size, text: value });
	};

	rowLabel('OP. GRAVADAS:', money(data.opGravadas));
	rowLabel('I.G.V.:', money(data.igv));
	rowLabel('OP. EXONERADAS:', money(data.opExoneradas));
	rowLabel('I.C.B.P.E.R:', money(data.icbper));
	divider();
	rowLabel('TOTAL:', money(data.total), true);
	if (data.montoPagado > 0) {
		rowLabel('TOTAL PAGADO:', money(data.montoPagado));
		rowLabel('VUELTO:', money(data.vuelto));
	}
	if (data.metodoPago) {
		spacer(2);
		push(`Método de Pago: ${data.metodoPago}`, 7);
	}

	if (data.infoAdicional) {
		spacer(2);
		const extra = wrapText(data.infoAdicional, 7);
		push(`Obs.: ${extra[0]}`, 7);
		for (const e of extra.slice(1)) push(e, 7);
	}

	spacer();
	divider();
	spacer();
	push('GRACIAS POR SU COMPRA', 8, { align: 'center' });

	const height = y + 12;
	return { width: PAGE_WIDTH, height, lines };
}

function buildContentStream(layout: LayoutResult): string {
	const pageHeight = layout.height;
	let content = '';
	for (const line of layout.lines) {
		const escaped = escapePdf(line.text);
		content += `BT /F1 ${line.size.toFixed(2)} Tf 1 0 0 1 ${line.x.toFixed(2)} ${(pageHeight - line.y).toFixed(2)} Tm (${escaped}) Tj ET\n`;
	}
	return content;
}

function createPdf(widthPt: number, heightPt: number, contentStream: string): Uint8Array {
	const objects: Record<number, string> = {
		1: '<< /Type /Catalog /Pages 2 0 R >>',
		2: '<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
		3: `<< /Type /Page /Parent 2 0 R /MediaBox [0 0 ${widthPt.toFixed(2)} ${heightPt.toFixed(
			2
		)}] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>`,
		4: '<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>',
		5: `<< /Length ${contentStream.length} >>\nstream\n${contentStream}\nendstream`
	};

	let pdf = '%PDF-1.4\n';
	const offsets: number[] = [0];
	for (let i = 1; i <= 5; i++) {
		offsets[i] = pdf.length;
		pdf += `${i} 0 obj\n${objects[i]}\nendobj\n`;
	}

	const xrefStart = pdf.length;
	pdf += `xref\n0 6\n0000000000 65535 f \n`;
	for (let i = 1; i <= 5; i++) {
		pdf += `${String(offsets[i]).padStart(10, '0')} 00000 n \n`;
	}
	pdf += `trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n${xrefStart}\n%%EOF`;

	return new TextEncoder().encode(pdf);
}

/** Genera un PDF (ticket de 80mm) con el comprobante de venta. */
export function generateVoucherPdf(data: VoucherData): Uint8Array {
	const layout = layoutVoucher(data);
	const contentStream = buildContentStream(layout);
	return createPdf(layout.width, layout.height, contentStream);
}

/* -------------------------------------------------------------------------- */
/*  GENERACIÓN DE XML UBL 2.1 (formato electrónico SUNAT)                     */
/* -------------------------------------------------------------------------- */

function escapeXml(input: string): string {
	return input
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;')
		.replace(/'/g, '&apos;');
}

function ublId(tipo: string): string {
	// 01 = Factura, 03 = Boleta
	return tipo === '03' ? '03' : '01';
}

/** Genera el XML UBL 2.1 (Invoice) que SUNAT exige para la facturación electrónica. */
export function generateVoucherXml(data: VoucherData): string {
	const baseDate = data.fechaEmision.split(' ')[0];
	const inverseDate = baseDate.split('/').reverse().join('-');
	const mono = (value: number) => value.toFixed(2);
	const currencyId = data.moneda;

	const lines = data.items
		.map(
			(item, index) => `
			<cac:InvoiceLine>
				<cbc:ID>${index + 1}</cbc:ID>
				<cbc:InvoicedQuantity unitCode="${escapeXml(item.unidad)}">${item.cantidad}</cbc:InvoicedQuantity>
				<cbc:LineExtensionAmount currencyID="${currencyId}">${mono(item.subtotal || item.total)}</cbc:LineExtensionAmount>
				<cac:Item>
					<cbc:Description>${escapeXml(item.descripcion)}</cbc:Description>
				</cac:Item>
				<cac:Price>
					<cbc:PriceAmount currencyID="${currencyId}">${mono(item.precioUnitario)}</cbc:PriceAmount>
				</cac:Price>
			</cac:InvoiceLine>`
		)
		.join('\n');

	const taxes = `
			<cac:TaxTotal>
				<cbc:TaxAmount currencyID="${currencyId}">${mono(data.igv + data.icbper)}</cbc:TaxAmount>
				<cac:TaxSubtotal>
					<cbc:TaxableAmount currencyID="${currencyId}">${mono(data.opGravadas)}</cbc:TaxableAmount>
					<cbc:TaxAmount currencyID="${currencyId}">${mono(data.igv)}</cbc:TaxAmount>
					<cac:TaxScheme>
						<cbc:ID>1000</cbc:ID>
						<cbc:Name>IGV</cbc:Name>
						<cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>
					</cac:TaxScheme>
				</cac:TaxSubtotal>
			</cac:TaxTotal>`;

	const supplier = data.empresa;
	const customer = data.cliente;

	const xml = `<?xml version="1.0" encoding="UTF-8"?>
<Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"
	xmlns:cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
	xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2">
	<cbc:UBLVersionID>2.1</cbc:UBLVersionID>
	<cbc:CustomizationID>1.1</cbc:CustomizationID>
	<cbc:ID>${data.numeroCompleto}</cbc:ID>
	<cbc:IssueDate>${inverseDate}</cbc:IssueDate>
	<cbc:InvoiceTypeCode listID="0101">${ublId(data.tipoComprobante)}</cbc:InvoiceTypeCode>
	<cbc:DocumentCurrencyCode>${currencyId}</cbc:DocumentCurrencyCode>
	<cac:Signature>
		<cbc:ID>${data.empresa.ruc}</cbc:ID>
		<cac:SignatoryParty>
			<cac:PartyIdentification>
				<cbc:ID>${escapeXml(data.empresa.ruc)}</cbc:ID>
			</cac:PartyIdentification>
			<cac:PartyName>
				<cbc:Name>${escapeXml(data.empresa.razonSocial)}</cbc:Name>
			</cac:PartyName>
		</cac:SignatoryParty>
	</cac:Signature>
	<cac:AccountingSupplierParty>
		<cac:Party>
			<cac:PartyIdentification>
				<cbc:ID schemeID="6">${escapeXml(supplier.ruc)}</cbc:ID>
			</cac:PartyIdentification>
			<cac:PartyName>
				<cbc:Name>${escapeXml(supplier.razonSocial)}</cbc:Name>
			</cac:PartyName>
			<cac:PostalAddress>
				<cbc:AddressTypeCode>0000</cbc:AddressTypeCode>
				<cbc:Line>${escapeXml(supplier.direccion)}</cbc:Line>
			</cac:PostalAddress>
			<cac:PartyLegalEntity>
				<cbc:RegistrationName>${escapeXml(supplier.razonSocial)}</cbc:RegistrationName>
			</cac:PartyLegalEntity>
		</cac:Party>
	</cac:AccountingSupplierParty>
	<cac:AccountingCustomerParty>
		<cac:Party>
			<cac:PartyIdentification>
				<cbc:ID schemeID="${customer ? (customer.tipoDocumento === '6' ? '6' : '1') : '1'}">${escapeXml(
					customer?.numeroDocumento ?? '99999999'
				)}</cbc:ID>
			</cac:PartyIdentification>
			<cac:PartyLegalEntity>
				<cbc:RegistrationName>${escapeXml(customer?.nombre ?? 'Clientes Varios')}</cbc:RegistrationName>
			</cac:PartyLegalEntity>
		</cac:Party>
	</cac:AccountingCustomerParty>
	${taxes}
	<cac:LegalMonetaryTotal>
		<cbc:LineExtensionAmount currencyID="${currencyId}">${mono(data.opGravadas + data.opExoneradas)}</cbc:LineExtensionAmount>
		<cbc:TaxInclusiveAmount currencyID="${currencyId}">${mono(data.total)}</cbc:TaxInclusiveAmount>
		<cbc:PayableAmount currencyID="${currencyId}">${mono(data.total)}</cbc:PayableAmount>
	</cac:LegalMonetaryTotal>
	${lines}
</Invoice>`;

	return xml;
}
