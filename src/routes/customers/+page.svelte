<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { IconPlus, IconSearch, IconEdit, IconTrash, IconX } from '@tabler/icons-svelte';

	type Customer = {
		id: number;
		document_type: string;
		document_number: string;
		name: string;
		address: string | null;
		email: string | null;
		is_active: boolean;
	};

	let customers = $state<Customer[]>([]);
	let searchQuery = $state('');
	let showModal = $state(false);
	let isEditing = $state(false);
	let currentId = $state<number | null>(null);

	let formData = $state({
		document_type: '6', // 6: RUC, 1: DNI
		document_number: '',
		name: '',
		address: '',
		email: ''
	});

	let loading = $state(true);

	async function loadCustomers() {
		try {
			loading = true;
			customers = await invoke('get_customers');
		} catch (e) {
			console.error(e);
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		loadCustomers();
	});

	let filteredCustomers = $derived(
		customers.filter(
			(c) =>
				c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				c.document_number.includes(searchQuery)
		)
	);

	function openCreateModal() {
		isEditing = false;
		currentId = null;
		formData = { document_type: '6', document_number: '', name: '', address: '', email: '' };
		showModal = true;
	}

	function openEditModal(customer: Customer) {
		isEditing = true;
		currentId = customer.id;
		formData = {
			document_type: customer.document_type,
			document_number: customer.document_number,
			name: customer.name,
			address: customer.address || '',
			email: customer.email || ''
		};
		showModal = true;
	}

	async function saveCustomer() {
		try {
			if (isEditing && currentId !== null) {
				await invoke('update_customer', { id: currentId, payload: formData });
			} else {
				await invoke('create_customer', { payload: formData });
			}
			showModal = false;
			await loadCustomers();
		} catch (e) {
			console.error(e);
			alert('Error al guardar el cliente');
		}
	}

	async function deleteCustomer(id: number) {
		if (confirm('¿Estás seguro de eliminar este cliente?')) {
			try {
				await invoke('delete_customer', { id });
				await loadCustomers();
			} catch (e) {
				console.error(e);
				alert('Error al eliminar');
			}
		}
	}
</script>

