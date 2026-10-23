<script lang="ts">
	import PurchasePanel from './PurchasePanel.svelte';
	import TablePurchase from './TablePurchase.svelte';
	import PurchaseDetails from './PurchaseDetails.svelte';
</script>

<main class="flex flex-col gap-2 p-2">
	<PurchasePanel />
	<TablePurchase />
	<PurchaseDetails />
</main>
