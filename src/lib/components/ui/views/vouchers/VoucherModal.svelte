<script lang="ts">
	import { get } from 'svelte/store';
	import Modal from '$lib/components/core/primitives/Modal.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';
	import {
		IconFileText,
		IconFileCode,
		IconDownload,
		IconCircleCheck,
		IconAlertTriangle
	} from '@tabler/icons-svelte';
	import { salesStore } from '$lib/stores/sales';
	import { customersStore } from '$lib/stores/customers';
	import { tenantStore } from '$lib/stores/tenant';
	import { voucherConfigStore } from './voucherContext';
	import {
		buildVoucherData,
		generateVoucherPdf,
		generateVoucherXml,
		type VoucherData,
		type VoucherFormat
	} from './voucherGenerator';
	import { saveVoucherFile } from './voucherFile';
	import { voucherClient } from '$lib/services/vouchers/clientVoucher';

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
	let estado = $state<'idle' | 'generando' | 'exito' | 'error'>('idle');
	let mensaje = $state('');
	let archivosGuardados = $state<string[]>([]);
	let voucherData = $state<VoucherData | null>(null);
	const sinItems = $derived($salesStore.length === 0);

	$effect(() => {
		if (isOpen && !voucherData) {
			estado = 'idle';
			mensaje = '';
			archivosGuardados = [];
			const config = get(voucherConfigStore);
			const cliente =
				get(customersStore).find((c) => String(c.id) === String(config.clienteId)) ?? null;
			const tenant = get(tenantStore).tenant;
			const items = get(salesStore);
			const serie = config.serie || (config.tipoComprobante === '03' ? 'B001' : 'F001');

			let activo = true;
			voucherClient
				.getNextCorrelativo(serie)
				.then((n) => {
					if (!activo) return;
					voucherData = buildVoucherData(config, tenant, items, cliente, n);
				})
				.catch(() => {
					// Si la base de datos no responde, usa el correlativo de sesión.
					if (!activo) return;
					voucherData = buildVoucherData(config, tenant, items, cliente);
				});

			return () => {
				activo = false;
			};
		} else if (!isOpen) {
			voucherData = null;
		}
	});

	async function generar() {
		if (!voucherData) return;
		estado = 'generando';
		mensaje = '';

		try {
			await voucherClient.createVoucher({
				fecha_de_emision: `${voucherData.fechaEmision} ${voucherData.horaEmision}`,
				cliente: voucherData.cliente?.nombre ?? 'Clientes Varios',
				numero_comprobante: voucherData.numeroCompleto,
				serie: voucherData.serie,
				correlativo: voucherData.correlativo,
				tipo_comprobante: voucherData.tipoComprobante,
				moneda: voucherData.moneda,
				gravado: voucherData.opGravadas + voucherData.opExoneradas,
				igv: voucherData.igv,
				total: voucherData.total,
				estado_pago: voucherData.montoPagado > 0 ? 'pagado' : 'pendiente'
			});
		} catch (e) {
			estado = 'error';
			mensaje = `No se pudo registrar el comprobante: ${e instanceof Error ? e.message : String(e)}`;
			return;
		}

		const guardados: string[] = [];
		if (formatoSeleccionado === 'pdf' || formatoSeleccionado === 'ambos') {
			const pdf = generateVoucherPdf(voucherData);
			const ruta = await saveVoucherFile(pdf, `${voucherData.numeroCompleto}.pdf`, 'pdf');
			if (ruta) guardados.push(`${voucherData.numeroCompleto}.pdf`);
		}

		if (formatoSeleccionado === 'xml' || formatoSeleccionado === 'ambos') {
			const xml = generateVoucherXml(voucherData);
			const ruta = await saveVoucherFile(xml, `${voucherData.numeroCompleto}.xml`, 'xml');
			if (ruta) guardados.push(`${voucherData.numeroCompleto}.xml`);
		}

		archivosGuardados = guardados;
		estado = 'exito';
		salesStore.clear();
		voucherConfigStore.reset();
	}
