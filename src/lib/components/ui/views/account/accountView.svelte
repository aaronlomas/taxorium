<script lang="ts">
	import PanelPerfil from '$lib/components/setups/account/PanelPerfil.svelte';
  import PanelSeller from '$lib/components/setups/account/PanelSeller.svelte';
  import TableSeller from '$lib/components/setups/account/TableSeller.svelte';
</script>

<div class="h-full grid grid-cols-2 grid-rows-[auto_1fr]">
    <div>
      <h1 class="border-b border-neutral-700 p-2 font-extrabold">Cuenta de Administrador</h1>
      <div>
        <h1 class="p-2 font-extrabold">Perfil:</h1>
        <PanelPerfil />
      </div>
    </div>
    <div>
      <h1 class="border-b border-neutral-700 p-2 font-extrabold">Crear Punto de Venta</h1>
      <PanelSeller />
    </div>
    <div class="col-span-2">
      <TableSeller />
    </div>
</div>
