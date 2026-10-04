/**
 * Generador de PDFs para Taxorium.
 *
 * Estrategia: abre una WebviewWindow de Tauri con la ruta /print, que carga
 * el componente InvoiceTemplate y dispara window.print() de forma nativa.
 * Esto evita completamente el uso de html2canvas (que congela en Tauri).
 *
 * IMPORTANTE: Cada WebviewWindow de Tauri tiene su propio localStorage aislado,
 * por lo que los datos se pasan directamente como query param `d` en la URL.
 * Esto garantiza que la ventana /print siempre reciba los datos correctamente.
 *
 * FLUJO DE CONFIRMACIÓN:
 * 1. VoucherModal llama a openVoucherPreview() — no guarda nada todavía.
 * 2. La ventana /print muestra el comprobante con botones "Confirmar" y "Cancelar".
 * 3. Al confirmar, la ventana llama a createVoucher, imprime y emite el evento
 *    'taxorium:voucher-confirmed' para que la app principal limpie los stores.
 */

import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import type { VoucherData } from './voucherGenerator';
import type { PaperFormat, PdfTemplateConfig } from './pdfTemplateConfig';
import type { PrintableInvoiceData } from '../../print/invoiceTypes';
import type { CreateVoucherPayload } from '$lib/services/vouchers/clientVoucher';

// ─── Tipos exportados que consume +page.svelte ───────────────────────────────

/**
 * Payload completo que se codifica en la URL de la ventana /print.
 * Contiene los datos de previsualización Y el payload de registro que se
 * ejecutará solo si el usuario confirma.
 */
export interface PrintWindowPayload {
	/** Datos listos para renderizar InvoiceTemplate */
	printable: PrintableInvoiceData;
	/**
	 * Payload de registro para createVoucher.
	 * Se ejecuta únicamente cuando el usuario pulsa "Confirmar y Generar".
	 * Incluye el path del certificado (no los bytes, para no inflar la URL)
	 * y la contraseña leída desde el localStorage de la ventana principal.
	 * Omitido cuando viewOnly=true (comprobante ya registrado).
	 */
	registration?: Omit<CreateVoucherPayload, 'p12_bytes'> & {
		certificado_path: string;
		p12_password: string;
	};
	/** Formato de papel para @page al imprimir */
	paperFormat: PaperFormat;
	/**
	 * Si true, el comprobante ya está registrado: la ventana muestra solo
	 * el botón "Imprimir / Guardar PDF" sin opción de registro.
	 */
	viewOnly?: boolean;
}

// ─── Función privada de mapeo ─────────────────────────────────────────────────

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

// ─── API pública ──────────────────────────────────────────────────────────────

/**
 * Abre la ventana de previsualización del comprobante SIN registrar nada.
 *
 * La ventana mostrará el comprobante y tendrá dos botones:
 * - "Confirmar y Generar": llama a createVoucher, imprime y notifica a la app.
 * - "Cancelar": cierra sin efecto secundario alguno.
 *
 * @param voucherData   Datos del comprobante ya calculados.
 * @param registration  Payload de registro (sin p12_bytes; el certificado se lee
 *                      desde certificado_path dentro de la ventana al confirmar).
 * @param format        Formato de papel ('a4' | 'ticket80mm').
 */
export function openVoucherPreview(
	voucherData: VoucherData,
	registration: PrintWindowPayload['registration'],
	format: PaperFormat
): void {
	const printable = mapToPrintableData(voucherData, format);
	const isTicket = format === 'ticket80mm';

	const payload: PrintWindowPayload = { printable, registration, paperFormat: format };

	// URL absoluta necesaria en Linux/Tauri v2 (las ventanas secundarias no
	// heredan el baseURL de la ventana principal).
	const base = window.location.origin;
	const encoded = encodeURIComponent(JSON.stringify(payload));
	const url = `${base}/print?d=${encoded}`;

	new WebviewWindow(`preview-${voucherData.numeroCompleto}-${Date.now()}`, {
		url,
		title: `Vista previa — ${voucherData.numeroCompleto}`,
		// decorations: true → el marco del OS permite mover la ventana y
		// NO aparece en el PDF (window.print solo captura el HTML).
		width: isTicket ? 380 : 960,
		height: isTicket ? 780 : 720,
		center: true,
		focus: true,
		decorations: true,
		resizable: true
	});
}

/**
 * @deprecated Usa openVoucherPreview() para el nuevo flujo con confirmación.
 * Se mantiene solo por compatibilidad con código existente.
 */
export async function generateProfessionalPdf(
	data: VoucherData,
	format: PaperFormat = 'a4',
	_config?: Partial<PdfTemplateConfig>
): Promise<Uint8Array> {
	console.warn('[pdfGenerator] generateProfessionalPdf está deprecado. Usa openVoucherPreview().');
	const printable = mapToPrintableData(data, format);
	const isTicket = format === 'ticket80mm';
	const base = window.location.origin;
	const encoded = encodeURIComponent(JSON.stringify({ printable, paperFormat: format }));
	new WebviewWindow(`print-${data.numeroCompleto}-${Date.now()}`, {
		url: `${base}/print?d=${encoded}`,
		title: `Comprobante: ${data.numeroCompleto}`,
		width: isTicket ? 380 : 960,
		height: isTicket ? 780 : 720,
		center: true,
		focus: true,
		decorations: true,
		resizable: true
	});
	return new Uint8Array(0);
}
