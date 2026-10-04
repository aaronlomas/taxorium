<script lang="ts">
	import { onMount } from 'svelte';
	import InvoiceTemplate from '$lib/components/ui/print/InvoiceTemplate.svelte';
	import type { PrintableInvoiceData } from '$lib/components/ui/print/invoiceTypes';
	import type { PrintWindowPayload } from '$lib/components/ui/views/vouchers/pdfGenerator';
	import { voucherClient } from '$lib/services/vouchers/clientVoucher';
	import { readFile } from '@tauri-apps/plugin-fs';
	import { emit } from '@tauri-apps/api/event';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	let data = $state<PrintableInvoiceData | null>(null);
	let registration = $state<PrintWindowPayload['registration'] | null>(null);
	let paperFormat = $state<'a4' | 'ticket80mm'>('a4');
	let viewOnly = $state(false);
	let error = $state<string | null>(null);

	// Estado del proceso de confirmación
	let confirmState = $state<'idle' | 'registrando' | 'exito' | 'error'>('idle');
	let confirmError = $state<string | null>(null);

	onMount(() => {
		const params = new URLSearchParams(window.location.search);
		const raw = params.get('d');
		if (!raw) {
			error = 'No se recibieron datos del comprobante.';
			return;
		}
		try {
			const payload = JSON.parse(decodeURIComponent(raw)) as PrintWindowPayload;
			data = payload.printable;
			registration = payload.registration ?? null;
			paperFormat = payload.paperFormat ?? 'a4';
			viewOnly = payload.viewOnly ?? false;

			// Inyectar @page según formato para que el PDF tenga el tamaño correcto.
			const style = document.createElement('style');
			style.textContent =
				paperFormat === 'ticket80mm'
					? `@page { size: 80mm auto; margin: 0; }`
					: `@page { size: A4 portrait; margin: 10mm; }`;
			document.head.appendChild(style);
		} catch {
			error = 'No se pudieron leer los datos del comprobante.';
		}
	});

	/** Registra el comprobante en la BD y abre el diálogo de impresión. */
	async function confirmarYGenerar() {
		if (!registration || confirmState === 'registrando') return;
		confirmState = 'registrando';
		confirmError = null;

		try {
			// Leer los bytes del certificado desde la ruta (el path viene en el payload).
			let p12_bytes: number[];
			try {
				const bytes = await readFile(registration.certificado_path);
				p12_bytes = Array.from(bytes);
			} catch {
				throw new Error('No se pudo leer el certificado digital (.p12). Verifica la ruta en configuración.');
			}

			// Registrar en la base de datos
			await voucherClient.createVoucher({
				fecha_de_emision: registration.fecha_de_emision,
				cliente: registration.cliente,
				numero_comprobante: registration.numero_comprobante,
				serie: registration.serie,
				correlativo: registration.correlativo,
				tipo_comprobante: registration.tipo_comprobante,
				moneda: registration.moneda,
				estado_pago: registration.estado_pago,
				emisor_ruc: registration.emisor_ruc,
				emisor_razon_social: registration.emisor_razon_social,
				emisor_ubigeo: registration.emisor_ubigeo,
				emisor_direccion: registration.emisor_direccion,
				receptor_tipo_doc: registration.receptor_tipo_doc,
				receptor_num_doc: registration.receptor_num_doc,
				tipo_operacion: registration.tipo_operacion,
				p12_bytes,
				p12_password: registration.p12_password,
				items: registration.items
			});

			confirmState = 'exito';

			// Notificar a la ventana principal para que limpie los stores.
			await emit('taxorium:voucher-confirmed', { numero: registration.numero_comprobante });

			// Abrir el diálogo de impresión nativo del sistema.
			window.print();
		} catch (e) {
			confirmState = 'error';
			confirmError = e instanceof Error ? e.message : String(e);
		}
	}

	async function cerrar() {
		await getCurrentWindow().close();
	}
</script>

