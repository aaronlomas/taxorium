<script lang="ts">
	import Modal from '$lib/components/core/primitives/feedback/Modal.svelte';
	import Input from '$lib/components/core/primitives/forms/Input.svelte';
	import Select from '$lib/components/core/primitives/forms/Select.svelte';
	import Button from '$lib/components/core/primitives/actions/Button.svelte';

	import { vendorClient, type CreateVendorPayload } from '$lib/features/vendors';
	import { vendorsStore, editingVendor } from '$lib/features/vendors';
	import { taxoLog } from '$lib/features/taxoLog';
	import { IconSearch } from '@tabler/icons-svelte';

	let { isOpen = $bindable(false), onClose }: { isOpen: boolean; onClose: () => void } = $props();

	// Form State
	let tipo_documento = $state('RUC');
	let numero_documento = $state('');
	let nombre = $state('');
	let nombre_comercial = $state('');
	let pais = $state('PERU');
	let departamento = $state('');
	let provincia = $state('');
	let distrito = $state('');
	let direccion = $state('');
	let telefono = $state('');
	let correo = $state('');
	let contacto = $state('');
	let cuenta_bancaria = $state('');
	let observaciones = $state('');

	let error = $state('');
	let loading = $state(false);

	const tiposDoc = [
		{ value: 'RUC', label: 'RUC' },
		{ value: 'DNI', label: 'DNI' },
		{ value: 'CE', label: 'CE' },
		{ value: 'PASAPORTE', label: 'PASAPORTE' }
	];

	const paises = [{ value: 'PERU', label: 'PERU' }];

	// Sync with editingVendor
	$effect(() => {
		if ($editingVendor) {
			tipo_documento = $editingVendor.tipo_documento || 'RUC';
			numero_documento = $editingVendor.numero_documento || '';
			nombre = $editingVendor.nombre || '';
			nombre_comercial = $editingVendor.nombre_comercial || '';
			pais = $editingVendor.pais || 'PERU';
			departamento = $editingVendor.departamento || '';
			provincia = $editingVendor.provincia || '';
			distrito = $editingVendor.distrito || '';
			direccion = $editingVendor.direccion || '';
			telefono = $editingVendor.telefono || '';
			correo = $editingVendor.correo || '';
			contacto = $editingVendor.contacto || '';
			cuenta_bancaria = $editingVendor.cuenta_bancaria || '';
			observaciones = $editingVendor.observaciones || '';
			error = '';
		} else if (isOpen) {
			resetForm();
		}
	});

	function resetForm() {
		tipo_documento = 'RUC';
		numero_documento = '';
		nombre = '';
		nombre_comercial = '';
		pais = 'PERU';
		departamento = '';
		provincia = '';
		distrito = '';
		direccion = '';
		telefono = '';
		correo = '';
		contacto = '';
		cuenta_bancaria = '';
		observaciones = '';
		error = '';
	}

	async function handleSubmit() {
		if (!numero_documento.trim() || !nombre.trim()) {
			error = 'Número de documento y razón social/nombre son requeridos';
			return;
		}

		error = '';
		loading = true;

		try {
			const payload: CreateVendorPayload = {
				tipo_documento,
				numero_documento: numero_documento.trim(),
				nombre: nombre.trim(),
				nombre_comercial: nombre_comercial.trim() || undefined,
				pais: pais.trim() || undefined,
				departamento: departamento.trim() || undefined,
				provincia: provincia.trim() || undefined,
				distrito: distrito.trim() || undefined,
				direccion: direccion.trim() || undefined,
				telefono: telefono.trim() || undefined,
				correo: correo.trim() || undefined,
				contacto: contacto.trim() || undefined,
				cuenta_bancaria: cuenta_bancaria.trim() || undefined,
				observaciones: observaciones.trim() || undefined
			};

			if ($editingVendor) {
				await vendorClient.updateVendor($editingVendor.id, payload);
				taxoLog.info(`Proveedor '${payload.nombre}' actualizado`, 'proveedores');
			} else {
				await vendorClient.createVendor(payload);
				taxoLog.info(`Proveedor '${payload.nombre}' creado`, 'proveedores');
			}

			$editingVendor = null;
			resetForm();
			await vendorsStore.load();
			onClose();
		} catch (err: any) {
			const msg = typeof err === 'string' ? err : (err?.message ?? 'Error al guardar el proveedor');
			error = msg;
			taxoLog.error(msg, 'proveedores');
		} finally {
			loading = false;
		}
	}

	function handleCloseModal() {
		$editingVendor = null;
		resetForm();
		onClose();
	}
</script>

{#snippet sunatButton()}
	<button
		type="button"
		class="flex h-full items-center justify-center gap-1 border-l border-brand-border px-3 text-xs font-medium text-brand-text-secondary hover:text-brand-text-primary"
	>
		<IconSearch size={14} /> SUNAT
	</button>
{/snippet}

<Modal
	bind:isOpen
	onClose={handleCloseModal}
	title={$editingVendor ? 'Editar Proveedor' : 'Nuevo Proveedor'}
>
	<div class="flex w-150 flex-col gap-3 p-4">
		<div class="grid grid-cols-2 gap-4">
			<Select
				id="tipo_documento"
				label="Tipo Doc. Identidad"
				required
				bind:value={tipo_documento}
				options={tiposDoc}
			/>
			<Input
				id="numero_documento"
				label="Número"
				required
				variant="double"
				action={sunatButton}
				bind:value={numero_documento}
				placeholder="99999"
			/>
		</div>

		<div class="grid grid-cols-2 gap-4">
			<Input id="nombre" label="Razón Social / Nombre" required variant="simple" bind:value={nombre} />
			<Input
				id="nombre_comercial"
				label="Nombre comercial"
				variant="simple"
				bind:value={nombre_comercial}
			/>
		</div>

		<div class="grid grid-cols-3 gap-4">
			<Select id="pais" label="País" bind:value={pais} options={paises} />
			<Select
				id="departamento"
				label="Departamento"
				bind:value={departamento}
				placeholder="Seleccionar"
			/>
			<Select id="provincia" label="Provincia" bind:value={provincia} placeholder="Seleccionar" />
		</div>

		<div class="grid grid-cols-3 gap-4">
			<Select id="distrito" label="Distrito" bind:value={distrito} placeholder="Seleccionar" />
			<div class="col-span-2">
				<Input id="direccion" label="Dirección" variant="simple" bind:value={direccion} />
			</div>
		</div>

		<div class="grid grid-cols-2 gap-4">
			<Input id="telefono" label="Teléfono" variant="simple" bind:value={telefono} />
			<Input id="correo" label="Correo electrónico" variant="simple" bind:value={correo} />
		</div>

		<div class="grid grid-cols-2 gap-4">
			<Input id="contacto" label="Otro contacto" variant="simple" bind:value={contacto} />
			<Input id="cuenta_bancaria" label="Cuenta Bancaria" variant="simple" bind:value={cuenta_bancaria} />
		</div>

		<Input id="observaciones" label="Observaciones" variant="simple" bind:value={observaciones} />

		{#if error}
			<div class="text-xs text-red-400">
				{error}
			</div>
		{/if}
	</div>

	<!-- Footer -->
	<div class="flex items-center justify-end gap-3 p-3">
		<Button type="button" variant="outline" onclick={handleCloseModal}>Cancelar</Button>
		<Button type="button" variant="primary" onclick={handleSubmit} disabled={loading}>
			{loading ? 'Guardando...' : 'Guardar'}
		</Button>
	</div>
</Modal>

