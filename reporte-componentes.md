# Evaluación de la carpeta de componentes

## Alcance

Se revisó la estructura de [src/lib/components](src/lib/components) y algunos archivos clave para entender cómo están organizados los componentes, la semántica de nombres, la separación de responsabilidades y el nivel de robustez actual.

## Resumen ejecutivo

La estructura tiene una buena base conceptual: hay una capa de primitivas reutilizables y una capa de vistas funcionales, pero aún presenta mezcla de responsabilidades, inconsistencias de nomenclatura y algunos patrones heredados que conviene normalizar. El mayor riesgo no es la ausencia de componentes, sino la pérdida de claridad a medida que se agregan más módulos.

En su estado actual, la carpeta está funcionando como una estructura de “prototipo funcional”, pero necesita una limpieza de arquitectura para sostener crecimiento sin que cada vista dependa de decisiones internas de otra.

---

## Fortalezas observadas

### 1. Separación inicial de capas

Existe una distinción útil entre:

- [src/lib/components/core/primitives](src/lib/components/core/primitives): componentes base reutilizables
- [src/lib/components/ui](src/lib/components/ui): composición visual de pantallas, paneles, tabs y vistas
- [src/lib/components/log](src/lib/components/log): utilidades/logging específicas

Esto es una buena base, porque deja claro que hay un nivel “fundacional” y otro “de producto”.

### 2. Componentes reutilizables bien identificados

Los primitives de la carpeta core muestran un buen intento de encapsular elementos como:

- [src/lib/components/core/primitives/Button.svelte](src/lib/components/core/primitives/Button.svelte)
- [src/lib/components/core/primitives/Input.svelte](src/lib/components/core/primitives/Input.svelte)
- [src/lib/components/core/primitives/Modal.svelte](src/lib/components/core/primitives/Modal.svelte)
- [src/lib/components/core/primitives/Table.svelte](src/lib/components/core/primitives/Table.svelte)

Ese tipo de componentes es precisamente lo que permite escalar sin duplicación.

### 3. Organización por dominio funcional dentro de vistas

La carpeta [src/lib/components/ui/views](src/lib/components/ui/views) está bien segmentada por módulos como:

- customers
- products
- sales
- reports
- settings
- account
- vouchers

Esto ayuda a mantener el código más cercano al dominio del negocio.

---

## Problemas detectados

### 1. Mezcla de responsabilidades en la carpeta ui

La carpeta [src/lib/components/ui](src/lib/components/ui) incluye elementos muy distintos:

- navegación: [src/lib/components/ui/sidebar](src/lib/components/ui/sidebar)
- tabs: [src/lib/components/ui/tab-bar](src/lib/components/ui/tab-bar)
- vistas: [src/lib/components/ui/views](src/lib/components/ui/views)
- impresión/PDF: [src/lib/components/ui/print](src/lib/components/ui/print)
- setups: [src/lib/components/ui/setups](src/lib/components/ui/setups)

Esto es coherente para un inicio, pero a la larga genera una “ui” demasiado amplia y difícil de navegar. La semántica de “ui” es muy genérica. Faltan capas más específicas como app-shell, features y shared.

### 2. Nombres inconsistentes y errores de ortografía

Se observan varios nombres con mezcla de español/inglés y typos:

- [src/lib/components/ui/sidebar/navegation.ts](src/lib/components/ui/sidebar/navegation.ts): “navegation” debería ser “navigation”
- “SubItemNavegacion”, “ElementoNavegacion” en el mismo archivo
- [src/lib/components/ui/views/viewRoutes.ts](src/lib/components/ui/views/viewRoutes.ts): la lógica de rutas vive dentro una “vista”, pero semánticamente es un registry/router
- nomenclatura mezclada entre “Sales”, “Ventas”, “PanelSalesView”, “BoletaVentaView”, etc.

La convención debería ser una sola: todo en inglés o todo en español, según el estándar del proyecto. Lo importante es la consistencia.

### 3. La capa de rutas está acoplada a la capa de vistas

En [src/lib/components/ui/views/viewRoutes.ts](src/lib/components/ui/views/viewRoutes.ts) se define un mapa de componentes por nombre de tab. Eso es práctico, pero también tiende a convertirse en una fuente de acoplamiento entre:

- configuración de navegación
- nombres de pantallas
- importaciones directas
- lógica de render

Esto se vuelve rígido al crecer. Una mejor separación sería tener un archivo de routing o feature registry más centralizado, sin depender de nombres literales de UI.

### 4. Legacy Svelte 3/4 en componentes clave

Se detecta sintaxis heredada en [src/lib/components/ui/tab-bar/tab/Tab.svelte](src/lib/components/ui/tab-bar/tab/Tab.svelte):

- `export let`
- `on:click`
- `on:keydown`
- `<slot />` en [src/lib/components/ui/tab-bar/TabBar.svelte](src/lib/components/ui/tab-bar/TabBar.svelte)

Esto contradice la regla del proyecto que exige Svelte 5 con `$state()`, `$props()`, snippets y atributos HTML estándar.

Dado que la base del proyecto indica modernización con Svelte 5, debería tratarse como una deuda técnica prioritaria, porque afecta:

- mantenimiento
- compatibilidad con el estilo del proyecto
- legibilidad del código
- futuras refactorizaciones

### 5. Falta de una convención clara para “feature” vs “shared”

Hay componentes “de dominio” dentro de la carpeta ui que no están claramente diferenciados según su propósito:

- shell global
- navegación
- formularios
- tablas
- modales
- páginas de negocio
- utilidades de impresión

