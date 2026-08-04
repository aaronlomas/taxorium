import type { Component } from 'svelte';

import DashboardView from './dashboard/Dashboard.svelte';
import ProductsView from './products/ProductControls.svelte';
import ReportsView from './reports/ReportControls.svelte';
import CustomersView from './customers/CustomerControls.svelte';
// SUB OPCIONES EN VENTAS
import PanelSalesView from './sales/panel/SalesControls.svelte';
import ComprobantesSalesView from './sales/vouchers/VoucherList.svelte';
import SalesSummaryView from './sales/summary/SalesSummary.svelte';

// CONFIGURACIÓN
import SettingsView from './settings/SettingsView.svelte';
import CuentaView from './account/accountView.svelte';

/**
 * Mapa central de rutas de vistas de la aplicación.
 * Cada clave corresponde al `id` de una pestaña (el mismo valor
 * que se usa en `elementosNavegacion[].nombre` del Sidebar).
 *
 * Para agregar una nueva vista:
 *   1. Crea el componente en views/<nombre>/
 *   2. Importalo aquí
 *   3. Añade la entrada al mapa
 */
export const viewRoutes: Record<string, Component<any>> = {

	//SECCIÓN DE REGISTROS GENERALES
	Dashboard: DashboardView,
	Productos: ProductsView,
	Reportes: ReportsView,
	Clientes: CustomersView,
	// Sub Opciones de Registros/ventas/
	'Lista de Comprobantes': ComprobantesSalesView,
	'Lista de Resúmenes': SalesSummaryView,

	//SECCIÓN DE OPERACIONES
	'Panel de Ventas': PanelSalesView,

	//CONFIGURACIÓN
	'Configuración': SettingsView,
	'Panel de Cuenta': CuentaView,
};

/** Devuelve el componente para un id de pestaña, o null si no existe */
export function resolveView(tabId: string): Component<any> | null {
	return viewRoutes[tabId] ?? null;
}
