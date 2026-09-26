import * as XLSX from 'xlsx';
import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';

export type ExportFormat = 'xlsx' | 'csv-comma' | 'csv-semicolon';

export interface ColumnDefinition {
	key: string;
	label: string;
}

/**
 * Convierte un arreglo de objetos a CSV.
 * @param data Datos a exportar
 * @param columns Definición de columnas a incluir
 * @param separator Separador de columnas (',' o ';')
 */
function generateCSV(data: any[], columns: ColumnDefinition[], separator: string): string {
	const headers = columns.map((col) => `"${col.label.replace(/"/g, '""')}"`).join(separator);

	const rows = data.map((row) => {
		return columns
			.map((col) => {
				let val = row[col.key];
				if (val === null || val === undefined) {
					val = '';
				} else if (typeof val === 'string') {
					val = val.trim();
				}
				const str = String(val).replace(/"/g, '""');
				return `"${str}"`;
			})
			.join(separator);
	});

	return [headers, ...rows].join('\n');
}

/**
 * Guarda el archivo utilizando el diálogo nativo del sistema operativo.
 * Retorna true si se guardó, false si el usuario canceló.
 */
async function saveFileNative(
	content: Uint8Array,
	defaultPath: string,
	filters: { name: string; extensions: string[] }[]
): Promise<string | null> {
	try {
		const filePath = await save({
			defaultPath,
			filters
		});

		if (filePath) {
			await writeFile(filePath, content);
			return filePath;
		}
		return null;
	} catch (error) {
		console.error('Error al guardar archivo nativo:', error);
		return null;
	}
}

/**
 * Exporta datos al formato especificado y permite al usuario guardarlo en su sistema.
 */
export async function exportData(
	data: any[],
	columns: ColumnDefinition[],
	filename: string,
	format: ExportFormat
): Promise<string | null> {
	if (data.length === 0) return null;

	if (format === 'csv-comma' || format === 'csv-semicolon') {
		const separator = format === 'csv-comma' ? ',' : ';';
		const csv = generateCSV(data, columns, separator);

		const textEncoder = new TextEncoder();
		const csvBuffer = textEncoder.encode(csv);

		// Agregar BOM para que Excel lea correctamente el UTF-8 en CSV
		const contentWithBOM = new Uint8Array(3 + csvBuffer.length);
		contentWithBOM.set([0xef, 0xbb, 0xbf], 0);
		contentWithBOM.set(csvBuffer, 3);

		return await saveFileNative(contentWithBOM, `${filename}.csv`, [
			{ name: 'CSV', extensions: ['csv'] }
		]);
	} else if (format === 'xlsx') {
		// Preparar datos para SheetJS usando las columnas definidas
		const exportDataArray = data.map((row) => {
			const newRow: Record<string, any> = {};
			columns.forEach((col) => {
				let val = row[col.key];
				if (typeof val === 'string') {
					val = val.trim();
				}
				newRow[col.label] = val;
			});
			return newRow;
		});

		const worksheet = XLSX.utils.json_to_sheet(exportDataArray);
		const workbook = XLSX.utils.book_new();
		XLSX.utils.book_append_sheet(workbook, worksheet, 'Datos');

		const excelBuffer = XLSX.write(workbook, { bookType: 'xlsx', type: 'array' });

		return await saveFileNative(new Uint8Array(excelBuffer), `${filename}.xlsx`, [
			{ name: 'Excel', extensions: ['xlsx'] }
		]);
	}

	return null;
}
