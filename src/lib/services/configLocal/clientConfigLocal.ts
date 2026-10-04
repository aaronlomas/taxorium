/**
 * Cliente de la configuración local del dispositivo.
 *
 * Estos valores (certificado_path, clave_cert) son intrínsecos de la máquina: la
 * ruta del .p12 y su contraseña solo existen en el equipo donde está el archivo.
 * Por eso van por Tauri IPC contra `device_config.json` y nunca por HTTP — si
 * se leyeran del servidor, un nodo cliente recibiría una ruta que no existe en
 * su disco y no podría firmar comprobantes.
 */

import { invoke } from '@tauri-apps/api/core';

export interface LocalConfig {
	certificado_path?: string;
	clave_cert?: string;
	[key: string]: string | undefined;
}

export const configLocalClient = {
	/** Obtiene toda la configuración local del dispositivo como clave-valor. */
	getAll(): Promise<LocalConfig> {
		return invoke<LocalConfig>('get_device_config');
	},

	/** Guarda o actualiza un valor en la configuración local del dispositivo. */
	set(clave: string, valor: string): Promise<void> {
		return invoke<void>('set_device_config', { clave, valor });
	}
};
