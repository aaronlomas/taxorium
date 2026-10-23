<script lang="ts">
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/data/Table.svelte';
	import { purchaseStore } from '$lib/features/purchase';
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
		purchaseStore.remove(id);
	}
</script>

<Table>
	{#snippet head()}
		<tr class="text-sm text-blue-400">
			{#snippet headIndex()}
				#
			{/snippet}
			{@render headCell({ children: headIndex })}
			{#each ['Descripción', 'U. Compra', 'Cantidad', 'Factor', 'Cant. Base', 'Costo U.', 'Sub Total'] as label (label)}
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
		{#if $purchaseStore.length === 0}
			<tr>
				{#snippet emptyState()}
					No hay productos agregados.
				{/snippet}
				{@render cell({
					colspan: 9,
					class: 'py-8 text-neutral-500',
					children: emptyState
				})}
			</tr>
		{:else}
			{#each $purchaseStore as item, index (item.id)}
				{#snippet rowData()}
					{#snippet cellIndex()}
						{index + 1}
					{/snippet}
					{@render cell({ class: 'px-3 py-2 text-neutral-500', children: cellIndex })}
					{#snippet cellDesc()}
						{item.descripcion}
					{/snippet}
					{@render cell({ children: cellDesc })}
					{#snippet cellUnidadCompra()}
						{getUnitDisplay(item.unidadCompra)}
					{/snippet}
					{@render cell({ children: cellUnidadCompra })}
					{#snippet cellCantidad()}
						{item.cantidadCompra}
					{/snippet}
					{@render cell({ children: cellCantidad })}
					{#snippet cellFactor()}
						× {item.factorConversion}
					{/snippet}
					{@render cell({ children: cellFactor })}
					{#snippet cellCantidadBase()}
						{item.cantidadBase}
						<span class="text-neutral-500"> {getUnitDisplay(item.unidadBase)}</span>
					{/snippet}
					{@render cell({ children: cellCantidadBase })}
					{#snippet cellCosto()}
						{formatMoney(item, item.costoUnitarioCompra)}
					{/snippet}
					{@render cell({ children: cellCosto })}
					{#snippet cellSubtotal()}
						{formatMoney(item, item.subtotal)}
					{/snippet}
					{@render cell({ children: cellSubtotal })}
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
