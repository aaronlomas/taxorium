<script lang="ts">
	import Option from '$lib/components/core/primitives/Select.svelte';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Fecha from '$lib/components/core/primitives/InputData.svelte';
	import TableSales from './TableSales.svelte';
	import SalesSummary from './SalesDetails.svelte';

	// TIPOS DE COMPROBANTE SEGUN SUNAT
	const tipoDeComprobante = [
		{ value: '03', label: 'Boleta de Venta Electrónica' },
		{ value: '01', label: 'Factura Electrónica' }
	];
	let comprobanteSeleccionado = '01';

	//SERIES SEGUN SUNAT
	const series = [
		{ value: 'F001', label: 'F001' },
		{ value: 'B001', label: 'B001' }
	];
	let serieSeleccionada = 'B001'; // por defecto

	//ESTABLECIMIENTOS
	const establecimientos = [
		{ value: '001', label: 'Oficina Principal' },
		{ value: '002', label: 'Establecimiento 002' },
		{ value: '003', label: 'Establecimiento 003' }
	];
	let establecimientoPrincipal = '001'; // por defecto

	// TIPO DE OPERACION SEGUN SUNAT
	const tiposDeOperacion = [
		{ value: '0101', label: 'Venta Interna' },
		{ value: '0102', label: 'Venta Interna - Anticipos' }
	];
	let operacionSeleccionada = '0101'; // por defecto

	//TIPO DE MONEDA SEGUN SUNAT
	const tipoDeMoneda = [
		{ value: 'PEN', label: 'Soles' },
		{ value: 'USD', label: 'Dólares' }
	];
  let monedaSeleccionada = 'PEN'

  //TIPO DE PAGO SEGÚN SUNAT
  const tipoDePago = [
    {value: 'Contado', label: 'Contado'},
    {value: 'Credito', label: 'Crédito'}
  ]
  let pagoSeleccionado = 'Contado'
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
				options={tipoDeComprobante}
        label="Tipo de Comprobante"
				bind:value={comprobanteSeleccionado}
			/>

			<Option id="seleccionarCliente" label="Seleccionar Cliente" />

			<Option id="tipoPago" label="Tipo de Pago" options={tipoDePago} bind:value={pagoSeleccionado}/>

			<Input id="infoAdicional" label="Información Adicional"/>
		</div>

    <!-- COLUMNA 2 -->
		<!-- Otras configuraciones -->

		<div class="grid grid-cols-2 gap-4 p-4">
			<div class="flex flex-col">
				<Option id="serie" label="Serie" options={series} bind:value={serieSeleccionada} />

				<Option id="tipoOperacion" label="Tipo de Operación" options={tiposDeOperacion} bind:value={operacionSeleccionada} />

        <Option id="moneda" label="Moneda" options={tipoDeMoneda} bind:value={monedaSeleccionada}/>

				<Input id="tipoCambio" label="Orden de Compra"/>
			</div>

			<div class="grid">
				<Fecha id="fechaEmision" label="Fecha de Emisión"/>

				<Fecha id="fechaVencimiento" label="Fecha de Vencimiento"/>

				<Option
					id="establecimiento"
          label="Establecimiento"
					options={establecimientos}
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
