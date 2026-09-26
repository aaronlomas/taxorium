import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';

export type LogLevel = 'error' | 'warn' | 'info';

export interface LogEntry {
	id: number;
	level: LogLevel;
	message: string;
	source: string;
	timestamp: Date;
}

let _idCounter = 0;

function createTaxoLogStore() {
	const { subscribe, update } = writable<LogEntry[]>([]);

	function push(level: LogLevel, message: string, source = 'app') {
		update((entries) => [
			...entries,
			{ id: _idCounter++, level, message, source, timestamp: new Date() }
		]);
	}

	/** Start listening for Tauri backend log events */
	async function initListener() {
		if (typeof window === 'undefined' || (!window.__TAURI_INTERNALS__ && !window.__TAURI__)) return;
		await listen<{ level: string; message: string; source: string }>('taxo://log', (event) => {
			const lvl = (event.payload.level as LogLevel) ?? 'info';
			push(lvl, event.payload.message, event.payload.source ?? 'backend');
		});
	}

	return {
		subscribe,
		info: (msg: string, src?: string) => push('info', msg, src),
		warn: (msg: string, src?: string) => push('warn', msg, src),
		error: (msg: string, src?: string) => push('error', msg, src),
		initListener
	};
}

export const taxoLog = createTaxoLogStore();
