<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { onMount } from 'svelte';
	import { auth } from '$lib/stores/auth';
	import { tenantStore } from '$lib/stores/tenant';
	import { licenseStore } from '$lib/stores/license';
	import TitleBar from '$lib/components/ui/TitleBar.svelte';
	import ResizeBorder from '$lib/components/ui/ResizeBorder.svelte';

	let { children } = $props();

	onMount(async () => {
		// Inicializar licencia (lee lease local y renueva silenciosamente si hay internet)
		licenseStore.init();
		await auth.init();
		// Cargar datos del tenant si el usuario ya está autenticado
		if ($auth.user) {
			await tenantStore.load();
		}
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="flex flex-col h-screen w-full overflow-hidden bg-neutral-950">
	<ResizeBorder />
	<TitleBar />
	<div class="flex-1 w-full h-full relative overflow-hidden">
		{@render children()}
	</div>
</div>
