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
	import ProductModal from './ProductModal.svelte';
	import { productsStore, editingProduct } from '$lib/stores/products';
	import { apiClient, type Product } from '$lib/services/apiClient';
	import { taxoLog } from '$lib/stores/taxoLog';
	import { SUNAT_UNITS } from '$lib/constants/units';

	interface ColumnConfig extends CheckItem {
		key: keyof Product;
	}

	let columnas = $state<ColumnConfig[]>([
		{ id: 'internal_code', key: 'internal_code', label: 'Cód. Interno', checked: true },
		{ id: 'unit_code', key: 'unit_code', label: 'Unidad', checked: true },
		{ id: 'name', key: 'name', label: 'Descripción', checked: true },
		{ id: 'stock_local', key: 'stock_local', label: 'Stock', checked: true },
		{ id: 'price_sale', key: 'price_sale', label: 'Precio Venta', checked: true },
		{ id: 'price_purchase', key: 'price_purchase', label: 'Precio Compra', checked: false },
		{ id: 'sunat_code', key: 'sunat_code', label: 'Código SUNAT', checked: false },
		{ id: 'brand', key: 'brand', label: 'Marca', checked: false },
		{ id: 'category', key: 'category', label: 'Categoría', checked: false },
		{ id: 'branch', key: 'branch', label: 'Sede', checked: false }
	]);

	// Columnas visibles reactivas según TableFilter
	let columnasVisibles = $derived(columnas.filter((c) => c.checked));

	let isModalOpen = $state(false);
	let searchTerm = $state('');

	onMount(() => {
		productsStore.load();
	});

	// Filtrado de lista por término de búsqueda
	let productosFiltrados = $derived(
		$productsStore.filter((p) => {
			if (!searchTerm.trim()) return true;
			const term = searchTerm.toLowerCase();
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

	function formatCellValue(product: Product, key: keyof Product): string {
		const val = product[key];
		if (val === null || val === undefined) return '-';
		if (key === 'price_sale' || key === 'price_purchase') {
			const num = typeof val === 'number' ? val : parseFloat(String(val));
			return `S/. ${num.toFixed(2)}`;
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

<div class="grid grid-rows-[40px_auto_1fr] border-b border-neutral-800 text-sm h-full">
	<div class="flex items-center justify-center font-extrabold text-white">
		<p>Lista de Productos</p>
	</div>

	<div class="grid grid-cols-[1fr_auto] border-t border-t-neutral-800 p-4">
		<div class="flex items-center gap-4">
			<button
				class="flex cursor-pointer gap-2 rounded-xl bg-neutral-800 px-3 py-1 hover:bg-neutral-700"
				onclick={handleAdd}
			>
				<IconPlus size={20} />Añadir
			</button>
			<button class="flex cursor-pointer gap-2 rounded-xl bg-neutral-800 px-3 py-1 hover:bg-neutral-700">
				<IconDatabaseImport size={20} />Importar
			</button>
			<button class="flex cursor-pointer gap-2 rounded-xl bg-neutral-800 px-3 py-1 hover:bg-neutral-700">
				<IconDatabaseExport size={20} />Exportar
			</button>
			<TableFilter label="Columnas" bind:items={columnas} />
		</div>
		<div class="flex items-center rounded-lg border border-neutral-800 px-4">
			<IconSearch size={16} />
			<input
				type="text"
				placeholder="Buscar producto..."
				bind:value={searchTerm}
				class="w-full border-0 bg-transparent text-sm focus:ring-0"
			/>
		</div>
	</div>

	<ProductModal bind:isOpen={isModalOpen} onClose={() => (isModalOpen = false)} />

	<!-- TABLA DE PRODUCTOS DINÁMICA DE ACUERDO A TableFilter -->
	<div class="overflow-auto">
		<table class="w-full bg-neutral-900 text-center">
			<!-- Cabecera dinámica -->
			<thead class="border-b border-neutral-800 text-neutral-400">
				<tr>
					<th class="py-2 px-3">#</th>
					{#each columnasVisibles as col (col.id)}
						<th class="py-2 px-3">{col.label}</th>
					{/each}
					<th class="py-2 px-3">Acciones</th>
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
							<td class="py-2 px-3 text-neutral-500">{index + 1}</td>
							{#each columnasVisibles as col (col.id)}
								<td class="py-2 px-3">
									{formatCellValue(product, col.key)}
								</td>
							{/each}
							<td class="py-2 px-3">
								<div class="flex items-center justify-center gap-2">
									<button
										class="text-neutral-400 transition-colors hover:text-blue-400 cursor-pointer"
										onclick={() => handleEdit(product)}
										title="Editar"
									>
										<IconEdit size={16} />
									</button>
									<button
										class="text-neutral-400 transition-colors hover:text-red-400 cursor-pointer"
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