</script>

<Modal bind:isOpen {onClose} title="Generar Comprobante">
	<div class="grid w-140 max-w-full gap-4 p-4">
		{#if estado === 'exito'}
			<div class="flex flex-col gap-3">
				<div class="flex items-center gap-2 text-emerald-400">
					<IconCircleCheck size={20} />
					<span class="font-medium">Comprobante registrado correctamente</span>
				</div>
				{#if archivosGuardados.length > 0}
					<ul class="list-inside list-disc text-sm text-neutral-400">
						{#each archivosGuardados as archivo (archivo)}
							<li>{archivo}</li>
						{/each}
					</ul>
				{:else}
					<p class="text-sm text-neutral-400">
						Quedó registrado en la base de datos. No se guardó ningún archivo.
					</p>
				{/if}
				<div class="flex justify-end gap-2">
					<Button variant="primary" onclick={onClose}>
						{#snippet children()}
							Listo
						{/snippet}
					</Button>
				</div>
			</div>
		{:else}
			<!-- Resumen del comprobante -->
			<div
				class="flex flex-col gap-1 rounded-sm border border-neutral-800 bg-neutral-900 p-3 text-sm"
			>
				<div class="flex items-center justify-between">
					<span class="text-neutral-400">{voucherData?.tipoComprobanteLabel}</span>
					<span class="font-mono text-blue-400">{voucherData?.numeroCompleto}</span>
				</div>
				<div class="flex items-center justify-between">
					<span class="text-neutral-400">Fecha de Emisión</span>
					<span>{voucherData?.fechaEmision} {voucherData?.horaEmision}</span>
				</div>
				<div class="flex items-center justify-between">
					<span class="text-neutral-400">Total a Pagar</span>
					<span class="font-medium"
						>{voucherData?.monedaSimbolo} {voucherData?.total.toFixed(2)}</span
					>
				</div>
			</div>

			<!-- Elección de formato -->
			<div>
				<span class="mb-2 block text-sm font-medium text-neutral-400">
					¿En qué formato deseas generar el comprobante?
				</span>
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

			{#if sinItems}
				<div class="flex items-center gap-2 text-sm text-amber-400">
					<IconAlertTriangle size={16} />
					<span>No hay productos en la venta. Agrega al menos un ítem antes de generar.</span>
				</div>
			{/if}

			{#if voucherData?.tipoComprobante === '01' && (!voucherData.cliente || (voucherData.cliente.tipoDocumento !== '6' && voucherData.cliente.tipoDocumento.toUpperCase() !== 'RUC'))}
				<div class="flex items-center gap-2 text-sm text-red-400">
					<IconAlertTriangle size={16} class="shrink-0" />
					<span>Para emitir una Factura Electrónica es obligatorio seleccionar un cliente con RUC válido.</span>
				</div>
			{/if}

			{#if estado === 'error'}
				<div class="flex items-center gap-2 text-sm text-red-400">
					<IconAlertTriangle size={16} />
					<span>{mensaje}</span>
				</div>
			{/if}

			<div class="flex items-center justify-end gap-2 border-t border-neutral-800 pt-3">
				<Button variant="outline" onclick={onClose}>
					{#snippet children()}
						Cerrar
					{/snippet}
				</Button>
				<Button 
					variant="primary" 
					onclick={generar} 
					disabled={estado === 'generando' || sinItems || (voucherData?.tipoComprobante === '01' && (!voucherData.cliente || (voucherData.cliente.tipoDocumento !== '6' && voucherData.cliente.tipoDocumento.toUpperCase() !== 'RUC')))}
				>
					{#snippet children()}
						{#if estado === 'generando'}Generando...{:else}Generar{/if}
					{/snippet}
				</Button>
			</div>
		{/if}
	</div>
</Modal>
