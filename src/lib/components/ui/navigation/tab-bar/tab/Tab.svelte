<script lang="ts">
	import { IconX } from '@tabler/icons-svelte';

	interface Props {
		title: string;
		icon?: any;
		active?: boolean;
		iconColor?: string;
		onClick?: () => void;
		onClose?: () => void;
	}

	let {
		title,
		icon = null,
		active = false,
		iconColor = 'text-neutral-400',
		onClick,
		onClose
	}: Props = $props();

	function manejarClick() {
		onClick?.();
	}

	function manejarCierre(e: MouseEvent) {
		e.stopPropagation();
		onClose?.();
	}
</script>

<div
	class="group flex max-w-48 min-w-32 cursor-pointer items-center gap-2 border-t border-r border-r-neutral-800 px-3 py-1 transition-colors last:border-r-0"
	class:bg-neutral-950={active}
	class:border-t-blue-500={active}
	class:text-white={active}
	class:bg-neutral-900={!active}
	class:border-t-transparent={!active}
	class:text-neutral-400={!active}
	class:hover:bg-neutral-800={!active}
	onclick={manejarClick}
	onkeydown={(e) => e.key === 'Enter' && manejarClick()}
	role="button"
	tabindex="0"
>
	{#if icon}
		{@const Icon = icon}
		<Icon size={16} class="{iconColor} shrink-0" />
	{/if}

	<span class="flex-1 truncate text-sm">{title}</span>

	<button
		type="button"
		class="shrink-0 rounded-md text-neutral-400 opacity-0 transition-all group-hover:opacity-100 hover:bg-neutral-700 hover:text-white focus:opacity-100"
		class:opacity-100={active}
		onclick={manejarCierre}
		aria-label="Cerrar pestaña"
	>
		<IconX size={14} />
	</button>
</div>
