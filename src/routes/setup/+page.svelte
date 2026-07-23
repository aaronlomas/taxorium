<script lang="ts">
	import { goto } from '$app/navigation';
	import { auth } from '$lib/stores/auth';
	import { tenantStore } from '$lib/stores/tenant';
	import Button from '$lib/components/core/primitives/Button.svelte';
	
	import StepEmpresa from '$lib/components/setups/StepEmpresa.svelte';
	import StepSeries from '$lib/components/setups/StepSeries.svelte';
	import StepConfirmar from '$lib/components/setups/StepConfirmar.svelte';
	import Stepper from '$lib/components/setups/Stepper.svelte';

	type Step = 'empresa' | 'series' | 'confirmar';
	let currentStep = $state<Step>('empresa');

	let ruc = $state('');
	let razonSocial = $state('');
	let nombreComercial = $state('');
	let direccion = $state('');
	let departamento = $state('');
	let telefono = $state('');
	let email = $state('');
	let serieBoleta = $state('B001');
	let serieFactura = $state('F001');

	let loading = $state(false);
	let errorMsg = $state('');
	let rucValidating = $state(false);
	let rucValid = $state<boolean | null>(null);

	const steps: { id: Step; label: string; icon: string }[] = [
		{ id: 'empresa', label: 'Empresa', icon: '🏢' },
		{ id: 'series', label: 'Series', icon: '📄' },
		{ id: 'confirmar', label: 'Confirmar', icon: '✅' }
	];

	function isValidRuc(r: string): boolean {
		return /^(10|20)\d{9}$/.test(r);
	}

	async function validateRuc() {
		if (!isValidRuc(ruc)) {
			rucValid = false;
			return;
		}
		rucValidating = true;
		await new Promise((r) => setTimeout(r, 500));
		rucValid = true;
		rucValidating = false;
	}

	function nextStep() {
		errorMsg = '';
		if (currentStep === 'empresa') {
			if (!ruc || !isValidRuc(ruc)) { errorMsg = 'Ingresa un RUC válido (11 dígitos).'; return; }
			if (!razonSocial.trim()) { errorMsg = 'La razón social es obligatoria.'; return; }
			if (!direccion.trim()) { errorMsg = 'La dirección fiscal es obligatoria.'; return; }
			currentStep = 'series';
		} else if (currentStep === 'series') {
			if (!/^[BF]\d{3}$/.test(serieBoleta)) { errorMsg = 'Serie de boleta inválida (ej: B001).'; return; }
			if (!/^[F]\d{3}$/.test(serieFactura)) { errorMsg = 'Serie de factura inválida (ej: F001).'; return; }
			currentStep = 'confirmar';
		}
	}

	function prevStep() {
		errorMsg = '';
		if (currentStep === 'series') currentStep = 'empresa';
		else if (currentStep === 'confirmar') currentStep = 'series';
	}

	async function handleFinish() {
		errorMsg = '';
		loading = true;
		const user = $auth.user;
		if (!user) { errorMsg = 'Sesión no encontrada. Vuelve a iniciar sesión.'; loading = false; return; }

		try {
			const tenant = await tenantStore.create(user.id, {
				user_id: user.id,
				ruc,
				razon_social: razonSocial.trim(),
				nombre_comercial: nombreComercial.trim() || null,
				direccion: direccion.trim(),
				departamento: departamento.trim() || null,
				telefono: telefono.trim() || null,
				email: email.trim() || null,
				serie_boleta: serieBoleta.toUpperCase(),
				serie_factura: serieFactura.toUpperCase(),
				configurado: true,
				activo: true
			});
			if (tenant) {
				await tenantStore.markConfigured(tenant.id);
			}
			goto('/');
		} catch (err: unknown) {
			const msg = err instanceof Error ? err.message : 'Error al guardar';
			if (msg.includes('duplicate') || msg.includes('unique')) {
				errorMsg = 'Ya existe una empresa registrada con ese RUC.';
			} else {
				errorMsg = msg;
			}
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Taxorium — Configuración Inicial</title>
</svelte:head>

<div class="min-h-screen flex items-center justify-center p-4 sm:p-8 relative overflow-hidden bg-slate-950 text-slate-200">
	<div class="fixed inset-0 pointer-events-none z-0" style="background-image: linear-gradient(rgba(59, 130, 246, 0.05) 1px, transparent 1px), linear-gradient(90deg, rgba(59, 130, 246, 0.05) 1px, transparent 1px); background-size: 40px 40px;"></div>

	<div class="w-full max-w-2xl relative z-10">
		<div class="text-center mb-8">
			<h1 class="text-3xl font-extrabold text-slate-100 mb-2 tracking-tight">Configuración inicial</h1>
			<p class="text-slate-400 text-sm">Configura tu empresa para empezar a emitir comprobantes electrónicos</p>
		</div>

		<Stepper bind:currentStep={currentStep} {steps} />

		<div class="bg-white/5 border border-white/10 rounded-2xl p-6 sm:p-10 backdrop-blur-xl shadow-2xl">
			{#if currentStep === 'empresa'}
				<StepEmpresa
					bind:ruc
					bind:razonSocial
					bind:nombreComercial
					bind:departamento
					bind:direccion
					bind:telefono
					bind:email
					bind:rucValid
					bind:rucValidating
					{validateRuc}
				/>
			{:else if currentStep === 'series'}
				<StepSeries
					bind:serieBoleta
					bind:serieFactura
				/>
			{:else if currentStep === 'confirmar'}
				<StepConfirmar
					{ruc}
					{razonSocial}
					{nombreComercial}
					{departamento}
					{direccion}
					{telefono}
					{email}
					{serieBoleta}
					{serieFactura}
				/>
			{/if}

			{#if errorMsg}
				<div class="flex items-center gap-2 bg-red-500/10 border border-red-500/20 rounded-xl px-4 py-3 text-red-400 text-sm mb-4" role="alert">
					<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
						<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5"/>
						<path d="M8 5v4M8 11v.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
					</svg>
					{errorMsg}
				</div>
			{/if}

			<div class="flex justify-between items-center mt-2 pt-6 border-t border-white/10">
				{#if currentStep !== 'empresa'}
					<Button type="button" variant="secondary" onclick={prevStep} disabled={loading} class="bg-white/5 text-slate-400 border border-white/10 hover:bg-white/10 hover:text-slate-200">
						← Anterior
					</Button>
				{:else}
					<div></div>
				{/if}

				{#if currentStep !== 'confirmar'}
					<Button type="button" variant="primary" onclick={nextStep} class="shadow-lg shadow-blue-500/30 hover:-translate-y-1 hover:shadow-xl hover:shadow-blue-500/40 transition-all duration-200">
						Siguiente →
					</Button>
				{:else}
					<Button type="button" variant="primary" onclick={handleFinish} disabled={loading} double={true} class="px-8 shadow-lg shadow-blue-500/30 hover:-translate-y-1 hover:shadow-xl hover:shadow-blue-500/40 transition-all duration-200">
						{#if loading}
							<span class="inline-block w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin mr-2"></span> Guardando...
						{:else}
							<span class="mr-2">🚀</span> Finalizar configuración
						{/if}
					</Button>
				{/if}
			</div>
		</div>
	</div>
</div>
