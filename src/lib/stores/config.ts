import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface NodeConfig {
	role: 'server' | 'client' | null;
	server_ip: string | null;
}

function createConfigStore() {
	const { subscribe, set, update } = writable<NodeConfig>({
		role: null,
		server_ip: null
	});

	let initialized = false;

	return {
		subscribe,
		init: async () => {
			if (initialized) return;
			try {
				if (window.__TAURI__) {
					const config = await invoke<NodeConfig>('get_node_config');
					set(config);
				}
			} catch (e) {
				console.error('Failed to get node config:', e);
			} finally {
				initialized = true;
			}
		},
		setConfig: async (role: 'server' | 'client', server_ip: string | null = null) => {
			try {
				if (window.__TAURI__) {
					await invoke('set_node_config', { role, serverIp: server_ip });
					set({ role, server_ip });
				}
			} catch (e) {
				console.error('Failed to save node config:', e);
				throw e;
			}
		},
		initServerDb: async () => {
			try {
				if (window.__TAURI__) {
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
