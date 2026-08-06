<script lang="ts">
	import { createTypewriterLog } from './utilities/typewriter.ts';
	import type { LogEntry } from '$lib/stores/taxoLog';

	// Instanciamos el log animado con velocidad opcional (por defecto 25ms por carácter)
	const animatedLog = createTypewriterLog(20);

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

	function fmt(d: Date) {
		return d.toLocaleTimeString('es-PE', { hour12: false });
	}
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
</footer>
