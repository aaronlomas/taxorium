<script lang="ts">
	import { licenseStore } from '$lib/stores/license';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';

	let licenseKey = $state('');

	function formatearKey(valor: string) {
		const limpio = valor.replace(/[^A-Z0-9]/gi, '').toUpperCase();
		const partes = limpio.match(/.{1,4}/g) ?? [];
		return partes.slice(0, 4).join('-');
	}

	function onInput(e: Event) {
		const target = e.target as HTMLInputElement;
		licenseKey = formatearKey(target.value);
	}

	async function activar() {
		if (licenseKey.length < 19) return;
		await licenseStore.activar(licenseKey);
		// El AccountPanel reaccionará automáticamente al cambio de licenseValid
	}

	function onKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') activar();
	}
</script>

<div class="panel-activar">
	<!-- Header -->
	<div class="activar-header">
		<div class="activar-icon-wrap">
			<span class="activar-icon">🔑</span>
		</div>
		<div>
			<h2 class="activar-title">Activar Taxorium</h2>
			<p class="activar-subtitle">Ingresa la license key para desbloquear el sistema</p>
		</div>
	</div>

	<!-- Input de licencia -->
	<div class="key-section">
		<Input
			id="pa-license-key"
			type="text"
			value={licenseKey}
			oninput={onInput}
			onkeydown={onKeydown}
			label="License Key"
			placeholder="TAXO-XXXX-XXXX-XXXX"
			maxlength={19}
			spellcheck={false}
			autocomplete="off"
			class="key-input {licenseKey.length === 19 ? 'key-valid' : ''}"
			disabled={$licenseStore.activating}
		/>
		<div class="key-meta">
			<span class="key-format">Formato: TAXO-XXXX-XXXX-XXXX</span>
			<span class="key-count" class:key-count-done={licenseKey.length === 19}>
				{licenseKey.length}/19
			</span>
		</div>
	</div>

	<!-- Progreso visual -->
	<div class="key-progress-track">
		<div
			class="key-progress-fill"
			style="width: {Math.min(100, (licenseKey.length / 19) * 100)}%"
		></div>
	</div>

	{#if $licenseStore.error}
		<div class="alert-error" role="alert">
			<svg
				width="14"
				height="14"
				viewBox="0 0 16 16"
				fill="none"
				style="flex-shrink:0;margin-top:1px"
			>
				<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5" />
				<path d="M8 5v4M8 11v.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
			</svg>
			{$licenseStore.error}
		</div>
	{/if}

	<Button
		id="pa-btn-activar"
		type="button"
		onclick={activar}
		disabled={$licenseStore.activating || licenseKey.length < 19}
		variant="primary"
		class="mt-2 w-full"
	>
		{#if $licenseStore.activating}
			<span class="spinner"></span>
			Verificando con el servidor...
		{:else}
			🚀 Activar licencia
		{/if}
	</Button>

	<p class="activar-footer">¿No tienes una licencia? Contacta con el administrador del sistema.</p>
</div>

<style>
	.panel-activar {
		display: flex;
		flex-direction: column;
		gap: 1.125rem;
		padding: 1.25rem;
	}

	.activar-header {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.06);
	}
	.activar-icon-wrap {
		width: 2.25rem;
		height: 2.25rem;
		border-radius: 10px;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.2);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}
	.activar-icon {
		font-size: 1.125rem;
	}
	.activar-title {
		font-size: 0.9375rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.15rem;
		letter-spacing: -0.02em;
	}
	.activar-subtitle {
		font-size: 0.7rem;
		color: #64748b;
		margin: 0;
	}

	.key-section {
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
	}
	:global(.key-input) {
		text-align: center;
		font-size: 0.9375rem;
		font-family: 'Courier New', monospace;
		letter-spacing: 0.15em;
	}
	:global(.key-input)::placeholder {
		letter-spacing: 0.05em;
	}
	:global(.key-valid) {
		border-color: rgba(16, 185, 129, 0.5) !important;
		box-shadow: 0 0 0 2px rgba(16, 185, 129, 0.1) !important;
	}
	.key-meta {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.key-format {
		font-size: 0.6875rem;
		color: #475569;
	}
	.key-count {
		font-size: 0.6875rem;
		color: #475569;
		transition: color 0.2s;
	}
	.key-count-done {
		color: #34d399;
	}

	.key-progress-track {
		height: 3px;
		background: rgba(255, 255, 255, 0.06);
		border-radius: 999px;
		overflow: hidden;
	}
	.key-progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #06b6d4);
		border-radius: 999px;
		transition: width 0.2s ease;
	}

	.alert-error {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		background: rgba(239, 68, 68, 0.08);
		border: 1px solid rgba(239, 68, 68, 0.2);
		color: #fca5a5;
		padding: 0.6rem 0.75rem;
		border-radius: 8px;
		font-size: 0.75rem;
		line-height: 1.5;
	}

	.spinner {
		width: 13px;
		height: 13px;
		border: 2px solid rgba(255, 255, 255, 0.25);
		border-top-color: #fff;
		border-radius: 50%;
		animation: spin 0.65s linear infinite;
		flex-shrink: 0;
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.activar-footer {
		text-align: center;
		font-size: 0.6875rem;
		color: #475569;
		margin: 0;
	}
</style>
