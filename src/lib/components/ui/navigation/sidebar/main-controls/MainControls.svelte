<script lang="ts">
	import {
		IconBook2,
		IconUserCircle,
		IconSettings,
		IconLogin,
		IconUserPlus,
		IconLogout,
		IconUserSquare,
		IconDeviceDesktopPin,
		IconAdjustments
	} from '@tabler/icons-svelte';
	import { isAuthenticated, isLoading, auth } from '$lib/features/auth';
	import { sellerAuth } from '$lib/features/sellerAuth';
	import { currentTenant, tenantLoading } from '$lib/features/tenant';
	import Modal from '$lib/components/core/primitives/feedback/Modal.svelte';
	import Dropdown from '$lib/components/core/primitives/navigation/dropdown/Dropdown.svelte';
	import DropdownItem from '$lib/components/core/primitives/navigation/dropdown/DropdownItem.svelte';
	import DropdownDivider from '$lib/components/core/primitives/navigation/dropdown/DropdownDivider.svelte';
	import PanelLogin from '$lib/components/ui/features/auth/setups/PanelLogin.svelte';
	import type { ElementoNavegacion } from '../navigation';

	let {
		seccionActiva,
		onCambiarSeccion,
		onSeleccionarOpcion
	}: {
		seccionActiva: 'registros' | null;
		onCambiarSeccion?: (seccion: 'registros') => void;
		onSeleccionarOpcion?: (opcion: ElementoNavegacion) => void;
	} = $props();

	// Estado de configuración de la cuenta
	let authPendiente = $derived(!$isAuthenticated && !$isLoading);
	let tenantPendiente = $derived($isAuthenticated && !$currentTenant && !$tenantLoading);
	let cuentaPendiente = $derived(authPendiente || tenantPendiente);

	// Modales
	let modalLoginOpen = $state(false);
	let modalRegisterOpen = $state(false);

	function cambiarSeccion(seccion: 'registros') {
		onCambiarSeccion?.(seccion);
	}

	function abrirLogin() {
		modalLoginOpen = true;
	}

	function abrirRegistro() {
		modalRegisterOpen = true;
	}

	function abrirMiCuenta() {
		onSeleccionarOpcion?.({ nombre: 'Mi Cuenta', icono: IconUserCircle });
	}

	function abrirPuntosVenta() {
		onSeleccionarOpcion?.({ nombre: 'Mis Puntos de Venta', icono: IconDeviceDesktopPin });
	}

	async function cerrarSesion() {
		if ($sellerAuth) {
			sellerAuth.logout();
		} else {
			await auth.signOut();
		}
	}

	// Auto-apertura única al montar si falta autenticar.
	// La falta de tenant NO abre ninguna pestaña: se señala con el punto ámbar
	// del icono de usuario, y el usuario la completa desde Mi Cuenta → Editar Información.
	let hasAutoOpened = $state(false);
	$effect(() => {
		if ($isLoading || $tenantLoading) return;
		if (hasAutoOpened) return;

		if (authPendiente) {
			hasAutoOpened = true;
			modalLoginOpen = true;
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
		<!-- SECCIÓN DE REGISTROS GENERALES -->
		<button
			type="button"
			disabled={cuentaPendiente}
			class="flex cursor-pointer justify-center p-2 text-neutral-400 transition-colors {cuentaPendiente
				? 'cursor-not-allowed opacity-30'
				: 'hover:text-white'} {seccionActiva === 'registros'
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
	<div class="relative grid justify-center">
		<!-- CUENTA DE USUARIO con dropdown -->
		<Dropdown
			id="mc-btn-cuenta"
			side="right"
			align="end"
			title={$isAuthenticated ? 'Mi cuenta' : 'Cuenta'}
			aria-label="Panel de cuenta"
			class="flex cursor-pointer justify-center p-2 text-neutral-400 transition-colors hover:text-white"
		>
			{#snippet trigger()}
				<IconUserCircle size={28} class="pointer-events-none" />
				<!-- Indicador de pendiente -->
				{#if cuentaPendiente}
					<span class="pending-dot" aria-hidden="true"></span>
				{/if}
			{/snippet}

			{#snippet menu()}
				{#if $isAuthenticated || $sellerAuth}
					<!-- Usuario autenticado: cuenta y cerrar sesión -->
					{#if !$sellerAuth}
						<DropdownItem onclick={abrirMiCuenta}>
							<IconUserCircle size={15} stroke={1.5} />
							Mi Cuenta
						</DropdownItem>
						<DropdownItem onclick={abrirPuntosVenta}>
							<IconDeviceDesktopPin size={15} stroke={1.5} />
							Mis Puntos de Venta
						</DropdownItem>
					{/if}
					<DropdownItem variant="danger" onclick={cerrarSesion}>
						<IconLogout size={15} stroke={1.5} />
						Cerrar Sesión
					</DropdownItem>
				{:else}
					<!-- No autenticado: iniciar sesión o crear cuenta -->
					<DropdownItem onclick={abrirLogin}>
						<IconLogin size={15} stroke={1.5} />
						Iniciar Sesión
					</DropdownItem>
					<DropdownDivider />
					<DropdownItem onclick={abrirRegistro}>
						<IconUserPlus size={15} stroke={1.5} />
						Crear Cuenta
					</DropdownItem>
					<DropdownItem onclick={abrirMiCuenta}>
						<IconUserSquare size={15} stroke={1.5} />
						Mi Cuenta
					</DropdownItem>
				{/if}
			{/snippet}
		</Dropdown>

		{#if !$sellerAuth}
			<Dropdown
				side="right"
				align="end"
				title="Configuraciones"
				aria-label="Panel de configuraciones"
				class="flex w-full cursor-pointer items-center gap-2 p-2 text-sm text-neutral-400 transition-colors hover:text-white"
			>
				{#snippet trigger()}
					<IconSettings size={28} />
				{/snippet}

				{#snippet menu()}
					<DropdownItem
						onclick={() => onSeleccionarOpcion?.({ nombre: 'Configuración', icono: IconSettings })}
					>
						<IconSettings size={15} stroke={1.5} />
						Configuración
					</DropdownItem>
					<DropdownItem
						onclick={() =>
							onSeleccionarOpcion?.({ nombre: 'Preferencias', icono: IconAdjustments })}
					>
						<IconAdjustments size={15} stroke={1.5} />
						Preferencias
					</DropdownItem>
				{/snippet}
			</Dropdown>
		{/if}
	</div>
</div>

<!-- Modal: Iniciar Sesión -->
<Modal bind:isOpen={modalLoginOpen} onClose={() => (modalLoginOpen = false)} title="Iniciar Sesión">
	<div class="modal-form-wrap">
		<PanelLogin initialMode="login" onSuccess={() => (modalLoginOpen = false)} />
	</div>
</Modal>

<!-- Modal: Crear Cuenta -->
<Modal
	bind:isOpen={modalRegisterOpen}
	onClose={() => (modalRegisterOpen = false)}
	title="Crear Cuenta"
>
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
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.6;
			transform: scale(0.85);
		}
	}
</style>
