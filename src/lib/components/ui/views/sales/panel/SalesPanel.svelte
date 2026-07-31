<script lang="ts">
	import Option from '$lib/components/core/primitives/Option.svelte';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Fecha from '$lib/components/core/primitives/InputData.svelte';
	import TableSales from './TableSales.svelte';
	import SalesSummary from './SalesSummary.svelte';

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

		<!-- configuraciones rápidas para venta -->

		<div class="border-r border-r-neutral-800 p-4">
			<label for="tipoComprobante" class="select-none">Tipo de Comprobante</label>
			<Option
				id="tipoComprobante"
				options={tipoDeComprobante}
				bind:value={comprobanteSeleccionado}
			/>

			<label for="seleccionarCliente" class="select-none">Seleccionar Cliente</label>
			<Option id="seleccionarCliente" />

			<label for="tipoPago" class="select-none">Tipo de Pago</label>
			<Option id="tipoPago" options={tipoDePago} bind:value={pagoSeleccionado}/>

			<label for="infoAdicional" class="select-none">Información Adicional</label>
			<Input id="infoAdicional"/>
		</div>

		<!-- Otras configuraciones -->

		<div class="grid grid-cols-2 gap-4 p-4">
			<div class="flex flex-col">
				<label for="serie" class="select-none">Serie</label>
				<Option id="serie" options={series} bind:value={serieSeleccionada} />

				<label for="tipoOperacion" class="select-none">Tipo de Operación</label>
				<Option id="tipoOperacion" options={tiposDeOperacion} bind:value={operacionSeleccionada} />

        <label for="moneda" class="select-none">Moneda</label>
        <Option id="moneda" options={tipoDeMoneda} bind:value={monedaSeleccionada}/>

				<label for="tipoCambio" class="select-none">Orden de Compra</label>
				<Input id="tipoCambio" />
			</div>

			<div class="grid">
				<label for="fechaEmision" class="select-none">F. Emisión</label>
				<Fecha id="fechaEmision" />

				<label for="fechaVencimiento" class="select-none">F. Vencimiento</label>
				<Fecha id="fechaVencimiento" />

				<label for="establecimiento" class="select-none">Establecimiento</label>
				<Option
					id="establecimiento"
					options={establecimientos}
					bind:value={establecimientoPrincipal}
				/>


				<label for="tipoCambio" class="select-none">Tipo de Cambio</label>
				<Input id="tipoCambio" />
			</div>
		</div>
	</form>

  <div>
    <h1 class="font-extrabold">Detalles de Venta</h1>
    <TableSales/>
  </div>
  <div>
    <SalesSummary/>
  </div>
</div>
