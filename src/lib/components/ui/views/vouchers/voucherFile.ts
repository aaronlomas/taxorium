import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';

function isTauri(): boolean {
	return typeof window !== 'undefined' && !!(window.__TAURI_INTERNALS__ || window.__TAURI__);
}

/**
 * Guarda un archivo de comprobante usando el diálogo nativo del SO (Tauri).
 * En el navegador (desarrollo con `vite dev`) cae a una descarga normal.
 */
export async function saveVoucherFile(
	content: Uint8Array | string,
	defaultName: string,
	extension: 'pdf' | 'xml'
): Promise<string | null> {
	if (isTauri()) {
		try {
			const filePath = await save({
				defaultPath: defaultName,
				filters: [{ name: extension.toUpperCase(), extensions: [extension] }]
			});

			if (filePath) {
				const bytes = typeof content === 'string' ? new TextEncoder().encode(content) : content;
				await writeFile(filePath, bytes);
				return filePath;
			}
			return null;
		} catch (error) {
			console.error('Error al guardar comprobante nativo:', error);
			return null;
		}
	}

	const mime = extension === 'pdf' ? 'application/pdf' : 'application/xml';
	const blob =
		typeof content === 'string'
			? new Blob([content], { type: 'text/xml' })
			: new Blob([content as unknown as BlobPart], { type: mime });
	const url = URL.createObjectURL(blob);
	const anchor = document.createElement('a');
	anchor.href = url;
	anchor.download = defaultName;
	document.body.appendChild(anchor);
	anchor.click();
	document.body.removeChild(anchor);
	URL.revokeObjectURL(url);
	return defaultName;
}
