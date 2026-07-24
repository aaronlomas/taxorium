<script lang="ts">
	import { IconX } from '@tabler/icons-svelte';

	export let title: string;
	export let icon: any = null;
	export let active: boolean = false;
	export let iconColor: string = 'text-neutral-400';

	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	function manejarClick() {
		dispatch('click');
	}

	function manejarCierre(e: MouseEvent) {
		e.stopPropagation();
		dispatch('close');
	}
</script>

<div
	class="group flex max-w-48 min-w-32 cursor-pointer items-center gap-2 border-t-2 border-r border-r-neutral-800 px-3 py-2 transition-colors last:border-r-0"
	class:bg-neutral-950={active}
	class:border-t-blue-500={active}
	class:text-white={active}
	class:bg-neutral-900={!active}
	class:border-t-transparent={!active}
	class:text-neutral-400={!active}
	class:hover:bg-neutral-800={!active}
	on:click={manejarClick}
	on:keydown={(e) => e.key === 'Enter' && manejarClick()}
	role="button"
	tabindex="0"
>
	{#if icon}
		<svelte:component this={icon} size={16} class="{iconColor} shrink-0" />
	{/if}

	<span class="flex-1 truncate text-sm">{title}</span>

	<button
		class="shrink-0 rounded-md text-neutral-400 opacity-0 transition-all group-hover:opacity-100 hover:bg-neutral-700 hover:text-white focus:opacity-100"
		class:opacity-100={active}
		on:click={manejarCierre}
		aria-label="Cerrar pestaña"
	>
		<IconX size={14} />
	</button>
</div>