<svelte:head>
	<title>Vista previa — {data?.serie_correlativo ?? 'Comprobante'}</title>
	<style>
		*,
		*::before,
		*::after {
			box-sizing: border-box;
		}

		body {
			margin: 0;
			padding: 0;
			background: #0f0f17;
			font-family: 'Inter', system-ui, sans-serif;
		}

		/* ── Barra de herramientas ─────────────────────────────────── */
		.toolbar {
			position: sticky;
			top: 0;
			z-index: 50;
			display: flex;
			align-items: center;
			justify-content: space-between;
			gap: 12px;
			padding: 10px 16px;
			background: #12121e;
			border-bottom: 1px solid #252540;
			box-shadow: 0 2px 12px rgba(0, 0, 0, 0.5);
		}

		.toolbar-brand {
			display: flex;
			align-items: center;
			gap: 8px;
			color: #a78bfa;
			font-size: 13px;
			font-weight: 600;
			letter-spacing: 0.04em;
		}

		.toolbar-actions {
			display: flex;
			align-items: center;
			gap: 8px;
		}

		/* Botón base */
		.btn {
			display: inline-flex;
			align-items: center;
			gap: 6px;
			padding: 7px 16px;
			font-size: 13px;
			font-weight: 600;
			border: none;
			border-radius: 8px;
			cursor: pointer;
			transition: opacity 0.15s, transform 0.1s;
			white-space: nowrap;
		}
		.btn:active { transform: scale(0.97); }
		.btn:disabled { opacity: 0.5; cursor: not-allowed; }

		.btn-confirm {
			background: linear-gradient(135deg, #7c3aed, #4f46e5);
			color: white;
		}
		.btn-confirm:hover:not(:disabled) { opacity: 0.88; }

		.btn-cancel {
			background: transparent;
			color: #9ca3af;
			border: 1px solid #374151;
		}
		.btn-cancel:hover:not(:disabled) { background: #1f2937; color: #d1d5db; }

		/* Estado de carga */
		.spinner {
			width: 14px;
			height: 14px;
			border: 2px solid rgba(255,255,255,0.3);
			border-top-color: white;
			border-radius: 50%;
			animation: spin 0.7s linear infinite;
			display: inline-block;
		}
		@keyframes spin { to { transform: rotate(360deg); } }

		/* Banner de error */
		.error-banner {
			margin: 0 16px;
			padding: 8px 12px;
			background: rgba(239, 68, 68, 0.12);
			border: 1px solid rgba(239, 68, 68, 0.4);
			border-radius: 6px;
			color: #fca5a5;
			font-size: 12px;
		}

		/* Banner de éxito */
		.success-banner {
			margin: 0 16px;
			padding: 8px 12px;
			background: rgba(52, 211, 153, 0.12);
			border: 1px solid rgba(52, 211, 153, 0.4);
			border-radius: 6px;
			color: #6ee7b7;
			font-size: 12px;
		}

		/* Área del comprobante */
		.preview-area {
			padding: 24px 16px;
			min-height: calc(100vh - 56px);
		}

		/* Al imprimir: ocultar toolbar, solo el comprobante */
		@media print {
			.toolbar,
			.error-banner,
			.success-banner {
				display: none !important;
			}
			body {
				background: white !important;
			}
			.preview-area {
				padding: 0 !important;
			}
		}
	</style>
</svelte:head>

<!-- Barra de herramientas -->
<div class="toolbar">
	<span class="toolbar-brand">
		⚡ {viewOnly ? 'Comprobante' : 'Vista previa'} — {data?.serie_correlativo ?? '…'}
	</span>

	<div class="toolbar-actions">
		{#if viewOnly}
			<!-- Modo solo lectura: comprobante ya registrado -->
			<button class="btn btn-cancel" onclick={cerrar}>✕ Cerrar</button>
			<button class="btn btn-confirm" onclick={() => window.print()}>
				🖨️ Imprimir / Guardar PDF
			</button>
		{:else if confirmState === 'exito'}
			<span class="success-banner" style="margin:0;">✓ Registrado correctamente</span>
			<button class="btn btn-cancel" onclick={cerrar}>Cerrar</button>
		{:else}
			<button
				class="btn btn-cancel"
				onclick={cerrar}
				disabled={confirmState === 'registrando'}
			>
				✕ Cancelar
			</button>
			<button
				class="btn btn-confirm"
				onclick={confirmarYGenerar}
				disabled={!registration || confirmState === 'registrando'}
			>
				{#if confirmState === 'registrando'}
					<span class="spinner"></span>
					Registrando…
				{:else}
					🖨️ Confirmar y Generar
				{/if}
			</button>
		{/if}
	</div>
</div>

<!-- Mensajes de error -->
{#if confirmState === 'error' && confirmError}
	<div class="error-banner" style="margin-top: 12px;">
		⚠ {confirmError}
	</div>
{/if}
{#if error}
	<div class="error-banner" style="margin-top: 12px;">
		⚠ {error}
	</div>
{/if}

<!-- Comprobante -->
<div class="preview-area">
	{#if data}
		<InvoiceTemplate {data} />
	{:else if !error}
		<p style="text-align:center; color:#6b7280; padding:2rem;">Cargando comprobante…</p>
	{/if}
</div>
