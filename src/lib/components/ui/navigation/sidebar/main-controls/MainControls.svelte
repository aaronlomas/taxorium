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
	import { activeOption } from '$lib/features/navigation';

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
		const opcion = { nombre: 'Mi Cuenta', icono: IconUserCircle };
		activeOption.set(opcion);
		onSeleccionarOpcion?.(opcion);
	}

	function abrirPuntosVenta() {
		const opcion = { nombre: 'Mis Puntos de Venta', icono: IconDeviceDesktopPin };
		activeOption.set(opcion);
		onSeleccionarOpcion?.(opcion);
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
	class="row-span-3 flex flex-col justify-between border-r border-brand-border {seccionActiva
		? ''
		: 'border-r-0'}"
>
	<!-- TRABAJO -->
	<div class="grid justify-center">
		<!-- SECCIÓN DE REGISTROS GENERALES -->
		<button
			type="button"
			disabled={cuentaPendiente}
			class="flex cursor-pointer justify-center p-2 text-brand-text-secondary transition-colors {cuentaPendiente
				? 'cursor-not-allowed opacity-30'
				: 'hover:text-brand-text-hover'} {seccionActiva === 'registros'
				? 'border-l-2 border-l-blue-500'
				: ''}"
			aria-label="Toggle Panel Registros"
			onclick={() => cambiarSeccion('registros')}
		>
			<IconBook2
				size={28}
				class="pointer-events-none {seccionActiva === 'registros' ? 'text-brand-text-primary' : ''}"
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
			aria-label="Panel de cuenta"
			class="group flex w-full cursor-pointer items-center justify-center p-2 text-sm text-brand-text-secondary hover:text-brand-text-primary"
		>
			{#snippet trigger()}
				<IconUserCircle
					size={28}
					class="pointer-events-none text-brand-text-secondary transition-colors group-hover:text-brand-text-primary {$activeOption?.nombre ===
						'Mi Cuenta' || $activeOption?.nombre === 'Mis Puntos de Venta'
						? 'text-brand-text-primary'
						: ''}"
				/>
				<!-- Indicador de pendiente -->
				{#if cuentaPendiente}
					<span
						class="pointer-events-none absolute top-1.5 right-1.5 size-1.75 animate-pulse rounded-full bg-amber-500 shadow-[0_0_6px_#f59e0b]"
						aria-hidden="true"
					></span>
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
				class="group flex w-full cursor-pointer items-center justify-center p-2 text-sm text-brand-text-secondary hover:text-brand-text-primary"
			>
				{#snippet trigger()}
					<IconSettings
						size={28}
						class="text-brand-text-secondary transition-colors group-hover:text-brand-text-primary {$activeOption?.nombre ===
							'Configuración' || $activeOption?.nombre === 'Preferencias'
							? 'text-brand-text-primary'
							: ''}"
					/>
				{/snippet}

				{#snippet menu()}
					<DropdownItem
						onclick={() => {
							const opcion = { nombre: 'Configuración', icono: IconSettings };
							activeOption.set(opcion);
							onSeleccionarOpcion?.(opcion);
						}}
					>
						<IconSettings size={15} stroke={1.5} />
						Configuración
					</DropdownItem>
					<DropdownItem
						onclick={() => {
							const opcion = { nombre: 'Preferencias', icono: IconAdjustments };
							activeOption.set(opcion);
							onSeleccionarOpcion?.(opcion);
						}}
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
<Modal bind:isOpen={modalLoginOpen} onClose={() => (modalLoginOpen = false)}>
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
