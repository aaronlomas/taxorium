<script lang="ts">
	import { onMount } from 'svelte';
	import Modal from '$lib/components/core/primitives/feedback/Modal.svelte';
	import Button from '$lib/components/core/primitives/actions/Button.svelte';
	import { salesStore } from '$lib/features/sales';
	import { icbperStore } from '$lib/features/settings';
	import { catalogoStore } from '$lib/features/catalogos';
	import { customersStore } from '$lib/features/customers';
	import { voucherConfigStore } from '$lib/components/ui/features/sales/vouchers/state/voucherContext';
	import { calculateSalesTaxes, clasificarAfectacion, type SalesTaxesSummary } from './salesTaxes';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		montoRecibido?: string | number;
		medioPagoLabel?: string;
	}

	let {
		isOpen = $bindable(false),
		onClose,
		montoRecibido = '0',
		medioPagoLabel = 'Efectivo'
	}: Props = $props();

	onMount(() => {
		catalogoStore.load();
	});

	const CURRENCY_SYMBOL: Record<string, string> = {
		PEN: 'S/.',
		USD: '$',
		EUR: '€'
	};

	const moneda = $derived($salesStore[0]?.moneda ?? $voucherConfigStore.moneda ?? 'PEN');
	const symbol = $derived(CURRENCY_SYMBOL[moneda] ?? 'S/.');

	const taxes = $derived<SalesTaxesSummary>(
		calculateSalesTaxes($salesStore, $catalogoStore.afectacionesVenta, $icbperStore)
	);

	const montoRecibidoNum = $derived(Number(montoRecibido) || 0);
	const vuelto = $derived(
		montoRecibidoNum > taxes.totalPagable
			? Math.round((montoRecibidoNum - taxes.totalPagable) * 100) / 100
			: 0
	);
	const saldoPendiente = $derived(
		taxes.totalPagable > montoRecibidoNum && montoRecibidoNum > 0
			? Math.round((taxes.totalPagable - montoRecibidoNum) * 100) / 100
			: 0
	);

	const cliente = $derived(
		$customersStore.find((c) => String(c.id) === String($voucherConfigStore.clienteId))
	);

	const tipoComprobanteLabel = $derived(
		$voucherConfigStore.tipoComprobante === '01'
			? 'Factura Electrónica (01)'
			: $voucherConfigStore.tipoComprobante === '03'
				? 'Boleta de Venta Electrónica (03)'
				: `Comprobante (${$voucherConfigStore.tipoComprobante || '-'})`
	);

	const totalUnidades = $derived($salesStore.reduce((sum, i) => sum + i.cantidad, 0));

	const FALLBACK_AFECTACIONES = [
		{ codigo: '10', descripcion: 'Gravado - Op. Onerosa (IGV 18%)' },
		{ codigo: '11', descripcion: 'Gravado - Retiro (Transferencia Gratuita)' },
		{ codigo: '20', descripcion: 'Exonerado - Op. Onerosa (IGV 0%)' },
		{ codigo: '21', descripcion: 'Exonerado - Transferencia Gratuita' },
		{ codigo: '30', descripcion: 'Inafecto - Op. Onerosa (IGV 0%)' },
		{ codigo: '31', descripcion: 'Inafecto - Retiro (Transferencia Gratuita)' },
		{ codigo: '40', descripcion: 'Exportación de Bienes o Servicios' }
	];

	// Muestra TODAS las afectaciones registradas en la tabla `afectaciones_venta`
	// para fines de auditoría y transparencia, incluso si su importe es 0.00
	const todasAfectaciones = $derived.by(() => {
		const catalogo = $catalogoStore.afectacionesVenta;
		const baseList = catalogo.length > 0 ? catalogo : FALLBACK_AFECTACIONES;

		const map = new Map<string, { codigo: string; descripcion: string }>();
		for (const a of baseList) {
			map.set(a.codigo, a);
		}
		for (const item of $salesStore) {
			if (item.afectacion && !map.has(item.afectacion)) {
				map.set(item.afectacion, {
					codigo: item.afectacion,
					descripcion: `Afectación ${item.afectacion}`
				});
			}
		}

		const lista = Array.from(map.values()).sort((a, b) => a.codigo.localeCompare(b.codigo));

		return lista.map((af) => {
			const itemsConEsta = $salesStore.filter(
				(i) => (i.afectacion || '20') === af.codigo
			);
			const totalBruto = itemsConEsta.reduce((sum, i) => sum + i.total, 0);
			const { categoria } = clasificarAfectacion(af.codigo, af.descripcion);

			let monto = 0;
			if (categoria === 'gravado') {
				const igvItem = Math.round(totalBruto * (0.18 / 1.18) * 100) / 100;
				monto = Math.round((totalBruto - igvItem) * 100) / 100;
			} else {
				monto = totalBruto;
			}

			return {
				codigo: af.codigo,
				descripcion: af.descripcion,
				categoria,
				monto,
				cantidadItems: itemsConEsta.length,
				esGratuita: categoria === 'gratuito'
			};
		});
	});
