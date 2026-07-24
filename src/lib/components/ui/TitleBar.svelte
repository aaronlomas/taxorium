<script lang="ts">
  import { IconMinus, IconMaximize, IconX } from '@tabler/icons-svelte';
  
  // En Tauri v2, importamos getCurrentWindow para interactuar con la ventana actual
  import { getCurrentWindow } from '@tauri-apps/api/window';

  const ventanaApp = getCurrentWindow();

  async function minimizar() {
    try { await ventanaApp.minimize(); } catch(e) { console.error(e); }
  }

  async function alternarMaximizar() {
    try { await ventanaApp.toggleMaximize(); } catch(e) { console.error(e); }
  }

  async function cerrar() {
    try { await ventanaApp.close(); } catch(e) { console.error(e); }
  }
  
  async function iniciarArrastre(e: MouseEvent) {
    // Solo arrastrar si es el click izquierdo
    if (e.button === 0) {
      try { await ventanaApp.startDragging(); } catch(e) { console.error(e); }
    }
  }
</script>

<div
  class="h-8 flex justify-between items-center bg-neutral-950 select-none border-b border-neutral-900 shrink-0"
>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div 
    class="px-4 flex items-center gap-2 h-full cursor-default" 
    on:mousedown={iniciarArrastre}
  >
    <div class="w-4 h-4 rounded-sm bg-blue-600 flex items-center justify-center pointer-events-none">
      <span class="text-[9px] font-bold text-white">TX</span>
    </div>
    <span class="text-xs font-medium text-neutral-400 pointer-events-none tracking-wide">
      Taxorium
    </span>
  </div>
  
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div 
    class="flex-1 h-full cursor-default" 
    on:mousedown={iniciarArrastre}
  ></div>
  
  <div class="flex h-full">
    <button 
      class="h-full px-3.5 hover:bg-neutral-800 text-neutral-400 hover:text-neutral-200 transition-colors flex items-center justify-center cursor-default"
      on:click={minimizar}
      title="Minimizar"
      type="button"
      tabindex="-1"
    >
      <IconMinus size={16} stroke={1.5} />
    </button>
    <button 
      class="h-full px-3.5 hover:bg-neutral-800 text-neutral-400 hover:text-neutral-200 transition-colors flex items-center justify-center cursor-default"
      on:click={alternarMaximizar}
      title="Maximizar"
      type="button"
      tabindex="-1"
    >
      <IconMaximize size={14} stroke={1.5} />
    </button>
    <button 
      class="h-full px-4 hover:bg-red-500 hover:text-white text-neutral-400 transition-colors flex items-center justify-center group cursor-default"
      on:click={cerrar}
      title="Cerrar"
      type="button"
      tabindex="-1"
    >
      <IconX size={16} stroke={1.5} class="group-hover:text-white text-neutral-400 transition-colors" />
    </button>
  </div>
</div>
