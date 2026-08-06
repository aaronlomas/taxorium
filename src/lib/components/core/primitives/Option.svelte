<script lang="ts">
	import { IconChevronDown } from '@tabler/icons-svelte';

	export interface OptionItem {
		value: string | number;
		label: string;
	}

	interface Props {
		label?: string;
		error?: string;
		value?: string | number;
		options?: (OptionItem | string | number)[];
		id?: string;
		containerClass?: string;
		class?: string;
		placeholder?: string;
		/** Si es false (default), se comporta como Select fijo. Si es true, permite escribir libremente. */
		editable?: boolean;
		/** Si es true y editable es true, filtra las opciones según lo que escribe el usuario */
		filterOptions?: boolean;
	}

	let {
		label,
		error,
		value = $bindable(''),
		options = [],
		class: className = '',
		containerClass = '',
		id = crypto.randomUUID(),
		placeholder = 'Seleccionar...',
		editable = false,
		filterOptions = true
	}: Props = $props();

	let isOpen = $state(false);
	let wrapperElement: HTMLDivElement | undefined = $state();
	let displayValue = $state('');

	let normalizedOptions = $derived(
		options.map((opt) => {
			if (typeof opt === 'object' && opt !== null && 'value' in opt) {
				return opt as OptionItem;
			}
			const strOpt = String(opt).toUpperCase();
			return { value: strOpt, label: strOpt } as OptionItem;
		})
	);

	// Sincronizar el texto del input/select con el 'value' externo
	$effect(() => {
		const matchedOption = normalizedOptions.find((opt) => opt.value === value);
		displayValue = matchedOption ? matchedOption.label : String(value ?? '');
	});

	// Filtrar la lista si está en modo editable y el usuario está escribiendo
	let filteredOptions = $derived(
		editable && filterOptions && displayValue && isOpen
			? normalizedOptions.filter((opt) => opt.label.toLowerCase().includes(displayValue.toLowerCase()))
			: normalizedOptions
	);

	let selectedLabel = $derived(normalizedOptions.find((opt) => opt.value === value)?.label || placeholder);

	function toggle() {
		isOpen = !isOpen;
	}

	function selectOption(option: OptionItem) {
		value = option.value;
		displayValue = option.label;
		isOpen = false;
	}

	function handleInput(event: Event) {
		if (!editable) return;
		const target = event.target as HTMLInputElement;
		// Forzar a mayúsculas por seguridad/estandarización en DB
		displayValue = target.value.toUpperCase();
		value = displayValue;
		isOpen = true;
	}

	function handleWindowClick(event: MouseEvent) {
		if (isOpen && wrapperElement && !wrapperElement.contains(event.target as Node)) {
			isOpen = false;
		}
	}
</script>

<svelte:window onclick={handleWindowClick} />

<div class="flex w-full flex-col {containerClass}" bind:this={wrapperElement}>
	{#if label}
		<label for={id} class="block text-sm font-medium text-neutral-400">
			{label}
		</label>
	{/if}

	<div class="relative w-full rounded-sm">
		{#if editable}
			<!-- MODO EDITABLE (Input + Botón desplegable) -->
			<div
				class="flex w-full items-center justify-between rounded-sm border border-neutral-800 bg-neutral-900 text-sm focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500 {error
					? 'border-red-500 focus-within:border-red-500 focus-within:ring-red-500'
					: ''} {className}"
			>
				<input
					{id}
					type="text"
					value={displayValue}
					oninput={handleInput}
					onfocus={() => (isOpen = true)}
					{placeholder}
					class="h-9 w-full border-0 bg-transparent text-sm text-neutral-100 placeholder-neutral-500 focus:ring-0"
				/>
				<button
					type="button"
					tabindex="-1"
					onclick={toggle}
					aria-label="Abrir opciones"
					class="px-2 text-neutral-400 hover:text-neutral-200 focus:outline-none"
				>
					<IconChevronDown class="size-4 transition-transform {isOpen ? 'rotate-180' : ''}" />
				</button>
			</div>
		{:else}
			<!-- MODO FIJO / SELECT (Solo selección de lista) -->
			<button
				type="button"
				{id}
				onclick={toggle}
				class="flex w-full items-center justify-between rounded-sm border border-neutral-800 bg-neutral-900 px-2 py-2 text-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none {error
					? 'border-red-500 focus:border-red-500 focus:ring-red-500'
					: ''} {className}"
			>
				<span class="truncate {value ? 'text-neutral-100' : 'text-neutral-500'}">
					{selectedLabel}
				</span>
				<IconChevronDown
					class="size-4 text-neutral-400 transition-transform {isOpen ? 'rotate-180' : ''}"
				/>
			</button>
		{/if}

		<!-- LISTA DESPLEGABLE (Compartida por ambas variantes) -->
		{#if isOpen && filteredOptions.length > 0}
			<ul
				class="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-sm border border-neutral-700 bg-neutral-900 text-sm shadow-lg focus:outline-none"
			>
				{#each filteredOptions as option}
					<!-- svelte-ignore a11y_click_events_have_key_events -->
					<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
					<li
						class="cursor-pointer px-2 py-1.5 hover:bg-neutral-800 {value === option.value
							? 'bg-blue-600/20 font-medium text-blue-400'
							: 'text-neutral-200'}"
						onclick={() => selectOption(option)}
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