La falta de esta distinción hace que sea difícil saber si un nuevo componente debe ir en “core”, “ui”, o “views”.

### 6. El código de UI está muy cargado de clases inline

Se observa que muchos componentes se construyen con largas cadenas Tailwind en cada archivo. Eso funciona, pero en el mediano plazo:

- duplica estilos
- dificulta la reutilización
- dificulta testear variantes visuales
- hace más difícil la coherencia del diseño

La recomendación es centralizar tokens visuales, componentes básicos y patrones repetidos.

### 7. Algunas vistas parecen combinar la capa de presentación con la capa de negocio

En [src/lib/components/ui/views](src/lib/components/ui/views) hay muchos archivos de “pantalla” que probablemente contienen tanto lógica de la vista como lógica de negocio, filtros, formularios y estado local. Esto no es un problema si es intencional, pero cuando crece, conviene separar:

- presentational view
- data hooks
- business domain logic
- local state orchestration

---

## Recomendaciones de mejora

### Prioridad alta

#### 1. Corregir la estructura semántica

Propuesta de estructura base:

```text
src/lib/components/
  app-shell/
    TopBar/
    Sidebar/
    Tabs/
    ResizeBorder/
  features/
    account/
    customers/
    products/
    reports/
    sales/
    settings/
    print/
  shared/
    primitives/
      Button/
      Input/
      Modal/
      Table/
      Select/
      Dropdown/
    forms/
    feedback/
  navigation/
    routes.ts
    menu.ts
  index.ts
```

Esto deja claro qué es de infraestructura, qué es de negocio y qué es reutilizable.

#### 2. Migrar a Svelte 5 sin excepciones

Actualizar todos los componentes heredados a:

- `$props()`
- `$state()`
- `$effect()` si aplica
- snippets en lugar de `<slot />`
- `onclick`, `onkeydown` estándar

El caso más claro es [src/lib/components/ui/tab-bar/tab/Tab.svelte](src/lib/components/ui/tab-bar/tab/Tab.svelte) y [src/lib/components/ui/tab-bar/TabBar.svelte](src/lib/components/ui/tab-bar/TabBar.svelte).

#### 3. Unificar nomenclatura del proyecto

Elegir una convención y aplicarla en toda la carpeta:

- inglés para APIs y UI: `navigation`, `routes`, `sidebar`, `tab`, `panel`
- o español para toda la capa de producto si ese es el estándar del negocio

El punto importante es no mezclar “navegation” con “navegación”, ni “SalesView” con “VentasView” sin criterio.

### Prioridad media

#### 4. Separar routing y componentes visuales

Crear un módulo de navegación con responsabilidades bien definidas:

- definición de menú
- traducción de rutas
- mapeo a feature views
- validación de permisos

El archivo [src/lib/components/ui/views/viewRoutes.ts](src/lib/components/ui/views/viewRoutes.ts) debería reducirse a una capa más limpia o incluso moverse a un módulo de routing dedicado.

#### 5. Centralizar exports y contratos

Crear un `index.ts` por capa para evitar imports dispersos y mejorar legibilidad. Ejemplo:

- `components/shared/primitives/index.ts`
- `components/features/sales/index.ts`
- `components/navigation/index.ts`

Esto también ayuda a manejar cambios de nombre con menos impacto.

#### 6. Consolidar tipos y props compartidos

Si existen props repetidas o modelos de negocio en varios componentes, conviene centralizarlos. Por ejemplo, en un archivo de tipos de feature para pagos, reportes, clientes o facturación.

### Prioridad baja pero valiosa

#### 7. Mejorar accesibilidad y semántica HTML

Revisar todos los elementos interactivos para garantizar:

- `button` para acciones, no solo `div` con `role="button"`
- `aria-label` consistente
- navegación por teclado robusta
- enfoque visual claro

#### 8. Preparar una estrategia de pruebas visuales y unitarias

La carpeta ya tiene una prueba de ejemplo en [src/lib/components/ui/views/vouchers/voucherGenerator.spec.ts](src/lib/components/ui/views/vouchers/voucherGenerator.spec.ts). Se podría ampliar para cubrir:

- primitives básicos
- navegación de tabs
- apertura/cierre de modales
- validación de rutas

---

## Propuesta concreta de orden

Una propuesta razonable para la carpeta es:

```text
src/lib/components/
  app-shell/
    TopBar/
    Sidebar/
    WorkspaceTabs/
  features/
    sales/
      panel/
      vouchers/
      summary/
    customers/
    products/
    reports/
    account/
    settings/
    print/
  shared/
    primitives/
    forms/
    layout/
    feedback/
  navigation/
  types/
  index.ts
```

Con esta estructura:

- `app-shell` maneja la infraestructura visual
- `features` concentra funcionalidad por negocio
- `shared` centraliza lo reusable
- `navigation` toma la responsabilidad de rutas y menús
- `types` evita duplicación de contratos

---

## Conclusión

La estructura actual tiene una base sólida y varias decisiones correctas, pero está empezando a mostrar señales típicas de crecimiento sin una arquitectura de diseño explícita. El principal trabajo pendiente es volverla más semántica, más consistente y más moderna con Svelte 5.

Si se hace bien, esta limpieza no sólo mejora la legibilidad, sino también:

- robustez del mantenimiento
- facilidad de incorporación de nuevas funcionalidades
- reducción de acoplamiento
- escalabilidad del producto

La prioridad más importante es limpiar la convención de nombres, corregir la sintaxis legacy y separar claramente navegación, vistas, shared components y feature modules.
