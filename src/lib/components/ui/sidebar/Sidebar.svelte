<script lang="ts">
	import { onDestroy } from 'svelte';
	import { createEventDispatcher } from 'svelte';
	import { IconLogout, IconSettings, IconReceipt, IconFileInvoice } from '@tabler/icons-svelte';
	import MainControls from './main-controls/MainControls.svelte';
	import ViewControls from './view-controls/ViewControls.svelte';
	import { auth, currentUser } from '$lib/stores/auth';
	import { currentTenant } from '$lib/stores/tenant';

	const dispatch = createEventDispatcher();

	let anchoBarraLateral = 220;
	let estaRedimensionando = false;

	// Manejo de pestaña activa: 'vender' | 'registros' | null (colapsado)
	let seccionActiva: 'vender' | 'registros' | null = 'vender';

	function cambiarSeccion(seccion: 'vender' | 'registros') {
		if (seccionActiva === seccion) {
			seccionActiva = null; // Si hace clic en la activa, se colapsa
		} else {
			seccionActiva = seccion; // Si hace clic en otra, cambia de panel
		}
	}

	function redimensionar(e: MouseEvent) {
		if (!estaRedimensionando) return;

		let nuevoAncho = e.clientX;

		// Auto-colapsar si se arrastra por debajo de 100px
		if (nuevoAncho < 100) {
			seccionActiva = null;
			anchoBarraLateral = 220;
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

	function handleCambiarSeccion(e: CustomEvent<'vender' | 'registros'>) {
		cambiarSeccion(e.detail);
	}
</script>

<aside
	class="relative grid h-full grid-rows-[1fr_auto_auto] border-r border-neutral-800 bg-neutral-950 text-neutral-300 select-none {seccionActiva
		? 'grid-cols-[auto_1fr]'
		: 'grid-cols-[auto]'}"
	style={seccionActiva ? `width: ${anchoBarraLateral}px;` : 'width: max-content;'}
>
	<!-- BARRA DE ACCIONES IZQUIERDA (FIJA) -->
	<MainControls {seccionActiva} on:cambiarSeccion={handleCambiarSeccion} />

	<!-- PANEL DESPLEGABLE DINÁMICO -->
	{#if seccionActiva === 'vender'}
		<!-- PANEL ESPECÍFICO DE VENTA RÁPIDA -->
		<nav class="flex flex-1 flex-col overflow-y-auto">
			<p class="border-b border-neutral-800 p-2 text-sm text-neutral-400 italic">Venta Rápida</p>
			<button
				type="button"
				on:click={() =>
					dispatch('seleccionarOpcion', { nombre: 'Nueva Factura', ruta: '/vender/factura' })}
				class="flex items-center gap-2 rounded p-2 text-sm font-medium transition-colors hover:bg-neutral-800"
			>
				<IconFileInvoice size={18} class="text-blue-400" />
				<span>Nueva Factura</span>
			</button>
			<button
				type="button"
				on:click={() =>
					dispatch('seleccionarOpcion', { nombre: 'Nueva Boleta', ruta: '/vender/boleta' })}
				class="flex items-center gap-2 rounded p-2 text-sm font-medium transition-colors hover:bg-neutral-800"
			>
				<IconReceipt size={18} class="text-green-400" />
				<span>Nueva Boleta</span>
			</button>
		</nav>
	{:else if seccionActiva === 'registros'}
		<!-- PANEL GENERAL DE REGISTROS/MÓDULOS -->
		<ViewControls on:seleccionarOpcion={(e) => dispatch('seleccionarOpcion', e.detail)} />
	{/if}

	{#if seccionActiva}
		<!-- SECCIÓN DE PERFIL -->
		<div class="flex flex-col gap-2 border-t border-neutral-800 p-2">
			<div class="flex min-w-0 flex-col gap-1">
				<h2 class="mb-1 text-xs font-bold tracking-widest text-neutral-500 uppercase">Perfil</h2>
				<p
					class="truncate text-sm font-medium text-neutral-200"
					title={$currentTenant?.razon_social}
				>
					{$currentTenant?.razon_social ?? 'Cargando empresa...'}
				</p>
				<p class="truncate text-xs text-neutral-400">
					RUC: {$currentTenant?.ruc ?? '...'}
				</p>
				<p class="truncate text-xs text-neutral-500" title={$currentUser?.email}>
					{$currentUser?.email ?? '...'}
				</p>
			</div>
		</div>

		<!-- BOTONES DE CONTROL/ACCIONES -->
		<div class="flex flex-col border-t border-neutral-800 py-2">
			<button
				type="button"
				class="flex w-full cursor-pointer items-center gap-2 p-2 text-sm text-neutral-400 transition-colors hover:bg-neutral-800 hover:text-white"
				on:click={() =>
					dispatch('seleccionarOpcion', { nombre: 'Configuración', ruta: '/settings' })}
			>
				<IconSettings size={20} stroke={1.5} />
				<span>Configuración</span>
			</button>

			<button
				type="button"
				class="flex w-full cursor-pointer items-center gap-2 p-2 text-sm font-medium text-neutral-400 transition-colors hover:bg-red-950 hover:text-red-400"
				on:click={() => auth.signOut()}
			>
				<IconLogout size={20} stroke={1.5} />
				<span>Cerrar sesión</span>
			</button>
		</div>

		<!-- CONTROL DE REDIMENSIONAMIENTO -->
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			role="separator"
			aria-orientation="vertical"
			class="absolute top-0 right-0 z-10 h-full w-1 cursor-ew-resize opacity-0 transition-colors hover:bg-blue-500 hover:opacity-100"
			class:bg-blue-500={estaRedimensionando}
			class:opacity-100={estaRedimensionando}
			on:mousedown={iniciarRedimension}
		></div>
	{/if}
</aside>
