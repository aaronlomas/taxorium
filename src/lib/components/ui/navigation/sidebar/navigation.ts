import type { ComponentType } from 'svelte';
import {
	IconLayoutDashboard,
	IconReportMoney,
	IconShoppingBag,
	IconPackage,
	IconReport,
	IconUsers,
	IconReceipt,
	IconFileInvoice,
	IconBuildingEstate
} from '@tabler/icons-svelte';

export interface SubItemNavegacion {
	nombre: string;
}

export interface ElementoNavegacion {
	nombre: string;
	icono?: ComponentType;
	subitems?: SubItemNavegacion[];
	accesoRequerido?: string;
}

// Opciones para el panel de Registros
export const navegacionRegistros: ElementoNavegacion[] = [
	{ nombre: 'Dashboard', icono: IconLayoutDashboard },
	{
		nombre: 'Comprobantes',
		icono: IconReceipt,
		accesoRequerido: 'sales',
		subitems: [{ nombre: 'Boleta de Venta' }, { nombre: 'Factura' }, { nombre: 'Envíos SUNAT' }]
	},
	{ nombre: 'Productos', icono: IconPackage, accesoRequerido: 'products' },
	{ nombre: 'Reportes', icono: IconReport, accesoRequerido: 'sales' },
	{ nombre: 'Clientes', icono: IconUsers, accesoRequerido: 'customers' },
	{ nombre: 'Proveedores', icono: IconBuildingEstate, accesoRequerido: 'vendors' },
	{ nombre: 'Diseñador de PDF', icono: IconFileInvoice }
];

// Opciones para el panel de Operaciones
export const navegacionOperaciones: ElementoNavegacion[] = [
	{
		nombre: 'Ventas',
		icono: IconReportMoney,
		accesoRequerido: 'sales'
	},
	{
		nombre: 'Compras',
		icono: IconShoppingBag,
		accesoRequerido: 'shop',
		subitems: [{ nombre: 'Panel' }, { nombre: 'Registros' }]
	}
];
