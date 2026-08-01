<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { IconFileInvoice, IconReceipt } from "@tabler/icons-svelte";

  const dispatch = createEventDispatcher();
</script>
<!-- PANEL ESPECÍFICO DE VENTA RÁPIDA -->
<nav class="flex flex-1 flex-col overflow-y-auto">
	<p class="p-2 text-sm text-neutral-400 italic">Venta Rápida</p>
	<button
		type="button"
		on:click={() =>
			dispatch('seleccionarOpcion', { nombre: 'Nueva Factura', ruta: '/vender/factura' })}
		class="flex items-center gap-2 rounded p-2 text-sm font-medium transition-colors hover:bg-neutral-800"
	>
		<IconFileInvoice size={18} class="text-blue-400" />
		<span>Nueva Factura</span>
	</button>
	<button
		type="button"
		on:click={() =>
			dispatch('seleccionarOpcion', { nombre: 'Nueva Boleta', ruta: '/vender/boleta' })}
		class="flex items-center gap-2 rounded p-2 text-sm font-medium transition-colors hover:bg-neutral-800"
	>
		<IconReceipt size={18} class="text-green-400" />
		<span>Nueva Boleta</span>
	</button>
</nav>
