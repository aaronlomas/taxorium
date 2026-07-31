<script lang="ts">
  import Modal from "$lib/components/core/primitives/Modal.svelte";
  import Button from "$lib/components/core/primitives/Button.svelte";
  import Input from "$lib/components/core/primitives/Input.svelte";
  import Option from "$lib/components/core/primitives/Option.svelte";
  import Number from "$lib/components/core/primitives/Number.svelte";

  let { isOpen = $bindable(false), onClose }: { isOpen: boolean; onClose: () => void } = $props();

  let productos;
  let afectacionIgv;
  let diferencia;

</script>

<Modal bind:isOpen {onClose}>
	<div class="grid grid-cols-2 h-full w-full px-2 gap-4">
		<!-- Cabecera -->
		<div class="flex items-center justify-between border-b border-neutral-800 p-2 col-span-2">
			<h2 class="text-lg font-semibold text-slate-100">Registrar</h2>
		</div>

		<!-- Contenido -->
    <div>
      <label for="" class="select-none">Producto/Servicio</label>
      <Option options={productos}/>
    </div>
    <div>
      <label for="" class="select-none">Cantidad</label>
      <Number />
    </div>
    <div>
      <label for="" class="select-none">Precio Unitario</label>
      <Input />
    </div>
    <div>
      <label for="" class="select-none">Afectación IGV</label>
      <Input />
    </div>
    <div>
      <label for="" class="select-none">Monto Recibido</label>
      <Input />
    </div>
    <div class="grid">
      <label for="" class="select-none">Diferencia</label>
      <span>00.00</span>
    </div>

		<!-- Footer -->
		<div class="flex items-center justify-end gap-3 border-t border-neutral-800 p-2 col-span-2">
			<Button type="button" variant="outline" onclick={onClose}>Cancelar</Button>
			<Button type="button" variant="primary">Guardar</Button>
		</div>
	</div>
</Modal>