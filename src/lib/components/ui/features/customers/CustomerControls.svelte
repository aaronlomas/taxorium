<script lang="ts">
	import { IconDatabaseImport, IconDatabaseExport, IconPlus } from '@tabler/icons-svelte';
	//PRIMITIVAS
	import TableToolbar from '$lib/components/core/primitives/data/TableToolbar.svelte';
	import TableFilter, { type CheckItem } from '$lib/components/core/primitives/data/TableFilter.svelte';
	import Search from '$lib/components/core/primitives/data/Search.svelte';
	import Select from '$lib/components/core/primitives/forms/Select.svelte';
	import Hint from '$lib/components/core/primitives/data/Hint.svelte';
	import { onMount } from 'svelte';

	//TABLA Y MODAL
	import TableCustomers from './TableCustomers.svelte';
	import CustomersModal from './CustomersModal.svelte';

	//STORES
	import { customersStore, editingCustomer } from '$lib/features/customers';
	import { exportData, type ExportFormat } from '$lib/utilities/formats/export';
	import type { Customer } from '$lib/features/customers';

	let isModalOpen = $state(false);

	onMount(() => {
		customersStore.load();
	});

	function handleAdd() {
		$editingCustomer = null;
		isModalOpen = true;
	}

	function handleEdit(customer: Customer) {
		$editingCustomer = customer;
		isModalOpen = true;
	}

	let searchTerm = $state('');
	let searchBy = $state('');
	const SEARCH_OPTIONS = [
		{ value: '', label: 'Todos' },
		{ value: 'nombre', label: 'Nombre' },
		{ value: 'dni', label: 'DNI' },
		{ value: 'ruc', label: 'RUC' }
	];

	async function handleExportFile(format: ExportFormat) {
		// implement export logic here if needed
	}

	interface ColumnConfig extends CheckItem {
		key: keyof Customer;
	}

	let columnas = $state<ColumnConfig[]>([
		{ id: 'tipo_documento', key: 'tipo_documento', label: 'Tipo doc. identidad', checked: true },
		{ id: 'numero_documento', key: 'numero_documento', label: 'Número', checked: true },
		{ id: 'nombre', key: 'nombre', label: 'Nombre', checked: true },
		{ id: 'nombre_comercial', key: 'nombre_comercial', label: 'Nombre Comercial', checked: false },
		{ id: 'pais', key: 'pais', label: 'Pais', checked: false },
		{ id: 'departamento', key: 'departamento', label: 'Departamente', checked: false },
		{ id: 'provincia', key: 'provincia', label: 'Provincia', checked: false },
		{ id: 'distrito', key: 'distrito', label: 'Distrito', checked: false },
		{ id: 'direccion', key: 'direccion', label: 'Direccion', checked: false },
		{ id: 'telefono', key: 'telefono', label: 'Teléfono', checked: false },
		{ id: 'correo', key: 'correo', label: 'Correo electrónico', checked: false }
	]);

	let columnasVisibles = $derived(columnas.filter((c) => c.checked));

	let clientesFiltrados = $derived(
		$customersStore.filter((c) => {
			if (!searchTerm.trim()) return true;
			const term = searchTerm.toLowerCase();

			if (searchBy === 'nombre') {
				return c.nombre.toLowerCase().includes(term);
			} else if (searchBy === 'dni') {
				return c.numero_documento.toLowerCase().includes(term) && c.tipo_documento === 'DNI';
			} else if (searchBy === 'ruc') {
				return c.numero_documento.toLowerCase().includes(term) && c.tipo_documento === 'RUC';
			}

			return (
				c.nombre.toLowerCase().includes(term) ||
				c.numero_documento.toLowerCase().includes(term) ||
				(c.nombre_comercial && c.nombre_comercial.toLowerCase().includes(term))
			);
		})
	);
</script>

<!-- PANEL DE CONTROLES -->
<div class="grid h-full grid-rows-[auto_1fr] gap-2 px-2 pb-2 text-sm">
	<TableToolbar>
		<!-- CONTROLES -->
		{#snippet actions()}
			<Hint text="Agregar Cliente" side="bottom" align="start">
				<button
					class="flex cursor-pointer gap-2 rounded-xl bg-neutral-800 px-3 py-1 hover:bg-neutral-700"
					onclick={handleAdd}
				>
					<IconPlus size={20} />
				</button>
			</Hint>
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
			<TableFilter
				storageKey="clientes_v2"
				legacyStorageKeys={['clientes']}
				bind:items={columnas}
			/>
		{/snippet}
		<!-- FILTROS -->
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
	<TableCustomers clientes={clientesFiltrados} columnas={columnasVisibles} onEdit={handleEdit} />

	<CustomersModal
		bind:isOpen={isModalOpen}
		onClose={() => {
			isModalOpen = false;
		}}
	/>
</div>
