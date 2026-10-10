<script lang="ts">
	import { onMount } from 'svelte';
	import Modal from '$lib/components/core/primitives/Modal.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';
	import Select from '$lib/components/core/primitives/Select.svelte';
	import Number from '$lib/components/core/primitives/Number.svelte';
	import ProductModal from '$lib/components/ui/views/products/ProductModal.svelte';
	import { productsStore, editingProduct } from '$lib/features/products';
	import { salesStore } from '$lib/features/sales';
	import { catalogoStore, afectacionesVentaOptions } from '$lib/features/catalogos';

	let { isOpen = $bindable(false), onClose }: { isOpen: boolean; onClose: () => void } = $props();

	let isProductModalOpen = $state(false);

	let productoSeleccionado = $state<string | number>('');
	let cantidad = $state(1);
	let precioUnitario = $state('0');
	let afectacionIgv = $state('');
	const exoneradoDefault = $derived(
		$afectacionesVentaOptions.find((o) => o.label.toLowerCase().includes('exonerado'))?.value
	);
	$effect(() => {
		if (!afectacionIgv && exoneradoDefault) afectacionIgv = exoneradoDefault;
	});

	let prevProducto = '';
	$effect(() => {
		if (String(productoSeleccionado) !== prevProducto) {
			prevProducto = String(productoSeleccionado);
			if (productoSeleccionado) {
				const producto = $productsStore.find((p) => String(p.id) === String(productoSeleccionado));
				if (producto) {
					precioUnitario = String(producto.precio_unitario_venta);
					// Cada producto nace con su afectación de venta: si el catálogo todavía
					// no cargó, la línea se agregaría sin código de afectación y el backend
					// la rechazaría al emitir.
					afectacionIgv = producto.afectacion_venta || afectacionIgv;
				}
			} else {
				precioUnitario = '0';
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

		const precio = +precioUnitario;
		const subtotal = cantidad * precio;

		salesStore.add({
			productoId: producto.id,
			descripcion: producto.nombre,
			// El código del catálogo 03 viaja tal cual al `cbc:InvoicedQuantity/@unitCode`;
			// el nombre con símbolo se arma al mostrarlo con `getUnitDisplay`.
			unidad: producto.codigo_unidad || 'NIU',
			cantidad,
			precioUnitario: precio,
			subtotal,
			total: subtotal,
			afectacion: afectacionIgv || producto.afectacion_venta || '20',
			tieneIcbper: producto.tiene_icbper ?? false,
			moneda: producto.moneda
		});

		productoSeleccionado = '';
		cantidad = 1;
		precioUnitario = '0';
	}

	function handleNuevoProducto() {
		$editingProduct = null;
		isProductModalOpen = true;
	}
</script>

<Modal bind:isOpen {onClose} title="Registrar">
	<div class="grid h-full w-2xl grid-cols-[1fr_140px] gap-4 p-2">
		<!-- Contenido -->
		<div class="flex items-end gap-x-2">
			<Select
				label="Producto/Servicio"
				options={productosOptions}
				bind:value={productoSeleccionado}
			/>
			<Button variant="outline" size="md" onclick={handleNuevoProducto}>Nuevo</Button>
		</div>
		<div>
			<Number label="Cantidad" bind:value={cantidad} />
		</div>
		<div>
			<Select
				label="Afectación IGV"
				options={$afectacionesVentaOptions}
				bind:value={afectacionIgv}
			/>
		</div>
		<div>
			<Number label="Precio Unitario" bind:value={precioUnitario} />
		</div>
	</div>

	<!-- Footer -->
	<div class="col-span-2 flex items-center justify-end gap-3 border-t border-neutral-800 p-2">
		<Button type="button" variant="outline" onclick={onClose}>Cerrar</Button>
		<Button type="button" variant="primary" onclick={handleSave}>Aceptar</Button>
	</div>
</Modal>

<ProductModal bind:isOpen={isProductModalOpen} onClose={() => (isProductModalOpen = false)} />
