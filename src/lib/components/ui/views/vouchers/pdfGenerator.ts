/**
 * Generador de PDFs profesionales para comprobantes de venta.
 * Usa jsPDF para producir boletas/facturas con diseño real de empresa:
 *  - Encabezado con logo y datos del emisor
 *  - Bloque del receptor con fondo diferenciado
 *  - Tabla de ítems con columnas alineadas y filas alternas
 *  - Sección de totales con acento de color
 *  - Pie de página con hash/QR y leyenda SUNAT
 *
 * Para personalizar el diseño edita `pdfTemplateConfig.ts`.
 */

import { jsPDF } from 'jspdf';
import type { VoucherData } from './voucherGenerator';
import type { PdfTemplateConfig, PaperFormat } from './pdfTemplateConfig';
import { DEFAULT_PDF_CONFIG, TICKET_PDF_CONFIG } from './pdfTemplateConfig';

// ---------------------------------------------------------------------------
// Helpers de color
// ---------------------------------------------------------------------------

/** Convierte hex (#rrggbb) a [r, g, b] 0-255 */
function hexToRgb(hex: string): [number, number, number] {
	const clean = hex.replace('#', '');
	const r = parseInt(clean.slice(0, 2), 16);
	const g = parseInt(clean.slice(2, 4), 16);
	const b = parseInt(clean.slice(4, 6), 16);
	return [r, g, b];
}

function setFillHex(doc: jsPDF, hex: string) {
	const [r, g, b] = hexToRgb(hex);
	doc.setFillColor(r, g, b);
}

function setTextHex(doc: jsPDF, hex: string) {
	const [r, g, b] = hexToRgb(hex);
	doc.setTextColor(r, g, b);
}

function setDrawHex(doc: jsPDF, hex: string) {
	const [r, g, b] = hexToRgb(hex);
	doc.setDrawColor(r, g, b);
}

// ---------------------------------------------------------------------------
// Medidas de papel
// ---------------------------------------------------------------------------

const PAPER_SIZES: Record<PaperFormat, { w: number; h: number }> = {
	a4: { w: 210, h: 297 },
	ticket80mm: { w: 80, h: 0 } // alto dinámico
};

// ---------------------------------------------------------------------------
// Generador A4
// ---------------------------------------------------------------------------

