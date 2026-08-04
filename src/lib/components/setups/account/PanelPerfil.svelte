<script lang="ts">
	import { currentUser } from '$lib/stores/auth';
	import { currentTenant } from '$lib/stores/tenant';
	import { licenseExpiry } from '$lib/stores/license';
	import { IconKey, IconMail, IconBuilding } from '@tabler/icons-svelte';

	function formatDate(date: Date | null): string {
		if (!date) return '—';
		return date.toLocaleDateString('es-PE', { day: '2-digit', month: 'short', year: 'numeric' });
	}
</script>

<div
	class="m-5 grid grid-cols-2 items-center justify-center gap-4 rounded-xl border border-neutral-700 p-4 overflow-scroll"
>
	<!-- EMPRESA -->
	<!-- DETALLES -->
	<div>
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
					{($currentTenant.razon_social).charAt(0).toUpperCase()}
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
				<span>Ej: http://198.168.1.2/root</span>
			</div>
		</div>
	</div>

	<!-- PERFIL -->
	<div>
		<div class="m-auto size-52 rounded-full bg-green-700 text-center">Foto.png</div>
	</div>
</div>
