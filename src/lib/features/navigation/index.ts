import { writable } from 'svelte/store';
import type { ElementoNavegacion } from '$lib/components/ui/navigation/sidebar/navigation';

export const activeOption = writable<ElementoNavegacion | null>(null);
