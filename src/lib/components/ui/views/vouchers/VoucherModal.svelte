<script lang="ts">
	import { get } from 'svelte/store';
	import { onMount, onDestroy } from 'svelte';
	import Modal from '$lib/components/core/primitives/Modal.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';
	import {
		IconFileText,
		IconFileCode,
		IconDownload,
		IconAlertTriangle,
		IconReceipt,
		IconFile,
		IconEye
	} from '@tabler/icons-svelte';
	import { salesStore } from '$lib/features/sales';
	import { icbperStore } from '$lib/features/settings';
	import { customersStore } from '$lib/features/customers';
	import { tenantStore } from '$lib/features/tenant';
	import { voucherConfigStore } from './voucherContext';
	import { buildVoucherData, type VoucherData, type VoucherFormat } from './voucherGenerator';
	import { openVoucherPreview } from './pdfGenerator';
	import type { PaperFormat } from './pdfTemplateConfig';
	import { voucherClient } from '$lib/features/vouchers';
	import { readFile } from '@tauri-apps/plugin-fs';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { configLocalClient } from '$lib/integrations/tauri/deviceConfig';

	let { isOpen = $bindable(false), onClose }: { isOpen: boolean; onClose: () => void } = $props();

	const formatos: { value: VoucherFormat; label: string; descripcion: string }[] = [
		{
			value: 'pdf',
			label: 'PDF',
			descripcion: 'Ticket / comprobante para imprimir o enviar al cliente'
		},
		{
			value: 'xml',
			label: 'XML SUNAT',
			descripcion: 'UBL 2.1 para la facturación electrónica'
		},
		{ value: 'ambos', label: 'Ambos', descripcion: 'Generar PDF y XML juntos' }
	];

	let formatoSeleccionado = $state<VoucherFormat>('pdf');
	let papelSeleccionado = $state<PaperFormat>('a4');
	let estado = $state<'idle' | 'preparando' | 'error'>('idle');
	let mensaje = $state('');
	let voucherData = $state<VoucherData | null>(null);
	const sinItems = $derived($salesStore.length === 0);
	const sinRucEnFactura = $derived(
		voucherData?.tipoComprobante === '01' &&
			(!voucherData.cliente ||
				(voucherData.cliente.tipoDocumento !== '6' &&
					voucherData.cliente.tipoDocumento.toUpperCase() !== 'RUC'))
	);

	// Escuchar el evento que emite la ventana de preview cuando el usuario confirma.
	// Los stores se limpian SOLO si el usuario realmente pulsó "Confirmar y Generar".
	let unlistenConfirmed: UnlistenFn | null = null;
	onMount(async () => {
		unlistenConfirmed = await listen('taxorium:voucher-confirmed', () => {
			salesStore.clear();
			voucherConfigStore.reset();
		});
	});
	onDestroy(() => {
		unlistenConfirmed?.();
	});

	$effect(() => {
		if (isOpen && !voucherData) {
			estado = 'idle';
			mensaje = '';
			const config = get(voucherConfigStore);
			const cliente =
				get(customersStore).find((c) => String(c.id) === String(config.clienteId)) ?? null;
			const tenant = get(tenantStore).tenant;
			const items = get(salesStore);
			const serie = config.serie || (config.tipoComprobante === '03' ? 'B001' : 'F001');

			const tasaIcbper = get(icbperStore);
			let activo = true;
			voucherClient
				.getNextCorrelativo(serie)
				.then((n) => {
					if (!activo) return;
					voucherData = buildVoucherData(config, tenant, items, cliente, n, tasaIcbper);
				})
				.catch(() => {
					if (!activo) return;
					voucherData = buildVoucherData(config, tenant, items, cliente, undefined, tasaIcbper);
				});

			return () => {
				activo = false;
			};
		} else if (!isOpen) {
			voucherData = null;
		}
	});

	/**
	 * Valida el certificado, construye el payload de registro y abre la ventana
	 * de previsualización SIN guardar nada en la base de datos todavía.
	 *
	 * El registro real ocurre solo si el usuario pulsa "Confirmar y Generar"
	 * dentro de la ventana de previsualización.
	 */
	async function abrirPreview() {
		if (!voucherData || estado === 'preparando') return;
		estado = 'preparando';
		mensaje = '';

		try {
			const config = get(voucherConfigStore);
			const tenant = get(tenantStore).tenant;

			// Leer ruta y contraseña del certificado desde SQLite local (sin internet)
			let certPath: string;
			let p12_password: string;
			try {
				const cfg = await configLocalClient.getAll();
				certPath = cfg.certificado_path ?? tenant?.certificado_path ?? '';
				p12_password = cfg.clave_cert ?? localStorage.getItem('taxorium_cert_pwd') ?? '';
			} catch {
				// Fallback si SQLite falla
				certPath = tenant?.certificado_path ?? '';
				p12_password = localStorage.getItem('taxorium_cert_pwd') ?? '';
			}

			if (!certPath) {
				throw new Error('La empresa no tiene un certificado configurado.');
			}

			// Verificar que el certificado sea legible ANTES de abrir el preview
			try {
				await readFile(certPath);
			} catch {
				throw new Error('No se pudo leer el archivo de certificado (.p12). Verifica la ruta.');
			}

			// Los bytes del certificado NO se incluyen en la URL para no inflarla;
			// la ventana de preview los lee desde certificado_path al confirmar.
			const registration = {
				fecha_de_emision: `${voucherData.fechaEmision} ${voucherData.horaEmision}`,
				cliente: voucherData.cliente?.nombre ?? 'Clientes Varios',
				numero_comprobante: voucherData.numeroCompleto,
				serie: voucherData.serie,
				correlativo: voucherData.correlativo,
				tipo_comprobante: voucherData.tipoComprobante,
				moneda: voucherData.moneda,
				estado_pago: voucherData.estadoPago,
				emisor_ruc: tenant!.ruc,
				emisor_razon_social: tenant!.razon_social,
				emisor_ubigeo: tenant!.ubigeo || '',
				emisor_direccion: tenant!.direccion,
				receptor_tipo_doc: voucherData.cliente?.tipoDocumento || '0',
				receptor_num_doc: voucherData.cliente?.numeroDocumento || '0',
				tipo_operacion: config.tipoOperacion || '0101',
				certificado_path: certPath,
				p12_password,
				tasa_icbper: get(icbperStore),
				// El precio ya incluye IGV; el backend desglosa base + IGV por afectación.
				items: voucherData.items.map((item) => ({
					unidad: item.unidad,
					cantidad: item.cantidad,
					precio_unitario: item.precioUnitario,
					total: item.total,
					afectacion: item.afectacion,
					tiene_icbper: item.tieneIcbper,
					descripcion: item.descripcion
				}))
			};

			openVoucherPreview(voucherData, registration, papelSeleccionado);
			onClose(); // Cerrar modal — el usuario continúa en la ventana de preview
		} catch (e) {
			estado = 'error';
			mensaje = e instanceof Error ? e.message : String(e);
		}
	}
