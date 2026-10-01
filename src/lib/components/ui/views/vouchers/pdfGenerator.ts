/**
 * Generador de PDFs para Taxorium.
 *
 * Estrategia: abre una WebviewWindow de Tauri con la ruta /print, que carga
 * el componente InvoiceTemplate y dispara window.print() de forma nativa.
 * Esto evita completamente el uso de html2canvas (que congela en Tauri).
 *
 * Para el guardado a disco automático (flujo actual), el PDF se guarda manualmente
 * desde el diálogo de impresión nativo del OS. Si el usuario elige "Guardar como PDF",
 * el archivo queda en la ubicación que elija.
 *
 * Para mantener la API existente de VoucherModal.svelte sin romper nada, la función
 * generateProfessionalPdf ahora retorna un Uint8Array vacío y gestiona la impresión
 * de forma asíncrona abriendo la ventana de impresión.
 */

import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import type { VoucherData } from './voucherGenerator';
import type { PaperFormat, PdfTemplateConfig } from './pdfTemplateConfig';
import type { PrintableInvoiceData } from '../../print/invoiceTypes';

function mapToPrintableData(data: VoucherData, format: PaperFormat): PrintableInvoiceData {
	return {
		formato: format === 'ticket80mm' ? 'TICKET' : 'A4',
		tipo_comprobante: data.tipoComprobanteLabel,
		serie_correlativo: data.numeroCompleto,
		fecha_emision: `${data.fechaEmision} ${data.horaEmision}`,
		empresa: {
			ruc: data.empresa.ruc,
			razon_social: data.empresa.razonSocial,
			direccion: data.empresa.direccion,
			ubigeo: '',
			telefono: data.empresa.telefono || undefined,
			email: data.empresa.email || undefined
		},
		cliente: data.cliente
			? {
					tipo_doc:
						data.cliente.tipoDocumento === '6'
							? 'RUC'
							: data.cliente.tipoDocumento === '4'
								? 'RUC Ext.'
								: 'DNI',
					num_doc: data.cliente.numeroDocumento,
					nombre_o_razon_social: data.cliente.nombre,
					direccion: data.cliente.direccion
				}
			: {
					tipo_doc: 'DNI',
					num_doc: '00000000',
					nombre_o_razon_social: 'Clientes Varios'
				},
		items: data.items.map((i) => ({
			cantidad: i.cantidad,
			unidad: i.unidad,
			descripcion: i.descripcion,
			precio_unitario: i.precioUnitario,
			total: i.total
		})),
		totales: {
			moneda: data.moneda,
			gravado: data.opGravadas,
			igv: data.igv,
			exonerado: data.opExoneradas,
			total: data.total,
			total_letras: `SON: ${data.total.toFixed(2)} ${data.moneda === 'PEN' ? 'SOLES' : data.moneda}`
		},
		hash_cpe: '[Generado al enviar a SUNAT]',
		qr_code_data: '123456789'
	};
}

/**
 * Abre una ventana de impresión nativa de Tauri con el comprobante.
 * Retorna un Uint8Array vacío para mantener compatibilidad con el flujo existente.
 * El PDF se genera y guarda desde el diálogo nativo del sistema operativo.
 */
export async function generateProfessionalPdf(
	data: VoucherData,
	format: PaperFormat = 'a4',
	_config?: Partial<PdfTemplateConfig>
): Promise<Uint8Array> {
	const printableData = mapToPrintableData(data, format);

	// Guardar los datos en localStorage para que la ventana de impresión los lea
	localStorage.setItem('taxorium_print_data', JSON.stringify(printableData));

	const isTicket = format === 'ticket80mm';

	// Abrir una nueva ventana WebView de Tauri con la ruta /print
	const printWindow = new WebviewWindow(`print-${data.numeroCompleto}-${Date.now()}`, {
		url: '/print',
		title: `Comprobante: ${data.numeroCompleto}`,
		width: isTicket ? 360 : 900,
		height: 700,
		center: true,
		focus: true,
		decorations: true,
		resizable: true
	});

	// No esperamos a que se cree, simplemente retornamos para no bloquear el UI
	return new Uint8Array(0);
}
