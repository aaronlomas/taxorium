<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { currentUser } from '$lib/features/auth';
	import { currentTenant } from '$lib/features/tenant';
	import { configStore } from '$lib/integrations/tauri/nodeConfig';
	import { licenseExpiry } from '$lib/features/license';
	import { IconKey, IconMail, IconBuilding, IconEdit } from '@tabler/icons-svelte';
	import ModalCompany from '$lib/components/ui/views/account/company/ModalCompany.svelte';

	let modalCompanyOpen = $state(false);
	let dominioAsignado = $state('…');

	function formatDate(date: Date | null): string {
		if (!date) return '—';
		return date.toLocaleDateString('es-PE', { day: '2-digit', month: 'short', year: 'numeric' });
	}

	function conProtocolo(direccion: string): string {
		return direccion.startsWith('http://') || direccion.startsWith('https://')
			? direccion
			: `http://${direccion}`;
	}

	$effect(() => {
		const rol = $configStore.role;
		const ipServidor = $configStore.server_ip;

		if (rol === 'client') {
			dominioAsignado = ipServidor ? conProtocolo(ipServidor) : '—';
			return;
		}
		if (rol !== 'server') {
			dominioAsignado = '—';
			return;
		}

		let vivo = true;
		invoke<string>('get_local_ip')
			.then((direccion) => {
				if (vivo) dominioAsignado = conProtocolo(direccion);
			})
			.catch((e) => {
				console.error('No se pudo obtener la IP local:', e);
				if (vivo) dominioAsignado = '—';
			});

		return () => {
			vivo = false;
		};
	});
</script>

<div class="grid grid-cols-2 gap-2 overflow-scroll rounded-xl border border-neutral-800 p-2">
	<!-- EMPRESA -->
	<!-- DETALLES -->
	<div class="grid">
		{#if $currentTenant}
			<div class="text-sm">
				<p class="text-blue-400">Empresa</p>
				{#if $currentTenant.direccion}
					<div class="flex w-full items-center gap-x-2">
						<IconBuilding size={14} />
						<span class="text-sm">{$currentTenant.direccion}</span>
					</div>
				{/if}
			</div>
		{/if}
		{#if $currentTenant}
			<div>
				<div>
					{$currentTenant.razon_social.charAt(0).toUpperCase()}
				</div>
				<div class="text-sm">
					<p>{$currentTenant.razon_social}</p>
					{#if $currentTenant.nombre_comercial}
						<p>{$currentTenant.nombre_comercial}</p>
					{/if}
					<p>RUC {$currentTenant.ruc}</p>
				</div>
			</div>
		{/if}

		<!-- Datos de la sesión -->
		<div class="text-sm">
			<h1 class="text-blue-400">Usuario</h1>
			<div class="flex w-full items-center gap-x-2 text-sm">
				<IconMail size={14} />
				<span>{$currentUser?.email ?? '—'}</span>
			</div>
		</div>

		<!-- Licencia -->
		<div class="text-sm">
			<p class="text-blue-400">Licencia</p>
			<!-- Mientras que el usuario no desbloquee todas las funciones del sistema, el estado de licencia debe mostrar solo "Registrada" no "Activa" -->
			<div class="flex w-full items-center gap-x-2 text-sm">
				<IconKey size={14} />
				<span>Registrada</span>
				{#if $licenseExpiry}
					<span>Vence {formatDate($licenseExpiry)}</span>
				{/if}
			</div>
		</div>

		<div>
			<div>
				<h1 class="text-sm text-blue-400">Información de Usuario:</h1>
				<span class="text-sm">Administrador</span>
			</div>
			<div class="text-sm">
				<h1 class="text-blue-400">Dominio Asignado:</h1>
				<span class="text-sm">{dominioAsignado}</span>
			</div>
		</div>
	</div>

	<!-- PERFIL -->
	<div
		class="grid h-full items-center justify-center overflow-scroll rounded-md border border-neutral-800"
	>
		<div
			class="flex size-52 items-end justify-center overflow-hidden rounded-full border-2 border-neutral-700 text-center"
		>
			<button
				class="flex w-full cursor-pointer justify-center border-t border-neutral-800 bg-transparent hover:text-green-400"
				><IconEdit size={20} /></button
			>
		</div>
		<div class="text-center">
			<button class="cursor-pointer hover:text-white" onclick={() => (modalCompanyOpen = true)}>
				Editar Información
			</button>
		</div>
	</div>
</div>

<!-- Modal: Configuración de Empresa -->
<ModalCompany bind:isOpen={modalCompanyOpen} onClose={() => (modalCompanyOpen = false)} />
