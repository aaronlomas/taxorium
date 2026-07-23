<script lang="ts">
	let {
		ruc,
		razonSocial,
		nombreComercial,
		departamento,
		direccion,
		telefono,
		email,
		serieBoleta,
		serieFactura
	}: {
		ruc: string;
		razonSocial: string;
		nombreComercial: string;
		departamento: string;
		direccion: string;
		telefono: string;
		email: string;
		serieBoleta: string;
		serieFactura: string;
	} = $props();
</script>

<div class="mb-6">
	<h2 class="text-xl font-bold text-slate-100 mb-1 tracking-tight">Confirma tu configuración</h2>
	<p class="text-sm text-slate-400 mb-7 leading-relaxed">Revisa los datos antes de guardar. Podrás editarlos después desde Configuración.</p>

	<div class="flex flex-col gap-5">
		<div class="bg-white/5 border border-white/10 rounded-xl p-5">
			<h3 class="text-xs font-semibold text-slate-400 mb-3.5 uppercase tracking-wider">🏢 Empresa</h3>
			<div class="flex justify-between items-center py-1.5 border-b border-white/5 text-sm"><span class="text-slate-400">RUC</span><strong class="text-slate-200 text-right">{ruc}</strong></div>
			<div class="flex justify-between items-center py-1.5 border-b border-white/5 text-sm"><span class="text-slate-400">Razón social</span><strong class="text-slate-200 text-right">{razonSocial}</strong></div>
			{#if nombreComercial}<div class="flex justify-between items-center py-1.5 border-b border-white/5 text-sm"><span class="text-slate-400">Nombre comercial</span><strong class="text-slate-200 text-right">{nombreComercial}</strong></div>{/if}
			<div class="flex justify-between items-center py-1.5 border-b border-white/5 text-sm"><span class="text-slate-400">Dirección</span><strong class="text-slate-200 text-right">{direccion}</strong></div>
			{#if departamento}<div class="flex justify-between items-center py-1.5 border-b border-white/5 text-sm"><span class="text-slate-400">Departamento</span><strong class="text-slate-200 text-right">{departamento}</strong></div>{/if}
			{#if telefono}<div class="flex justify-between items-center py-1.5 border-b border-white/5 text-sm"><span class="text-slate-400">Teléfono</span><strong class="text-slate-200 text-right">{telefono}</strong></div>{/if}
			{#if email}<div class="flex justify-between items-center py-1.5 text-sm"><span class="text-slate-400">Email</span><strong class="text-slate-200 text-right">{email}</strong></div>{/if}
		</div>

		<div class="bg-white/5 border border-white/10 rounded-xl p-5">
			<h3 class="text-xs font-semibold text-slate-400 mb-3.5 uppercase tracking-wider">📄 Series</h3>
			<div class="flex justify-between items-center py-1.5 border-b border-white/5 text-sm"><span class="text-slate-400">Boletas</span><strong class="text-slate-200 text-right">{serieBoleta}-00000001</strong></div>
			<div class="flex justify-between items-center py-1.5 text-sm"><span class="text-slate-400">Facturas</span><strong class="text-slate-200 text-right">{serieFactura}-00000001</strong></div>
		</div>
	</div>
</div>
