<script lang="ts">
	import Sidebar from '$lib/components/ui/navigation/sidebar/Sidebar.svelte';
	import type { ElementoNavegacion } from '$lib/components/ui/navigation/sidebar/navigation';
	import TabBar from '$lib/components/ui/navigation/tab-bar/TabBar.svelte';
	import Tab from '$lib/components/ui/navigation/tab-bar/tab/Tab.svelte';
	import { resolveView } from '$lib/components/ui/navigation/viewRoutes';
	import { auth, currentUser } from '$lib/features/auth';
	import { tenantStore, currentTenant } from '$lib/features/tenant';
	import { sellerAuth } from '$lib/features/sellerAuth';
	import { configStore } from '$lib/integrations/tauri/nodeConfig';
	import PosLogin from '$lib/components/ui/features/auth/setups/PosLogin.svelte';
	import Log from '$lib/components/log/TaxoLog.svelte';

	$effect(() => {
		if ($currentUser) {
			// Solo cargamos si no tenemos los datos en memoria para este usuario
			if (!$currentTenant || $currentTenant.user_id !== $currentUser.id) {
				tenantStore.load();
			}
		} else {
			tenantStore.clear();
		}
	});

	// Sin pestañas iniciales: el usuario elige qué módulo abrir desde el sidebar.
	let pestanas = $state<
		{ id: string; titulo: string; icono: any; colorIcono: string; activo: boolean }[]
	>([]);

	function seleccionarPestana(id: string) {
		pestanas = pestanas.map((p) => ({ ...p, activo: p.id === id }));
	}

	function cerrarPestana(id: string) {
		pestanas = pestanas.filter((p) => p.id !== id);
		// Si cerramos el activo, activar la última pestaña disponible
		if (pestanas.length > 0 && !pestanas.some((p) => p.activo)) {
			pestanas[pestanas.length - 1].activo = true;
		}
	}

	function manejarSeleccionSidebar(elemento: ElementoNavegacion) {
		const existe = pestanas.find((p) => p.id === elemento.nombre);

		if (existe) {
			seleccionarPestana(elemento.nombre);
		} else {
			// Agregamos la nueva pestaña y la activamos
			const nuevaPestana = {
				id: elemento.nombre,
				titulo: elemento.nombre,
				icono: elemento.icono,
				colorIcono: 'text-blue-500', // Color predeterminado
				activo: true
			};

			// Desactivamos todas las demás y agregamos la nueva
			pestanas = [...pestanas.map((p) => ({ ...p, activo: false })), nuevaPestana];
		}
	}

	// Obtenemos la pestaña activa para renderizar el contenido correcto
	let pestanaActiva = $derived(pestanas.find((p) => p.activo));

	// El componente resuelto para la pestaña activa (null = no registrada)
	let VistaActiva = $derived(pestanaActiva ? resolveView(pestanaActiva.id) : null);

	// Show PosLogin ONLY on client (caja) nodes when no seller is logged in.
	// Server nodes go directly to the full app (admin already authenticated via Supabase).
	let showPosLogin = $derived($configStore.role === 'client' && !$sellerAuth);
</script>

{#if showPosLogin}
	<PosLogin />
{:else}
	<div
		class="grid h-full w-full grid-cols-[auto_1fr] grid-rows-[1fr_24px] overflow-hidden bg-neutral-950 font-sans text-neutral-300"
	>
		<Sidebar onSeleccionarOpcion={manejarSeleccionSidebar} />

		<div class="flex min-h-0 min-w-0 flex-col bg-neutral-950">
			<TabBar>
				{#each pestanas as pestana (pestana.id)}
					<Tab
						title={pestana.titulo}
						icon={pestana.icono}
						iconColor={pestana.colorIcono}
						active={pestana.activo}
						onClick={() => seleccionarPestana(pestana.id)}
						onClose={() => cerrarPestana(pestana.id)}
					/>
				{/each}
			</TabBar>

			<!-- El área de contenido renderiza dinámicamente según el registro de vistas -->
			<div class="min-h-0 flex-1 overflow-hidden">
				{#if pestanas.length === 0}
					<div class="flex h-full items-center justify-center text-sm text-neutral-500">
						Selecciona una opción para comenzar
					</div>
				{:else if VistaActiva}
					<VistaActiva />
				{:else}
					{@const Icon = pestanaActiva?.icono}
					<div class="flex h-full flex-col items-center justify-center gap-3 text-neutral-500">
						{#if Icon}
							<Icon size={48} stroke={1.5} class="text-neutral-700" />
						{/if}
						<p class="text-sm">
							El módulo <span class="font-medium text-neutral-400">{pestanaActiva?.titulo}</span> está
							en desarrollo.
						</p>
					</div>
				{/if}
			</div>
		</div>
		<Log />
	</div>
{/if}
