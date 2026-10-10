<script lang="ts" module>
	import type { Snippet } from 'svelte';

	// Exportamos los snippets reutilizables para las celdas y filas
	export { row, cell, headCell };
</script>

<script lang="ts">
	let {
		head,
		body
	}: {
		head?: Snippet;
		body?: Snippet;
	} = $props();
</script>

<!-- Definición interna de snippets con estilos garantizados -->
{#snippet row({ children }: { children: Snippet })}
	<tr class="hover:bg-neutral-850 transition-colors">
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
	<td {colspan} class="px-3 py-2 text-sm {className}">
		{@render children()}
	</td>
{/snippet}

{#snippet headCell({ children }: { children: Snippet })}
	<th class="px-2">
		{@render children()}
	</th>
{/snippet}

<!-- Estructura HTML contenedora -->
<div class="overflow-auto rounded-md border border-neutral-800">
	<table class="w-full bg-neutral-900 text-center">
		<thead class="border-b border-neutral-800 text-neutral-400">
			{@render head?.()}
		</thead>
		<tbody class="divide-y divide-neutral-800 text-neutral-200">
			{@render body?.()}
		</tbody>
	</table>
</div>
