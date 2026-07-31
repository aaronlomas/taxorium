<script lang="ts">
	import { IconSearch } from '@tabler/icons-svelte';
</script>

<div class="m-4 grid grid-rows-[40px_auto_1fr] border border-neutral-800 text-sm">
	<div class="flex items-center justify-center font-extrabold text-white">
		<p>Lista de Comprobantes</p>
	</div>
	<div class="flex justify-end p-4 border-t border-t-neutral-800">
		<div>
			<div class="flex items-center rounded-lg border border-neutral-800 px-4">
				<IconSearch size={16}/>
				<input type="text" placeholder="Buscar" class="w-full border-0 bg-transparent focus:ring-0" />
			</div>
		</div>
	</div>

  <!-- TABLA DE COMPROBANTES -->

  <div>
    <table class="w-full bg-neutral-900 text-center">

      <!-- cabecera -->

      <thead class="border-b border-neutral-800 bg-blue-900">
        <tr>
          <th class="whitespace-nowrap">#</th>
          <th class="whitespace-nowrap">Fecha de Emision</th>
          <th class="whitespace-nowrap">Cliente</th>
          <th class="whitespace-nowrap">Número</th>
          <th class="whitespace-nowrap">Estado</th>
          <th class="whitespace-nowrap">Estado de Pago</th>
          <th class="whitespace-nowrap">Moneda</th>
          <th class="whitespace-nowrap">Gravado</th>
          <th class="whitespace-nowrap">Igv</th>
          <th class="whitespace-nowrap">Total</th>
          <th class="whitespace-nowrap">Descargar</th>
          <th class="whitespace-nowrap">Acciones</th>
        </tr>
      </thead>

      <!-- contenido -->

      <tbody class="border-b border-blue-700">
        <tr>
          <td>1</td>
          <td>NIU</td>
          <td>Zapatos de cuero</td>
          <td>F001-1 Factura Electrónica</td>
          <td>s/. 200.00</td>
          <td>s/. 200.00</td>
          <td>s/. 200.00</td>
          <td>s/. 200.00</td>
          <td>s/. 200.00</td>
          <td>s/. 200.00</td>
          <td>s/. 200.00</td>
          <td>
            <button>Editar</button>
            <button>Eliminar</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</div>
