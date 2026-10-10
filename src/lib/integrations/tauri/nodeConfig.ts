import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

export interface NodeConfig {
	role: 'server' | 'client' | null;
	server_ip: string | null;
}

export function createConfigStore() {
	const { subscribe, set } = writable<NodeConfig>({
		role: null,
		server_ip: null
	});

	let initialized = false;

	return {
		subscribe,
		init: async () => {
			if (initialized) return;
			try {
				if (window.__TAURI_INTERNALS__ || window.__TAURI__) {
					const config = await invoke<NodeConfig>('get_node_config');
					set(config);
					if (config.role) {
						initialized = true;
					}
				}
			} catch (e) {
				console.error('Failed to get node config:', e);
			}
		},
		setConfig: async (role: 'server' | 'client', server_ip: string | null = null) => {
			try {
				if (window.__TAURI_INTERNALS__ || window.__TAURI__) {
					await invoke('set_node_config', { role, serverIp: server_ip });
					set({ role, server_ip });
					initialized = true;
				}
			} catch (e) {
				console.error('Failed to save node config:', e);
				throw e;
			}
		},
		initServerDb: async () => {
			try {
				if (window.__TAURI_INTERNALS__ || window.__TAURI__) {
					await invoke('init_server_db');
				}
			} catch (e) {
				console.error('Failed to init server db:', e);
				throw e;
			}
		}
	};
}

export const configStore = createConfigStore();
