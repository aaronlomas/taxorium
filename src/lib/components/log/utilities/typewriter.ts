// $lib/utils/typewriter.ts
import { writable, type Readable } from 'svelte/store';
import { taxoLog, type LogEntry } from '$lib/features/taxoLog';

export interface AnimatedLog {
	entry: LogEntry | null;
	text: string;
}

export function createTypewriterLog(speedMs: number = 25): Readable<AnimatedLog> {
	const { subscribe, set } = writable<AnimatedLog>({ entry: null, text: '' });

	let timer: ReturnType<typeof setInterval> | null = null;
	let lastHandledId: string | number | null = null;

	taxoLog.subscribe((logs) => {
		if (logs.length === 0) {
			if (timer) clearInterval(timer);
			set({ entry: null, text: '' });
			lastHandledId = null;
			return;
		}

		// Tomar únicamente el último log para no romper la barra de 40px
		const latest = logs[logs.length - 1];

		// Evitar reiniciar el efecto si el último mensaje ya es el actual
		if (latest.id === lastHandledId) return;
		lastHandledId = latest.id;

		if (timer) clearInterval(timer);

		let charIndex = 0;
		const fullText = latest.message;

		// Iniciar con texto vacío y comenzar la animación carácter por carácter
		set({ entry: latest, text: '' });

		timer = setInterval(() => {
			charIndex++;
			set({ entry: latest, text: fullText.slice(0, charIndex) });

			if (charIndex >= fullText.length) {
				if (timer) clearInterval(timer);
			}
		}, speedMs);
	});

	return { subscribe };
}
