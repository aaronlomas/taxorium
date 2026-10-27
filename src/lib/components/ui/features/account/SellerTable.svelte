<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { IconEye, IconEyeOff, IconTrash, IconEdit } from '@tabler/icons-svelte';
	import Option from '$lib/components/core/primitives/forms/Select.svelte';
	import Search from '$lib/components/core/primitives/data/Search.svelte';
	import Table, { row, cell, headCell } from '$lib/components/core/primitives/data/Table.svelte';
	import { sellersStore, editingSeller, sellerPasswordCache } from '$lib/features/sellers';
	import { sellerClient, type Seller } from '$lib/features/sellers';
	import { taxoLog } from '$lib/features/taxoLog';

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

	let visibleKeys = $state<Record<number, boolean>>({});
	let loadingKeys = $state<Record<number, boolean>>({});

	async function toggleVisibility(id: number) {
		if (visibleKeys[id]) {
			visibleKeys[id] = false;
			return;
		}

		if ($sellerPasswordCache[id] !== undefined) {
			visibleKeys[id] = true;
			return;
		}

		if (loadingKeys[id]) {
			return;
		}

		loadingKeys[id] = true;
		await tick();
		await new Promise((resolve) => setTimeout(resolve, 0));
		try {
			const password = await sellerClient.getSellerPassword(id);
			sellerPasswordCache.update((cache) => ({ ...cache, [id]: password }));
			visibleKeys[id] = true;
		} catch (e: any) {
			console.error('Error al obtener contraseña del vendedor:', e);
			const msg =
				typeof e === 'string'
					? e
					: e?.message ?? 'No se pudo obtener la contraseña del vendedor';
			alert(msg);
			taxoLog.error(msg, 'vendedores');
		} finally {
			loadingKeys[id] = false;
		}
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

<div class="grid h-full grid-rows-[auto_1fr] gap-2 text-sm">
	<div class="rounded-md border border-neutral-800 p-2 text-sm">
		<div class="flex items-center gap-2">
			<h1 class="text-xs font-semibold tracking-wider text-neutral-400 uppercase">Filtrar</h1>
			<div class="grid w-full grid-cols-[auto_1fr] items-center gap-2 p-1">
				<Option options={VENDEDORES} />
				<Search bind:value={searchQuery} placeholder="Buscar por usuario o dominio..." />
			</div>
		</div>
	</div>

	{#if $sellersStore.length === 0}
		<div class="p-8 text-center text-neutral-500">No hay vendedores registrados todavía.</div>
	{:else}
		<Table>
			{#snippet head()}
				<tr>
					{#snippet headIndex()}#{/snippet}
					{@render headCell({ children: headIndex })}
					{#snippet headName()}Nombres y Apellidos{/snippet}
					{@render headCell({ children: headName })}
					{#snippet headUser()}Usuario{/snippet}
					{@render headCell({ children: headUser })}
					{#snippet headPass()}Contraseña{/snippet}
					{@render headCell({ children: headPass })}
					{#snippet headDomain()}Dominio{/snippet}
					{@render headCell({ children: headDomain })}
					{#snippet headAccess()}Accesos{/snippet}
					{@render headCell({ children: headAccess })}
					{#snippet headStatus()}Estado{/snippet}
					{@render headCell({ children: headStatus })}
					{#snippet headActions()}Acciones{/snippet}
					{@render headCell({ children: headActions })}
				</tr>
			{/snippet}

			{#snippet body()}
				{#each filteredSellers as seller, i (seller.id)}
					{#snippet rowData()}
						{#snippet cellIndex()}{i + 1}{/snippet}
						{@render cell({ class: 'px-2 py-2 text-neutral-500', children: cellIndex })}
						{#snippet cellName()}{seller.nombres || ''} {seller.apellidos || ''}{/snippet}
						{@render cell({ class: 'px-2 py-2', children: cellName })}
						{#snippet cellUser()}{seller.usuario}{/snippet}
						{@render cell({ class: 'px-2 py-2', children: cellUser })}
						{#snippet cellPass()}
							<div class="inline-flex items-center gap-2">
								{#if $sellerPasswordCache[seller.id] !== undefined}
									<span class="font-mono text-neutral-300">
										{visibleKeys[seller.id] ? $sellerPasswordCache[seller.id] : '••••••••'}
									</span>
									<button
										type="button"
										class="mb-0.5 text-neutral-500 transition-colors hover:text-neutral-200"
										onclick={() => toggleVisibility(seller.id)}
										title={visibleKeys[seller.id] ? 'Ocultar' : 'Mostrar'}
										disabled={loadingKeys[seller.id]}
									>
										{#if loadingKeys[seller.id]}
											<span class="inline-block h-3 w-3 animate-spin rounded-full border border-neutral-500 border-t-transparent"></span>
										{:else if visibleKeys[seller.id]}
											<IconEyeOff size={16} />
										{:else}
											<IconEye size={16} />
										{/if}
									</button>
								{:else}
									<button
										type="button"
										class="inline-flex items-center gap-1.5 text-xs text-neutral-400 transition-colors hover:text-neutral-200"
										onclick={() => toggleVisibility(seller.id)}
										disabled={loadingKeys[seller.id]}
										title="Ver contraseña"
									>
										{#if loadingKeys[seller.id]}
											<span class="inline-block h-3 w-3 animate-spin rounded-full border border-neutral-400 border-t-transparent"></span>
											<span>Cargando...</span>
										{:else}
											<IconEye size={16} />
										{/if}
									</button>
								{/if}
							</div>
						{/snippet}
						{@render cell({ class: 'px-2 py-2', children: cellPass })}
						{#snippet cellDomain()}{seller.dominio ?? '—'}{/snippet}
						{@render cell({ class: 'px-2 py-2', children: cellDomain })}
						{#snippet cellAccess()}<span class="text-xs">{accessLabel(seller.accesos)}</span>{/snippet}
						{@render cell({ class: 'px-2 py-2', children: cellAccess })}
						{#snippet cellStatus()}
							<span class={seller.activo ? 'text-emerald-400' : 'text-neutral-500'}>
								{seller.activo ? 'Activo' : 'Inactivo'}
							</span>
						{/snippet}
						{@render cell({ class: 'px-2 py-2 text-center', children: cellStatus })}
						{#snippet cellActions()}
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
						{/snippet}
						{@render cell({ class: 'px-2 py-2 text-center', children: cellActions })}
					{/snippet}
					{@render row({ children: rowData })}
				{/each}

				{#if filteredSellers.length === 0}
					<tr>
						{#snippet emptyState()}
							No se encontraron resultados para "{searchQuery}"
						{/snippet}
						{@render cell({
							colspan: 9,
							class: 'p-4 text-center text-neutral-500',
							children: emptyState
						})}
					</tr>
				{/if}
			{/snippet}
		</Table>
	{/if}
</div>
