<script lang="ts">
	import { onMount } from 'svelte';
	import Modal from '$lib/components/core/primitives/Modal.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Select from '$lib/components/core/primitives/Select.svelte';
	import Number from '$lib/components/core/primitives/Number.svelte';
	import { productsStore } from '$lib/stores/products';
	import { salesStore } from '$lib/stores/sales';
	import { catalogoStore, afectacionesVentaOptions, getUnitDisplay } from '$lib/stores/catalogos';

	let { isOpen = $bindable(false), onClose }: { isOpen: boolean; onClose: () => void } = $props();

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
			unidad: getUnitDisplay(producto.codigo_unidad),
			cantidad,
			precioUnitario: precio,
			subtotal,
			total: subtotal,
			afectacion: afectacionIgv,
			moneda: producto.moneda
		});

		productoSeleccionado = '';
		cantidad = 1;
		precioUnitario = '0';
	}
</script>

<Modal bind:isOpen {onClose} title="Registrar">
	<div class="grid h-full w-full grid-cols-2 gap-4 p-2">
		<!-- Contenido -->
		<div>
			<Select label='Producto/Servicio' options={productosOptions} bind:value={productoSeleccionado} />
		</div>
		<div>
			<Number label='Cantidad' bind:value={cantidad} />
		</div>
		<div>
			<Number label='Precio Unitario' bind:value={precioUnitario} />
		</div>
		<div>
			<Select label='Afectación IGV' options={$afectacionesVentaOptions} bind:value={afectacionIgv} />
		</div>
	</div>

	<!-- Footer -->
	<div class="col-span-2 flex items-center justify-end gap-3 border-t border-neutral-800 p-2">
		<Button type="button" variant="outline" onclick={onClose}>Cerrar</Button>
		<Button type="button" variant="primary" onclick={handleSave}>Guardar</Button>
	</div>
</Modal>
