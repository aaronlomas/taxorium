<script lang="ts">
	import PanelVentas from './SalesPanel.svelte';
	import TableSales from './TableSales.svelte';
	import SalesDetails from './SalesDetails.svelte';
</script>

<main class="flex flex-col gap-2 p-2">
	<PanelVentas />
	<TableSales />
	<SalesDetails />
</main>
