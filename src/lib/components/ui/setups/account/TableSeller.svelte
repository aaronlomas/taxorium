<script lang="ts">
	import { onMount } from 'svelte';
	import { IconEye, IconEyeOff, IconTrash, IconEdit, IconSearch } from '@tabler/icons-svelte';
	import Option from '$lib/components/core/primitives/Select.svelte';
	import { sellersStore, editingSeller, sellerPasswordCache } from '$lib/stores/sellers';
	import { sellerClient, type Seller } from '$lib/services/sellers/clientSellers';

	const MODULOS = [
		{ value: 'sales', label: 'Módulo de Ventas' },
		{ value: 'shop', label: 'Módulo de Compras' },
		{ value: 'products', label: 'Módulo de Productos' }
	];

	const VENDEDORES = [
		{ value: 'apellido', label: 'Apellido' },
		{ value: 'nombre', label: 'Nombre' },
		{ value: 'puesto', label: 'Puesto' }
	];

	let searchQuery = $state('');

	onMount(() => {
		sellersStore.load();
	});

	async function deleteSeller(id: number) {
		if (!confirm('¿Estás seguro de eliminar este vendedor?')) return;
		try {
			await sellerClient.deleteSeller(id);
			await sellersStore.load();
		} catch (e: any) {
			alert(e?.message ?? 'Error al eliminar el vendedor');
		}
	}

	function editSeller(seller: Seller) {
		editingSeller.set(seller);
	}

	function accessLabel(accesos?: string): string {
		if (!accesos) return '—';
		return accesos
			.split(',')
			.map((a) => MODULOS.find((m) => m.value === a.trim())?.label ?? a.trim())
			.join(', ');
	}

	// Toggles visibility of "••••••••" placeholder per row
	let visibleKeys = $state<Record<number, boolean>>({});

	function toggleVisibility(id: number) {
		visibleKeys[id] = !visibleKeys[id];
	}

	let filteredSellers = $derived(
		$sellersStore.filter((s) => {
			const searchLower = searchQuery.toLowerCase();
			const fullName = `${s.nombres || ''} ${s.apellidos || ''}`.toLowerCase();
			return (
				s.usuario.toLowerCase().includes(searchLower) ||
				fullName.includes(searchLower) ||
				(s.dominio || '').toLowerCase().includes(searchLower)
			);
		})
	);
</script>

<div class="rounded-md border border-neutral-800 p-2 text-sm">
	<!-- PANEL DE FILTRADO -->
	<div class="flex items-center gap-2">
		<h1 class="text-xs font-semibold tracking-wider text-neutral-400 uppercase">Filtrar</h1>
		<!-- FILTRADO -->
		<div class="grid w-full grid-cols-[auto_1fr] items-center gap-2 p-1">
			<Option options={VENDEDORES} />
			<!-- BUSCADOR -->
			<div
				class="flex items-center rounded-sm border border-neutral-800 bg-neutral-900 px-3 focus-within:border-blue-500"
			>
				<IconSearch size={16} class="text-neutral-400" />
				<input
					type="text"
					placeholder="Buscar por usuario o dominio..."
					bind:value={searchQuery}
					class="w-full border-0 bg-transparent px-2 text-sm text-neutral-200 outline-none focus:ring-0"
				/>
			</div>
		</div>
	</div>

	{#if $sellersStore.length === 0}
		<div class="p-8 text-center text-neutral-500">No hay vendedores registrados todavía.</div>
	{:else}
		<div class="mt-2 overflow-x-auto rounded-sm border border-neutral-800">
			<table class="w-full bg-neutral-900 text-left">
				<thead class="border-b border-neutral-800 text-neutral-400">
					<tr class="text-blue-400">
						<th class="px-2 font-medium">#</th>
						<th class="px-2 font-medium">Nombres y Apellidos</th>
						<th class="px-2 font-medium">Usuario</th>
						<th class="px-2 font-medium">Contraseña</th>
						<th class="px-2 font-medium">Dominio</th>
						<th class="px-2 font-medium">Accesos</th>
						<th class="px-2 text-center font-medium">Estado</th>
						<th class="px-2 text-center font-medium">Acciones</th>
					</tr>
				</thead>
				<tbody class="text-neutral-300">
					{#each filteredSellers as seller, i (seller.id)}
						<tr
							class="border-b border-neutral-800/50 transition-colors last:border-0 hover:bg-neutral-800/20"
						>
							<td class="p-2">{i + 1}</td>
							<td class="p-2">{seller.nombres || ''} {seller.apellidos || ''}</td>
							<td class="p-2">{seller.usuario}</td>
							<td class="p-2">
								<div class="inline-flex items-center gap-2">
									{#if $sellerPasswordCache[seller.id]}
										<span class="font-mono text-neutral-300">
											{visibleKeys[seller.id] ? $sellerPasswordCache[seller.id] : '••••••••'}
										</span>
										<button
											type="button"
											class="mb-0.5 text-neutral-500 transition-colors hover:text-neutral-200"
											onclick={() => toggleVisibility(seller.id)}
											title="Mostrar/Ocultar"
										>
											{#if visibleKeys[seller.id]}
												<IconEyeOff size={16} />
											{:else}
												<IconEye size={16} />
											{/if}
										</button>
									{:else}
										<span class="text-xs text-neutral-600 italic">No disponible</span>
									{/if}
								</div>
							</td>
							<td class="px-3 py-2">{seller.dominio ?? '—'}</td>
							<td class="px-3 py-2 text-xs">{accessLabel(seller.accesos)}</td>
							<td class="px-3 py-2 text-center">
								<span class={seller.activo ? 'text-emerald-400' : 'text-neutral-500'}>
									{seller.activo ? 'Activo' : 'Inactivo'}
								</span>
							</td>
							<td class="px-3 py-2 text-center">
								<div class="flex items-center justify-center gap-3">
									<button
										onclick={() => editSeller(seller)}
										class="text-neutral-500 transition-colors hover:text-blue-400"
										title="Editar"
									>
										<IconEdit size={16} />
									</button>
									<button
										onclick={() => deleteSeller(seller.id)}
										class="text-neutral-500 transition-colors hover:text-red-400"
										title="Eliminar"
									>
										<IconTrash size={16} />
									</button>
								</div>
							</td>
						</tr>
					{/each}

					{#if filteredSellers.length === 0}
						<tr>
							<td colspan="8" class="p-4 text-center text-neutral-500">
								No se encontraron resultados para "{searchQuery}"
							</td>
						</tr>
					{/if}
				</tbody>
			</table>
		</div>
	{/if}
</div>