</script>

<Modal bind:isOpen {onClose} title="Tipo de formato">
	<div class="grid w-140 max-w-full gap-4 p-4">
		<!-- Elección de formato -->
		<div>
			<div class="grid grid-cols-3 gap-2">
				{#each formatos as formato (formato.value)}
					<button
						type="button"
						class="flex flex-col items-start gap-1 rounded-sm border p-3 text-left transition-colors {formatoSeleccionado ===
						formato.value
							? 'border-blue-500 bg-blue-900/30 text-white'
							: 'border-neutral-800 bg-neutral-900 text-neutral-400 hover:border-neutral-700'}"
						onclick={() => (formatoSeleccionado = formato.value)}
					>
						<span class="flex items-center gap-2">
							{#if formato.value === 'pdf'}
								<IconFileText size={18} class="text-blue-400" />
							{:else if formato.value === 'xml'}
								<IconFileCode size={18} class="text-blue-400" />
							{:else}
								<IconDownload size={18} class="text-blue-400" />
							{/if}
							<span class="font-medium">{formato.label}</span>
						</span>
						<span class="text-xs">{formato.descripcion}</span>
					</button>
				{/each}
			</div>
		</div>

		<!-- Tamaño de papel (solo cuando se elige PDF) -->
		{#if formatoSeleccionado === 'pdf' || formatoSeleccionado === 'ambos'}
			<div>
				<span class="mb-2 block text-sm font-medium text-neutral-400">Tamaño de papel</span>
				<div class="grid grid-cols-2 gap-2">
					<button
						type="button"
						class="flex items-center gap-2 rounded-sm border p-2.5 text-left text-sm transition-colors {papelSeleccionado ===
						'a4'
							? 'border-blue-500 bg-blue-900/30 text-white'
							: 'border-neutral-800 bg-neutral-900 text-neutral-400 hover:border-neutral-700'}"
						onclick={() => (papelSeleccionado = 'a4')}
					>
						<IconFile size={16} class="text-blue-400" />
						<span>
							<span class="block font-medium">A4</span>
							<span class="text-xs opacity-70">Factura/boleta oficial</span>
						</span>
					</button>
					<button
						type="button"
						class="flex items-center gap-2 rounded-sm border p-2.5 text-left text-sm transition-colors {papelSeleccionado ===
						'ticket80mm'
							? 'border-blue-500 bg-blue-900/30 text-white'
							: 'border-neutral-800 bg-neutral-900 text-neutral-400 hover:border-neutral-700'}"
						onclick={() => (papelSeleccionado = 'ticket80mm')}
					>
						<IconReceipt size={16} class="text-blue-400" />
						<span>
							<span class="block font-medium">Ticket 80mm</span>
							<span class="text-xs opacity-70">Impresora térmica</span>
						</span>
					</button>
				</div>
			</div>
		{/if}

		<!-- Advertencias -->
		{#if sinItems}
			<div class="flex items-center gap-2 text-sm text-amber-400">
				<IconAlertTriangle size={16} />
				<span>No hay productos en la venta. Agrega al menos un ítem antes de continuar.</span>
			</div>
		{/if}

		{#if sinRucEnFactura}
			<div class="flex items-center gap-2 text-sm text-red-400">
				<IconAlertTriangle size={16} class="shrink-0" />
				<span
					>Para emitir una Factura Electrónica es obligatorio seleccionar un cliente con RUC válido.</span
				>
			</div>
		{/if}

		{#if estado === 'error'}
			<div class="flex items-center gap-2 text-sm text-red-400">
				<IconAlertTriangle size={16} />
				<span>{mensaje}</span>
			</div>
		{/if}

		<!-- Acciones -->
		<div class="flex items-center justify-end gap-2 border-t border-neutral-800 pt-3">
			<Button variant="outline" onclick={onClose}>
				{#snippet children()}
					Cancelar
				{/snippet}
			</Button>
			<Button
				variant="primary"
				onclick={abrirPreview}
				disabled={estado === 'preparando' || sinItems || sinRucEnFactura}
			>
				{#snippet children()}
					<IconEye size={16} />
					{estado === 'preparando' ? 'Preparando...' : 'Ver previsualización'}
				{/snippet}
			</Button>
		</div>
	</div>
</Modal>
