<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		title?: string;
		children?: Snippet;
	}

	let { isOpen = $bindable(), onClose, title = 'Modal', children }: Props = $props();

	// Estado para las coordenadas de arrastre
	let position = $state({ x: 0, y: 0 });
	let isDragging = $state(false);
	let startCoords = { x: 0, y: 0 };

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			onClose();
		}
	}

	// Inicio del arrastre (al hacer clic/toque en la barra superior)
	function handlePointerDown(event: PointerEvent) {
		// Solo permitir arrastre con el botón principal (clic izquierdo / toque)
		if (event.button !== 0) return;

		isDragging = true;
		startCoords = {
			x: event.clientX - position.x,
			y: event.clientY - position.y
		};

		// Capturar el puntero para mantener el rastreo aunque el ratón salga de la barra
		(event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
	}

	// Movimiento
	function handlePointerMove(event: PointerEvent) {
		if (!isDragging) return;

		position = {
			x: event.clientX - startCoords.x,
			y: event.clientY - startCoords.y
		};
	}

	// Fin del arrastre
	function handlePointerUp(event: PointerEvent) {
		if (!isDragging) return;
		isDragging = false;
		(event.currentTarget as HTMLElement).releasePointerCapture(event.pointerId);
	}

	// Reiniciar la posición cuando se abre el modal
	$effect(() => {
		if (isOpen) {
			position = { x: 0, y: 0 };
		}
	});
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 transition-opacity"
		role="dialog"
		aria-modal="true"
	>
		<!-- Contenedor principal del Modal -->
		<div
			class="relative flex size-auto max-w-4xl flex-col border border-neutral-800 bg-neutral-950 text-slate-100 shadow-2xl transition-shadow"
			style="transform: translate3d({position.x}px, {position.y}px, 0);"
		>
			<!-- Barra Superior / Header para arrastrar -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				class="flex cursor-grab items-center justify-between border-b border-neutral-800 bg-neutral-900 px-2 py-1 select-none active:cursor-grabbing"
				onpointerdown={handlePointerDown}
				onpointermove={handlePointerMove}
				onpointerup={handlePointerUp}
			>
				<span class="text-sm font-medium text-blue-400">{title}</span>
				<button
					type="button"
					onclick={onClose}
					onpointerdown={(e) => e.stopPropagation()}
					class="rounded text-white transition-colors hover:text-red-700"
					aria-label="Cerrar modal"
				>
					✕
				</button>
			</div>

			<!-- Contenido principal -->
			<div>
				{@render children?.()}
			</div>
		</div>
	</div>
{/if}
