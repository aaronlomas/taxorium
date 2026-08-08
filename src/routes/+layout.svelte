<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { auth } from '$lib/stores/auth';
	import { tenantStore } from '$lib/stores/tenant';
	import { licenseStore } from '$lib/stores/license';
	import { configStore } from '$lib/stores/config';
	import { taxoLog } from '$lib/stores/taxoLog';
	import TitleBar from '$lib/components/ui/TitleBar.svelte';
	import ResizeBorder from '$lib/components/ui/ResizeBorder.svelte';

	let { children } = $props();

	onMount(async () => {
		// Inicializar listener de logs del backend
		taxoLog.initListener();
		// Inicializar configuración del nodo (Servidor/Cliente)
		await configStore.init();
		
		let unsub = configStore.subscribe((config) => {
			if (config.role === null && $page.url.pathname !== '/setup-node') {
				goto('/setup-node');
			}
		});

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

<div class="flex flex-col h-screen w-full bg-neutral-950">
	<ResizeBorder />
	<TitleBar />
	<div class="flex-1 w-full h-full relative">
		{@render children()}
	</div>
</div>
