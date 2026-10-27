<script lang="ts">
	import { onMount } from 'svelte';
	import Select from '$lib/components/core/primitives/forms/Select.svelte';
	import Number from '$lib/components/core/primitives/forms/Number.svelte';
	import Button from '$lib/components/core/primitives/actions/Button.svelte';
	import { salesStore } from '$lib/features/sales';
	import { icbperStore } from '$lib/features/settings';
	import { catalogoStore } from '$lib/features/catalogos';
	import VoucherModal from '$lib/components/ui/features/sales/vouchers/components/VoucherModal.svelte';
	import SalesDetailsModal from './SalesDetailsModal.svelte';
	import { voucherConfigStore } from '$lib/components/ui/features/sales/vouchers/state/voucherContext';
	import { calculateSalesTaxes, type SalesTaxesSummary } from './salesTaxes';
	import { IconWindowMaximize } from '@tabler/icons-svelte';

	const medioDePagoContado = [
		{ value: '008', label: 'Efectivo' },
		{ value: '006', label: 'Tarjeta Visa (Crédito)' },
		{ value: '005', label: 'Tarjeta Visa (Débito)' },
		{ value: '003', label: 'Transferencia de Fondos (Yape/Plin)' }
	];
	let medioSeleccionado = $state('008');
	let montoRecibido = $state('0');
	let isVoucherModalOpen = $state(false);
	let isDetailsModalOpen = $state(false);

	onMount(() => {
		catalogoStore.load();
	});

	const opcionMedio = $derived(medioDePagoContado.find((m) => m.value === medioSeleccionado));
	const medioPagoLabel = $derived(opcionMedio?.label ?? medioSeleccionado);

	$effect(() => voucherConfigStore.updateField('medioPago', medioSeleccionado));
	$effect(() => voucherConfigStore.updateField('medioPagoLabel', medioPagoLabel));
	$effect(() => voucherConfigStore.updateField('montoRecibido', montoRecibido));

	const CURRENCY_SYMBOL: Record<string, string> = {
		PEN: 'S/.',
		USD: '$',
		EUR: '€'
	};

	const moneda = $derived($salesStore[0]?.moneda ?? 'PEN');
	const symbol = $derived(CURRENCY_SYMBOL[moneda] ?? 'S/.');

	// Cálculo unificado y dinámico de afectaciones según la tabla afectaciones_venta
	const taxes = $derived<SalesTaxesSummary>(
		calculateSalesTaxes($salesStore, $catalogoStore.afectacionesVenta, $icbperStore)
	);

	const opGravadas = $derived(taxes.opGravadas);
	const igv = $derived(taxes.igv);
	const opExoneradas = $derived(taxes.opExoneradas);
	const opInafectas = $derived(taxes.opInafectas);
	const opGratuitas = $derived(taxes.opGratuitas);
	const opExportacion = $derived(taxes.opExportacion);
	const icbper = $derived(taxes.icbper);
	const totalPagable = $derived(taxes.totalPagable);
	const vuelto = $derived(
		+montoRecibido > totalPagable ? Math.round((+montoRecibido - totalPagable) * 100) / 100 : 0
	);
</script>

<div
	class="relative grid h-full grid-cols-[290px_1fr] rounded-md border border-t border-brand-border bg-brand-bg-primary p-2"
>
	<!-- Controles -->
	<div class="grid grid-cols-2 grid-rows-[auto_auto] gap-2">
		<Select
			id="medioDePago"
			label="Medio de Pago"
			options={medioDePagoContado}
			bind:value={medioSeleccionado}
		/>
		<Number label="Monto Recibido" bind:value={montoRecibido} />
		<Button onclick={() => (isVoucherModalOpen = true)}>Generar Venta</Button>
	</div>

	<!-- Cálculos Dinámicos -->
	<div class="flex justify-end text-sm text-brand-text-secondary">
		<table class="mr-6">
			<tbody>
				<!-- OP. GRAVADAS E IGV -->
				{#if opGravadas > 0 || igv > 0}
					<tr class="border-b border-brand-border">
						<td class="px-2 text-right">OP.GRAVADAS:</td>
						<td class="pr-2 text-right">{symbol} {opGravadas.toFixed(2)}</td>
					</tr>
					<tr class="border-b border-brand-border">
						<td class="px-2 text-right">IGV (18%):</td>
						<td class="pr-2 text-right">{symbol} {igv.toFixed(2)}</td>
					</tr>
				{/if}

				<!-- OP. EXONERADAS -->
				{#if opExoneradas > 0}
					<tr class="border-b border-brand-border">
						<td class="px-2 text-right">OP.EXONERADAS:</td>
						<td class="pr-2 text-right">{symbol} {opExoneradas.toFixed(2)}</td>
					</tr>
				{/if}

				<!-- OP. INAFECTAS (Dinámico según afectaciones_venta) -->
				{#if opInafectas > 0}
					<tr class="border-b border-brand-border">
						<td class="px-2 text-right">OP.INAFECTAS:</td>
						<td class="pr-2 text-right">{symbol} {opInafectas.toFixed(2)}</td>
					</tr>
				{/if}

				<!-- OP. GRATUITAS (Transferencias gratuitas / retiros) -->
				{#if opGratuitas > 0}
					<tr class="border-b border-brand-border">
						<td class="px-2 text-right">OP.GRATUITAS (REF):</td>
						<td class="pr-2 text-right">{symbol} {opGratuitas.toFixed(2)}</td>
					</tr>
				{/if}

				<!-- OP. EXPORTACIÓN -->
				{#if opExportacion > 0}
					<tr class="border-b border-brand-border">
						<td class="px-2 text-right">OP.EXPORTACIÓN:</td>
						<td class="pr-2 text-right">{symbol} {opExportacion.toFixed(2)}</td>
					</tr>
				{/if}

				<!-- I.C.B.P.E.R -->
				{#if icbper > 0}
					<tr class="border-b border-brand-border">
						<td class="px-2 text-right">I.C.B.P.E.R:</td>
						<td class="pr-2 text-right">{symbol} {icbper.toFixed(2)}</td>
					</tr>
				{/if}

				<!-- TOTALES -->
				<tr class="border-b border-brand-border">
					<th class="px-2 text-right">TOTAL A PAGAR:</th>
					<td class="pr-2 text-right">{symbol} {totalPagable.toFixed(2)}</td>
				</tr>
				<tr class="border-b border-brand-border">
					<th class="px-2 text-right">TOTAL PAGADO:</th>
					<td class="pr-2 text-right"
						>{symbol} {montoRecibido && +montoRecibido > 0 ? (+montoRecibido).toFixed(2) : '0.00'}</td
					>
				</tr>
				<tr class="border-b border-brand-border">
					<th class="px-2 text-right">VUELTO:</th>
					<td class="pr-2 text-right">{symbol} {vuelto.toFixed(2)}</td>
				</tr>
			</tbody>
		</table>
	</div>

	<!-- Botón para ver desglose detallado de afectaciones e impuestos -->
	<button
		type="button"
		class="absolute right-0 top-0 flex cursor-pointer items-center justify-center rounded text-brand-text-secondary hover:text-brand-text-primary"
		onclick={() => (isDetailsModalOpen = true)}
	>
		<IconWindowMaximize size={18} />
	</button>
</div>

<VoucherModal bind:isOpen={isVoucherModalOpen} onClose={() => (isVoucherModalOpen = false)} />

<SalesDetailsModal
	bind:isOpen={isDetailsModalOpen}
	onClose={() => (isDetailsModalOpen = false)}
	{montoRecibido}
	{medioPagoLabel}
/>

