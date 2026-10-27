<script lang="ts">
	import { IconX } from '@tabler/icons-svelte';

	interface Props {
		title: string;
		icon?: any;
		active?: boolean;
		onClick?: () => void;
		onClose?: () => void;
	}

	let { title, icon = null, active = false, onClick, onClose }: Props = $props();

	function manejarClick() {
		onClick?.();
	}

	function manejarCierre(e: MouseEvent) {
		e.stopPropagation();
		onClose?.();
	}
</script>

<div
	class="group flex max-w-48 min-w-24 cursor-pointer items-center gap-2 border-t border-r border-brand-border px-3 py-1 last:border-r-0 outline-none focus:outline-none"
	class:bg-brand-bg-primary={active}
	class:border-t-blue-500={active}
	class:text-brand-text-primary={active}
	class:bg-none={!active}
	class:border-t-transparent={!active}
	class:text-brand-text-secondary={!active}
	class:hover:bg-brand-bg-secondary={!active}
	onclick={manejarClick}
	onkeydown={(e) => e.key === 'Enter' && manejarClick()}
	role="button"
	tabindex="0"
>
	{#if icon}
		{@const Icon = icon}
		<Icon size={16} class="{active ? 'text-brand-text-sky' : 'text-brand-text-secondary'} shrink-0" />
	{/if}

	<span class="flex w-full justify-center truncate text-sm">{title}</span>

	<button
		type="button"
		class="shrink-0 rounded-md text-brand-text-secondary opacity-0 group-hover:opacity-100 hover:bg-brand-bg-hover hover:text-brand-text-hover focus:opacity-100"
		class:opacity-100={active}
		onclick={manejarCierre}
		aria-label="Cerrar pestaña"
	>
		<IconX size={14} />
	</button>
</div>
