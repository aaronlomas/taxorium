<script lang="ts">
	import TableProduct from "./TableProduct.svelte";
</script>

<main class="grid grid-rows-[1fr_30px] h-full">
    <TableProduct/>
  <footer class="flex text-sm border-y border-y-neutral-800">
  <div class="w-full border-r border-r-neutral-800">
    <button class="bg-blue-700 px-2 cursor-pointer h-full hover:bg-blue-600">Ver Tutorial</button>
  </div>
    <button class="border-r border-r-neutral-800 px-2 cursor-pointer">Agregar</button>
    <button class="border-r border-r-neutral-800 px-2 cursor-pointer">Importar</button>
    <button class="border-r border-r-neutral-800 px-2 cursor-pointer">Exportar</button>
  </footer>
</main>