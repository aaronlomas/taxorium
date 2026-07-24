<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		children?: Snippet; // contenido opcional
	}

	let { isOpen = $bindable(), onClose, children }: Props = $props();

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			onClose();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<!-- Backdrop / Fondo oscuro con desenfoque -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 transition-opacity"
		role="dialog"
		aria-modal="true"
	>
		<!-- Contenedor principal del Modal -->
		<div
			class="relative flex h-[85vh] w-full max-w-4xl flex-col overflow-hidden border border-neutral-800 bg-neutral-950 text-slate-100 shadow-2xl"
		>
			<!-- Si se pasan slots/children, se renderizan aquí. De lo contrario, muestra la rejilla por defecto -->
			{#if children}
				{@render children()}
			{:else}
				<!-- Grid de 4 Columnas y 7 Filas -->
				<div class="grid h-full w-full grid-cols-4 grid-rows-7 gap-3 p-6">
					{#each Array(28) as _, i}
						<div
							class="flex items-center justify-center rounded-lg border border-neutral-800 bg-neutral-800 font-mono text-xs text-slate-400 transition-all hover:border-indigo-500/50 hover:bg-slate-800"
						>
							Celda {i + 1}
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
{/if}
