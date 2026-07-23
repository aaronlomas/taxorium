<script lang="ts">
	import Input from '$lib/components/core/primitives/Input.svelte';

	let {
		serieBoleta = $bindable(),
		serieFactura = $bindable()
	}: {
		serieBoleta: string;
		serieFactura: string;
	} = $props();
</script>

<div class="mb-6">
	<h2 class="text-xl font-bold text-slate-100 mb-1 tracking-tight">Configuración de series</h2>
	<p class="text-sm text-slate-400 mb-7 leading-relaxed">
		Define las series con las que iniciarás tu numeración. SUNAT requiere series distintas para
		boletas y facturas.
	</p>

	<div class="flex flex-col gap-3 mb-7">
		<div class="flex items-start gap-3 bg-blue-500/5 border border-blue-500/10 rounded-xl p-4">
			<span class="text-xl shrink-0">📋</span>
			<div class="flex flex-col gap-1">
				<strong class="text-slate-200 text-sm font-semibold">Boletas electrónicas</strong>
				<span class="text-slate-400 text-xs">Deben comenzar con <code class="bg-blue-500/15 text-blue-400 px-1 py-0.5 rounded font-mono">B</code> + 3 dígitos (ej: B001)</span>
			</div>
		</div>
		<div class="flex items-start gap-3 bg-blue-500/5 border border-blue-500/10 rounded-xl p-4">
			<span class="text-xl shrink-0">🏭</span>
			<div class="flex flex-col gap-1">
				<strong class="text-slate-200 text-sm font-semibold">Facturas electrónicas</strong>
				<span class="text-slate-400 text-xs">Deben comenzar con <code class="bg-blue-500/15 text-blue-400 px-1 py-0.5 rounded font-mono">F</code> + 3 dígitos (ej: F001)</span>
			</div>
		</div>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
		<div class="flex flex-col gap-1.5">
			<Input
				id="serie-boleta"
				type="text"
				bind:value={serieBoleta}
				label="Serie de boleta *"
				placeholder="B001"
				maxlength={4}
				class="border border-white/10 rounded-xl bg-white/5 transition-all text-slate-100 focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20"
				oninput={(e: Event) => { serieBoleta = (e.currentTarget as HTMLInputElement).value.toUpperCase(); }}
			/>
			<span class="text-xs text-slate-500 pl-1">El correlativo iniciará en B001-00000001</span>
		</div>

		<div class="flex flex-col gap-1.5">
			<Input
				id="serie-factura"
				type="text"
				bind:value={serieFactura}
				label="Serie de factura *"
				placeholder="F001"
				maxlength={4}
				class="border border-white/10 rounded-xl bg-white/5 transition-all text-slate-100 focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20"
				oninput={(e: Event) => { serieFactura = (e.currentTarget as HTMLInputElement).value.toUpperCase(); }}
			/>
			<span class="text-xs text-slate-500 pl-1">El correlativo iniciará en F001-00000001</span>
		</div>
	</div>
</div>
