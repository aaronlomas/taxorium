import type { ComponentType } from 'svelte';
import {
	IconLayoutDashboard,
	IconReportMoney,
	IconShoppingBag,
	IconPackage,
	IconReport,
	IconUsers,
	IconReceipt,
	IconFileInvoice
} from '@tabler/icons-svelte';

export interface SubItemNavegacion {
	nombre: string;
	ruta: string;
}

export interface ElementoNavegacion {
	nombre: string;
	icono?: ComponentType;
	ruta?: string;
	subitems?: SubItemNavegacion[];
	accesoRequerido?: string;
}

// Opciones para el panel de Registros
export const navegacionRegistros: ElementoNavegacion[] = [
	{ nombre: 'Dashboard', icono: IconLayoutDashboard, ruta: '/dashboard' },
	{
		nombre: 'Comprobantes',
		icono: IconReceipt,
		accesoRequerido: 'sales',
		subitems: [
			{ nombre: 'Boleta de Venta', ruta: '/sales/vouchers/boleta' },
			{ nombre: 'Factura', ruta: '/sales/vouchers/factura' },
			{ nombre: 'Envíos SUNAT', ruta: '/sales/summary' }
		]
	},
	{ nombre: 'Productos', icono: IconPackage, ruta: '/productos', accesoRequerido: 'products' },
	{ nombre: 'Reportes', icono: IconReport, ruta: '/reportes', accesoRequerido: 'sales' },
	{ nombre: 'Clientes', icono: IconUsers, ruta: '/customers', accesoRequerido: 'sales' },
	{ nombre: 'Diseñador de PDF', icono: IconFileInvoice, ruta: '/print/designer' }
];

// Opciones para el panel de Operaciones
export const navegacionOperaciones: ElementoNavegacion[] = [
	{
		nombre: 'Ventas',
		icono: IconReportMoney,
		ruta: '/sales/panel',
		accesoRequerido: 'sales'
	},
	{
		nombre: 'Compras',
		icono: IconShoppingBag,
		ruta: '/sales/receipt/new',
		accesoRequerido: 'shop'
	}
];
