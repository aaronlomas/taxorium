<script lang="ts">
	import { IconChevronDown, IconChevronRight } from '@tabler/icons-svelte';

	// Revizar navegation.ts para agregar nuevos elementos u opciones
	import {
		navegacionOperaciones,
		navegacionRegistros,
		type ElementoNavegacion
	} from '../navegation';
	import { sellerAuth } from '$lib/features/sellerAuth';

	let { onSeleccionarOpcion }: { onSeleccionarOpcion?: (opcion: ElementoNavegacion) => void } =
		$props();

	let menusAbiertos = $state<Record<string, boolean>>({});

	function toggleMenu(nombre: string) {
		menusAbiertos[nombre] = !menusAbiertos[nombre];
	}

	function filtrar(elementos: ElementoNavegacion[]) {
		return elementos.filter((el) => {
			if (!$sellerAuth) return true; // Dueño (Admin) ve todo
			if (!el.accesoRequerido) return true; // Elementos públicos (ej. Dashboard)
			// Si tiene acceso específico, verificar que coincida o si tiene acceso a todo (si existiera "all")
			return $sellerAuth.accesos === el.accesoRequerido;
		});
	}

	let operacionesFiltradas = $derived(filtrar(navegacionOperaciones));
	let registrosFiltrados = $derived(filtrar(navegacionRegistros));
</script>

{#snippet elementoNav(elemento: ElementoNavegacion)}
	{#if elemento.subitems}
		<button
			type="button"
			aria-expanded={Boolean(menusAbiertos[elemento.nombre])}
			onclick={() => toggleMenu(elemento.nombre)}
			class="group flex w-full items-center justify-between p-2 transition-colors hover:bg-neutral-800 hover:text-white"
		>
			<div class="flex items-center gap-2">
				{#if elemento.icono}
					<elemento.icono
						size={20}
						stroke={1.5}
						class="text-neutral-400 transition-colors group-hover:text-blue-400"
					/>
				{/if}
				<span class="text-sm font-medium">{elemento.nombre}</span>
			</div>
			{#if menusAbiertos[elemento.nombre]}
				<IconChevronDown size={16} class="text-neutral-500" />
			{:else}
				<IconChevronRight size={16} class="text-neutral-500" />
			{/if}
		</button>

		{#if menusAbiertos[elemento.nombre]}
			<div class="ml-4 flex flex-col border-l border-neutral-800">
				{#each elemento.subitems as subitem}
					<button
						type="button"
						onclick={() => onSeleccionarOpcion?.(subitem)}
						class="block w-full truncate px-2 py-1 text-left text-sm font-medium text-neutral-400 transition-colors hover:bg-neutral-800 hover:text-white"
					>
						{subitem.nombre}
					</button>
				{/each}
			</div>
		{/if}
	{:else}
		<button
			type="button"
			onclick={() => onSeleccionarOpcion?.(elemento)}
			class="group flex w-full items-center gap-2 p-2 transition-colors hover:bg-neutral-800 hover:text-white"
		>
			{#if elemento.icono}
				<elemento.icono
					size={20}
					stroke={1.5}
					class="text-neutral-400 transition-colors group-hover:text-blue-400"
				/>
			{/if}
			<span class="text-sm font-medium">{elemento.nombre}</span>
		</button>
	{/if}
{/snippet}

<nav class="flex flex-1 flex-col overflow-y-auto">
	<p class="p-2 text-sm text-neutral-400 italic">Operaciones</p>
	{#each operacionesFiltradas as elemento}
		{@render elementoNav(elemento)}
	{/each}

	<p class="p-2 text-sm text-neutral-400 italic">Registros</p>
	{#each registrosFiltrados as elemento}
		{@render elementoNav(elemento)}
	{/each}
</nav>
