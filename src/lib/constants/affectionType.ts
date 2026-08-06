// CATÁLOGO REAL DE TIPO DE AFECTACIÓN EN VENTAS SEGÚN SUNAT
export const TIPOS_AFECTACION_VENTAS = [
	{ id: '10', value: '10', label: 'Gravado - Op. Onerosa (IGV 18%)' },
	{ id: '20', value: '20', label: 'Exonerado - Op. Onerosa (IGV 0%)' },
	{ id: '30', value: '30', label: 'Inafecto - Op. Onerosa (IGV 0%)' },
	{ id: '11', value: '11', label: 'Gravado - Retiro (Transferencia Gratuita)' },
	{ id: '31', value: '31', label: 'Inafecto - Retiro (Transferencia Gratuita)' }
];

// CATÁLOGO REAL DE TIPO DE AFECTACIÓN EN COMPRAS
export const DESTINO_AFECTACION_COMPRAS = [
	{
		value: 'GRAVADO_V_GRAVADO',
		label: 'Adq. Gravada - Destinada a Ventas Gravadas (Crédito Fiscal)'
	},
	{
		value: 'GRAVADO_V_MIXTO',
		label: 'Adq. Gravada - Destinada a Ventas Gravadas y No Gravadas'
	},
	{
		value: 'GRAVADO_V_NOGRAVADO',
		label: 'Adq. Gravada - Destinada a Ventas No Gravadas (Costo/Gasto)'
	},
	{ value: 'NO_GRAVADO', label: 'Adq. No Gravada - (Exonerada / Inafecta / Sin IGV)' }
];
