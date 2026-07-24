import type { ComponentType } from 'svelte';

import DashboardView from './dashboard/Dashboard.svelte';
import ProductsView from './products/Products.svelte';

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
export const viewRoutes: Record<string, ComponentType> = {
	Dashboard: DashboardView,
	Productos: ProductsView
};

/** Devuelve el componente para un id de pestaña, o null si no existe */
export function resolveView(tabId: string): ComponentType | null {
	return viewRoutes[tabId] ?? null;
}
