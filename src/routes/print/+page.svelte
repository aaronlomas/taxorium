<script lang="ts">
	import { onMount } from 'svelte';
	import InvoiceTemplate from '$lib/components/ui/print/InvoiceTemplate.svelte';
	import type { PrintableInvoiceData } from '$lib/components/ui/print/invoiceTypes';

	let data = $state<PrintableInvoiceData | null>(null);

	onMount(() => {
		const raw = localStorage.getItem('taxorium_print_data');
		if (raw) {
			try {
				data = JSON.parse(raw) as PrintableInvoiceData;
			} catch (e) {
				document.title = 'Error: datos inválidos';
			}
		}
	});

	function imprimir() {
		window.print();
	}
</script>

<svelte:head>
	<title>Comprobante — Taxorium</title>
	<style>
		/* Barra de acciones — solo visible en pantalla, nunca en el PDF */
		.print-toolbar {
			position: sticky;
			top: 0;
			z-index: 50;
			display: flex;
			align-items: center;
			justify-content: space-between;
			gap: 12px;
			padding: 10px 16px;
			background: #1a1a2e;
			border-bottom: 1px solid #2d2d44;
			box-shadow: 0 2px 8px rgba(0,0,0,0.4);
			font-family: 'Inter', sans-serif;
		}

		.print-toolbar .brand {
			color: #a78bfa;
			font-size: 13px;
			font-weight: 600;
			letter-spacing: 0.05em;
		}

		.print-toolbar .btn-print {
			display: flex;
			align-items: center;
			gap: 6px;
			padding: 7px 18px;
			background: linear-gradient(135deg, #7c3aed, #4f46e5);
			color: white;
			font-size: 13px;
			font-weight: 600;
			border: none;
			border-radius: 8px;
			cursor: pointer;
			transition: opacity 0.15s;
		}
		.print-toolbar .btn-print:hover { opacity: 0.88; }

		.print-toolbar .hint {
			font-size: 11px;
			color: #6b7280;
		}

		/* Al imprimir: ocultar toolbar y mostrar solo el comprobante */
		@media print {
			.print-toolbar {
				display: none !important;
			}
			body {
				margin: 0;
				padding: 0;
				background: white;
			}
		}
	</style>
</svelte:head>

<!-- Barra de acciones superior -->
<div class="print-toolbar">
	<span class="brand">⚡ Taxorium — Vista previa del comprobante</span>
	<div style="display:flex; align-items:center; gap:10px;">
		<span class="hint">Elige «Guardar como PDF» en el diálogo para exportar</span>
		<button class="btn-print" onclick={imprimir}>
			🖨️ Imprimir / Guardar PDF
		</button>
	</div>
</div>

{#if data}
	<InvoiceTemplate {data} />
{:else}
	<p style="padding:2rem; text-align:center; color:#6b7280;">Cargando comprobante…</p>
{/if}
