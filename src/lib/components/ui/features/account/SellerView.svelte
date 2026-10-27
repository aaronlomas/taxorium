<script lang="ts">
	import SellerPanel from '$lib/components/ui/features/account/SellerPanel.svelte';
	import SellerTable from '$lib/components/ui/features/account/SellerTable.svelte';
</script>

<!--
	Vista de puntos de venta: alta/edición y listado de vendedores del usuario.
	Compone las interfaces de views/account/ — no contiene lógica propia.
-->
<div class="grid h-full grid-rows-[auto_auto_1fr] overflow-hidden">
	<div>
		<SellerPanel />
	</div>
	<div class="overflow-scroll">
		<SellerTable />
	</div>
</div>
