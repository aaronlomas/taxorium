<script lang="ts">
	import PanelSeller from '$lib/components/ui/views/account/PanelSeller.svelte';
	import TableSeller from '$lib/components/ui/views/account/TableSeller.svelte';
</script>

<!--
	Vista de puntos de venta: alta/edición y listado de vendedores del usuario.
	Compone las interfaces de views/account/ — no contiene lógica propia.
-->
<div class="grid h-full grid-rows-[auto_auto_1fr] overflow-hidden">
	<div class="flex border-b border-neutral-700">
		<h1 class="p-2 font-extrabold">Administrar Puntos de Venta</h1>
	</div>
	<div>
		<PanelSeller />
	</div>
	<div class="overflow-scroll px-2">
		<TableSeller />
	</div>
</div>
