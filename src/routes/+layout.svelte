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
	const RUTAS_SIN_LICENCIA = ['/activate', '/setup'];

	onMount(async () => {
		// Inicializar licencia (lee lease local y renueva silenciosamente si hay internet)
		licenseStore.init();
		await auth.init();
		// Cargar datos del tenant si el usuario ya está autenticado
		if ($auth.user) {
			await tenantStore.load();
		}
	});

	// Guard reactivo
	$effect(() => {
		const pathname = page.url.pathname;
		const authed = $isAuthenticated;
		const configured = $isConfigured;
		const authReady = $auth.initialized;
		const licValid = $licenseValid;
		const licenseReady = $licenseStore.initialized;
		const tenantCargando = $tenantStore.loading;

		// Esperar a que tanto auth como licencia estén inicializados
		if (!authReady || !licenseReady) return;
		if (authed && tenantCargando) return;

		const tenantExiste = !!$tenantStore.tenant;

		// 1. Si no está autenticado y no está en ruta pública -> login
		if (!authed && !PUBLIC_ROUTES.includes(pathname)) {
			goto('/login');
			return;
		}

		// 2. Si está autenticado y en ruta pública -> sacarlo de ahí
		if (authed && PUBLIC_ROUTES.includes(pathname)) {
			goto(tenantExiste ? (licValid ? '/' : '/activate') : '/setup');
			return;
		}

		// 3. Flujo para usuarios autenticados:
		if (authed) {
			if (!tenantExiste) {
				// Si no tiene tenant, FORZAR a /setup
				if (pathname !== '/setup') goto('/setup');
				return;
			}

			// Si tiene tenant, pero no tiene licencia válida, FORZAR a /activate
			if (!licValid) {
				if (pathname !== '/activate') goto('/activate');
				return;
			}

			// Si tiene tenant y licencia válida, no debe estar ni en /setup ni en /activate
			if (licValid && RUTAS_SIN_LICENCIA.includes(pathname)) {
				goto('/');
				return;
			}
		}
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

{@render children()}
