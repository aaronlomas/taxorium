import type { Component } from 'svelte';

import DashboardView from './dashboard/Dashboard.svelte';
import ProductsView from './products/ProductControls.svelte';
import ReportsView from './reports/ReportControls.svelte';
import VentasView from './sales/SalesControls.svelte';
import CustomersView from './customers/CustomerControls.svelte';

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
	Dashboard: DashboardView,
	Productos: ProductsView,
	Reportes: ReportsView,
	Ventas: VentasView,
	Clientes: CustomersView
};

/** Devuelve el componente para un id de pestaña, o null si no existe */
export function resolveView(tabId: string): Component<any> | null {
	return viewRoutes[tabId] ?? null;
}
