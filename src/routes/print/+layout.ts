// La ventana de impresión se abre en tiempo de ejecución desde Tauri.
// No se puede pre-renderizar porque necesita acceder a la URL en el cliente.
export const prerender = false;
export const ssr = false;
