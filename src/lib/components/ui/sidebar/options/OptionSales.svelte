<script lang="ts">
  import { createEventDispatcher } from "svelte";

	// Revizar navegation.ts para agregar nuevos elementos u opciones
	import { navegacionVentas } from '../navegation';

  const dispatch = createEventDispatcher();
</script>
<!-- PANEL ESPECÍFICO DE VENTA RÁPIDA -->
<nav class="flex flex-1 flex-col overflow-y-auto">
	<p class="p-2 text-sm text-neutral-400 italic">Venta Rápida</p>
	{#each navegacionVentas as elemento}
		<button
			type="button"
			on:click={() => dispatch('seleccionarOpcion', { nombre: elemento.nombre, ruta: elemento.ruta })}
			class="flex items-center gap-2 rounded p-2 text-sm font-medium transition-colors hover:bg-neutral-800"
		>
			{#if elemento.icono}
				<svelte:component this={elemento.icono} size={18} class="text-blue-400" />
			{/if}
			<span>{elemento.nombre}</span>
		</button>
	{/each}
</nav>
