<script lang="ts">
	import { auth, currentUser } from '$lib/stores/auth';
	import { currentTenant, tenantStore } from '$lib/stores/tenant';
	import { licenseStore, licenseExpiry } from '$lib/stores/license';
	import {
		IconLogout,
		IconBuildingStore,
		IconKey,
		IconMail,
		IconBuilding
	} from '@tabler/icons-svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';

	let signingOut = $state(false);

	async function handleSignOut() {
		signingOut = true;
		licenseStore.limpiar();
		tenantStore.clear();
		await auth.signOut();
		signingOut = false;
	}

	function formatDate(date: Date | null): string {
		if (!date) return '—';
		return date.toLocaleDateString('es-PE', { day: '2-digit', month: 'short', year: 'numeric' });
	}
</script>

<div class="panel-perfil">
	<!-- Empresa card -->
	{#if $currentTenant}
		<div class="empresa-card">
			<div class="empresa-avatar">
				{($currentTenant.razon_social ?? 'E').charAt(0).toUpperCase()}
			</div>
			<div class="empresa-info">
				<p class="empresa-nombre">{$currentTenant.razon_social}</p>
				{#if $currentTenant.nombre_comercial}
					<p class="empresa-comercial">{$currentTenant.nombre_comercial}</p>
				{/if}
				<p class="empresa-ruc">RUC {$currentTenant.ruc}</p>
			</div>
		</div>
	{/if}

	<!-- Datos de la sesión -->
	<div class="info-section">
		<p class="section-label">Cuenta</p>
		<div class="info-row">
			<IconMail size={14} class="info-icon" />
			<span class="info-value">{$currentUser?.email ?? '—'}</span>
		</div>
	</div>

	{#if $currentTenant}
		<div class="info-section">
			<p class="section-label">Empresa</p>
			{#if $currentTenant.direccion}
				<div class="info-row">
					<IconBuilding size={14} class="info-icon" />
					<span class="info-value">{$currentTenant.direccion}</span>
				</div>
			{/if}
		</div>
	{/if}

	<!-- Licencia -->
	<div class="info-section">
		<p class="section-label">Licencia</p>
		<div class="license-badge">
			<div class="license-dot"></div>
			<div class="license-text">
				<span class="license-status">Activa</span>
				{#if $licenseExpiry}
					<span class="license-expiry">Vence {formatDate($licenseExpiry)}</span>
				{/if}
			</div>
			<IconKey size={14} class="info-icon" style="margin-left:auto" />
		</div>
	</div>
</div>

<style>
	.panel-perfil {
		display: flex;
		flex-direction: column;
		gap: 0.875rem;
		padding: 1.25rem;
	}

	/* Empresa card */
	.empresa-card {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		background: rgba(59, 130, 246, 0.05);
		border: 1px solid rgba(59, 130, 246, 0.12);
		border-radius: 10px;
		padding: 0.875rem;
	}
	.empresa-avatar {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 8px;
		background: linear-gradient(135deg, #3b82f6, #06b6d4);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.125rem;
		font-weight: 800;
		color: #fff;
		flex-shrink: 0;
	}
	.empresa-info {
		min-width: 0;
	}
	.empresa-nombre {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.1rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}
	.empresa-comercial {
		font-size: 0.725rem;
		color: #94a3b8;
		margin: 0 0 0.1rem;
	}
	.empresa-ruc {
		font-size: 0.7rem;
		color: #64748b;
		margin: 0;
		font-family: 'Courier New', monospace;
	}

	/* Info sections */
	.info-section {
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
	}
	.section-label {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.07em;
		color: #475569;
		margin: 0;
	}
	.info-row {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
	}
	:global(.info-icon) {
		color: #64748b;
		flex-shrink: 0;
		margin-top: 1px;
	}
	.info-value {
		font-size: 0.775rem;
		color: #94a3b8;
		line-height: 1.4;
	}

	/* License badge */
	.license-badge {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: rgba(16, 185, 129, 0.06);
		border: 1px solid rgba(16, 185, 129, 0.15);
		border-radius: 8px;
		padding: 0.5rem 0.75rem;
	}
	.license-dot {
		width: 7px;
		height: 7px;
		border-radius: 50%;
		background: #10b981;
		box-shadow: 0 0 5px #10b981;
		animation: pulse-dot 2s ease-in-out infinite;
		flex-shrink: 0;
	}
	@keyframes pulse-dot {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.4;
		}
	}
	.license-text {
		display: flex;
		flex-direction: column;
		gap: 0.05rem;
	}
	.license-status {
		font-size: 0.775rem;
		font-weight: 600;
		color: #34d399;
	}
	.license-expiry {
		font-size: 0.6875rem;
		color: #6ee7b7;
		opacity: 0.7;
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
