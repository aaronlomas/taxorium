<script lang="ts">
	import {
		IconSearch,
		IconDatabaseImport,
		IconDatabaseExport,
		IconPlus
	} from '@tabler/icons-svelte';
	import { editingProduct } from '$lib/stores/products';
	import { exportData, type ExportFormat } from '$lib/utilities/formats/export';
	let isModalOpen = $state(false);

	function handleAdd() {
		$editingProduct = null;
		isModalOpen = true;
	}

	async function handleExportFile(format: ExportFormat) {}
</script>

<div class="grid h-full grid-rows-[auto_1fr] gap-2 px-2 pb-2 text-sm">
	<!-- PANEL DE CONTROLES -->
	<div class="grid grid-cols-[1fr_auto] rounded-b-md border-x border-b border-neutral-800 p-2">
		<!-- BOTONES DE ACCIÓN -->
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
		</div>
		<!-- FILTROS Y BÚSQUEDA -->
		<div class="grid grid-cols-[auto_1fr] items-end gap-2">
			<div class="w-30"></div>
			<div class="flex items-center rounded-lg border border-neutral-800 px-4">
				<IconSearch size={16} />
				<input class="w-full border-0 bg-transparent text-sm focus:ring-0" />
			</div>
		</div>
	</div>

	<div class="overflow-auto rounded-md border border-neutral-800">
		<table class="w-full bg-neutral-900 text-center">
			<thead class="border-b border-neutral-800 text-neutral-400">
				<tr class="text-blue-400">
					<th class="px-2">#</th>
				</tr>
			</thead>

			<tbody class="divide-y divide-neutral-800 text-neutral-200">
				<tr> </tr>
			</tbody>
		</table>
	</div>
</div>
