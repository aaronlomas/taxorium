<script lang="ts">
	import {
		IconDatabaseImport,
		IconDatabaseExport,
		IconDownload,
		IconSend
	} from '@tabler/icons-svelte';
	import TableFilter, { type CheckItem } from '$lib/components/core/primitives/TableFilter.svelte';
	import TableToolbar from '$lib/components/core/primitives/TableToolbar.svelte';
	import Select from '$lib/components/core/primitives/Select.svelte';
	import Search from '$lib/components/core/primitives/Search.svelte';
	import InputDate from '$lib/components/core/primitives/InputDate.svelte';
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/Table.svelte';

	import { exportData, type ExportFormat } from '$lib/utilities/formats/export';
	import { taxoLog } from '$lib/stores/taxoLog';
	import { formatFecha, parseFecha } from '$lib/components/ui/views/vouchers/voucherContext';

	interface Resumen {
		id: number;
		fecha_emision: string;
		fecha_referencia: string;
		identificador: string;
		estado: 'registrado' | 'rechazado' | 'aceptado';
		ticket: string;
	}

	interface ColumnConfig extends CheckItem {
		key: keyof Resumen;
	}

	let columnas = $state<ColumnConfig[]>([
		{ id: 'fecha_emision', key: 'fecha_emision', label: 'Fecha de Emisión', checked: true },
		{
			id: 'fecha_referencia',
			key: 'fecha_referencia',
			label: 'Fecha de Referencia',
			checked: true
		},
		{ id: 'identificador', key: 'identificador', label: 'Identificador', checked: true },
		{ id: 'estado', key: 'estado', label: 'Estado', checked: true },
		{ id: 'ticket', key: 'ticket', label: 'Ticket', checked: true }
	]);

	let columnasVisibles = $derived(columnas.filter((c) => c.checked));

	let resumenes = $state<Resumen[]>([
		{
			id: 1,
			fecha_emision: '2026-08-01',
			fecha_referencia: '2026-08-01',
			identificador: 'RC-2026081-1',
			estado: 'aceptado',
			ticket: '12334235135123'
		}
	]);

	let searchTerm = $state('');
	let searchBy = $state('');
	let fechaDesde = $state('');

	const SEARCH_OPTIONS = [
		{ value: '', label: 'Todos' },
		{ value: 'identificador', label: 'Identificador' },
		{ value: 'estado', label: 'Estado' },
		{ value: 'ticket', label: 'Ticket' }
	];

	let resumenesFiltrados = $derived(
		resumenes.filter((r) => {
			if (fechaDesde) {
				const desde = parseFecha(fechaDesde);
				const referencia = parseFecha(r.fecha_referencia);
				if (desde && referencia && referencia < desde) return false;
			}

			if (!searchTerm.trim()) return true;
			const term = searchTerm.toLowerCase();

			if (searchBy === 'identificador') {
				return r.identificador.toLowerCase().includes(term);
			} else if (searchBy === 'estado') {
				return r.estado.toLowerCase().includes(term);
			} else if (searchBy === 'ticket') {
				return r.ticket.toLowerCase().includes(term);
			}

			return (
				r.identificador.toLowerCase().includes(term) ||
				r.estado.toLowerCase().includes(term) ||
				r.ticket.toLowerCase().includes(term)
			);
		})
	);

	async function handleExportFile(format: ExportFormat) {
		const savedPath = await exportData(resumenesFiltrados, columnasVisibles, 'resumenes', format);
		if (savedPath) {
			taxoLog.info(`Exportado exitosamente en: ${savedPath}`, 'resumenes');
		}
	}

	function handleEmitir(resumen: Resumen) {
		taxoLog.info(`Emitiendo resumen ${resumen.identificador} a SUNAT`, 'resumenes');
	}

	function handleDescargar(resumen: Resumen, tipo: 'XML' | 'CDR') {
		taxoLog.info(`Descargando ${tipo} del resumen ${resumen.identificador}`, 'resumenes');
	}

	function formatCellValue(resumen: Resumen, key: keyof Resumen): string {
		const val = resumen[key];
		if (val === null || val === undefined) return '-';
		if (key === 'fecha_emision' || key === 'fecha_referencia') {
			const s = String(val);
			return /^\d{2}\/\d{2}\/\d{4}/.test(s) ? s.slice(0, 10) : formatFecha(s);
		}
		return String(val);
	}

	function estadoClass(estado: Resumen['estado']): string {
		switch (estado) {
			case 'aceptado':
				return 'text-emerald-400';
			case 'rechazado':
				return 'text-red-400';
			default:
				return 'text-amber-400';
		}
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
			<TableFilter storageKey="resumenes" bind:items={columnas} />
		{/snippet}

		{#snippet filters()}
			<div class="flex items-end gap-2">
				<div class="w-40">
					<InputDate label="Desde la fecha:" bind:value={fechaDesde} />
				</div>
			</div>
			<div class="flex items-end gap-2">
				<div class="w-30">
					<Select placeholder="Buscar por:" options={SEARCH_OPTIONS} bind:value={searchBy} />
				</div>
				<Search
					bind:value={searchTerm}
					placeholder={searchBy === 'identificador'
						? 'Buscar identificador...'
						: searchBy === 'estado'
							? 'Buscar estado...'
							: searchBy === 'ticket'
								? 'Buscar ticket...'
								: 'Buscar resumen...'}
				/>
			</div>
		{/snippet}
	</TableToolbar>

	<!-- TABLA DE RESÚMENES -->
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
				{#snippet headDownload()}
					Descargar
				{/snippet}
				{@render headCell({ children: headDownload })}
				{#snippet headActions()}
					Acciones
				{/snippet}
				{@render headCell({ children: headActions })}
			</tr>
		{/snippet}

		{#snippet body()}
			{#if resumenesFiltrados.length === 0}
				<tr>
					{#snippet emptyState()}
						No hay resúmenes registrados.
					{/snippet}
					{@render cell({
						colspan: columnasVisibles.length + 3,
						class: 'py-8 text-neutral-500',
						children: emptyState
					})}
				</tr>
			{:else}
				{#each resumenesFiltrados as resumen, index (resumen.id)}
					{#snippet rowData()}
						{#snippet cellIndex()}
							{index + 1}
						{/snippet}
						{@render cell({ class: 'px-3 py-2 text-neutral-500', children: cellIndex })}
						{#each columnasVisibles as col (col.id)}
							{#snippet cellVal()}
								{#if col.key === 'estado'}
									<span class={estadoClass(resumen.estado)}>
										{resumen.estado}
									</span>
								{:else}
									{formatCellValue(resumen, col.key)}
								{/if}
							{/snippet}
							{@render cell({ children: cellVal })}
						{/each}
						{#snippet cellDownload()}
							<div class="flex items-center justify-center gap-2">
								<button
									class="flex cursor-pointer items-center gap-1 rounded-sm border border-blue-500/40 px-2 py-0.5 text-blue-400 transition-colors hover:bg-blue-500/10"
									onclick={() => handleDescargar(resumen, 'XML')}
									title="Descargar XML"
								>
									<IconDownload size={14} />
									XML
								</button>
								<button
									class="flex cursor-pointer items-center gap-1 rounded-sm border border-blue-500/40 px-2 py-0.5 text-blue-400 transition-colors hover:bg-blue-500/10"
									onclick={() => handleDescargar(resumen, 'CDR')}
									title="Descargar CDR"
								>
									<IconDownload size={14} />
									CDR
								</button>
							</div>
						{/snippet}
						{@render cell({ children: cellDownload })}
						{#snippet cellActions()}
							<div class="flex items-center justify-center gap-2">
								<button
									class="flex cursor-pointer items-center gap-1 rounded-sm border border-blue-500/40 px-2 py-0.5 text-blue-400 transition-colors hover:bg-blue-500/10"
									onclick={() => handleEmitir(resumen)}
									title="Emitir a SUNAT"
								>
									<IconSend size={14} />
									Emitir
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
