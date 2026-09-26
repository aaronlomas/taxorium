<script lang="ts">
	import { IconSearch } from '@tabler/icons-svelte';

	interface Props {
		value?: string;
		placeholder?: string;
	}

	let { value = $bindable(''), placeholder = 'Buscar...' }: Props = $props();
</script>

<div class="flex items-center rounded-lg border border-neutral-800 px-4">
	<IconSearch size={16} />
	<input
		type="text"
		{placeholder}
		bind:value
		class="w-full border-0 bg-transparent text-sm focus:ring-0"
	/>
</div>
