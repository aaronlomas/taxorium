<script lang="ts">
	import { onMount } from 'svelte';
	import { createTypewriterLog } from './utilities/typewriter.ts';
	import Modal from '$lib/components/core/primitives/feedback/Modal.svelte';
	import { IconWindowMaximize, IconMoon, IconSun, IconBell } from '@tabler/icons-svelte';
	import { taxoLog, type LogEntry } from '$lib/features/taxoLog';

	// Instanciamos el log animado con velocidad opcional (por defecto 25ms por carácter)
	const animatedLog = createTypewriterLog(20);

	let isOpen = $state(false);
	let listaLog: HTMLDivElement | undefined = $state();
	let isDark = $state(true);

	const levelClass: Record<LogEntry['level'], string> = {
		error: 'text-red-400',
		warn: 'text-yellow-400',
		info: 'text-green-400'
	};

	const levelLabel: Record<LogEntry['level'], string> = {
		error: 'ERR',
		warn: 'WARN',
		info: 'INFO'
	};

	let totalEntradas = $derived($taxoLog.length);

	function fmt(d: Date) {
		return d.toLocaleTimeString('es-PE', { hour12: false });
	}

	function applyTheme(mode: 'dark' | 'light') {
		if (typeof document === 'undefined') return;

		const root = document.documentElement;
		root.classList.toggle('dark', mode === 'dark');
		root.classList.toggle('light', mode === 'light');
		localStorage.setItem('taxorium-theme', mode);
		window.dispatchEvent(new CustomEvent('taxorium-theme-change', { detail: { mode } }));
		isDark = mode === 'dark';
	}

	function toggleTheme() {
		applyTheme(isDark ? 'light' : 'dark');
	}

	onMount(() => {
		const savedTheme = localStorage.getItem('taxorium-theme');
		const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
		const nextMode =
			savedTheme === 'light' || savedTheme === 'dark' ? savedTheme : prefersDark ? 'dark' : 'light';
		applyTheme(nextMode);
	});

	// Mantiene la vista del log siempre al final (mensajes más recientes)
	$effect(() => {
		if (totalEntradas >= 0 && isOpen && listaLog) {
			listaLog.scrollTop = listaLog.scrollHeight;
		}
	});
</script>

<footer
	class="col-span-2 flex items-center gap-2 border-t border-brand-border px-3 font-mono text-xs text-brand-text-primary"
>
	<span>Log:</span>
	<div class="w-full truncate">
		{#if $animatedLog.entry}
			<span class="truncate {$animatedLog.entry ? levelClass[$animatedLog.entry.level] : ''}">
				[{fmt($animatedLog.entry.timestamp)}] [{levelLabel[$animatedLog.entry.level]}] [{$animatedLog
					.entry.source}] {$animatedLog.text}<span class="animate-pulse">|</span>
			</span>
		{:else}
			<span class="text-brand-text-secondary">Sin eventos</span>
		{/if}
	</div>
	<!-- BOTONES/CONTROLES/ACCIONES -->
	<div class="flex h-full items-center gap-x-2">
		<div>
			<button
				type="button"
				onclick={() => (isOpen = true)}
				title="Ver log completo"
				aria-label="Ver log completo"
				class="flex cursor-pointer items-center text-brand-text-secondary hover:text-brand-text-primary"
			>
				<IconWindowMaximize size={18} />
			</button>
		</div>
		<div class="border-l border-brand-border pl-2 gap-x-2 flex">
			<button
				type="button"
				onclick={toggleTheme}
				class="flex cursor-pointer items-center text-brand-text-secondary hover:text-brand-text-primary"
			>
				{#if isDark}
					<IconSun size={18} />
				{:else}
					<IconMoon size={18} />
				{/if}
			</button>
			<button
				type="button"
				onclick={toggleTheme}
				class="flex cursor-pointer items-center text-brand-text-secondary hover:text-brand-text-primary"
			>
				<IconBell size={18} />
			</button>
		</div>
	</div>
</footer>

<!-- VENTANA SECUNDARIA DEL LOG -->
<Modal bind:isOpen onClose={() => (isOpen = false)} title="Log del Sistema">
	<div
		bind:this={listaLog}
		class="h-[60vh] w-208 max-w-[80vw] overflow-auto bg-neutral-950 font-mono text-xs"
	>
		{#if $taxoLog.length === 0}
			<p class="p-2 text-neutral-600">Sin eventos</p>
		{:else}
			{#each $taxoLog as entry (entry.id)}
				<div class="flex flex-wrap gap-x-2 border-b border-neutral-800/60 px-2 py-1.5">
					<span class="text-neutral-500">[{fmt(entry.timestamp)}]</span>
					<span class={levelClass[entry.level]}>[{levelLabel[entry.level]}]</span>
					<span class="text-neutral-500">[{entry.source}]</span>
					<span class="wrap-break flex-1 whitespace-pre-wrap {levelClass[entry.level]}">
						{entry.message}
					</span>
				</div>
			{/each}
		{/if}
	</div>
</Modal>
