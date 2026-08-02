<script lang="ts">
  import { onDestroy, createEventDispatcher, type ComponentType, type Component } from 'svelte';
  import { IconLogout, IconSettings } from '@tabler/icons-svelte';
  import MainControls from './main-controls/MainControls.svelte';
  import OptionRecords from './options/OptionRecords.svelte';
  import OptionSales from './options/OptionSales.svelte';
  import { auth, currentUser } from '$lib/stores/auth';
  import { currentTenant } from '$lib/stores/tenant';

  type Seccion = 'operaciones' | 'registros';

  // Configuración centralizada de paneles
  const PANELS = {
    operaciones: OptionSales,
    registros: OptionRecords
  };

  const dispatch = createEventDispatcher();

  let anchoBarraLateral = 240;
  let estaRedimensionando = false;
  let seccionActiva: Seccion | null = 'operaciones';

  function cambiarSeccion(seccion: Seccion) {
    seccionActiva = seccionActiva === seccion ? null : seccion;
  }

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
  <MainControls {seccionActiva} on:cambiarSeccion={(e) => cambiarSeccion(e.detail)} />

  <!-- PANEL DESPLEGABLE DINÁMICO -->
  {#if seccionActiva && PANELS[seccionActiva]}
    <svelte:component
      this={PANELS[seccionActiva]}
      on:seleccionarOpcion={handleSeleccionarOpcion}
    />
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