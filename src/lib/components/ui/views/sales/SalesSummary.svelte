<script lang="ts">

	const medioDePagoContado = [
		{ value: '008', label: 'Efectivo' },
		{ value: '006', label: 'Tarjeta Visa (Crédito)' },
		{ value: '005', label: 'Tarjeta Visa (Débito)' },
		{ value: '003', label: 'Transferencia de Fondos (Yape/Plin)' } // (Yape/Plin o cuentas)
	];
  let medioSeleccionado = '008'

  //Si se selecciona tipo de pago en crédito
	const condicionesDePago = [
		{ value: 'Contado', label: 'Pago al Contado' },
		{ value: 'Credito', label: 'Pago al Crédito' }
	];
  let condicionSeleccionada = 'Contado'
</script>

<!-- <div class="grid h-full grid-cols-2 border border-neutral-800">
  <div>
    <label for="medioDePago">Forma de Pago</label>
    <span>{medioSeleccionado}</span>
  </div>
	<div>
		<p class="text-right">OP.EXONERADAS:</p>
		<p class="text-right">I.C.B.P.E.R:</p>
		<p class="text-right"><strong>TOTAL A PAGAR:</strong></p>
		<p class="text-right"><strong>TOTAL A PAGADO:</strong></p>
		<p class="text-right"><strong>VUELTO:</strong></p>
	</div>
</div> -->
