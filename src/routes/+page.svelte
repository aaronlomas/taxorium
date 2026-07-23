<script lang="ts">
	import { auth, currentUser } from '$lib/stores/auth';
	import { tenantStore, currentTenant } from '$lib/stores/tenant';
	import { onMount } from 'svelte';

	onMount(async () => {
		if ($currentUser) {
			await tenantStore.load();
		}
	});
</script>

<svelte:head>
	<title>Taxorium — Dashboard</title>
	<link
		href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<div class="min-h-dvh flex justify-center items-center p-2">
	<div class="bg-grid" aria-hidden="true"></div>

	<div class="content">
		<div class="welcome-card">
			<div class="logo">
				<svg width="40" height="40" viewBox="0 0 48 48" fill="none">
					<rect width="48" height="48" rx="12" fill="url(#dg)" />
					<path
						d="M14 10h14l-4 11h8L18 38l4-14h-8l0-14z"
						fill="white"
						stroke="white"
						stroke-width="0.5"
						stroke-linejoin="round"
					/>
					<defs>
						<linearGradient id="dg" x1="0" y1="0" x2="48" y2="48" gradientUnits="userSpaceOnUse">
							<stop stop-color="#3b82f6" /><stop offset="1" stop-color="#06b6d4" />
						</linearGradient>
					</defs>
				</svg>
				<span class="logo-text">Taxorium</span>
			</div>

			<h1 class="welcome-title">
				Bienvenido{$currentTenant ? ', ' + $currentTenant.razon_social : ''}
			</h1>
			<p class="welcome-sub">
				{$currentUser?.email} · <span class="status-badge">✓ Sesión activa</span>
			</p>

			{#if $currentTenant}
				<div class="tenant-info">
					<div class="info-pill">RUC {$currentTenant.ruc}</div>
					<div class="info-pill">Serie boleta: {$currentTenant.serie_boleta}</div>
					<div class="info-pill">Serie factura: {$currentTenant.serie_factura}</div>
				</div>
			{/if}

			<div class="coming-soon">
				<div class="cs-icon">🚀</div>
				<p>
					El módulo de emisión de comprobantes está en desarrollo.<br />
					La base de seguridad y autenticación está completa.
				</p>
			</div>

			<button id="logout-btn" type="button" class="btn-logout" onclick={() => auth.signOut()}>
				Cerrar sesión
			</button>
		</div>
	</div>
</div>