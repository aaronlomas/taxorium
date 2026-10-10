<script lang="ts">
	import { IconKey } from '@tabler/icons-svelte';
	import { licenseStore } from '$lib/features/license';
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

<div class="flex flex-col gap-4 p-3">
	<!-- Header -->
	<div class="flex items-start gap-3 border-b border-white/6 pb-3">
		<div
			class="flex size-9 shrink-0 items-center justify-center rounded-[10px] border border-blue-500/20 bg-blue-500/10"
		>
			<IconKey />
		</div>
		<div>
			<h2 class="mb-0.5 text-sm font-bold text-slate-100">Activar Taxorium</h2>
			<p class="text-xs text-slate-500">Ingresa la license key para desbloquear el sistema</p>
		</div>
	</div>

	<!-- Input de licencia -->
	<div class="flex flex-col gap-1">
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
			class="text-center font-mono text-sm [&_input::placeholder]:tracking-wider {licenseKey.length ===
			19
				? 'border-emerald-500/50! shadow-[0_0_0_2px_#10b9811a]!'
				: ''}"
			disabled={$licenseStore.activating}
		/>
		<div class="flex items-center justify-between">
			<span class="text-sm text-slate-600">Formato: TAXO-XXXX-XXXX-XXXX</span>
			<span
				class="text-xs transition-colors duration-200 {licenseKey.length === 19
					? 'text-emerald-400'
					: 'text-slate-600'}"
			>
				{licenseKey.length}/19
			</span>
		</div>
	</div>

	<!-- Progreso visual -->
	<div class="h-0.75 overflow-hidden rounded-full bg-white/6">
		<div
			class="h-full rounded-full bg-linear-to-r from-blue-500 to-cyan-500 transition-[width] duration-200"
			style="width: {Math.min(100, (licenseKey.length / 19) * 100)}%"
		></div>
	</div>

	{#if $licenseStore.error}
		<div
			role="alert"
			class="flex items-start gap-2 rounded-lg border border-red-500/20 bg-red-500/8 px-3 py-1.5 text-xs leading-relaxed text-red-300"
		>
			<svg width="14" height="14" viewBox="0 0 16 16" fill="none" class="mt-px shrink-0">
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
			<span
				class="size-3 shrink-0 animate-[spin_0.65s_linear_infinite] rounded-full border-2 border-white/25 border-t-white"
			></span>
			Verificando con el servidor...
		{:else}
			Activar licencia
		{/if}
	</Button>

	<p class="m-0 text-center text-[0.6875rem] text-slate-600">
		¿No tienes una licencia? Contacta con el administrador del sistema.
	</p>
</div>
