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

	// Filtra la entrada del teclado en tiempo real
	function handleInput(e: Event) {
		const target = e.target as HTMLInputElement;
		let rawValue = target.value;

		// Permite números, un solo punto decimal y un solo signo menos al inicio
		rawValue = rawValue
			.replace(/[^0-9.-]/g, '') // Elimina caracteres no válidos
			.replace(/(\..*?)\..*/g, '$1') // Deja solo el primer punto decimal
			.replace(/(?!^)-/g, ''); // Deja el signo menos solo si está al principio

		let numValue = Number(rawValue);

		// Valida los límites si es un número válido
		if (!isNaN(numValue) && rawValue !== '' && rawValue !== '-') {
			if (max !== undefined && numValue > max) rawValue = String(max);
			if (min !== undefined && numValue < min) rawValue = String(min);
		}

		// Actualiza el estado y la vista del input
		value = rawValue;
		target.value = rawValue;
	}

	// Convierte a número válido al perder el foco (ej: si quedó un "-" solo o vacío)
	function handleBlur() {
		let current = Number(value);
		if (isNaN(current) || value === '') {
			value = min !== undefined ? min : 0;
		} else {
			value = current;
		}
	}
</script>

<div class="flex w-full flex-col {containerClass}">
	{#if label}
		<label for={id} class="block text-sm font-medium text-neutral-400">
			{label}
		</label>
	{/if}

	<div
		class="flex w-full overflow-hidden rounded-sm border border-neutral-800 bg-neutral-900 focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500 {error
			? 'border-red-500 focus-within:border-red-500 focus-within:ring-red-500'
			: ''}"
	>
		<!-- Usamos type="text" con inputmode="decimal" para teclados móviles -->
		<input
			{id}
			type="text"
			inputmode="decimal"
			{value}
			oninput={handleInput}
			onblur={handleBlur}
			class="h-full w-full border-none bg-transparent px-3 text-sm text-neutral-200 outline-none focus:ring-0 focus:outline-none {className}"
		/>

		<div class="flex flex-col border-l border-neutral-700">
			<button
				aria-label="Incrementar"
				title="Incrementar"
				type="button"
				tabindex="-1"
				onclick={increment}
				class="flex flex-1 items-center justify-center bg-neutral-800 px-2 text-neutral-400 transition-colors hover:bg-neutral-700 hover:text-white"
			>
				<svg class="size-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
				<svg class="size-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
