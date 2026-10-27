<script lang="ts">
	import { onMount } from 'svelte';
	import Modal from '$lib/components/core/primitives/feedback/Modal.svelte';
	import Button from '$lib/components/core/primitives/actions/Button.svelte';
	import Select from '$lib/components/core/primitives/forms/Select.svelte';
	import Number from '$lib/components/core/primitives/forms/Number.svelte';
	import ProductsModal from '$lib/components/ui/features/products/ProductsModal.svelte';
	import { productsStore, editingProduct } from '$lib/features/products';
	import { purchaseStore } from '$lib/features/purchase';
	import {
		catalogoStore,
		unidadesOptions,
		afectacionesCompraOptions,
		getUnitDisplay
	} from '$lib/features/catalogos';

	let { isOpen = $bindable(false), onClose }: { isOpen: boolean; onClose: () => void } = $props();

	let isProductModalOpen = $state(false);

	let productoSeleccionado = $state<string | number>('');
	let cantidadCompra = $state(1);
	let unidadCompra = $state('NIU');
	let unidadBase = $state('NIU');
	let factorConversion = $state(1);
	let costoUnitarioCompra = $state(0);
	let afectacionCompra = $state('');

	// Cada producto de compra nace con su afectación; si el catálogo todavía no
	// cargó se usa la primera opción "no gravado" para no registrar la línea vacía.
	const afectacionDefault = $derived(
		$afectacionesCompraOptions.find((o) => o.label.toLowerCase().includes('no gravado'))?.value
	);
	$effect(() => {
		if (!afectacionCompra && afectacionDefault) afectacionCompra = afectacionDefault;
	});

	// Cantidad convertida a la unidad base del producto (stock).
	const cantidadBase = $derived((+cantidadCompra || 0) * (+factorConversion || 0));

	let prevProducto = '';
	$effect(() => {
		if (String(productoSeleccionado) !== prevProducto) {
			prevProducto = String(productoSeleccionado);
			const producto = $productsStore.find((p) => String(p.id) === String(productoSeleccionado));
			if (producto) {
				unidadBase = producto.codigo_unidad || 'NIU';
				unidadCompra = producto.codigo_unidad_compra || producto.codigo_unidad || 'NIU';
				factorConversion = producto.factor_a_unidad_base || 1;
				costoUnitarioCompra = producto.precio_unitario_compra ?? 0;
				afectacionCompra = producto.afectacion_compra || afectacionCompra;
			} else {
				factorConversion = 1;
				costoUnitarioCompra = 0;
			}
		}
	});

	let productosOptions = $derived(
		$productsStore
			.filter((p) => p.activo)
			.map((p) => ({
				value: String(p.id),
				label: `${p.codigo_interno ?? ''} - ${p.nombre}`.trim()
			}))
	);

	onMount(() => {
		productsStore.load();
		catalogoStore.load();
	});

	function handleSave() {
		const producto = $productsStore.find((p) => String(p.id) === String(productoSeleccionado));
		if (!producto) return;

		const factor = +factorConversion > 0 ? +factorConversion : 1;
		const cantidad = +cantidadCompra || 0;
		const costo = +costoUnitarioCompra || 0;

		purchaseStore.add({
			productoId: producto.id,
			descripcion: producto.nombre,
			unidadCompra,
			unidadBase,
			factorConversion: factor,
			cantidadCompra: cantidad,
			// Se materializa la cantidad en unidad base: es lo que consumirá el
			// motor de inventario al registrar la entrada.
			cantidadBase: cantidad * factor,
			costoUnitarioCompra: costo,
			costoUnitarioBase: costo / factor,
			subtotal: cantidad * costo,
			afectacion: afectacionCompra || producto.afectacion_compra || 'NO_GRAVADO',
			moneda: producto.moneda
		});

		productoSeleccionado = '';
		cantidadCompra = 1;
		factorConversion = 1;
		costoUnitarioCompra = 0;
	}

	function handleNuevoProducto() {
		$editingProduct = null;
		isProductModalOpen = true;
	}
</script>

<Modal bind:isOpen {onClose} title="Registrar Compra">
	<div class="grid w-2xl grid-cols-2 gap-4 p-3">
		<div class="col-span-2 flex items-end gap-x-2">
			<Select
				label="Producto/Servicio"
				options={productosOptions}
				bind:value={productoSeleccionado}
			/>
			<Button variant="outline" size="md" onclick={handleNuevoProducto}>Nuevo</Button>
		</div>

		<Number label="Cantidad (compra)" bind:value={cantidadCompra} min={0} />
		<Select label="Unidad de Compra" options={$unidadesOptions} bind:value={unidadCompra} />

		<Number label="Factor de Conversión" bind:value={factorConversion} min={0} step={0.01} />
		<Select
			label="Unidad Base (Stock)"
			options={$unidadesOptions}
			bind:value={unidadBase}
			disabled={true}
		/>

		<Number label="Costo Unitario (compra)" bind:value={costoUnitarioCompra} min={0} step={0.01} />
		<Select
			label="Afectación (compra)"
			options={$afectacionesCompraOptions}
			bind:value={afectacionCompra}
		/>

		<div
			class="col-span-2 rounded-sm border border-brand-border bg-brand-bg-primary px-3 py-2 text-xs text-brand-text-secondary"
		>
			Equivalencia:
			{cantidadCompra}
			{getUnitDisplay(unidadCompra)} ×
			{factorConversion}
			=
			<span class="font-medium text-blue-400">
				{cantidadBase}
				{getUnitDisplay(unidadBase)}
			</span>
		</div>
	</div>

	<!-- Footer -->
	<div class="flex items-center justify-end gap-3 p-2">
		<Button type="button" variant="outline" onclick={onClose}>Cerrar</Button>
		<Button type="button" variant="primary" onclick={handleSave}>Aceptar</Button>
	</div>
</Modal>

<ProductsModal bind:isOpen={isProductModalOpen} onClose={() => (isProductModalOpen = false)} />
