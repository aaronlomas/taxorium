<script lang="ts">
	/**
	 * @component DropdownItem
	 * @description Ítem de menú para `Dropdown`. El click cierra automáticamente el menú.
	 */
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	interface Props extends HTMLButtonAttributes {
		variant?: 'default' | 'danger';
		children?: Snippet;
	}

	let { variant = 'default', children, class: className = '', ...rest }: Props = $props();

	const variants = {
		default: 'hover:bg-neutral-800',
		danger: 'hover:bg-red-700'
	} as const;
</script>

<button
	type="button"
	role="menuitem"
	class="flex w-full cursor-pointer items-center gap-x-2 p-2 text-left whitespace-nowrap {variants[
		variant
	]} {className}"
	{...rest}
>
	{@render children?.()}
</button>
