<script lang="ts">
	import TableReport from './TableReport.svelte';
</script>

<main class="grid h-full grid-rows-[1fr_30px]">
	<TableReport />
	<footer class="flex border-y border-y-neutral-800 text-sm">
		<div class="w-full border-r border-r-neutral-800">
			<button class="h-full cursor-pointer bg-blue-700 px-2 hover:bg-blue-600">Ver Tutorial</button>
		</div>
		<button class="cursor-pointer border-r border-r-neutral-800 px-2">Eliminar</button>
		<button class="cursor-pointer border-r border-r-neutral-800 px-2">Importar</button>
		<button class="cursor-pointer border-r border-r-neutral-800 px-2">Exportar</button>
	</footer>
</main>
