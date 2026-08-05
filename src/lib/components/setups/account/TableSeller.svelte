<script>
	import { IconEye, IconEyeOff, IconTrash, IconEdit, IconSearch } from '@tabler/icons-svelte';
	import Option from '$lib/components/core/primitives/Option.svelte';

	let mostrarClave = false;
	const clave = '12d23d11';

  const VENDEDORES = [
    {value: '', label: 'Apellido'},
    {value: '', label: 'Nombre'},
    {value: '', label: 'Puesto'}
  ]
</script>

<div class="rounded-md border border-neutral-800 p-2 text-sm">

  <!-- PANEL DE FILTRADO -->
	<div class="flex items-center gap-2">
		<h1>Filtrar</h1>
      <!-- FILTRADO -->
		<div class="w-full grid grid-cols-[auto_1fr] items-center p-1 gap-2">
			<Option options={VENDEDORES}/>
      <!-- BUSCADOR -->
			<div class="flex items-center rounded-lg border border-neutral-800 bg-neutral-900 px-4">
				<IconSearch size={16} />
				<input
					type="text"
					placeholder="Buscar"
					class="w-full border-0 text-sm bg-transparent focus:ring-0"
				/>
			</div>
		</div>
	</div>
	<table class="w-full bg-neutral-900">
		<thead class="border-b border-neutral-800 bg-blue-900">
			<tr>
				<th>#</th>
				<th>Nombre</th>
				<th>Usuario</th>
				<th>Contraseña</th>
				<th>Dominio</th>
				<th>Accesos</th>
				<th>Estado</th>
				<th>Acciones</th>
			</tr>
		</thead>
		<tbody class="border-b border-blue-700 text-center">
			<tr>
				<td>1</td>
				<td>Elisa</td>
				<td>elisa@gmail.com</td>
				<td>
					<div class="inline-flex items-center gap-1.5">
						<span class="font-mono">
							{mostrarClave ? clave : '••••••••'}
						</span>
						<button
							type="button"
							class="grid items-center pb-0.5 text-neutral-400 transition-colors hover:text-white"
							on:click={() => (mostrarClave = !mostrarClave)}
							title="Mostrar/Ocultar"
						>
							{#if mostrarClave}
								<IconEyeOff size={18} />
							{:else}
								<IconEye size={18} />
							{/if}
						</button>
					</div>
				</td>
				<td>CAJA-1</td>
				<td>ventas, compras</td>
				<td>Activo</td>
				<td>
					<div class="flex items-center justify-center gap-2">
						<button>
							<IconEdit size={16} />
						</button>
						<button>
							<IconTrash size={16} />
						</button>
					</div>
				</td>
			</tr>
		</tbody>
	</table>
</div>
