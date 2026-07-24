<script lang="ts">
	import TableProduct from "./TableProduct.svelte";
	import ProductModal from "./ProductModal.svelte";

	let isModalOpen = $state(false);
</script>

<main class="grid grid-rows-[1fr_30px] h-full">
  <TableProduct/>
  <footer class="flex text-sm border-y border-y-neutral-800">
    <div class="w-full border-r border-r-neutral-800">
      <button class="bg-blue-700 px-2 cursor-pointer h-full hover:bg-blue-600">Ver Tutorial</button>
    </div>
    <button onclick={() => isModalOpen = true} class="border-r border-r-neutral-800 px-2 cursor-pointer hover:bg-neutral-900 transition-colors">Agregar</button>
    <button class="border-r border-r-neutral-800 px-2 cursor-pointer hover:bg-neutral-900 transition-colors">Importar</button>
    <button class="border-r border-r-neutral-800 px-2 cursor-pointer hover:bg-neutral-900 transition-colors">Exportar</button>
  </footer>

  <ProductModal bind:isOpen={isModalOpen} onClose={() => isModalOpen = false} />
</main>