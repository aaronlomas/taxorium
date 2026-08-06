<script lang="ts">
	import { onMount } from 'svelte';
	import { IconEye, IconEyeOff, IconTrash, IconEdit, IconSearch } from '@tabler/icons-svelte';
	import Option from '$lib/components/core/primitives/Option.svelte';
	import { sellersStore, editingSeller, sellerPasswordCache } from '$lib/stores/sellers';
	import { apiClient, type Seller } from '$lib/services/apiClient';

	const MODULOS = [
		{ value: 'sales', label: 'Módulo de Ventas' },
		{ value: 'shop', label: 'Módulo de Compras' },
		{ value: 'products', label: 'Módulo de Productos' }
	];

	const VENDEDORES = [
		{ value: '', label: 'Apellido' },
		{ value: '', label: 'Nombre' },
		{ value: '', label: 'Puesto' }
	];

	let searchQuery = $state('');
	
	onMount(() => {
		sellersStore.load();
	});

	async function deleteSeller(id: number) {
		if (!confirm('¿Estás seguro de eliminar este vendedor?')) return;
		try {
			await apiClient.deleteSeller(id);
			await sellersStore.load();
		} catch (e: any) {
			alert(e?.message ?? 'Error al eliminar el vendedor');
		}
	}

	function editSeller(seller: Seller) {
		editingSeller.set(seller);
	}

	function accessLabel(accesses?: string): string {
		if (!accesses) return '—';
		return accesses
			.split(',')
			.map((a) => MODULOS.find((m) => m.value === a.trim())?.label ?? a.trim())
			.join(', ');
	}

	// Toggles visibility of "••••••••" placeholder per row
	let visibleKeys = $state<Record<number, boolean>>({});
	
	function toggleVisibility(id: number) {
		visibleKeys[id] = !visibleKeys[id];
	}

	let filteredSellers = $derived($sellersStore.filter(s => {
		const searchLower = searchQuery.toLowerCase();
		const fullName = `${s.first_name || ''} ${s.last_name || ''}`.toLowerCase();
		return s.username.toLowerCase().includes(searchLower) || 
			fullName.includes(searchLower) ||
			(s.domain || '').toLowerCase().includes(searchLower);
	}));
</script>

<div class="rounded-md border border-neutral-800 p-2 text-sm">
	<!-- PANEL DE FILTRADO -->
	<div class="flex items-center gap-2">
		<h1 class="text-neutral-400 font-semibold uppercase tracking-wider text-xs">Filtrar</h1>
		<!-- FILTRADO -->
		<div class="w-full grid grid-cols-[auto_1fr] items-center p-1 gap-2">
			<Option options={VENDEDORES} />
			<!-- BUSCADOR -->
			<div class="flex items-center rounded-sm border border-neutral-800 bg-neutral-900 px-3 focus-within:border-blue-500">
				<IconSearch size={16} class="text-neutral-400" />
				<input
					type="text"
					placeholder="Buscar por usuario o dominio..."
					bind:value={searchQuery}
					class="w-full py-1.5 px-2 border-0 text-sm bg-transparent text-neutral-200 outline-none focus:ring-0"
				/>
			</div>
		</div>
	</div>

	{#if $sellersStore.length === 0}
		<div class="p-8 text-center text-neutral-500">
			No hay vendedores registrados todavía.
		</div>
	{:else}
		<div class="mt-2 overflow-x-auto border border-neutral-800 rounded-sm">
			<table class="w-full bg-neutral-900 text-left">
				<thead class="border-b border-neutral-800 bg-blue-900 text-neutral-400">
					<tr>
						<th class="px-2 font-medium">#</th>
						<th class="px-2 font-medium">Nombres y Apellidos</th>
						<th class="px-2 font-medium">Usuario</th>
						<th class="px-2 font-medium">Contraseña</th>
						<th class="px-2 font-medium">Dominio</th>
						<th class="px-2 font-medium">Accesos</th>
						<th class="px-2 font-medium text-center">Estado</th>
						<th class="px-2 font-medium text-center">Acciones</th>
					</tr>
				</thead>
				<tbody class="text-neutral-300">
					{#each filteredSellers as seller, i (seller.id)}
						<tr class="border-b border-neutral-800/50 hover:bg-neutral-800/20 last:border-0 transition-colors">
							<td class="py-2 px-3">{i + 1}</td>
							<td class="py-2 px-3">{seller.first_name || ''} {seller.last_name || ''}</td>
							<td class="py-2 px-3">{seller.username}</td>
							<td class="py-2 px-3">
								<div class="inline-flex items-center gap-2">
									{#if $sellerPasswordCache[seller.id]}
										<span class="font-mono text-neutral-300">
											{visibleKeys[seller.id] ? $sellerPasswordCache[seller.id] : '••••••••'}
										</span>
										<button
											type="button"
											class="text-neutral-500 transition-colors hover:text-neutral-200"
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
							<td class="py-2 px-3">{seller.domain ?? '—'}</td>
							<td class="py-2 px-3 text-xs">{accessLabel(seller.accesses)}</td>
							<td class="py-2 px-3 text-center">
								<span class={seller.is_active ? 'text-emerald-400' : 'text-neutral-500'}>
									{seller.is_active ? 'Activo' : 'Inactivo'}
								</span>
							</td>
							<td class="py-2 px-3 text-center">
								<div class="flex items-center justify-center gap-3">
									<button 
										onclick={() => editSeller(seller)}
										class="text-neutral-500 hover:text-blue-400 transition-colors"
										title="Editar"
									>
										<IconEdit size={16} />
									</button>
									<button 
										onclick={() => deleteSeller(seller.id)}
										class="text-neutral-500 hover:text-red-400 transition-colors"
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
