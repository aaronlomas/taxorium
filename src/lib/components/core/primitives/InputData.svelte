<script lang="ts">
	interface Props {
		label?: string;
		value?: string;
		id?: string;
		containerClass?: string;
		class?: string;
	}

	let {
		label,
		value = $bindable(today()),
		id = crypto.randomUUID(),
		containerClass = '',
		class: className = ''
	}: Props = $props();

	let isOpen = $state(false);
	let wrapperEl: HTMLDivElement | undefined = $state();

	// Navegación del calendario
	let viewYear = $state(new Date().getFullYear());
	let viewMonth = $state(new Date().getMonth()); // 0-indexed

	const MONTHS = [
		'Enero',
		'Febrero',
		'Marzo',
		'Abril',
		'Mayo',
		'Junio',
		'Julio',
		'Agosto',
		'Septiembre',
		'Octubre',
		'Noviembre',
		'Diciembre'
	];
	const DAYS = ['Do', 'Lu', 'Ma', 'Mi', 'Ju', 'Vi', 'Sá'];

	function today(): string {
		return new Date().toLocaleDateString('es-PE', {
			day: '2-digit',
			month: '2-digit',
			year: 'numeric'
		});
	}

	function toggle() {
		isOpen = !isOpen;
	}

	function handleWindowClick(e: MouseEvent) {
		if (isOpen && wrapperEl && !wrapperEl.contains(e.target as Node)) {
			isOpen = false;
		}
	}

	function prevMonth() {
		if (viewMonth === 0) {
			viewMonth = 11;
			viewYear -= 1;
		} else viewMonth -= 1;
	}

	function nextMonth() {
		if (viewMonth === 11) {
			viewMonth = 0;
			viewYear += 1;
		} else viewMonth += 1;
	}

	// Genera las celdas del mes (con huecos iniciales)
	let cells = $derived(
		(() => {
			const firstDay = new Date(viewYear, viewMonth, 1).getDay(); // 0=Dom
			const daysInMonth = new Date(viewYear, viewMonth + 1, 0).getDate();
			const result: (number | null)[] = Array(firstDay).fill(null);
			for (let d = 1; d <= daysInMonth; d++) result.push(d);
			return result;
		})()
	);

	function selectDay(day: number | null) {
		if (!day) return;
		const dd = String(day).padStart(2, '0');
		const mm = String(viewMonth + 1).padStart(2, '0');
		value = `${dd}/${mm}/${viewYear}`;
		isOpen = false;
	}

	// Día actualmente seleccionado para resaltarlo
	let selectedDay = $derived(
		(() => {
			if (!value) return null;
			const parts = value.split('/');
			if (parts.length !== 3) return null;
			const [d, m, y] = parts;
			if (Number(m) - 1 === viewMonth && Number(y) === viewYear) return Number(d);
			return null;
		})()
	);
</script>

<svelte:window onclick={handleWindowClick} />

<div class="relative flex w-full flex-col {containerClass}" bind:this={wrapperEl}>
	{#if label}
		<label for={id} class="block text-sm font-medium text-neutral-400">
			{label}
		</label>
	{/if}

	<!-- Input Fecha -->
	<div
		class="flex w-full items-center overflow-hidden border border-neutral-700 bg-neutral-900 text-sm {isOpen
			? 'border-blue-500 ring-1 ring-blue-500'
			: ''} {className}"
	>
		<span {id} class="flex-1 px-2 py-2 text-neutral-200 select-none">{value}</span>

		<button
			type="button"
			onclick={toggle}
			title="Abrir calendario"
			class="flex h-full items-center justify-center border-l border-neutral-700 bg-neutral-800 px-2 text-neutral-400 transition-colors hover:bg-neutral-700 hover:text-white"
		>
			<svg fill="none" stroke="currentColor" class="size-4" viewBox="0 0 24 24"
				><path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2"
				/></svg
			>
		</button>
	</div>

	<!-- Calendario desplegable personalizado -->
	{#if isOpen}
		<div
			class="absolute top-full z-50 mt-1 w-64 border border-neutral-700 bg-neutral-900 p-3 shadow-xl"
		>
			<!-- Encabezado: mes/año y navegación -->
			<div class="mb-2 flex items-center justify-between">
				<button
					type="button"
					onclick={prevMonth}
					aria-label="Mes anterior"
					class="flex size-6 items-center justify-center text-neutral-400 hover:text-white"
				>
					<svg
						fill="none"
						stroke="currentColor"
						aria-hidden="true"
						class="size-4"
						viewBox="0 0 24 24"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="m15 19-7-7 7-7"
						/></svg
					>
				</button>
				<span class="text-xs font-semibold text-neutral-200">
					{MONTHS[viewMonth]}
					{viewYear}
				</span>
				<button
					type="button"
					onclick={nextMonth}
					aria-label="Mes siguiente"
					class="flex size-6 items-center justify-center text-neutral-400 hover:text-white"
				>
					<svg
						fill="none"
						stroke="currentColor"
						aria-hidden="true"
						class="size-4"
						viewBox="0 0 24 24"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="m9 5 7 7-7 7"
						/></svg
					>
				</button>
			</div>

			<!-- Días de la semana -->
			<div class="mb-1 grid grid-cols-7 gap-0.5">
				{#each DAYS as d}
					<div class="text-center text-[10px] font-medium text-neutral-500">{d}</div>
				{/each}
			</div>

			<!-- Celdas del mes -->
			<div class="grid grid-cols-7 gap-0.5">
				{#each cells as cell}
					{#if cell === null}
						<div></div>
					{:else}
						<button
							type="button"
							onclick={() => selectDay(cell)}
							class="flex h-7 w-full items-center justify-center rounded text-xs transition-colors
								{cell === selectedDay ? 'bg-blue-600 text-white' : 'text-neutral-300 hover:bg-neutral-700'}"
						>
							{cell}
						</button>
					{/if}
				{/each}
			</div>
		</div>
	{/if}
</div>
