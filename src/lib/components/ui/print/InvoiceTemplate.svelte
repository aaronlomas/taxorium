<script lang="ts">
	import type { PrintableInvoiceData } from './invoiceTypes';

	interface Props {
		/**
		 * Esta es la estructura JSON unificada que recibirás.
		 * Modifica el HTML de este componente libremente;
		 * la estructura de datos se mantendrá igual.
		 */
		data: PrintableInvoiceData;
	}

	const { data }: Props = $props();

	// Utilidades de formato
	const formatCurrency = (val: number, currency: string) => {
		const symbol = currency === 'USD' ? '$' : currency === 'EUR' ? '€' : 'S/';
		return `${symbol} ${val.toFixed(2)}`;
	};
</script>

<!-- 
	==============================================================
	 ZONA DE DISEÑO DE COMPROBANTE (FACTURA / BOLETA)
	==============================================================
	- Aquí puedes cambiar las clases de Tailwind, los colores, 
	  la estructura de tablas, posiciones, etc.
	- Esta vista está preparada para tener fondo blanco y texto 
	  negro (ideal para impresión/PDF).
	- Usa `data.empresa`, `data.cliente`, `data.items`, etc.
	==============================================================
-->

{#if data.formato === 'TICKET'}
	<div
		class="print-container mx-auto bg-white p-2 font-sans text-xs text-black"
		style="width: 80mm;"
	>
		<!-- CABECERA TICKET -->
		<header class="flex flex-col text-center gap-2">
			{#if data.empresa.logo_url}
				<img src={data.empresa.logo_url} alt="Logo" class="mx-auto mb-2 h-12 object-contain" />
			{/if}
			<div>
				<h1 class="text-sm font-bold uppercase">{data.empresa.razon_social}</h1>
				<p>{data.empresa.direccion}</p>
				<p>RUC: {data.empresa.ruc}</p>
				{#if data.empresa.telefono}<p>Tel: {data.empresa.telefono}</p>{/if}
			</div>

			<p class="border-y border-neutral-400 py-1 uppercase">
				{data.tipo_comprobante}<br />
				<strong class="text-lg">
					{data.serie_correlativo}
				</strong>
			</p>
		</header>

		<!-- DATOS CLIENTE -->
		<section class="grid grid-cols-2 py-2 text-[11px]">
			<p>Fecha de Emisión:</p>
			{data.fecha_emision}
			<p>Cliente:</p>
			{data.cliente.nombre_o_razon_social}
			<p>{data.cliente.tipo_doc}:</p>
			{data.cliente.num_doc}
			{#if data.cliente.direccion}
				<p>Dirección:</p>
				{data.cliente.direccion}
			{/if}
			<p>Forma de Pago:</p>
			{data.forma_pago ?? '-'}
			{#if data.estado_pago}
				<p>Estado:</p>
				{data.estado_pago}
			{/if}
		</section>

		<!-- DETALLE ITEMS -->
		<section class="border-b border-neutral-400">
			<table class="w-full text-left">
				<thead>
					<tr class="border-y border-neutral-500 text-[11px]">
						<th class="text-center">CANT.</th>
						<th class="text-center">UND.</th>
						<th class="text-center">DESCRIPCIÓN</th>
						<th class="text-center">P.UNIT</th>
						<th class="text-center">TOTAL</th>
					</tr>
				</thead>
				<tbody>
					{#each data.items as item}
						<tr class="text-[11px]">
							<td class="py-1 align-top">{item.cantidad}</td>
							<td class="py-1 align-top">{item.unidad}</td>
							<td class="py-1 align-top">{item.descripcion}</td>
							<td class="py-1 align-top">{item.precio_unitario}</td>
							<td class="py-1 align-top text-right"
								>{formatCurrency(item.total, data.totales.moneda)}</td
							>
						</tr>
					{/each}
				</tbody>
			</table>
		</section>

		<!-- TOTALES TICKET -->
		<section>
			<table class="w-full">
				<tbody>
					{#if data.totales.descuento && data.totales.descuento > 0}
						<tr
							><td class="text-right">Desc:</td><td class="w-16 text-right"
								>{formatCurrency(data.totales.descuento, data.totales.moneda)}</td
							></tr
						>
					{/if}
					<tr
						><td class="text-right">Op. Gravada:</td><td class="w-16 text-right"
							>{formatCurrency(data.totales.gravado, data.totales.moneda)}</td
						></tr
					>
					<tr
						><td class="text-right">IGV (18%):</td><td class="w-16 text-right"
							>{formatCurrency(data.totales.igv, data.totales.moneda)}</td
						></tr
					>
					<tr class="text-sm font-bold">
						<td class="py-1 text-right">TOTAL:</td>
						<td class="py-1 text-right"
							>{formatCurrency(data.totales.total, data.totales.moneda)}</td
						>
					</tr>
				</tbody>
			</table>
		</section>

		<!-- FOOTER TICKET -->
		<footer class="flex flex-col gap-2 text-center">
			{#if data.totales.total_letras}
				<p>SON: {data.totales.total_letras}</p>
			{/if}

			{#if data.qr_code_data}
				<div
					class="mx-auto flex h-32 w-32 items-center justify-center border border-neutral-300 bg-neutral-200 text-neutral-500"
				>
					[QR]
				</div>
			{/if}

			<div class="mt-2 text-[10px] text-neutral-500">
				{#if data.hash_cpe}<p>Hash: {data.hash_cpe}</p>{/if}
				<p>Representación impresa del Comprobante Electrónico</p>
				<p>Consulte en taxorium.app/consultas</p>
			</div>
		</footer>
	</div>
{:else}
	<div
		class="print-container mx-auto max-w-4xl bg-white p-8 font-sans text-sm text-black shadow-lg"
	>
		<!-- CABECERA -->
		<header class="mb-6 flex items-start justify-between border-b-2 border-neutral-200 pb-6">
			<!-- Datos de la Empresa -->
			<div class="flex max-w-sm flex-col gap-2">
				{#if data.empresa.logo_url}
					<!-- Opcional: Logo de la empresa -->
					<!-- svelte-ignore a11y_missing_attribute -->
					<img src={data.empresa.logo_url} alt="Logo" class="h-16 self-start object-contain" />
				{/if}
				<h1 class="text-xl font-bold uppercase">{data.empresa.razon_social}</h1>
				<div class="leading-tight text-neutral-600">
					<p>{data.empresa.direccion}</p>
					{#if data.empresa.telefono}<p>Tel: {data.empresa.telefono}</p>{/if}
					{#if data.empresa.email}<p>Email: {data.empresa.email}</p>{/if}
				</div>
			</div>

			<!-- Recuadro del Comprobante (RUC, Tipo, Serie) -->
			<div class="min-w-62.5 rounded-lg border-2 border-neutral-800 p-4 text-center">
				<p class="mb-2 text-lg font-bold">RUC: {data.empresa.ruc}</p>
				<p class="mb-2 bg-neutral-100 py-1 text-lg font-bold uppercase">
					{data.tipo_comprobante}
				</p>
				<p class="text-xl font-bold">{data.serie_correlativo}</p>
			</div>
		</header>

		<!-- DATOS DEL CLIENTE Y EMISIÓN -->
		<section class="mb-6 grid grid-cols-2 gap-4">
			<div>
				<h2 class="mb-2 border-b border-neutral-300 text-xs font-bold text-neutral-500 uppercase">
					Adquiriente
				</h2>
				<p><span class="font-semibold">Señor(es):</span> {data.cliente.nombre_o_razon_social}</p>
				<p><span class="font-semibold">{data.cliente.tipo_doc}:</span> {data.cliente.num_doc}</p>
				{#if data.cliente.direccion}
					<p><span class="font-semibold">Dirección:</span> {data.cliente.direccion}</p>
				{/if}
			</div>
			<div>
				<h2 class="mb-2 border-b border-neutral-300 text-xs font-bold text-neutral-500 uppercase">
					Detalles de Emisión
				</h2>
				<p><span class="font-semibold">Fecha de Emisión:</span> {data.fecha_emision}</p>
				<p><span class="font-semibold">Moneda:</span> {data.totales.moneda}</p>
				{#if data.forma_pago}
					<p><span class="font-semibold">Forma de Pago:</span> {data.forma_pago}</p>
				{/if}
				{#if data.estado_pago}
					<p><span class="font-semibold">Estado:</span> {data.estado_pago}</p>
				{/if}
			</div>
		</section>

		<!-- DETALLE DE ITEMS -->
		<section class="mb-6">
			<table class="w-full border-collapse text-left">
				<thead>
					<tr class="bg-neutral-800 text-white">
						<th class="w-16 px-3 py-2 text-center">Cant.</th>
						<th class="w-16 px-3 py-2 text-center">Und.</th>
						<th class="px-3 py-2">Descripción</th>
						<th class="w-28 px-3 py-2 text-right">P. Unitario</th>
						<th class="w-28 px-3 py-2 text-right">Total</th>
					</tr>
				</thead>
				<tbody>
					{#each data.items as item}
						<tr class="border-b border-neutral-200">
							<td class="px-3 py-2 text-center">{item.cantidad}</td>
							<td class="px-3 py-2 text-center text-xs text-neutral-500">{item.unidad}</td>
							<td class="px-3 py-2">{item.descripcion}</td>
							<td class="px-3 py-2 text-right"
								>{formatCurrency(item.precio_unitario, data.totales.moneda)}</td
							>
							<td class="px-3 py-2 text-right font-medium"
								>{formatCurrency(item.total, data.totales.moneda)}</td
							>
						</tr>
					{/each}
				</tbody>
			</table>
		</section>

		<!-- TOTALES Y FOOTER -->
		<section class="flex items-start justify-between gap-8">
			<!-- Info extra (QR, Hash, Letras) -->
			<div class="flex flex-1 flex-col gap-4">
				{#if data.totales.total_letras}
					<div class="text-sm">
						<span class="font-bold">SON:</span>
						{data.totales.total_letras}
					</div>
				{/if}

				<div class="mt-4 flex items-center gap-4">
					{#if data.qr_code_data}
						<!-- Placeholder visual del QR. Puedes usar una librería svelte-qrcode si lo prefieres -->
						<div
							class="flex h-24 w-24 items-center justify-center border border-neutral-300 bg-neutral-200 text-center text-xs text-neutral-500"
						>
							[QR CODE]<br />{data.qr_code_data.slice(0, 10)}...
						</div>
					{/if}
					<div class="flex flex-col gap-1 text-xs text-neutral-500">
						{#if data.hash_cpe}
							<p><strong>Hash:</strong> {data.hash_cpe}</p>
						{/if}
						{#if data.resolucion_sunat}
							<p>{data.resolucion_sunat}</p>
						{:else}
							<p>Representación impresa del Comprobante de Pago Electrónico.</p>
						{/if}
						<p>Consulte su comprobante en <strong>taxorium.app/consultas</strong></p>
					</div>
				</div>
			</div>

			<!-- Tabla de Totales -->
			<div class="w-64">
				<table class="w-full text-sm">
					<tbody>
						{#if data.totales.descuento && data.totales.descuento > 0}
							<tr>
								<td class="py-1 text-right font-semibold">Descuentos:</td>
								<td class="py-1 text-right"
									>{formatCurrency(data.totales.descuento, data.totales.moneda)}</td
								>
							</tr>
						{/if}
						<tr>
							<td class="py-1 text-right font-semibold">Op. Gravadas:</td>
							<td class="py-1 text-right"
								>{formatCurrency(data.totales.gravado, data.totales.moneda)}</td
							>
						</tr>
						{#if data.totales.exonerado && data.totales.exonerado > 0}
							<tr>
								<td class="py-1 text-right font-semibold">Op. Exoneradas:</td>
								<td class="py-1 text-right"
									>{formatCurrency(data.totales.exonerado, data.totales.moneda)}</td
								>
							</tr>
						{/if}
						{#if data.totales.inafecto && data.totales.inafecto > 0}
							<tr>
								<td class="py-1 text-right font-semibold">Op. Inafectas:</td>
								<td class="py-1 text-right"
									>{formatCurrency(data.totales.inafecto, data.totales.moneda)}</td
								>
							</tr>
						{/if}
						<tr>
							<td class="py-1 text-right font-semibold">IGV (18%):</td>
							<td class="py-1 text-right"
								>{formatCurrency(data.totales.igv, data.totales.moneda)}</td
							>
						</tr>
						<tr class="border-t-2 border-neutral-800 text-lg">
							<td class="py-2 text-right font-bold">TOTAL:</td>
							<td class="py-2 text-right font-bold"
								>{formatCurrency(data.totales.total, data.totales.moneda)}</td
							>
						</tr>
					</tbody>
				</table>
			</div>
		</section>
	</div>
{/if}

<style>
	/* Aquí puedes asegurar que el diseño impreso respete estilos específicos */
	@media print {
		:global(body) {
			background: white !important;
		}
		.print-container {
			box-shadow: none !important;
			padding: 0 !important;
			max-width: 100% !important;
		}
	}
</style>
