import { writable } from 'svelte/store';
import { configLocalClient } from '$lib/integrations/tauri/deviceConfig';

/**
 * Tarifa del ICBPER por bolsa plástica. La ley la mantiene en S/ 0.50 por unidad
 * desde 2019, pero se expone como configuración editable (Ajustes) para no
 * recompilar si cambia.
 */
export const ICBPER_TASA_DEFAULT = 0.5;

const CLAVE = 'icbper_tasa';

function createIcbperStore() {
	const { subscribe, set } = writable<number>(ICBPER_TASA_DEFAULT);

	return {
		subscribe,

		/** Lee la tarifa guardada en SQLite local; si no existe o es inválida usa 0.50. */
		async load() {
			try {
				const cfg = await configLocalClient.getAll();
				const tasa = Number(cfg[CLAVE]);
				if (Number.isFinite(tasa) && tasa >= 0) set(tasa);
			} catch {
				// Sin Tauri (p. ej. build web) o sin config: se mantiene el default.
			}
		},

		/** Guarda la tarifa en SQLite local y actualiza el store. */
		async setTasa(tasa: number) {
			const valor = Number.isFinite(tasa) && tasa >= 0 ? tasa : ICBPER_TASA_DEFAULT;
			set(valor);
			try {
				await configLocalClient.set(CLAVE, String(valor));
			} catch (e) {
				console.warn('No se pudo guardar la tarifa ICBPER:', e);
			}
		}
	};
}

export const icbperStore = createIcbperStore();
