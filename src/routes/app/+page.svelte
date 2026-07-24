<script lang="ts">
  import Sidebar from '$lib/components/ui/Sidebar.svelte';
  import TabBar from '$lib/components/ui/tab-bar/TabBar.svelte';
  import Tab from '$lib/components/ui/tab-bar/tab/Tab.svelte';
  import { 
    IconBrandSvelte, 
    IconDatabase, 
    IconBolt,
    IconCheck
  } from '@tabler/icons-svelte';

  import { auth, currentUser } from '$lib/stores/auth';
  import { tenantStore, currentTenant } from '$lib/stores/tenant';
  import { onMount } from 'svelte';

  onMount(async () => {
    if ($currentUser) {
      await tenantStore.load();
    }
  });

  import {
    IconLayoutDashboard,
    IconUsers,
    IconPackage
  } from '@tabler/icons-svelte';

  // Inicializamos solo con el Dashboard
  let pestanas = [
    { id: 'Dashboard', titulo: 'Dashboard', icono: IconLayoutDashboard, colorIcono: 'text-blue-500', activo: true }
  ];

  function seleccionarPestana(id: string) {
    pestanas = pestanas.map(p => ({ ...p, activo: p.id === id }));
  }

  function cerrarPestana(id: string) {
    pestanas = pestanas.filter(p => p.id !== id);
    // Si cerramos el activo, activar la última pestaña disponible
    if (pestanas.length > 0 && !pestanas.some(p => p.activo)) {
      pestanas[pestanas.length - 1].activo = true;
    }
  }

  function manejarSeleccionSidebar(event: CustomEvent<any>) {
    const elemento = event.detail;
    const existe = pestanas.find(p => p.id === elemento.nombre);
    
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
      pestanas = [...pestanas.map(p => ({ ...p, activo: false })), nuevaPestana];
    }
  }

  // Obtenemos la pestaña activa para renderizar el contenido correcto
  $: pestanaActiva = pestanas.find(p => p.activo);
</script>

<div class="flex h-full w-full bg-neutral-950 overflow-hidden font-sans text-neutral-300">
  <Sidebar on:seleccionarOpcion={manejarSeleccionSidebar} />
  
  <main class="flex-1 flex flex-col min-w-0 bg-neutral-950">
    <TabBar>
      {#each pestanas as pestana (pestana.id)}
        <Tab 
          title={pestana.titulo} 
          icon={pestana.icono}
          iconColor={pestana.colorIcono}
          active={pestana.activo} 
          on:click={() => seleccionarPestana(pestana.id)}
          on:close={() => cerrarPestana(pestana.id)}
        />
      {/each}
    </TabBar>

    <!-- Área de contenido principal centrado -->
    <div class="flex-1 overflow-auto p-12 flex items-center justify-center">
      {#if pestanas.length === 0}
        <div class="text-neutral-500 text-sm">Selecciona una opción en la barra lateral para comenzar</div>
      {:else if pestanaActiva?.id === 'Dashboard'}
        <div class="max-w-md w-full flex flex-col gap-2 animate-in fade-in duration-300">
          <div class="w-12 h-12 bg-blue-500 rounded-2xl flex items-center justify-center text-white mb-2 shadow-lg shadow-blue-500/20">
            <IconBolt size={28} fill="currentColor" />
          </div>
          
          <h1 class="text-xl font-medium text-neutral-200">Taxorium</h1>
          <p class="text-neutral-400">
            Bienvenido{$currentTenant ? ', ' + $currentTenant.razon_social : ''}
          </p>
          <div class="text-neutral-500 flex items-center gap-2 text-sm mt-1">
            {$currentUser?.email || 'Cargando usuario...'} 
            <span>·</span>
            <span class="text-neutral-400 flex items-center gap-1">
              <IconCheck size={16} stroke={2} />
              Sesión activa
            </span>
          </div>
          
          {#if $currentTenant}
            <div class="mt-4 flex flex-col gap-1.5 text-neutral-400">
              <p>RUC {$currentTenant.ruc}</p>
              <p>Serie boleta: {$currentTenant.serie_boleta}</p>
              <p>Serie factura: {$currentTenant.serie_factura}</p>
            </div>
          {/if}
          
          <div class="mt-6 flex flex-col gap-2 text-neutral-400">
            <p>🚀</p>
            <p>El módulo de emisión de comprobantes está en desarrollo.</p>
            <p>La base de seguridad y autenticación está completa.</p>
          </div>

          <button 
            class="mt-6 self-start text-neutral-400 hover:text-white transition-colors cursor-pointer text-sm font-medium"
            on:click={() => auth.signOut()}
          >
            Cerrar sesión
          </button>
        </div>
      {:else}
        <div class="flex flex-col items-center justify-center gap-4 text-neutral-500 animate-in fade-in duration-300">
          <svelte:component this={pestanaActiva?.icono} size={48} stroke={1.5} class="text-neutral-700" />
          <h2 class="text-lg font-medium text-neutral-400">Módulo de {pestanaActiva?.titulo}</h2>
          <p class="text-sm">Esta sección se encuentra en desarrollo.</p>
        </div>
      {/if}
    </div>
  </main>
</div>
