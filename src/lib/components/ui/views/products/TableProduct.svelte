<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconDatabaseImport,
		IconDatabaseExport,
		IconPlus,
		IconTrash,
		IconEdit
	} from '@tabler/icons-svelte';
	import TableFilter, { type CheckItem } from '$lib/components/core/primitives/TableFilter.svelte';
	import TableToolbar from '$lib/components/core/primitives/TableToolbar.svelte';
	import Search from '$lib/components/core/primitives/Search.svelte';
	import Select from '$lib/components/core/primitives/Select.svelte';
	import Hint from '$lib/components/core/primitives/Hint.svelte';
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/Table.svelte';

	import ProductModal from './ProductModal.svelte';
	import { productsStore, editingProduct } from '$lib/stores/products';
	import { productClient, type Product } from '$lib/services/products/clientProducts';
	import { taxoLog } from '$lib/stores/taxoLog';
	import { catalogoStore, getUnitDisplay } from '$lib/stores/catalogos';
	import { exportData, type ExportFormat } from '$lib/utilities/formats/export';

	interface ColumnConfig extends CheckItem {
		key: keyof Product;
	}

	let columnas = $state<ColumnConfig[]>([
		{ id: 'codigo_interno', key: 'codigo_interno', label: 'Cód. Interno', checked: true },
		{ id: 'codigo_unidad', key: 'codigo_unidad', label: 'Unidad', checked: true },
		{ id: 'nombre', key: 'nombre', label: 'Descripción', checked: true },
		{
			id: 'precio_unitario_venta',
			key: 'precio_unitario_venta',
			label: 'P. Unitario Venta',
			checked: true
		},
		{
			id: 'precio_unitario_compra',
			key: 'precio_unitario_compra',
			label: 'P. Unitario Compra',
			checked: false
		},
		{ id: 'moneda', key: 'moneda', label: 'Moneda', checked: true },
		{ id: 'codigo_sunat', key: 'codigo_sunat', label: 'Código SUNAT', checked: false },
		{ id: 'marca', key: 'marca', label: 'Marca', checked: false },
		{ id: 'categoria', key: 'categoria', label: 'Categoría', checked: false },
		{ id: 'codigo_sede', key: 'codigo_sede', label: 'Sede', checked: false }
	]);

	let columnasVisibles = $derived(columnas.filter((c) => c.checked));

	let isModalOpen = $state(false);
	let searchTerm = $state('');
	let searchBy = $state('');

	const SEARCH_OPTIONS = [
		{ value: '', label: 'Todos' },
		{ value: 'nombre', label: 'Descripción' },
		{ value: 'categoria', label: 'Categoría' },
		{ value: 'marca', label: 'Marca' }
	];

	onMount(() => {
		productsStore.load();
		catalogoStore.load();
	});

	let productosFiltrados = $derived(
		$productsStore.filter((p) => {
			if (!searchTerm.trim()) return true;
			const term = searchTerm.toLowerCase();

			if (searchBy === 'nombre') {
				return p.nombre.toLowerCase().includes(term);
			} else if (searchBy === 'marca') {
				return p.marca?.toLowerCase().includes(term);
			} else if (searchBy === 'categoria') {
				return p.categoria?.toLowerCase().includes(term);
			}

			return (
				p.nombre.toLowerCase().includes(term) ||
				(p.codigo_interno && p.codigo_interno.toLowerCase().includes(term)) ||
				(p.codigo_sunat && p.codigo_sunat.toLowerCase().includes(term)) ||
				(p.marca && p.marca.toLowerCase().includes(term)) ||
				(p.categoria && p.categoria.toLowerCase().includes(term))
			);
		})
	);

	function handleAdd() {
		$editingProduct = null;
		isModalOpen = true;
	}

	function handleEdit(product: Product) {
		$editingProduct = product;
		isModalOpen = true;
	}

	async function handleExportFile(format: ExportFormat) {
		const savedPath = await exportData(productosFiltrados, columnasVisibles, 'productos', format);
		if (savedPath) {
			taxoLog.info(`Exportado exitosamente en: ${savedPath}`, 'productos');
		}
	}

	async function handleDelete(id: number, nombre: string) {
		if (confirm(`¿Estás seguro de eliminar el producto "${nombre}"?`)) {
			try {
				await productClient.deleteProduct(id);
				taxoLog.info(`Producto '${nombre}' eliminado`, 'productos');
				await productsStore.load();
			} catch (e: any) {
				const msg = e?.message ?? 'Error al eliminar el producto';
				taxoLog.error(msg, 'productos');
			}
		}
	}

	const CURRENCY_SYMBOL: Record<string, string> = {
		PEN: 'S/.',
		USD: '$',
		EUR: '€'
	};

	function formatCellValue(product: Product, key: keyof Product): string {
		const val = product[key];
		if (val === null || val === undefined) return '-';
		if (key === 'precio_unitario_venta' || key === 'precio_unitario_compra') {
			const num = typeof val === 'number' ? val : parseFloat(String(val));
			const symbol = CURRENCY_SYMBOL[product.moneda ?? 'PEN'] ?? 'S/.';
			return `${symbol} ${num.toFixed(2)}`;
		}
		if (key === 'moneda') {
			const labels: Record<string, string> = {
				PEN: 'Soles (S/.)',
				USD: 'Dólares ($)',
				EUR: 'Euros (€)'
			};
			return labels[String(val)] ?? String(val);
		}
		if (key === 'codigo_unidad') {
			return getUnitDisplay(String(val));
		}
		if (typeof val === 'boolean') {
			return val ? 'Sí' : 'No';
		}
		return String(val);
	}
