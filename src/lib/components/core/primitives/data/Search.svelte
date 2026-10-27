<script lang="ts">
	import { IconSearch } from '@tabler/icons-svelte';

	interface Props {
		value?: string;
		placeholder?: string;
	}

	let { value = $bindable(''), placeholder = 'Buscar...' }: Props = $props();
</script>

<div
	class="flex items-center rounded-lg border border-brand-border bg-brand-bg-secondary px-4 text-brand-text-secondary"
>
	<IconSearch size={16} />
	<input
		type="text"
		{placeholder}
		bind:value
		class="h-7.5 w-full border-0 bg-brand-bg-secondary text-sm leading-none focus:ring-0"
	/>
</div>

<style>
	::placeholder {
		color: var(--brand-text-secondary);
	}
</style>
