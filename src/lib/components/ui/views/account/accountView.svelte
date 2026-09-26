<script lang="ts">
	import PanelPerfil from '$lib/components/ui/setups/account/PanelPerfil.svelte';
	import PanelSeller from '$lib/components/ui/setups/account/PanelSeller.svelte';
	import TableSeller from '$lib/components/ui/setups/account/TableSeller.svelte';
</script>

<div class="grid h-full grid-cols-[1fr_auto] grid-rows-[auto_auto_1fr]">
	<div class="col-span-2 flex border-b border-neutral-700">
		<h1 class="p-2 font-extrabold">Administrar Cuentas</h1>
	</div>
	<div>
		<div class="p-2">
			<h1 class="p-2 font-extrabold">Perfil:</h1>
			<PanelPerfil />
		</div>
	</div>
	<div>
		<PanelSeller />
	</div>
	<div class="col-span-2 overflow-scroll px-2">
		<TableSeller />
	</div>
</div>
