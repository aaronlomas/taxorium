<script lang="ts">
	import Modal from '$lib/components/core/primitives/Modal.svelte';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Select from '$lib/components/core/primitives/Select.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';

	import {
		customerClient,
		type CreateCustomerPayload
	} from '$lib/services/customers/clientCustomer';
	import { customersStore, editingCustomer } from '$lib/stores/customers';
	import { taxoLog } from '$lib/stores/taxoLog';
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

	let error = $state('');
	let loading = $state(false);

	const tiposDoc = [
		{ value: 'RUC', label: 'RUC' },
		{ value: 'DNI', label: 'DNI' },
		{ value: 'CE', label: 'CE' },
		{ value: 'PASAPORTE', label: 'PASAPORTE' }
	];

	const paises = [{ value: 'PERU', label: 'PERU' }];

	// Sync with editingCustomer
	$effect(() => {
		if ($editingCustomer) {
			tipo_documento = $editingCustomer.tipo_documento || 'RUC';
			numero_documento = $editingCustomer.numero_documento || '';
			nombre = $editingCustomer.nombre || '';
			nombre_comercial = $editingCustomer.nombre_comercial || '';
			pais = $editingCustomer.pais || 'PERU';
			departamento = $editingCustomer.departamento || '';
			provincia = $editingCustomer.provincia || '';
			distrito = $editingCustomer.distrito || '';
			direccion = $editingCustomer.direccion || '';
			telefono = $editingCustomer.telefono || '';
			correo = $editingCustomer.correo || '';
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
		error = '';
	}

	async function handleSubmit() {
		if (!numero_documento.trim() || !nombre.trim()) {
			error = 'Número de documento y nombre son requeridos';
			return;
		}

		error = '';
		loading = true;

		try {
			const payload: CreateCustomerPayload = {
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
				correo: correo.trim() || undefined
			};

			if ($editingCustomer) {
				await customerClient.updateCustomer($editingCustomer.id, payload);
				taxoLog.info(`Cliente '${payload.nombre}' actualizado`, 'clientes');
			} else {
				await customerClient.createCustomer(payload);
				taxoLog.info(`Cliente '${payload.nombre}' creado`, 'clientes');
			}

			$editingCustomer = null;
			resetForm();
			await customersStore.load();
			onClose();
		} catch (err: any) {
			const msg = err?.message ?? 'Error al guardar el cliente';
			error = msg;
			taxoLog.error(msg, 'clientes');
		} finally {
			loading = false;
		}
	}

	function handleCloseModal() {
		$editingCustomer = null;
		resetForm();
		onClose();
	}
</script>

{#snippet sunatButton()}
	<button
		type="button"
		class="flex h-full items-center justify-center gap-1 border-l border-neutral-800 px-3 text-xs font-medium text-neutral-400 transition-colors hover:text-neutral-200"
	>
		<IconSearch size={14} /> SUNAT
	</button>
{/snippet}

<Modal
	bind:isOpen
	onClose={handleCloseModal}
	title={$editingCustomer ? 'Editar Cliente' : 'Nuevo Cliente'}
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
			<Input
				id="nombre"
				label="Nombre"
				required
				variant="simple"
				bind:value={nombre}
			/>
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

		{#if error}
			<div class="text-xs text-red-400">
				{error}
			</div>
		{/if}
	</div>

	<!-- Footer -->
	<div class="flex items-center justify-end gap-3 border-t border-neutral-800 p-3">
		<Button type="button" variant="outline" onclick={handleCloseModal}>Cancelar</Button>
		<Button type="button" variant="primary" onclick={handleSubmit} disabled={loading}>
			{loading ? 'Guardando...' : 'Guardar'}
		</Button>
	</div>
</Modal>
