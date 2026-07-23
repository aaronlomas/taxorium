<script lang="ts">
	import Input from '$lib/components/core/primitives/Input.svelte';

	let {
		ruc = $bindable(),
		razonSocial = $bindable(),
		nombreComercial = $bindable(),
		departamento = $bindable(),
		direccion = $bindable(),
		telefono = $bindable(),
		email = $bindable(),
		rucValid = $bindable(),
		rucValidating = $bindable(),
		validateRuc
	}: {
		ruc: string;
		razonSocial: string;
		nombreComercial: string;
		departamento: string;
		direccion: string;
		telefono: string;
		email: string;
		rucValid: boolean | null;
		rucValidating: boolean;
		validateRuc: () => void;
	} = $props();
</script>

<div class="mb-6">
	<h2 class="text-xl font-bold text-slate-100 mb-1 tracking-tight">Datos de tu empresa</h2>
	<p class="text-sm text-slate-400 mb-7 leading-relaxed">Esta información aparecerá en todos tus comprobantes electrónicos.</p>

	<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
		<div class="md:col-span-2">
			<Input
				id="ruc"
				type="text"
				bind:value={ruc}
				label="RUC *"
				placeholder="20123456789"
				maxlength={11}
				class="border rounded-xl bg-white/5 transition-all text-slate-100 {rucValid === true ? 'border-emerald-500/50' : rucValid === false ? 'border-red-500/50' : 'border-white/10 focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20'}"
				onblur={validateRuc}
				oninput={() => { rucValid = null; }}
				variant={rucValidating || rucValid !== null ? "double" : "simple"}
			>
				{#snippet action()}
					{#if rucValidating}
						<span class="inline-block w-3 h-3 border-2 border-white/20 border-t-blue-500 rounded-full animate-spin"></span>
					{:else if rucValid === true}
						<span class="text-emerald-500 text-sm">✓</span>
					{:else if rucValid === false}
						<span class="text-red-500 text-sm">✗</span>
					{/if}
				{/snippet}
			</Input>
		</div>

		<div class="md:col-span-2">
			<Input id="razon-social" type="text" bind:value={razonSocial} label="Razón social *" placeholder="EMPRESA SAC" class="border border-white/10 rounded-xl bg-white/5 transition-all text-slate-100 focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20" />
		</div>

		<Input id="nombre-comercial" type="text" bind:value={nombreComercial} label="Nombre comercial" placeholder="Mi Tienda (opcional)" class="border border-white/10 rounded-xl bg-white/5 transition-all text-slate-100 focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20" />
		
		<Input id="departamento" type="text" bind:value={departamento} label="Departamento" placeholder="Lima" class="border border-white/10 rounded-xl bg-white/5 transition-all text-slate-100 focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20" />

		<div class="md:col-span-2">
			<Input id="direccion" type="text" bind:value={direccion} label="Dirección fiscal *" placeholder="Av. Javier Prado Este 123, San Isidro" class="border border-white/10 rounded-xl bg-white/5 transition-all text-slate-100 focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20" />
		</div>

		<Input id="telefono-empresa" type="tel" bind:value={telefono} label="Teléfono" placeholder="01-234-5678" class="border border-white/10 rounded-xl bg-white/5 transition-all text-slate-100 focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20" />
		
		<Input id="email-empresa" type="email" bind:value={email} label="Email de empresa" placeholder="contacto@empresa.com" class="border border-white/10 rounded-xl bg-white/5 transition-all text-slate-100 focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20" />
	</div>
</div>
