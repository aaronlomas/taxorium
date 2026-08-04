<script lang="ts">
	import Input from '$lib/components/core/primitives/Input.svelte';
  import Button from '$lib/components/core/primitives/Button.svelte';
  import Option from '$lib/components/core/primitives/Option.svelte';
	import { IconLock, IconUser, IconEyeOff, IconEye } from '@tabler/icons-svelte';
	
  const MODULOS = [
    {value: "sales", label: "Módulo de Ventas"},
    {value: "shop", label: "Módulo de Compras"},
    {value: "products", label: "Módulo de Productos"}
  ]
  const DOMINIOS = [
    {value: "http://192.168.1.3", label: "CAJA-1"},
    {value: "http://192.168.1.4", label: "CAJA-2"},
    {value: "http://192.168.1.5", label: "CAJA-3"}
  ]

	let mostrarPassword = $state(false);
</script>

<div class="border-l border-neutral-700 p-2">
	<form action="" class="grid gap-2">
		<h1 class="font-bold">Registrar nuevo vendedor o similar</h1>
		<Input label="Usuario" variant="triple">
			{#snippet icon()}
				<IconUser size={18} />
			{/snippet}
		</Input>
		<Input label="Contraseña" variant="triple" type={mostrarPassword ? 'text' : 'password'}>
			{#snippet icon()}
				<IconLock size={18} />
			{/snippet}
			{#snippet action()}
				<button
					type="button"
					class="flex items-center justify-center hover:text-neutral-200 focus:outline-none p-2"
					onclick={() => (mostrarPassword = !mostrarPassword)}
				>
					{#if mostrarPassword}
						<IconEyeOff size={18} stroke={1.5} />
					{:else}
						<IconEye size={18} stroke={1.5} />
					{/if}
				</button>
			{/snippet}
		</Input>

    <Option label="Asignar Accesos" options={MODULOS}/>
    <Option label="Asignar Dominio / Puesto" options={DOMINIOS}/>
    
    <div>
      <Button type="submit">
        Guardar
      </Button>
    </div>
	</form>

  <!-- DETALLES -->
</div>
