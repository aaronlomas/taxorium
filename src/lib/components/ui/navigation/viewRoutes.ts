import type { Component } from 'svelte';

import DashboardView from '../features/dashboard/Dashboard.svelte';
import ProductsView from '../features/products/ProductView.svelte';
import ReportsView from '../features/reports/ReportControls.svelte';
import CustomersView from '../features/customers/CustomerControls.svelte';

import PanelSalesView from '../features/sales/panel/SalesView.svelte';
import SalesReceiptView from '../features/sales/vouchers/SalesReceiptView.svelte';
import InvoiceView from '../features/sales/vouchers/InvoiceView.svelte';
import SalesSummaryView from '../features/sales/summary/SalesSummary.svelte';

import SettingsView from '../features/settings/SettingsView.svelte';
import DesignerPreviewView from '../features/print/DesignerPreviewView.svelte';

import ProfileView from '../features/account/ProfileView.svelte';
import SellerView from '../features/account/SellerView.svelte';

export const viewRoutes: Record<string, Component<any>> = {
	Dashboard: DashboardView,
	Productos: ProductsView,
	Reportes: ReportsView,
	Clientes: CustomersView,
	'Boleta de Venta': SalesReceiptView,
	Factura: InvoiceView,
	'Envíos SUNAT': SalesSummaryView,
	Ventas: PanelSalesView,
	Configuración: SettingsView,
	'Diseñador de PDF': DesignerPreviewView,
	'Mi Cuenta': ProfileView,
	'Mis Puntos de Venta': SellerView
};

export function resolveView(tabId: string): Component<any> | null {
	return viewRoutes[tabId] ?? null;
}
