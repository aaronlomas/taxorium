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
		/** Cantidad fija de decimales con la que se muestra y redondea el valor. */
		decimals?: number;
		disabled?: boolean;
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
		max,
		decimals,
		disabled = false
	}: Props = $props();

	function increment() {
		if (disabled) return;
		let current = Number(value) || 0;
		if (max !== undefined && current >= max) return;
		value = redondear(current + step);
	}

	function decrement() {
		if (disabled) return;
		let current = Number(value) || 0;
		if (min !== undefined && current <= min) return;
		value = redondear(current - step);
	}

	/** Redondea según los decimales configurados (o los del step) para evitar artefactos de coma flotante. */
	function redondear(n: number): number {
		const decimales = decimals ?? (String(step).split('.')[1] ?? '').length;
		return decimales > 0 ? Number(n.toFixed(decimales)) : n;
	}

	// Filtra la entrada del teclado en tiempo real
	function handleInput(e: Event) {
		if (disabled) return;
		const target = e.target as HTMLInputElement;
		let rawValue = target.value;

		// Acepta la coma como separador decimal (teclados con locale es-ES).
		if (rawValue.includes(',') && !rawValue.includes('.')) {
			rawValue = rawValue.replace(',', '.');
		}

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
		if (disabled) return;
		let current = Number(value);
		if (isNaN(current) || value === '') {
			current = min !== undefined ? min : 0;
		}
		value = decimals !== undefined ? current.toFixed(decimals) : current;
	}
</script>

<div class="flex w-full flex-col {containerClass}">
	{#if label}
		<label for={id} class="block text-sm font-medium text-brand-text-secondary">
			{label}
		</label>
	{/if}

	<div
		class="flex w-full overflow-hidden rounded-sm border border-brand-border bg-brand-bg-secondary focus-within:border-brand-border-sky {error
			? 'border-red-500 focus-within:border-red-500'
			: ''} {disabled ? 'cursor-not-allowed opacity-50' : ''}"
	>
		<!-- Usamos type="text" con inputmode="decimal" para teclados móviles -->
		<input
			{id}
			type="text"
			inputmode="decimal"
			{value}
			{disabled}
			oninput={handleInput}
			onblur={handleBlur}
			class="h-7.5 w-full border-none bg-transparent px-3 text-sm text-brand-text-primary outline-none focus:ring-0 focus:outline-none {disabled
				? 'cursor-not-allowed'
				: ''} {className}"
		/>
		<!-- Contenedor de botones -->
		<div class="flex flex-col border-l border-brand-border bg-brand-bg-secondary">
			<button
				aria-label="Incrementar"
				title="Incrementar"
				type="button"
				tabindex="-1"
				{disabled}
				onclick={increment}
				class="flex h-full items-center justify-center bg-none text-brand-text-secondary px-2 hover:bg-brand-bg-hover hover:text-brand-text-primary disabled:cursor-not-allowed disabled:hover:bg-brand-bg-secondary disabled:hover:text-brand-text-secondary"
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
				{disabled}
				onclick={decrement}
				class="flex h-full items-center justify-center border-t border-brand-border bg-none px-2 text-brand-text-secondary hover:bg-brand-bg-hover hover:text-brand-text-primary disabled:cursor-not-allowed disabled:hover:bg-brand-bg-secondary disabled:hover:text-brand-text-secondary"
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
