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
		ubigeo = $bindable(),
		usuarioSol = $bindable(),
		claveSol = $bindable(),
		certificadoPath = $bindable(),
		certificadoPassword = $bindable(),
		rucValid = $bindable(),
		rucValidating = $bindable(),
		validateRuc,
		onSelectCertificado,
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
		ubigeo?: string;
		usuarioSol?: string;
		claveSol?: string;
		certificadoPath?: string;
		certificadoPassword?: string;
		rucValid: boolean | null;
		rucValidating: boolean;
		validateRuc: () => void;
		onSelectCertificado?: () => void;
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

<form class="flex flex-col gap-4 rounded-md border border-neutral-900 p-2" onsubmit={handleSubmit}>
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

		<Input
			id="ubigeo"
			type="text"
			bind:value={ubigeo}
			label="Ubigeo"
			placeholder="150101 (Código INEI de 6 dígitos)"
			maxlength={6}
		/>

		<!-- SECCIÓN CREDENCIALES SUNAT -->
		<div class="mt-4 border-t border-neutral-800 pt-6 md:col-span-2">
			<h3 class="mb-4 text-lg font-semibold text-slate-100">Credenciales SUNAT</h3>
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
				<Input
					id="usuario-sol"
					type="text"
					bind:value={usuarioSol}
					label="Usuario SOL"
					placeholder="MODDATOS"
				/>
				<Input
					id="clave-sol"
					type="password"
					bind:value={claveSol}
					label="Clave SOL"
					placeholder="••••••••"
				/>
				<div class="md:col-span-2">
					<label for="" class="mb-2 block text-sm font-medium text-neutral-300"
						>Certificado Digital (.p12)</label
					>
					<div class="flex gap-2">
						<Input
							id="certificado-path"
							type="text"
							bind:value={certificadoPath}
							placeholder="/ruta/al/certificado.p12"
							class="flex-1"
							readonly
						/>
						{#if onSelectCertificado}
							<Button type="button" variant="secondary" onclick={onSelectCertificado}>
								Examinar
							</Button>
						{/if}
					</div>
					<p class="mt-1 text-xs text-neutral-500">
						Selecciona el archivo .p12 de tu certificado digital para la firma de comprobantes.
					</p>
				</div>
				<Input
					id="certificado-password"
					type="password"
					bind:value={certificadoPassword}
					label="Contraseña del Certificado"
					placeholder="••••••••"
				/>
			</div>
		</div>
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
