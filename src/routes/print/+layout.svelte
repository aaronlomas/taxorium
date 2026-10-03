<script lang="ts">
	// Layout completamente vacío para la ventana de impresión.
	// NO hereda el layout raíz (TitleBar, auth checks, etc.)
	// porque se abre en una WebviewWindow independiente de Tauri.
	let { children } = $props();
</script>

{@render children()}
