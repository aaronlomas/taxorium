<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { IconBook2, IconCalculator, IconUserCircle, IconSettings, IconLogin, IconUserPlus, IconLogout } from '@tabler/icons-svelte';
	import { isAuthenticated, isLoading, auth } from '$lib/stores/auth';
	import { currentTenant, tenantLoading } from '$lib/stores/tenant';
	import { licenseValid } from '$lib/stores/license';
	import Modal from '$lib/components/core/primitives/Modal.svelte';
	import PanelLogin from '$lib/components/setups/PanelLogin.svelte';

	let {
		seccionActiva
	}: { seccionActiva: 'operaciones' | 'registros' | null } = $props();

	const dispatch = createEventDispatcher();

	// Estado de configuración de la cuenta
	let authPendiente = $derived(!$isAuthenticated && !$isLoading);
	let tenantPendiente = $derived($isAuthenticated && !$currentTenant && !$tenantLoading);
	let cuentaPendiente = $derived(authPendiente || tenantPendiente);

	// Modales
	let modalLoginOpen = $state(false);
	let modalRegisterOpen = $state(false);

	// Hover dropdown
	let cuentaHovered = $state(false);
	let dropdownHovered = $state(false);
	let dropdownVisible = $derived(cuentaHovered || dropdownHovered);

	function cambiarSeccion(seccion: 'operaciones' | 'registros') {
		dispatch('cambiarSeccion', seccion);
	}

	function abrirLogin() {
		dropdownHovered = false;
		cuentaHovered = false;
		modalLoginOpen = true;
	}

	function abrirRegistro() {
		dropdownHovered = false;
		cuentaHovered = false;
		modalRegisterOpen = true;
	}

	async function cerrarSesion() {
		dropdownHovered = false;
		cuentaHovered = false;
		await auth.signOut();
	}

	// Auto-apertura única al montar si falta configurar
	let hasAutoOpened = $state(false);
	$effect(() => {
		if ($isLoading || $tenantLoading) return;
		if (hasAutoOpened) return;

		if (authPendiente) {
			hasAutoOpened = true;
			modalLoginOpen = true;
		} else if (tenantPendiente) {
			hasAutoOpened = true;
			dispatch('seleccionarOpcion', { nombre: 'Configuración', icono: IconSettings });
		}
	});
</script>

<div
	class="row-span-3 flex flex-col justify-between border-r border-neutral-800 {seccionActiva
		? ''
		: 'border-r-0'}"
