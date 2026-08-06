<script lang="ts">
	import { taxoLog, type LogEntry } from '$lib/stores/taxoLog';

	// Color classes per level — user controls layout, we control color only
	const levelClass: Record<LogEntry['level'], string> = {
		error: 'text-red-400',
		warn:  'text-yellow-400',
		info:  'text-neutral-400',
	};

	const levelLabel: Record<LogEntry['level'], string> = {
		error: 'ERR',
		warn:  'WARN',
		info:  'INFO',
	};

	function fmt(d: Date) {
		return d.toLocaleTimeString('es-PE', { hour12: false });
	}
</script>

<footer class="col-span-2 border-t border-neutral-800">
	<div>
		{#each $taxoLog as entry (entry.id)}
			<span class={levelClass[entry.level]}>
				[{fmt(entry.timestamp)}] [{levelLabel[entry.level]}] [{entry.source}] {entry.message}
			</span>
		{/each}
		{#if $taxoLog.length === 0}
			<span class="text-neutral-600">Sin eventos</span>
		{/if}
	</div>
</footer>