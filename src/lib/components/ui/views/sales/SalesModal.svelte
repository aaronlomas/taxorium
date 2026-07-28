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
	<div class="flex h-full w-full flex-col">
		<!-- Cabecera -->
		<div class="flex items-center justify-between border-b border-neutral-800 p-2">
			<h2 class="text-lg font-semibold text-slate-100">Registrar</h2>
		</div>

		<!-- Contenido -->
    <div>
      <label for="">Producto/Servicio</label>
      <Option options={productos}/>
    </div>
    <div>
      <label for="">Cantidad</label>
      <Number />
    </div>
    <div>
      <label for="">Precio Unitario</label>
      <Input />
    </div>
    <div>
      <label for="">Afectación IGV</label>
      <Input />
    </div>
    <div>
      <label for="">Monto Recibido</label>
      <Input />
    </div>
    <div>
      <label for="">Diferencia</label>
      <span>00.00</span>
    </div>

		<!-- Footer -->
		<div class="flex items-center justify-end gap-3 border-t border-neutral-800 p-2">
			<Button type="button" variant="outline" onclick={onClose}>Cancelar</Button>
			<Button type="button" variant="primary">Guardar</Button>
		</div>
	</div>
</Modal>