<script lang="ts">
	import { goto } from '$app/navigation';
	import { auth } from '$lib/stores/auth';
	import { tenantStore } from '$lib/stores/tenant';

	// Pasos del wizard
	type Step = 'empresa' | 'series' | 'confirmar';
	let currentStep = $state<Step>('empresa');

	// Datos del formulario
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

	function stepIndex(s: Step) {
		return steps.findIndex((x) => x.id === s);
	}

	// Validación básica de RUC (11 dígitos, empieza en 10 o 20)
	function isValidRuc(r: string): boolean {
		return /^(10|20)\d{9}$/.test(r);
	}

	async function validateRuc() {
		if (!isValidRuc(ruc)) {
			rucValid = false;
			return;
		}
		rucValidating = true;
		// Aquí iría una llamada a la API de consulta RUC de SUNAT
		// Por ahora validamos solo el formato
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
	<link
		href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<div class="setup-root">
	<div class="bg-grid" aria-hidden="true"></div>

	<div class="setup-container">
		<!-- Header -->
		<div class="setup-header">
			<div class="logo">
				<svg width="36" height="36" viewBox="0 0 48 48" fill="none">
					<rect width="48" height="48" rx="12" fill="url(#sg)" />
					<path d="M14 10h14l-4 11h8L18 38l4-14h-8l0-14z" fill="white" stroke="white" stroke-width="0.5" stroke-linejoin="round" />
					<defs>
						<linearGradient id="sg" x1="0" y1="0" x2="48" y2="48" gradientUnits="userSpaceOnUse">
							<stop stop-color="#3b82f6" /><stop offset="1" stop-color="#06b6d4" />
						</linearGradient>
					</defs>
				</svg>
				<span>Taxorium</span>
			</div>
			<h1 class="setup-title">Configuración inicial</h1>
			<p class="setup-subtitle">Configura tu empresa para empezar a emitir comprobantes electrónicos</p>
		</div>

		<!-- Stepper -->
		<div class="stepper" role="list">
			{#each steps as step, i}
				<div
					class="step"
					class:active={currentStep === step.id}
					class:done={stepIndex(currentStep) > i}
					role="listitem"
				>
					<div class="step-circle">
						{#if stepIndex(currentStep) > i}
							<svg width="14" height="14" viewBox="0 0 14 14" fill="none">
								<path d="M2.5 7l3 3 6-6" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
							</svg>
						{:else}
							{step.icon}
						{/if}
					</div>
					<span class="step-label">{step.label}</span>
				</div>
				{#if i < steps.length - 1}
					<div class="step-line" class:done={stepIndex(currentStep) > i}></div>
				{/if}
			{/each}
		</div>

		<!-- Tarjeta de contenido -->
		<div class="setup-card">

			<!-- PASO 1: Datos de empresa -->
			{#if currentStep === 'empresa'}
				<div class="step-content">
					<h2 class="step-title">Datos de tu empresa</h2>
					<p class="step-desc">Esta información aparecerá en todos tus comprobantes electrónicos.</p>

					<div class="form-grid">
						<div class="field-group field-full">
							<label for="ruc" class="field-label">RUC <span class="required">*</span></label>
							<div class="input-wrap">
								<input
									id="ruc"
									type="text"
									bind:value={ruc}
									placeholder="20123456789"
									maxlength="11"
									class="field-input"
									class:valid={rucValid === true}
									class:invalid={rucValid === false}
									onblur={validateRuc}
									oninput={() => { rucValid = null; }}
								/>
								{#if rucValidating}
									<span class="input-icon"><span class="mini-spinner"></span></span>
								{:else if rucValid === true}
									<span class="input-icon valid-icon">✓</span>
								{:else if rucValid === false}
									<span class="input-icon invalid-icon">✗</span>
								{/if}
							</div>
						</div>

						<div class="field-group field-full">
							<label for="razon-social" class="field-label">Razón social <span class="required">*</span></label>
							<input id="razon-social" type="text" bind:value={razonSocial} placeholder="EMPRESA SAC" class="field-input" />
						</div>

						<div class="field-group">
							<label for="nombre-comercial" class="field-label">Nombre comercial</label>
							<input id="nombre-comercial" type="text" bind:value={nombreComercial} placeholder="Mi Tienda (opcional)" class="field-input" />
						</div>

						<div class="field-group">
							<label for="departamento" class="field-label">Departamento</label>
							<input id="departamento" type="text" bind:value={departamento} placeholder="Lima" class="field-input" />
						</div>

						<div class="field-group field-full">
							<label for="direccion" class="field-label">Dirección fiscal <span class="required">*</span></label>
							<input id="direccion" type="text" bind:value={direccion} placeholder="Av. Javier Prado Este 123, San Isidro" class="field-input" />
						</div>

						<div class="field-group">
							<label for="telefono-empresa" class="field-label">Teléfono</label>
							<input id="telefono-empresa" type="tel" bind:value={telefono} placeholder="01-234-5678" class="field-input" />
						</div>

						<div class="field-group">
							<label for="email-empresa" class="field-label">Email de empresa</label>
							<input id="email-empresa" type="email" bind:value={email} placeholder="contacto@empresa.com" class="field-input" />
						</div>
					</div>
				</div>
			{/if}

			<!-- PASO 2: Series -->
			{#if currentStep === 'series'}
				<div class="step-content">
					<h2 class="step-title">Configuración de series</h2>
					<p class="step-desc">
						Define las series con las que iniciarás tu numeración. SUNAT requiere series distintas para
						boletas y facturas.
					</p>

					<div class="series-info">
						<div class="info-item">
							<span class="info-icon">📋</span>
							<div>
								<strong>Boletas electrónicas</strong>
								<span>Deben comenzar con <code>B</code> + 3 dígitos (ej: B001)</span>
							</div>
						</div>
						<div class="info-item">
							<span class="info-icon">🏭</span>
							<div>
								<strong>Facturas electrónicas</strong>
								<span>Deben comenzar con <code>F</code> + 3 dígitos (ej: F001)</span>
							</div>
						</div>
					</div>

					<div class="form-grid">
						<div class="field-group">
							<label for="serie-boleta" class="field-label">Serie de boleta <span class="required">*</span></label>
							<input
								id="serie-boleta"
								type="text"
								bind:value={serieBoleta}
								placeholder="B001"
								maxlength="4"
								class="field-input"
								oninput={(e) => { serieBoleta = (e.target as HTMLInputElement).value.toUpperCase(); }}
							/>
							<span class="field-hint">El correlativo iniciará en B001-00000001</span>
						</div>

						<div class="field-group">
							<label for="serie-factura" class="field-label">Serie de factura <span class="required">*</span></label>
							<input
								id="serie-factura"
								type="text"
								bind:value={serieFactura}
								placeholder="F001"
								maxlength="4"
								class="field-input"
								oninput={(e) => { serieFactura = (e.target as HTMLInputElement).value.toUpperCase(); }}
							/>
							<span class="field-hint">El correlativo iniciará en F001-00000001</span>
						</div>
					</div>
				</div>
			{/if}

			<!-- PASO 3: Confirmar -->
			{#if currentStep === 'confirmar'}
				<div class="step-content">
					<h2 class="step-title">Confirma tu configuración</h2>
					<p class="step-desc">Revisa los datos antes de guardar. Podrás editarlos después desde Configuración.</p>

					<div class="summary-grid">
						<div class="summary-section">
							<h3 class="summary-section-title">🏢 Empresa</h3>
							<div class="summary-row"><span>RUC</span><strong>{ruc}</strong></div>
							<div class="summary-row"><span>Razón social</span><strong>{razonSocial}</strong></div>
							{#if nombreComercial}<div class="summary-row"><span>Nombre comercial</span><strong>{nombreComercial}</strong></div>{/if}
							<div class="summary-row"><span>Dirección</span><strong>{direccion}</strong></div>
							{#if departamento}<div class="summary-row"><span>Departamento</span><strong>{departamento}</strong></div>{/if}
							{#if telefono}<div class="summary-row"><span>Teléfono</span><strong>{telefono}</strong></div>{/if}
							{#if email}<div class="summary-row"><span>Email</span><strong>{email}</strong></div>{/if}
						</div>

						<div class="summary-section">
							<h3 class="summary-section-title">📄 Series</h3>
							<div class="summary-row"><span>Boletas</span><strong>{serieBoleta}-00000001</strong></div>
							<div class="summary-row"><span>Facturas</span><strong>{serieFactura}-00000001</strong></div>
						</div>
					</div>
				</div>
			{/if}

			<!-- Error -->
			{#if errorMsg}
				<div class="alert-error" role="alert">
					<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
						<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5"/>
						<path d="M8 5v4M8 11v.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
					</svg>
					{errorMsg}
				</div>
			{/if}

			<!-- Navegación -->
			<div class="step-actions">
				{#if currentStep !== 'empresa'}
					<button id="prev-step-btn" type="button" class="btn-secondary" onclick={prevStep} disabled={loading}>
						← Anterior
					</button>
				{:else}
					<div></div>
				{/if}

				{#if currentStep !== 'confirmar'}
					<button id="next-step-btn" type="button" class="btn-primary" onclick={nextStep}>
						Siguiente →
					</button>
				{:else}
					<button id="finish-setup-btn" type="button" class="btn-primary btn-finish" onclick={handleFinish} disabled={loading}>
						{#if loading}
							<span class="spinner"></span> Guardando...
						{:else}
							🚀 Finalizar configuración
						{/if}
					</button>
				{/if}
			</div>
		</div>
	</div>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		font-family: 'Inter', system-ui, sans-serif;
		background: #0a0d14;
	}

	.setup-root {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem 1rem;
		position: relative;
		overflow: hidden;
	}

	.bg-grid {
		position: fixed;
		inset: 0;
		background-image:
			linear-gradient(rgba(59, 130, 246, 0.05) 1px, transparent 1px),
			linear-gradient(90deg, rgba(59, 130, 246, 0.05) 1px, transparent 1px);
		background-size: 40px 40px;
		pointer-events: none;
		z-index: 0;
	}

	.setup-container {
		width: 100%;
		max-width: 680px;
		position: relative;
		z-index: 1;
	}

	.setup-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.logo {
		display: inline-flex;
		align-items: center;
		gap: 0.625rem;
		margin-bottom: 1.25rem;
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.setup-title {
		font-size: 1.75rem;
		font-weight: 800;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
		letter-spacing: -0.03em;
	}

	.setup-subtitle {
		color: #64748b;
		font-size: 0.9375rem;
		margin: 0;
	}

	/* ─── Stepper ──────────────────────────────────────── */
	.stepper {
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 2rem;
		gap: 0;
	}

	.step {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.375rem;
	}

	.step-circle {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.05);
		border: 1.5px solid rgba(255, 255, 255, 0.1);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1rem;
		transition: all 0.3s;
	}

	.step.active .step-circle {
		background: rgba(59, 130, 246, 0.15);
		border-color: #3b82f6;
		box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.1);
	}

	.step.done .step-circle {
		background: #3b82f6;
		border-color: #3b82f6;
	}

	.step-label {
		font-size: 0.75rem;
		color: #475569;
		font-weight: 500;
		transition: color 0.2s;
	}

	.step.active .step-label { color: #3b82f6; }
	.step.done .step-label { color: #60a5fa; }

	.step-line {
		height: 1.5px;
		flex: 1;
		min-width: 60px;
		background: rgba(255, 255, 255, 0.08);
		margin-bottom: 22px;
		transition: background 0.3s;
	}

	.step-line.done { background: #3b82f6; }

	/* ─── Tarjeta ───────────────────────────────────────── */
	.setup-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.07);
		border-radius: 20px;
		padding: 2.5rem;
		backdrop-filter: blur(20px);
		box-shadow: 0 32px 64px -16px rgba(0, 0, 0, 0.5);
	}

	.step-content {
		margin-bottom: 1.5rem;
	}

	.step-title {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.375rem 0;
		letter-spacing: -0.02em;
	}

	.step-desc {
		font-size: 0.875rem;
		color: #64748b;
		margin: 0 0 1.75rem 0;
		line-height: 1.6;
	}

	/* ─── Formulario ────────────────────────────────────── */
	.form-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.field-group { display: flex; flex-direction: column; gap: 0.375rem; }
	.field-full { grid-column: 1 / -1; }

	.field-label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.required { color: #f87171; }

	.field-input {
		background: rgba(255, 255, 255, 0.04);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 10px;
		padding: 0.65rem 0.875rem;
		color: #f1f5f9;
		font-size: 0.9375rem;
		font-family: 'Inter', sans-serif;
		outline: none;
		transition: border-color 0.2s, box-shadow 0.2s;
		width: 100%;
		box-sizing: border-box;
	}

	.field-input::placeholder { color: #334155; }
	.field-input:focus {
		border-color: rgba(59, 130, 246, 0.6);
		background: rgba(59, 130, 246, 0.04);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.12);
	}
	.field-input.valid { border-color: rgba(16, 185, 129, 0.5); }
	.field-input.invalid { border-color: rgba(239, 68, 68, 0.5); }

	.field-hint {
		font-size: 0.75rem;
		color: #475569;
	}

	.input-wrap { position: relative; }
	.input-wrap .field-input { padding-right: 2.25rem; }
	.input-icon {
		position: absolute;
		right: 0.75rem;
		top: 50%;
		transform: translateY(-50%);
		font-size: 0.875rem;
	}
	.valid-icon { color: #10b981; }
	.invalid-icon { color: #ef4444; }

	.mini-spinner {
		display: inline-block;
		width: 12px;
		height: 12px;
		border: 1.5px solid rgba(255,255,255,0.2);
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.7s linear infinite;
	}

	/* ─── Series info ───────────────────────────────────── */
	.series-info {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 1.75rem;
	}

	.info-item {
		display: flex;
		gap: 0.875rem;
		align-items: flex-start;
		background: rgba(59, 130, 246, 0.05);
		border: 1px solid rgba(59, 130, 246, 0.1);
		border-radius: 10px;
		padding: 0.875rem 1rem;
	}

	.info-icon { font-size: 1.25rem; flex-shrink: 0; }

	.info-item div {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.info-item strong {
		color: #e2e8f0;
		font-size: 0.875rem;
	}

	.info-item span {
		color: #64748b;
		font-size: 0.8125rem;
	}

	.info-item code {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
		padding: 0.1em 0.35em;
		border-radius: 4px;
		font-family: monospace;
	}

	/* ─── Summary ───────────────────────────────────────── */
	.summary-grid {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.summary-section {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.06);
		border-radius: 12px;
		padding: 1.25rem;
	}

	.summary-section-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #94a3b8;
		margin: 0 0 0.875rem 0;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.summary-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.375rem 0;
		border-bottom: 1px solid rgba(255, 255, 255, 0.04);
		font-size: 0.875rem;
	}

	.summary-row:last-child { border-bottom: none; }
	.summary-row span { color: #64748b; }
	.summary-row strong { color: #e2e8f0; text-align: right; }

	/* ─── Alertas y acciones ────────────────────────────── */
	.alert-error {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: rgba(239, 68, 68, 0.08);
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 10px;
		padding: 0.75rem 1rem;
		color: #fca5a5;
		font-size: 0.8438rem;
		margin-bottom: 1rem;
	}

	.step-actions {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 0.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.06);
	}

	.btn-primary, .btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 10px;
		font-size: 0.9375rem;
		font-weight: 600;
		font-family: 'Inter', sans-serif;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
		color: #fff;
		box-shadow: 0 4px 14px rgba(59, 130, 246, 0.3);
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 6px 20px rgba(59, 130, 246, 0.4);
	}

	.btn-primary:disabled { opacity: 0.6; cursor: not-allowed; }

	.btn-secondary {
		background: rgba(255, 255, 255, 0.05);
		color: #94a3b8;
		border: 1px solid rgba(255, 255, 255, 0.08);
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.08);
		color: #e2e8f0;
	}

	.btn-finish { padding: 0.875rem 2rem; }

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: #fff;
		border-radius: 50%;
		animation: spin 0.7s linear infinite;
		flex-shrink: 0;
	}

	@keyframes spin { to { transform: rotate(360deg); } }

	@media (max-width: 600px) {
		.form-grid { grid-template-columns: 1fr; }
		.field-full { grid-column: auto; }
		.setup-card { padding: 1.5rem; }
	}
</style>
