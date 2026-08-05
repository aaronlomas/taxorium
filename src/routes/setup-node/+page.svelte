<script lang="ts">
	import { goto } from '$app/navigation';
	import { configStore } from '$lib/stores/config';
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

<div class="flex h-full w-full flex-col items-center justify-center p-8 bg-neutral-950 text-neutral-100">
	<div class="max-w-3xl w-full space-y-8">
		<div class="text-center space-y-2">
			<h1 class="text-3xl font-bold tracking-tight">Bienvenido a Taxorium</h1>
			<p class="text-neutral-400">Para comenzar, necesitamos configurar el rol de esta computadora en tu red.</p>
		</div>

		{#if error}
			<div class="bg-red-500/10 border border-red-500/20 text-red-400 p-4 rounded-lg text-center">
				{error}
			</div>
		{/if}

		{#if step === 1}
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-8">
				<!-- Servidor Principal -->
				<button
					onclick={selectServer}
					class="flex flex-col items-center justify-center p-8 border-2 border-neutral-800 rounded-2xl hover:border-blue-500 hover:bg-neutral-900 transition-all group text-left h-full"
				>
					<div class="h-20 w-20 rounded-full bg-blue-500/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
						<IconServer class="w-10 h-10 text-blue-500" />
					</div>
					<h3 class="text-xl font-bold mb-2">Servidor Principal</h3>
					<p class="text-neutral-400 text-center text-sm">
						Esta computadora almacenará la base de datos local y administrará el sistema. Solo debe haber un Servidor Principal en la tienda.
					</p>
				</button>

				<!-- Cliente / Terminal -->
				<button
					onclick={selectClient}
					class="flex flex-col items-center justify-center p-8 border-2 border-neutral-800 rounded-2xl hover:border-emerald-500 hover:bg-neutral-900 transition-all group text-left h-full"
				>
					<div class="h-20 w-20 rounded-full bg-emerald-500/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
						<IconDeviceDesktop class="w-10 h-10 text-emerald-500" />
					</div>
					<h3 class="text-xl font-bold mb-2">Terminal / Caja</h3>
					<p class="text-neutral-400 text-center text-sm">
						Esta computadora se conectará al Servidor Principal a través de la red local para registrar ventas o administrar datos.
					</p>
				</button>
			</div>
		{:else if step === 2}
			<div class="max-w-md mx-auto mt-8 p-8 border border-neutral-800 rounded-2xl bg-neutral-900/50">
				<h3 class="text-xl font-bold mb-4">Conectar al Servidor</h3>
				<p class="text-neutral-400 text-sm mb-6">
					Ingrese la dirección IP del Servidor Principal de su red.
					Por ejemplo: <code class="bg-neutral-950 px-2 py-1 rounded text-neutral-300">192.168.1.100:3000</code>
				</p>
				
				<div class="space-y-4">
					<div class="space-y-2">
						<label for="ip" class="text-sm font-medium text-neutral-300">Dirección IP del Servidor</label>
						<input
							id="ip"
							type="text"
							bind:value={serverIp}
							placeholder="192.168.1.x:3000"
							class="w-full bg-neutral-950 border border-neutral-800 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors"
						/>
					</div>
					
					<div class="flex gap-4 pt-4">
						<button
							onclick={() => (step = 1)}
							class="flex-1 px-4 py-3 rounded-lg border border-neutral-700 hover:bg-neutral-800 transition-colors"
						>
							Atrás
						</button>
						<button
							onclick={saveClient}
							class="flex-1 px-4 py-3 rounded-lg bg-emerald-600 hover:bg-emerald-700 text-white font-medium transition-colors"
						>
							Conectar
						</button>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>
