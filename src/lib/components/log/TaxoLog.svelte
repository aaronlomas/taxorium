<script lang="ts">
	import { createTypewriterLog } from './utilities/typewriter.ts';
	import Modal from '$lib/components/core/primitives/feedback/Modal.svelte';
	import { IconWindowMaximize } from '@tabler/icons-svelte';
	import { taxoLog, type LogEntry } from '$lib/features/taxoLog';

	// Instanciamos el log animado con velocidad opcional (por defecto 25ms por carácter)
	const animatedLog = createTypewriterLog(20);

	let isOpen = $state(false);
	let listaLog: HTMLDivElement | undefined = $state();

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

	// Mantiene la vista del log siempre al final (mensajes más recientes)
	$effect(() => {
		if (totalEntradas >= 0 && isOpen && listaLog) {
			listaLog.scrollTop = listaLog.scrollHeight;
		}
	});
</script>

<footer
	class="col-span-2 flex items-center gap-2 border-t border-neutral-800 px-3 font-mono text-xs"
>
	<span>Log:</span>
	<div class="w-full truncate">
		{#if $animatedLog.entry}
			<span class="truncate {$animatedLog.entry ? levelClass[$animatedLog.entry.level] : ''}">
				[{fmt($animatedLog.entry.timestamp)}] [{levelLabel[$animatedLog.entry.level]}] [{$animatedLog
					.entry.source}] {$animatedLog.text}<span class="animate-pulse">|</span>
			</span>
		{:else}
			<span class="text-neutral-600">Sin eventos</span>
		{/if}
	</div>
	<button
		type="button"
		onclick={() => (isOpen = true)}
		title="Ver log completo"
		aria-label="Ver log completo"
		class="flex shrink-0 cursor-pointer items-center text-neutral-400 hover:border-neutral-700 hover:text-neutral-100"
	>
		<IconWindowMaximize size={18} />
	</button>
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
