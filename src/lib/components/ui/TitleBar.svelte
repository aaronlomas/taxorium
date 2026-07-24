<script lang="ts">
	import { IconMinus, IconMaximize, IconX } from '@tabler/icons-svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	const ventanaApp = getCurrentWindow();

	async function minimizar() {
		try {
			await ventanaApp.minimize();
		} catch (e) {
			console.error(e);
		}
	}

	async function alternarMaximizar() {
		try {
			await ventanaApp.toggleMaximize();
		} catch (e) {
			console.error(e);
		}
	}

	async function cerrar() {
		try {
			await ventanaApp.close();
		} catch (e) {
			console.error(e);
		}
	}
</script>

<div
	class="flex h-8 shrink-0 select-none items-center justify-between border-b border-neutral-900 bg-neutral-950"
>
	<div
		data-tauri-drag-region
		class="flex h-full flex-1 cursor-default items-center gap-2 px-4"
	>
		<div
			class="pointer-events-none flex h-4 w-4 items-center justify-center rounded-sm bg-blue-600"
		>
			<span class="text-[9px] font-bold text-white">TX</span>
		</div>
		<span class="pointer-events-none text-xs font-medium tracking-wide text-neutral-400">
			Taxorium
		</span>
	</div>
  
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
