<script lang="ts">
	import ProfilePanel from '$lib/components/ui/features/account/ProfilePanel.svelte';
</script>

<!--
	Vista de cuenta: se muestra al usuario para consultar o editar sus datos.
	Compone las interfaces de views/account/ — no contiene lógica propia.
-->
<div class="grid h-full grid-rows-[auto_1fr] overflow-hidden">
	<div class="overflow-scroll p-2">
		<ProfilePanel />
	</div>
</div>
