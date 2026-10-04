<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconDatabaseImport,
		IconDatabaseExport,
		IconDownload,
		IconSend
	} from '@tabler/icons-svelte';
	import TableFilter, { type CheckItem } from '$lib/components/core/primitives/TableFilter.svelte';
	import TableToolbar from '$lib/components/core/primitives/TableToolbar.svelte';
	import Select from '$lib/components/core/primitives/Select.svelte';
	import Search from '$lib/components/core/primitives/Search.svelte';
	import InputDate from '$lib/components/core/primitives/InputDate.svelte';
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/Table.svelte';

	import { exportData, type ExportFormat } from '$lib/utilities/formats/export';
	import { taxoLog } from '$lib/stores/taxoLog';
	import { formatFecha, parseFecha } from '$lib/components/ui/views/vouchers/voucherContext';
	import { voucherClient, type Voucher } from '$lib/services/vouchers/clientVoucher';
	import { invoke } from '@tauri-apps/api/core';
	import { get } from 'svelte/store';
	import { tenantStore } from '$lib/stores/tenant';
	import { save } from '@tauri-apps/plugin-dialog';
	import { writeFile } from '@tauri-apps/plugin-fs';

	interface ColumnConfig extends CheckItem {
		key: string;
	}

	let columnas = $state<ColumnConfig[]>([
		{ id: 'fecha_de_emision', key: 'fecha_de_emision', label: 'Fecha Emisión', checked: true },
		{ id: 'numero_comprobante', key: 'numero_comprobante', label: 'Comprobante', checked: true },
		{ id: 'cliente', key: 'cliente', label: 'Cliente', checked: true },
		{ id: 'estado_validez', key: 'estado_validez', label: 'Estado', checked: true },
		{ id: 'estado_sunat', key: 'estado_sunat', label: 'Enviado a SUNAT', checked: true },
		{ id: 'codigo_cdr', key: 'codigo_cdr', label: 'Código CDR', checked: true },
		{ id: 'descripcion_cdr', key: 'descripcion_cdr', label: 'Respuesta CDR', checked: true }
	]);

	let columnasVisibles = $derived(columnas.filter((c) => c.checked));

	let vouchers = $state<Voucher[]>([]);
	let isLoading = $state(true);

	let searchTerm = $state('');
	let searchBy = $state('');
	let fechaDesde = $state('');

	const SEARCH_OPTIONS = [
		{ value: '', label: 'Todos' },
		{ value: 'comprobante', label: 'Comprobante' },
		{ value: 'cliente', label: 'Cliente' },
		{ value: 'estado', label: 'Estado' }
	];

	onMount(async () => {
		try {
			// En el futuro, aquí se consultaría una tabla o vista unificada "envios_sunat"
			// que devuelva tanto comprobantes individuales como resúmenes diarios (RC/RA).
			// Por ahora mostramos los comprobantes.
			vouchers = await voucherClient.getVouchers();
		} catch (e) {
			taxoLog.error(`Error al cargar documentos SUNAT: ${e}`, 'sunat');
		} finally {
			isLoading = false;
		}
	});

	let enviosFiltrados = $derived(
		vouchers.filter((v) => {
			if (fechaDesde) {
				const desde = parseFecha(fechaDesde);
				const emision = parseFecha(v.fecha_de_emision);
				if (desde && emision && emision < desde) return false;
			}

			if (!searchTerm.trim()) return true;
			const term = searchTerm.toLowerCase();

			if (searchBy === 'comprobante') {
				return v.numero_comprobante.toLowerCase().includes(term);
			} else if (searchBy === 'cliente') {
				return v.cliente.toLowerCase().includes(term);
			} else if (searchBy === 'estado') {
				return v.estado_validez.toLowerCase().includes(term);
			}

			return (
				v.numero_comprobante.toLowerCase().includes(term) ||
				v.cliente.toLowerCase().includes(term) ||
				v.estado_validez.toLowerCase().includes(term)
			);
		})
	);

	async function handleExportFile(format: ExportFormat) {
		const savedPath = await exportData(enviosFiltrados, columnasVisibles, 'envios_sunat', format);
		if (savedPath) {
			taxoLog.info(`Exportado exitosamente en: ${savedPath}`, 'sunat');
		}
	}

	let isEmitting = $state<number | null>(null);

	async function handleEmitir(doc: Voucher) {
		if (doc.estado_sunat !== 0) {
			taxoLog.warn('Este comprobante ya fue emitido a SUNAT.', 'sunat');
			return;
		}

		const tenant = get(tenantStore).tenant;
		if (!tenant || !tenant.usuario_sol || !tenant.clave_sol || !tenant.ruc) {
			taxoLog.error('Faltan configurar las credenciales SOL de la empresa.', 'sunat');
			return;
		}

		isEmitting = doc.id;
		taxoLog.info(`Enviando ${doc.numero_comprobante} a SUNAT...`, 'sunat');

		try {
			const res = await invoke('enviar_a_sunat', {
				payload: {
					ruc: tenant.ruc,
					usuario_sol: tenant.usuario_sol,
					clave_sol: tenant.clave_sol,
					client_id: (tenant as any).sunat_client_id || '25f61db1-0854-4efb-bc54-c1f10cf8db17',
					client_secret: (tenant as any).sunat_client_secret || 'rvSu0MjYw+hB+vxONyA7jA==',
					ambiente: 'beta',
					voucher_id: doc.id
				}
			});
			taxoLog.info(
				`Comprobante aceptado por SUNAT. CDR: ${(res as any).descripcion}`,
				'sunat'
			);
			vouchers = await voucherClient.getVouchers();
		} catch (e) {
			taxoLog.error(`Error de SUNAT: ${e}`, 'sunat');
		} finally {
			isEmitting = null;
		}
	}

	async function handleDescargar(doc: Voucher, tipo: 'XML' | 'CDR') {
		try {
			const contenido = await invoke<number[]>('descargar_documento_sunat', {
				voucherId: doc.id,
				tipo
			});
			const nombreArchivo = `${doc.numero_comprobante.replace(/[^a-zA-Z0-9-]/g, '_')}_${tipo}.xml`;
			const filePath = await save({
				defaultPath: nombreArchivo,
				filters: [{ name: 'XML', extensions: ['xml', 'zip'] }]
			});
			if (filePath) {
				await writeFile(filePath, new Uint8Array(contenido));
				taxoLog.info(`Archivo ${tipo} guardado en: ${filePath}`, 'sunat');
			}
		} catch (e) {
			taxoLog.error(`Error al descargar ${tipo}: ${e}`, 'sunat');
		}
	}

	function formatCellValue(doc: Voucher, key: string): string {
		const val = (doc as any)[key];
		if (val === null || val === undefined) return '-';
		if (key === 'fecha_de_emision') {
			const s = String(val);
			return /^\d{2}\/\d{2}\/\d{4}/.test(s) ? s.slice(0, 10) : formatFecha(s);
		}
		if (key === 'estado_sunat') {
			return Number(val) === 0 ? 'Pendiente' : 'Enviado';
		}
		if (key === 'codigo_cdr') {
			const code = String(val);
			if (code === '0') return 'Aceptado';
			if (parseInt(code) > 0) return `Error (${code})`;
			return '-';
		}
		return String(val);
	}

	function estadoClass(estado: string): string {
		switch (estado) {
			case 'aceptado':
				return 'text-emerald-400';
			case 'rechazado':
				return 'text-red-400';
			default:
				return 'text-amber-400';
		}
	}

	function cdrClass(codigo: string | null): string {
		if (codigo === '0') return 'text-emerald-400';
		if (codigo && parseInt(codigo) > 0) return 'text-red-400';
		return 'text-neutral-500';
	}
