<script lang="ts">
	import { IconSearch } from '@tabler/icons-svelte';
  //sitio para pedir los customers de la base de datos y mostrarlos en la tabla, solo # y nombre
</script>

<div class="m-4 grid grid-rows-[40px_auto_1fr] border border-neutral-800">
	<div class="flex items-center justify-center font-extrabold text-white">
		<p>Lista de Clientes</p>
	</div>
	<div class="flex justify-end p-4 border-t border-t-neutral-800">
		<div>
			<div class="flex items-center rounded-lg border border-neutral-800 px-4">
				<IconSearch size={16}/>
				<input type="text" placeholder="Buscar" class="w-full border-0 bg-transparent focus:ring-0" />
			</div>
		</div>
	</div>
  <div class="text-sm border border-neutral-800">
    <table class="bg-neutral-900 w-full">
      <thead class="border-b border-neutral-800">
        <tr>
          <th class="align-top">#</th>
          <th class="align-top">Nombre</th>
          <th class="align-top">Acciones</th>
        </tr>
      </thead>
      <tbody class="text-center border-b border-blue-700">
        <tr>
          <td class="align-top">1</td>
          <td class="align-top">Elisa</td>
          <td class="align-top">
            <button>Editar</button>
            <button>Eliminar</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</div>

