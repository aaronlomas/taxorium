<script lang="ts">
	import Select from '$lib/components/core/primitives/Select.svelte';
	import Number from '$lib/components/core/primitives/Number.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';
	import { salesStore } from '$lib/stores/sales';
	import VoucherModal from '$lib/components/ui/views/vouchers/VoucherModal.svelte';
	import { voucherConfigStore } from '$lib/components/ui/views/vouchers/voucherContext';

	const medioDePagoContado = [
		{ value: '008', label: 'Efectivo' },
		{ value: '006', label: 'Tarjeta Visa (Crédito)' },
		{ value: '005', label: 'Tarjeta Visa (Débito)' },
		{ value: '003', label: 'Transferencia de Fondos (Yape/Plin)' }
	];
	let medioSeleccionado = $state('008');
	let montoRecibido = $state('0');
	let isVoucherModalOpen = $state(false);

	$effect(() => voucherConfigStore.updateField('medioPago', medioSeleccionado));
	$effect(() => {
		const opcion = medioDePagoContado.find((m) => m.value === medioSeleccionado);
		voucherConfigStore.updateField('medioPagoLabel', opcion?.label ?? medioSeleccionado);
	});
	$effect(() => voucherConfigStore.updateField('montoRecibido', montoRecibido));

	const CURRENCY_SYMBOL: Record<string, string> = {
		PEN: 'S/.',
		USD: '$',
		EUR: '€'
	};

	const moneda = $derived($salesStore[0]?.moneda ?? 'PEN');
	const symbol = $derived(CURRENCY_SYMBOL[moneda] ?? 'S/.');

	const exoneradoCodigos = ['20', '21', '30'];
	const icbperCodigos = ['71', '72'];

	const totalPagable = $derived($salesStore.reduce((sum, i) => sum + i.total, 0));
	const opExoneradas = $derived(
		$salesStore
			.filter((i) => exoneradoCodigos.includes(i.afectacion))
			.reduce((sum, i) => sum + i.total, 0)
	);
	const icbper = $derived(
		$salesStore
			.filter((i) => icbperCodigos.includes(i.afectacion))
			.reduce((sum, i) => sum + i.total, 0)
	);
	const vuelto = $derived(+montoRecibido - totalPagable);
</script>

<div class="grid h-full grid-cols-[1fr_auto] border-t border-r border-neutral-800">
	<div class="grid grid-cols-[auto_1fr] grid-rows-[auto_auto] gap-2">
		<div class="w-56">
			<Select
				id="medioDePago"
				label="Forma de Pago"
				options={medioDePagoContado}
				bind:value={medioSeleccionado}
			/>
		</div>
		<div class="w-56">
			<Number label="Monto Recibido" bind:value={montoRecibido} />
		</div>
		<Button onclick={() => (isVoucherModalOpen = true)}>Generar Venta</Button>
	</div>
	<div class="border-l border-neutral-800 text-sm">
		<table>
			<tbody>
				<tr class="border-b border-neutral-800">
					<td class="px-2 text-right">OP.EXONERADAS:</td>
					<td class="pr-2 text-right">{symbol} {opExoneradas.toFixed(2)}</td>
				</tr>
				<tr class="border-b border-neutral-800">
					<td class="px-2 text-right">I.C.B.P.E.R:</td>
					<td class="pr-2 text-right">{symbol} {icbper.toFixed(2)}</td>
				</tr>
				<tr class="border-b border-neutral-800">
					<th class="px-2 text-right">TOTAL A PAGAR:</th>
					<td class="pr-2 text-right">{symbol} {totalPagable.toFixed(2)}</td>
				</tr>
				<tr class="border-b border-neutral-800">
					<th class="px-2 text-right">TOTAL PAGADO:</th>
					<td class="pr-2 text-right"
						>{symbol} {montoRecibido && +montoRecibido > 0 ? montoRecibido : '0.00'}</td
					>
				</tr>
				<tr class="border-b border-neutral-800">
					<th class="px-2 text-right">VUELTO:</th>
					<td class="pr-2 text-right">{symbol} {vuelto.toFixed(2)}</td>
				</tr>
			</tbody>
		</table>
	</div>
</div>

<VoucherModal bind:isOpen={isVoucherModalOpen} onClose={() => (isVoucherModalOpen = false)} />
