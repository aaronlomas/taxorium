<script lang="ts">
	import type { Snippet } from 'svelte';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';

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
		validateRuc,
		onSave,
		saving = false,
		submitLabel = 'Guardar cambios',
		savingLabel = 'Guardando...',
		messages
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
		/** Acción a ejecutar al enviar el formulario. */
		onSave: () => void;
		/** Bloquea el botón mientras la acción se ejecuta. */
		saving?: boolean;
		submitLabel?: string;
		savingLabel?: string;
		/** Contenido opcional a renderizar justo encima del botón (mensajes de error/éxito). */
		messages?: Snippet;
	} = $props();

	function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		if (saving) return;
		onSave();
	}
</script>

<form class="flex flex-col gap-4 border border-neutral-900 p-2 rounded-md" onsubmit={handleSubmit}>
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
		<div class="md:col-span-2">
			<Input
				id="ruc"
				type="text"
				bind:value={ruc}
				label="RUC"
				placeholder="20123456789"
				maxlength={11}
				required
				class=" {rucValid === true
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
				label="Razón social"
				placeholder="EMPRESA SAC"
				required
			/>
		</div>

		<Input
			id="nombre-comercial"
			type="text"
			bind:value={nombreComercial}
			label="Nombre comercial"
			placeholder="Mi Tienda (opcional)"
		/>

		<Input
			id="departamento"
			type="text"
			bind:value={departamento}
			label="Departamento"
			placeholder="Lima"
			required
		/>

		<div class="md:col-span-2">
			<Input
				id="direccion"
				type="text"
				bind:value={direccion}
				label="Dirección fiscal"
				placeholder="Av. Javier Prado Este 123, San Isidro"
				required
			/>
		</div>

		<Input
			id="telefono-empresa"
			type="tel"
			bind:value={telefono}
			label="Teléfono"
			placeholder="01-234-5678"
		/>

		<Input
			id="email-empresa"
			type="email"
			bind:value={email}
			label="Email de empresa"
			placeholder="contacto@empresa.com"
		/>
	</div>

	<!-- MENSAJES / ACCIONES -->
	{#if messages}
		<div class="mt-4">
			{@render messages()}
		</div>
	{/if}

	<div class="flex justify-end">
		<Button type="submit" variant="primary" disabled={saving} class="px-6 py-2.5">
			{#if saving}
				<span
					class="mr-2 inline-block h-4 w-4 animate-spin rounded-full border-2 border-white/20 border-t-white"
				></span>
				{savingLabel}
			{:else}
				{submitLabel}
			{/if}
		</Button>
	</div>
</form>
