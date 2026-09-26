<script lang="ts">
	import { onMount } from 'svelte';
	import { IconPlus, IconDatabaseImport, IconDatabaseExport, IconEye } from '@tabler/icons-svelte';
	import TableFilter, { type CheckItem } from '$lib/components/core/primitives/TableFilter.svelte';
	import TableToolbar from '$lib/components/core/primitives/TableToolbar.svelte';
	import Select from '$lib/components/core/primitives/Select.svelte';
	import Search from '$lib/components/core/primitives/Search.svelte';
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/Table.svelte';

	import { exportData, type ExportFormat } from '$lib/utilities/formats/export';
	import { taxoLog } from '$lib/stores/taxoLog';
	import { voucherClient, type Voucher } from '$lib/services/vouchers/clientVoucher';
	import { formatFecha } from '$lib/components/ui/views/vouchers/voucherContext';

	interface ColumnConfig extends CheckItem {
		key: keyof Voucher;
	}

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

	const SEARCH_OPTIONS = [
		{ value: '', label: 'Todos' },
		{ value: 'cliente', label: 'Cliente' },
		{ value: 'numero_comprobante', label: 'Número' },
		{ value: 'estado', label: 'Estado' }
	];

	onMount(async () => {
		try {
			vouchers = await voucherClient.getVouchers();
		} catch (e) {
			const msg =
				e instanceof Error ? e.message : typeof e === 'string' ? e : JSON.stringify(e);
			taxoLog.error(`Error al cargar los comprobantes: ${msg}`, 'comprobantes');
		} finally {
			isLoading = false;
		}
	});

	let vouchersFiltrados = $derived(
		vouchers.filter((v) => {
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
				EUR: 'Euros (€)'
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
			default:
				return 'text-amber-400';
		}
	}

	function estadoPagoClass(estado: Voucher['estado_pago']): string {
		return estado === 'pagado' ? 'text-emerald-400' : 'text-amber-400';
	}
</script>

<div class="grid h-full grid-rows-[auto_1fr] gap-2 px-2 pb-2 text-sm">
	<!-- PRIMITIVA TOOLBAR -->
	<TableToolbar>
		{#snippet actions()}
			<button
				class="flex cursor-pointer gap-2 rounded-xl bg-neutral-800 px-3 py-1 hover:bg-neutral-700"
			>
				<IconPlus size={20} />Añadir
			</button>
			<button
				class="flex cursor-pointer gap-2 rounded-xl bg-neutral-800 px-3 py-1 hover:bg-neutral-700"
			>
				<IconDatabaseImport size={20} />Importar
			</button>
			<div class="group relative">
				<button
					class="flex cursor-pointer items-center gap-2 rounded-xl bg-neutral-800 px-3 py-1 hover:bg-neutral-700"
				>
					<IconDatabaseExport size={20} />Exportar
				</button>
				<div
					class="absolute top-full z-10 hidden w-48 flex-col overflow-hidden rounded-md border border-neutral-700 bg-neutral-800 shadow-lg group-hover:flex"
				>
					<button
						class="px-4 py-2 text-left text-sm hover:bg-neutral-700 hover:text-white"
						onclick={() => handleExportFile('xlsx')}>Excel (.xlsx)</button
					>
					<button
						class="px-4 py-2 text-left text-sm hover:bg-neutral-700 hover:text-white"
						onclick={() => handleExportFile('csv-comma')}>CSV (comas)</button
					>
					<button
						class="px-4 py-2 text-left text-sm hover:bg-neutral-700 hover:text-white"
						onclick={() => handleExportFile('csv-semicolon')}>CSV (puntos y comas)</button
					>
				</div>
			</div>
			<TableFilter label="Columnas" storageKey="comprobantes" bind:items={columnas} />
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
			<tr class="text-blue-400">
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
						No hay comprobantes registrados.
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
							<div class="flex items-center justify-center gap-2">
								<button
									class="flex cursor-pointer items-center gap-1 rounded-sm border border-blue-500/40 px-2 py-0.5 text-blue-400 transition-colors hover:bg-blue-500/10"
									onclick={() => handleConsultar(voucher)}
									title="Consultar comprobante"
								>
									<IconEye size={14} />
									Consultar
								</button>
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
