import { invoke } from '@tauri-apps/api/core';
import { get } from 'svelte/store';
import { configStore } from '$lib/stores/config';

/**
 * Clase base con la infraestructura compartida para comunicarse con el backend.
 * Cada dominio (clientes, productos, vendedores) extiende esta clase y aporta
 * sus propios métodos y tipos.
 */
export class ApiClient {
	private async getBaseUrl(): Promise<string> {
		const config = get(configStore);
		if (!config.server_ip) {
			throw new Error('Dirección IP del servidor no configurada para el nodo cliente.');
		}
		// Asegurar que el protocolo esté presente
		let ip = config.server_ip;
		if (!ip.startsWith('http://') && !ip.startsWith('https://')) {
			ip = `http://${ip}`;
		}
		return ip;
	}

	/**
	 * Ejecuta una petición según el rol del nodo:
	 * - 'server': ejecución local vía Tauri IPC (invoke).
	 * - 'client': ejecución remota vía HTTP (Axum).
	 *
	 * @param endpoint Ruta HTTP (solo usada en modo cliente).
	 * @param tauriCommand Nombre del comando Tauri (solo usado en modo servidor).
	 * @param options Opciones adicionales de fetch.
	 * @param payload Cuerpo de la petición (se envía como `payload` al invoke o como JSON por HTTP).
	 * @param extraInvokeArgs Argumentos extra que se pasan al invoke (p. ej. `{ id }`).
	 */
	protected async request<T>(
		endpoint: string,
		tauriCommand: string,
		options: RequestInit = {},
		payload?: unknown,
		extraInvokeArgs?: Record<string, unknown>
	): Promise<T> {
		let config = get(configStore);

		// Si el rol aún no se cargó (carrera al iniciar la app), forzar una reinicialización
		if (!config.role) {
			await configStore.init();
			config = get(configStore);
		}

		if (config.role === 'server') {
			// Ejecución local vía Tauri IPC
			const invokeArgs: Record<string, unknown> = { ...extraInvokeArgs };
			if (payload !== undefined) {
				invokeArgs.payload = payload;
			}
			if (Object.keys(invokeArgs).length > 0) {
				return invoke<T>(tauriCommand, invokeArgs);
			}
			return invoke<T>(tauriCommand);
		} else if (config.role === 'client') {
			// Ejecución remota vía HTTP (Axum)
			const baseUrl = await this.getBaseUrl();

			// Formatear la URL correctamente para evitar dobles barras
			const cleanEndpoint = endpoint.startsWith('/') ? endpoint.slice(1) : endpoint;
			const url = `${baseUrl}/${cleanEndpoint}`;

			const headers = new Headers(options.headers);
			headers.set('Content-Type', 'application/json');

			const fetchOptions: RequestInit = {
				...options,
				headers,
			};

			if (payload !== undefined) {
				fetchOptions.body = JSON.stringify(payload);
			}

			const response = await fetch(url, fetchOptions);

			if (!response.ok) {
				const text = await response.text();
				throw new Error(`API Error (${response.status}): ${text}`);
			}

			// Si es un DELETE o 204 No Content, devolver null como T
			if (response.status === 204) {
				return null as T;
			}

			return response.json();
		} else {
			throw new Error('Rol de nodo no configurado.');
		}
	}
}
