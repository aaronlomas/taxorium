<script lang="ts">
	import { IconAdjustments, IconCheck, IconSquare } from '@tabler/icons-svelte';
	import Hint from './Hint.svelte';

	export interface CheckItem {
		id: string;
		label: string;
		checked: boolean;
	}

	interface Props {
		label?: string;
		items?: CheckItem[];
		storageKey?: string;
		legacyStorageKeys?: string[];
		containerClass?: string;
		class?: string;
	}

	let {
		label = undefined,
		items = $bindable([]),
		storageKey = '',
		legacyStorageKeys = [],
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

	function clearLegacy() {
		try {
			if (typeof localStorage === 'undefined') return;
			for (const legacyKey of legacyStorageKeys) {
				localStorage.removeItem(`taxorium.columnas.${legacyKey}`);
			}
		} catch {
			console.error('Error limpiando configuración de columnas anterior');
		}
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
			clearLegacy();
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
	<Hint text="Columnas" side="bottom">
		<button
			type="button"
			onclick={toggleDropdown}
			class="flex items-center rounded-sm border border-brand-border bg-brand-bg-secondary p-1 text-sm text-brand-text-primary hover:bg-brand-bg-hover hover:text-brand-text-primary focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none {className}"
		>
			<IconAdjustments size={20} class="text-brand-text-secondary" />
			<span class="font-medium">{label}</span>
		</button>
	</Hint>

	<!-- LISTA DESPLEGABLE DE CHECKBOXES -->
	{#if isOpen}
		<div
			class="absolute top-full z-10 min-w-48 rounded-sm border border-brand-border bg-brand-bg-primary py-1 text-sm shadow-lg focus:outline-none"
		>
			<div class="flex flex-col">
				{#each items as item (item.id)}
					<button
						type="button"
						onclick={() => toggleItem(item.id)}
						class="flex items-center justify-between px-3 py-1 text-left text-brand-text-primary hover:bg-brand-bg-hover hover:text-brand-text-primary"
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
								<IconSquare size={16} class="text-brand-text-secondary hover:text-brand-text-primary" />
							{/if}
						</div>
					</button>
				{/each}
			</div>
		</div>
	{/if}
</div>