</script>

<div class="grid h-full grid-rows-[auto_1fr] gap-2 px-2 pb-2 text-sm">
	<!-- PRIMITIVA TOOLBAR -->
	<TableToolbar>
		{#snippet actions()}
			<Hint text="Agregar Producto" side="bottom" align="start">
				<button
					class="flex cursor-pointer gap-2 rounded-xl bg-neutral-800 px-3 py-1 hover:bg-neutral-700"
					onclick={handleAdd}
				>
					<IconPlus size={20} />
				</button>
			</Hint>
			<button
				class="flex cursor-pointer gap-2 rounded-xl bg-neutral-800 px-3 py-1 hover:bg-neutral-700"
			>
				<IconDatabaseImport size={20} />Importar
			</button>
			<div class="group relative">
				<button
					class="flex cursor-pointer items-center gap-2 rounded-xl bg-neutral-800 px-3 py-1 hover:bg-neutral-700"
				>
					<IconDatabaseExport size={20} />Exportar
				</button>
				<div
					class="absolute top-full z-10 hidden w-48 flex-col overflow-hidden rounded-md border border-neutral-700 bg-neutral-800 shadow-lg group-hover:flex"
				>
					<button
						class="px-4 py-2 text-left text-sm hover:bg-neutral-700 hover:text-white"
						onclick={() => handleExportFile('xlsx')}>Excel (.xlsx)</button
					>
					<button
						class="px-4 py-2 text-left text-sm hover:bg-neutral-700 hover:text-white"
						onclick={() => handleExportFile('csv-comma')}>CSV (comas)</button
					>
					<button
						class="px-4 py-2 text-left text-sm hover:bg-neutral-700 hover:text-white"
						onclick={() => handleExportFile('csv-semicolon')}>CSV (puntos y comas)</button
					>
				</div>
			</div>
			<TableFilter storageKey="productos" bind:items={columnas} />
		{/snippet}

		{#snippet filters()}
			<div class="w-30">
				<Select placeholder="Buscar por:" options={SEARCH_OPTIONS} bind:value={searchBy} />
			</div>
			<Search
				bind:value={searchTerm}
				placeholder={searchBy === 'nombre'
					? 'Buscar descripción...'
					: searchBy === 'categoria'
						? 'Buscar categoría...'
						: searchBy === 'marca'
							? 'Buscar marca...'
							: 'Buscar producto...'}
			/>
		{/snippet}
	</TableToolbar>

	<ProductModal bind:isOpen={isModalOpen} onClose={() => (isModalOpen = false)} />

	<!-- TABLA DE PRODUCTOS -->
	<Table>
		{#snippet head()}
			<tr class="text-blue-400">
				{#snippet headIndex()}
					#
				{/snippet}
				{@render headCell({ children: headIndex })}
				{#each columnasVisibles as col (col.id)}
					{#snippet headCol()}
						{col.label}
					{/snippet}
					{@render headCell({ children: headCol })}
				{/each}
				{#snippet headActions()}
					Acciones
				{/snippet}
				{@render headCell({ children: headActions })}
			</tr>
		{/snippet}

		{#snippet body()}
			{#if productosFiltrados.length === 0}
				<tr>
					{#snippet emptyState()}
						No hay productos registrados.
					{/snippet}
					{@render cell({
						colspan: columnasVisibles.length + 2,
						class: 'py-8 text-neutral-500',
						children: emptyState
					})}
				</tr>
			{:else}
				{#each productosFiltrados as product, index (product.id)}
					{#snippet rowData()}
						{#snippet cellIndex()}
							{index + 1}
						{/snippet}
						{@render cell({ class: 'px-3 py-2 text-neutral-500', children: cellIndex })}
						{#each columnasVisibles as col (col.id)}
							{#snippet cellVal()}
								{formatCellValue(product, col.key)}
							{/snippet}
							{@render cell({ children: cellVal })}
						{/each}
						{#snippet cellActions()}
							<div class="flex items-center justify-center gap-2">
								<button
									class="cursor-pointer text-neutral-400 transition-colors hover:text-blue-400"
									onclick={() => handleEdit(product)}
									title="Editar"
								>
									<IconEdit size={16} />
								</button>
								<button
									class="cursor-pointer text-neutral-400 transition-colors hover:text-red-400"
									onclick={() => handleDelete(product.id, product.nombre)}
									title="Eliminar"
								>
									<IconTrash size={16} />
								</button>
							</div>
						{/snippet}
						{@render cell({ children: cellActions })}
					{/snippet}
					{@render row({ children: rowData })}
				{/each}
			{/if}
		{/snippet}
	</Table>
</div>
