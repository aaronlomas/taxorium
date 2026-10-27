<script lang="ts">
	import {
		IconDownload,
		IconFileTypePdf,
		IconFileTypeXml,
		IconReceipt,
		IconFileCertificate,
		IconAlertCircle,
		IconPrinter
	} from '@tabler/icons-svelte';
	import Modal from '$lib/components/core/primitives/feedback/Modal.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { get } from 'svelte/store';
	import { tenantStore } from '$lib/features/tenant';
	import { taxoLog } from '$lib/features/taxoLog';
	import type { Voucher } from '$lib/features/vouchers';
	import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
	import type { PrintableInvoiceData } from '$lib/components/ui/features/print/invoiceTypes';
	import { saveVoucherFile } from '$lib/components/ui/features/sales/vouchers/services/voucherFile';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		voucher: Voucher | null;
	}

	let { isOpen = $bindable(false), onClose, voucher }: Props = $props();

	let isDownloadingXml = $state(false);
	let isDownloadingCdr = $state(false);
	let isDownloadingCdrBaja = $state(false);
	let xmlError = $state<string | null>(null);
	let cdrError = $state<string | null>(null);

	$effect(() => {
		if (isOpen) {
			xmlError = null;
			cdrError = null;
			isDownloadingXml = false;
			isDownloadingCdr = false;
			isDownloadingCdrBaja = false;
		}
	});

	/**
	 * Formato 1: PDF (A4 o Ticket 80mm).
	 * Abre la ventana nativa de previsualización e impresión del comprobante en el formato elegido.
	 * Intenta parsear los ítems reales del XML firmado para mostrar el detalle completo.
	 */
	async function handleDescargarPdf(paperFormat: 'a4' | 'ticket80mm') {
		if (!voucher) return;
		const tenant = get(tenantStore).tenant;
		if (!tenant) {
			taxoLog.error('No se encontraron datos de la empresa.', 'comprobantes');
			return;
		}

		const CURRENCY_LABEL: Record<string, string> = { PEN: 'SOLES', USD: 'DÓLARES', EUR: 'EUROS' };

		let parsedItems: PrintableInvoiceData['items'] | null = null;
		let parsedCliente: PrintableInvoiceData['cliente'] | null = null;
		let parsedFormaPago: string | undefined = undefined;

		try {
			const xmlBytes = await invoke<number[]>('descargar_documento_sunat', {
				voucherId: voucher.id,
				tipo: 'XML'
			});
			if (xmlBytes && xmlBytes.length > 0) {
				const xmlStr = new TextDecoder().decode(new Uint8Array(xmlBytes));
				const parser = new DOMParser();
				const xmlDoc = parser.parseFromString(xmlStr, 'application/xml');

				const customerParty = xmlDoc.getElementsByTagNameNS('*', 'AccountingCustomerParty')[0];
				if (customerParty) {
					const idEl = customerParty.getElementsByTagNameNS('*', 'ID')[0];
					const nameEl = customerParty.getElementsByTagNameNS('*', 'RegistrationName')[0];
					const schemeId = idEl?.getAttribute('schemeID') ?? '1';
					parsedCliente = {
						tipo_doc: schemeId === '6' ? 'RUC' : schemeId === '4' ? 'RUC Ext.' : 'DNI',
						num_doc: idEl?.textContent?.trim() ?? '',
						nombre_o_razon_social: nameEl?.textContent?.trim() ?? voucher.cliente
					};
				}

				const paymentTerms = xmlDoc.getElementsByTagNameNS('*', 'PaymentTerms');
				for (let i = 0; i < paymentTerms.length; i++) {
					const meansId = paymentTerms[i].getElementsByTagNameNS('*', 'PaymentMeansID')[0];
					if (meansId?.textContent) {
						parsedFormaPago = meansId.textContent.trim();
						break;
					}
				}

				const lines =
					xmlDoc.getElementsByTagNameNS('*', 'InvoiceLine').length > 0
						? xmlDoc.getElementsByTagNameNS('*', 'InvoiceLine')
						: xmlDoc.getElementsByTagNameNS('*', 'CreditNoteLine').length > 0
							? xmlDoc.getElementsByTagNameNS('*', 'CreditNoteLine')
							: xmlDoc.getElementsByTagNameNS('*', 'DebitNoteLine');

				if (lines.length > 0) {
					parsedItems = [];
					for (let i = 0; i < lines.length; i++) {
						const line = lines[i];
						const qtyEl =
							line.getElementsByTagNameNS('*', 'InvoicedQuantity')[0] ??
							line.getElementsByTagNameNS('*', 'CreditedQuantity')[0] ??
							line.getElementsByTagNameNS('*', 'DebitedQuantity')[0];
						const descEl = line.getElementsByTagNameNS('*', 'Description')[0];
						const priceEl = line.getElementsByTagNameNS('*', 'PriceAmount')[0];
						const totalEl = line.getElementsByTagNameNS('*', 'LineExtensionAmount')[0];

						parsedItems.push({
							cantidad: qtyEl ? parseFloat(qtyEl.textContent?.trim() || '1') : 1,
							unidad: qtyEl?.getAttribute('unitCode') || 'NIU',
							descripcion: descEl?.textContent?.trim() || `Ítem ${i + 1}`,
							precio_unitario: priceEl ? parseFloat(priceEl.textContent?.trim() || '0') : 0,
							total: totalEl ? parseFloat(totalEl.textContent?.trim() || '0') : 0
						});
					}
				}
			}
		} catch {
			// Fallback a los datos del comprobante
		}

		const printable: PrintableInvoiceData = {
			formato: paperFormat === 'ticket80mm' ? 'TICKET' : 'A4',
			tipo_comprobante: voucher.numero_comprobante.startsWith('F')
				? 'FACTURA ELECTRÓNICA'
				: voucher.numero_comprobante.startsWith('B')
					? 'BOLETA DE VENTA ELECTRÓNICA'
					: voucher.numero_comprobante.startsWith('FC') ||
						  voucher.numero_comprobante.startsWith('BC')
						? 'NOTA DE CRÉDITO ELECTRÓNICA'
						: 'COMPROBANTE ELECTRÓNICO',
			serie_correlativo: voucher.numero_comprobante,
			fecha_emision: voucher.fecha_de_emision,
			tipo_cambio: voucher.tipo_cambio ?? undefined,
			empresa: {
				ruc: tenant.ruc,
				razon_social: tenant.razon_social,
				direccion: tenant.direccion,
				ubigeo: tenant.ubigeo ?? '',
				telefono: tenant.telefono ?? undefined,
				email: tenant.email ?? undefined
			},
			cliente: parsedCliente ?? {
				tipo_doc: voucher.numero_comprobante.startsWith('F') ? 'RUC' : 'DNI',
				num_doc: '',
				nombre_o_razon_social: voucher.cliente
			},
			items:
				parsedItems && parsedItems.length > 0
					? parsedItems
					: [
							{
								cantidad: 1,
								unidad: 'NIU',
								descripcion: `Total gravado comprobante ${voucher.numero_comprobante}`,
								precio_unitario: voucher.gravado,
								total: voucher.gravado
							}
						],
			totales: {
				moneda: voucher.moneda,
				gravado: voucher.gravado,
				igv: voucher.igv,
				total: voucher.total,
				total_letras: `SON: ${voucher.total.toFixed(2)} ${CURRENCY_LABEL[voucher.moneda] ?? voucher.moneda}`
			},
			forma_pago: parsedFormaPago ?? voucher.forma_pago ?? 'Contado',
			estado_pago: voucher.estado_pago === 'pagado' ? 'Pagado' : 'Pendiente',
			hash_cpe: voucher.hash_cpe ?? undefined
		};

		const base = window.location.origin;
		const encoded = encodeURIComponent(JSON.stringify({ printable, paperFormat, viewOnly: true }));
		const isTicket = paperFormat === 'ticket80mm';
		new WebviewWindow(`view-${voucher.numero_comprobante}-${paperFormat}-${Date.now()}`, {
			url: `${base}/print?d=${encoded}`,
			title: `Comprobante ${paperFormat.toUpperCase()} — ${voucher.numero_comprobante}`,
			width: isTicket ? 768 : 960,
			height: isTicket ? 780 : 720,
			center: true,
			focus: true,
			decorations: true,
			resizable: true
		});

		taxoLog.info(
			`Abriendo comprobante ${voucher.numero_comprobante} en formato ${paperFormat === 'ticket80mm' ? 'Ticket 80mm' : 'A4'}`,
			'comprobantes'
		);
	}

	/**
	 * Formato 2: XML (Comprobante Electrónico UBL 2.1 firmado).
	 * Guarda el archivo XML que legalmente constituye el comprobante oficial.
	 */
	async function handleDescargarXml() {
		if (!voucher) return;
		isDownloadingXml = true;
		xmlError = null;
		try {
			const bytes = await invoke<number[]>('descargar_documento_sunat', {
				voucherId: voucher.id,
				tipo: 'XML'
			});
			if (!bytes || bytes.length === 0) {
				throw new Error('El comprobante no tiene XML disponible.');
			}

			const tenant = get(tenantStore).tenant;
			const ruc = tenant?.ruc ? `${tenant.ruc}-` : '';
			const tipoCode = voucher.numero_comprobante.startsWith('F')
				? '01-'
				: voucher.numero_comprobante.startsWith('B')
					? '03-'
					: '';
			const filename = `${ruc}${tipoCode}${voucher.numero_comprobante}.xml`;

			const saved = await saveVoucherFile(new Uint8Array(bytes), filename, 'xml');
			if (saved) {
				taxoLog.info(`XML guardado exitosamente: ${filename}`, 'comprobantes');
			}
		} catch (e) {
			const msg = e instanceof Error ? e.message : typeof e === 'string' ? e : JSON.stringify(e);
			xmlError = msg;
			taxoLog.error(`Error al descargar XML: ${msg}`, 'comprobantes');
		} finally {
			isDownloadingXml = false;
		}
	}

	/**
	 * Formato 3: CDR (Constancia de Recepción de SUNAT).
	 * Guarda la constancia emitida por SUNAT con la certificación del documento.
	 */
	async function handleDescargarCdr() {
		if (!voucher) return;
		if (voucher.estado_sunat === 0 && !voucher.codigo_cdr) {
			taxoLog.warn('Este comprobante aún no ha sido emitido a SUNAT.', 'comprobantes');
			return;
		}

		isDownloadingCdr = true;
		cdrError = null;
		try {
			const bytes = await invoke<number[]>('descargar_documento_sunat', {
				voucherId: voucher.id,
				tipo: 'CDR'
			});
			if (!bytes || bytes.length === 0) {
				throw new Error('No se encontró constancia CDR para este comprobante.');
			}

			const tenant = get(tenantStore).tenant;
			const ruc = tenant?.ruc ? `${tenant.ruc}-` : '';
			const tipoCode = voucher.numero_comprobante.startsWith('F')
				? '01-'
				: voucher.numero_comprobante.startsWith('B')
					? '03-'
					: '';
			const filename = `R-${ruc}${tipoCode}${voucher.numero_comprobante}.xml`;

			const saved = await saveVoucherFile(new Uint8Array(bytes), filename, 'xml');
			if (saved) {
				taxoLog.info(`CDR guardado exitosamente: ${filename}`, 'comprobantes');
			}
		} catch (e) {
			const msg = e instanceof Error ? e.message : typeof e === 'string' ? e : JSON.stringify(e);
			cdrError = msg;
			taxoLog.error(`Error al descargar CDR: ${msg}`, 'comprobantes');
		} finally {
			isDownloadingCdr = false;
		}
	}

	/**
	 * Formato CDR de Baja para comprobantes anulados.
	 */
	async function handleDescargarCdrBaja() {
		if (!voucher) return;
		isDownloadingCdrBaja = true;
		try {
			const bytes = await invoke<number[]>('descargar_documento_sunat', {
				voucherId: voucher.id,
				tipo: 'CDR_BAJA'
			});
			if (!bytes || bytes.length === 0) {
				throw new Error('No se encontró constancia CDR de baja para este comprobante.');
			}

			const tenant = get(tenantStore).tenant;
			const ruc = tenant?.ruc ? `${tenant.ruc}-` : '';
			const filename = `R-BAJA-${ruc}${voucher.numero_comprobante}.xml`;

			const saved = await saveVoucherFile(new Uint8Array(bytes), filename, 'xml');
			if (saved) {
				taxoLog.info(`CDR de baja guardado exitosamente: ${filename}`, 'comprobantes');
			}
		} catch (e) {
			const msg = e instanceof Error ? e.message : typeof e === 'string' ? e : JSON.stringify(e);
			taxoLog.error(`Error al descargar CDR de baja: ${msg}`, 'comprobantes');
		} finally {
			isDownloadingCdrBaja = false;
		}
	}
