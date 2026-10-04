<script lang="ts">
	import { auth } from '$lib/stores/auth';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';
	import FormCompany from '$lib/components/ui/views/account/company/FormCompany.svelte';
	import { open } from '@tauri-apps/plugin-dialog';

	// ─── Tipos ─────────────────────────────────────────────────────────────────
	// 'login'    → formulario de inicio de sesión
	// 'register' → paso 1: datos del usuario (email + contraseña)
	// 'company'  → paso 2: datos de la empresa (solo registro nuevo)
	type Mode = 'login' | 'register' | 'company';

	const STEP_LABELS: Record<Mode, string> = {
		login: 'Iniciar sesión',
		register: 'Crear cuenta',
		company: 'Datos de la empresa'
	};

	const STEP_SUBTITLES: Record<Mode, string> = {
		login: 'Ingresa tus credenciales para continuar',
		register: 'Configura tu cuenta de administrador',
		company: 'Completa la información de tu empresa'
	};

	// ─── Props ─────────────────────────────────────────────────────────────────
	let { initialMode = 'login', onSuccess }: { initialMode?: Mode; onSuccess?: () => void } =
		$props();

	// ─── Estado ────────────────────────────────────────────────────────────────
	let mode = $state<Mode>(initialMode);

	// Paso 1: datos del usuario
	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');

	// Paso 2: datos de la empresa (usando los mismos de FormCompany)
	let companyData = $state({
		ruc: '',
		razonSocial: '',
		nombreComercial: '',
		departamento: '',
		direccion: '',
		telefono: '',
		email: '',
		ubigeo: '',
		usuarioSol: '',
		claveSol: '',
		certificadoPath: '',
		certificadoPassword: '',
		rucValid: null as boolean | null,
		rucValidating: false
	});

	// UI
	let errorMsg = $state('');
	let successMsg = $state('');
	let loading = $state(false);

	function resetMessages() {
		errorMsg = '';
		successMsg = '';
	}

	function goToMode(next: Mode) {
		mode = next;
		resetMessages();
	}

	function validateRuc() {
		// Mock de validación básica de RUC
		if (companyData.ruc.length === 11) {
			companyData.rucValid = true;
		} else {
			companyData.rucValid = null;
		}
	}

	async function handleSelectCertificado() {
		try {
			const selected = await open({
				multiple: false,
				filters: [{ name: 'Certificados', extensions: ['p12', 'pfx'] }]
			});
			if (selected && typeof selected === 'string') {
				companyData.certificadoPath = selected;
			}
		} catch (err) {
			console.error('Error al seleccionar certificado:', err);
		}
	}

	// ─── Paso 1 → Paso 2: validar datos del usuario antes de avanzar ───────────
	function handleNextStep(e: Event) {
		e.preventDefault();
		resetMessages();

		if (password !== confirmPassword) {
			errorMsg = 'Las contraseñas no coinciden.';
			return;
		}
		if (password.length < 8) {
			errorMsg = 'La contraseña debe tener al menos 8 caracteres.';
			return;
		}

		goToMode('company');
	}

	// ─── Paso 2 → Crear cuenta en Supabase ─────────────────────────────────────
	async function handleCreateAccount() {
		resetMessages();
		loading = true;

		try {
			// 1. Crear el usuario en auth
			await auth.signUp(email, password);

			// 2. TODO: Guardar los datos de la empresa (companyData) en tu API/BD

			successMsg =
				'Cuenta y empresa creadas. Revisa tu correo para confirmar y luego inicia sesión.';
			// Limpiar todo y volver al login
			email = '';
			password = '';
			confirmPassword = '';
			// reset company data
			companyData = {
				ruc: '',
				razonSocial: '',
				nombreComercial: '',
				departamento: '',
				direccion: '',
				telefono: '',
				email: '',
				ubigeo: '',
				usuarioSol: '',
				claveSol: '',
				certificadoPath: '',
				certificadoPassword: '',
				rucValid: null,
				rucValidating: false
			};
			goToMode('login');
		} catch (err: unknown) {
			const msg = err instanceof Error ? err.message : 'Error desconocido';
			if (msg.includes('User already registered')) {
				errorMsg = 'Ya existe una cuenta con ese correo.';
			} else {
				errorMsg = msg;
			}
		} finally {
			loading = false;
		}
	}

	// ─── Login ─────────────────────────────────────────────────────────────────
	async function handleLogin(e: Event) {
		e.preventDefault();
		resetMessages();
		loading = true;

		try {
			await auth.signIn(email, password);
			onSuccess?.();
		} catch (err: unknown) {
			const msg = err instanceof Error ? err.message : 'Error desconocido';
			if (msg.includes('Invalid login credentials')) {
				errorMsg = 'Correo o contraseña incorrectos.';
			} else if (msg.includes('Email not confirmed')) {
				errorMsg = 'Confirma tu correo antes de iniciar sesión.';
			} else {
				errorMsg = msg;
			}
		} finally {
			loading = false;
		}
	}

	// Despacha al handler correcto según el paso activo
	function handleSubmit(e: Event) {
		if (mode === 'login') return handleLogin(e);
		if (mode === 'register') return handleNextStep(e);
		// company form is handled by FormCompany's own onSave
	}
