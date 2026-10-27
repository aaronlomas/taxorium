<script lang="ts">
	import { onMount } from 'svelte';
	// ICONOS
	import {
		IconDatabaseImport,
		IconDatabaseExport,
		IconEye,
		IconSend,
		IconDownload,
		IconBan
	} from '@tabler/icons-svelte';
	// PRIMITIVAS
	import Hint from '$lib/components/core/primitives/data/Hint.svelte';
	import TableFilter, {
		type CheckItem
	} from '$lib/components/core/primitives/data/TableFilter.svelte';
	import TableToolbar from '$lib/components/core/primitives/data/TableToolbar.svelte';
	import Select from '$lib/components/core/primitives/forms/Select.svelte';
	import Search from '$lib/components/core/primitives/data/Search.svelte';
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/data/Table.svelte';
	// MODALES MODULARIZADOS
	import VoucherDownloadModal from './VoucherDownloadModal.svelte';
	import VoucherAnularModal from './VoucherAnularModal.svelte';
	// SERVICIOS
	import { invoke } from '@tauri-apps/api/core';
	import { get } from 'svelte/store';
	import { tenantStore } from '$lib/features/tenant';
	import { exportData, type ExportFormat } from '$lib/utilities/formats/export';
	import { taxoLog } from '$lib/features/taxoLog';
	import { voucherClient, type Voucher } from '$lib/features/vouchers';
	import { formatFecha } from '$lib/components/ui/features/sales/vouchers/state/voucherContext';

	interface ColumnConfig extends CheckItem {
		key: keyof Voucher;
	}

	let { tipoComprobante = 'todos' }: { tipoComprobante?: 'todos' | 'boleta' | 'factura' } =
		$props();

	let columnas = $state<ColumnConfig[]>([
		{ id: 'fecha_de_emision', key: 'fecha_de_emision', label: 'Fecha de Emisión', checked: true },
		{ id: 'cliente', key: 'cliente', label: 'Cliente', checked: true },
		{ id: 'numero_comprobante', key: 'numero_comprobante', label: 'Número', checked: true },
		{ id: 'estado_validez', key: 'estado_validez', label: 'Estado', checked: true },
		{ id: 'estado_pago', key: 'estado_pago', label: 'Estado de Pago', checked: true },
		{ id: 'moneda', key: 'moneda', label: 'Moneda', checked: true },
		{ id: 'gravado', key: 'gravado', label: 'Gravado', checked: true },
		{ id: 'igv', key: 'igv', label: 'IGV', checked: true },
		{ id: 'total', key: 'total', label: 'Total', checked: true }
	]);

	let columnasVisibles = $derived(columnas.filter((c) => c.checked));

	let vouchers = $state<Voucher[]>([]);
	let isLoading = $state(true);
	let searchTerm = $state('');
	let searchBy = $state('');

	// Modales
	let isDownloadOpen = $state(false);
	let downloadVoucher = $state<Voucher | null>(null);

	let isAnularOpen = $state(false);
	let anularVoucher = $state<Voucher | null>(null);

	let isEmitting = $state<number | null>(null);

	const SEARCH_OPTIONS = [
		{ value: '', label: 'Todos' },
		{ value: 'cliente', label: 'Cliente' },
		{ value: 'numero_comprobante', label: 'Número' },
		{ value: 'estado', label: 'Estado' }
	];

	async function reloadVouchers() {
		try {
			vouchers = await voucherClient.getVouchers();
		} catch (e) {
			const msg = e instanceof Error ? e.message : typeof e === 'string' ? e : JSON.stringify(e);
			taxoLog.error(`Error al cargar los comprobantes: ${msg}`, 'comprobantes');
		}
	}

	onMount(async () => {
		try {
			await reloadVouchers();
		} finally {
			isLoading = false;
		}
	});

	let vouchersFiltrados = $derived(
		vouchers.filter((v) => {
			if (tipoComprobante !== 'todos') {
				const esFactura = v.numero_comprobante?.toUpperCase().startsWith('F') ?? false;
				if (tipoComprobante === 'factura' && !esFactura) return false;
				if (tipoComprobante === 'boleta' && esFactura) return false;
			}

			if (!searchTerm.trim()) return true;
			const term = searchTerm.toLowerCase();

			if (searchBy === 'cliente') {
				return v.cliente.toLowerCase().includes(term);
			} else if (searchBy === 'numero_comprobante') {
				return v.numero_comprobante.toLowerCase().includes(term);
			} else if (searchBy === 'estado') {
				return (
					v.estado_validez.toLowerCase().includes(term) ||
					v.estado_pago.toLowerCase().includes(term)
				);
			}

			return (
				v.cliente.toLowerCase().includes(term) ||
				v.numero_comprobante.toLowerCase().includes(term) ||
				v.estado_validez.toLowerCase().includes(term) ||
				v.estado_pago.toLowerCase().includes(term)
			);
		})
	);

	async function handleExportFile(format: ExportFormat) {
		const savedPath = await exportData(vouchersFiltrados, columnasVisibles, 'comprobantes', format);
		if (savedPath) {
			taxoLog.info(`Exportado exitosamente en: ${savedPath}`, 'comprobantes');
		}
	}

	function handleConsultar(voucher: Voucher) {
		taxoLog.info(`Consultando comprobante ${voucher.numero_comprobante}`, 'comprobantes');
	}

	function handleAbrirDescargas(voucher: Voucher) {
		downloadVoucher = voucher;
		isDownloadOpen = true;
	}

	function handleAbrirAnular(voucher: Voucher) {
		anularVoucher = voucher;
		isAnularOpen = true;
	}

	async function handleEmitir(voucher: Voucher) {
		if (voucher.estado_sunat !== 0) {
			taxoLog.warn('Este comprobante ya fue emitido a SUNAT.', 'comprobantes');
			return;
		}

		const tenant = get(tenantStore).tenant;
		if (!tenant || !tenant.usuario_sol || !tenant.clave_sol || !tenant.ruc) {
			taxoLog.error('Faltan configurar las credenciales SOL de la empresa.', 'comprobantes');
			return;
		}

		isEmitting = voucher.id;
		taxoLog.info(`Enviando comprobante ${voucher.numero_comprobante} a SUNAT...`, 'comprobantes');

		try {
			const res = await invoke('enviar_a_sunat', {
				payload: {
					ruc: tenant.ruc,
					usuario_sol: tenant.usuario_sol,
					clave_sol: tenant.clave_sol,
					client_id: (tenant as any).sunat_client_id || '25f61db1-0854-4efb-bc54-c1f10cf8db17',
					client_secret: (tenant as any).sunat_client_secret || 'rvSu0MjYw+hB+vxONyA7jA==',
					ambiente: 'beta',
					voucher_id: voucher.id
				}
			});

			taxoLog.info(
				`Comprobante aceptado por SUNAT. CDR: ${(res as any).descripcion}`,
				'comprobantes'
			);

			await reloadVouchers();
		} catch (e) {
			taxoLog.error(`Error de SUNAT: ${e}`, 'comprobantes');
		} finally {
			isEmitting = null;
		}
	}

	const CURRENCY_SYMBOL: Record<string, string> = {
		PEN: 'S/.',
		USD: '$',
		EUR: '€'
	};

	function formatCellValue(voucher: Voucher, key: keyof Voucher): string {
		const val = voucher[key];
		if (val === null || val === undefined) return '-';
		if (key === 'gravado' || key === 'igv' || key === 'total') {
			const num = typeof val === 'number' ? val : parseFloat(String(val));
			const symbol = CURRENCY_SYMBOL[voucher.moneda ?? 'PEN'] ?? 'S/.';
			return `${symbol} ${num.toFixed(2)}`;
		}
		if (key === 'fecha_de_emision') {
			const s = String(val);
			return /^\d{2}\/\d{2}\/\d{4}/.test(s) ? s.slice(0, 10) : formatFecha(s);
		}
		if (key === 'moneda') {
			const labels: Record<string, string> = {
				PEN: 'Soles (S/.)',
				USD: 'Dólares ($)',
			};
			return labels[String(val)] ?? String(val);
		}
		return String(val);
	}

	function estadoValidezClass(estado: Voucher['estado_validez']): string {
		switch (estado) {
			case 'aceptado':
				return 'text-emerald-400';
			case 'rechazado':
				return 'text-red-400';
			case 'anulado':
				return 'text-rose-400 line-through';
			default:
				return 'text-amber-400';
		}
	}

	function estadoPagoClass(estado: Voucher['estado_pago']): string {
		return estado === 'pagado' ? 'text-emerald-400' : 'text-amber-400';
	}
