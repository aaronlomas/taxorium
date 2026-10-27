<script lang="ts">
	/**
	 * @component Scroll
	 * @description Contenedor con scroll personalizado: scrollbar nativo oculto,
	 * barra visual pura controlada por estado Svelte. Compacta (4px), tipo escritorio.
	 */
	import type { Snippet } from 'svelte';

	interface Props {
		children?: Snippet;
		class?: string;
		contentClass?: string;
		orientation?: 'vertical' | 'horizontal' | 'both';
		maxHeight?: string;
		maxWidth?: string;
	}

	let {
		children,
		class: className = '',
		contentClass = '',
		orientation = 'vertical',
		maxHeight,
		maxWidth
	}: Props = $props();

	// ── Referencias DOM ───────────────────────────────────────────────────────
	let scrollEl = $state<HTMLDivElement | null>(null);
	let contentEl = $state<HTMLDivElement | null>(null);
	const scrollId = crypto.randomUUID();

	// ── Estado medido ─────────────────────────────────────────────────────────
	let scrollLeft = $state(0);
	let scrollTop = $state(0);
	let clientW = $state(0);
	let clientH = $state(0);
	let scrollW = $state(0);
	let scrollH = $state(0);

	// ── Visibilidad de scrollbars ─────────────────────────────────────────────
	const showH = $derived(
		(orientation === 'horizontal' || orientation === 'both') && scrollW > clientW
	);
	const showV = $derived(
		(orientation === 'vertical' || orientation === 'both') && scrollH > clientH
	);

	// ── Dimensiones del thumb ─────────────────────────────────────────────────
	const MIN_THUMB = 24;

	const thumbW = $derived(
		scrollW > 0 ? Math.max(MIN_THUMB, (clientW / scrollW) * clientW) : MIN_THUMB
	);
	const thumbX = $derived(
		scrollW > clientW ? (scrollLeft / (scrollW - clientW)) * (clientW - thumbW) : 0
	);

	const thumbH = $derived(
		scrollH > 0 ? Math.max(MIN_THUMB, (clientH / scrollH) * clientH) : MIN_THUMB
	);
	const thumbY = $derived(
		scrollH > clientH ? (scrollTop / (scrollH - clientH)) * (clientH - thumbH) : 0
	);

	// ── Estado de arrastre ────────────────────────────────────────────────────
	let draggingH = $state(false);
	let draggingV = $state(false);
	let dragStartX = $state(0);
	let dragStartScrollLeft = $state(0);
	let dragStartY = $state(0);
	let dragStartScrollTop = $state(0);

	// ── Medición ──────────────────────────────────────────────────────────────
	function measure() {
		if (!scrollEl) return;
		clientW = scrollEl.clientWidth;
		clientH = scrollEl.clientHeight;
		scrollW = scrollEl.scrollWidth;
		scrollH = scrollEl.scrollHeight;
		scrollLeft = scrollEl.scrollLeft;
		scrollTop = scrollEl.scrollTop;
	}

	function onScroll() {
		if (!scrollEl) return;
		scrollLeft = scrollEl.scrollLeft;
		scrollTop = scrollEl.scrollTop;
		scrollW = scrollEl.scrollWidth;
		scrollH = scrollEl.scrollHeight;
	}

	function onWheel(e: WheelEvent) {
		if (orientation === 'horizontal' && scrollEl && showH) {
			if (Math.abs(e.deltaY) > Math.abs(e.deltaX)) {
				scrollEl.scrollLeft += e.deltaY;
				e.preventDefault();
			}
		}
	}

	$effect(() => {
		if (!scrollEl) return;
		measure();
		const ro = new ResizeObserver(measure);
		ro.observe(scrollEl);
		if (contentEl) ro.observe(contentEl);
		return () => ro.disconnect();
	});

	// ── Arrastre del thumb ────────────────────────────────────────────────────
	function startDragH(e: MouseEvent) {
		e.preventDefault();
		draggingH = true;
		dragStartX = e.clientX;
		dragStartScrollLeft = scrollEl?.scrollLeft ?? 0;
	}

	function startDragV(e: MouseEvent) {
		e.preventDefault();
		draggingV = true;
		dragStartY = e.clientY;
		dragStartScrollTop = scrollEl?.scrollTop ?? 0;
	}

	$effect(() => {
		if (!draggingH && !draggingV) return;

		function onMove(e: MouseEvent) {
			if (!scrollEl) return;
			if (draggingH && thumbW < clientW) {
				const dx = e.clientX - dragStartX;
				const ratio = (scrollW - clientW) / (clientW - thumbW);
				scrollEl.scrollLeft = Math.min(scrollW - clientW, Math.max(0, dragStartScrollLeft + dx * ratio));
			}
			if (draggingV && thumbH < clientH) {
				const dy = e.clientY - dragStartY;
				const ratio = (scrollH - clientH) / (clientH - thumbH);
				scrollEl.scrollTop = Math.min(scrollH - clientH, Math.max(0, dragStartScrollTop + dy * ratio));
			}
		}

		function onUp() {
			draggingH = false;
			draggingV = false;
		}

		window.addEventListener('mousemove', onMove);
		window.addEventListener('mouseup', onUp);
		return () => {
			window.removeEventListener('mousemove', onMove);
			window.removeEventListener('mouseup', onUp);
		};
	});

	// ── Click en la pista (centrar posición) ──────────────────────────────────
	function clickTrackH(e: MouseEvent) {
		if (!scrollEl || e.target !== e.currentTarget) return;
		const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
		const clickPos = e.clientX - rect.left;
		const maxScroll = scrollW - clientW;
		const maxTrack = clientW - thumbW;
		if (maxTrack <= 0) return;
		const targetThumbX = clickPos - thumbW / 2;
		const ratio = Math.max(0, Math.min(1, targetThumbX / maxTrack));
		scrollEl.scrollLeft = ratio * maxScroll;
	}

	function clickTrackV(e: MouseEvent) {
		if (!scrollEl || e.target !== e.currentTarget) return;
		const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
		const clickPos = e.clientY - rect.top;
		const maxScroll = scrollH - clientH;
		const maxTrack = clientH - thumbH;
		if (maxTrack <= 0) return;
		const targetThumbY = clickPos - thumbH / 2;
		const ratio = Math.max(0, Math.min(1, targetThumbY / maxTrack));
		scrollEl.scrollTop = ratio * maxScroll;
	}

	// ── Accesibilidad por teclado ─────────────────────────────────────────────
	function onKeyDownH(e: KeyboardEvent) {
		if (!scrollEl) return;
		const step = 40;
		if (e.key === 'ArrowLeft') {
			scrollEl.scrollLeft -= step;
			e.preventDefault();
		} else if (e.key === 'ArrowRight') {
			scrollEl.scrollLeft += step;
			e.preventDefault();
		}
	}

	function onKeyDownV(e: KeyboardEvent) {
		if (!scrollEl) return;
		const step = 40;
		if (e.key === 'ArrowUp') {
			scrollEl.scrollTop -= step;
			e.preventDefault();
		} else if (e.key === 'ArrowDown') {
			scrollEl.scrollTop += step;
			e.preventDefault();
		} else if (e.key === 'PageUp') {
			scrollEl.scrollTop -= clientH;
			e.preventDefault();
		} else if (e.key === 'PageDown') {
			scrollEl.scrollTop += clientH;
			e.preventDefault();
		}
	}

	// ── Clases de overflow y recorte ─────────────────────────────────────────
	const overflowClass = $derived(
		({
			vertical: 'overflow-y-auto overflow-x-hidden',
			horizontal: 'overflow-x-auto overflow-y-hidden',
			both: 'overflow-auto'
		})[orientation]
	);

	const clipOffsetClass = $derived(
		({
			vertical: 'pr-3 -mr-3',
			horizontal: 'pb-3 -mb-3',
			both: 'pr-3 -mr-3 pb-3 -mb-3'
		})[orientation]
	);

	const style = $derived(
		[maxHeight ? `max-height: ${maxHeight};` : '', maxWidth ? `max-width: ${maxWidth};` : '']
			.filter(Boolean)
			.join(' ')
	);