</script>

<div class="flex flex-col gap-5 p-5">
	<div class="flex items-center gap-2.5 border-b border-white/6 pb-3">
		<div class="shrink-0">
			<svg width="28" height="28" fill="none" viewBox="0 0 48 48"
				><rect width="48" height="48" fill="url(#pl-grad)" rx="12" /><path
					fill="#fff"
					stroke="#fff"
					stroke-linejoin="round"
					stroke-width=".5"
					d="M14 10h14l-4 11h8L18 38l4-14h-8z"
				/><defs
					><linearGradient id="pl-grad" x1="0" x2="48" y1="0" y2="48" gradientUnits="userSpaceOnUse"
						><stop stop-color="#3b82f6" /><stop offset="1" stop-color="#06b6d4" /></linearGradient
					></defs
				></svg
			>
		</div>
		<div class="flex flex-col gap-0.5">
			<span class="text-[0.9375rem] font-bold tracking-[-0.02em] text-slate-100">Taxorium</span>
			<span class="text-[0.6875rem] text-slate-600">Facturación Electrónica SUNAT</span>
		</div>
	</div>

	<div class="flex flex-col gap-1">
		{#if mode === 'register' || mode === 'company'}
			<!-- Indicador de pasos -->
			<div class="mb-1 flex items-center gap-1.5 text-[0.6875rem]">
				<span
					class="flex size-4 items-center justify-center rounded-full text-[0.6rem] font-bold
					{mode === 'company' ? 'bg-emerald-500/20 text-emerald-400' : 'bg-blue-500/20 text-blue-400'}"
				>
					{mode === 'company' ? '✓' : '1'}
				</span>
				<span class={mode === 'company' ? 'text-slate-500' : 'text-blue-400'}>Cuenta</span>
				<span class="text-slate-700">→</span>
				<span
					class="flex size-4 items-center justify-center rounded-full text-[0.6rem] font-bold
					{mode === 'company' ? 'bg-blue-500/20 text-blue-400' : 'bg-white/6 text-slate-600'}">2</span
				>
				<span class={mode === 'company' ? 'text-blue-400' : 'text-slate-600'}>Empresa</span>
			</div>
		{/if}
		<h2 class="m-0 text-base font-bold tracking-[-0.02em] text-slate-100">
			{STEP_LABELS[mode]}
		</h2>
		<p class="m-0 text-xs text-slate-500">
			{STEP_SUBTITLES[mode]}
		</p>
	</div>

	{#if mode === 'company'}
		<!-- Paso 2: datos de la empresa -->
		<!-- Renderizamos FormCompany, omitiendo su título interno si es necesario, 
		     pero reutilizando todos los campos y su botón de submit -->
		<div class="custom-scrollbar max-h-[50vh] overflow-y-auto pr-2">
			<FormCompany
				bind:ruc={companyData.ruc}
				bind:razonSocial={companyData.razonSocial}
				bind:nombreComercial={companyData.nombreComercial}
				bind:departamento={companyData.departamento}
				bind:direccion={companyData.direccion}
				bind:telefono={companyData.telefono}
				bind:email={companyData.email}
				bind:ubigeo={companyData.ubigeo}
				bind:usuarioSol={companyData.usuarioSol}
				bind:claveSol={companyData.claveSol}
				bind:certificadoPath={companyData.certificadoPath}
				bind:certificadoPassword={companyData.certificadoPassword}
				bind:rucValid={companyData.rucValid}
				bind:rucValidating={companyData.rucValidating}
				{validateRuc}
				onSelectCertificado={handleSelectCertificado}
				onSave={handleCreateAccount}
				saving={loading}
				submitLabel="Crear Cuenta"
				savingLabel="Creando cuenta..."
			>
				{#snippet messages()}
					{#if errorMsg}
						<div
							role="alert"
							class="flex items-start gap-2 rounded-lg border border-red-500/20 bg-red-500/8 px-3 py-1.5 text-xs leading-relaxed text-red-300"
						>
							{errorMsg}
						</div>
					{/if}
					{#if successMsg}
						<div
							role="status"
							class="flex items-start gap-2 rounded-lg border border-emerald-500/20 bg-emerald-500/8 px-3 py-1.5 text-xs leading-relaxed text-emerald-300"
						>
							{successMsg}
						</div>
					{/if}
				{/snippet}
			</FormCompany>
		</div>
	{:else}
		<form onsubmit={handleSubmit} class="flex flex-col gap-3.5">
			<!-- Paso 1: datos del usuario -->
			<Input
				id="pl-email"
				type="email"
				bind:value={email}
				label="Correo electrónico"
				placeholder="usuario@empresa.com"
				required
				autocomplete="email"
			/>

			<Input
				id="pl-password"
				type="password"
				bind:value={password}
				label="Contraseña"
				placeholder="••••••••"
				required
				autocomplete={mode === 'login' ? 'current-password' : 'new-password'}
			/>

			{#if mode === 'register'}
				<Input
					id="pl-confirm"
					type="password"
					bind:value={confirmPassword}
					label="Confirmar contraseña"
					placeholder="••••••••"
					required
					autocomplete="new-password"
				/>
			{/if}

			{#if errorMsg}
				<div
					role="alert"
					class="flex items-start gap-2 rounded-lg border border-red-500/20 bg-red-500/8 px-3 py-1.5 text-xs leading-relaxed text-red-300"
				>
					<svg width="14" height="14" viewBox="0 0 16 16" fill="none" class="mt-px shrink-0">
						<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5" />
						<path
							d="M8 5v4M8 11v.5"
							stroke="currentColor"
							stroke-width="1.5"
							stroke-linecap="round"
						/>
					</svg>
					{errorMsg}
				</div>
			{/if}

			{#if successMsg}
				<div
					role="status"
					class="flex items-start gap-2 rounded-lg border border-emerald-500/20 bg-emerald-500/8 px-3 py-1.5 text-xs leading-relaxed text-emerald-300"
				>
					<svg width="14" height="14" viewBox="0 0 16 16" fill="none" class="mt-px shrink-0">
						<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5" />
						<path
							d="M5 8l2 2 4-4"
							stroke="currentColor"
							stroke-width="1.5"
							stroke-linecap="round"
							stroke-linejoin="round"
						/>
					</svg>
					{successMsg}
				</div>
			{/if}

			<Button id="pl-submit" type="submit" variant="primary" disabled={loading} class="mt-2 w-full">
				{#if loading}
					<span
						class="size-3 shrink-0 animate-[spin_0.65s_linear_infinite] rounded-full border-2 border-white/25 border-t-white"
					></span>
					{mode === 'login' ? 'Ingresando...' : 'Creando cuenta...'}
				{:else if mode === 'login'}
					Iniciar Sesión
				{:else if mode === 'register'}
					Siguiente
				{/if}
			</Button>
		</form>
	{/if}

	<div class="text-center text-xs text-slate-500">
		{#if mode === 'login'}
			<p class="m-0">
				¿Primera vez? <button
					id="pl-switch-register"
					type="button"
					class="cursor-pointer border-0 bg-transparent p-0 font-[inherit] font-medium text-blue-500 transition-colors hover:text-blue-400 hover:underline"
					onclick={() => goToMode('register')}>Crear cuenta</button
				>
			</p>
		{:else if mode === 'register'}
			<p class="m-0">
				¿Ya tienes cuenta? <button
					id="pl-switch-login"
					type="button"
					class="cursor-pointer border-0 bg-transparent p-0 font-[inherit] font-medium text-blue-500 transition-colors hover:text-blue-400 hover:underline"
					onclick={() => goToMode('login')}>Iniciar sesión</button
				>
			</p>
		{:else if mode === 'company'}
			<p class="m-0">
				<button
					id="pl-back-register"
					type="button"
					class="cursor-pointer border-0 bg-transparent p-0 font-[inherit] font-medium text-slate-400 transition-colors hover:text-slate-300 hover:underline"
					onclick={() => goToMode('register')}>← Volver al paso anterior</button
				>
			</p>
		{/if}
	</div>
</div>
