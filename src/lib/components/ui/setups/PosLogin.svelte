<script lang="ts">
	import { IconLock, IconArrowLeft } from '@tabler/icons-svelte';
	import { apiClient, type Seller } from '$lib/services/apiClient';
	import { sellerAuth } from '$lib/stores/sellerAuth';
	import { configStore } from '$lib/stores/config';
	import Input from '$lib/components/core/primitives/Input.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';

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
			sellers = await apiClient.getSellers();
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
				username: selectedSeller.username,
				password_plain: password
			});
			// Success! The activeSeller store will update, and layout will hide PosLogin
		} catch (e: any) {
			loginError = 'Contraseña incorrecta';
		} finally {
			loggingIn = false;
		}
	}

	function getInitials(seller: Seller) {
		if (seller.first_name && seller.last_name) {
			return `${seller.first_name[0]}${seller.last_name[0]}`.toUpperCase();
		}
		return seller.username[0].toUpperCase();
	}

	function getName(seller: Seller) {
		if (seller.first_name) {
			return `${seller.first_name} ${seller.last_name || ''}`.trim();
		}
		return seller.username;
	}
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-neutral-950/95 backdrop-blur-md">
	<div class="flex w-full max-w-2xl flex-col rounded-2xl border border-neutral-800 bg-neutral-900/50 p-8 shadow-2xl">
		<div class="mb-8 text-center">
			<h1 class="text-3xl font-bold tracking-tight text-neutral-100">Taxorium POS</h1>
			<p class="mt-2 text-neutral-400">Selecciona tu usuario para iniciar turno</p>
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
				<div class="rounded-lg border border-amber-500/20 bg-amber-500/10 p-6 text-center text-amber-400">
					<p class="mb-2 font-medium">No hay vendedores registrados.</p>
					<p class="text-sm">El Administrador debe crear un vendedor en "Configuración > Administrar Cuentas" antes de poder usar la caja.</p>
				</div>
			{:else}
				<div class="grid grid-cols-2 gap-4 sm:grid-cols-3">
					{#each sellers as seller (seller.id)}
						<button
							onclick={() => selectSeller(seller)}
							class="group flex flex-col items-center justify-center gap-3 rounded-xl border border-neutral-800 bg-neutral-900 p-6 transition-all hover:border-blue-500 hover:bg-neutral-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
						>
							<div class="flex h-16 w-16 items-center justify-center rounded-full bg-blue-500/10 text-xl font-bold text-blue-400 transition-transform group-hover:scale-110">
								{getInitials(seller)}
							</div>
							<div class="text-center">
								<p class="font-medium text-neutral-200">{getName(seller)}</p>
								<p class="text-xs text-neutral-500">{seller.domain || 'Sin caja asignada'}</p>
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
					<div class="flex h-14 w-14 items-center justify-center rounded-full bg-blue-500/10 text-lg font-bold text-blue-400">
						{getInitials(selectedSeller)}
					</div>
					<div>
						<h2 class="text-xl font-bold text-neutral-200">{getName(selectedSeller)}</h2>
						<p class="text-sm text-neutral-500">{selectedSeller.username}</p>
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
</div>
