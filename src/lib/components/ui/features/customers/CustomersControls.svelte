<script lang="ts">
	import { IconDatabaseImport, IconDatabaseExport, IconPlus } from '@tabler/icons-svelte';
	import TableFilter, {
		type CheckItem
	} from '$lib/components/core/primitives/data/TableFilter.svelte';
	import TableToolbar from '$lib/components/core/primitives/data/TableToolbar.svelte';
	import Search from '$lib/components/core/primitives/data/Search.svelte';
	import Select from '$lib/components/core/primitives/forms/Select.svelte';
	import Hint from '$lib/components/core/primitives/data/Hint.svelte';
	import type { ExportFormat } from '$lib/utilities/formats/export';

	interface ColumnConfig extends CheckItem {
		key: string;
	}

	interface Props {
		columnas: ColumnConfig[];
		searchTerm: string;
		searchBy: string;
		onAdd: () => void;
		onExport: (format: ExportFormat) => void;
	}

	let {
		columnas = $bindable(),
		searchTerm = $bindable(''),
		searchBy = $bindable(''),
		onAdd,
		onExport
	}: Props = $props();

	const SEARCH_OPTIONS = [
		{ value: '', label: 'Todos' },
		{ value: 'nombre', label: 'Nombre' },
		{ value: 'dni', label: 'DNI' },
		{ value: 'ruc', label: 'RUC' }
	];
</script>

<TableToolbar>
	{#snippet actions()}
		<Hint text="Agregar Cliente" side="bottom" align="start">
			<button
				class="flex cursor-pointer gap-2 rounded-xl border border-brand-border bg-brand-bg-secondary px-3 py-1 hover:bg-brand-bg-hover"
				onclick={onAdd}
			>
				<IconPlus size={20} />
			</button>
		</Hint>
		<button
			class="flex cursor-pointer gap-2 rounded-xl border border-brand-border bg-brand-bg-secondary px-3 py-1 hover:bg-brand-bg-hover"
		>
			<IconDatabaseImport size={20} />Importar
		</button>
		<div class="group relative">
			<button
				class="flex cursor-pointer items-center gap-2 rounded-xl border border-brand-border bg-brand-bg-secondary px-3 py-1 hover:bg-brand-bg-hover"
			>
				<IconDatabaseExport size={20} />Exportar
			</button>
			<div
				class="absolute top-full z-10 hidden w-48 flex-col overflow-hidden rounded-md border border-brand-border bg-brand-bg-primary shadow-lg group-hover:flex"
			>
				<button
					class="px-4 py-2 text-left text-sm hover:bg-brand-bg-hover"
					onclick={() => onExport('xlsx')}>Excel (.xlsx)</button
				>
				<button
					class="px-4 py-2 text-left text-sm hover:bg-brand-bg-hover"
					onclick={() => onExport('csv-comma')}>CSV (comas)</button
				>
				<button
					class="px-4 py-2 text-left text-sm hover:bg-brand-bg-hover"
					onclick={() => onExport('csv-semicolon')}>CSV (puntos y comas)</button
				>
			</div>
		</div>
		<TableFilter storageKey="clientes_v2" legacyStorageKeys={['clientes']} bind:items={columnas} />
	{/snippet}

	{#snippet filters()}
		<div class="w-30">
			<Select placeholder="Buscar por:" options={SEARCH_OPTIONS} bind:value={searchBy} />
		</div>
		<Search
			bind:value={searchTerm}
			placeholder={searchBy === 'nombre'
				? 'Buscar nombre...'
				: searchBy === 'dni'
					? 'Buscar DNI...'
					: searchBy === 'ruc'
						? 'Buscar RUC...'
						: 'Buscar cliente...'}
		/>
	{/snippet}
</TableToolbar>
