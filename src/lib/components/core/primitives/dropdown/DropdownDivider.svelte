<script lang="ts">
	/** @component DropdownDivider — Separador visual entre grupos de `DropdownItem`. */
	let { class: className = '' }: { class?: string } = $props();
</script>

<div role="separator" class="my-1 h-px w-full bg-neutral-800 {className}"></div>
