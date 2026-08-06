<script lang="ts">
	import Modal from '$lib/components/core/primitives/Modal.svelte';
	import Number from '$lib/components/core/primitives/Number.svelte';
	import Option from '$lib/components/core/primitives/Option.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import { SUNAT_UNITS } from '$lib/constants/units';
	import { apiClient, type CreateProductPayload } from '$lib/services/apiClient';
	import { productsStore, editingProduct } from '$lib/stores/products';
	import { taxoLog } from '$lib/stores/taxoLog';

	let { isOpen = $bindable(false), onClose }: { isOpen: boolean; onClose: () => void } = $props();

	// Form State
	let codigoInterno = $state('');
	let unidad = $state('NIU');
	let descripcion = $state('');
	let codigoSunat = $state('');
	let codigoGsl = $state('');
	let moneda = $state('PEN');
	let precioVenta = $state(0);
	let precioCompra = $state(0);
	let stockMinimo = $state(1);
	let afectacionVenta = $state('20');
	let afectacionCompra = $state('NO_GRAVADO');
	let hasIcbper = $state(false);
	let marca = $state('');
	let categoria = $state('');
	let sede = $state('oficina-01');
	let stockLocal = $state(0);

	let error = $state('');
	let loading = $state(false);

	// Sync with editingProduct
	$effect(() => {
		if ($editingProduct) {
			codigoInterno = $editingProduct.internal_code || '';
			unidad = $editingProduct.unit_code || 'NIU';
			descripcion = $editingProduct.name || '';
			codigoSunat = $editingProduct.sunat_code || '';
			codigoGsl = $editingProduct.gsl_code || '';
			moneda = $editingProduct.currency || 'PEN';
			precioVenta = $editingProduct.price_sale ?? 0;
			precioCompra = $editingProduct.price_purchase ?? 0;
			stockMinimo = $editingProduct.stock_minimo ?? 1;
			afectacionVenta = $editingProduct.afectacion_venta || '20';
			afectacionCompra = $editingProduct.afectacion_compra || 'NO_GRAVADO';
			hasIcbper = $editingProduct.has_icbper ?? false;
			marca = $editingProduct.brand || '';
			categoria = $editingProduct.category || '';
			sede = $editingProduct.branch || 'oficina-01';
			stockLocal = $editingProduct.stock_local ?? 0;
			error = '';
		} else if (isOpen) {
			resetForm();
		}
	});

	function resetForm() {
		codigoInterno = '';
		unidad = 'NIU';
		descripcion = '';
		codigoSunat = '';
		codigoGsl = '';
		moneda = 'PEN';
		precioVenta = 0;
		precioCompra = 0;
		stockMinimo = 1;
		afectacionVenta = '20';
		afectacionCompra = 'NO_GRAVADO';
		hasIcbper = false;
		marca = '';
		categoria = '';
		sede = 'oficina-01';
		stockLocal = 0;
		error = '';
	}

	async function handleSubmit() {
		if (!descripcion.trim()) {
			error = 'La descripción es requerida';
			return;
		}

		// Validar que la unidad sea estrictamente una de las permitidas
		const validUnit = SUNAT_UNITS.find(u => u.value === unidad || u.label === unidad);
		if (!validUnit) {
			error = 'Por favor, seleccione una Unidad válida de la lista desplegable.';
			return;
		}

		error = '';
		loading = true;

		try {
			const payload: CreateProductPayload = {
				internal_code: codigoInterno.trim() || undefined,
				unit_code: validUnit.value,
				name: descripcion.trim(),
				sunat_code: codigoSunat.trim() || undefined,
				gsl_code: codigoGsl.trim() || undefined,
				currency: moneda || 'PEN',
				price_sale: window.Number(precioVenta) || 0,
				price_purchase: window.Number(precioCompra) || 0,
				stock_minimo: window.Number(stockMinimo) || 0,
				afectacion_venta: afectacionVenta || '20',
				afectacion_compra: afectacionCompra || 'NO_GRAVADO',
				has_icbper: hasIcbper,
				brand: marca.trim() || undefined,
				category: categoria.trim() || undefined,
				branch: sede.trim() || undefined,
				stock_local: window.Number(stockLocal) || 0
			};

			if ($editingProduct) {
				await apiClient.updateProduct($editingProduct.id, payload);
				taxoLog.info(`Producto '${payload.name}' actualizado`, 'productos');
			} else {
				await apiClient.createProduct(payload);
				taxoLog.info(`Producto '${payload.name}' creado`, 'productos');
			}

			$editingProduct = null;
			resetForm();
			await productsStore.load();
			onClose();
		} catch (err: any) {
			const msg = err?.message ?? 'Error al guardar el producto';
			error = msg;
			taxoLog.error(msg, 'productos');
		} finally {
			loading = false;
		}
	}

	function handleCloseModal() {
		$editingProduct = null;
		resetForm();
		onClose();
	}

	// CATÁLOGO REAL DE TIPO DE AFECTACIÓN EN VENTAS SEGÚN SUNAT
	const TIPOS_AFECTACION_VENTAS = [
		{ id: '10', value: '10', label: 'Gravado - Op. Onerosa (IGV 18%)' },
		{ id: '20', value: '20', label: 'Exonerado - Op. Onerosa (IGV 0%)' },
		{ id: '30', value: '30', label: 'Inafecto - Op. Onerosa (IGV 0%)' },
		{ id: '11', value: '11', label: 'Gravado - Retiro (Transferencia Gratuita)' },
		{ id: '31', value: '31', label: 'Inafecto - Retiro (Transferencia Gratuita)' }
	];

	// CATÁLOGO REAL DE TIPO DE AFECTACIÓN EN COMPRAS
	const DESTINO_AFECTACION_COMPRAS = [
		{
			value: 'GRAVADO_V_GRAVADO',
			label: 'Adq. Gravada - Destinada a Ventas Gravadas (Crédito Fiscal)'
		},
		{
			value: 'GRAVADO_V_MIXTO',
			label: 'Adq. Gravada - Destinada a Ventas Gravadas y No Gravadas'
		},
		{
			value: 'GRAVADO_V_NOGRAVADO',
			label: 'Adq. Gravada - Destinada a Ventas No Gravadas (Costo/Gasto)'
		},
		{ value: 'NO_GRAVADO', label: 'Adq. No Gravada - (Exonerada / Inafecta / Sin IGV)' }
	];

	const TIPO_MONEDA = [
		{ value: 'PEN', label: 'Soles' },
		{ value: 'USD', label: 'Dólares' }
	];

	const CATEGORIAS = [
		{ value: 'abarrotes', label: 'Abarrotes' },
		{ value: 'bebidas', label: 'Bebidas' },
		{ value: 'limpieza', label: 'Limpieza' },
		{ value: 'snacks', label: 'Snacks' },
		{ value: 'otros', label: 'Otros' }
	];
	const MARCAS = [
		{ value: 'gloria', label: 'Gloria' },
		{ value: 'molitalia', label: 'Molitalia' },
		{ value: 'costa', label: 'Costa' }
	];

	const SEDE = [
		{ value: 'oficina-01', label: 'Oficina Principal' },
		{ value: 'sede-01', label: 'Sede 01' },
		{ value: 'sede-02', label: 'Sede 02' },
		{ value: 'almacen', label: 'Almacén' }
	];
