<script lang="ts">
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/Table.svelte';
	import { IconTrash, IconEdit } from '@tabler/icons-svelte';
	import type { Customer } from '$lib/services/customers/clientCustomer';
	import { customerClient } from '$lib/services/customers/clientCustomer';
	import { customersStore } from '$lib/stores/customers';
	import { taxoLog } from '$lib/stores/taxoLog';
	import type { CheckItem } from '$lib/components/core/primitives/TableFilter.svelte';

	interface ColumnConfig extends CheckItem {
		key: keyof Customer;
	}

	let {
		clientes = [],
		columnas = [],
		onEdit
	}: {
		clientes: Customer[];
		columnas: ColumnConfig[];
		onEdit: (customer: Customer) => void;
	} = $props();

	async function handleDelete(id: number, nombre: string) {
		if (confirm(`¿Estás seguro de eliminar el cliente "${nombre}"?`)) {
			try {
				await customerClient.deleteCustomer(id);
				taxoLog.info(`Cliente '${nombre}' eliminado`, 'clientes');
				await customersStore.load();
			} catch (e: any) {
				const msg = e?.message ?? 'Error al eliminar el cliente';
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

<div class="h-full overflow-hidden">
	<Table>
		{#snippet head()}
			<tr class="text-blue-400">
				{#snippet headIndex()}
					#
				{/snippet}
				{@render headCell({ children: headIndex })}
				{#each columnas as col (col.id)}
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
			{#if clientes.length === 0}
				<tr>
					{#snippet emptyState()}
						No hay clientes registrados.
					{/snippet}
					{@render cell({
						colspan: columnas.length + 2,
						class: 'py-8 text-neutral-500',
						children: emptyState
					})}
				</tr>
			{:else}
				{#each clientes as customer, index (customer.id)}
					{#snippet rowData()}
						{#snippet cellIndex()}
							{index + 1}
						{/snippet}
						{@render cell({ class: 'px-3 py-2 text-neutral-500', children: cellIndex })}

						{#each columnas as col (col.id)}
							{#snippet cellVal()}
								{formatCellValue(customer, col.key)}
							{/snippet}
							{@render cell({ children: cellVal })}
						{/each}

						{#snippet cellActions()}
							<div class="flex items-center justify-center gap-2">
								<button
									class="cursor-pointer text-neutral-400 transition-colors hover:text-blue-400"
									onclick={() => onEdit(customer)}
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
