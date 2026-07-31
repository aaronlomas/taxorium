<script lang="ts">
	const medioDePagoContado = [
		{ value: '008', label: 'Efectivo' },
		{ value: '006', label: 'Tarjeta Visa (Crédito)' },
		{ value: '005', label: 'Tarjeta Visa (Débito)' },
		{ value: '003', label: 'Transferencia de Fondos (Yape/Plin)' } // (Yape/Plin o cuentas)
	];
	let medioSeleccionado = '008';

	//Si se selecciona tipo de pago en crédito
	const condicionesDePago = [
		{ value: 'Contado', label: 'Pago al Contado' },
		{ value: 'Credito', label: 'Pago al Crédito' }
	];
	let condicionSeleccionada = 'Contado';
</script>

<div class="mt-4 grid h-full grid-cols-[1fr_auto] border-t border-r border-neutral-800">
	<div>
		<label for="medioDePago">Forma de Pago</label>
		<span>{medioSeleccionado}</span>
	</div>
	<div class="border-l border-neutral-800 text-sm">
		<table>
			<tbody>
				<tr class="border-b border-neutral-800">
					<td class="text-right px-2">OP.EXONERADAS:</td>
					<td class="text-right pr-2">S/. 120</td>
				</tr>
				<tr class="border-b border-neutral-800">
					<td class="text-right px-2">I.C.B.P.E.R:</td>
					<td class="text-right pr-2">S/. 0</td>
				</tr>
				<tr class="border-b border-neutral-800">
					<th class="text-right px-2">TOTAL A PAGAR:</th>
					<td class="text-right pr-2">S/. 23.00</td>
				</tr>
				<tr class="border-b border-neutral-800">
					<th class="text-right px-2">TOTAL A PAGADO:</th>
					<td class="text-right pr-2">S/. 330</td>
				</tr>
				<tr class="border-b border-neutral-800">
					<th class="text-right px-2">VUELTO:</th>
					<td class="text-right pr-2">S/. 0.00</td>
				</tr>
			</tbody>
		</table>
	</div>
</div>