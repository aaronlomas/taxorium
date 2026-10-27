<script lang="ts">
	/**
	 * @component Check
	 * @description Componente primitivo de casilla de verificación (Checkbox) accesible y personalizable.
	 */
	import type { Snippet } from 'svelte';
	import type { HTMLInputAttributes } from 'svelte/elements';
	import { IconCheck } from '@tabler/icons-svelte';

	interface Props extends Omit<HTMLInputAttributes, 'size'> {
		checked?: boolean;
		label?: string;
		description?: string;
		disabled?: boolean;
		size?: 'sm' | 'md' | 'lg';
		variant?: 'primary' | 'emerald' | 'purple' | 'blue';
		containerClass?: string;
		children?: Snippet;
	}

	let {
		checked = $bindable(false),
		label,
		description,
		disabled = false,
		size = 'md',
		variant = 'emerald',
		class: className = '',
		containerClass = '',
		title,
		children,
		id = crypto.randomUUID(),
		...rest
	}: Props = $props();

	const boxSizes = {
		sm: 'size-3.5 rounded-sm',
		md: 'size-4 rounded-sm',
		lg: 'size-5 rounded'
	};

	const iconSizes = {
		sm: 10,
		md: 12,
		lg: 14
	};

	const variantStyles = {
		primary: 'bg-emerald-600 border-emerald-500 text-white',
		emerald: 'bg-emerald-600 border-emerald-500 text-white',
		blue: 'bg-brand-bg-blue border-brand-border text-white',
		purple: 'bg-purple-600 border-purple-500 text-white'
	};
</script>

<label
	for={id}
	{title}
	class={[
		'inline-flex items-center gap-2 select-none',
		disabled ? 'cursor-not-allowed opacity-50' : 'cursor-pointer',
		containerClass
	]}
>
	<input
		type="checkbox"
		{id}
		bind:checked
		{disabled}
		class="sr-only peer"
		{...rest}
	/>

	<div
		class={[
			'flex items-center justify-center border transition-all duration-150',
			boxSizes[size],
			checked
				? variantStyles[variant]
				: 'border-brand-border bg-brand-bg-primary hover:border-neutral-500',
			'peer-focus-visible:ring-2 peer-focus-visible:ring-emerald-500 peer-focus-visible:ring-offset-2 peer-focus-visible:ring-offset-neutral-950',
			className
		]}
	>
		{#if checked}
			<IconCheck size={iconSizes[size]} stroke={3} />
		{/if}
	</div>

	{#if children}
		<div class="flex flex-col">
			{@render children()}
		</div>
	{:else if label}
		<div class="flex flex-col">
			<span class="text-xs font-medium text-brand-text-secondary">{label}</span>
			{#if description}
				<span class="text-[11px] text-brand-text-secondary">{description}</span>
			{/if}
		</div>
	{/if}
</label>
