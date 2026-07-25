<script lang="ts">
	import PanelVentas from '$lib/components/ui/views/sales/SalesPanel.svelte';
	let isModalOpen = $state(false);
</script>

<main class="grid h-full grid-rows-[1fr_30px]">
	<PanelVentas />
	<footer class="flex border-y border-y-neutral-800 text-sm">
		<div class="w-full border-r border-r-neutral-800">
			<button class="h-full cursor-pointer bg-blue-700 px-2 hover:bg-blue-600">Ver Tutorial</button>
		</div>
		<button
			onclick={() => (isModalOpen = true)}
			class="cursor-pointer border-r border-r-neutral-800 px-2 transition-colors hover:bg-neutral-900 whitespace-nowrap"
			>Agregar Cliente</button
		>
		<button
			class="cursor-pointer border-r border-r-neutral-800 px-2 transition-colors hover:bg-neutral-900"
			>Importar</button
		>
		<button
			class="cursor-pointer border-r border-r-neutral-800 px-2 transition-colors hover:bg-neutral-900"
			>Exportar</button
		>
	</footer>
</main>
