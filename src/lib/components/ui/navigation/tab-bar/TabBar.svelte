<script lang="ts">
	import type { Snippet } from 'svelte';
	import Scroll from '$lib/components/core/primitives/Scroll.svelte';

	interface Props {
		children?: Snippet;
	}

	let { children }: Props = $props();
</script>

<Scroll orientation="horizontal" class="bg-brand-bg-secondary select-none px-1" contentClass="flex items-center">
	{@render children?.()}
</Scroll>