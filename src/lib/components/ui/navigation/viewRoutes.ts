import type { Component } from 'svelte';

import DashboardView from '../features/dashboard/Dashboard.svelte';
import ProductsView from '../features/products/ProductsView.svelte';
import ReportsView from '../features/reports/ReportControls.svelte';
import CustomersView from '../features/customers/CustomersView.svelte';

import PanelSalesView from '../features/sales/panel/SalesView.svelte';
import SalesReceiptView from '../features/sales/vouchers/components/SalesReceiptView.svelte';
import InvoiceView from '../features/sales/vouchers/components/InvoiceView.svelte';
import SalesSummaryView from '../features/sales/summary/SalesSummary.svelte';
import PanelPurchaseView from '../features/purchase/panel/PurchaseView.svelte';

import SettingsView from '../features/settings/SettingsView.svelte';
import DesignerPreviewView from '../features/print/DesignerPreviewView.svelte';

import ProfileView from '../features/account/ProfileView.svelte';
import SellerView from '../features/account/SellerView.svelte';
import PurchaseRecords from '../features/purchase/records/PurchaseRecords.svelte';
import VendorsView from '../features/vendors/VendorsView.svelte';

export const viewRoutes: Record<string, Component<any>> = {
	//MENUS---------------------------------
	Dashboard: DashboardView,
	Ventas: PanelSalesView,
	Productos: ProductsView,
	Reportes: ReportsView,
	Clientes: CustomersView,
	Proveedores: VendorsView,
	'Diseñador de PDF': DesignerPreviewView,
	//---------------------------------------
	/*SUB-MENUS*/
	//ventas
	'Boleta de Venta': SalesReceiptView,
	Factura: InvoiceView,
	'Envíos SUNAT': SalesSummaryView,
	//---------------------------------------
	//compras
	Panel: PanelPurchaseView,
	Registros: PurchaseRecords,
	//---------------------------------------
	//configuracion
	Configuración: SettingsView,
	'Mi Cuenta': ProfileView,
	'Mis Puntos de Venta': SellerView
};

export function resolveView(tabId: string): Component<any> | null {
	return viewRoutes[tabId] ?? null;
}
