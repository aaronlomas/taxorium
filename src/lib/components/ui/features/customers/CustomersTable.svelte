<script lang="ts">
	import { onMount } from 'svelte';
	import { IconTrash, IconEdit } from '@tabler/icons-svelte';
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/data/Table.svelte';
	import type { CheckItem } from '$lib/components/core/primitives/data/TableFilter.svelte';
	import CustomersControls from './CustomersControls.svelte';
	import CustomersModal from './CustomersModal.svelte';
	import {
		customersStore,
		editingCustomer,
		customerClient,
		type Customer
	} from '$lib/features/customers';
	import { taxoLog } from '$lib/features/taxoLog';
	import { exportData, type ExportFormat } from '$lib/utilities/formats/export';

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
	let isModalOpen = $state(false);
	let searchTerm = $state('');
	let searchBy = $state('');

	onMount(() => {
		customersStore.load();
	});

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

	function handleAdd() {
		$editingCustomer = null;
		isModalOpen = true;
	}

	function handleEdit(customer: Customer) {
		$editingCustomer = customer;
		isModalOpen = true;
	}

	async function handleExportFile(format: ExportFormat) {
		const savedPath = await exportData(clientesFiltrados, columnasVisibles, 'clientes', format);
		if (savedPath) {
			taxoLog.info(`Exportado exitosamente en: ${savedPath}`, 'clientes');
		}
	}

	async function handleDelete(id: number, nombre: string) {
		if (confirm(`¿Estás seguro de eliminar el cliente "${nombre}"?`)) {
			try {
				await customerClient.deleteCustomer(id);
				taxoLog.info(`Cliente '${nombre}' eliminado`, 'clientes');
				await customersStore.load();
			} catch (e: any) {
				const msg = typeof e === 'string' ? e : (e?.message ?? 'Error al eliminar el cliente');
				taxoLog.error(msg, 'clientes');
			}
		}
	}

	function formatCellValue(customer: Customer, key: keyof Customer): string {
		const val = customer[key];
		if (val === null || val === undefined || val === '') return '-';
		if (typeof val === 'boolean') return val ? 'Sí' : 'No';
		return String(val);
	}
</script>

<div class="grid h-full grid-rows-[auto_1fr] gap-1 text-sm">
	<CustomersControls
		bind:columnas
		bind:searchTerm
		bind:searchBy
		onAdd={handleAdd}
		onExport={handleExportFile}
	/>

	<CustomersModal bind:isOpen={isModalOpen} onClose={() => (isModalOpen = false)} />

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
			{#if clientesFiltrados.length === 0}
				<tr>
					{#snippet emptyState()}
						No hay clientes registrados.
					{/snippet}
					{@render cell({
						colspan: columnasVisibles.length + 2,
						class: 'py-8 text-neutral-500',
						children: emptyState
					})}
				</tr>
			{:else}
				{#each clientesFiltrados as customer, index (customer.id)}
					{#snippet rowData()}
						{#snippet cellIndex()}
							{index + 1}
						{/snippet}
						{@render cell({ class: 'px-3 py-2 text-neutral-500', children: cellIndex })}
						{#each columnasVisibles as col (col.id)}
							{#snippet cellVal()}
								{formatCellValue(customer, col.key)}
							{/snippet}
							{@render cell({ children: cellVal })}
						{/each}
						{#snippet cellActions()}
							<div class="flex items-center justify-center gap-2">
								<button
									class="cursor-pointer text-neutral-400 transition-colors hover:text-blue-400"
									onclick={() => handleEdit(customer)}
									title="Editar"
								>
									<IconEdit size={16} />
								</button>
								<button
									class="cursor-pointer text-neutral-400 transition-colors hover:text-red-400"
									onclick={() => handleDelete(customer.id, customer.nombre)}
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
