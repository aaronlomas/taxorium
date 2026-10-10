<script lang="ts">
	import { IconLock, IconArrowLeft } from '@tabler/icons-svelte';
	import { sellerClient, type Seller } from '$lib/features/sellers';
	import { sellerAuth } from '$lib/features/sellerAuth';
	import { configStore } from '$lib/integrations/tauri/nodeConfig';
	import Input from '$lib/components/core/primitives/forms/Input.svelte';
	import Button from '$lib/components/core/primitives/actions/Button.svelte';
	import Modal from '$lib/components/core/primitives/feedback/Modal.svelte';

	let sellers = $state<Seller[]>([]);
	let loading = $state(true);
	let errorMsg = $state('');

	let selectedSeller = $state<Seller | null>(null);
	let password = $state('');
	let loggingIn = $state(false);
	let loginError = $state('');

	// Reactively wait until the node role is configured before loading sellers
	$effect(() => {
		if ($configStore.role) {
			loadSellers();
		}
	});

	async function loadSellers() {
		loading = true;
		errorMsg = '';
		try {
			sellers = await sellerClient.getSellers();
		} catch (e: any) {
			errorMsg = e?.message || 'Error cargando los vendedores';
		} finally {
			loading = false;
		}
	}

	function selectSeller(seller: Seller) {
		selectedSeller = seller;
		password = '';
		loginError = '';
	}

	function cancelLogin() {
		selectedSeller = null;
		password = '';
		loginError = '';
	}

	async function handleLogin(e: SubmitEvent) {
		e.preventDefault();
		if (!selectedSeller || !password) return;

		loginError = '';
		loggingIn = true;

		try {
			await sellerAuth.login({
				usuario: selectedSeller.usuario,
				clave_plana: password
			});
			// Success! The activeSeller store will update, and layout will hide PosLogin
		} catch (e: any) {
			loginError = 'Contraseña incorrecta';
		} finally {
			loggingIn = false;
		}
	}

	function getInitials(seller: Seller) {
		if (seller.nombres && seller.apellidos) {
			return `${seller.nombres[0]}${seller.apellidos[0]}`.toUpperCase();
		}
		return seller.usuario[0].toUpperCase();
	}

	function getName(seller: Seller) {
		if (seller.nombres) {
			return `${seller.nombres} ${seller.apellidos || ''}`.trim();
		}
		return seller.usuario;
	}
</script>

<!--
	Login de caja: pantalla obligatoria para los nodos client sin vendedor autenticado.
	Usa el primitivo Modal para el backdrop y el arrastre con el mouse.
	closable={false} porque no se puede seguir operando la caja sin vendedor.
-->
<Modal isOpen variant="fullscreen" title="Taxorium POS" closable={false} onClose={() => {}}>
	<div class="p-8">
		<div class="mb-8 text-center">
			<p class="text-neutral-400">Selecciona tu usuario para iniciar turno</p>
		</div>

		{#if loading}
			<div class="flex h-48 items-center justify-center text-neutral-500">
				Cargando vendedores...
			</div>
		{:else if errorMsg}
			<div class="rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-center text-red-400">
				{errorMsg}
			</div>
		{:else if !selectedSeller}
			{#if sellers.length === 0}
				<div
					class="rounded-lg border border-amber-500/20 bg-amber-500/10 p-6 text-center text-amber-400"
				>
					<p class="mb-2 font-medium">No hay vendedores registrados.</p>
					<p class="text-sm">
						El Administrador debe crear un vendedor en "Configuración > Administrar Cuentas" antes
						de poder usar la caja.
					</p>
				</div>
			{:else}
				<div class="grid grid-cols-2 gap-4 sm:grid-cols-3">
					{#each sellers as seller (seller.id)}
						<button
							onclick={() => selectSeller(seller)}
							class="group flex flex-col items-center justify-center gap-3 rounded-xl border border-neutral-800 bg-neutral-900 p-6 transition-all hover:border-blue-500 hover:bg-neutral-800 focus:ring-2 focus:ring-blue-500 focus:outline-none"
						>
							<div
								class="flex h-16 w-16 items-center justify-center rounded-full bg-blue-500/10 text-xl font-bold text-blue-400 transition-transform group-hover:scale-110"
							>
								{getInitials(seller)}
							</div>
							<div class="text-center">
								<p class="font-medium text-neutral-200">{getName(seller)}</p>
								<p class="text-xs text-neutral-500">{seller.dominio || 'Sin caja asignada'}</p>
							</div>
						</button>
					{/each}
				</div>
			{/if}
		{:else}
			<div class="mx-auto w-full max-w-sm">
				<button
					onclick={cancelLogin}
					class="mb-6 flex items-center gap-2 text-sm text-neutral-400 transition-colors hover:text-neutral-200"
				>
					<IconArrowLeft size={16} />
					Volver a la lista
				</button>

				<div class="mb-6 flex items-center gap-4">
					<div
						class="flex h-14 w-14 items-center justify-center rounded-full bg-blue-500/10 text-lg font-bold text-blue-400"
					>
						{getInitials(selectedSeller)}
					</div>
					<div>
						<h2 class="text-xl font-bold text-neutral-200">{getName(selectedSeller)}</h2>
						<p class="text-sm text-neutral-500">{selectedSeller.usuario}</p>
					</div>
				</div>

				<form onsubmit={handleLogin} class="flex flex-col gap-4">
					<Input
						label="Contraseña"
						type="password"
						bind:value={password}
						placeholder="••••••••"
						autocomplete="current-password"
						required
						autofocus
					>
						{#snippet icon()}<IconLock size={18} />{/snippet}
					</Input>

					{#if loginError}
						<p class="text-sm text-red-400">{loginError}</p>
					{/if}

					<Button type="submit" disabled={loggingIn || !password} class="w-full">
						{loggingIn ? 'Ingresando...' : 'Entrar a Caja'}
					</Button>
				</form>
			</div>
		{/if}
	</div>
</Modal>