</script>

<div
	class="relative flex flex-col overflow-hidden max-h-[inherit] max-w-[inherit] {className}"
	style={style || undefined}
>
	<div
		bind:this={scrollEl}
		id={scrollId}
		class="no-scrollbar flex-1 min-h-0 min-w-0 w-full h-full max-h-[inherit] max-w-[inherit] {overflowClass} overscroll-contain {clipOffsetClass}"
		onscroll={onScroll}
		onwheel={onWheel}
		role="region"
		tabindex="-1"
	>
		<div
			bind:this={contentEl}
			class="min-w-0 {orientation === 'horizontal' ? 'w-max min-w-full' : ''} {contentClass}"
		>
			{@render children?.()}
		</div>
	</div>

	<!-- Barra horizontal personalizada -->
	{#if showH}
		<div
			class="absolute bottom-0 left-0 right-0 h-1.5 cursor-pointer z-20 bg-transparent {showV ? 'right-1.5' : ''}"
			onclick={clickTrackH}
			role="presentation"
		>
			<div
				class="absolute bottom-px left-0 h-1 rounded-full cursor-grab select-none outline-none transition-colors duration-100 {draggingH ? 'bg-neutral-300/80 cursor-grabbing' : 'bg-neutral-500/40 hover:bg-neutral-400/60 focus-visible:bg-neutral-400/60'}"
				style="width: {thumbW}px; transform: translateX({thumbX}px);"
				onmousedown={startDragH}
				onkeydown={onKeyDownH}
				role="scrollbar"
				aria-controls={scrollId}
				aria-orientation="horizontal"
				aria-valuenow={Math.round(scrollLeft)}
				aria-valuemin={0}
				aria-valuemax={Math.round(scrollW - clientW)}
				tabindex="0"
			></div>
		</div>
	{/if}

	<!-- Barra vertical personalizada -->
	{#if showV}
		<div
			class="absolute top-0 right-0 bottom-0 w-1.5 cursor-pointer z-20 bg-transparent {showH ? 'bottom-1.5' : ''}"
			onclick={clickTrackV}
			role="presentation"
		>
			<div
				class="absolute top-0 right-px w-1 rounded-full cursor-grab select-none outline-none transition-colors duration-100 {draggingV ? 'bg-neutral-300/80 cursor-grabbing' : 'bg-neutral-500/40 hover:bg-neutral-400/60 focus-visible:bg-neutral-400/60'}"
				style="height: {thumbH}px; transform: translateY({thumbY}px);"
				onmousedown={startDragV}
				onkeydown={onKeyDownV}
				role="scrollbar"
				aria-controls={scrollId}
				aria-orientation="vertical"
				aria-valuenow={Math.round(scrollTop)}
				aria-valuemin={0}
				aria-valuemax={Math.round(scrollH - clientH)}
				tabindex="0"
			></div>
		</div>
	{/if}
</div>

<style>
	/* Único CSS puro donde Tailwind no tiene alcance: pseudo-elementos del motor del navegador */
	.no-scrollbar {
		scrollbar-width: none !important;
		-ms-overflow-style: none !important;
	}
	.no-scrollbar::-webkit-scrollbar {
		display: none !important;
		width: 0 !important;
		height: 0 !important;
		background: transparent !important;
		-webkit-appearance: none !important;
	}
	.no-scrollbar::-webkit-scrollbar-thumb,
	.no-scrollbar::-webkit-scrollbar-track,
	.no-scrollbar::-webkit-scrollbar-corner {
		display: none !important;
		width: 0 !important;
		height: 0 !important;
		background: transparent !important;
	}
</style>
