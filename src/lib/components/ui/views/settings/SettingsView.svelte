<script lang="ts">
	import { auth } from '$lib/stores/auth';
	import { currentTenant, tenantStore } from '$lib/stores/tenant';
	import StepEmpresa from '$lib/components/setups/StepEmpresa.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';

	let ruc = $state($currentTenant?.ruc ?? '');
	let razonSocial = $state($currentTenant?.razon_social ?? '');
	let nombreComercial = $state($currentTenant?.nombre_comercial ?? '');
	let departamento = $state($currentTenant?.departamento ?? '');
	let direccion = $state($currentTenant?.direccion ?? '');
	let telefono = $state($currentTenant?.telefono ?? '');
	let email = $state($currentTenant?.email ?? '');
	
	let rucValid = $state<boolean | null>(true);
	let rucValidating = $state(false);
	
	let saving = $state(false);
	let errorMsg = $state('');
	let successMsg = $state('');

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

	async function handleSave() {
		errorMsg = '';
		successMsg = '';
		
		if (!ruc || !isValidRuc(ruc)) { errorMsg = 'Ingresa un RUC válido (11 dígitos).'; return; }
		if (!razonSocial.trim()) { errorMsg = 'La razón social es obligatoria.'; return; }
		if (!direccion.trim()) { errorMsg = 'La dirección fiscal es obligatoria.'; return; }

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
					email: email.trim() || null
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
					configurado: true,
					activo: true
				});
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
				setTimeout(() => { successMsg = ''; }, 3000);
			}
		}
	}
</script>

<div class="flex flex-col overflow-scroll h-full">
	<div class="settings-header">
		<div>
			<h2 class="text-xl font-bold text-slate-100 tracking-tight">Configuración de empresa</h2>
			<p class="text-sm text-slate-400 mt-1">Gestiona los datos de tu empresa para la emisión de comprobantes</p>
		</div>
	</div>

	<div class="p-2">
		<div class="max-w-3xl">
			<!-- Reusamos el componente StepEmpresa -->
			<div class="bg-neutral-900 border border-neutral-800 rounded-xl p-2 shadow-sm">
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

				{#if errorMsg}
					<div class="mt-4 flex items-start gap-2 bg-red-500/10 border border-red-500/20 text-red-400 p-3 rounded-lg text-sm">
						<svg width="16" height="16" viewBox="0 0 16 16" fill="none" class="shrink-0 mt-0.5">
							<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5" />
							<path d="M8 5v4M8 11v.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
						</svg>
						{errorMsg}
					</div>
				{/if}

				{#if successMsg}
					<div class="mt-4 flex items-start gap-2 bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 p-3 rounded-lg text-sm">
						<svg width="16" height="16" viewBox="0 0 16 16" fill="none" class="shrink-0 mt-0.5">
							<circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1.5" />
							<path d="M5 8l2 2 4-4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
						</svg>
						{successMsg}
					</div>
				{/if}

				<div class="mt-8 flex justify-end">
					<Button type="button" variant="primary" onclick={handleSave} disabled={saving} class="px-6 py-2.5">
						{#if saving}
							<span class="inline-block w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin mr-2"></span>
							Guardando...
						{:else}
							Guardar cambios
						{/if}
					</Button>
				</div>
			</div>
		</div>
	</div>
</div>