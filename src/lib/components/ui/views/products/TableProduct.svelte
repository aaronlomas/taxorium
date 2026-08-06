<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconSearch,
		IconDatabaseImport,
		IconDatabaseExport,
		IconPlus,
		IconTrash,
		IconEdit
	} from '@tabler/icons-svelte';
	import TableFilter, { type CheckItem } from '$lib/components/core/primitives/TableFilter.svelte';
	import Option from '$lib/components/core/primitives/Option.svelte';

	import ProductModal from './ProductModal.svelte';
	import { productsStore, editingProduct } from '$lib/stores/products';
	import { apiClient, type Product } from '$lib/services/apiClient';
	import { taxoLog } from '$lib/stores/taxoLog';
	import { SUNAT_UNITS } from '$lib/constants/units';
	import { exportData, type ExportFormat } from '$lib/utilities/formats/export';

	interface ColumnConfig extends CheckItem {
		key: keyof Product;
	}

	let columnas = $state<ColumnConfig[]>([
		{ id: 'internal_code', key: 'internal_code', label: 'Cód. Interno', checked: true },
		{ id: 'unit_code', key: 'unit_code', label: 'Unidad', checked: true },
		{ id: 'name', key: 'name', label: 'Descripción', checked: true },
		{ id: 'stock_local', key: 'stock_local', label: 'Stock', checked: true },
		{ id: 'price_unit_sale', key: 'price_unit_sale', label: 'P. Unitario Venta', checked: true },
		{
			id: 'price_unit_purchase',
			key: 'price_unit_purchase',
			label: 'P. Unitario Compra',
			checked: false
		},
		{ id: 'currency', key: 'currency', label: 'Moneda', checked: true },
		{ id: 'sunat_code', key: 'sunat_code', label: 'Código SUNAT', checked: false },
		{ id: 'brand', key: 'brand', label: 'Marca', checked: false },
		{ id: 'category', key: 'category', label: 'Categoría', checked: false },
		{ id: 'branch', key: 'branch', label: 'Sede', checked: false }
	]);

	// Columnas visibles reactivas según TableFilter
	let columnasVisibles = $derived(columnas.filter((c) => c.checked));

	let isModalOpen = $state(false);
	let searchTerm = $state('');
	let searchBy = $state('');

	const SEARCH_OPTIONS = [
		{ value: '', label: 'Todos' },
		{ value: 'name', label: 'Descripción' },
		{ value: 'category', label: 'Categoría' },
		{ value: 'brand', label: 'Marca' }
	];

	onMount(() => {
		productsStore.load();
	});

	// Filtrado de lista por término de búsqueda y por campo
	let productosFiltrados = $derived(
		$productsStore.filter((p) => {
			if (!searchTerm.trim()) return true;
			const term = searchTerm.toLowerCase();

			if (searchBy === 'name') {
				return p.name.toLowerCase().includes(term);
			} else if (searchBy === 'brand') {
				return p.brand?.toLowerCase().includes(term);
			} else if (searchBy === 'category') {
				return p.category?.toLowerCase().includes(term);
			}

			// Si es 'Todos' (searchBy === '')
			return (
				p.name.toLowerCase().includes(term) ||
				(p.internal_code && p.internal_code.toLowerCase().includes(term)) ||
				(p.sunat_code && p.sunat_code.toLowerCase().includes(term)) ||
				(p.brand && p.brand.toLowerCase().includes(term)) ||
				(p.category && p.category.toLowerCase().includes(term))
			);
		})
	);

	function getUnitDisplay(code: string): string {
		const found = SUNAT_UNITS.find((u) => u.value === code);
		return found ? `${found.value} (${found.symbol || found.value})` : code;
	}

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

	async function handleDelete(id: number, name: string) {
		if (confirm(`¿Estás seguro de eliminar el producto "${name}"?`)) {
			try {
				await apiClient.deleteProduct(id);
				taxoLog.info(`Producto '${name}' eliminado`, 'productos');
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
		if (key === 'price_unit_sale' || key === 'price_unit_purchase') {
			const num = typeof val === 'number' ? val : parseFloat(String(val));
			const symbol = CURRENCY_SYMBOL[product.currency ?? 'PEN'] ?? 'S/.';
			return `${symbol} ${num.toFixed(2)}`;
		}
		if (key === 'currency') {
			const labels: Record<string, string> = {
				PEN: 'Soles (S/.)',
				USD: 'Dólares ($)',
				EUR: 'Euros (€)'
			};
			return labels[String(val)] ?? String(val);
		}
		if (key === 'unit_code') {
			return getUnitDisplay(String(val));
		}
		if (typeof val === 'boolean') {
			return val ? 'Sí' : 'No';
		}
		return String(val);
	}
</script>

<div class="grid h-full grid-rows-[auto_1fr] gap-2 p-2 text-sm">
	<div class="grid grid-cols-[1fr_auto] rounded-md border border-neutral-800 p-4">
		<div class="flex items-center gap-4">
			<button
				class="flex cursor-pointer gap-2 rounded-xl bg-neutral-800 px-3 py-1 hover:bg-neutral-700"
				onclick={handleAdd}
			>
				<IconPlus size={20} />Añadir
			</button>
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
				<div class="absolute left-0 top-full z-10 mt-1 hidden w-48 flex-col overflow-hidden rounded-md border border-neutral-700 bg-neutral-800 shadow-lg group-hover:flex">
					<button class="px-4 py-2 text-left text-sm hover:bg-neutral-700 hover:text-white" onclick={() => handleExportFile('xlsx')}>Excel (.xlsx)</button>
					<button class="px-4 py-2 text-left text-sm hover:bg-neutral-700 hover:text-white" onclick={() => handleExportFile('csv-comma')}>CSV (comas)</button>
					<button class="px-4 py-2 text-left text-sm hover:bg-neutral-700 hover:text-white" onclick={() => handleExportFile('csv-semicolon')}>CSV (puntos y comas)</button>
				</div>
			</div>
			<TableFilter label="Columnas" bind:items={columnas} />
		</div>

		<div class="grid grid-cols-[auto_1fr] items-end gap-2">
			<!-- Esta opcion solo filtra por categoria, marca y descripcion -->
			<div class="w-30">
				<Option 
					placeholder="Buscar por:" 
					options={SEARCH_OPTIONS} 
					bind:value={searchBy} 
				/>
			</div>
			<!-- buscador compatible con el filtro 'Buscar por:' -->
			<div class="flex items-center rounded-lg border border-neutral-800 px-4">
				<IconSearch size={16} />
				<input
					type="text"
					placeholder={searchBy === 'name' ? 'Buscar descripción...' : searchBy === 'category' ? 'Buscar categoría...' : searchBy === 'brand' ? 'Buscar marca...' : 'Buscar producto...'}
					bind:value={searchTerm}
					class="w-full border-0 bg-transparent text-sm focus:ring-0"
				/>
			</div>
		</div>
	</div>

	<ProductModal bind:isOpen={isModalOpen} onClose={() => (isModalOpen = false)} />

	<!-- TABLA DE PRODUCTOS DINÁMICA DE ACUERDO A TableFilter -->
	<div class="overflow-auto rounded-md border border-neutral-800">
		<table class="w-full bg-neutral-900 text-center">
			<!-- Cabecera dinámica -->
			<thead class="border-b border-neutral-800 text-neutral-400">
				<tr class="text-blue-400">
					<th class="px-2">#</th>
					{#each columnasVisibles as col (col.id)}
						<th class="px-2">{col.label}</th>
					{/each}
					<th class="px-2">Acciones</th>
				</tr>
			</thead>

			<!-- Contenido dinámico -->
			<tbody class="divide-y divide-neutral-800 text-neutral-200">
				{#if productosFiltrados.length === 0}
					<tr>
						<td colspan={columnasVisibles.length + 2} class="py-8 text-neutral-500">
							No hay productos registrados.
						</td>
					</tr>
				{:else}
					{#each productosFiltrados as product, index (product.id)}
						<tr class="hover:bg-neutral-850 transition-colors">
							<td class="px-3 py-2 text-neutral-500">{index + 1}</td>
							{#each columnasVisibles as col (col.id)}
								<td class="px-3 py-2">
									{formatCellValue(product, col.key)}
								</td>
							{/each}
							<td class="px-3 py-2">
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
										onclick={() => handleDelete(product.id, product.name)}
										title="Eliminar"
									>
										<IconTrash size={16} />
									</button>
								</div>
							</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>
</div>