</script>

<div class="grid h-full grid-rows-[auto_1fr] gap-1 text-sm">
	<!-- PRIMITIVA TOOLBAR -->
	<TableToolbar>
		{#snippet actions()}
			<button
				class="flex cursor-pointer gap-2 rounded-xl bg-brand-bg-secondary px-3 py-1 hover:bg-brand-bg-hover border border-brand-border"
			>
				<IconDatabaseImport size={20} />Importar
			</button>
			<div class="group relative">
				<button
					class="flex cursor-pointer gap-2 rounded-xl bg-brand-bg-secondary px-3 py-1 hover:bg-brand-bg-hover border border-brand-border"
				>
					<IconDatabaseExport size={20} />Exportar
				</button>
				<div
					class="absolute top-full z-10 hidden w-48 flex-col overflow-hidden rounded-md border border-brand-border bg-brand-bg-secondary shadow-lg group-hover:flex"
				>
					<button
						class="px-4 py-2 text-left text-sm hover:bg-brand-bg-hover"
						onclick={() => handleExportFile('xlsx')}>Excel (.xlsx)</button
					>
					<button
						class="px-4 py-2 text-left text-sm hover:bg-brand-bg-hover"
						onclick={() => handleExportFile('csv-comma')}>CSV (comas)</button
					>
					<button
						class="px-4 py-2 text-left text-sm hover:bg-brand-bg-hover"
						onclick={() => handleExportFile('csv-semicolon')}>CSV (puntos y comas)</button
					>
				</div>
			</div>
			<TableFilter storageKey="comprobantes" bind:items={columnas} />
		{/snippet}

		{#snippet filters()}
			<div class="w-30">
				<Select placeholder="Buscar por:" options={SEARCH_OPTIONS} bind:value={searchBy} />
			</div>
			<Search
				bind:value={searchTerm}
				placeholder={searchBy === 'cliente'
					? 'Buscar cliente...'
					: searchBy === 'numero_comprobante'
						? 'Buscar número...'
						: searchBy === 'estado'
							? 'Buscar estado...'
							: 'Buscar comprobante...'}
			/>
		{/snippet}
	</TableToolbar>

	<!-- TABLA DE COMPROBANTES -->
	<Table>
		{#snippet head()}
			<tr>
				{#snippet headIndex()}
					#
				{/snippet}
				{@render headCell({ children: headIndex })}
				{#each columnasVisibles as col (col.id)}
					{#snippet headCol()}
						{col.label}
					{/snippet}
					{@render headCell({ children: headCol })}
				{/each}
				{#snippet headActions()}
					Acciones
				{/snippet}
				{@render headCell({ children: headActions })}
			</tr>
		{/snippet}

		{#snippet body()}
			{#if isLoading}
				<tr>
					{#snippet loadingState()}
						Cargando comprobantes...
					{/snippet}
					{@render cell({
						colspan: columnasVisibles.length + 2,
						class: 'py-8 text-neutral-500',
						children: loadingState
					})}
				</tr>
			{:else if vouchersFiltrados.length === 0}
				<tr>
					{#snippet emptyState()}
						No hay {tipoComprobante === 'boleta'
							? 'boletas de venta'
							: tipoComprobante === 'factura'
								? 'facturas'
								: 'comprobantes'} registrados.
					{/snippet}
					{@render cell({
						colspan: columnasVisibles.length + 2,
						class: 'py-8 text-neutral-500',
						children: emptyState
					})}
				</tr>
			{:else}
				{#each vouchersFiltrados as voucher, index (voucher.id)}
					{#snippet rowData()}
						{#snippet cellIndex()}
							{index + 1}
						{/snippet}
						{@render cell({ class: 'px-3 py-2 text-neutral-500', children: cellIndex })}
						{#each columnasVisibles as col (col.id)}
							{#snippet cellVal()}
								{#if col.key === 'estado_validez'}
									<span class={estadoValidezClass(voucher.estado_validez)}>
										{voucher.estado_validez}
									</span>
								{:else if col.key === 'estado_pago'}
									<span class={estadoPagoClass(voucher.estado_pago)}>
										{voucher.estado_pago}
									</span>
								{:else}
									{formatCellValue(voucher, col.key)}
								{/if}
							{/snippet}
							{@render cell({ children: cellVal })}
						{/each}
						{#snippet cellActions()}
							<div class="flex items-center justify-center gap-1.5">
								{#if voucher.estado_sunat === 0}
									<Hint text="Emitir a SUNAT" side="top">
										<button
											class="flex cursor-pointer items-center justify-center rounded-sm border border-emerald-500/40 p-1.5 text-emerald-400 transition-colors hover:bg-emerald-500/10 disabled:opacity-50"
											onclick={() => handleEmitir(voucher)}
											disabled={isEmitting === voucher.id ||
												(isAnularOpen && anularVoucher?.id === voucher.id)}
										>
											{#if isEmitting === voucher.id}
												<span
													class="inline-block size-3.5 animate-spin rounded-full border-2 border-emerald-400 border-t-transparent"
												></span>
											{:else}
												<IconSend size={15} />
											{/if}
										</button>
									</Hint>
								{:else if (voucher.estado_sunat === 1 && voucher.estado_validez !== 'anulado') || (voucher.estado_sunat === 3 && voucher.codigo_cdr !== '0')}
									<Hint
										text={voucher.estado_sunat === 3
											? 'Declarar baja ante SUNAT'
											: 'Anular comprobante'}
										side="top"
									>
										<button
											class="flex cursor-pointer items-center justify-center rounded-sm border border-rose-500/40 p-1.5 text-rose-400 transition-colors hover:bg-rose-500/10 disabled:opacity-50"
											onclick={() => handleAbrirAnular(voucher)}
											disabled={isEmitting === voucher.id}
										>
											<IconBan size={15} />
										</button>
									</Hint>
								{/if}
								<Hint text="Consultar comprobante" side="top">
									<button
										class="flex cursor-pointer items-center justify-center rounded-sm border border-blue-500/40 p-1.5 text-blue-400 transition-colors hover:bg-blue-500/10 disabled:opacity-50"
										onclick={() => handleConsultar(voucher)}
										disabled={isEmitting === voucher.id}
									>
										<IconEye size={15} />
									</button>
								</Hint>
								<Hint text="Opciones de descarga (PDF, XML, CDR)" side="top">
									<button
										class="flex cursor-pointer items-center justify-center rounded-sm border border-purple-500/40 p-1.5 text-purple-400 transition-colors hover:bg-purple-500/10 disabled:opacity-50"
										onclick={() => handleAbrirDescargas(voucher)}
										disabled={isEmitting === voucher.id}
									>
										<IconDownload size={15} />
									</button>
								</Hint>
							</div>
						{/snippet}
						{@render cell({ children: cellActions })}
					{/snippet}
					{@render row({ children: rowData })}
				{/each}
			{/if}
		{/snippet}
	</Table>
</div>

<!-- Modal de Anulación -->
<VoucherAnularModal
	bind:isOpen={isAnularOpen}
	voucher={anularVoucher}
	onClose={() => (isAnularOpen = false)}
	onSuccess={reloadVouchers}
/>

<!-- Modal de Descargas (Tarea 2 de tasks.md: PDF A4/Ticket, XML, CDR) -->
<VoucherDownloadModal
	bind:isOpen={isDownloadOpen}
	voucher={downloadVoucher}
	onClose={() => (isDownloadOpen = false)}
/>
