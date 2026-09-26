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
	<h2 class="mb-1 text-xl font-bold tracking-tight text-slate-100">Datos de tu empresa</h2>
	<p class="mb-7 text-sm leading-relaxed text-slate-400">
		Esta información aparecerá en todos tus comprobantes electrónicos.
	</p>

	<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
		<div class="md:col-span-2">
			<Input
				id="ruc"
				type="text"
				bind:value={ruc}
				label="RUC *"
				placeholder="20123456789"
				maxlength={11}
				class="rounded-xl border bg-white/5 text-slate-100 transition-all {rucValid === true
					? 'border-emerald-500/50'
					: rucValid === false
						? 'border-red-500/50'
						: 'border-white/10 focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20'}"
				onblur={validateRuc}
				oninput={() => {
					rucValid = null;
				}}
				variant={rucValidating || rucValid !== null ? 'double' : 'simple'}
			>
				{#snippet action()}
					{#if rucValidating}
						<span
							class="inline-block h-3 w-3 animate-spin rounded-full border-2 border-white/20 border-t-blue-500"
						></span>
					{:else if rucValid === true}
						<span class="text-sm text-emerald-500">✓</span>
					{:else if rucValid === false}
						<span class="text-sm text-red-500">✗</span>
					{/if}
				{/snippet}
			</Input>
		</div>

		<div class="md:col-span-2">
			<Input
				id="razon-social"
				type="text"
				bind:value={razonSocial}
				label="Razón social *"
				placeholder="EMPRESA SAC"
				class="rounded-xl border border-white/10 bg-white/5 text-slate-100 transition-all focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20"
			/>
		</div>

		<Input
			id="nombre-comercial"
			type="text"
			bind:value={nombreComercial}
			label="Nombre comercial"
			placeholder="Mi Tienda (opcional)"
			class="rounded-xl border border-white/10 bg-white/5 text-slate-100 transition-all focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20"
		/>

		<Input
			id="departamento"
			type="text"
			bind:value={departamento}
			label="Departamento"
			placeholder="Lima"
			class="rounded-xl border border-white/10 bg-white/5 text-slate-100 transition-all focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20"
		/>

		<div class="md:col-span-2">
			<Input
				id="direccion"
				type="text"
				bind:value={direccion}
				label="Dirección fiscal *"
				placeholder="Av. Javier Prado Este 123, San Isidro"
				class="rounded-xl border border-white/10 bg-white/5 text-slate-100 transition-all focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20"
			/>
		</div>

		<Input
			id="telefono-empresa"
			type="tel"
			bind:value={telefono}
			label="Teléfono"
			placeholder="01-234-5678"
			class="rounded-xl border border-white/10 bg-white/5 text-slate-100 transition-all focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20"
		/>

		<Input
			id="email-empresa"
			type="email"
			bind:value={email}
			label="Email de empresa"
			placeholder="contacto@empresa.com"
			class="rounded-xl border border-white/10 bg-white/5 text-slate-100 transition-all focus-within:border-blue-500/50 focus-within:ring-2 focus-within:ring-blue-500/20"
		/>
	</div>
</div>
