<script lang="ts" module>
	import type { Snippet } from 'svelte';

	// Exportamos los snippets reutilizables para las celdas y filas
	export { row, cell, headCell };
</script>

<script lang="ts">
	let {
		head,
		body,
		sm = false
	}: {
		head?: Snippet;
		body?: Snippet;
		/** Aplica un alto fijo de 380px con scroll al contenedor de la tabla. */
		sm?: boolean;
	} = $props();
</script>

<!-- Definición interna de snippets con estilos garantizados -->
{#snippet row({ children }: { children: Snippet })}
	<tr class="">
		{@render children()}
	</tr>
{/snippet}

{#snippet cell({
	children,
	colspan,
	class: className = ''
}: {
	children: Snippet;
	colspan?: number;
	class?: string;
})}
	<td {colspan} class="px-3 text-sm {className}">
		{@render children()}
	</td>
{/snippet}

{#snippet headCell({ children }: { children: Snippet })}
	<th class="px-2 text-sm text-brand-text-sky">
		{@render children()}
	</th>
{/snippet}

<!-- Contenedor -->
<div class="rounded-md overflow-auto border border-brand-border {sm ? 'max-h-65' : ''}">
	<table class="w-full bg-brand-bg-primary text-center">
		<thead class="border-b border-brand-border text-brand-text-secondary">
			{@render head?.()}
		</thead>
		<tbody class="divide-y divide-brand-border text-brand-text-primary">
			{@render body?.()}
		</tbody>
	</table>
</div>
