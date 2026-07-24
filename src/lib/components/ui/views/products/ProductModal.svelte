<script lang="ts">
	import Modal from '$lib/components/core/primitives/Modal.svelte';
	import Number from '$lib/components/core/primitives/Number.svelte';
	import Option from '$lib/components/core/primitives/Option.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';
	import Input from '$lib/components/core/primitives/Input.svelte';

	let { isOpen = $bindable(false), onClose }: { isOpen: boolean; onClose: () => void } = $props();

	// Definimos las clases base para los selects y contenedores para mantener consistencia
	const labelClass = 'mb-1 block text-xs font-medium text-neutral-400';

	// CATÁLOGO DE TIPO DE AFECTACIÓN EN VENTAS SEGÚN SUNAT
	const tipoDeAfectacion = [
		{ value: '10', label: 'Gravado - Op. Onerosa (IGV 18%)' },
		{ value: '20', label: 'Exonerado - Op. Onerosa (IGV 0%)' },
		{ value: '15', label: 'Transferencia Gratuita - Bonificación' }
	];
	// CATÁLOGO DE TIPO DE AFECTACIÓN EN COMPRAS
	// DESTINO FISCAL DE LA ADQUISICIÓN - REGISTRO DE COMPRAS SUNAT

	const destinoCompras = [
		{
			value: 'GRAVADO_V_GRAVADO',
			label: 'Adquisición Gravada destinada a Ventas Gravadas (Crédito Fiscal)'
		},
		{
			value: 'GRAVADO_V_MIXTO',
			label: 'Adquisición Gravada destinada a Ventas Gravadas y No Gravadas'
		},
		{
			value: 'GRAVADO_V_NOGRAVADO',
			label: 'Adquisición Gravada destinada a Ventas No Gravadas (Costo/Gasto)'
		},
		{ value: 'NO_GRAVADO', label: 'Adquisición No Gravada (Exonerada / Inafecta / Sin IGV)' }
	];

	const tipoDeMoneda = [
		{ value: 'PEN', label: 'Soles' },
		{ value: 'USD', label: 'Dólares' }
	];
</script>

<Modal bind:isOpen {onClose}>
	<div class="flex h-full w-full flex-col">
		<!-- Cabecera -->
		<div class="flex items-center justify-between border-b border-neutral-800 p-2">
			<h2 class="text-lg font-semibold text-slate-100">Nuevo Producto</h2>
		</div>

		<!-- Cuerpo del Modal (Grid Layout de 4 columnas) -->
		<div class="flex-1 overflow-y-auto p-6">
			<div class="grid grid-cols-4 gap-x-6 gap-y-5">
				<!-- Fila 1 -->
				<div class="col-span-1">
					<Input id="codigoInterno" label="Código Interno" variant="simple" />
				</div>
				<div class="col-span-1">
					<Option
						id="unidad"
						label="Unidad"
						value="NIU"
						options={[{ value: 'NIU', label: 'Unidades' }]}
					/>
				</div>
				<div class="col-span-2">
					<Input
						id="descripcion"
						label="Descripción <span class='text-red-500'>*</span>"
						variant="simple"
						placeholder="Ej. Zapatillas T30..."
					/>
				</div>

				<!-- Fila 2 -->
				<div class="col-span-1">
					<Input id="codigoSunat" label="Código Sunat" variant="simple" />
				</div>
				<div class="col-span-1">
					<Input id="codigoGsl" label="Código GSL" variant="simple" />
				</div>
				<div class="col-span-1">
					<Option id="moneda" label="Moneda" value="PEN" options={tipoDeMoneda} />
				</div>
				<div class="col-span-1">
					<Number id="precioVenta" label="Precio Unitario (Venta)" value={0} />
				</div>

				<!-- Fila 3 -->
				<div class="col-span-1">
					<Number id="precioCompra" label="Precio Unitario (Compra)" value={0} />
				</div>
				<div class="col-span-1">
					<Number id="stockMinimo" label="Stock Mínimo" value={1} />
				</div>

				<!-- TIPO DE AFECTACION EN VENTAS -->
				<div class="col-span-2">
					<Option
						id="afectacionVenta"
						label="Tipo de afectación (Venta)"
						value="20"
						options={tipoDeAfectacion}
					/>
				</div>

				<!-- TIPO DE AFECTACION EN COMPRAS -->
				<div class="col-span-2">
					<Option
						id="afectacionCompra"
						label="Tipo de afectación (Compra)"
						value="NO_GRAVADO"
						options={destinoCompras}
					/>
				</div>
				<div class="col-span-2 flex items-center pt-5">
					<label class="flex cursor-pointer items-center gap-2 text-sm text-neutral-300">
						<input
							type="checkbox"
							class="h-4 w-4 rounded border-neutral-700 bg-neutral-900 text-blue-600 focus:ring-blue-500 focus:ring-offset-neutral-950"
						/>
						ICBPER (Impuesto a la bolsa)
					</label>
				</div>

				<!-- Fila 5 -->
				<div class="col-span-2">
					<Option
						id="marca"
						label="Marca"
						value=""
						options={[{ value: '', label: 'No seleccionado' }]}
					/>
				</div>
				<div class="col-span-2">
					<Option
						id="categoria"
						label="Categoría"
						value=""
						options={[{ value: '', label: 'No seleccionado' }]}
					/>
				</div>

				<!-- Fila 6 -->
				<div class="col-span-2">
					<span class={labelClass}>Sede</span>
					<div class="flex h-10 items-center text-sm text-neutral-300">Oficina Principal</div>
				</div>
				<div class="col-span-2">
					<Number id="stockLocal" label="Stock Local" value={0} />
				</div>
			</div>
		</div>

		<!-- Footer -->
		<div class="flex items-center justify-end gap-3 border-t border-neutral-800 p-2">
			<Button type="button" variant="outline" onclick={onClose}>Cancelar</Button>
			<Button type="button" variant="primary">Guardar</Button>
		</div>
	</div>
</Modal>
