<script lang="ts">
	import { auth } from '$lib/stores/auth';
	import { currentTenant, tenantStore } from '$lib/stores/tenant';
	import StepEmpresa from '$lib/components/ui/setups/StepEmpresa.svelte';
	import { open } from '@tauri-apps/plugin-dialog';

	let ruc = $state($currentTenant?.ruc ?? '');
	let razonSocial = $state($currentTenant?.razon_social ?? '');
	let nombreComercial = $state($currentTenant?.nombre_comercial ?? '');
	let departamento = $state($currentTenant?.departamento ?? '');
	let direccion = $state($currentTenant?.direccion ?? '');
	let telefono = $state($currentTenant?.telefono ?? '');
	let email = $state($currentTenant?.email ?? '');

	// Campos SUNAT
	let ubigeo = $state($currentTenant?.ubigeo ?? '');
	let usuarioSol = $state($currentTenant?.usuario_sol ?? '');
	let claveSol = $state($currentTenant?.clave_sol ?? '');
	let certificadoPath = $state($currentTenant?.certificado_path ?? '');
	let certificadoPassword = $state(localStorage.getItem('taxorium_cert_pwd') || '');

	let rucValid = $state<boolean | null>(true);
	let rucValidating = $state(false);

	let saving = $state(false);
	let errorMsg = $state('');
	let successMsg = $state('');

	function isValidRuc(r: string): boolean {
		return /^(10|20)\d{9}$/.test(r);
	}

	async function handleSelectCertificado() {
		try {
			const selected = await open({
				multiple: false,
				filters: [{ name: 'Certificados', extensions: ['p12', 'pfx'] }]
			});
			if (selected && typeof selected === 'string') {
				certificadoPath = selected;
			}
		} catch (err) {
			console.error('Error al seleccionar certificado:', err);
		}
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

	async function handleSave() {
		errorMsg = '';
		successMsg = '';

		if (!ruc || !isValidRuc(ruc)) {
			errorMsg = 'Ingresa un RUC válido (11 dígitos).';
			return;
		}
		if (!razonSocial.trim()) {
			errorMsg = 'La razón social es obligatoria.';
			return;
		}
		if (!direccion.trim()) {
			errorMsg = 'La dirección fiscal es obligatoria.';
			return;
		}

		saving = true;
		try {
			if ($currentTenant) {
				await tenantStore.updateTenant($currentTenant.id, {
					ruc,
					razon_social: razonSocial.trim(),
					nombre_comercial: nombreComercial.trim() || null,
					direccion: direccion.trim(),
					departamento: departamento.trim() || null,
					telefono: telefono.trim() || null,
					email: email.trim() || null,
					ubigeo: ubigeo.trim() || null,
					usuario_sol: usuarioSol.trim() || null,
					clave_sol: claveSol.trim() || null,
					certificado_path: certificadoPath.trim() || null
				});
				localStorage.setItem('taxorium_cert_pwd', certificadoPassword);
			} else {
				if (!$auth.user) {
					errorMsg = 'Debes iniciar sesión primero.';
					saving = false;
					return;
				}
				const tenant = await tenantStore.create($auth.user.id, {
					user_id: $auth.user.id,
					ruc,
					razon_social: razonSocial.trim(),
					nombre_comercial: nombreComercial.trim() || null,
					direccion: direccion.trim(),
					departamento: departamento.trim() || null,
					telefono: telefono.trim() || null,
					email: email.trim() || null,
					ubigeo: ubigeo.trim() || null,
					usuario_sol: usuarioSol.trim() || null,
					clave_sol: claveSol.trim() || null,
					certificado_path: certificadoPath.trim() || null,
					configurado: true,
					activo: true
				});
				localStorage.setItem('taxorium_cert_pwd', certificadoPassword);
				if (tenant) {
					await tenantStore.markConfigured(tenant.id);
				}
			}
			successMsg = 'Configuración de empresa actualizada correctamente.';
		} catch (err: unknown) {
			const msg = err instanceof Error ? err.message : 'Error al guardar';
			if (msg.includes('duplicate') || msg.includes('unique')) {
				errorMsg = 'Ya existe una empresa registrada con ese RUC.';
			} else {
				errorMsg = msg;
			}
		} finally {
			saving = false;
			// Limpiar mensaje de éxito después de unos segundos
			if (successMsg) {
				setTimeout(() => {
					successMsg = '';
				}, 3000);
			}
		}
	}
</script>

<div class="flex h-full flex-col overflow-scroll p-2">
	<h2 class="text-xl font-bold tracking-tight text-slate-100">Configuración de Empresa</h2>
	<p class="text-sm leading-relaxed text-neutral-300">
		Esta información se mostrará en los comprobantes.
	</p>
	<!-- Reusamos el componente StepEmpresa -->
	<StepEmpresa
		bind:ruc
		bind:razonSocial
		bind:nombreComercial
		bind:departamento
		bind:direccion
		bind:telefono
		bind:email
		bind:ubigeo
		bind:usuarioSol
		bind:claveSol
		bind:certificadoPath
		bind:certificadoPassword
		bind:rucValid
		bind:rucValidating
		{validateRuc}
		onSelectCertificado={handleSelectCertificado}
		onSave={handleSave}
		{saving}
	>
		{#snippet messages()}
			{#if errorMsg}
				<div
					class="flex items-start gap-2 rounded-lg border border-red-500/20 bg-red-500/10 p-3 text-sm text-red-400"
				>
					<svg width="16" height="16" viewBox="0 0 16 16" fill="none" class="mt-0.5 shrink-0">
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
					class="flex items-start gap-2 rounded-lg border border-emerald-500/20 bg-emerald-500/10 p-3 text-sm text-emerald-400"
				>
					<svg width="16" height="16" viewBox="0 0 16 16" fill="none" class="mt-0.5 shrink-0">
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
		{/snippet}
	</StepEmpresa>
</div>
