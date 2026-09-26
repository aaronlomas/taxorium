/**
 * Configuración de la plantilla de comprobantes PDF.
 * Edita este archivo para personalizar el aspecto de las facturas y boletas
 * sin tocar la lógica de negocio.
 */

export type PaperFormat = 'a4' | 'ticket80mm';

export interface PdfTemplateConfig {
	/** Tamaño de papel: 'a4' (hoja completa oficial) o 'ticket80mm' (impresora térmica) */
	format: PaperFormat;

	/**
	 * Logo de la empresa en formato base64 (ej: 'data:image/png;base64,...').
	 * Si es null, se muestra solo texto.
	 */
	logoBase64: string | null;

	/**
	 * Dimensiones del logo (en mm).
	 * Solo relevante si logoBase64 no es null.
	 */
	logoWidth: number;
	logoHeight: number;

	/** Colores principales (hex, ej: '#1e3a5f') */
	colors: {
		/** Color del encabezado/banda superior */
		headerBg: string;
		/** Color del texto sobre el encabezado */
		headerText: string;
		/** Color de acento (líneas, totales, nro. comprobante) */
		accent: string;
		/** Color de fondo alterno en filas de la tabla */
		rowAlt: string;
		/** Color de texto principal */
		textPrimary: string;
		/** Color de texto secundario / etiquetas */
		textSecondary: string;
		/** Color del texto del total */
		totalText: string;
		/** Color de fondo del bloque de totales */
		totalBg: string;
	};

	/** Pie de página personalizado (máximo 2 líneas) */
	footerLines: [string?, string?];

	/**
	 * Mostrar sección de código QR de verificación SUNAT.
	 * (Actualmente dibuja un placeholder; conéctalo a tu hash/firma cuando esté disponible)
	 */
	showQr: boolean;

	/** Texto adicional sobre el código de barras / hash */
	hashLabel: string;

	/** Márgenes en mm (solo para A4) */
	margins: {
		top: number;
		right: number;
		bottom: number;
		left: number;
	};
}

/**
 * Configuración por defecto.
 * Cámbiala aquí para ajustar el diseño de todos los comprobantes.
 */
export const DEFAULT_PDF_CONFIG: PdfTemplateConfig = {
	format: 'a4',

	logoBase64: null,
	logoWidth: 40,
	logoHeight: 20,

	colors: {
		headerBg: '#1a2744',    // Azul marino oscuro
		headerText: '#ffffff',
		accent: '#2563eb',      // Azul brillante
		rowAlt: '#f1f5fb',      // Gris azulado muy claro
		textPrimary: '#111827',
		textSecondary: '#6b7280',
		totalText: '#ffffff',
		totalBg: '#1a2744'
	},

	footerLines: [
		'Representación impresa del Comprobante de Pago Electrónico',
		'Consulte su comprobante en: https://ww1.sunat.gob.pe/ol-ti-itconsultaunificadabem/consultaUnificadaBem.htm'
	],

	showQr: true,
	hashLabel: 'Hash de verificación:',

	margins: {
		top: 15,
		right: 15,
		bottom: 15,
		left: 15
	}
};

/**
 * Configuración para el formato ticket de 80 mm.
 * Más compacto y pensado para impresoras térmicas.
 */
export const TICKET_PDF_CONFIG: PdfTemplateConfig = {
	format: 'ticket80mm',

	logoBase64: null,
	logoWidth: 30,
	logoHeight: 15,

	colors: {
		headerBg: '#1a2744',
		headerText: '#ffffff',
		accent: '#2563eb',
		rowAlt: '#f3f4f6',
		textPrimary: '#111827',
		textSecondary: '#6b7280',
		totalText: '#ffffff',
		totalBg: '#1a2744'
	},

	footerLines: [
		'Gracias por su compra',
		'Comprobante electrónico válido'
	],

	showQr: false,
	hashLabel: 'Hash:',

	margins: {
		top: 4,
		right: 3,
		bottom: 4,
		left: 3
	}
};
