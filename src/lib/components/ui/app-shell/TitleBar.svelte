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
	class="flex h-8 shrink-0 items-center justify-between border-b border-brand-border bg-brand-bg-primary select-none"
>
	<!-- Región de arrastre: cubre toda la barra excepto los botones -->
	<div data-tauri-drag-region class="flex h-full flex-1 cursor-default items-center gap-2 px-4">
		<div
			class="pointer-events-none flex h-4 w-4 items-center justify-center rounded-sm bg-blue-600"
		>
			<span class="text-[9px] font-bold text-white">TX</span>
		</div>
		<span class="pointer-events-none text-xs font-medium tracking-wide text-brand-text-secondary">
			Taxorium
		</span>
	</div>

	<div class="flex h-full">
		<button
			class="flex h-full cursor-default items-center justify-center px-3.5 text-brand-text-secondary transition-colors hover:bg-brand-bg-secondary hover:text-brand-text-hover"
			onclick={minimizar}
			aria-label="Minimizar"
			type="button"
			tabindex="-1"
		>
			<IconMinus size={16} stroke={1.5} />
		</button>
		<button
			class="flex h-full cursor-default items-center justify-center px-3.5 text-brand-text-secondary transition-colors hover:bg-brand-bg-secondary hover:text-brand-text-hover"
			onclick={alternarMaximizar}
			aria-label="Maximizar"
			type="button"
			tabindex="-1"
		>
			<IconMaximize size={14} stroke={1.5} />
		</button>
		<button
			class="group flex h-full cursor-default items-center justify-center px-4 text-brand-text-secondary transition-colors hover:bg-red-500 hover:text-brand-text-hover"
			onclick={cerrar}
			aria-label="Cerrar"
			type="button"
			tabindex="-1"
		>
			<IconX
				size={16}
				stroke={1.5}
				class="text-brand-text-secondary transition-colors group-hover:text-brand-text-hover"
			/>
		</button>
	</div>
</div>