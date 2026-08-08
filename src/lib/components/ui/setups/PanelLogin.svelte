<script lang="ts">
	import { auth } from '$lib/stores/auth';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';

	type Mode = 'login' | 'register';

	let {
		initialMode = 'login',
		onSuccess
	}: { initialMode?: Mode; onSuccess?: () => void } = $props();

	let mode = $state<Mode>(initialMode);
	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let errorMsg = $state('');
	let successMsg = $state('');
	let loading = $state(false);

	async function handleSubmit(e: Event) {
		e.preventDefault();
		errorMsg = '';
		successMsg = '';
		loading = true;

		try {
			if (mode === 'login') {
				await auth.signIn(email, password);
				onSuccess?.();
			} else {
				if (password !== confirmPassword) {
					errorMsg = 'Las contraseñas no coinciden.';
					return;
				}
				if (password.length < 8) {
					errorMsg = 'La contraseña debe tener al menos 8 caracteres.';
					return;
				}
				await auth.signUp(email, password);
				successMsg = 'Cuenta creada. Revisa tu correo para confirmar y luego inicia sesión.';
				mode = 'login';
				email = '';
				password = '';
				confirmPassword = '';
			}
		} catch (err: unknown) {
			const msg = err instanceof Error ? err.message : 'Error desconocido';
			if (msg.includes('Invalid login credentials')) {
				errorMsg = 'Correo o contraseña incorrectos.';
			} else if (msg.includes('User already registered')) {
				errorMsg = 'Ya existe una cuenta con ese correo.';
			} else if (msg.includes('Email not confirmed')) {
				errorMsg = 'Confirma tu correo antes de iniciar sesión.';
			} else {
				errorMsg = msg;
			}
		} finally {
			loading = false;
		}
	}
</script>

<div class="panel-login">
	<div class="panel-brand">
		<div class="brand-icon">
			<svg width="28" height="28" viewBox="0 0 48 48" fill="none">
				<rect width="48" height="48" rx="12" fill="url(#pl-grad)" />
				<path d="M14 10h14l-4 11h8L18 38l4-14h-8l0-14z" fill="white" stroke="white" stroke-width="0.5" stroke-linejoin="round" />
				<defs>
					<linearGradient id="pl-grad" x1="0" y1="0" x2="48" y2="48" gradientUnits="userSpaceOnUse">
						<stop stop-color="#3b82f6" />
						<stop offset="1" stop-color="#06b6d4" />
					</linearGradient>
				</defs>
			</svg>
		</div>
		<div class="brand-text">
			<span class="brand-name">Taxorium</span>
			<span class="brand-sub">Facturación Electrónica SUNAT</span>
		</div>
	</div>

	<div class="panel-header">
		<h2 class="panel-title">
			{mode === 'login' ? 'Iniciar sesión' : 'Crear cuenta'}
		</h2>
		<p class="panel-subtitle">
			{mode === 'login' ? 'Ingresa tus credenciales para continuar' : 'Configura tu cuenta de administrador'}
		</p>
	</div>

	<form onsubmit={handleSubmit} class="auth-form">
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
			<div class="alert alert-error" role="alert">
				<svg width="14" height="14" viewBox="0 0 16 16" fill="none" style="flex-shrink:0;margin-top:1px">
					<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5" />
					<path d="M8 5v4M8 11v.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
				</svg>
				{errorMsg}
			</div>
		{/if}

		{#if successMsg}
			<div class="alert alert-success" role="status">
				<svg width="14" height="14" viewBox="0 0 16 16" fill="none" style="flex-shrink:0;margin-top:1px">
					<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5" />
					<path d="M5 8l2 2 4-4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
				</svg>
				{successMsg}
			</div>
		{/if}

		<Button id="pl-submit" type="submit" variant="primary" disabled={loading} class="w-full mt-2">
			{#if loading}
				<span class="spinner"></span>
				{mode === 'login' ? 'Ingresando...' : 'Creando cuenta...'}
			{:else}
				{mode === 'login' ? 'Iniciar Sesión' : 'Crear Cuenta'}
			{/if}
		</Button>
	</form>

	<div class="form-footer">
		{#if mode === 'login'}
			<p>¿Primera vez? <button id="pl-switch-register" type="button" class="link-btn" onclick={() => { mode = 'register'; errorMsg = ''; successMsg = ''; }}>Crear cuenta</button></p>
		{:else}
			<p>¿Ya tienes cuenta? <button id="pl-switch-login" type="button" class="link-btn" onclick={() => { mode = 'login'; errorMsg = ''; successMsg = ''; }}>Iniciar sesión</button></p>
		{/if}
	</div>
</div>

<style>
	.panel-login { display: flex; flex-direction: column; gap: 1.25rem; padding: 1.25rem; }
	.panel-brand { display: flex; align-items: center; gap: 0.625rem; padding-bottom: 0.75rem; border-bottom: 1px solid rgba(255,255,255,0.06); }
	.brand-icon { flex-shrink: 0; }
	.brand-text { display: flex; flex-direction: column; gap: 0.1rem; }
	.brand-name { font-size: 0.9375rem; font-weight: 700; color: #f1f5f9; letter-spacing: -0.02em; }
	.brand-sub { font-size: 0.6875rem; color: #475569; }
	.panel-header { display: flex; flex-direction: column; gap: 0.25rem; }
	.panel-title { font-size: 1rem; font-weight: 700; color: #f1f5f9; margin: 0; letter-spacing: -0.02em; }
	.panel-subtitle { font-size: 0.75rem; color: #64748b; margin: 0; }
	.auth-form { display: flex; flex-direction: column; gap: 0.875rem; }
	.alert { display: flex; align-items: flex-start; gap: 0.5rem; padding: 0.6rem 0.75rem; border-radius: 8px; font-size: 0.75rem; line-height: 1.5; }
	.alert-error { background: rgba(239,68,68,0.08); border: 1px solid rgba(239,68,68,0.2); color: #fca5a5; }
	.alert-success { background: rgba(16,185,129,0.08); border: 1px solid rgba(16,185,129,0.2); color: #6ee7b7; }
	.spinner { width: 13px; height: 13px; border: 2px solid rgba(255,255,255,0.25); border-top-color: #fff; border-radius: 50%; animation: spin 0.65s linear infinite; flex-shrink: 0; }
	@keyframes spin { to { transform: rotate(360deg); } }
	.form-footer { text-align: center; font-size: 0.75rem; color: #64748b; }
	.form-footer p { margin: 0; }
	.link-btn { background: none; border: none; color: #3b82f6; cursor: pointer; font-size: inherit; font-family: inherit; font-weight: 500; padding: 0; transition: color 0.1s; }
	.link-btn:hover { color: #60a5fa; text-decoration: underline; }
</style>
