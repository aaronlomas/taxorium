<script lang="ts">
	interface Props {
		label?: string;
		error?: string;
		value?: number | string;
		id?: string;
		containerClass?: string;
		class?: string;
		step?: number;
		min?: number;
		max?: number;
	}

	let {
		label,
		error,
		value = $bindable(0),
		class: className = '',
		containerClass = '',
		id = crypto.randomUUID(),
		step = 1,
		min,
		max
	}: Props = $props();

	function increment() {
		let current = Number(value) || 0;
		if (max !== undefined && current >= max) return;
		value = current + step;
	}

	function decrement() {
		let current = Number(value) || 0;
		if (min !== undefined && current <= min) return;
		value = current - step;
	}
</script>

<div class="flex w-full flex-col {containerClass}">
	{#if label}
		<label for={id} class="mb-1 block text-xs font-medium text-neutral-400">
			{label}
		</label>
	{/if}

	<div
		class="flex h-10 w-full overflow-hidden border border-neutral-700 bg-neutral-900 focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500 {error
			? 'border-red-500 focus-within:border-red-500 focus-within:ring-red-500'
			: ''}"
	>
		<div
			{id}
			class="flex h-full w-full items-center bg-transparent px-3 text-sm text-neutral-200 {className}"
		>
			{value}
		</div>
		<div class="flex flex-col border-l border-neutral-700">
			<button
				aria-label="Incrementar"
				title="Incrementar"
				type="button"
				tabindex="-1"
				onclick={increment}
				class="flex flex-1 items-center justify-center bg-neutral-800 px-2 text-neutral-400 transition-colors hover:bg-neutral-700 hover:text-white"
			>
				<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
				</svg>
			</button>
			<button
				aria-label="Decrementar"
				title="Decrementar"
				type="button"
				tabindex="-1"
				onclick={decrement}
				class="flex flex-1 items-center justify-center border-t border-neutral-700 bg-neutral-800 px-2 text-neutral-400 transition-colors hover:bg-neutral-700 hover:text-white"
			>
				<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M19 9l-7 7-7-7"
					/>
				</svg>
			</button>
		</div>
	</div>

	{#if error}
		<span class="mt-1 text-xs text-red-500">{error}</span>
	{/if}
</div>
