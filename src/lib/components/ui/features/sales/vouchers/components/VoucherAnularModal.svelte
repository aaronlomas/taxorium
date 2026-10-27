<script lang="ts">
	import Modal from '$lib/components/core/primitives/feedback/Modal.svelte';
	import InputTextarea from '$lib/components/core/primitives/forms/InputTextarea.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { get } from 'svelte/store';
	import { tenantStore } from '$lib/features/tenant';
	import { readFile } from '@tauri-apps/plugin-fs';
	import { configLocalClient } from '$lib/integrations/tauri/deviceConfig';
	import type { Voucher } from '$lib/features/vouchers';

	type AnulacionStep = 'motivo' | 'resultado';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		voucher: Voucher | null;
		onSuccess?: () => void;
	}

	let {
		isOpen = $bindable(false),
		onClose,
		voucher,
		onSuccess
	}: Props = $props();

	let step = $state<AnulacionStep>('motivo');
	let motivo = $state('');
	let motivoError = $state('');
	let resultado = $state<{ ok: boolean; mensaje: string } | null>(null);
	let cargando = $state(false);

	$effect(() => {
		if (isOpen) {
			step = 'motivo';
			motivo = '';
			motivoError = '';
			resultado = null;
			cargando = false;
		}
	});

	async function confirmarAnulacion() {
		if (!voucher) return;
		if (!motivo.trim()) {
			motivoError = 'El motivo es obligatorio.';
			return;
		}
		motivoError = '';
		cargando = true;

		try {
			const tenant = get(tenantStore).tenant;
			if (!tenant || !tenant.ruc || !tenant.usuario_sol || !tenant.clave_sol) {
				throw new Error('Faltan las credenciales SOL de la empresa.');
			}

			// Leer certificado desde config local
			let certPath: string;
			let p12Password: string;
			try {
				const cfg = await configLocalClient.getAll();
				certPath = cfg.certificado_path ?? tenant?.certificado_path ?? '';
				p12Password = cfg.clave_cert ?? localStorage.getItem('taxorium_cert_pwd') ?? '';
			} catch {
				certPath = tenant?.certificado_path ?? '';
				p12Password = localStorage.getItem('taxorium_cert_pwd') ?? '';
			}
			if (!certPath) throw new Error('La empresa no tiene un certificado configurado.');

			const p12Bytes = await readFile(certPath);

			// Enviar Comunicación de Baja (VoidedDocuments) ante SUNAT
			const correlativoBaja = Math.floor(Date.now() / 1000) % 99999;
			const res = await invoke<{ ticket: string; codigo: string; descripcion: string }>(
				'enviar_baja_sunat',
				{
					payload: {
						ruc: tenant.ruc,
						usuario_sol: tenant.usuario_sol,
						clave_sol: tenant.clave_sol,
						client_id: (tenant as any).sunat_client_id || '',
						client_secret: (tenant as any).sunat_client_secret || '',
						ambiente: 'beta',
						p12_bytes: Array.from(p12Bytes),
						p12_password: p12Password,
						voucher_id: voucher.id,
						motivo: motivo.trim(),
						emisor_razon_social: tenant.razon_social,
						correlativo_baja: correlativoBaja
					}
				}
			);
			const aceptado = res.codigo === '0';
			resultado = {
				ok: aceptado,
				mensaje: aceptado
					? `✓ SUNAT aceptó la Comunicación de Baja. Ticket: ${res.ticket}. ${res.descripcion}`
					: `✕ SUNAT procesó la baja (código ${res.codigo}): ${res.descripcion}`
			};

			onSuccess?.();
		} catch (e) {
			const msg = e instanceof Error ? e.message : typeof e === 'string' ? e : JSON.stringify(e);
			resultado = { ok: false, mensaje: `Error al anular ante SUNAT: ${msg}` };
		} finally {
			cargando = false;
			step = 'resultado';
		}
	}
</script>

<Modal
	bind:isOpen
	{onClose}
	title={step === 'motivo' ? 'Anular Comprobante' : 'Resultado de Anulación'}
	closable={!cargando}
>
	<div class="w-120 p-4">
		{#if step === 'motivo'}
			<!-- Paso 1: Ingresar motivo -->
			<div
				class="mb-3 rounded-sm border border-rose-500/30 bg-rose-950/30 px-3 py-2 text-xs text-rose-300"
			>
				<strong>Atención:</strong> La anulación marcará el comprobante como anulado en el sistema.
				Ante SUNAT deberá gestionar una <strong>Comunicación de Baja</strong> (dentro de los 7 días) o
				una <strong>Nota de Crédito</strong> según el tipo de comprobante.
			</div>

			{#if voucher}
				<p class="mb-3 text-sm text-neutral-400">
					Comprobante: <span class="font-semibold text-neutral-200"
						>{voucher.numero_comprobante}</span
					>
				</p>
			{/if}

			<InputTextarea
				label="Motivo de anulación"
				bind:value={motivo}
				error={motivoError}
				placeholder="Describa el motivo de la anulación..."
				rows={4}
				required
				disabled={cargando}
			/>

			<div class="mt-4 flex items-center justify-end gap-2">
				<button
					type="button"
					onclick={onClose}
					disabled={cargando}
					class="rounded-sm border border-neutral-700 px-4 py-1.5 text-sm text-neutral-300 transition-colors hover:bg-neutral-800 disabled:opacity-50"
				>
					Cancelar
				</button>
				<button
					type="button"
					onclick={confirmarAnulacion}
					disabled={cargando}
					class="flex items-center gap-2 rounded-sm border border-rose-500/50 bg-rose-950/40 px-4 py-1.5 text-sm text-rose-300 transition-colors hover:bg-rose-900/50 disabled:opacity-50"
				>
					{#if cargando}
						<span
							class="inline-block size-3.5 animate-spin rounded-full border-2 border-rose-400 border-t-transparent"
						></span>
						Procesando...
					{:else}
						Confirmar Anulación
					{/if}
				</button>
			</div>
		{:else if step === 'resultado' && resultado}
			<!-- Paso 2: Mostrar resultado -->
			<div
				class="mb-4 rounded-sm border px-3 py-3 text-sm {resultado.ok
					? 'border-emerald-500/30 bg-emerald-950/30 text-emerald-300'
					: 'border-red-500/30 bg-red-950/30 text-red-300'}"
			>
				<p class="font-semibold">{resultado.ok ? '✓ Anulación Registrada' : '✕ Error'}</p>
				<p class="mt-1 text-xs opacity-90">{resultado.mensaje}</p>
			</div>

			<div class="flex justify-end">
				<button
					type="button"
					onclick={onClose}
					class="rounded-sm border border-neutral-700 px-4 py-1.5 text-sm text-neutral-300 transition-colors hover:bg-neutral-800"
				>
					Aceptar
				</button>
			</div>
		{/if}
	</div>
</Modal>
