<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { IconBook2, IconPlus } from '@tabler/icons-svelte';

	export let seccionActiva: 'vender' | 'registros' | null;

	const dispatch = createEventDispatcher();

	function cambiarSeccion(seccion: 'vender' | 'registros') {
		dispatch('cambiarSeccion', seccion);
	}
</script>

<div class="row-span-3 flex flex-col border-r border-neutral-800 {seccionActiva ? '' : 'border-r-0'}">
	<!-- ACCIÓN DE VENDER -->
	<button
		type="button"
		class="flex cursor-pointer justify-center p-2 hover:bg-neutral-800 {seccionActiva === 'vender'
			? 'bg-neutral-800'
			: ''}"
		title={seccionActiva === 'vender' ? 'Ocultar Venta Rápida' : 'Venta Rápida'}
		aria-label="Toggle Panel Vender"
		on:click={() => cambiarSeccion('vender')}
	>
		<IconPlus
			size={20}
			class="pointer-events-none text-neutral-400 {seccionActiva === 'vender' ? 'text-white' : ''}"
		/>
	</button>

	<!-- REGISTROS GENERALES -->
	<button
		type="button"
		class="flex cursor-pointer justify-center p-2 hover:bg-neutral-800 {seccionActiva === 'registros'
			? 'bg-neutral-800'
			: ''}"
		title={seccionActiva === 'registros' ? 'Ocultar Registros' : 'Mostrar Registros'}
		aria-label="Toggle Panel Registros"
		on:click={() => cambiarSeccion('registros')}
	>
		<IconBook2
			size={20}
			class="pointer-events-none text-neutral-400 {seccionActiva === 'registros'
				? 'text-white'
				: ''}"
		/>
	</button>
</div>
