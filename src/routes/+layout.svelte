<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { auth } from '$lib/features/auth';
	import { tenantStore } from '$lib/features/tenant';
	import { licenseStore } from '$lib/features/license';
	import { icbperStore } from '$lib/features/settings';
	import { configStore } from '$lib/integrations/tauri/nodeConfig';
	import { taxoLog } from '$lib/features/taxoLog';
	import TitleBar from '$lib/components/ui/app-shell/TitleBar.svelte';
	import ResizeBorder from '$lib/components/ui/app-shell/ResizeBorder.svelte';

	let { children } = $props();

	// La ruta /print se abre en una WebviewWindow independiente de Tauri.
	// NO debe heredar el shell de la app (TitleBar, fondo oscuro, ResizeBorder)
	// porque todo eso aparecería en el PDF al imprimir.
	const isPrintRoute = $derived($page.url.pathname.startsWith('/print'));

	onMount(async () => {
		// La ventana de impresión es standalone — no inicializa nada del sistema.
		if (isPrintRoute) return;

		// Inicializar listener de logs del backend
		taxoLog.initListener();
		// Inicializar configuración del nodo (Servidor/Cliente)
		await configStore.init();
		// Tarifa ICBPER configurable (Ajustes), usada en el cálculo de ventas
		await icbperStore.load();

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

{#if isPrintRoute}
	<!-- Ventana de impresión standalone: sin shell, sin fondo oscuro, solo el comprobante -->
	{@render children()}
{:else}
	<div class="flex h-screen w-full flex-col overflow-hidden bg-neutral-950">
		<ResizeBorder />
		<TitleBar />
		<div class="relative h-full w-full flex-1">
			{@render children()}
		</div>
	</div>
{/if}
