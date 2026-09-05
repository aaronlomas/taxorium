<script lang="ts">
	import { IconAdjustments, IconCheck, IconSquare } from '@tabler/icons-svelte';

	export interface CheckItem {
		id: string;
		label: string;
		checked: boolean;
	}

	interface Props {
		label?: string;
		items?: CheckItem[];
		storageKey?: string;
		containerClass?: string;
		class?: string;
	}

	let {
		label = 'Columnas',
		items = $bindable([]),
		storageKey = '',
		containerClass = '',
		class: className = ''
	}: Props = $props();

	let isOpen = $state(false);
	let wrapperElement: HTMLDivElement | undefined = $state();
	let restored = $state(false);

	function toggleDropdown() {
		isOpen = !isOpen;
	}

	function toggleItem(id: string) {
		items = items.map((item) => (item.id === id ? { ...item, checked: !item.checked } : item));
	}

	function storagePath() {
		return `taxorium.columnas.${storageKey}`;
	}

	function loadSaved(): Record<string, boolean> | null {
		try {
			if (typeof localStorage === 'undefined') return null;
			const raw = localStorage.getItem(storagePath());
			if (!raw) return null;
			const parsed = JSON.parse(raw);
			if (parsed && typeof parsed === 'object' && !Array.isArray(parsed)) return parsed;
			return null;
		} catch {
			return null;
		}
	}

	function persist() {
		try {
			if (typeof localStorage === 'undefined') return;
			const record: Record<string, boolean> = {};
			for (const item of items) record[item.id] = item.checked;
			localStorage.setItem(storagePath(), JSON.stringify(record));
		} catch {
			console.error('Error guardando configuración de columnas');
		}
	}

	$effect(() => {
		if (!storageKey) return;

		if (!restored) {
			restored = true;
			const saved = loadSaved();
			if (saved) {
				items = items.map((item) =>
					saved[item.id] !== undefined ? { ...item, checked: saved[item.id] } : item
				);
			}
			return;
		}

		persist();
	});

	function handleWindowClick(event: MouseEvent) {
		if (isOpen && wrapperElement && !wrapperElement.contains(event.target as Node)) {
			isOpen = false;
		}
	}
</script>

<svelte:window onclick={handleWindowClick} />

<div class="relative flex flex-col {containerClass}" bind:this={wrapperElement}>
	<!-- BOTÓN PRINCIPAL -->
	<button
		type="button"
		onclick={toggleDropdown}
		class="flex items-center gap-2 rounded-sm border border-neutral-800 bg-neutral-900 px-3 py-1 text-sm text-neutral-300 hover:border-neutral-700 hover:text-neutral-100 focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none {className}"
	>
		<IconAdjustments size={20} class="text-neutral-400" />
		<span class="font-medium">{label}</span>
	</button>

	<!-- LISTA DESPLEGABLE DE CHECKBOXES -->
	{#if isOpen}
		<div
			class="absolute z-10 top-full min-w-48 rounded-sm border border-neutral-700 bg-neutral-900 py-1 text-sm shadow-lg focus:outline-none"
		>
			<div class="flex flex-col">
				{#each items as item (item.id)}
					<button
						type="button"
						onclick={() => toggleItem(item.id)}
						class="flex items-center justify-between px-3 py-1.5 text-left text-neutral-300 transition-colors hover:bg-neutral-800 hover:text-neutral-100"
					>
						<span class="truncate pr-2">{item.label}</span>

						<!-- CHECKBOX PERSONALIZADO CON TABLER ICONS -->
						<div class="flex items-center justify-center text-blue-400">
							{#if item.checked}
								<div
									class="flex size-4 items-center justify-center rounded bg-blue-600/20 text-blue-400"
								>
									<IconCheck size={14} stroke={2.5} />
								</div>
							{:else}
								<IconSquare size={16} class="text-neutral-600 hover:text-neutral-500" />
							{/if}
						</div>
					</button>
				{/each}
			</div>
		</div>
	{/if}
</div>
