<script lang="ts">
	import Select from '$lib/components/core/primitives/Select.svelte';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Fecha from '$lib/components/core/primitives/InputDate.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';
	import SalesModal from '$lib/components/ui/views/sales/SalesModal.svelte';
	import { onMount } from 'svelte';
	import {
		catalogoStore,
		seriesOptionsPorTipo,
		sedesOptions,
		tiposOperacionOptions,
		monedasOptions,
		tiposPagoOptions,
		tiposComprobanteOptions
	} from '$lib/stores/catalogos';
	import { customersStore } from '$lib/stores/customers';
	import type { Customer } from '$lib/services/customers/clientCustomer';
	import CustomersModal from '$lib/components/ui/views/customers/CustomersModal.svelte';

	onMount(() => {
		catalogoStore.load();
		customersStore.load();
	});

	let clientesOptions = $derived(
		$customersStore.map((c: Customer) => ({
			value: c.id,
			label: `${c.tipo_documento} ${c.numero_documento} - ${c.nombre}`
		}))
	);

	// Valores por defecto
	let comprobanteSeleccionado = $state('01');
	let clienteSeleccionado = $state<number | ''>('');
	let serieSeleccionada = $state('F001');
	let establecimientoPrincipal = $state('0000');
	let operacionSeleccionada = $state('0101');
	let monedaSeleccionada = $state('PEN');
	let pagoSeleccionado = $state('Contado');

	let isModalOpen = $state(false);
	let isCustomersModalOpen = $state(false);

	const seriesDisponibles = $derived($seriesOptionsPorTipo(comprobanteSeleccionado));
	$effect(() => {
		if (seriesDisponibles.length > 0 && !seriesDisponibles.some((s) => s.value === serieSeleccionada)) {
			serieSeleccionada = seriesDisponibles[0].value;
		}
	});
</script>

<div class="grid grid-cols-2 border border-neutral-800 text-sm">
	<!-- COLUMNA 1 -->
	<!-- configuraciones rápidas para venta -->

	<div class="flex flex-col justify-between border-r border-r-neutral-800 p-2">
		<div>
			<div class="flex gap-x-2">
				<Select
					id="tipoComprobante"
					options={$tiposComprobanteOptions}
					label="Tipo de Comprobante"
					bind:value={comprobanteSeleccionado}
				/>
				<Select
					id="tipoPago"
					label="Tipo de Pago"
					options={$tiposPagoOptions}
					bind:value={pagoSeleccionado}
				/>
			</div>
			<div class="flex items-end gap-x-2">
				<Select
					id="seleccionarCliente"
					label="Seleccionar Cliente"
					placeholder="Seleccionar cliente..."
					options={clientesOptions}
					bind:value={clienteSeleccionado}
				/>
				<Button variant="outline" size="md" onclick={() => (isCustomersModalOpen = true)}
					>+ Nuevo</Button
				>
			</div>

			<Input id="infoAdicional" label="Información Adicional" />
		</div>
		<!-- BOTONES DEL PANEL -->
		<div>
			<Button variant="primary" size="md" onclick={() => (isModalOpen = true)}
				>+ Agregar Producto</Button
			>
			<Button variant="primary" size="md" onclick={() => (isModalOpen = true)}>Ver Reportes</Button>
		</div>
	</div>

	<!-- COLUMNA 2 -->
	<!-- Otras configuraciones -->

	<div class="grid grid-cols-2 gap-4 p-2">
		<div class="flex flex-col">
			<Select
				id="serie"
				label="Serie"
				options={seriesDisponibles}
				bind:value={serieSeleccionada}
				disabled={seriesDisponibles.length === 0}
			/>

			<Select
				id="tipoOperacion"
				label="Tipo de Operación"
				options={$tiposOperacionOptions}
				bind:value={operacionSeleccionada}
			/>

			<Select
				id="moneda"
				label="Moneda"
				options={$monedasOptions}
				bind:value={monedaSeleccionada}
			/>

			<Input id="tipoCambio" label="Orden de Compra" />
		</div>

		<div class="grid">
			<Fecha id="fechaEmision" label="Fecha de Emisión" />

			<Fecha id="fechaVencimiento" label="Fecha de Vencimiento" />

			<Select
				id="establecimiento"
				label="Establecimiento"
				options={$sedesOptions}
				bind:value={establecimientoPrincipal}
			/>

			<Input id="tipoCambio" label="Tipo de Cambio" />
		</div>
	</div>
	<SalesModal bind:isOpen={isModalOpen} onClose={() => (isModalOpen = false)} />
	<CustomersModal
		bind:isOpen={isCustomersModalOpen}
		onClose={() => (isCustomersModalOpen = false)}
	/>
</div>