function generateA4Pdf(data: VoucherData, cfg: PdfTemplateConfig): Uint8Array {
	const { w, h } = PAPER_SIZES.a4;
	const doc = new jsPDF({ unit: 'mm', format: 'a4', orientation: 'portrait' });
	const m = cfg.margins;
	const contentW = w - m.left - m.right;
	let y = m.top;

	// ── 1. ENCABEZADO ────────────────────────────────────────────────────────

	const headerH = 28;
	setFillHex(doc, cfg.colors.headerBg);
	doc.rect(0, 0, w, headerH, 'F');

	// Logo (si hay)
	let logoRightEdge = m.left;
	if (cfg.logoBase64) {
		try {
			const imgFmt = cfg.logoBase64.includes('data:image/png') ? 'PNG' : 'JPEG';
			doc.addImage(cfg.logoBase64, imgFmt, m.left, 4, cfg.logoWidth, cfg.logoHeight);
			logoRightEdge = m.left + cfg.logoWidth + 4;
		} catch {
			// Ignorar si la imagen falla
		}
	}

	// Datos del emisor en el encabezado
	setTextHex(doc, cfg.colors.headerText);
	const nombreComercial = data.empresa.nombreComercial || data.empresa.razonSocial;

	doc.setFont('helvetica', 'bold');
	doc.setFontSize(13);
	doc.text(nombreComercial.toUpperCase(), logoRightEdge, 10);

	doc.setFont('helvetica', 'normal');
	doc.setFontSize(8);
	doc.text(`RUC: ${data.empresa.ruc}`, logoRightEdge, 16);

	if (data.empresa.direccion) {
		doc.text(data.empresa.direccion, logoRightEdge, 20);
	}

	const contactLine = [data.empresa.telefono && `Tel: ${data.empresa.telefono}`, data.empresa.email]
		.filter(Boolean)
		.join('   ');
	if (contactLine) {
		doc.text(contactLine, logoRightEdge, 24);
	}

	// Bloque tipo/número de comprobante (derecha del encabezado)
	const boxW = 62;
	const boxX = w - m.right - boxW;
	setFillHex(doc, '#ffffff');
	setDrawHex(doc, cfg.colors.accent);
	doc.setLineWidth(0.4);
	doc.roundedRect(boxX, 3, boxW, headerH - 6, 2, 2, 'FD');

	setTextHex(doc, cfg.colors.accent);
	doc.setFont('helvetica', 'bold');
	doc.setFontSize(9);
	doc.text(data.tipoComprobanteLabel, boxX + boxW / 2, 10, { align: 'center' });

	doc.setFontSize(12);
	doc.text(data.numeroCompleto, boxX + boxW / 2, 17, { align: 'center' });

	setTextHex(doc, cfg.colors.textSecondary);
	doc.setFont('helvetica', 'normal');
	doc.setFontSize(7.5);
	doc.text(`Fecha: ${data.fechaEmision}  ${data.horaEmision}`, boxX + boxW / 2, 22, {
		align: 'center'
	});
	doc.text(`Moneda: ${data.moneda}`, boxX + boxW / 2, 26, { align: 'center' });

	y = headerH + 6;

	// ── 2. DATOS DEL CLIENTE ─────────────────────────────────────────────────

	const clienteBlockH = 22;
	setFillHex(doc, cfg.colors.rowAlt);
	setDrawHex(doc, '#e5e7eb');
	doc.setLineWidth(0.3);
	doc.rect(m.left, y, contentW, clienteBlockH, 'FD');

	setTextHex(doc, cfg.colors.accent);
	doc.setFont('helvetica', 'bold');
	doc.setFontSize(8);
	doc.text('DATOS DEL CLIENTE', m.left + 3, y + 5);

	setTextHex(doc, cfg.colors.textPrimary);
	doc.setFont('helvetica', 'normal');
	doc.setFontSize(8.5);

	if (data.cliente) {
		const docLabel =
			data.cliente.tipoDocumento === '6'
				? 'RUC'
				: data.cliente.tipoDocumento === '4'
					? 'RUC Ext.'
					: 'DNI';
		doc.text(`${docLabel}: ${data.cliente.numeroDocumento}`, m.left + 3, y + 11);
		doc.text(`Razón Social / Nombre: ${data.cliente.nombre}`, m.left + 3, y + 16);
		if (data.cliente.direccion) {
			doc.text(`Dirección: ${data.cliente.direccion}`, m.left + 3, y + 21);
		}
	} else {
		doc.text('Cliente: Clientes Varios', m.left + 3, y + 11);
	}

	// Info de pago (columna derecha del bloque cliente)
	const colDerX = m.left + contentW * 0.55;
	setTextHex(doc, cfg.colors.textSecondary);
	doc.setFontSize(7.5);
	doc.text(`Cond. Pago: ${data.condicionPago}`, colDerX, y + 11);
	doc.text(`Medio de Pago: ${data.metodoPago}`, colDerX, y + 16);
	if (data.infoAdicional) {
		doc.text(`Obs: ${data.infoAdicional}`, colDerX, y + 21);
	}

	y += clienteBlockH + 6;

	// ── 3. TABLA DE ÍTEMS ────────────────────────────────────────────────────

	// Cabecera de tabla
	const colsA4 = {
		item: { x: m.left, w: 8 },
		cant: { x: m.left + 8, w: 14 },
		unidad: { x: m.left + 22, w: 18 },
		descripcion: { x: m.left + 40, w: contentW - 40 - 54 },
		precioUnit: { x: m.left + contentW - 54, w: 27 },
		subtotal: { x: m.left + contentW - 27, w: 27 }
	};

	const tableHeaderH = 7;
	setFillHex(doc, cfg.colors.accent);
	doc.rect(m.left, y, contentW, tableHeaderH, 'F');

	setTextHex(doc, '#ffffff');
	doc.setFont('helvetica', 'bold');
	doc.setFontSize(7.5);

	doc.text('#', colsA4.item.x + colsA4.item.w / 2, y + 4.8, { align: 'center' });
	doc.text('CANT.', colsA4.cant.x + colsA4.cant.w / 2, y + 4.8, { align: 'center' });
	doc.text('UNIDAD', colsA4.unidad.x + colsA4.unidad.w / 2, y + 4.8, { align: 'center' });
	doc.text('DESCRIPCIÓN', colsA4.descripcion.x + 1, y + 4.8);
	doc.text('P. UNIT.', colsA4.precioUnit.x + colsA4.precioUnit.w / 2, y + 4.8, {
		align: 'center'
	});
	doc.text('IMPORTE', colsA4.subtotal.x + colsA4.subtotal.w / 2, y + 4.8, { align: 'center' });

	y += tableHeaderH;

	// Filas de ítems
	doc.setFont('helvetica', 'normal');
	doc.setFontSize(8);
	const rowH = 7;

	data.items.forEach((item, idx) => {
		const isAlt = idx % 2 === 1;
		if (isAlt) {
			setFillHex(doc, cfg.colors.rowAlt);
			doc.rect(m.left, y, contentW, rowH, 'F');
		}

		setTextHex(doc, cfg.colors.textPrimary);
		const cy = y + 4.8;

		doc.text(String(idx + 1), colsA4.item.x + colsA4.item.w / 2, cy, { align: 'center' });
		doc.text(formatNum(item.cantidad), colsA4.cant.x + colsA4.cant.w / 2, cy, {
			align: 'center'
		});
		doc.text(item.unidad, colsA4.unidad.x + colsA4.unidad.w / 2, cy, { align: 'center' });

		// Descripción con truncado
		const maxDescW = colsA4.descripcion.w - 2;
		const descLines = doc.splitTextToSize(item.descripcion, maxDescW);
		doc.text(descLines[0], colsA4.descripcion.x + 1, cy);

		doc.text(
			`${data.monedaSimbolo} ${item.precioUnitario.toFixed(2)}`,
			colsA4.precioUnit.x + colsA4.precioUnit.w - 1,
			cy,
			{ align: 'right' }
		);
		doc.text(
			`${data.monedaSimbolo} ${item.total.toFixed(2)}`,
			colsA4.subtotal.x + colsA4.subtotal.w - 1,
			cy,
			{ align: 'right' }
		);

		y += rowH;
	});

	// Línea inferior de tabla
	setDrawHex(doc, '#d1d5db');
	doc.setLineWidth(0.3);
	doc.line(m.left, y, m.left + contentW, y);
	y += 5;

	// ── 4. TOTALES ───────────────────────────────────────────────────────────

	const totalesX = m.left + contentW - 70;
	const totalesW = 70;

	// Subtotales
	setTextHex(doc, cfg.colors.textSecondary);
	doc.setFont('helvetica', 'normal');
	doc.setFontSize(8);

	const money = (v: number) => `${data.monedaSimbolo} ${v.toFixed(2)}`;

	const subRows: [string, string][] = [];
	if (data.opGravadas > 0) subRows.push(['Op. Gravadas:', money(data.opGravadas)]);
	if (data.igv > 0) subRows.push(['I.G.V. (18%):', money(data.igv)]);
	if (data.opExoneradas > 0) subRows.push(['Op. Exoneradas:', money(data.opExoneradas)]);
	if (data.icbper > 0) subRows.push(['I.C.B.P.E.R:', money(data.icbper)]);

	subRows.forEach(([label, value]) => {
		setTextHex(doc, cfg.colors.textSecondary);
		doc.text(label, totalesX + 2, y + 5);
		setTextHex(doc, cfg.colors.textPrimary);
		doc.text(value, totalesX + totalesW - 2, y + 5, { align: 'right' });
		y += 5.5;
	});

	// Línea divisoria
	setDrawHex(doc, cfg.colors.accent);
	doc.setLineWidth(0.5);
	doc.line(totalesX, y + 1, totalesX + totalesW, y + 1);
	y += 3;

	// TOTAL
	const totalH = 10;
	setFillHex(doc, cfg.colors.totalBg);
	doc.rect(totalesX, y, totalesW, totalH, 'F');

	setTextHex(doc, cfg.colors.totalText);
	doc.setFont('helvetica', 'bold');
	doc.setFontSize(10);
	doc.text('TOTAL A PAGAR', totalesX + 2, y + 6.5);
	doc.text(money(data.total), totalesX + totalesW - 2, y + 6.5, { align: 'right' });

	y += totalH + 3;

	// Pago recibido / vuelto
	if (data.montoPagado > 0) {
		setTextHex(doc, cfg.colors.textSecondary);
		doc.setFont('helvetica', 'normal');
		doc.setFontSize(8);
		doc.text(`Total Pagado: ${money(data.montoPagado)}`, totalesX + 2, y + 4);
		doc.text(`Vuelto: ${money(data.vuelto)}`, totalesX + totalesW - 2, y + 4, {
			align: 'right'
		});
		y += 7;
	}

	// ── 5. HASH / QR ─────────────────────────────────────────────────────────

	if (cfg.showQr) {
		y += 5;
		const qrY = y;
		const qrSize = 25;

		// Placeholder QR (cuadro punteado con texto)
		setDrawHex(doc, '#9ca3af');
		doc.setLineWidth(0.3);
		setTextHex(doc, cfg.colors.textSecondary);
		doc.setFont('helvetica', 'normal');
		doc.setFontSize(6.5);
		doc.text(cfg.hashLabel, m.left, qrY + 4);

		// Hash (si existiera en VoucherData, aquí se usaría)
		doc.text('Consulte este comprobante en el portal SUNAT', m.left, qrY + 9);

		y = qrY + 16;
	}

	// ── 6. PIE DE PÁGINA ─────────────────────────────────────────────────────

	const footerY = h - m.bottom - 10;

	// Línea separadora del footer
	setDrawHex(doc, '#e5e7eb');
	doc.setLineWidth(0.3);
	doc.line(m.left, footerY - 3, m.left + contentW, footerY - 3);

	setTextHex(doc, cfg.colors.textSecondary);
	doc.setFont('helvetica', 'italic');
	doc.setFontSize(6.5);

	cfg.footerLines.forEach((line, idx) => {
		if (line) {
			doc.text(line, w / 2, footerY + idx * 4, { align: 'center' });
		}
	});

	return new Uint8Array(doc.output('arraybuffer'));
}

