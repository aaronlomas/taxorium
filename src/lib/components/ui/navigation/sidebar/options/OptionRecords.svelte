<script lang="ts">
	import { IconChevronDown, IconChevronRight } from '@tabler/icons-svelte';

	// Revisar navigation.ts para agregar nuevos elementos u opciones
	import {
		navegacionOperaciones,
		navegacionRegistros,
		type ElementoNavegacion
	} from '../navigation';
	import { sellerAuth } from '$lib/features/sellerAuth';
	import { activeOption } from '$lib/features/navigation';

	let { onSeleccionarOpcion }: { onSeleccionarOpcion?: (opcion: ElementoNavegacion) => void } =
		$props();

	// Este estado controla qué menús desplegables quedan abiertos.
	// Solo tiene sentido para opciones que tienen subitems (ej. Comprobantes).
	let menusAbiertos = $state<Record<string, boolean>>({});

	function toggleMenu(nombre: string) {
		menusAbiertos[nombre] = !menusAbiertos[nombre];
	}

	// Filtra las opciones por permisos del seller.
	// Si no hay sellerAuth, el dueño/admin ve todo.
	// Las entradas sin accesoRequerido (ej. Dashboard) también se muestran.
	function filtrar(elementos: ElementoNavegacion[]) {
		return elementos.filter((el) => {
			if (!$sellerAuth) return true; // Dueño (Admin) ve todo
			if (!el.accesoRequerido) return true; // Elementos públicos (ej. Dashboard)
			// Si tiene acceso específico, verificar que coincida o si tiene acceso a todo (si existiera "all")
			return $sellerAuth.accesos === el.accesoRequerido;
		});
	}

	let operacionesFiltradas = $derived(filtrar(navegacionOperaciones));
	let registrosFiltrados = $derived(filtrar(navegacionRegistros));

	// Estilos base compartidos para cualquier opción del sidebar.
	// La diferencia está en si el item es un grupo con submenu (ej. Comprobantes)
	// o una opción simple (ej. Dashboard, Productos, Clientes).
	function getButtonClass(isActive: boolean, isGroup: boolean) {
		return [
			'group flex w-full items-center p-2 transition-colors hover:bg-brand-bg-secondary hover:text-brand-text-hover',
			isGroup ? 'justify-between' : 'gap-2',
			isActive ? 'text-brand-text-primary' : 'text-brand-text-secondary'
		].join(' ');
	}

	// Estilo del icono: mismo comportamiento para ambos tipos, pero con una regla
	// ligeramente distinta según el estado activo del item.
	function getIconClass(isActive: boolean) {
		return isActive
			? 'text-brand-text-sky group-hover:text-brand-text-sky'
			: 'text-brand-text-secondary group-hover:text-brand-text-sky';
	}
</script>

{#snippet elementoNav(elemento: ElementoNavegacion)}
	{@const isGroup = Boolean(elemento.subitems?.length)}
	{@const isActive = $activeOption?.nombre === elemento.nombre}
	{@const isExpanded = Boolean(menusAbiertos[elemento.nombre])}

	<div class="w-full">
		<!--
			ESTILO PARA ITEMS CON SUBITEMS:
			Se usa para elementos tipo "Comprobantes". Tiene flecha de expand/collapse
			y muestra el submenu debajo. El botón principal mantiene el mismo patrón visual
			que los items simples, pero con un layout de grupo.
		-->
		<button
			type="button"
			aria-expanded={isGroup ? isExpanded : undefined}
			onclick={() => {
				if (isGroup) {
					toggleMenu(elemento.nombre);
					return;
				}
				activeOption.set(elemento);
				onSeleccionarOpcion?.(elemento);
			}}
			class={getButtonClass(isActive, isGroup)}
		>
			<div class="flex items-center gap-2">
				{#if elemento.icono}
					<elemento.icono size={20} stroke={1.5} class={getIconClass(isActive)} />
				{/if}
				<span class="text-sm font-medium">{elemento.nombre}</span>
			</div>

			{#if isGroup}
				{#if isExpanded}
					<IconChevronDown size={16} class="text-brand-text-secondary" />
				{:else}
					<IconChevronRight size={16} class="text-brand-text-secondary" />
				{/if}
			{/if}
		</button>

		{#if isGroup && isExpanded}
			<!--
				SUBITEMS: estilos del submenú. Aquí va la lista interna de cada grupo
				como Boleta, Factura o Envíos SUNAT dentro de Comprobantes.
			-->
			<div class="ml-4 flex flex-col border-l border-brand-border">
				{#each elemento.subitems ?? [] as subitem}
					{@const isSubActive = $activeOption?.nombre === subitem.nombre}
					<button
						type="button"
						onclick={() => {
							activeOption.set(subitem);
							onSeleccionarOpcion?.(subitem);
						}}
						class={[
							'block w-full truncate px-2 py-1 text-left text-sm font-medium hover:bg-brand-bg-secondary hover:text-brand-text-hover',
							isSubActive ? 'text-brand-text-primary' : 'text-brand-text-secondary'
						].join(' ')}
					>
						{subitem.nombre}
					</button>
				{/each}
			</div>
		{/if}
	</div>
{/snippet}

<nav class="flex flex-1 flex-col overflow-y-auto">
	<!--
		Estas dos secciones vienen de arrays distintos en navigation.ts:
		- operacionesFiltradas: Ventas, Compras
		- registrosFiltrados: Dashboard, Comprobantes, Productos, Reportes, Clientes...
		El componente los renderiza en el mismo nav, pero cada elemento puede ir a una rama distinta
		según si tiene subitems o no.
	-->
	<p class="p-2 text-sm text-brand-text-secondary italic">Operaciones</p>
	{#each operacionesFiltradas as elemento}
		{@render elementoNav(elemento)}
	{/each}

	<p class="p-2 text-sm text-brand-text-secondary italic">Registros</p>
	{#each registrosFiltrados as elemento}
		{@render elementoNav(elemento)}
	{/each}
</nav>
