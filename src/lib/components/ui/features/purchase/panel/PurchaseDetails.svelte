<script lang="ts">
	import Select from '$lib/components/core/primitives/forms/Select.svelte';
	import Number from '$lib/components/core/primitives/forms/Number.svelte';
	import Button from '$lib/components/core/primitives/actions/Button.svelte';
	import { purchaseStore } from '$lib/features/purchase';
	import { tiposPagoOptions } from '$lib/features/catalogos';
	import { taxoLog } from '$lib/features/taxoLog';
	import { purchaseConfigStore } from '$lib/components/ui/features/purchase/purchaseContext';

	let formaPago = $state('Contado');
	let montoPagado = $state('0');

	$effect(() => purchaseConfigStore.updateField('formaPago', formaPago));
	$effect(() => purchaseConfigStore.updateField('montoPagado', montoPagado));

	const CURRENCY_SYMBOL: Record<string, string> = {
		PEN: 'S/.',
		USD: '$',
		EUR: '€'
	};

	const moneda = $derived($purchaseStore[0]?.moneda ?? 'PEN');
	const symbol = $derived(CURRENCY_SYMBOL[moneda] ?? 'S/.');

	/** IGV de referencia para compras gravadas (18% del valor neto). */
	const IGV_TASA = 0.18;

	/** Las afectaciones de compra gravadas no empiezan con "NO_GRAVADO". */
	function esGravado(afectacion: string): boolean {
		return afectacion.includes('GRAVADO') && !afectacion.startsWith('NO_GRAVADO');
	}

	const subtotalProductos = $derived($purchaseStore.reduce((sum, i) => sum + i.subtotal, 0));
	const opGravadas = $derived(
		$purchaseStore.filter((i) => esGravado(i.afectacion)).reduce((sum, i) => sum + i.subtotal, 0)
	);
	const opExoneradas = $derived(subtotalProductos - opGravadas);
	const igv = $derived(opGravadas * IGV_TASA);
	const totalPagable = $derived(subtotalProductos + igv);
	const saldo = $derived(+montoPagado - totalPagable);

	function handleRegistrar() {
		if ($purchaseStore.length === 0) return;

		taxoLog.info(
			`Compra registrada: ${$purchaseStore.length} ítem(s), total ${symbol} ${totalPagable.toFixed(2)}`,
			'compras'
		);
		purchaseStore.clear();
		purchaseConfigStore.reset();
		montoPagado = '0';
	}
</script>

<div
	class="grid h-full grid-cols-[290px_1fr] rounded-md border border-t border-brand-border bg-brand-bg-primary p-2"
>
	<!-- Controles -->
	<div class="grid grid-cols-2 grid-rows-[auto_auto] gap-2">
		<Select
			id="formaPago"
			label="Forma de Pago"
			options={$tiposPagoOptions}
			bind:value={formaPago}
		/>
		<Number label="Monto Pagado" bind:value={montoPagado} />
		<Button onclick={handleRegistrar}>Registrar Compra</Button>
	</div>
	<!-- Cálculos -->
	<div class="flex justify-end text-sm text-brand-text-secondary">
		<table>
			<tbody>
				{#if opGravadas > 0}
					<tr class="border-b border-brand-border">
						<td class="px-2 text-right">OP.GRAVADAS:</td>
						<td class="pr-2 text-right">{symbol} {opGravadas.toFixed(2)}</td>
					</tr>
				{/if}
				{#if opExoneradas > 0}
					<tr class="border-b border-brand-border">
						<td class="px-2 text-right">OP.EXONERADAS:</td>
						<td class="pr-2 text-right">{symbol} {opExoneradas.toFixed(2)}</td>
					</tr>
				{/if}
				{#if igv > 0}
					<tr class="border-b border-brand-border">
						<td class="px-2 text-right">I.G.V. (18%):</td>
						<td class="pr-2 text-right">{symbol} {igv.toFixed(2)}</td>
					</tr>
				{/if}
				<tr class="border-b border-brand-border">
					<th class="px-2 text-right">TOTAL A PAGAR:</th>
					<td class="pr-2 text-right">{symbol} {totalPagable.toFixed(2)}</td>
				</tr>
				<tr class="border-b border-brand-border">
					<th class="px-2 text-right">TOTAL PAGADO:</th>
					<td class="pr-2 text-right"
						>{symbol} {montoPagado && +montoPagado > 0 ? montoPagado : '0.00'}</td
					>
				</tr>
				<tr class="border-b border-brand-border">
					<th class="px-2 text-right">SALDO:</th>
					<td class="pr-2 text-right">{symbol} {saldo.toFixed(2)}</td>
				</tr>
			</tbody>
		</table>
	</div>
</div>