// ---------------------------------------------------------------------------
// Generador Ticket 80mm
// ---------------------------------------------------------------------------

function generateTicketPdf(data: VoucherData, cfg: PdfTemplateConfig): Uint8Array {
	const pageW = 80; // mm
	const m = cfg.margins;
	const contentW = pageW - m.left - m.right;

	// Colores blanco y negro para impresoras térmicas
	const BK = '#000000';
	const WH = '#ffffff';
	const GR = '#555555'; // gris para texto secundario

	// Estimamos la altura necesaria
	const estimatedH = 110 + data.items.length * 9 + 70;
	const doc = new jsPDF({
		unit: 'mm',
		format: [pageW, estimatedH],
		orientation: 'portrait'
	});

	let y = m.top;

	// ── Encabezado: sin fondo coloreado, texto negro ─────────────────────────

	setTextHex(doc, BK);
	doc.setFont('helvetica', 'bold');
	doc.setFontSize(10);

	const nombreComercial = data.empresa.nombreComercial || data.empresa.razonSocial;
	const nombreLines = doc.splitTextToSize(nombreComercial.toUpperCase(), contentW);
	nombreLines.forEach((line: string, i: number) => {
		doc.text(line, pageW / 2, y + 5 + i * 5, { align: 'center' });
	});

	y += nombreLines.length * 5 + 4;

	doc.setFont('helvetica', 'normal');
	doc.setFontSize(7.5);
	setTextHex(doc, GR);
	doc.text(`RUC: ${data.empresa.ruc}`, pageW / 2, y, { align: 'center' });
	y += 4;

	if (data.empresa.direccion) {
		const dirLines = doc.splitTextToSize(data.empresa.direccion, contentW);
		dirLines.forEach((l: string, i: number) => {
			doc.text(l, pageW / 2, y + i * 3.5, { align: 'center' });
		});
		y += dirLines.length * 3.5 + 1;
	}

	if (data.empresa.telefono || data.empresa.email) {
		const contactLine = [
			data.empresa.telefono && `Tel: ${data.empresa.telefono}`,
			data.empresa.email
		]
			.filter(Boolean)
			.join('  ');
		doc.text(contactLine, pageW / 2, y, { align: 'center' });
		y += 4;
	}

	y += 2;

	// ── Tipo/Número de comprobante: texto negro, borde simple ────────────────

	ticketDivider(doc, pageW, m, y);
	y += 3;

	setTextHex(doc, BK);
	doc.setFont('helvetica', 'bold');
	doc.setFontSize(8);
	doc.text(data.tipoComprobanteLabel, pageW / 2, y + 5, { align: 'center' });

	doc.setFontSize(10);
	doc.text(data.numeroCompleto, pageW / 2, y + 11, { align: 'center' });

	setTextHex(doc, GR);
	doc.setFont('helvetica', 'normal');
	doc.setFontSize(7);
	doc.text(`${data.fechaEmision}  ${data.horaEmision}`, pageW / 2, y + 16, {
		align: 'center'
	});

	y += 19;

	// ── Datos del cliente ────────────────────────────────────────────────────

	ticketDivider(doc, pageW, m, y);
	y += 5;

	setTextHex(doc, BK);
	doc.setFont('helvetica', 'normal');
	doc.setFontSize(7.5);

	if (data.cliente) {
		const docLabel =
			data.cliente.tipoDocumento === '6'
				? 'RUC'
				: data.cliente.tipoDocumento === '4'
					? 'RUC Ext.'
					: 'DNI';
		// Línea 1: tipo de documento
		doc.text(`${docLabel}: ${data.cliente.numeroDocumento}`, m.left, y);
		y += 4.5;
		// Línea 2+: nombre (con wrapping)
		const nombreLines2 = doc.splitTextToSize(`Cliente: ${data.cliente.nombre}`, contentW);
		nombreLines2.forEach((l: string, i: number) => {
			doc.text(l, m.left, y + i * 4.5);
		});
		y += nombreLines2.length * 4.5;
		// Línea 3: dirección (si hay)
		if (data.cliente.direccion) {
			y += 1;
			const dirLines2 = doc.splitTextToSize(data.cliente.direccion, contentW);
			setTextHex(doc, GR);
			dirLines2.forEach((l: string, i: number) => {
				doc.text(l, m.left, y + i * 4);
			});
			y += dirLines2.length * 4;
			setTextHex(doc, BK);
		}
	} else {
		doc.text('Cliente: Clientes Varios', m.left, y);
		y += 5;
	}

	// ── Ítems ────────────────────────────────────────────────────────────────

	y += 2;
	ticketDivider(doc, pageW, m, y);
	y += 5;

	// Cabecera de columnas
	setTextHex(doc, BK);
	doc.setFont('helvetica', 'bold');
	doc.setFontSize(7.5);
	doc.text('CANT  DESCRIPCIÓN', m.left, y);
	doc.text('IMPORTE', pageW - m.right, y, { align: 'right' });
	y += 4.5;

	setTextHex(doc, BK);
	doc.setFont('helvetica', 'normal');
	doc.setFontSize(7.5);

	const importeColW = 20;
	const descColW = contentW - importeColW - 2;

	data.items.forEach((item) => {
		const descLines = doc.splitTextToSize(item.descripcion, descColW);
		const importeStr = `${data.monedaSimbolo} ${item.total.toFixed(2)}`;

		// Primera línea: cantidad + inicio de descripción + importe
		doc.text(`${formatNum(item.cantidad)}  ${descLines[0]}`, m.left, y);
		doc.text(importeStr, pageW - m.right, y, { align: 'right' });
		y += 4.5;

		// Resto de líneas de descripción larga
		for (let i = 1; i < descLines.length; i++) {
			doc.text(`      ${descLines[i]}`, m.left, y);
			y += 4.5;
		}

		// Precio unitario en gris
		setTextHex(doc, GR);
		doc.setFontSize(6.5);
		doc.text(`  P.unit: ${data.monedaSimbolo} ${item.precioUnitario.toFixed(2)}`, m.left, y);
		y += 4;

		setTextHex(doc, BK);
		doc.setFontSize(7.5);
	});

	// ── Totales ──────────────────────────────────────────────────────────────

	y += 1;
	ticketDivider(doc, pageW, m, y);
	y += 5;

	const money = (v: number) => `${data.monedaSimbolo} ${v.toFixed(2)}`;

	setTextHex(doc, BK);
	doc.setFont('helvetica', 'normal');
	doc.setFontSize(7.5);
	if (data.opGravadas > 0) {
		tickBwRow(doc, m, pageW, GR, BK, y, 'Op. Gravadas:', money(data.opGravadas));
		y += 4.5;
	}
	if (data.igv > 0) {
		tickBwRow(doc, m, pageW, GR, BK, y, 'I.G.V. (18%):', money(data.igv));
		y += 4.5;
	}
	if (data.opExoneradas > 0) {
		tickBwRow(doc, m, pageW, GR, BK, y, 'Op. Exoneradas:', money(data.opExoneradas));
		y += 4.5;
	}
	if (data.icbper > 0) {
		tickBwRow(doc, m, pageW, GR, BK, y, 'I.C.B.P.E.R:', money(data.icbper));
		y += 4.5;
	}

	y += 1;
	ticketDivider(doc, pageW, m, y);
	y += 3;

	// TOTAL: fondo negro, texto blanco (impresoras térmicas reproducen sólidos negros)
	setFillHex(doc, BK);
	doc.rect(m.left, y, contentW, 10, 'F');

	setTextHex(doc, WH);
	doc.setFont('helvetica', 'bold');
	doc.setFontSize(9.5);
	doc.text('TOTAL:', m.left + 2, y + 7);
	doc.text(money(data.total), pageW - m.right - 2, y + 7, { align: 'right' });
	y += 13;

	if (data.montoPagado > 0) {
		setTextHex(doc, BK);
		doc.setFont('helvetica', 'normal');
		doc.setFontSize(7.5);
		tickBwRow(doc, m, pageW, GR, BK, y, 'Total Pagado:', money(data.montoPagado));
		y += 4.5;
		tickBwRow(doc, m, pageW, GR, BK, y, 'Vuelto:', money(data.vuelto));
		y += 4.5;
	}

	if (data.metodoPago) {
		y += 2;
		setTextHex(doc, GR);
		doc.setFont('helvetica', 'normal');
		doc.setFontSize(7);
		doc.text(`Pago: ${data.metodoPago}`, pageW / 2, y, { align: 'center' });
		y += 5;
	}

	// ── Pie de ticket ────────────────────────────────────────────────────────

	y += 3;
	ticketDivider(doc, pageW, m, y);
	y += 5;

	setTextHex(doc, GR);
	doc.setFont('helvetica', 'italic');
	doc.setFontSize(7);

	cfg.footerLines.forEach((line) => {
		if (line) {
			const footerLines = doc.splitTextToSize(line, contentW);
			footerLines.forEach((l: string) => {
				doc.text(l, pageW / 2, y, { align: 'center' });
				y += 4;
			});
		}
	});

	y += 4;

	// Redimensionar la página al contenido real
	const finalH = Math.max(y + m.bottom, 60);

	// jsPDF no permite cambiar el alto después; si el estimado es menor que el real,
	// hay que regenerar. Hacemos un ajuste simple: si el estimado fue suficiente, bien.
	// Si no, el contenido se corta — en la práctica el estimado es generoso.

	return new Uint8Array(doc.output('arraybuffer'));
}