<div class="min-h-screen bg-slate-50/50 p-8 pt-12 dark:bg-slate-900">
	<div class="mx-auto max-w-6xl">
		<!-- Cabecera -->
		<div class="mb-8 flex items-end justify-between">
			<div>
				<h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white">Clientes</h1>
				<p class="mt-2 text-sm text-slate-500 dark:text-slate-400">
					Administra tu cartera de clientes para la facturación.
				</p>
			</div>
			<button
				onclick={openCreateModal}
				class="flex items-center gap-2 rounded-xl bg-indigo-600 px-5 py-2.5 text-sm font-semibold text-white shadow-sm transition-all hover:bg-indigo-500 hover:shadow-indigo-500/25 active:scale-95"
			>
				<IconPlus size={18} />
				Nuevo Cliente
			</button>
		</div>

		<!-- Buscador y Filtros -->
		<div class="mb-6 rounded-2xl border border-slate-200/60 bg-white/50 p-2 shadow-sm backdrop-blur-xl dark:border-slate-800/60 dark:bg-slate-900/50">
			<div class="relative">
				<IconSearch
					size={18}
					class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400"
				/>
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="Buscar por nombre, RUC o DNI..."
					class="w-full rounded-xl border-none bg-transparent py-2.5 pl-10 pr-4 text-sm text-slate-900 placeholder:text-slate-400 focus:ring-2 focus:ring-indigo-500/50 dark:text-white dark:placeholder:text-slate-500"
				/>
			</div>
		</div>

		<!-- Tabla -->
		<div class="overflow-hidden rounded-2xl border border-slate-200/60 bg-white/80 shadow-sm backdrop-blur-xl dark:border-slate-800/60 dark:bg-slate-900/80">
			<table class="w-full text-left text-sm text-slate-600 dark:text-slate-300">
				<thead class="border-b border-slate-200/60 bg-slate-50/50 text-xs font-semibold uppercase text-slate-500 dark:border-slate-800/60 dark:bg-slate-800/50 dark:text-slate-400">
					<tr>
						<th class="px-6 py-4">Cliente</th>
						<th class="px-6 py-4">Documento</th>
						<th class="px-6 py-4">Contacto</th>
						<th class="px-6 py-4 text-right">Acciones</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-slate-100/60 dark:divide-slate-800/60">
					{#if loading}
						<tr>
							<td colspan="4" class="px-6 py-12 text-center text-slate-400">Cargando clientes...</td>
						</tr>
					{:else if filteredCustomers.length === 0}
						<tr>
							<td colspan="4" class="px-6 py-12 text-center text-slate-400">No se encontraron clientes.</td>
						</tr>
					{:else}
						{#each filteredCustomers as customer (customer.id)}
							<tr class="group transition-colors hover:bg-slate-50/50 dark:hover:bg-slate-800/30">
								<td class="px-6 py-4">
									<div class="font-medium text-slate-900 dark:text-white">{customer.name}</div>
									<div class="text-xs text-slate-500">{customer.address || '-'}</div>
								</td>
								<td class="px-6 py-4">
									<span class="inline-flex items-center rounded-md bg-slate-100 px-2 py-1 text-xs font-medium text-slate-600 dark:bg-slate-800 dark:text-slate-300">
										{customer.document_type === '6' ? 'RUC' : customer.document_type === '1' ? 'DNI' : 'OTRO'}
									</span>
									<span class="ml-2 font-mono text-sm">{customer.document_number}</span>
								</td>
								<td class="px-6 py-4">
									<div class="text-sm">{customer.email || '-'}</div>
								</td>
								<td class="px-6 py-4 text-right">
									<div class="flex justify-end gap-2 opacity-0 transition-opacity group-hover:opacity-100">
										<button
											onclick={() => openEditModal(customer)}
											class="rounded-lg p-2 text-slate-400 hover:bg-indigo-50 hover:text-indigo-600 dark:hover:bg-indigo-500/10 dark:hover:text-indigo-400"
										>
											<IconEdit size={18} />
										</button>
										<button
											onclick={() => deleteCustomer(customer.id)}
											class="rounded-lg p-2 text-slate-400 hover:bg-red-50 hover:text-red-600 dark:hover:bg-red-500/10 dark:hover:text-red-400"
										>
											<IconTrash size={18} />
										</button>
									</div>
								</td>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>
	</div>
</div>

<!-- Modal -->
{#if showModal}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 px-4 py-6 backdrop-blur-sm sm:px-6">
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="absolute inset-0" onclick={() => (showModal = false)}></div>
		<div class="relative w-full max-w-lg overflow-hidden rounded-2xl bg-white shadow-2xl ring-1 ring-slate-900/5 dark:bg-slate-900 dark:ring-white/10">
			<!-- Cabecera del Modal -->
			<div class="flex items-center justify-between border-b border-slate-100 px-6 py-4 dark:border-slate-800">
				<h2 class="text-lg font-semibold text-slate-900 dark:text-white">
					{isEditing ? 'Editar Cliente' : 'Nuevo Cliente'}
				</h2>
				<button
					onclick={() => (showModal = false)}
					class="rounded-lg p-2 text-slate-400 hover:bg-slate-100 hover:text-slate-500 dark:hover:bg-slate-800 dark:hover:text-slate-300"
				>
					<IconX size={20} />
				</button>
			</div>

			<!-- Cuerpo del Modal -->
			<div class="p-6">
				<div class="grid grid-cols-2 gap-4">
					<div class="col-span-2 sm:col-span-1">
						<label for="docType" class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300">Tipo Doc.</label>
						<select
							id="docType"
							bind:value={formData.document_type}
							class="w-full rounded-xl border-slate-200 bg-slate-50 px-3 py-2 text-sm text-slate-900 focus:border-indigo-500 focus:ring-indigo-500 dark:border-slate-700 dark:bg-slate-800/50 dark:text-white"
						>
							<option value="6">RUC (Registro Único de Contribuyentes)</option>
							<option value="1">DNI (Documento Nacional de Identidad)</option>
							<option value="4">CE (Carnet de Extranjería)</option>
						</select>
					</div>
					<div class="col-span-2 sm:col-span-1">
						<label for="docNum" class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300">Número</label>
						<input
							type="text"
							id="docNum"
							bind:value={formData.document_number}
							class="w-full rounded-xl border-slate-200 bg-slate-50 px-3 py-2 text-sm text-slate-900 focus:border-indigo-500 focus:ring-indigo-500 dark:border-slate-700 dark:bg-slate-800/50 dark:text-white"
						/>
					</div>
					<div class="col-span-2">
						<label for="name" class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300">Razón Social o Nombre</label>
						<input
							type="text"
							id="name"
							bind:value={formData.name}
							class="w-full rounded-xl border-slate-200 bg-slate-50 px-3 py-2 text-sm text-slate-900 focus:border-indigo-500 focus:ring-indigo-500 dark:border-slate-700 dark:bg-slate-800/50 dark:text-white"
						/>
					</div>
					<div class="col-span-2">
						<label for="address" class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300">Dirección</label>
						<input
							type="text"
							id="address"
							bind:value={formData.address}
							class="w-full rounded-xl border-slate-200 bg-slate-50 px-3 py-2 text-sm text-slate-900 focus:border-indigo-500 focus:ring-indigo-500 dark:border-slate-700 dark:bg-slate-800/50 dark:text-white"
						/>
					</div>
					<div class="col-span-2">
						<label for="email" class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300">Correo Electrónico</label>
						<input
							type="email"
							id="email"
							bind:value={formData.email}
							class="w-full rounded-xl border-slate-200 bg-slate-50 px-3 py-2 text-sm text-slate-900 focus:border-indigo-500 focus:ring-indigo-500 dark:border-slate-700 dark:bg-slate-800/50 dark:text-white"
						/>
					</div>
				</div>
			</div>

			<!-- Pie del Modal -->
			<div class="flex justify-end gap-3 border-t border-slate-100 bg-slate-50/50 px-6 py-4 dark:border-slate-800 dark:bg-slate-800/30">
				<button
					onclick={() => (showModal = false)}
					class="rounded-xl px-4 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100 dark:text-slate-300 dark:hover:bg-slate-800"
				>
					Cancelar
				</button>
				<button
					onclick={saveCustomer}
					class="rounded-xl bg-indigo-600 px-5 py-2 text-sm font-semibold text-white shadow-sm transition-all hover:bg-indigo-500 hover:shadow-indigo-500/25 active:scale-95"
				>
					{isEditing ? 'Actualizar' : 'Guardar Cliente'}
				</button>
			</div>
		</div>
	</div>
{/if}
