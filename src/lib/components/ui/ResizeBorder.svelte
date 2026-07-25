<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';

	const win = getCurrentWindow();

	// Tauri v2 ResizeDirection values
	type Direction =
		| 'North'
		| 'South'
		| 'East'
		| 'West'
		| 'NorthWest'
		| 'NorthEast'
		| 'SouthWest'
		| 'SouthEast';

	async function startResize(direction: Direction) {
		try {
			await win.startResizeDragging(direction as any);
		} catch (e) {
			console.error('resize error:', e);
		}
	}

	const handles: { dir: Direction; style: string; cursor: string }[] = [
		// Bordes
		{
			dir: 'North',
			style: 'top-0 left-[6px] right-[6px] h-[5px]',
			cursor: 'cursor-n-resize'
		},
		{
			dir: 'South',
			style: 'bottom-0 left-[6px] right-[6px] h-[5px]',
			cursor: 'cursor-s-resize'
		},
		{
			dir: 'West',
			style: 'left-0 top-[6px] bottom-[6px] w-[5px]',
			cursor: 'cursor-w-resize'
		},
		{
			dir: 'East',
			style: 'right-0 top-[6px] bottom-[6px] w-[5px]',
			cursor: 'cursor-e-resize'
		},
		// Esquinas
		{
			dir: 'NorthWest',
			style: 'top-0 left-0 w-[10px] h-[10px]',
			cursor: 'cursor-nw-resize'
		},
		{
			dir: 'NorthEast',
			style: 'top-0 right-0 w-[10px] h-[10px]',
			cursor: 'cursor-ne-resize'
		},
		{
			dir: 'SouthWest',
			style: 'bottom-0 left-0 w-[10px] h-[10px]',
			cursor: 'cursor-sw-resize'
		},
		{
			dir: 'SouthEast',
			style: 'bottom-0 right-0 w-[10px] h-[10px]',
			cursor: 'cursor-se-resize'
		}
	];
</script>

<!--
  Handles de resize: fijos sobre toda la ventana, sólo responden en los bordes/esquinas.
  El centro queda libre (pointer-events-none) para no bloquear el contenido.
-->
{#each handles as h}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="fixed z-9999 select-none {h.style} {h.cursor}"
		onmousedown={() => startResize(h.dir)}
	></div>
{/each}
