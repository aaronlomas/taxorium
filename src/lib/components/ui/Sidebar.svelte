<script lang="ts">
	import {
		IconLayoutDashboard,
		IconReportMoney,
		IconPackage,
		IconLogout,
		IconSettings,
		IconReport
	} from '@tabler/icons-svelte';
	import { createEventDispatcher } from 'svelte';
	import { auth, currentUser } from '$lib/stores/auth';
	import { currentTenant } from '$lib/stores/tenant';

	const dispatch = createEventDispatcher();

	let anchoBarraLateral = 210;
	let estaRedimensionando = false;

	function iniciarRedimension(e: MouseEvent) {
		e.preventDefault();
		estaRedimensionando = true;
		document.addEventListener('mousemove', redimensionar);
		document.addEventListener('mouseup', detenerRedimension);
		// Prevenir la selección de texto durante el arrastre
		document.body.style.cursor = 'col-resize';
	}

	function redimensionar(e: MouseEvent) {
		if (estaRedimensionando) {
			let nuevoAncho = e.clientX;
			if (nuevoAncho < 180) nuevoAncho = 180;
			if (nuevoAncho > 450) nuevoAncho = 450;
			anchoBarraLateral = nuevoAncho;
		}
	}

	function detenerRedimension() {
		estaRedimensionando = false;
		document.removeEventListener('mousemove', redimensionar);
		document.removeEventListener('mouseup', detenerRedimension);
		document.body.style.cursor = '';
	}

	const elementosNavegacion = [
		{ nombre: 'Dashboard', icono: IconLayoutDashboard, ruta: '/dashboard' },
		{ nombre: 'Ventas', icono: IconReportMoney, ruta: '/sales' },
		{ nombre: 'Productos', icono: IconPackage, ruta: '/productos' },
		{ nombre: 'Reportes', icono: IconReport, ruta: '/reportes' }
	];
</script>

<aside
	class="relative grid h-full grid-rows-[auto_1fr_auto_auto] border-r border-neutral-800 bg-neutral-950 text-neutral-300"
	style="width: {anchoBarraLateral}px;"
>
	<div class="flex items-center gap-3 border-b border-neutral-800 px-2 py-1 text-sm font-bold text-white">
		<span class="tracking-wide">Explorar</span>
	</div>

	<nav class="flex flex-1 flex-col gap-2 overflow-y-auto px-2 py-4">
		{#each elementosNavegacion as elemento}
			<button
				type="button"
				on:click={() => dispatch('seleccionarOpcion', elemento)}
				class="group flex w-full cursor-pointer items-center gap-3 rounded-lg px-3 py-2 transition-colors hover:bg-neutral-800 hover:text-white"
			>
				<svelte:component
					this={elemento.icono}
					size={20}
					stroke={1.5}
					class="text-neutral-400 transition-colors group-hover:text-blue-400"
				/>
				<span class="text-sm font-medium">{elemento.nombre}</span>
			</button>
		{/each}
	</nav>

	<div class="flex flex-col gap-2 border-t border-neutral-800 p-4">
		<div class="flex flex-col gap-1">
			<h2 class="mb-1 text-xs font-bold tracking-widest text-neutral-500 uppercase">Perfil</h2>
			<p class="truncate text-sm font-medium text-neutral-200" title={$currentTenant?.razon_social}>
				{$currentTenant?.razon_social || 'Cargando empresa...'}
			</p>
			<p class="text-xs text-neutral-400">
				RUC: {$currentTenant?.ruc || '...'}
			</p>
			<p class="truncate text-xs text-neutral-500" title={$currentUser?.email}>
				{$currentUser?.email || '...'}
			</p>
		</div>
	</div>
	<div class="border-t border-neutral-800 p-2">
		<button class="flex items-center gap-2 px-3 py-2 text-sm leading-none"
			><IconSettings size={20} /> Configuración</button
		>
		<button
			class="flex w-full cursor-pointer items-center gap-2 rounded-md px-3 py-2 text-sm font-medium text-neutral-400 transition-colors hover:bg-red-950 hover:text-red-400"
			on:click={() => auth.signOut()}
		>
			<IconLogout size={20} stroke={1.5} />
			<span>Cerrar sesión</span>
		</button>
	</div>
	<!-- Control de redimensionamiento -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div
		class="absolute top-0 right-0 z-10 h-full w-1 cursor-col-resize opacity-0 transition-colors hover:bg-blue-500 hover:opacity-100"
		class:bg-blue-500={estaRedimensionando}
		class:opacity-100={estaRedimensionando}
		on:mousedown={iniciarRedimension}
	></div>
</aside>
