<script lang="ts">
  /**
   * @component Button
   * @description Botón reutilizable para la interfaz de usuario.
   */
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  interface Props extends HTMLButtonAttributes {
    variant?: "primary" | "secondary" | "danger";
    size?: "sm" | "md" | "lg";
    double?: boolean; // ó hasIcon?: boolean
    children?: Snippet;
  }

  let {
    variant = "primary",
    size = "md",
    double = false,
    children,
    class: className = "",
    onclick,
    ...rest
  }: Props = $props();

  export const SIZING = {
    button: {
      sm: "px-3 py-1 text-xs",
      md: "px-3 py-2 text-sm",
      lg: "px-4 py-3 text-lg",
    }
  } as const;

  // Definición de estilos
  const variants = {
    primary: "text-white bg-blue-600 hover:bg-blue-700",
    secondary: "text-gray-900 bg-gray-100 hover:bg-gray-200",
    danger: "text-white bg-red-600 hover:bg-red-700",
  };

  const sizes = {
    sm: SIZING.button.sm,
    md: SIZING.button.md,
    lg: SIZING.button.lg,
  };
</script>

<button
  class={[
    "rounded-xl cursor-pointer",
    // Usar double para botones que requieran un icono a la izquierda
    double ? "grid grid-cols-[auto_1fr] items-center" : "inline-flex items-center justify-center",
    variants[variant],
    sizes[size],
    className
  ]}
  {onclick}
  {...rest}
>
  {#if children}
    {@render children()}
  {/if}
</button>