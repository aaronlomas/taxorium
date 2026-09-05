<script lang="ts">
	import InputData from "$lib/components/core/primitives/InputDate.svelte";
  import Button from "$lib/components/core/primitives/Button.svelte"
</script>

<div class="grid grid-rows-[40px_auto_1fr] border-b border-neutral-800 text-sm">
	<div class="flex items-center justify-center font-extrabold text-white">
		<p>Lista de Resúmenes</p>
	</div>
	<div class="flex justify-end p-4 border-t border-t-neutral-800">
		<div class="flex gap-4 items-end">
      <InputData label="Buscar resúmenes desde la fecha:"/>
      <Button variant="primary">Buscar</Button>
		</div>
	</div>

  <!-- TABLA DE RESUMEN -->

  <div>
    <table class="w-full bg-neutral-900 text-center">

      <!-- cabecera -->

      <thead class="border-b border-neutral-800 bg-blue-900">
        <tr>
          <th class="whitespace-nowrap">#</th>
          <th class="whitespace-nowrap">Fecha de Emision</th>
          <th class="whitespace-nowrap">Fecha de Referencia</th>
          <th class="whitespace-nowrap">Identificador</th>
          <th class="whitespace-nowrap">Estado</th>
          <th class="whitespace-nowrap">Ticket</th>
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
          <td>RC-2026081-1</td>
          <td>Aceptado</td>
          <td>12334235135123</td>
          <td>
            <button>XML</button>
            <button>CDR</button>
          </td>
          <td>
            <button title="Emitir a SUNAT">Emitir</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</div>
