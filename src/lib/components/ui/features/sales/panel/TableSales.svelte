<script lang="ts">
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/data/Table.svelte';
	import { salesStore } from '$lib/features/sales';
	import { getUnitDisplay } from '$lib/features/catalogos';

	const CURRENCY_SYMBOL: Record<string, string> = {
		PEN: 'S/.',
		USD: '$',
		EUR: '€'
	};

	function formatMoney(item: { moneda: string }, value: number): string {
		const symbol = CURRENCY_SYMBOL[item.moneda] ?? 'S/.';
		return `${symbol} ${value.toFixed(2)}`;
	}

	function handleDelete(id: number) {
		salesStore.remove(id);
	}
</script>

<Table>
	{#snippet head()}
		<tr class="text-sm text-blue-400">
			{#snippet headIndex()}
				#
			{/snippet}
			{@render headCell({ children: headIndex })}
			{#each ['Descripción', 'Unidad', 'Cantidad', 'Precio U.', 'Sub Total', 'Total'] as label (label)}
				{#snippet headCol()}
					{label}
				{/snippet}
				{@render headCell({ children: headCol })}
			{/each}
			{#snippet headActions()}
				Acción
			{/snippet}
			{@render headCell({ children: headActions })}
		</tr>
	{/snippet}

	{#snippet body()}
		{#if $salesStore.length === 0}
			<tr>
				{#snippet emptyState()}
					No hay productos agregados.
				{/snippet}
				{@render cell({
					colspan: 8,
					class: 'py-8 text-neutral-500',
					children: emptyState
				})}
			</tr>
		{:else}
			{#each $salesStore as item, index (item.id)}
				{#snippet rowData()}
					{#snippet cellIndex()}
						{index + 1}
					{/snippet}
					{@render cell({ class: 'px-3 py-2 text-neutral-500', children: cellIndex })}
					{#snippet cellDesc()}
						{item.descripcion}
					{/snippet}
					{@render cell({ children: cellDesc })}
					{#snippet cellUnidad()}
						{getUnitDisplay(item.unidad)}
					{/snippet}
					{@render cell({ children: cellUnidad })}
					{#snippet cellCantidad()}
						{item.cantidad}
					{/snippet}
					{@render cell({ children: cellCantidad })}
					{#snippet cellPrecio()}
						{formatMoney(item, item.precioUnitario)}
					{/snippet}
					{@render cell({ children: cellPrecio })}
					{#snippet cellSubtotal()}
						{formatMoney(item, item.subtotal)}
					{/snippet}
					{@render cell({ children: cellSubtotal })}
					{#snippet cellTotal()}
						{formatMoney(item, item.total)}
					{/snippet}
					{@render cell({ children: cellTotal })}
					{#snippet cellAccion()}
						<button
							class="cursor-pointer text-red-500 transition-colors hover:text-red-400"
							title="Eliminar"
							onclick={() => handleDelete(item.id)}
						>
							Eliminar
						</button>
					{/snippet}
					{@render cell({ children: cellAccion })}
				{/snippet}
				{@render row({ children: rowData })}
			{/each}
		{/if}
	{/snippet}
</Table>
