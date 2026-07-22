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

<div class="dashboard-root">
	<div class="bg-grid" aria-hidden="true"></div>

	<div class="content">
		<div class="welcome-card">
			<div class="logo">
				<svg width="40" height="40" viewBox="0 0 48 48" fill="none">
					<rect width="48" height="48" rx="12" fill="url(#dg)" />
					<path d="M14 10h14l-4 11h8L18 38l4-14h-8l0-14z" fill="white" stroke="white" stroke-width="0.5" stroke-linejoin="round" />
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
				<p>El módulo de emisión de comprobantes está en desarrollo.<br />
				La base de seguridad y autenticación está completa.</p>
			</div>

			<button
				id="logout-btn"
				type="button"
				class="btn-logout"
				onclick={() => auth.signOut()}
			>
				Cerrar sesión
			</button>
		</div>
	</div>
</div>

<style>
	:global(body) { margin: 0; padding: 0; font-family: 'Inter', system-ui, sans-serif; background: #0a0d14; }

	.dashboard-root {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		position: relative;
	}

	.bg-grid {
		position: fixed;
		inset: 0;
		background-image:
			linear-gradient(rgba(59, 130, 246, 0.05) 1px, transparent 1px),
			linear-gradient(90deg, rgba(59, 130, 246, 0.05) 1px, transparent 1px);
		background-size: 40px 40px;
		pointer-events: none;
	}

	.content { position: relative; z-index: 1; width: 100%; max-width: 560px; }

	.welcome-card {
		background: rgba(255,255,255,0.03);
		border: 1px solid rgba(255,255,255,0.07);
		border-radius: 20px;
		padding: 2.5rem;
		backdrop-filter: blur(20px);
		box-shadow: 0 32px 64px -16px rgba(0,0,0,0.5);
		text-align: center;
	}

	.logo {
		display: inline-flex;
		align-items: center;
		gap: 0.625rem;
		margin-bottom: 1.5rem;
	}

	.logo-text { font-size: 1.5rem; font-weight: 800; color: #f1f5f9; }

	.welcome-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
		letter-spacing: -0.02em;
	}

	.welcome-sub {
		font-size: 0.875rem;
		color: #64748b;
		margin: 0 0 1.5rem 0;
	}

	.status-badge {
		color: #10b981;
		font-weight: 500;
	}

	.tenant-info {
		display: flex;
		gap: 0.5rem;
		justify-content: center;
		flex-wrap: wrap;
		margin-bottom: 1.5rem;
	}

	.info-pill {
		background: rgba(59,130,246,0.1);
		border: 1px solid rgba(59,130,246,0.2);
		border-radius: 100px;
		padding: 0.25rem 0.75rem;
		font-size: 0.8125rem;
		color: #60a5fa;
		font-weight: 500;
	}

	.coming-soon {
		background: rgba(255,255,255,0.02);
		border: 1px solid rgba(255,255,255,0.06);
		border-radius: 12px;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.cs-icon { font-size: 2rem; margin-bottom: 0.75rem; }

	.coming-soon p {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
		line-height: 1.6;
	}

	.btn-logout {
		background: rgba(239,68,68,0.08);
		border: 1px solid rgba(239,68,68,0.2);
		border-radius: 10px;
		color: #fca5a5;
		font-size: 0.875rem;
		font-weight: 500;
		font-family: 'Inter', sans-serif;
		padding: 0.625rem 1.25rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-logout:hover {
		background: rgba(239,68,68,0.14);
		border-color: rgba(239,68,68,0.35);
	}
</style>
