<script lang="ts">
	import Select from '$lib/components/core/primitives/forms/Select.svelte';
	import Input from '$lib/components/core/primitives/forms/Input.svelte';
	import Fecha from '$lib/components/core/primitives/forms/InputDate.svelte';
	import Button from '$lib/components/core/primitives/actions/Button.svelte';
	import SalesModal from '$lib/components/ui/features/sales/SalesModal.svelte';
	import { onMount } from 'svelte';
	import {
		catalogoStore,
		seriesOptionsPorTipo,
		sedesOptions,
		tiposOperacionOptions,
		monedasOptions,
		tiposPagoOptions,
		tiposComprobanteOptions
	} from '$lib/features/catalogos';
	import { customersStore } from '$lib/features/customers';
	import type { Customer } from '$lib/features/customers';
	import CustomersModal from '$lib/components/ui/features/customers/CustomersModal.svelte';
	import { voucherConfigStore } from '$lib/components/ui/features/sales/vouchers/voucherContext';

	onMount(() => {
		catalogoStore.load();
		customersStore.load();
	});

	/**
	 * Determina si el tipo de comprobante es una factura (requiere RUC).
	 * '01' = Factura, '03' = Boleta (y otros).
	 */
	const esFactura = $derived($voucherConfigStore.tipoComprobante === '01');

	/**
	 * Filtra los clientes según el tipo de comprobante seleccionado:
	 * - Factura → solo clientes con RUC
	 * - Boleta u otro → clientes sin RUC (DNI, CE, PASAPORTE, etc.)
	 */
	let clientesOptions = $derived(
		$customersStore
			.filter((c: Customer) =>
				esFactura ? c.tipo_documento === 'RUC' : c.tipo_documento !== 'RUC'
			)
			.map((c: Customer) => ({
				value: c.id,
				label: `${c.tipo_documento} ${c.numero_documento} - ${c.nombre}`
			}))
	);

	let isModalOpen = $state(false);
	let isCustomersModalOpen = $state(false);

	const seriesDisponibles = $derived($seriesOptionsPorTipo($voucherConfigStore.tipoComprobante));

	$effect(() => {
		if (
			seriesDisponibles.length > 0 &&
			!seriesDisponibles.some((s) => s.value === $voucherConfigStore.serie)
		) {
			voucherConfigStore.updateField('serie', String(seriesDisponibles[0].value));
		}
	});

	/**
	 * Resetea el cliente seleccionado al cambiar el tipo de comprobante.
	 * `prevTipoComprobante` es una variable plain (no $state) para evitar
	 * que Svelte programe renders extra al escribirla dentro del $effect.
	 */
	let prevTipoComprobante = $voucherConfigStore.tipoComprobante;
	$effect(() => {
		const current = $voucherConfigStore.tipoComprobante;
		if (current !== prevTipoComprobante) {
			prevTipoComprobante = current;
			voucherConfigStore.updateField('clienteId', '');
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
					bind:value={$voucherConfigStore.tipoComprobante}
				/>
				<Select
					id="tipoPago"
					label="Tipo de Pago"
					options={$tiposPagoOptions}
					bind:value={$voucherConfigStore.tipoPago}
				/>
			</div>
			<div class="flex items-end gap-x-2">
				<Select
					id="seleccionarCliente"
					label="Seleccionar Cliente"
					placeholder="Seleccionar cliente..."
					options={clientesOptions}
					bind:value={$voucherConfigStore.clienteId}
				/>
				<Button variant="outline" size="md" onclick={() => (isCustomersModalOpen = true)}
					>+ Nuevo</Button
				>
			</div>

			<Input
				id="infoAdicional"
				label="Información Adicional"
				bind:value={$voucherConfigStore.infoAdicional}
			/>
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
			<Input id="serie" label="Serie" disabled={true} bind:value={$voucherConfigStore.serie} />

			<Select
				id="tipoOperacion"
				label="Tipo de Operación"
				options={$tiposOperacionOptions}
				bind:value={$voucherConfigStore.tipoOperacion}
			/>

			<Select
				id="moneda"
				label="Moneda"
				options={$monedasOptions}
				bind:value={$voucherConfigStore.moneda}
			/>

			<Input
				id="OrdenDeCompra"
				label="Orden de Compra"
				bind:value={$voucherConfigStore.ordenCompra}
			/>
		</div>

		<div class="grid">
			<Fecha
				id="fechaEmision"
				label="Fecha de Emisión"
				bind:value={$voucherConfigStore.fechaEmision}
			/>

			<Fecha
				id="fechaVencimiento"
				label="Fecha de Vencimiento"
				bind:value={$voucherConfigStore.fechaVencimiento}
			/>

			<Select
				id="establecimiento"
				label="Establecimiento"
				options={$sedesOptions}
				bind:value={$voucherConfigStore.establecimiento}
			/>

			<Input
				id="tipoCambio"
				label="Tipo de Cambio"
				disabled={true}
				bind:value={$voucherConfigStore.tipoCambio}
			/>
		</div>
	</div>
	<SalesModal bind:isOpen={isModalOpen} onClose={() => (isModalOpen = false)} />
	<CustomersModal
		bind:isOpen={isCustomersModalOpen}
		onClose={() => (isCustomersModalOpen = false)}
	/>
</div>
