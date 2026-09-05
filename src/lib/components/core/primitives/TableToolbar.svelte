<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    actions?: Snippet;
    filters?: Snippet;
  }

  let { actions, filters }: Props = $props();
</script>

<div class="grid grid-cols-[1fr_auto] rounded-b-md border-x border-b border-neutral-800 p-2">
  <div class="flex items-center gap-4">
    {@render actions?.()}
  </div>

  <div class="grid grid-cols-[auto_1fr] items-end gap-2">
    {@render filters?.()}
  </div>
</div>