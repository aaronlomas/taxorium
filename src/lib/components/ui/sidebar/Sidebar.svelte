<script lang="ts">
	import { onDestroy, createEventDispatcher } from 'svelte';
	import { isAuthenticated } from '$lib/stores/auth';
	import MainControls from './main-controls/MainControls.svelte';
	import OptionRecords from './options/OptionRecords.svelte';
	import OptionSales from './options/OptionSales.svelte';

	type Seccion = 'operaciones' | 'registros';

	// Configuración centralizada de paneles
	const PANELS = {
		operaciones: OptionSales,
		registros: OptionRecords
	};

	const dispatch = createEventDispatcher();

	let anchoBarraLateral = $state(240);
	let estaRedimensionando = $state(false);
	let seccionActiva: Seccion | null = $state($isAuthenticated ? 'operaciones' : null);

	function cambiarSeccion(seccion: Seccion) {
		seccionActiva = seccionActiva === seccion ? null : seccion;
	}

	// Mantener el sidebar encogido mientras no haya una sesión iniciada
	$effect(() => {
		if (!$isAuthenticated) {
			seccionActiva = null;
		}
	});

	function handleSeleccionarOpcion(e: CustomEvent) {
		dispatch('seleccionarOpcion', e.detail);
	}

	function redimensionar(e: MouseEvent) {
		if (!estaRedimensionando) return;

		let nuevoAncho = e.clientX;

		if (nuevoAncho < 100) {
			seccionActiva = null;
			anchoBarraLateral = 240;
			detenerRedimension();
			return;
		}

		anchoBarraLateral = Math.min(Math.max(nuevoAncho, 200), 450);
	}

	function detenerRedimension() {
		if (!estaRedimensionando) return;
		estaRedimensionando = false;

		document.removeEventListener('mousemove', redimensionar);
		document.removeEventListener('mouseup', detenerRedimension);
		document.body.style.cursor = '';
		document.body.style.userSelect = '';
	}

	function iniciarRedimension(e: MouseEvent) {
		e.preventDefault();
		estaRedimensionando = true;

		document.addEventListener('mousemove', redimensionar);
		document.addEventListener('mouseup', detenerRedimension);

		document.body.style.cursor = 'ew-resize';
		document.body.style.userSelect = 'none';
	}

	onDestroy(() => {
		detenerRedimension();
	});
</script>

<aside
	class="relative grid h-full grid-rows-[1fr_auto_auto] border-r border-neutral-800 bg-neutral-950 text-neutral-300 select-none {seccionActiva
		? 'grid-cols-[auto_1fr]'
		: 'grid-cols-[auto]'}"
	style={seccionActiva ? `width: ${anchoBarraLateral}px;` : 'width: max-content;'}
>
	<!-- BARRA DE ACCIONES IZQUIERDA (FIJA) -->
	<MainControls
		{seccionActiva}
		on:cambiarSeccion={(e) => cambiarSeccion(e.detail)}
		on:seleccionarOpcion={handleSeleccionarOpcion}
	/>

	<!-- PANEL DESPLEGABLE DINÁMICO -->
	{#if seccionActiva}
		{@const Panel = PANELS[seccionActiva]}
		<Panel on:seleccionarOpcion={handleSeleccionarOpcion} />
	{/if}

	{#if seccionActiva}
		<!-- CONTROL DE REDIMENSIONAMIENTO -->
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			role="separator"
			aria-orientation="vertical"
			class="absolute top-0 right-0 z-10 h-full w-1 cursor-ew-resize opacity-0 transition-colors hover:bg-blue-500 hover:opacity-100"
			class:bg-blue-500={estaRedimensionando}
			class:opacity-100={estaRedimensionando}
			onmousedown={iniciarRedimension}
		></div>
	{/if}
</aside>