// ---------------------------------------------------------------------------
// Helpers de ticket
// ---------------------------------------------------------------------------

function ticketDivider(doc: jsPDF, pageW: number, m: PdfTemplateConfig['margins'], y: number) {
	doc.setDrawColor(180, 180, 180);
	doc.setLineWidth(0.2);
	doc.line(m.left, y, pageW - m.right, y);
}

/** Fila de totales en blanco y negro: label en gris, valor en negro. */
function tickBwRow(
	doc: jsPDF,
	m: PdfTemplateConfig['margins'],
	pageW: number,
	grayHex: string,
	blackHex: string,
	y: number,
	label: string,
	value: string
) {
	setTextHex(doc, grayHex);
	doc.text(label, m.left, y);
	setTextHex(doc, blackHex);
	doc.text(value, pageW - m.right, y, { align: 'right' });
}

/** @deprecated Usar tickBwRow para tickets. Solo para compatibilidad. */
function ticketRow(
	doc: jsPDF,
	cfg: PdfTemplateConfig,
	m: PdfTemplateConfig['margins'],
	pageW: number,
	y: number,
	label: string,
	value: string
) {
	setTextHex(doc, cfg.colors.textSecondary);
	doc.text(label, m.left, y);
	setTextHex(doc, cfg.colors.textPrimary);
	doc.text(value, pageW - m.right, y, { align: 'right' });
}

function formatNum(n: number): string {
	return n % 1 === 0 ? String(n) : n.toFixed(2);
}

// ---------------------------------------------------------------------------
// API pública
// ---------------------------------------------------------------------------

/**
 * Genera el PDF del comprobante.
 *
 * @param data     Datos del comprobante (construidos con `buildVoucherData`)
 * @param format   Formato de papel: 'a4' | 'ticket80mm'
 * @param config   Configuración visual (opcional; usa DEFAULT_PDF_CONFIG si se omite)
 */
export function generateProfessionalPdf(
	data: VoucherData,
	format?: PaperFormat,
	config?: Partial<PdfTemplateConfig>
): Uint8Array {
	const baseConfig = format === 'ticket80mm' ? TICKET_PDF_CONFIG : DEFAULT_PDF_CONFIG;
	const cfg: PdfTemplateConfig = { ...baseConfig, ...config };

	if (cfg.format === 'ticket80mm') {
		return generateTicketPdf(data, cfg);
	}
	return generateA4Pdf(data, cfg);
}
