<script lang="ts">
	import { goto } from '$app/navigation';
	import { configStore } from '$lib/integrations/tauri/nodeConfig';
	import { IconServer, IconDeviceDesktop } from '@tabler/icons-svelte';

	let step = $state(1);
	let serverIp = $state('');
	let error = $state('');

	async function selectServer() {
		try {
			await configStore.setConfig('server');
			await configStore.initServerDb();
			goto('/app');
		} catch (e) {
			error = 'Error al configurar como servidor principal';
			console.error(e);
		}
	}

	function selectClient() {
		step = 2;
	}

	async function saveClient() {
		if (!serverIp.trim()) {
			error = 'Por favor, ingrese la IP del servidor';
			return;
		}
		try {
			await configStore.setConfig('client', serverIp);
			goto('/app');
		} catch (e) {
			error = 'Error al guardar la configuración del cliente';
			console.error(e);
		}
	}
</script>

<svelte:head>
	<title>Configuración de Red - Taxorium</title>
</svelte:head>

<div
	class="flex h-full w-full flex-col items-center justify-center bg-neutral-950 p-8 text-neutral-100"
>
	<div class="w-full max-w-3xl space-y-8">
		<div class="space-y-2 text-center">
			<h1 class="text-3xl font-bold tracking-tight">Bienvenido a Taxorium</h1>
			<p class="text-neutral-400">
				Para comenzar, necesitamos configurar el rol de esta computadora en tu red.
			</p>
		</div>

		{#if error}
			<div class="rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-center text-red-400">
				{error}
			</div>
		{/if}

		{#if step === 1}
			<div class="mt-8 grid grid-cols-1 gap-6 md:grid-cols-2">
				<!-- Servidor Principal -->
				<button
					onclick={selectServer}
					class="group flex h-full flex-col items-center justify-center rounded-2xl border-2 border-neutral-800 p-8 text-left transition-all hover:border-blue-500 hover:bg-neutral-900"
				>
					<div
						class="mb-6 flex h-20 w-20 items-center justify-center rounded-full bg-blue-500/10 transition-transform group-hover:scale-110"
					>
						<IconServer class="h-10 w-10 text-blue-500" />
					</div>
					<h3 class="mb-2 text-xl font-bold">Servidor Principal</h3>
					<p class="text-center text-sm text-neutral-400">
						Esta computadora almacenará la base de datos local y administrará el sistema. Solo debe
						haber un Servidor Principal en la tienda.
					</p>
				</button>

				<!-- Cliente / Terminal -->
				<button
					onclick={selectClient}
					class="group flex h-full flex-col items-center justify-center rounded-2xl border-2 border-neutral-800 p-8 text-left transition-all hover:border-emerald-500 hover:bg-neutral-900"
				>
					<div
						class="mb-6 flex h-20 w-20 items-center justify-center rounded-full bg-emerald-500/10 transition-transform group-hover:scale-110"
					>
						<IconDeviceDesktop class="h-10 w-10 text-emerald-500" />
					</div>
					<h3 class="mb-2 text-xl font-bold">Terminal / Caja</h3>
					<p class="text-center text-sm text-neutral-400">
						Esta computadora se conectará al Servidor Principal a través de la red local para
						registrar ventas o administrar datos.
					</p>
				</button>
			</div>
		{:else if step === 2}
			<div
				class="mx-auto mt-8 max-w-md rounded-2xl border border-neutral-800 bg-neutral-900/50 p-8"
			>
				<h3 class="mb-4 text-xl font-bold">Conectar al Servidor</h3>
				<p class="mb-6 text-sm text-neutral-400">
					Ingrese la dirección IP del Servidor Principal de su red. Por ejemplo: <code
						class="rounded bg-neutral-950 px-2 py-1 text-neutral-300">192.168.1.100:3000</code
					>
				</p>

				<div class="space-y-4">
					<div class="space-y-2">
						<label for="ip" class="text-sm font-medium text-neutral-300"
							>Dirección IP del Servidor</label
						>
						<input
							id="ip"
							type="text"
							bind:value={serverIp}
							placeholder="192.168.1.x:3000"
							class="w-full rounded-lg border border-neutral-800 bg-neutral-950 px-4 py-3 text-white transition-colors focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 focus:outline-none"
						/>
					</div>

					<div class="flex gap-4 pt-4">
						<button
							onclick={() => (step = 1)}
							class="flex-1 rounded-lg border border-neutral-700 px-4 py-3 transition-colors hover:bg-neutral-800"
						>
							Atrás
						</button>
						<button
							onclick={saveClient}
							class="flex-1 rounded-lg bg-emerald-600 px-4 py-3 font-medium text-white transition-colors hover:bg-emerald-700"
						>
							Conectar
						</button>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>
