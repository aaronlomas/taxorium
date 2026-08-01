<script lang="ts">
	import { onMount } from 'svelte';

	interface OptionItem {
		value: string | number;
		label: string;
	}

	interface Props {
		label?: string;
		error?: string;
		value?: string | number;
		options?: OptionItem[];
		id?: string;
		containerClass?: string;
		class?: string;
		placeholder?: string;
	}

	let {
		label,
		error,
		value = $bindable(),
		options = [],
		class: className = '',
		containerClass = '',
		id = crypto.randomUUID(),
		placeholder = 'Seleccionar...'
	}: Props = $props();

	let isOpen = $state(false);
	let wrapperElement: HTMLDivElement | undefined = $state();

	function toggle() {
		isOpen = !isOpen;
	}

	function selectOption(optionValue: string | number) {
		value = optionValue;
		isOpen = false;
	}

	function handleWindowClick(event: MouseEvent) {
		if (isOpen && wrapperElement && !wrapperElement.contains(event.target as Node)) {
			isOpen = false;
		}
	}

	let selectedLabel = $derived(options.find((opt) => opt.value === value)?.label || placeholder);
</script>

<svelte:window onclick={handleWindowClick} />

<div class="flex w-full flex-col {containerClass}" bind:this={wrapperElement}>
	{#if label}
		<!-- svelte-ignore a11y_label_has_associated_control -->
		<label class="block text-sm font-medium text-neutral-400">
			{label}
		</label>
	{/if}

	<div class="relative w-full">
		<button
			type="button"
			{id}
			onclick={toggle}
			class="flex w-full items-center justify-between border border-neutral-700 bg-neutral-900 px-2 py-2 text-sm focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500 {error
				? 'border-red-500 focus-within:border-red-500 focus-within:ring-red-500'
				: ''} {className}"
		>
			<span class="truncate text-neutral-200">
				{selectedLabel}
			</span>
			<svg
				class="size-4 text-neutral-400 transition-transform {isOpen ? 'rotate-180' : ''}"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
			</svg>
		</button>

		{#if isOpen}
			<ul
				class="absolute z-10 mt-1 max-h-60 w-full overflow-auto border border-neutral-700 bg-neutral-900 text-sm shadow-lg focus:outline-none"
			>
				{#each options as option}
					<!-- svelte-ignore a11y_click_events_have_key_events -->
					<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
					<li
						class="cursor-pointer px-2 py-1 text-neutral-200 hover:bg-neutral-800 {value ===
						option.value
							? 'bg-blue-600/20 text-blue-400'
							: ''}"
						onclick={() => selectOption(option.value)}
					>
						{option.label}
					</li>
				{/each}
			</ul>
		{/if}
	</div>

	{#if error}
		<span class="mt-1 text-xs text-red-500">{error}</span>
	{/if}
</div>
