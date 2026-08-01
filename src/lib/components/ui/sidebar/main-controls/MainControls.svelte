<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { IconBook2, IconCalculator } from '@tabler/icons-svelte';

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
		class="flex cursor-pointer justify-center p-2 text-neutral-400 hover:text-white {seccionActiva === 'vender'
			? 'border-l-2 border-l-blue-500 bg-blue-500/15'
			: ''}"
		title={seccionActiva === 'vender' ? 'Venta Rápida' : 'Venta Rápida'}
		aria-label="Toggle Panel Vender"
		on:click={() => cambiarSeccion('vender')}
	>
		<IconCalculator
			size={28}
			class="pointer-events-none {seccionActiva === 'vender' ? 'text-white' : ''}"
		/>
	</button>

	<!-- REGISTROS GENERALES -->
	<button 
		type="button"
		class="flex cursor-pointer justify-center p-2 text-neutral-400 hover:text-white {seccionActiva === 'registros'
			? 'border-l-2 border-l-blue-500 bg-blue-500/15'
			: ''}"
		title={seccionActiva === 'registros' ? 'Registros' : 'Registros'}
		aria-label="Toggle Panel Registros"
		on:click={() => cambiarSeccion('registros')}
	>
		<IconBook2
			size={28}
			class="pointer-events-none {seccionActiva === 'registros'
				? 'text-white'
				: ''}"
		/>
	</button>
</div>
