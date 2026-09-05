<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';
	import type { Snippet } from 'svelte';

	interface Props extends HTMLInputAttributes {
		label?: string;
		error?: string;
		value?: string;
		/** Input structure type */
		variant?: 'simple' | 'double' | 'triple';
		/** Initial icon (Required only in 'triple') */
		icon?: Snippet;
		/**Show and hide icon (Required in 'double' and 'triple') */
		action?: Snippet;
		containerClass?: string;
	}

	let {
		label,
		error,
		value = $bindable(''),
		variant = 'simple',
		icon,
		action,
		required = false,
		class: className = '',
		containerClass = '',
		id = crypto.randomUUID(),
		...rest
	}: Props = $props();

	// Layout: Mapping based on the variant chosen
	const variants = {
		simple: 'grid-cols-1',
		double: 'grid-cols-[1fr_auto]',
		triple: 'grid-cols-[auto_1fr_auto]'
	};
</script>

<div class="grid w-full {containerClass}">
	{#if label}
		<label for={id} class="block text-sm text-neutral-400">
			{label}{#if required}<span class="text-red-500"> *</span>{/if}
		</label>
	{/if}

	<!-- Base Container -->
	<div
		class="
      grid w-full items-center border border-neutral-800 bg-neutral-900 text-sm focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500 rounded-sm
      {variants[variant]}
      {error ? 'border-red-500 focus-within:border-red-500 focus-within:ring-red-500' : ''} 
      {className}
    "
	>
		{#if variant === 'triple' && icon}
			<div class="flex shrink-0 items-center justify-center text-neutral-400 border-r border-neutral-800 h-full px-2">
				{@render icon()}
			</div>
		{/if}

		<!-- 2. Simple input, to collect only informational data -->
		<input
			{id}
			bind:value
			class="border-none bg-transparent text-sm text-neutral-200 outline-none focus:ring-0 px-2"
			{...rest}
		/>

		<!-- Action icon (Variants 'double' and 'triple') -->
		{#if (variant === 'double' || variant === 'triple') && action}
			<div class="flex items-center justify-center text-neutral-400">
				{@render action()}
			</div>
		{/if}
	</div>

	{#if error}
		<span class="mt-1 text-xs text-red-500">{error}</span>
	{/if}
</div>
