import type { ComponentType } from 'svelte';
import {
  IconLayoutDashboard,
  IconReportMoney,
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
}

// Opciones para el panel de Registros
export const navegacionRegistros: ElementoNavegacion[] = [
  { nombre: 'Dashboard', icono: IconLayoutDashboard, ruta: '/dashboard' },
  {
    nombre: 'Ventas',
    icono: IconReportMoney,
    subitems: [
      { nombre: 'Panel de Ventas', ruta: '/sales/panel' },
      { nombre: 'Lista de Comprobantes', ruta: '/sales/vouchers' }
    ]
  },
  { nombre: 'Productos', icono: IconPackage, ruta: '/productos' },
  { nombre: 'Reportes', icono: IconReport, ruta: '/reportes' },
  { nombre: 'Clientes', icono: IconUsers, ruta: '/customers' }
];

// Opciones para el panel de Venta Rápida
export const navegacionVentas: ElementoNavegacion[] = [
  { nombre: 'Nueva Factura', icono: IconFileInvoice, ruta: '/sales/invoice/new' },
  { nombre: 'Nueva Boleta', icono: IconReceipt, ruta: '/sales/receipt/new' }
];