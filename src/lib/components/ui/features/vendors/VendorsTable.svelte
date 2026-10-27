<script lang="ts">
	import { onMount } from 'svelte';
	import { IconTrash, IconEdit } from '@tabler/icons-svelte';
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/data/Table.svelte';
	import { vendorsStore, editingVendor, vendorClient, type Vendor } from '$lib/features/vendors';
	import { taxoLog } from '$lib/features/taxoLog';
	import VendorsControls from './VendorsControls.svelte';
	import VendorsModal from './VendorsModal.svelte';
	import { exportData, type ExportFormat } from '$lib/utilities/formats/export';
	import type { CheckItem } from '$lib/components/core/primitives/data/TableFilter.svelte';

	interface ColumnConfig extends CheckItem {
		key: keyof Vendor;
	}

	let columnas = $state<ColumnConfig[]>([
		{ id: 'tipo_documento', key: 'tipo_documento', label: 'Tipo Doc.', checked: true },
		{ id: 'numero_documento', key: 'numero_documento', label: 'N° Documento', checked: true },
		{ id: 'nombre', key: 'nombre', label: 'Razón Social / Nombre', checked: true },
		{ id: 'nombre_comercial', key: 'nombre_comercial', label: 'Nombre Comercial', checked: true },
		{ id: 'direccion', key: 'direccion', label: 'Dirección', checked: false },
		{ id: 'telefono', key: 'telefono', label: 'Teléfono', checked: false },
		{ id: 'correo', key: 'correo', label: 'Correo', checked: false },
		{ id: 'contacto', key: 'contacto', label: 'Contacto', checked: false },
		{ id: 'cuenta_bancaria', key: 'cuenta_bancaria', label: 'Cuenta Bancaria', checked: false },
		{ id: 'observaciones', key: 'observaciones', label: 'Observaciones', checked: false },
		{ id: 'departamento', key: 'departamento', label: 'Departamento', checked: false },
		{ id: 'provincia', key: 'provincia', label: 'Provincia', checked: false },
		{ id: 'distrito', key: 'distrito', label: 'Distrito', checked: false }
	]);

	let columnasVisibles = $derived(columnas.filter((c) => c.checked));

	let isModalOpen = $state(false);
	let searchTerm = $state('');
	let searchBy = $state('');

	onMount(() => {
		vendorsStore.load();
	});

	let proveedoresFiltrados = $derived(
		$vendorsStore.filter((p) => {
			if (!searchTerm.trim()) return true;
			const term = searchTerm.toLowerCase();

			if (searchBy === 'numero_documento') {
				return p.numero_documento?.toLowerCase().includes(term) ?? false;
			} else if (searchBy === 'nombre') {
				return p.nombre.toLowerCase().includes(term);
			} else if (searchBy === 'nombre_comercial') {
				return p.nombre_comercial?.toLowerCase().includes(term) ?? false;
			} else if (searchBy === 'direccion') {
				return p.direccion?.toLowerCase().includes(term) ?? false;
			} else if (searchBy === 'telefono') {
				return p.telefono?.toLowerCase().includes(term) ?? false;
			} else if (searchBy === 'correo') {
				return p.correo?.toLowerCase().includes(term) ?? false;
			}

			return (
				p.nombre.toLowerCase().includes(term) ||
				(p.numero_documento && p.numero_documento.toLowerCase().includes(term)) ||
				(p.nombre_comercial && p.nombre_comercial.toLowerCase().includes(term)) ||
				(p.direccion && p.direccion.toLowerCase().includes(term)) ||
				(p.telefono && p.telefono.toLowerCase().includes(term)) ||
				(p.correo && p.correo.toLowerCase().includes(term)) ||
				(p.contacto && p.contacto.toLowerCase().includes(term))
			);
		})
	);

	function handleAdd() {
		$editingVendor = null;
		isModalOpen = true;
	}

	function handleEdit(proveedor: Vendor) {
		$editingVendor = proveedor;
		isModalOpen = true;
	}

	async function handleExportFile(format: ExportFormat) {
		const savedPath = await exportData(proveedoresFiltrados, columnasVisibles, 'proveedores', format);
		if (savedPath) {
			taxoLog.info(`Exportado exitosamente en: ${savedPath}`, 'proveedores');
		}
	}

	async function handleDelete(id: number, nombre: string) {
		if (confirm(`¿Estás seguro de eliminar el proveedor "${nombre}"?`)) {
			try {
				await vendorClient.deleteVendor(id);
				taxoLog.info(`Proveedor '${nombre}' eliminado`, 'proveedores');
				await vendorsStore.load();
			} catch (e: any) {
				const msg = e?.message ?? 'Error al eliminar el proveedor';
				taxoLog.error(msg, 'proveedores');
			}
		}
	}

	function formatCellValue(proveedor: Vendor, key: keyof Vendor): string {
		const val = proveedor[key];
		if (val === null || val === undefined || val === '') return '-';
		if (typeof val === 'boolean') {
			return val ? 'Sí' : 'No';
		}
		return String(val);
	}
</script>

<div class="grid h-full grid-rows-[auto_1fr] gap-1 text-sm">
	<VendorsControls
		bind:columnas
		bind:searchTerm
		bind:searchBy
		onAdd={handleAdd}
		onExport={handleExportFile}
	/>

	<VendorsModal bind:isOpen={isModalOpen} onClose={() => (isModalOpen = false)} />

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
			{#if proveedoresFiltrados.length === 0}
				<tr>
					{#snippet emptyState()}
						No hay proveedores registrados.
					{/snippet}
					{@render cell({
						colspan: columnasVisibles.length + 2,
						class: 'py-8 text-neutral-500',
						children: emptyState
					})}
				</tr>
			{:else}
				{#each proveedoresFiltrados as proveedor, index (proveedor.id)}
					{#snippet rowData()}
						{#snippet cellIndex()}
							{index + 1}
						{/snippet}
						{@render cell({ class: 'px-3 py-2 text-neutral-500', children: cellIndex })}
						{#each columnasVisibles as col (col.id)}
							{#snippet cellVal()}
								{formatCellValue(proveedor, col.key)}
							{/snippet}
							{@render cell({ children: cellVal })}
						{/each}
						{#snippet cellActions()}
							<div class="flex items-center justify-center gap-2">
								<button
									class="cursor-pointer text-neutral-400 transition-colors hover:text-blue-400"
									onclick={() => handleEdit(proveedor)}
									title="Editar"
								>
									<IconEdit size={16} />
								</button>
								<button
									class="cursor-pointer text-neutral-400 transition-colors hover:text-red-400"
									onclick={() => handleDelete(proveedor.id, proveedor.nombre)}
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
