<script lang="ts">
	import { goto } from '$app/navigation';
	import { licenseStore } from '$lib/stores/license';

	let licenseKey = $state('');
	let inputRef: HTMLInputElement | null = null;

	// Formato automático TAXO-XXXX-XXXX-XXXX mientras escribe
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
		if (licenseKey.length < 19) return; // TAXO-XXXX-XXXX-XXXX = 19 chars
		const ok = await licenseStore.activar(licenseKey);
		if (ok) {
			goto('/setup');
		}
	}

	function onKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') activar();
	}
</script>

<svelte:head>
	<title>Taxorium — Activar Licencia</title>
</svelte:head>

<div class="min-h-screen flex items-center justify-center p-6 bg-slate-950 text-slate-200 relative overflow-hidden">
	<!-- Fondo de rejilla -->
	<div
		class="fixed inset-0 pointer-events-none z-0"
		style="background-image: linear-gradient(rgba(59,130,246,0.05) 1px, transparent 1px), linear-gradient(90deg, rgba(59,130,246,0.05) 1px, transparent 1px); background-size: 40px 40px;"
	></div>

	<!-- Glow central -->
	<div class="fixed inset-0 pointer-events-none z-0 flex items-center justify-center">
		<div class="w-96 h-96 rounded-full bg-blue-600/10 blur-3xl"></div>
	</div>

	<div class="relative z-10 w-full max-w-md">
		<!-- Logo / Marca -->
		<div class="text-center mb-10">
			<div class="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-blue-500/10 border border-blue-500/20 mb-4">
				<span class="text-3xl">🔑</span>
			</div>
			<h1 class="text-3xl font-extrabold tracking-tight text-slate-100">Activar Taxorium</h1>
			<p class="text-slate-400 text-sm mt-2">Ingresa la license key que recibiste para desbloquear el sistema</p>
		</div>

		<!-- Card -->
		<div class="bg-white/5 border border-white/10 rounded-2xl p-8 backdrop-blur-xl shadow-2xl">
			<label for="license-key-input" class="block text-sm font-semibold text-slate-300 mb-3">
				License Key
			</label>

			<input
				id="license-key-input"
				bind:this={inputRef}
				type="text"
				value={licenseKey}
				oninput={onInput}
				onkeydown={onKeydown}
				placeholder="TAXO-XXXX-XXXX-XXXX"
				maxlength={19}
				spellcheck={false}
				autocomplete="off"
				class="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-center text-lg font-mono tracking-widest text-slate-100 placeholder-slate-600 focus:outline-none focus:border-blue-500/60 focus:ring-2 focus:ring-blue-500/20 transition-all duration-200"
				class:border-green-500={licenseKey.length === 19}
				class:ring-green-500={licenseKey.length === 19}
				disabled={$licenseStore.activating}
			/>

			<!-- Indicador de longitud -->
			<div class="flex justify-between items-center mt-2 mb-6">
				<span class="text-xs text-slate-600">Formato: TAXO-XXXX-XXXX-XXXX</span>
				<span class="text-xs {licenseKey.length === 19 ? 'text-green-400' : 'text-slate-600'}">
					{licenseKey.length}/19
				</span>
			</div>

			<!-- Error -->
			{#if $licenseStore.error}
				<div class="flex items-center gap-2 bg-red-500/10 border border-red-500/20 rounded-xl px-4 py-3 text-red-400 text-sm mb-5" role="alert">
					<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
						<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5"/>
						<path d="M8 5v4M8 11v.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
					</svg>
					{$licenseStore.error}
				</div>
			{/if}

			<!-- Botón activar -->
			<button
				id="btn-activar-licencia"
				type="button"
				onclick={activar}
				disabled={$licenseStore.activating || licenseKey.length < 19}
				class="w-full py-3 px-6 rounded-xl font-semibold text-sm bg-blue-600 hover:bg-blue-500 text-white shadow-lg shadow-blue-500/30 hover:shadow-xl hover:shadow-blue-500/40 hover:-translate-y-0.5 disabled:opacity-40 disabled:cursor-not-allowed disabled:hover:translate-y-0 transition-all duration-200"
			>
				{#if $licenseStore.activating}
					<span class="inline-flex items-center gap-2">
						<span class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
						Verificando con el servidor...
					</span>
				{:else}
					🚀 Activar licencia
				{/if}
			</button>
		</div>

		<!-- Pie -->
		<p class="text-center text-xs text-slate-600 mt-6">
			¿No tienes una licencia? Contacta con el administrador del sistema.
		</p>
	</div>
</div>
