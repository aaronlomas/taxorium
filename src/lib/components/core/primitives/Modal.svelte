<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		children?: Snippet;
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
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 transition-opacity"
		role="dialog"
		aria-modal="true"
	>
		<!-- Contenedor principal del Modal -->
		<div
			class="relative flex h-[85vh] w-full max-w-4xl flex-col overflow-hidden border border-neutral-800 bg-neutral-950 text-slate-100 shadow-2xl"
		>
			{@render children?.()}
		</div>
	</div>
{/if}
