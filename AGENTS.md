# SYSTEM INSTRUCTIONS - DEVELOPMENT RULES

## Core Rule: Svelte 5 Strict Syntax

You must EXCLUSIVELY use modern Svelte 5 syntax. Legacy Svelte 3/4 syntax is strictly forbidden.

### Required Patterns (Svelte 5):

- **State & Reactivity:** Use `$state()`, `$derived()`, and `$effect()` runes. NEVER use legacy `let` variables for reactive state or `$: label` statements.
- **Props:** Use destructuring with `$props()`. NEVER use `export let`.
- **Content Snippets:** Use `{#snippet name()}` and `{@render name()}`. NEVER use legacy `<slot />` or `$$slots`.
- **Event Handling:** Use standard HTML attributes like `onclick={...}` or `onkeydown={...}`. NEVER use legacy `on:click` or event modifiers.

### Enforcement:

If you see or generate any legacy Svelte syntax, stop immediately, delete it, and rewrite it using Svelte 5 runes and snippets.

### sql:

- Modify `01_esquema_inicial.sql` directly; no additional SQL files are allowed.