</script>

<Modal bind:isOpen {onClose} title="Detalles">
	<div class="flex max-h-[75vh] w-125 overflow-y-auto flex-col gap-3 p-3 text-xs">
		<div>
			<table class="w-full border-collapse text-xs text-brand-text-secondary">
				<tbody>
					<!-- SECCIÓN: DATOS DE LA VENTA / COMPROBANTE -->
					<tr class="border-b border-brand-border bg-brand-bg-secondary/70">
						<th
							colspan="2"
							class="px-2 py-1 text-left font-semibold text-brand-text-sky uppercase text-[11px]"
						>
							INFORMACIÓN DEL COMPROBANTE
						</th>
					</tr>
					<tr class="border-b border-brand-border">
						<td class="w-3/5 px-2 py-1 text-right font-medium">COMPROBANTE:</td>
						<td class="w-2/5 pr-2 text-right text-brand-text-primary">
							{tipoComprobanteLabel}
						</td>
					</tr>
					<tr class="border-b border-brand-border">
						<td class="px-2 py-1 text-right font-medium">SERIE:</td>
						<td class="pr-2 text-right text-brand-text-primary font-mono">
							{$voucherConfigStore.serie || '-'}
						</td>
					</tr>
					<tr class="border-b border-brand-border">
						<td class="px-2 py-1 text-right font-medium uppercase">
							{cliente?.tipo_documento ? `${cliente.tipo_documento}:` : 'RUC:'}
						</td>
						<td class="pr-2 text-right text-brand-text-primary font-mono">
							{cliente?.numero_documento || '-'}
						</td>
					</tr>
					<tr class="border-b border-brand-border">
						<td class="px-2 py-1 text-right font-medium">CLIENTE:</td>
						<td
							class="pr-2 text-right text-brand-text-primary truncate"
							title={cliente ? cliente.nombre : 'Clientes Varios'}
						>
							{cliente ? cliente.nombre : 'Clientes Varios'}
						</td>
					</tr>
					<tr class="border-b border-brand-border">
						<td class="px-2 py-1 text-right font-medium">CONDICIÓN DE PAGO:</td>
						<td class="pr-2 text-right text-brand-text-primary">
							{$voucherConfigStore.tipoPago || 'Contado'}
						</td>
					</tr>
					<tr class="border-b border-brand-border">
						<td class="px-2 py-1 text-right font-medium">MEDIO DE PAGO:</td>
						<td class="pr-2 text-right text-brand-text-primary">
							{medioPagoLabel}
						</td>
					</tr>
					<tr class="border-b border-brand-border">
						<td class="px-2 py-1 text-right font-medium">MONEDA:</td>
						<td class="pr-2 text-right text-brand-text-primary">
							{moneda === 'USD' ? 'Dólares Americanos (USD)' : 'Soles (PEN)'}
						</td>
					</tr>
					{#if moneda === 'USD' && Number($voucherConfigStore.tipoCambio) > 0}
						<tr class="border-b border-brand-border">
							<td class="px-2 py-1 text-right font-medium">TIPO DE CAMBIO:</td>
							<td class="pr-2 text-right text-brand-text-primary font-mono">
								{Number($voucherConfigStore.tipoCambio).toFixed(2)}
							</td>
						</tr>
					{/if}
					<tr class="border-b border-brand-border">
						<td class="px-2 py-1 text-right font-medium">PRODUCTOS / ÍTEMS:</td>
						<td class="pr-2 text-right text-brand-text-primary">
							{$salesStore.length} producto(s) ({totalUnidades} unds.)
						</td>
					</tr>

					<!-- SECCIÓN: TODAS LAS AFECTACIONES DE LA TABLA AFECTACIONES_VENTA -->
					<tr class="border-b border-brand-border bg-brand-bg-secondary/70">
						<th
							colspan="2"
							class="px-2 py-1 pt-2 text-left font-semibold text-brand-text-sky uppercase text-[11px]"
						>
							AFECTACIONES E IMPUESTOS
						</th>
					</tr>

					{#each todasAfectaciones as af (af.codigo)}
						<tr class="border-b border-brand-border">
							<td class="px-2 py-1 text-right uppercase text-[11px]" title="{af.codigo} - {af.descripcion}">
								[{af.codigo}] {af.descripcion.toUpperCase()}:
							</td>
							<td class="pr-2 text-right font-mono text-brand-text-primary">
								{symbol} {af.monto.toFixed(2)}
								{#if af.esGratuita && af.monto > 0}
									<span class="text-[10px] text-brand-text-secondary">(REF)</span>
								{/if}
							</td>
						</tr>
						<!-- Después de la operación gravada base (10), mostrar el IGV correspondiente -->
						{#if af.codigo === '10'}
							<tr class="border-b border-brand-border">
								<td class="px-2 py-1 text-right text-brand-text-secondary uppercase">
									IGV (18%):
								</td>
								<td class="pr-2 text-right font-mono text-brand-text-primary">
									{symbol} {taxes.igv.toFixed(2)}
								</td>
							</tr>
						{/if}
					{/each}

					<!-- I.C.B.P.E.R -->
					<tr class="border-b border-brand-border">
						<td class="px-2 py-1 text-right uppercase">
							I.C.B.P.E.R:
						</td>
						<td class="pr-2 text-right font-mono text-brand-text-primary">
							{symbol} {taxes.icbper.toFixed(2)}
							{#if taxes.bolsasIcbper > 0}
								<span class="text-[10px] text-brand-text-secondary">({taxes.bolsasIcbper} bols.)</span>
							{/if}
						</td>
					</tr>

					<!-- SECCIÓN: TOTALES Y LIQUIDACIÓN -->
					<tr class="border-b border-brand-border bg-brand-bg-secondary/70">
						<th
							colspan="2"
							class="px-2 py-1 pt-2 text-left font-semibold text-brand-text-sky uppercase text-[11px]"
						>
							TOTALES Y LIQUIDACIÓN
						</th>
					</tr>
					<tr class="border-b border-brand-border">
						<td class="px-2 py-1 text-right font-medium uppercase">SUBTOTAL (VALOR VENTA):</td>
						<td class="pr-2 text-right font-mono text-brand-text-primary">
							{symbol} {taxes.baseImponibleTotal.toFixed(2)}
						</td>
					</tr>
					<tr class="border-b border-brand-border">
						<td class="px-2 py-1 text-right font-medium uppercase">TOTAL IMPUESTOS:</td>
						<td class="pr-2 text-right font-mono text-brand-text-primary">
							{symbol} {taxes.impuestosTotal.toFixed(2)}
						</td>
					</tr>
					<tr class="border-b border-brand-border">
						<th class="px-2 py-1 text-right text-brand-text-primary uppercase">TOTAL A PAGAR:</th>
						<td class="pr-2 text-right font-mono font-bold text-brand-text-primary">
							{symbol} {taxes.totalPagable.toFixed(2)}
						</td>
					</tr>
					<tr class="border-b border-brand-border">
						<th class="px-2 py-1 text-right text-brand-text-primary uppercase">TOTAL PAGADO:</th>
						<td class="pr-2 text-right font-mono font-bold text-brand-text-primary">
							{symbol} {montoRecibidoNum > 0 ? montoRecibidoNum.toFixed(2) : '0.00'}
						</td>
					</tr>
					<tr class="border-b border-brand-border">
						<th class="px-2 py-1 text-right text-brand-text-primary uppercase">VUELTO:</th>
						<td class="pr-2 text-right font-mono font-bold text-brand-text-primary">
							{symbol} {vuelto.toFixed(2)}
						</td>
					</tr>
					{#if saldoPendiente > 0}
						<tr class="border-b border-brand-border">
							<th class="px-2 py-1 text-right text-amber-400 uppercase">SALDO PENDIENTE:</th>
							<td class="pr-2 text-right font-mono font-bold text-amber-400">
								{symbol} {saldoPendiente.toFixed(2)}
							</td>
						</tr>
					{/if}
				</tbody>
			</table>
		</div>

		<!-- BOTÓN CERRAR -->
		<div class="flex justify-end pt-2">
			<Button variant="outline" size="sm" onclick={onClose}>Cerrar</Button>
		</div>
	</div>
</Modal>
