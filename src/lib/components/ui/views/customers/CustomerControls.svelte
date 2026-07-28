<script lang="ts">
	import TableCustomers from './TableCustomers.svelte';
</script>

<main class="grid h-full grid-rows-[1fr_30px]">
  <TableCustomers/>
	<footer class="flex border-y border-y-neutral-800 text-sm">
		<div class="w-full border-r border-r-neutral-800">
			<button class="h-full cursor-pointer bg-blue-700 px-2 hover:bg-blue-600">Ver Tutorial</button>
		</div>
		<button class="cursor-pointer border-r border-r-neutral-800 px-2 whitespace-nowrap">+ Nuevo Cliente</button>
		<button class="cursor-pointer border-r border-r-neutral-800 px-2">Importar</button>
		<button class="cursor-pointer border-r border-r-neutral-800 px-2">Exportar</button>
	</footer>
</main>
