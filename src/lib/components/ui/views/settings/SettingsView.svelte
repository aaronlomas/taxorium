<script lang="ts">
	import NumberInput from '$lib/components/core/primitives/Number.svelte';
	import Button from '$lib/components/core/primitives/Button.svelte';
	import { icbperStore, ICBPER_TASA_DEFAULT } from '$lib/features/settings';

	let tasa = $state(ICBPER_TASA_DEFAULT);
	let guardando = $state(false);
	let guardado = $state(false);

	// Mantiene el input sincronizado con la tarifa cargada desde SQLite.
	$effect(() => {
		tasa = $icbperStore;
	});

	async function guardar() {
		guardando = true;
		guardado = false;
		await icbperStore.setTasa(Number(tasa));
		guardando = false;
		guardado = true;
		setTimeout(() => (guardado = false), 2500);
	}
</script>

<div class="flex h-full flex-col gap-4 p-4">
	<div>
		<h2 class="text-xl font-bold tracking-tight text-slate-100">Configuración</h2>
		<p class="text-sm leading-relaxed text-neutral-400">
			La configuración de la empresa se administra desde <span class="text-neutral-300"
				>Mi Cuenta → Editar Información</span
			>.
		</p>
	</div>

	<div class="w-full max-w-md rounded-sm border border-neutral-800 bg-neutral-900/50 p-4">
		<h3 class="mb-1 text-sm font-semibold text-neutral-200">
			Impuesto a la bolsa plástica (ICBPER)
		</h3>
		<p class="mb-3 text-xs leading-relaxed text-neutral-400">
			Monto fijo que se cobra por cada bolsa plástica entregada, adicional al precio del producto.
			Se declara ante SUNAT con el tributo 7152.
		</p>

		<div class="flex items-end gap-3">
			<div class="w-40">
				<NumberInput
					id="icbperTasa"
					label="Tarifa por bolsa (S/)"
					bind:value={tasa}
					step={0.1}
					min={0}
				/>
			</div>
			<Button variant="primary" onclick={guardar} disabled={guardando}>
				{#snippet children()}
					{guardando ? 'Guardando...' : 'Guardar'}
				{/snippet}
			</Button>
		</div>

		{#if guardado}
			<p class="mt-2 text-xs text-emerald-400">Tarifa guardada correctamente.</p>
		{/if}
	</div>
</div>