</script>

<Modal bind:isOpen onClose={handleCloseModal} title={$editingProduct ? 'Editar Producto' : 'Registrar Producto'}>
	<div class="grid h-full w-190 grid-cols-4 grid-rows-6 gap-2 p-2">
		<!-- Cuerpo del Modal (Grid Layout de 4 columnas) -->

		<!-- Fila 1 -->
		<Input id="codigoInterno" label="Código Interno" variant="simple" bind:value={codigoInterno} />

		<Option
			id="unidad"
			label="Unidad"
			bind:value={unidad}
			options={SUNAT_UNITS}
			editable={true}
		/>

		<div class="col-span-2">
			<Input
				id="descripcion"
				label="Descripción <span class='text-red-500'>*</span>"
				variant="simple"
				bind:value={descripcion}
				placeholder="Ej. Zapatillas T30..."
			/>
		</div>

		<!-- Fila 2 -->
		<Input id="codigoSunat" label="Código Sunat" variant="simple" bind:value={codigoSunat} />

		<Input id="codigoGsl" label="Código GSL" variant="simple" bind:value={codigoGsl} />

		<Option id="moneda" label="Moneda" bind:value={moneda} options={TIPO_MONEDA} />

		<Number id="precioVenta" label="Precio Unitario (Venta)" bind:value={precioVenta} />

		<!-- Fila 3 -->
		<Number id="precioCompra" label="Precio Unitario (Compra)" bind:value={precioCompra} />

		<Number id="stockMinimo" label="Stock Mínimo" bind:value={stockMinimo} />

		<!-- TIPO DE AFECTACION EN VENTAS -->
		<div class="col-span-2">
			<Option
				id="afectacionVenta"
				label="Tipo de afectación (Venta)"
				bind:value={afectacionVenta}
				options={TIPOS_AFECTACION_VENTAS}
			/>
		</div>

		<!-- TIPO DE AFECTACION EN COMPRAS -->
		<div class="col-span-2">
			<Option
				id="afectacionCompra"
				label="Tipo de afectación (Compra)"
				bind:value={afectacionCompra}
				options={DESTINO_AFECTACION_COMPRAS}
			/>
		</div>

		<div class="col-span-2 flex items-center pt-5">
			<label class="flex cursor-pointer items-center gap-2 text-sm text-neutral-300">
				<input
					type="checkbox"
					bind:checked={hasIcbper}
					class="h-4 w-4 rounded border-neutral-700 bg-neutral-900 text-blue-600 focus:ring-blue-500 focus:ring-offset-neutral-950"
				/>
				ICBPER (Impuesto a la bolsa)
			</label>
		</div>

		<!-- Fila 5 -->
		<div class="col-span-2">
			<Option id="marca" label="Marca" editable={true} bind:value={marca} options={MARCAS} />
		</div>

		<div class="col-span-2">
			<Option id="categoria" label="Categoría" editable={true} bind:value={categoria} options={CATEGORIAS} />
		</div>

		<!-- Fila 6 -->
		<div class="col-span-2">
			<Option id="sede" options={SEDE} editable={true} label="Sede" bind:value={sede} />
		</div>

		<div class="col-span-2">
			<Number id="stockLocal" label="Stock Local" bind:value={stockLocal} />
		</div>
	</div>

	{#if error}
		<div class="px-2 pb-1 text-xs text-red-400">
			{error}
		</div>
	{/if}

	<!-- Footer -->
	<div class="col-span-4 flex items-center justify-end gap-3 border-t border-neutral-800 p-2">
		<Button type="button" variant="outline" onclick={handleCloseModal}>Cancelar</Button>
		<Button type="button" variant="primary" onclick={handleSubmit} disabled={loading}>
			{loading ? 'Guardando...' : $editingProduct ? 'Actualizar' : 'Guardar'}
		</Button>
	</div>
</Modal>
