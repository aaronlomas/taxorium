<script lang="ts">
	import Select from '$lib/components/core/primitives/forms/Select.svelte';
	import Input from '$lib/components/core/primitives/forms/Input.svelte';
	import Number from '$lib/components/core/primitives/forms/Number.svelte';
	import Fecha from '$lib/components/core/primitives/forms/InputDate.svelte';
	import Button from '$lib/components/core/primitives/actions/Button.svelte';
	import PurchaseModal from '$lib/components/ui/features/purchase/PurchaseModal.svelte';
	import { onMount } from 'svelte';
	import {
		catalogoStore,
		sedesOptions,
		monedasOptions,
		tiposComprobanteOptions
	} from '$lib/features/catalogos';
	import { customersStore } from '$lib/features/customers';
	import type { Customer } from '$lib/features/customers';
	import CustomersModal from '$lib/components/ui/features/customers/CustomersModal.svelte';
	import { purchaseConfigStore } from '$lib/components/ui/features/purchase/purchaseContext';

	onMount(() => {
		catalogoStore.load();
		customersStore.load();
	});

	/**
	 * Proveedores: se toman los clientes con RUC, que es la unidad con la que se
	 * registran las compras. A futuro puede migrar a un catálogo propio.
	 */
	let proveedoresOptions = $derived(
		$customersStore
			.filter((c: Customer) => c.tipo_documento === 'RUC')
			.map((c: Customer) => ({
				value: c.id,
				label: `${c.numero_documento} - ${c.nombre}`
			}))
	);

	let isModalOpen = $state(false);
	let isCustomersModalOpen = $state(false);

	/** El tipo de cambio solo aplica cuando la moneda es dólares. */
	const esDolar = $derived($purchaseConfigStore.moneda === 'USD');
</script>

<div class="grid grid-cols-2 border border-neutral-800 text-sm">
	<!-- COLUMNA 1 -->
	<!-- configuraciones rápidas para la compra -->

	<div class="flex flex-col justify-between border-r border-r-neutral-800 p-2">
		<div>
			<div class="flex gap-x-2">
				<Select
					id="tipoDocumento"
					options={$tiposComprobanteOptions}
					label="Tipo de Documento"
					bind:value={$purchaseConfigStore.tipoDocumento}
				/>
				<Input id="serie" label="Serie" required bind:value={$purchaseConfigStore.serie} />
				<Input id="numero" label="Número" required bind:value={$purchaseConfigStore.numero} />
			</div>
			<div class="flex items-end gap-x-2">
				<Select
					id="seleccionarProveedor"
					label="Seleccionar Proveedor"
					placeholder="Seleccionar proveedor..."
					required
					options={proveedoresOptions}
					bind:value={$purchaseConfigStore.proveedorId}
				/>
				<Button variant="outline" size="md" onclick={() => (isCustomersModalOpen = true)}
					>+ Nuevo</Button
				>
			</div>
		</div>
		<!-- BOTONES DEL PANEL -->
		<div>
			<Button variant="primary" size="md" onclick={() => (isModalOpen = true)}
				>Agregar Producto</Button
			>
		</div>
	</div>

	<!-- COLUMNA 2 -->
	<!-- Otras configuraciones -->

	<div class="grid grid-cols-2 gap-2 p-2">
		<div class="flex flex-col">
			<Select
				id="moneda"
				label="Moneda"
				options={$monedasOptions}
				bind:value={$purchaseConfigStore.moneda}
			/>
			<Number
				id="tipoCambio"
				label="Tipo de Cambio"
				step={0.01}
				decimals={2}
				min={0}
				disabled={!esDolar}
				bind:value={$purchaseConfigStore.tipoCambio}
			/>
			<Select
				id="establecimiento"
				label="Establecimiento"
				options={$sedesOptions}
				bind:value={$purchaseConfigStore.establecimiento}
			/>
		</div>

		<div class="grid">
			<Fecha
				id="fechaEmision"
				label="Fecha de Emisión"
				bind:value={$purchaseConfigStore.fechaEmision}
			/>
			<Fecha
				id="fechaVencimiento"
				label="Fecha de Vencimiento"
				bind:value={$purchaseConfigStore.fechaVencimiento}
			/>
			<Input
				id="infoAdicional"
				label="Referencia / Guía de Remisión"
				bind:value={$purchaseConfigStore.infoAdicional}
			/>
		</div>
	</div>
	<PurchaseModal bind:isOpen={isModalOpen} onClose={() => (isModalOpen = false)} />
	<CustomersModal
		bind:isOpen={isCustomersModalOpen}
		onClose={() => (isCustomersModalOpen = false)}
	/>
</div>
