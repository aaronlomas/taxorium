<script lang="ts">
	import Option from '$lib/components/core/primitives/Select.svelte';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Fecha from '$lib/components/core/primitives/InputData.svelte';
	import TableSales from './TableSales.svelte';
	import SalesSummary from './SalesDetails.svelte';
	import { onMount } from 'svelte';
	import {
		catalogoStore,
		seriesOptions,
		sedesOptions,
		tiposOperacionOptions,
		monedasOptions,
		tiposPagoOptions,
		tiposComprobanteOptions
	} from '$lib/stores/catalogos';

	onMount(() => {
		catalogoStore.load();
	});

	// Valores por defecto
	let comprobanteSeleccionado = '01';
	let serieSeleccionada = 'F001';
	let establecimientoPrincipal = '0000';
	let operacionSeleccionada = '0101';
	let monedaSeleccionada = 'PEN';
	let pagoSeleccionado = 'Contado';
</script>

<div class="m-4">
	<div>
		<h1 class="font-extrabold">Configurar Nueva Venta</h1>
	</div>
	<form class="grid grid-cols-2 border border-neutral-800 text-sm">

    <!-- COLUMNA 1 -->
		<!-- configuraciones rápidas para venta -->

		<div class="border-r border-r-neutral-800 p-4">
			<Option
				id="tipoComprobante"
				options={$tiposComprobanteOptions}
        label="Tipo de Comprobante"
				bind:value={comprobanteSeleccionado}
			/>

			<Option id="seleccionarCliente" label="Seleccionar Cliente" />

			<Option id="tipoPago" label="Tipo de Pago" options={$tiposPagoOptions} bind:value={pagoSeleccionado}/>

			<Input id="infoAdicional" label="Información Adicional"/>
		</div>

    <!-- COLUMNA 2 -->
		<!-- Otras configuraciones -->

		<div class="grid grid-cols-2 gap-4 p-4">
			<div class="flex flex-col">
				<Option id="serie" label="Serie" options={$seriesOptions} bind:value={serieSeleccionada} />

				<Option id="tipoOperacion" label="Tipo de Operación" options={$tiposOperacionOptions} bind:value={operacionSeleccionada} />

        <Option id="moneda" label="Moneda" options={$monedasOptions} bind:value={monedaSeleccionada}/>

				<Input id="tipoCambio" label="Orden de Compra"/>
			</div>

			<div class="grid">
				<Fecha id="fechaEmision" label="Fecha de Emisión"/>

				<Fecha id="fechaVencimiento" label="Fecha de Vencimiento"/>

				<Option
					id="establecimiento"
          label="Establecimiento"
					options={$sedesOptions}
					bind:value={establecimientoPrincipal}
				/>

				<Input id="tipoCambio" label="Tipo de Cambio"/>
			</div>
		</div>
	</form>

  <!-- RESUMENES -->
   
  <div>
    <h1 class="font-extrabold">Detalles de Venta</h1>
    <TableSales/>
  </div>
  <div>
    <SalesSummary/>
  </div>
</div>