</script>

<Modal
	bind:isOpen
	{onClose}
	title={voucher
		? `Descargas del Comprobante — ${voucher.numero_comprobante}`
		: 'Descargar Comprobante'}
	closable={true}
>
	{#if voucher}
		<div class="w-full p-3 text-sm">
			<!-- Formatos Esenciales (según Tarea 2 tasks.md) -->
			<div class="space-y-3">
				<!-- 1. PDF -->
				<div class="rounded-md border border-neutral-800 bg-neutral-900/40">
					<!-- Icono -->
					<div class="flex justify-between">
						<div class="flex gap-3 p-3">
							<div
								class="flex size-9 shrink-0 items-center justify-center rounded-lg border border-neutral-800 text-red-400"
							>
								<IconFileTypePdf size={20} />
							</div>
							<!-- titulo y descripción -->
							<div>
								<h4 class="font-medium text-white">Formato PDF (Representación Impresa)</h4>
								<p class="mt-0.5 text-xs text-neutral-400">
									Formato para la impresión del comprobante.
								</p>
							</div>
						</div>
						<span
							class="self-start rounded bg-neutral-800 px-1.5 py-0.5 text-[10px] text-neutral-400"
							>Cliente final</span
						>
					</div>
					<!-- Botones -->
					<div class="flex gap-3 p-3">
						<button
							type="button"
							class="flex cursor-pointer items-center gap-1.5 rounded-sm border border-red-400/40 bg-red-950/30 px-3 py-1.5 text-xs font-medium text-red-300 transition-colors hover:bg-red-900/40 hover:text-white"
							onclick={() => handleDescargarPdf('a4')}
						>
							<IconPrinter size={15} />
							<span>Formato A4</span>
						</button>
						<button
							type="button"
							class="flex cursor-pointer items-center gap-1.5 rounded-sm border border-red-400/40 bg-red-950/30 px-3 py-1.5 text-xs font-medium text-red-300 transition-colors hover:bg-red-900/40 hover:text-white"
							onclick={() => handleDescargarPdf('ticket80mm')}
						>
							<IconReceipt size={15} />
							<span>Formato Ticket (80mm)</span>
						</button>
					</div>
				</div>

				<!-- 2. XML -->
				<div class="rounded-md border border-neutral-800 bg-neutral-900/40">
					<!-- Icono -->
					<div class="flex justify-between">
						<div class="flex gap-3 p-3">
							<div
								class="flex size-9 shrink-0 items-center justify-center rounded-lg border border-neutral-800 text-blue-400"
							>
								<IconFileTypeXml size={20} />
							</div>
							<!-- titulo y descripción -->
							<div>
								<h4 class="font-medium text-white">Formato XML (Comprobante Electrónico)</h4>
								<p class="mt-0.5 text-xs text-neutral-400">
									Comprobante digital firmado con estándar UBL 2.1
								</p>
							</div>
						</div>
						<span
							class="self-start rounded bg-neutral-800 px-1.5 py-0.5 text-[10px] text-neutral-400"
							>SUNAT / Contador</span
						>
					</div>
					<!-- botones -->
					<div class="p-3">
						<button
							type="button"
							class="flex shrink-0 cursor-pointer items-center gap-1.5 rounded-sm border border-blue-500/40 bg-blue-950/30 px-3 py-1.5 text-xs font-medium text-blue-300 transition-colors hover:bg-blue-900/40 hover:text-white disabled:opacity-50"
							onclick={handleDescargarXml}
							disabled={isDownloadingXml}
						>
							{#if isDownloadingXml}
								<span
									class="inline-block size-3.5 animate-spin rounded-full border-2 border-blue-400 border-t-transparent"
								></span>
								<span>Descargando...</span>
							{:else}
								<IconDownload size={15} />
								<span>Descargar XML</span>
							{/if}
						</button>
					</div>

					{#if xmlError}
						<div
							class="mt-2 rounded border border-red-500/30 bg-red-950/30 px-2 py-1 text-xs text-red-300"
						>
							{xmlError}
						</div>
					{/if}
				</div>

				<!-- 3. CDR -->
				<div class="rounded-md border border-neutral-800 bg-neutral-900/40">
					<div class="flex justify-between">
						<div class="flex gap-3 p-3">
							<!-- Icono -->
							<div
								class="flex size-9 shrink-0 items-center justify-center rounded-lg border border-neutral-800 text-emerald-400"
							>
								<IconFileCertificate size={20} />
							</div>
							<!-- titulo y descripcion -->
							<div>
								<h4 class="font-medium text-white">CDR (Constancia de Recepción)</h4>

								<p class="mt-0.5 text-xs text-neutral-400">
									Constancia de recepción y validez del comprobante.
								</p>
								{#if (voucher.estado_sunat === 0 && !voucher.codigo_cdr) || (voucher.codigo_cdr && voucher.codigo_cdr !== '0')}
									<div class="mt-1 flex items-center gap-2 text-xs">
										{#if voucher.estado_sunat === 0 && !voucher.codigo_cdr}
											<span class="flex items-center gap-1 text-amber-400">
												<IconAlertCircle size={13} />
												Emisión pendiente ante SUNAT
											</span>
										{:else if voucher.codigo_cdr && voucher.codigo_cdr !== '0'}
											<span class="text-red-400">
												SUNAT Código {voucher.codigo_cdr}
												{voucher.descripcion_cdr ? `— ${voucher.descripcion_cdr}` : ''}
											</span>
										{/if}
									</div>
								{/if}
							</div>
						</div>
						<span
							class="self-start rounded bg-neutral-800 px-1.5 py-0.5 text-[10px] text-neutral-400"
							>Emisor / Auditoría</span
						>
					</div>
					<!-- botones -->
					<div class="flex gap-3 p-3">
						<button
							type="button"
							class="flex cursor-pointer items-center gap-1.5 rounded-sm border border-emerald-500/40 bg-emerald-950/30 px-3 py-1.5 text-xs font-medium text-emerald-300 transition-colors hover:bg-emerald-900/40 hover:text-white disabled:cursor-not-allowed disabled:opacity-40"
							onclick={handleDescargarCdr}
							disabled={isDownloadingCdr || (voucher.estado_sunat === 0 && !voucher.codigo_cdr)}
							title={voucher.estado_sunat === 0 && !voucher.codigo_cdr
								? 'Debe emitir el comprobante a SUNAT para obtener el CDR'
								: 'Descargar Constancia de Recepción'}
						>
							{#if isDownloadingCdr}
								<span
									class="inline-block size-3.5 animate-spin rounded-full border-2 border-emerald-400 border-t-transparent"
								></span>
								<span>Descargando...</span>
							{:else}
								<IconDownload size={15} />
								<span>Descargar CDR (.xml)</span>
							{/if}
						</button>
						{#if voucher.estado_validez === 'anulado'}
							<button
								type="button"
								class="flex cursor-pointer items-center gap-1.5 rounded-sm border border-rose-500/40 bg-rose-950/30 px-2.5 py-1 text-xs font-medium text-rose-300 transition-colors hover:bg-rose-900/40 hover:text-white disabled:opacity-50"
								onclick={handleDescargarCdrBaja}
								disabled={isDownloadingCdrBaja}
							>
								{#if isDownloadingCdrBaja}
									<span
										class="inline-block size-3 animate-spin rounded-full border-2 border-rose-400 border-t-transparent"
									></span>
								{:else}
									<IconDownload size={13} />
								{/if}
								<span>CDR de Baja</span>
							</button>
						{/if}
					</div>

					{#if cdrError}
						<div
							class="mt-2 rounded border border-red-500/30 bg-red-950/30 px-2 py-1 text-xs text-red-300"
						>
							{cdrError}
						</div>
					{/if}
				</div>
			</div>

			<!-- Footer Modal -->
			<div class="mt-5 flex justify-end">
				<button
					type="button"
					onclick={onClose}
					class="rounded-sm border border-neutral-700 px-4 py-1.5 text-sm text-neutral-300 transition-colors hover:bg-neutral-800"
				>
					Cerrar
				</button>
			</div>
		</div>
	{/if}
</Modal>
