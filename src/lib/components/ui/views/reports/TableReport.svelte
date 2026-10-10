<script lang="ts">
	import { IconDatabaseImport, IconDatabaseExport, IconEye, IconTrash } from '@tabler/icons-svelte';
	import TableFilter, { type CheckItem } from '$lib/components/core/primitives/TableFilter.svelte';
	import TableToolbar from '$lib/components/core/primitives/TableToolbar.svelte';
	import Select from '$lib/components/core/primitives/Select.svelte';
	import Search from '$lib/components/core/primitives/Search.svelte';
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/Table.svelte';

	import { exportData, type ExportFormat } from '$lib/utilities/formats/export';
	import { taxoLog } from '$lib/features/taxoLog';

	interface Reporte {
		id: number;
		periodo: string;
		tipo: 'venta' | 'compra';
		descripcion: string;
		documentos: number;
		moneda: string;
		total: number;
		estado: 'registrado' | 'procesado';
	}

	interface ColumnConfig extends CheckItem {
		key: keyof Reporte;
	}

	let columnas = $state<ColumnConfig[]>([
		{ id: 'periodo', key: 'periodo', label: 'Periodo', checked: true },
		{ id: 'tipo', key: 'tipo', label: 'Tipo', checked: true },
		{ id: 'descripcion', key: 'descripcion', label: 'Descripción', checked: true },
		{ id: 'documentos', key: 'documentos', label: 'Documentos', checked: true },
		{ id: 'moneda', key: 'moneda', label: 'Moneda', checked: true },
		{ id: 'total', key: 'total', label: 'Total', checked: true },
		{ id: 'estado', key: 'estado', label: 'Estado', checked: true }
	]);

	let columnasVisibles = $derived(columnas.filter((c) => c.checked));

	let reportes = $state<Reporte[]>([
		{
			id: 1,
			periodo: '08/2026',
			tipo: 'venta',
			descripcion: 'Ventas de agosto 2026',
			documentos: 12,
			moneda: 'PEN',
			total: 15420.5,
			estado: 'procesado'
		},
		{
			id: 2,
			periodo: '08/2026',
			tipo: 'compra',
			descripcion: 'Compras de agosto 2026',
			documentos: 5,
			moneda: 'PEN',
			total: 8320,
			estado: 'registrado'
		}
	]);

	let searchTerm = $state('');
	let searchBy = $state('');
	let tipoFiltro = $state('');

	const SEARCH_OPTIONS = [
		{ value: '', label: 'Todos' },
		{ value: 'descripcion', label: 'Descripción' },
		{ value: 'tipo', label: 'Tipo' },
		{ value: 'periodo', label: 'Periodo' },
		{ value: 'estado', label: 'Estado' }
	];

	const TIPO_OPTIONS = [
		{ value: '', label: 'Venta y Compra' },
		{ value: 'venta', label: 'Ventas' },
		{ value: 'compra', label: 'Compras' }
	];

	let reportesFiltrados = $derived(
		reportes.filter((r) => {
			if (tipoFiltro && r.tipo !== tipoFiltro) return false;

			if (!searchTerm.trim()) return true;
			const term = searchTerm.toLowerCase();

			if (searchBy === 'descripcion') {
				return r.descripcion.toLowerCase().includes(term);
			} else if (searchBy === 'tipo') {
				return r.tipo.toLowerCase().includes(term);
			} else if (searchBy === 'periodo') {
				return r.periodo.toLowerCase().includes(term);
			} else if (searchBy === 'estado') {
				return r.estado.toLowerCase().includes(term);
			}

			return (
				r.descripcion.toLowerCase().includes(term) ||
				r.tipo.toLowerCase().includes(term) ||
				r.periodo.toLowerCase().includes(term) ||
				r.estado.toLowerCase().includes(term)
			);
		})
	);

	async function handleExportFile(format: ExportFormat) {
		const savedPath = await exportData(reportesFiltrados, columnasVisibles, 'reportes', format);
		if (savedPath) {
			taxoLog.info(`Exportado exitosamente en: ${savedPath}`, 'reportes');
		}
	}

	function handleVer(reporte: Reporte) {
		taxoLog.info(`Viendo reporte ${reporte.descripcion}`, 'reportes');
	}

	function handleDelete(reporte: Reporte) {
		taxoLog.info(`Eliminando reporte ${reporte.descripcion}`, 'reportes');
	}

	const CURRENCY_SYMBOL: Record<string, string> = {
		PEN: 'S/.',
		USD: '$',
		EUR: '€'
	};

	function formatCellValue(reporte: Reporte, key: keyof Reporte): string {
		const val = reporte[key];
		if (val === null || val === undefined) return '-';
		if (key === 'total') {
			const num = typeof val === 'number' ? val : parseFloat(String(val));
			const symbol = CURRENCY_SYMBOL[reporte.moneda ?? 'PEN'] ?? 'S/.';
			return `${symbol} ${num.toFixed(2)}`;
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

	function tipoClass(tipo: Reporte['tipo']): string {
		return tipo === 'venta' ? 'text-blue-400' : 'text-amber-400';
	}

	function estadoClass(estado: Reporte['estado']): string {
		return estado === 'procesado' ? 'text-emerald-400' : 'text-amber-400';
	}
</script>

<div class="grid h-full grid-rows-[auto_1fr] gap-2 px-2 pb-2 text-sm">
	<!-- PRIMITIVA TOOLBAR -->
	<TableToolbar>
		{#snippet actions()}
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
			<TableFilter storageKey="reportes" bind:items={columnas} />
		{/snippet}

		{#snippet filters()}
			<div class="flex items-end gap-2">
				<div class="w-38">
					<Select placeholder="Tipo:" options={TIPO_OPTIONS} bind:value={tipoFiltro} />
				</div>
				<div class="w-30">
					<Select placeholder="Buscar por:" options={SEARCH_OPTIONS} bind:value={searchBy} />
				</div>
			</div>
			<Search
				bind:value={searchTerm}
				placeholder={searchBy === 'descripcion'
					? 'Buscar descripción...'
					: searchBy === 'tipo'
						? 'Buscar tipo...'
						: searchBy === 'periodo'
							? 'Buscar periodo...'
							: searchBy === 'estado'
								? 'Buscar estado...'
								: 'Buscar reporte...'}
			/>
		{/snippet}
	</TableToolbar>

	<!-- TABLA DE REPORTES -->
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
			{#if reportesFiltrados.length === 0}
				<tr>
					{#snippet emptyState()}
						No hay reportes registrados.
					{/snippet}
					{@render cell({
						colspan: columnasVisibles.length + 2,
						class: 'py-8 text-neutral-500',
						children: emptyState
					})}
				</tr>
			{:else}
				{#each reportesFiltrados as reporte, index (reporte.id)}
					{#snippet rowData()}
						{#snippet cellIndex()}
							{index + 1}
						{/snippet}
						{@render cell({ class: 'px-3 py-2 text-neutral-500', children: cellIndex })}
						{#each columnasVisibles as col (col.id)}
							{#snippet cellVal()}
								{#if col.key === 'tipo'}
									<span class={tipoClass(reporte.tipo)}>
										{reporte.tipo}
									</span>
								{:else if col.key === 'estado'}
									<span class={estadoClass(reporte.estado)}>
										{reporte.estado}
									</span>
								{:else}
									{formatCellValue(reporte, col.key)}
								{/if}
							{/snippet}
							{@render cell({ children: cellVal })}
						{/each}
						{#snippet cellActions()}
							<div class="flex items-center justify-center gap-2">
								<button
									class="cursor-pointer text-neutral-400 transition-colors hover:text-blue-400"
									onclick={() => handleVer(reporte)}
									title="Ver reporte"
								>
									<IconEye size={16} />
								</button>
								<button
									class="cursor-pointer text-neutral-400 transition-colors hover:text-red-400"
									onclick={() => handleDelete(reporte)}
									title="Eliminar"
								>
									<IconTrash size={16} />
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
