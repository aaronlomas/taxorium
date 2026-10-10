<script lang="ts">
	/**
	 * @component Hint
	 * @description Tooltip con los estilos de la aplicación. Reemplaza al atributo `title`
	 * nativo, cuyo render depende del tema del sistema operativo y rompe el modo oscuro.
	 * Aparece al pasar el cursor y al enfocar con teclado vía :focus-visible, por lo que
	 * un click con mouse no lo deja pegado.
	 */
	import type { Snippet } from 'svelte';

	interface Props {
		/** Texto a mostrar dentro del hint. */
		text: string;
		/** Lado del elemento respecto al cual se ubica el hint. */
		side?: 'top' | 'bottom' | 'left' | 'right';
		/**
		 * Alineación sobre el eje perpendicular a `side`.
		 * Con `top`/`bottom` alinea horizontalmente; con `left`/`right`, verticalmente.
		 * Ej: `side="bottom" align="end"` = esquina inferior derecha.
		 */
		align?: 'start' | 'center' | 'end';
		/** Contenido que envuelve el hint (botón, icono, enlace...). */
		children: Snippet;
		/** Milisegundos de espera antes de mostrar el hint. */
		delay?: number;
		containerClass?: string;
		class?: string;
	}

	let {
		text,
		side = 'top',
		align = 'center',
		children,
		delay = 0,
		containerClass = '',
		class: className = ''
	}: Props = $props();

	// Eje principal: coloca el hint fuera del elemento, en el lado pedido.
	const sides = {
		top: 'bottom-full',
		bottom: 'top-full',
		left: 'right-full',
		right: 'left-full'
	} as const;

	const gaps = {
		top: 'mb-2',
		bottom: 'mt-2',
		left: 'mr-2',
		right: 'ml-2'
	} as const;

	// Eje perpendicular: alinea el hint a inicio, centro o fin del elemento.
	const aligns = {
		start: {
			horizontal: 'left-0',
			vertical: 'top-0'
		},
		center: {
			horizontal: 'left-1/2 -translate-x-1/2',
			vertical: 'top-1/2 -translate-y-1/2'
		},
		end: {
			horizontal: 'right-0',
			vertical: 'bottom-0'
		}
	} as const;

	const isVerticalSide = $derived(side === 'left' || side === 'right');
	const position = $derived(
		`${sides[side]} ${gaps[side]} ${aligns[align][isVerticalSide ? 'vertical' : 'horizontal']}`
	);
</script>

<!-- ELEMENTO QUE DISPARA EL HINT -->
<span class="group relative inline-flex {containerClass}" style="--hint-delay: {delay}ms">
	{@render children()}

	<!-- HINT -->
	<span
		role="tooltip"
		class="pointer-events-none invisible absolute z-30 w-max max-w-60 rounded-sm border
			border-neutral-700 bg-neutral-900 px-2 py-1 text-xs whitespace-pre-line text-neutral-300
			opacity-0 shadow-lg transition-opacity delay-(--hint-delay)
			group-hover:visible group-hover:opacity-100
			group-focus-visible:visible group-focus-visible:opacity-100
			{position} {className}"
	>
		{text}
	</span>
</span>
