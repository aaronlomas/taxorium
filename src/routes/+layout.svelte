<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { auth, isAuthenticated } from '$lib/stores/auth';
	import { tenantStore, isConfigured } from '$lib/stores/tenant';

	let { children } = $props();

	// Rutas que no requieren autenticación
	const PUBLIC_ROUTES = ['/login', '/register'];

	onMount(async () => {
		await auth.init();

		// Escuchar cambios reactivos para redireccionar
	});

	// Guard reactivo
	$effect(() => {
		const pathname = page.url.pathname;
		const authed = $isAuthenticated;
		const configured = $isConfigured;
		const authReady = $auth.initialized;

		if (!authReady) return;

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