</script>

<div class="grid h-full grid-rows-[auto_1fr] gap-2 px-2 pb-2 text-sm">
	<!-- PRIMITIVA TOOLBAR -->
	<TableToolbar>
		{#snippet actions()}
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
			<TableFilter storageKey="envios_sunat" bind:items={columnas} />
		{/snippet}

		{#snippet filters()}
			<div class="flex items-end gap-2">
				<div class="w-40">
					<InputDate label="Desde la fecha:" bind:value={fechaDesde} />
				</div>
			</div>
			<div class="flex items-end gap-2">
				<div class="w-30">
					<Select placeholder="Buscar por:" options={SEARCH_OPTIONS} bind:value={searchBy} />
				</div>
				<Search
					bind:value={searchTerm}
					placeholder={searchBy === 'comprobante'
						? 'Buscar comprobante...'
						: searchBy === 'cliente'
							? 'Buscar cliente...'
							: searchBy === 'estado'
								? 'Buscar estado...'
								: 'Buscar en todos...'}
				/>
			</div>
		{/snippet}
	</TableToolbar>

	<!-- TABLA DE RESÚMENES -->
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
				{#snippet headDownload()}
					Descargar
				{/snippet}
				{@render headCell({ children: headDownload })}
				{#snippet headActions()}
					Acciones
				{/snippet}
				{@render headCell({ children: headActions })}
			</tr>
		{/snippet}

		{#snippet body()}
			{#if isLoading}
				<tr>
					{#snippet loadingState()}
						Cargando documentos SUNAT...
					{/snippet}
					{@render cell({
						colspan: columnasVisibles.length + 3,
						class: 'py-8 text-neutral-500',
						children: loadingState
					})}
				</tr>
			{:else if enviosFiltrados.length === 0}
				<tr>
					{#snippet emptyState()}
						No hay documentos enviados a SUNAT.
					{/snippet}
					{@render cell({
						colspan: columnasVisibles.length + 3,
						class: 'py-8 text-neutral-500',
						children: emptyState
					})}
				</tr>
			{:else}
				{#each enviosFiltrados as doc, index (doc.id)}
					{#snippet rowData()}
						{#snippet cellIndex()}
							{index + 1}
						{/snippet}
						{@render cell({ class: 'px-3 py-2 text-neutral-500', children: cellIndex })}
						{#each columnasVisibles as col (col.id)}
							{#snippet cellVal()}
								{#if col.key === 'estado_validez'}
									<span class={estadoClass(doc.estado_validez)}>
										{doc.estado_validez.charAt(0).toUpperCase() + doc.estado_validez.slice(1)}
									</span>
								{:else}
									{formatCellValue(doc, col.key)}
								{/if}
							{/snippet}
							{@render cell({ children: cellVal })}
						{/each}
						{#snippet cellDownload()}
							<div class="flex items-center justify-center gap-2">
								<button
									class="flex cursor-pointer items-center gap-1 rounded-sm border border-blue-500/40 px-2 py-0.5 text-blue-400 transition-colors hover:bg-blue-500/10"
									onclick={() => handleDescargar(doc, 'XML')}
									title="Descargar XML"
								>
									<IconDownload size={14} />
									XML
								</button>
								<button
									class="flex cursor-pointer items-center gap-1 rounded-sm border border-blue-500/40 px-2 py-0.5 text-blue-400 transition-colors hover:bg-blue-500/10 disabled:opacity-50"
									onclick={() => handleDescargar(doc, 'CDR')}
									title="Descargar CDR"
									disabled={!doc.codigo_cdr}
								>
									<IconDownload size={14} />
									CDR
								</button>
							</div>
						{/snippet}
						{@render cell({ children: cellDownload })}
						{#snippet cellActions()}
							<div class="flex items-center justify-center gap-2">
								{#if doc.estado_sunat === 0}
									<button
										class="flex cursor-pointer items-center gap-1 rounded-sm border border-emerald-500/40 px-2 py-0.5 text-emerald-400 transition-colors hover:bg-emerald-500/10 disabled:opacity-50"
										onclick={() => handleEmitir(doc)}
										title="Emitir a SUNAT"
										disabled={isEmitting === doc.id}
									>
										{#if isEmitting === doc.id}
											<span
												class="inline-block h-3 w-3 animate-spin rounded-full border-2 border-emerald-400 border-t-transparent"
											></span>
											Enviando...
										{:else}
											<IconSend size={14} />
											Emitir
										{/if}
									</button>
								{/if}
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
