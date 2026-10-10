<script lang="ts">
	/**
	 * @component Dropdown
	 * @description Menú flotante anclado a un disparador. Se abre al pasar el cursor
	 * (opcional) y/o al hacer click (opcional), y se cierra al salir del área, al
	 * hacer click fuera o al presionar Escape. Usa `DropdownItem` y `DropdownDivider`
	 * para el contenido del menú.
	 */
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	interface Props extends HTMLButtonAttributes {
		/** Lado respecto al disparador en el que aparece el menú. */
		side?: 'top' | 'bottom' | 'left' | 'right';
		/** Alineación sobre el eje perpendicular a `side`. */
		align?: 'start' | 'center' | 'end';
		/** Abre el menú al pasar el cursor sobre el disparador o el contenido. */
		openOnHover?: boolean;
		/** Permite fijar el menú con click en el disparador. */
		openOnClick?: boolean;
		/** Estado de apertura (enlazable con `bind:isOpen`). */
		isOpen?: boolean;
		/** Clases para el contenedor que envuelve disparador y menú. */
		containerClass?: string;
		/** Clases extra para el menú flotante. */
		menuClass?: string;
		/** Contenido del disparador (se renderiza dentro de un `<button>`). */
		trigger: Snippet;
		/** Contenido del menú. */
		menu: Snippet;
	}

	let {
		side = 'bottom',
		align = 'start',
		openOnHover = true,
		openOnClick = true,
		isOpen = $bindable(false),
		containerClass = '',
		menuClass = '',
		trigger,
		menu,
		class: className = '',
		onclick,
		...rest
	}: Props = $props();

	let container = $state<HTMLElement>();
	let triggerHovered = $state(false);
	let contentHovered = $state(false);

	let visible = $derived(isOpen || (openOnHover && (triggerHovered || contentHovered)));

	const sides = {
		top: 'bottom-full',
		bottom: 'top-full',
		left: 'right-full',
		right: 'left-full'
	} as const;

	const aligns = {
		start: { horizontal: 'left-0', vertical: 'top-0' },
		center: { horizontal: 'left-1/2 -translate-x-1/2', vertical: 'top-1/2 -translate-y-1/2' },
		end: { horizontal: 'right-0', vertical: 'bottom-0' }
	} as const;

	const isVertical = $derived(side === 'left' || side === 'right');
	const position = $derived(
		`${sides[side]} ${aligns[align][isVertical ? 'vertical' : 'horizontal']}`
	);

	function close() {
		isOpen = false;
		triggerHovered = false;
		contentHovered = false;
	}

	function handleTriggerClick(
		event: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }
	) {
		onclick?.(event);
		if (!openOnClick) return;
		if (isOpen) {
			close();
		} else {
			isOpen = true;
		}
	}

	function handleMouseLeave() {
		triggerHovered = false;
		contentHovered = false;
		// Si el menú depende del hover, al salir del área se cierra (evita que quede pegado).
		if (openOnHover) close();
	}

	function handleMenuClick(event: MouseEvent) {
		if ((event.target as Element).closest('[role="menuitem"]')) close();
	}

	function handleWindowClick(event: MouseEvent) {
		if (!visible || !container || container.contains(event.target as Node)) return;
		close();
	}

	function handleWindowKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') close();
	}
</script>

<svelte:window onclick={handleWindowClick} onkeydown={handleWindowKeydown} />

<span
	class="relative inline-flex {containerClass}"
	role="presentation"
	bind:this={container}
	onmouseenter={() => (triggerHovered = true)}
	onmouseleave={handleMouseLeave}
>
	<button
		type="button"
		class="relative {className}"
		aria-haspopup="menu"
		aria-expanded={visible}
		onclick={handleTriggerClick}
		{...rest}
	>
		{@render trigger()}
	</button>

	{#if visible}
		<!-- svelte-ignore a11y_no_static_element_interactions, a11y_click_events_have_key_events -->
		<div
			role="menu"
			tabindex="-1"
			class="dropdown-menu absolute z-20 flex flex-col border border-neutral-800 bg-neutral-900 text-sm {position} {menuClass}"
			onmouseenter={() => (contentHovered = true)}
			onmouseleave={() => (contentHovered = false)}
			onclick={handleMenuClick}
		>
			{@render menu()}
		</div>
	{/if}
</span>

<style>
	.dropdown-menu {
		box-shadow: 0 0 10px #00000052;
		animation: dropdown-in 0.12s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes dropdown-in {
		from {
			opacity: 0;
			transform: scale(0.97);
		}
		to {
			opacity: 1;
			transform: scale(1);
		}
	}
</style>
