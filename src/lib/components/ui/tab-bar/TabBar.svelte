<script lang="ts">
	// Permite insertar Tabs y otros elementos
</script>

<div class="scrollbar-hide flex items-center overflow-x-auto border-0 bg-neutral-900 select-none">
	<slot />
</div>

<style>
	/* Ocultar scrollbar pero permitir scroll */
	.scrollbar-hide::-webkit-scrollbar {
		display: none;
	}
	.scrollbar-hide {
		-ms-overflow-style: none;
		scrollbar-width: none;
	}
</style>