>
	<!-- TRABAJO -->
	<div class="grid justify-center">
		<!-- SECCIÓN DE OPERACIONES -->
		<button
			type="button"
			disabled={cuentaPendiente}
			class="flex cursor-pointer justify-center p-2 text-neutral-400 transition-colors {cuentaPendiente ? 'opacity-30 cursor-not-allowed' : 'hover:text-white'} {seccionActiva ===
			'operaciones'
				? 'border-l-2 border-l-blue-500 bg-blue-500/15'
				: ''}"
			title={cuentaPendiente ? 'Completa tu configuración primero' : 'Venta Rápida'}
			aria-label="Toggle Panel Vender"
			onclick={() => cambiarSeccion('operaciones')}
		>
			<IconCalculator
				size={28}
				class="pointer-events-none {seccionActiva === 'operaciones' ? 'text-white' : ''}"
			/>
		</button>

		<!-- SECCIÓN DE REGISTROS GENERALES -->
		<button
			type="button"
			disabled={cuentaPendiente}
			class="flex cursor-pointer justify-center p-2 text-neutral-400 transition-colors {cuentaPendiente ? 'opacity-30 cursor-not-allowed' : 'hover:text-white'} {seccionActiva ===
			'registros'
				? 'border-l-2 border-l-blue-500 bg-blue-500/15'
				: ''}"
			title={cuentaPendiente ? 'Completa tu configuración primero' : 'Registros'}
			aria-label="Toggle Panel Registros"
			onclick={() => cambiarSeccion('registros')}
		>
			<IconBook2
				size={28}
				class="pointer-events-none {seccionActiva === 'registros' ? 'text-white' : ''}"
			/>
		</button>
	</div>

	<!-- ACCESO Y CONFIGURACIONES -->
	<div class="grid justify-center relative">
		<!-- CUENTA DE USUARIO con hover-dropdown -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="relative"
			onmouseenter={() => (cuentaHovered = true)}
			onmouseleave={() => (cuentaHovered = false)}
		>
			<button
				id="mc-btn-cuenta"
				type="button"
				class="flex cursor-pointer justify-center p-2 text-neutral-400 hover:text-white transition-colors"
				title={$isAuthenticated ? 'Mi cuenta' : 'Cuenta'}
				aria-label="Panel de cuenta"
				onclick={() => dispatch('seleccionarOpcion', { nombre: 'Cuenta', icono: IconUserCircle })}
			>
				<IconUserCircle size={28} class="pointer-events-none" />
				<!-- Indicador de pendiente -->
				{#if cuentaPendiente}
					<span class="pending-dot" aria-hidden="true"></span>
				{/if}
			</button>

			<!-- Dropdown de opciones -->
			{#if dropdownVisible}
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div
					class="account-dropdown absolute bottom-0 left-full z-20 border border-neutral-900 bg-neutral-900 gap-x-2 items-center text-sm"
					onmouseenter={() => (dropdownHovered = true)}
					onmouseleave={() => (dropdownHovered = false)}
				>
					{#if $isAuthenticated}
						<!-- Usuario autenticado: solo cerrar sesión -->
						<button type="button" class="w-full flex items-center hover:bg-red-700 whitespace-nowrap gap-x-2 p-2" onclick={cerrarSesion}>
							<IconLogout size={15} stroke={1.5} />
							Cerrar Sesión
						</button>
					{:else}
						<!-- No autenticado: iniciar sesión o crear cuenta -->
						<button type="button" class="w-full whitespace-nowrap flex items-center gap-x-2 p-2 hover:bg-neutral-800" onclick={abrirLogin}>
							<IconLogin size={15} stroke={1.5} />
							Iniciar Sesión
						</button>
						<div class="dropdown-divider"></div>
						<button type="button" class="w-full whitespace-nowrap flex items-center gap-x-2 p-2 hover:bg-neutral-800" onclick={abrirRegistro}>
							<IconUserPlus size={15} stroke={1.5} />
							Crear Cuenta
						</button>
					{/if}
				</div>
			{/if}
		</div>

		<button
			type="button"
			class="flex w-full cursor-pointer items-center gap-2 p-2 text-sm text-neutral-400 transition-colors hover:text-white"
			onclick={() => dispatch('seleccionarOpcion', { nombre: 'Configuración', icono: IconSettings })}
		>
			<IconSettings size={28} />
		</button>
	</div>
</div>

<!-- Modal: Iniciar Sesión -->
<Modal bind:isOpen={modalLoginOpen} onClose={() => (modalLoginOpen = false)} title="Iniciar Sesión">
	<div class="modal-form-wrap">
		<PanelLogin initialMode="login" onSuccess={() => (modalLoginOpen = false)} />
	</div>
</Modal>

<!-- Modal: Crear Cuenta -->
<Modal bind:isOpen={modalRegisterOpen} onClose={() => (modalRegisterOpen = false)} title="Crear Cuenta">
	<div class="modal-form-wrap">
		<PanelLogin initialMode="register" onSuccess={() => (modalRegisterOpen = false)} />
	</div>
</Modal>

<style>
	.pending-dot {
		position: absolute;
		top: 6px;
		right: 6px;
		width: 7px;
		height: 7px;
		border-radius: 50%;
		background: #f59e0b;
		box-shadow: 0 0 6px #f59e0b;
		animation: pulse-pending 2s ease-in-out infinite;
		pointer-events: none;
	}

	@keyframes pulse-pending {
		0%, 100% { opacity: 1; transform: scale(1); }
		50% { opacity: 0.6; transform: scale(0.85); }
	}

	/* Dropdown */
	.account-dropdown {
		box-shadow: 0 0 10px #00000052;
		animation: dropdown-in 0.12s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes dropdown-in {
		from { opacity: 0; transform: translateX(-4px) scale(0.97); }
		to   { opacity: 1; transform: translateX(0) scale(1); }
	}
</style>
