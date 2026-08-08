<script lang="ts">
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';
	import Option from '$lib/components/core/primitives/Select.svelte';
	import { IconLock, IconUser, IconEyeOff, IconEye } from '@tabler/icons-svelte';
	import { sellerClient, type CreateSellerPayload } from '$lib/services/sellers/clientSellers';
	import { sellersStore, editingSeller, sellerPasswordCache } from '$lib/stores/sellers';
	import { taxoLog } from '$lib/stores/taxoLog';

	const MODULOS = [
		{ value: 'sales', label: 'Módulo de Ventas' },
		{ value: 'shop', label: 'Módulo de Compras' },
		{ value: 'products', label: 'Módulo de Productos' }
	];
	const DOMINIOS = [
		{ value: 'CAJA-1', label: 'CAJA-1' },
		{ value: 'CAJA-2', label: 'CAJA-2' },
		{ value: 'CAJA-3', label: 'CAJA-3' }
	];

	let mostrarPassword = $state(false);
	let loading = $state(false);
	let error = $state('');
	let successMsg = $state('');

	// Form state
	let nombres = $state('');
	let apellidos = $state('');
	let usuario = $state('');
	let clavePlana = $state('');
	let accesosSeleccionados = $state<string>('');
	let dominioSeleccionado = $state<string>('');

	// Sync editing state
	$effect(() => {
		if ($editingSeller) {
			nombres = $editingSeller.nombres || '';
			apellidos = $editingSeller.apellidos || '';
			usuario = $editingSeller.usuario;
			clavePlana = '';
			accesosSeleccionados = $editingSeller.accesos ?? '';
			dominioSeleccionado = $editingSeller.dominio ?? '';
			successMsg = '';
			error = '';
		} else {
			resetForm();
		}
	});

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!usuario.trim()) {
			error = 'El usuario es requerido';
			return;
		}
		if (!$editingSeller && !clavePlana.trim()) {
			error = 'La contraseña es requerida';
			return;
		}

		error = '';
		successMsg = '';
		loading = true;

		try {
			if ($editingSeller) {
				const editId = $editingSeller.id;
				await sellerClient.updateSeller(editId, {
					nombres: nombres || undefined,
					apellidos: apellidos || undefined,
					usuario,
					clave_plana: clavePlana || undefined,
					accesos: accesosSeleccionados || undefined,
					dominio: (!dominioSeleccionado || dominioSeleccionado === 'null') ? undefined : dominioSeleccionado
				});
				// Update password cache if a new password was provided
				if (clavePlana.trim()) {
					sellerPasswordCache.update(cache => ({ ...cache, [editId]: clavePlana }));
				}
				successMsg = 'Vendedor actualizado correctamente';
			} else {
				const payload: CreateSellerPayload = {
					nombres: nombres || undefined,
					apellidos: apellidos || undefined,
					usuario,
					clave_plana: clavePlana,
					accesos: accesosSeleccionados || undefined,
					dominio: (!dominioSeleccionado || dominioSeleccionado === 'null') ? undefined : dominioSeleccionado
				};
				const newSeller = await sellerClient.createSeller(payload);
				// Store password in memory cache for display in table
				sellerPasswordCache.update(cache => ({ ...cache, [newSeller.id]: clavePlana }));
				successMsg = 'Vendedor registrado correctamente';
			}
			$editingSeller = null;
			resetForm();
			await sellersStore.load();
		} catch (e: any) {
			const msg = e?.message ?? 'Error al guardar el vendedor';
			error = msg;
			taxoLog.error(msg, 'vendedores');
		} finally {
			loading = false;
		}
	}

	function resetForm() {
		nombres = '';
		apellidos = '';
		usuario = '';
		clavePlana = '';
		accesosSeleccionados = '';
		dominioSeleccionado = '';
	}
</script>

<div class="grid p-2">
	<h1 class="p-2 font-bold text-neutral-400">
		{$editingSeller ? 'Editar Vendedor' : 'Registrar nuevo vendedor o similar'}
	</h1>

	<form
		onsubmit={handleSubmit}
		class="grid grid-cols-2 gap-2 rounded-md border border-neutral-800 p-2"
	>
		<div class="flex flex-col">
			<Input label="Nombres" bind:value={nombres} />
			<Input label="Apellidos" bind:value={apellidos} />

			<Input label="Usuario" variant="triple" bind:value={usuario} placeholder="nombre@correo.com">
				{#snippet icon()}<IconUser size={18} />{/snippet}
			</Input>

			<div class="mt-2">
				{#if $editingSeller}
					<Button type="button" onclick={() => ($editingSeller = null)}>Cancelar</Button>
				{/if}
				<Button type="submit" disabled={loading}>
					{loading ? 'Guardando...' : $editingSeller ? 'Actualizar' : 'Guardar'}
				</Button>
			</div>
		</div>

		<div class="flex flex-col">
			<Option label="Asignar Accesos" options={MODULOS} bind:value={accesosSeleccionados} />
			<Option label="Asignar Dominio / Puesto" options={DOMINIOS} bind:value={dominioSeleccionado} />

			<Input
				label="Contraseña{$editingSeller ? ' (dejar vacío para no cambiar)' : ''}"
				variant="triple"
				type={mostrarPassword ? 'text' : 'password'}
				bind:value={clavePlana}
				placeholder={$editingSeller ? 'Sin cambios' : '••••••••'}
			>
				{#snippet icon()}<IconLock size={18} />{/snippet}
				{#snippet action()}
					<button
						type="button"
						class="flex items-center justify-center p-2 hover:text-neutral-200 focus:outline-none"
						onclick={() => (mostrarPassword = !mostrarPassword)}
					>
						{#if mostrarPassword}
							<IconEyeOff size={18} stroke={1.5} />
						{:else}
							<IconEye size={18} stroke={1.5} />
						{/if}
					</button>
				{/snippet}
			</Input>
		</div>
	</form>
	<div class="mt-2">
		{#if error}
			<p class="rounded border border-red-500/20 bg-red-500/10 px-3 py-2 text-xs text-red-400">
				{error}
			</p>
		{/if}
		{#if successMsg}
			<p
				class="rounded border border-emerald-500/20 bg-emerald-500/10 px-3 py-2 text-xs text-emerald-400"
			>
				{successMsg}
			</p>
		{/if}
	</div>
</div>
