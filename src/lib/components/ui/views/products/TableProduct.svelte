<script lang="ts">
	import { IconSearch } from '@tabler/icons-svelte';
</script>

<div class="m-4 grid grid-rows-[40px_auto_1fr] border border-neutral-800">
	<div class="flex items-center justify-center font-extrabold text-white">
		<p>Lista de Productos</p>
	</div>
	<div class="flex justify-end p-4 border-t border-t-neutral-800">
		<div>
			<div class="flex items-center rounded-lg border border-neutral-800 px-4">
				<IconSearch size={16}/>
				<input type="text" placeholder="Buscar" class="w-full border-0 bg-transparent focus:ring-0" />
			</div>
		</div>
	</div>
	<table class="bg-neutral-900">
		<thead>
			<tr>
				<th class="align-top">Seleccionar</th>
				<th class="align-top">Producto</th>
				<th class="align-top">Precio</th>
				<th class="align-top">Stock</th>
				<th class="align-top">Acciones</th>
			</tr>
		</thead>
		<tbody>
			<tr>
				<td class="align-top"><input type="checkbox" /></td>
				<td class="align-top">Producto 1</td>
				<td class="align-top">10</td>
				<td class="align-top">10</td>
				<td class="align-top">
					<button>Editar</button>
					<button>Eliminar</button>
				</td>
			</tr>
		</tbody>
	</table>
</div>
