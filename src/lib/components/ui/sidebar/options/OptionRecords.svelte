<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { IconChevronDown, IconChevronRight } from '@tabler/icons-svelte';

	// Revizar navegation.ts para agregar nuevos elementos u opciones
	import { navegacionRegistros } from '../navegation';

	const dispatch = createEventDispatcher();

	let menusAbiertos: Record<string, boolean> = {};

	function toggleMenu(nombre: string) {
		menusAbiertos[nombre] = !menusAbiertos[nombre];
	}
</script>

<nav class="flex flex-1 flex-col overflow-y-auto">
	<p class="p-2 text-sm text-neutral-400 italic">Registros</p>
	{#each navegacionRegistros as elemento}
		{#if elemento.subitems}
			<button
				type="button"
				aria-expanded={Boolean(menusAbiertos[elemento.nombre])}
				on:click={() => toggleMenu(elemento.nombre)}
				class="group flex w-full items-center justify-between p-2 transition-colors hover:bg-neutral-800 hover:text-white"
			>
				<div class="flex items-center gap-2">
					<svelte:component
						this={elemento.icono}
						size={20}
						stroke={1.5}
						class="text-neutral-400 transition-colors group-hover:text-blue-400"
					/>
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
							on:click={() => dispatch('seleccionarOpcion', subitem)}
							class="block w-full text-left truncate px-2 py-1 text-sm font-medium text-neutral-400 transition-colors hover:bg-neutral-800 hover:text-white"
						>
							{subitem.nombre}
						</button>
					{/each}
				</div>
			{/if}
		{:else}
			<button
				type="button"
				on:click={() => dispatch('seleccionarOpcion', elemento)}
				class="group flex w-full items-center gap-2 p-2 transition-colors hover:bg-neutral-800 hover:text-white"
			>
				<svelte:component
					this={elemento.icono}
					size={20}
					stroke={1.5}
					class="text-neutral-400 transition-colors group-hover:text-blue-400"
				/>
				<span class="text-sm font-medium">{elemento.nombre}</span>
			</button>
		{/if}
	{/each}
</nav>
