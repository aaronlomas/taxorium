<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { auth, isAuthenticated } from '$lib/stores/auth';
	import { tenantStore, isConfigured } from '$lib/stores/tenant';
	import { licenseStore, licenseValid } from '$lib/stores/license';

	let { children } = $props();

	// Rutas que no requieren autenticación
	const PUBLIC_ROUTES = ['/login', '/register', '/customers'];
	const RUTAS_SIN_LICENCIA = ['/activate'];

	onMount(async () => {
		// Inicializar licencia (lee lease local y renueva silenciosamente si hay internet)
		licenseStore.init();
		await auth.init();
	});

	// Guard reactivo
	$effect(() => {
		const pathname = page.url.pathname;
		const authed = $isAuthenticated;
		const configured = $isConfigured;
		const authReady = $auth.initialized;
		const licValid = $licenseValid;

		if (!authReady) return;

		// Sin licencia válida → solo puede estar en /activate
		if (!licValid && !RUTAS_SIN_LICENCIA.includes(pathname)) {
			goto('/activate');
			return;
		}

		// Con licencia válida no debe estar en /activate
		if (licValid && RUTAS_SIN_LICENCIA.includes(pathname)) {
			goto(authed ? (configured ? '/' : '/setup') : '/login');
			return;
		}

		if (!authed && !PUBLIC_ROUTES.includes(pathname)) {
			goto('/login');
			return;
		}

		if (authed && PUBLIC_ROUTES.includes(pathname)) {
			goto(configured ? '/' : '/setup');
			return;
		}

		if (authed && !configured && pathname !== '/setup') {
			tenantStore.load().then((tenant) => {
				if (!tenant || !tenant.configurado) {
					goto('/setup');
				}
			});
		}
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

{@render children()}
