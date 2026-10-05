<script lang="ts">
	import { untrack } from 'svelte';
	import { auth } from '$lib/stores/auth';
	import { currentTenant, tenantStore } from '$lib/stores/tenant';
	import FormCompany from '$lib/components/ui/views/account/company/FormCompany.svelte';
	import Modal from '$lib/components/core/primitives/Modal.svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { configLocalClient } from '$lib/services/configLocal/clientConfigLocal';

	let { isOpen = $bindable(), onClose }: { isOpen: boolean; onClose: () => void } = $props();

	let ruc = $state('');
	let razonSocial = $state('');
	let nombreComercial = $state('');
	let departamento = $state('');
	let direccion = $state('');
	let telefono = $state('');
	let email = $state('');

	// Campos SUNAT
	let ubigeo = $state('');
	let usuarioSol = $state('');
	let claveSol = $state('');

	// certificado_path y clave_cert viven en SQLite local (sin internet).
	let certificadoPath = $state('');
	let certificadoPassword = $state('');

	let rucValid = $state<boolean | null>(true);
	let rucValidating = $state(false);

	let saving = $state(false);
	let errorMsg = $state('');
	let successMsg = $state('');

	// Cada vez que se abre el modal recargamos los datos actuales del tenant
	// (puede haber llegado de Supabase después del primer montaje) y del SQLite local.
	$effect(() => {
		if (!isOpen) return;

		untrack(() => {
			ruc = $currentTenant?.ruc ?? '';
			razonSocial = $currentTenant?.razon_social ?? '';
			nombreComercial = $currentTenant?.nombre_comercial ?? '';
			departamento = $currentTenant?.departamento ?? '';
			direccion = $currentTenant?.direccion ?? '';
			telefono = $currentTenant?.telefono ?? '';
			email = $currentTenant?.email ?? '';
			ubigeo = $currentTenant?.ubigeo ?? '';
			usuarioSol = $currentTenant?.usuario_sol ?? '';
			claveSol = $currentTenant?.clave_sol ?? '';

			errorMsg = '';
			successMsg = '';
			rucValid = true;
			rucValidating = false;

			// Config local (SQLite, no necesita internet)
			(async () => {
				try {
					const cfg = await configLocalClient.getAll();
					certificadoPath = cfg.certificado_path ?? '';
					certificadoPassword = cfg.clave_cert ?? '';
				} catch {
					// Fallback si SQLite aún no tiene la tabla (primer arranque antes de reiniciar)
					certificadoPath =
						localStorage.getItem('taxorium_cert_path') ?? $currentTenant?.certificado_path ?? '';
					certificadoPassword = localStorage.getItem('taxorium_cert_pwd') ?? '';
				}
			})();
		});
	});

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
			// 1. Guardar datos de empresa en Supabase (requiere internet solo aquí)
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
					clave_sol: claveSol.trim() || null
				});
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
					configurado: true,
					activo: true
				});
				if (tenant) {
					await tenantStore.markConfigured(tenant.id);
				}
			}

			// 2. Guardar certificado y contraseña en SQLite LOCAL (independiente de internet)
			// Fallo aquí no impide guardar los datos de empresa — se avisa aparte.
			try {
				if (certificadoPath.trim()) {
					await configLocalClient.set('certificado_path', certificadoPath.trim());
				}
				if (certificadoPassword) {
					await configLocalClient.set('clave_cert', certificadoPassword);
				}
			} catch (localErr) {
				console.warn('No se pudo guardar la config local en SQLite:', localErr);
				// Fallback: localStorage
				if (certificadoPassword) localStorage.setItem('taxorium_cert_pwd', certificadoPassword);
				if (certificadoPath) localStorage.setItem('taxorium_cert_path', certificadoPath);
			}

			successMsg = 'Configuración guardada correctamente.';
		} catch (err: unknown) {
			// Los errores de Supabase (PostgrestError) no son instancias de Error nativo
			// pero siempre tienen una propiedad .message
			const extractMsg = (e: unknown): string => {
				if (e instanceof Error) return e.message;
				if (e && typeof e === 'object' && 'message' in e)
					return String((e as { message: unknown }).message);
				if (typeof e === 'string') return e;
				return JSON.stringify(e);
			};
			const msg = extractMsg(err);
			if (msg.includes('duplicate') || msg.includes('unique')) {
				errorMsg = 'Ya existe una empresa registrada con ese RUC.';
			} else {
				errorMsg = `Error: ${msg}`;
			}
		} finally {
			saving = false;
			if (successMsg) {
				setTimeout(() => {
					successMsg = '';
				}, 3000);
			}
		}
	}
</script>

<Modal bind:isOpen {onClose} title="Configuración de Empresa">
	<div class="max-h-[80vh] overflow-y-auto">
		<p class="px-2 pt-2 text-sm leading-relaxed text-neutral-300">
			Esta información se mostrará en los comprobantes.
		</p>
		<FormCompany
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
		</FormCompany>
	</div>
</Modal>
