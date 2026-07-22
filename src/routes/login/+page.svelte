<script lang="ts">
	import { auth } from '$lib/stores/auth';
	import { goto } from '$app/navigation';

	type Mode = 'login' | 'register';
	let mode = $state<Mode>('login');
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
				goto('/');
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
				successMsg =
					'Cuenta creada. Revisa tu correo para confirmar tu cuenta y luego inicia sesión.';
				mode = 'login';
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

<svelte:head>
	<title>Taxorium — {mode === 'login' ? 'Iniciar Sesión' : 'Crear Cuenta'}</title>
	<meta name="description" content="Sistema de Facturación Electrónica para SUNAT" />
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<div class="login-root">
	<!-- Fondo con grid y gradiente -->
	<div class="bg-grid" aria-hidden="true"></div>
	<div class="bg-glow" aria-hidden="true"></div>

	<!-- Panel izquierdo — Branding -->
	<div class="brand-panel">
		<div class="brand-logo">
			<svg width="48" height="48" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
				<rect width="48" height="48" rx="12" fill="url(#brand-grad)" />
				<path
					d="M14 10h14l-4 11h8L18 38l4-14h-8l0-14z"
					fill="white"
					stroke="white"
					stroke-width="0.5"
					stroke-linejoin="round"
				/>
				<defs>
					<linearGradient id="brand-grad" x1="0" y1="0" x2="48" y2="48" gradientUnits="userSpaceOnUse">
						<stop stop-color="#3b82f6" />
						<stop offset="1" stop-color="#06b6d4" />
					</linearGradient>
				</defs>
			</svg>
			<span class="brand-name">Taxorium</span>
		</div>
		<p class="brand-tagline">Sistema de Facturación<br />Electrónica para SUNAT</p>

		<ul class="brand-features">
			<li>
				<span class="feat-icon">⚡</span>
				<span>Emisión instantánea de boletas y facturas</span>
			</li>
			<li>
				<span class="feat-icon">🔒</span>
				<span>Firma digital segura en la nube</span>
			</li>
			<li>
				<span class="feat-icon">📊</span>
				<span>Reportes y estadísticas en tiempo real</span>
			</li>
			<li>
				<span class="feat-icon">🌐</span>
				<span>Integración directa con SUNAT</span>
			</li>
		</ul>

		<div class="sunat-badge">
			<span class="sunat-dot"></span>
			<span>Conectado a SUNAT Beta</span>
		</div>
	</div>

	<!-- Panel derecho — Formulario -->
	<div class="form-panel">
		<div class="form-card">
			<div class="form-header">
				<h1 class="form-title">
					{mode === 'login' ? 'Bienvenido de nuevo' : 'Crear cuenta root'}
				</h1>
				<p class="form-subtitle">
					{mode === 'login'
						? 'Ingresa tus credenciales para continuar'
						: 'Configura tu cuenta de administrador'}
				</p>
			</div>

			<form onsubmit={handleSubmit} id="auth-form" class="auth-form">
				<div class="field-group">
					<label for="email" class="field-label">Correo electrónico</label>
					<input
						id="email"
						type="email"
						bind:value={email}
						placeholder="usuario@empresa.com"
						required
						autocomplete="email"
						class="field-input"
					/>
				</div>

				<div class="field-group">
					<label for="password" class="field-label">Contraseña</label>
					<input
						id="password"
						type="password"
						bind:value={password}
						placeholder="••••••••"
						required
						autocomplete={mode === 'login' ? 'current-password' : 'new-password'}
						class="field-input"
					/>
				</div>

				{#if mode === 'register'}
					<div class="field-group">
						<label for="confirm-password" class="field-label">Confirmar contraseña</label>
						<input
							id="confirm-password"
							type="password"
							bind:value={confirmPassword}
							placeholder="••••••••"
							required
							autocomplete="new-password"
							class="field-input"
						/>
					</div>
				{/if}

				{#if errorMsg}
					<div class="alert alert-error" role="alert">
						<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
							<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5" />
							<path d="M8 5v4M8 11v.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
						</svg>
						{errorMsg}
					</div>
				{/if}

				{#if successMsg}
					<div class="alert alert-success" role="status">
						<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
							<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5" />
							<path d="M5 8l2 2 4-4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
						</svg>
						{successMsg}
					</div>
				{/if}

				<button id="submit-btn" type="submit" class="btn-primary" disabled={loading}>
					{#if loading}
						<span class="spinner"></span>
						{mode === 'login' ? 'Ingresando...' : 'Creando cuenta...'}
					{:else}
						{mode === 'login' ? 'Iniciar Sesión' : 'Crear Cuenta'}
					{/if}
				</button>
			</form>

			<div class="form-footer">
				{#if mode === 'login'}
					<p>
						¿Primera vez? <button
							id="switch-to-register"
							type="button"
							class="link-btn"
							onclick={() => { mode = 'register'; errorMsg = ''; successMsg = ''; }}
						>Crear cuenta</button>
					</p>
				{:else}
					<p>
						¿Ya tienes cuenta? <button
							id="switch-to-login"
							type="button"
							class="link-btn"
							onclick={() => { mode = 'login'; errorMsg = ''; successMsg = ''; }}
						>Iniciar sesión</button>
					</p>
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

	.login-root {
		display: flex;
		min-height: 100vh;
		position: relative;
		overflow: hidden;
	}

	/* ─── Fondo ─────────────────────────────────────── */
	.bg-grid {
		position: fixed;
		inset: 0;
		background-image:
			linear-gradient(rgba(59, 130, 246, 0.06) 1px, transparent 1px),
			linear-gradient(90deg, rgba(59, 130, 246, 0.06) 1px, transparent 1px);
		background-size: 40px 40px;
		pointer-events: none;
		z-index: 0;
	}

	.bg-glow {
		position: fixed;
		top: -20%;
		left: -10%;
		width: 60%;
		height: 60%;
		background: radial-gradient(ellipse, rgba(59, 130, 246, 0.12) 0%, transparent 70%);
		pointer-events: none;
		z-index: 0;
	}

	/* ─── Panel izquierdo ───────────────────────────── */
	.brand-panel {
		flex: 1;
		display: flex;
		flex-direction: column;
		justify-content: center;
		padding: 4rem 3rem;
		position: relative;
		z-index: 1;
	}

	.brand-logo {
		display: flex;
		align-items: center;
		gap: 0.875rem;
		margin-bottom: 1.5rem;
	}

	.brand-name {
		font-size: 2.5rem;
		font-weight: 800;
		color: #f1f5f9;
		letter-spacing: -0.03em;
	}

	.brand-tagline {
		font-size: 1.125rem;
		color: #64748b;
		line-height: 1.6;
		margin: 0 0 2.5rem 0;
		font-weight: 400;
	}

	.brand-features {
		list-style: none;
		padding: 0;
		margin: 0 0 3rem 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.brand-features li {
		display: flex;
		align-items: center;
		gap: 0.875rem;
		color: #94a3b8;
		font-size: 0.9375rem;
	}

	.feat-icon {
		font-size: 1.125rem;
		width: 2rem;
		text-align: center;
	}

	.sunat-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: rgba(16, 185, 129, 0.08);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 100px;
		padding: 0.375rem 0.875rem;
		color: #10b981;
		font-size: 0.8125rem;
		font-weight: 500;
		width: fit-content;
	}

	.sunat-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #10b981;
		box-shadow: 0 0 6px #10b981;
		animation: pulse-dot 2s ease-in-out infinite;
	}

	@keyframes pulse-dot {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.4; }
	}

	/* ─── Panel derecho ─────────────────────────────── */
	.form-panel {
		flex: 0 0 480px;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		position: relative;
		z-index: 1;
	}

	.form-card {
		width: 100%;
		max-width: 400px;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.07);
		border-radius: 20px;
		padding: 2.5rem;
		backdrop-filter: blur(20px);
		box-shadow:
			0 0 0 1px rgba(59, 130, 246, 0.05),
			0 32px 64px -16px rgba(0, 0, 0, 0.6),
			inset 0 1px 0 rgba(255, 255, 255, 0.06);
	}

	.form-header {
		margin-bottom: 2rem;
	}

	.form-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.375rem 0;
		letter-spacing: -0.02em;
	}

	.form-subtitle {
		font-size: 0.875rem;
		color: #64748b;
		margin: 0;
	}

	/* ─── Formulario ─────────────────────────────────── */
	.auth-form {
		display: flex;
		flex-direction: column;
		gap: 1.125rem;
	}

	.field-group {
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
	}

	.field-label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
		letter-spacing: 0.01em;
	}

	.field-input {
		background: rgba(255, 255, 255, 0.04);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 10px;
		padding: 0.7rem 0.875rem;
		color: #f1f5f9;
		font-size: 0.9375rem;
		font-family: 'Inter', sans-serif;
		outline: none;
		transition: border-color 0.2s, box-shadow 0.2s, background 0.2s;
		width: 100%;
		box-sizing: border-box;
	}

	.field-input::placeholder {
		color: #334155;
	}

	.field-input:focus {
		border-color: rgba(59, 130, 246, 0.6);
		background: rgba(59, 130, 246, 0.04);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.12);
	}

	/* ─── Alertas ─────────────────────────────────────── */
	.alert {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		border-radius: 10px;
		font-size: 0.8438rem;
		line-height: 1.5;
	}

	.alert svg {
		flex-shrink: 0;
		margin-top: 1px;
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.08);
		border: 1px solid rgba(239, 68, 68, 0.2);
		color: #fca5a5;
	}

	.alert-success {
		background: rgba(16, 185, 129, 0.08);
		border: 1px solid rgba(16, 185, 129, 0.2);
		color: #6ee7b7;
	}

	/* ─── Botón principal ─────────────────────────────── */
	.btn-primary {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.8125rem;
		background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
		border: none;
		border-radius: 10px;
		color: #fff;
		font-size: 0.9375rem;
		font-weight: 600;
		font-family: 'Inter', sans-serif;
		cursor: pointer;
		transition: opacity 0.2s, transform 0.15s, box-shadow 0.2s;
		box-shadow: 0 4px 16px rgba(59, 130, 246, 0.3);
		margin-top: 0.25rem;
	}

	.btn-primary:hover:not(:disabled) {
		opacity: 0.92;
		transform: translateY(-1px);
		box-shadow: 0 6px 20px rgba(59, 130, 246, 0.4);
	}

	.btn-primary:active:not(:disabled) {
		transform: translateY(0);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Spinner */
	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: #fff;
		border-radius: 50%;
		animation: spin 0.7s linear infinite;
		flex-shrink: 0;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* ─── Footer del form ─────────────────────────────── */
	.form-footer {
		margin-top: 1.25rem;
		text-align: center;
		font-size: 0.875rem;
		color: #64748b;
	}

	.link-btn {
		background: none;
		border: none;
		color: #3b82f6;
		cursor: pointer;
		font-size: inherit;
		font-family: inherit;
		font-weight: 500;
		padding: 0;
		transition: color 0.15s;
	}

	.link-btn:hover {
		color: #60a5fa;
		text-decoration: underline;
	}

	/* ─── Responsivo ──────────────────────────────────── */
	@media (max-width: 768px) {
		.brand-panel {
			display: none;
		}
		.form-panel {
			flex: 1;
		}
	}
</style>
