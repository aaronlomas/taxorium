<script lang="ts">
	import type { HTMLTextareaAttributes } from 'svelte/elements';

	interface Props extends HTMLTextareaAttributes {
		label?: string;
		error?: string;
		value?: string;
		containerClass?: string;
		disabled?: boolean;
		rows?: number;
	}

	let {
		label,
		error,
		value = $bindable(''),
		required = false,
		disabled = false,
		rows = 4,
		class: className = '',
		containerClass = '',
		id = crypto.randomUUID(),
		...rest
	}: Props = $props();
</script>

<div class="grid w-full {containerClass}">
	{#if label}
		<label for={id} class="mb-1 block text-sm text-neutral-400">
			{label}{#if required}<span class="text-red-500"> *</span>{/if}
		</label>
	{/if}

	<div
		class="
      w-full rounded-sm border border-neutral-800 bg-neutral-900 text-sm focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500
      {error ? 'border-red-500 focus-within:border-red-500 focus-within:ring-red-500' : ''}
      {disabled ? 'cursor-not-allowed opacity-50' : ''}
      {className}
    "
	>
		<textarea
			{id}
			bind:value
			{required}
			{disabled}
			{rows}
			class="w-full resize-none border-none bg-transparent px-2 py-1.5 text-sm text-neutral-200 outline-none focus:ring-0 {disabled
				? 'cursor-not-allowed'
				: ''}"
			{...rest}
		></textarea>
	</div>

	{#if error}
		<span class="mt-1 text-xs text-red-500">{error}</span>
	{/if}
</div>
