<script lang="ts">
  // Permite insertar Tabs y otros elementos
</script>

<div class="flex items-center overflow-x-auto bg-neutral-900 scrollbar-hide select-none border-b border-neutral-950">
  <slot />
</div>

<style>
  /* Ocultar scrollbar pero permitir scroll */
  .scrollbar-hide::-webkit-scrollbar {
    display: none;
  }
  .scrollbar-hide {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
</style>
