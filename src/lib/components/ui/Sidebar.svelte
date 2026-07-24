<script lang="ts">
  import {
    IconLayoutDashboard,
    IconUsers,
    IconPackage
  } from '@tabler/icons-svelte';

  let anchoBarraLateral = 260;
  let estaRedimensionando = false;

  function iniciarRedimension(e: MouseEvent) {
    estaRedimensionando = true;
    document.addEventListener('mousemove', redimensionar);
    document.addEventListener('mouseup', detenerRedimension);
    // Prevenir la selección de texto durante el arrastre
    document.body.style.userSelect = 'none';
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
    document.body.style.userSelect = '';
  }

  const elementosNavegacion = [
    { nombre: 'Dashboard', icono: IconLayoutDashboard, ruta: '/dashboard' },
    { nombre: 'Clientes', icono: IconUsers, ruta: '/clientes' },
    { nombre: 'Productos', icono: IconPackage, ruta: '/productos' },
  ];
</script>

<aside
  class="relative h-screen bg-neutral-950 text-neutral-300 flex flex-col border-r border-neutral-800"
  style="width: {anchoBarraLateral}px;"
>
  <div class="p-4 font-bold text-white flex items-center gap-3 border-b border-neutral-800">
    <div class="w-8 h-8 rounded bg-blue-600 flex items-center justify-center text-xs">
      TX
    </div>
    <span class="tracking-wide">Taxorium</span>
  </div>

  <nav class="flex-1 overflow-y-auto py-4 px-2 flex flex-col gap-2">
    {#each elementosNavegacion as elemento}
      <a
        href={elemento.ruta}
        class="flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-neutral-800 hover:text-white transition-colors cursor-pointer group"
      >
        <svelte:component
          this={elemento.icono}
          size={20}
          stroke={1.5}
          class="text-neutral-400 group-hover:text-blue-400 transition-colors"
        />
        <span class="text-sm font-medium">{elemento.nombre}</span>
      </a>
    {/each}
  </nav>

  <!-- Control de redimensionamiento -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-blue-500 z-10 transition-colors opacity-0 hover:opacity-100"
    class:bg-blue-500={estaRedimensionando}
    class:opacity-100={estaRedimensionando}
    on:mousedown={iniciarRedimension}
  ></div>
</aside>
