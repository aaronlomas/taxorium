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
