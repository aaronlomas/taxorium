<script lang="ts">
	import InvoiceTemplate from './InvoiceTemplate.svelte';
	import type { PrintableInvoiceData } from './invoiceTypes';

	// Datos de prueba para FACTURA
	const dummyFactura: PrintableInvoiceData = {
		formato: 'A4',
		tipo_comprobante: 'Factura Electrónica',
		serie_correlativo: 'F001-00000123',
		fecha_emision: '2023-10-25 14:30:00',
		empresa: {
			ruc: '20123456789',
			razon_social: 'MI EMPRESA EJEMPLO S.A.C.',
			direccion: 'Av. Las Palmas 123, Lima, Perú',
			ubigeo: '150101',
			telefono: '987 654 321',
			email: 'ventas@miempresa.com'
		},
		cliente: {
			tipo_doc: 'RUC',
			num_doc: '20987654321',
			nombre_o_razon_social: 'CLIENTE PRUEBA S.R.L.',
			direccion: 'Calle Falsa 456, Arequipa'
		},
		items: [
			{ cantidad: 2, unidad: 'NIU', descripcion: 'Producto de prueba A', precio_unitario: 50.00, total: 100.00 },
			{ cantidad: 1, unidad: 'ZZ', descripcion: 'Servicio de mantenimiento B', precio_unitario: 136.00, total: 136.00 }
		],
		totales: {
			moneda: 'PEN', gravado: 200.00, igv: 36.00, total: 236.00,
			total_letras: 'DOSCIENTOS TREINTA Y SEIS Y 00/100 SOLES'
		},
		forma_pago: 'Efectivo - Contado',
		estado_pago: 'Pagado',
		hash_cpe: 'x8+ABcDeFgHiJkLmNoPqRsTuVwXyZ=',
		qr_code_data: '20123456789|01|F001|00000123|36.00|236.00|25/10/2023|6|20987654321|'
	};

	// Datos de prueba para TICKET
	const dummyTicket: PrintableInvoiceData = {
		formato: 'TICKET',
		tipo_comprobante: 'Boleta de Venta Electrónica',
		serie_correlativo: 'B001-00000456',
		fecha_emision: '2023-10-25 15:00:00',
		empresa: {
			ruc: '20123456789',
			razon_social: 'MI EMPRESA EJEMPLO S.A.C.',
			direccion: 'Av. Las Palmas 123, Lima, Perú',
			ubigeo: '150101',
			telefono: '987 654 321',
			email: 'ventas@miempresa.com'
		},
		cliente: {
			tipo_doc: 'DNI',
			num_doc: '76543210',
			nombre_o_razon_social: 'JUAN PEREZ',
			direccion: 'Av. Siempre Viva 742'
		},
		items: [
			{ cantidad: 1, unidad: 'NIU', descripcion: 'Producto de prueba C', precio_unitario: 25.00, total: 25.00 }
		],
		totales: {
			moneda: 'PEN', gravado: 21.19, igv: 3.81, total: 25.00,
			total_letras: 'VEINTICINCO Y 00/100 SOLES'
		},
		forma_pago: 'Efectivo - Contado',
		estado_pago: 'Pagado',
		hash_cpe: 'y9+BCdEfGhIjKkLmMNoPqRsTuVwXyZ=',
		qr_code_data: '20123456789|03|B001|00000456|3.81|25.00|25/10/2023|1|76543210|'
	};

	// Datos de prueba para BOLETA
	const dummyBoleta: PrintableInvoiceData = {
		tipo_comprobante: 'Boleta de Venta Electrónica',
		serie_correlativo: 'B001-00000456',
		fecha_emision: '2023-10-25 15:00:00',
		empresa: {
			ruc: '20123456789',
			razon_social: 'MI EMPRESA EJEMPLO S.A.C.',
			direccion: 'Av. Las Palmas 123, Lima, Perú',
			ubigeo: '150101',
			telefono: '987 654 321',
			email: 'ventas@miempresa.com'
		},
		cliente: {
			tipo_doc: 'DNI',
			num_doc: '76543210',
			nombre_o_razon_social: 'JUAN PEREZ',
			direccion: 'Av. Siempre Viva 742'
		},
		items: [
			{ cantidad: 1, unidad: 'NIU', descripcion: 'Producto de prueba C', precio_unitario: 25.00, total: 25.00 }
		],
		totales: {
			moneda: 'PEN', gravado: 21.19, igv: 3.81, total: 25.00,
			total_letras: 'VEINTICINCO Y 00/100 SOLES'
		},
		forma_pago: 'Efectivo - Contado',
		estado_pago: 'Pagado',
		hash_cpe: 'y9+BCdEfGhIjKkLmMNoPqRsTuVwXyZ=',
		qr_code_data: '20123456789|03|B001|00000456|3.81|25.00|25/10/2023|1|76543210|'
	};

	let tipoPrueba = $state<'factura' | 'boleta' | 'ticket'>('factura');
	let dummyData = $derived(tipoPrueba === 'factura' ? dummyFactura : tipoPrueba === 'boleta' ? dummyBoleta : dummyTicket);

	function handlePrint() {
		window.print();
	}
</script>

<div class="h-full flex flex-col bg-neutral-900 text-white overflow-hidden p-4">
	<div class="flex justify-between items-center mb-4">
		<div>
			<h1 class="text-xl font-bold text-blue-400">Diseñador de Comprobantes</h1>
			<p class="text-sm text-neutral-400">Edita <code>InvoiceTemplate.svelte</code> para cambiar el diseño.</p>
		</div>
		<div class="flex gap-4 items-center">
			<div class="bg-neutral-800 p-1 rounded-lg flex gap-1">
				<button 
					onclick={() => tipoPrueba = 'factura'}
					class="px-4 py-1 rounded-md text-sm transition-colors {tipoPrueba === 'factura' ? 'bg-blue-600 text-white' : 'text-neutral-400 hover:text-white'}">
					Ver Factura
				</button>
				<button 
					onclick={() => tipoPrueba = 'boleta'}
					class="px-4 py-1 rounded-md text-sm transition-colors {tipoPrueba === 'boleta' ? 'bg-blue-600 text-white' : 'text-neutral-400 hover:text-white'}">
					Ver Boleta
				</button>
				<button 
					onclick={() => tipoPrueba = 'ticket'}
					class="px-4 py-1 rounded-md text-sm transition-colors {tipoPrueba === 'ticket' ? 'bg-blue-600 text-white' : 'text-neutral-400 hover:text-white'}">
					Ver Ticket
				</button>
			</div>
			<button 
				onclick={handlePrint}
				class="bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded-lg font-medium transition-colors">
				Probar Impresión (PDF)
			</button>
		</div>
	</div>

	<!-- Contenedor scrolleable que muestra el template en el centro -->
	<div class="flex-1 overflow-y-auto bg-neutral-800 rounded-xl p-8 flex justify-center items-start shadow-inner">
		<!-- Envolvemos el template para aislarlo visualmente -->
		<div class="w-full max-w-4xl shadow-2xl">
			<InvoiceTemplate data={dummyData} />
		</div>
	</div>
</div>
