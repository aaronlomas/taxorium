# Reporte de orden y robustez de archivos TypeScript

Fecha: 2026-10-08
Proyecto: Taxorium

## 1. Resumen ejecutivo

La estructura actual tiene una base sólida y clara en términos de separación conceptual:

- `src/lib/components` agrupa UI y vistas.
- `src/lib/services` agrupó clientes de dominio (`customers`, `products`, `sellers`, `vouchers`, `catalogos`).
- `src/lib/stores` centraliza estado reactivo de la aplicación.
- `src/lib/utilities` mantiene helpers y formatters.

Esto ya es una buena base para un proyecto mediano y da señales de intención de diseño. El principal problema no es la ausencia de arquitectura, sino la mezcla de niveles: hay dominio, infraestructura y transporte en la misma carpeta; además, hay varias inconsistencias de nombres y de ubicación que pueden crecer en complejidad.

En síntesis: la estructura es útil, pero aún no está del todo “robusta” para crecimiento sostenido.

---

## 2. Evaluación del estado actual

### 2.1 Fortalezas

1. Separación funcional por capas
   - La aplicación distingue bien entre UI, estado y acceso a datos.
   - Ejemplos: `src/lib/components`, `src/lib/stores`, `src/lib/services`.

2. Lógica de transporte centralizada
   - `src/lib/services/apiClient.ts` encapsula la rotación entre Tauri IPC y HTTP.
   - Esto reduce duplicación y evita que cada servicio reimplemente fetch/invoke.

3. Estado reactivo limpio con Svelte stores
   - Archivos como `src/lib/stores/auth.ts` y `src/lib/stores/config.ts` siguen un patrón consistente de `createXStore()` + export constante.
   - Esto funciona bien para escenarios de UI reactiva.

4. Almacenamiento de utilidades por tipo de responsabilidad
   - `src/lib/utilities/formats` para formateo y export está bien ubicado.

### 2.2 Debilidades evidentes

1. Mezcla de niveles dentro de `src/lib/services`
   - Hay servicios de dominio (`customers`, `products`, `sellers`, `vouchers`) y también infraestructura (`apiClient.ts`, `configLocal/clientConfigLocal.ts`).
   - Es decir, la carpeta `services` mezcló: transporte, configuración local del sistema y acceso a dominios.

2. Convenciones de nombres inconsistentes
   - `clientCustomer.ts`, `clientProducts.ts`, `clientSellers.ts`, `clientVoucher.ts`, `clientCatalogos.ts`.
   - No hay un criterio único entre singular/plural, nombre de clase, sufijo `Client` y carpeta `customers` vs `catalogos`.
   - El proyecto usa español en dominio, pero algunas entidades están en inglés/mixto (`apiClient`, `configLocal`, `sellerAuth`, `taxoLog`, `tenant`).

3. Tipos duplicados y acoplados a la implementación
   - Las interfaces de dominio aparecen embebidas directamente en cada cliente (`customerClient.ts`, `clientCatalogos.ts`, etc.).
   - Esto hace que el modelo de negocio no esté centralizado y aumenta el riesgo de divergencia cuando se agregan campos.

4. `src/lib/index.ts` no aporta una API pública clara
   - Actualmente solo tiene un comentario placeholder.
   - Esto sugiere que el proyecto aún no tiene un “barrel” bien definido para exportar módulos reutilizables.

5. `src/lib/stores` parece estar absorbiendo responsabilidades de dominio y de UI
   - Hay stores para auth, catalogos, ventas, tenant, etc.
   - Eso no es incorrecto, pero si el proyecto crece, será importante decidir si los stores representan “estado de UI” o “estado del dominio”.

6. `configLocal` tiene semántica de capa de infraestructura, no de servicio de negocio
   - El archivo y la lógica disponible muestran que se trata de acceso a la configuración local del dispositivo, no de un servicio de dominio.

---

## 3. Evaluación de orden por carpetas

### Estado actual

```text
src/
  lib/
    components/
    database.types.ts
    services/
      apiClient.ts
      catalogos/
      configLocal/
      customers/
      products/
      sellers/
      vouchers/
    stores/
      auth.ts
      catalogos.ts
      config.ts
      customers.ts
      products.ts
      sales.ts
      sellerAuth.ts
      sellers.ts
      taxoLog.ts
      tenant.ts
    supabase.ts
    utilities/
```

### Lo que está bien

- Visualmente, la estructura tiene sentido para un proyecto Svelte.
- Todas las capas dominantes están representadas.
- Los servicios de dominio están agrupados por entidad principal.

### Lo que necesita mejora

- `services` no representa una sola categoría semántica: hay API, clientes de negocio, acceso local y transporte.
- `stores` parece estar mezclando estado global de sesión, catálogo, configuración, ventas y autenticación del vendedor.
- Faltan carpetas explícitas para `core`, `shared`, `features` o `integrations`.

---

## 4. Recomendación de ordenamiento y diseño

### 4.1 Estructura objetivo recomendada

La propuesta más sólida para este tipo de proyecto es una mezcla entre enfoque por dominio y capas compartidas:

```text
src/lib/
  core/
    http/
      apiClient.ts
      errors.ts
      response.ts
    types/
      api.ts

  features/
    auth/
      api.ts
      store.ts
      types.ts
      index.ts
    catalogos/
      api.ts
      store.ts
      types.ts
      index.ts
    customers/
      api.ts
      store.ts
      types.ts
      index.ts
    products/
      api.ts
      store.ts
      types.ts
      index.ts
    sellers/
      api.ts
      store.ts
      types.ts
      index.ts
    vouchers/
      api.ts
      store.ts
      types.ts
      index.ts
    tenant/
      api.ts
      store.ts
      types.ts
      index.ts

  integrations/
    supabase/
      client.ts
    tauri/
      config.ts
      deviceConfig.ts

  shared/
    formats/
      export.ts
    utils/
    constants/

  app/
    routes/
    layouts/

  components/
```

### 4.2 Regla semántica recomendada

- `core/` = infraestructura reutilizable general.
- `features/` = capacidades de negocio o flujos de usuario.
- `integrations/` = adaptadores con terceros o sistema operativo (`Supabase`, `Tauri`, `HTTP`).
- `shared/` = utilidades sin dependencia de negocio.
- `components/` = presentación.

Esto produce menor ambigüedad que mantener todo dentro de `services`, `stores` y `utilities` sin diferenciación de capas.

---

## 5. Recomendaciones de nombres y convención

### 5.1 Preferir un criterio único de lenguaje

Para un proyecto con dominio en español, recomiendo definir una convención clara:

- Mantener nombres de dominio en español: `customers` / `productos` / `vouchers` / `catalogos`.
- Preferir `api.ts` o `service.ts` antes que `clientCustomer.ts`.
- Evitar `clientX` cuando el archivo ya está en una carpeta por dominio.

Ejemplos recomendados:

- `src/lib/features/customers/api.ts`
- `src/lib/features/customers/store.ts`
- `src/lib/features/customers/types.ts`
- `src/lib/integrations/tauri/deviceConfig.ts`

### 5.2 Evitar nombres de archivo basados en implementación

Esto es importante:

- `clientCustomer.ts` comunica “implementación de cliente HTTP” más que “dominio customer”.
- `apiClient.ts` es buena base, pero el nombre del dominio debe estar en el contenedor, no en el nombre del archivo.

Preferible:

- `customers/api.ts`
- `customers/service.ts`
- `customers/store.ts`

No recomendable:

- `clientCustomer.ts`
- `clientCatalogos.ts`
- `sellerAuth.ts` cuando es más un store de sesión que una entidad de vendedor.

---

## 6. Agrupación de servicios recomendada

### 6.1 Servicios de negocio

Estos deben vivir bajo `features/*` o `domains/*`:

- `customers`
- `products`
- `sellers`
- `vouchers`
- `catalogos`
- `auth`
- `tenant`
- `license`

Cada uno debe incluir:

- `types.ts`: interfaces y tipos del dominio.
- `api.ts`: peticiones HTTP/Tauri del recurso.
- `store.ts`: estado consumible por Svelte.
- `index.ts`: exportación centralizada del feature.

### 6.2 Infraestructura compartida

Debe quedar fuera del dominio:

- `core/http/apiClient.ts`
- `integrations/supabase/client.ts`
- `integrations/tauri/deviceConfig.ts`

Esto evita que cada feature conozca detalles de transporte o del sistema operativo.

---

## 7. Mejores prácticas para mantener calidad

1. Centralizar tipos de dominio
   - No repetir interfaces en cada cliente.
   - Crear una fuente única para cada entidad.

2. Un solo responsable por módulo
   - Cada feature debe exponer una API única y predecible.

3. Conservar una capa de adaptación
   - La lógica de HTTP/Tauri debe ser “adaptadora”, no parte del dominio.

4. Separar UI de estado de negocio
   - Stores deben reflejar estado útil para la vista, pero no contener lógica de presentación compleja.

5. Mantener una firma consistente para APIs
   - `get*`, `create*`, `update*`, `delete*` es claro y predecible.
   - Si se implementa más de un canal (HTTP + Tauri), la capa superior debe ocultarlo.

---

## 8. Recomendación concreta para este proyecto

### Prioridad alta

- Mover `apiClient.ts` a `src/lib/core/http/`.
- Crear `src/lib/features/` y migrar los dominios actuales.
- Mover `supabase.ts` a `src/lib/integrations/supabase/client.ts`.
- Mover `configLocal` a `src/lib/integrations/tauri/deviceConfig.ts`.

### Prioridad media

- Crear `src/lib/features/*/types.ts` y sacar los tipos de cada cliente.
- Unificar nombres y decidir una regla: español consistente o inglés consistente.
- Definir un `index.ts` por feature para exportar paquetes limpios.

### Prioridad baja

- Rediseñar `src/lib/index.ts` para una API pública más clara.
- Reubicar utilidades de formato bajo `shared/` si el proyecto sigue creciendo.

---

## 9. Conclusión

La estructura actual ya tiene una organización razonable y una lógica funcional clara, pero todavía no está totalmente preparada para escalar sin fricción. La mayor oportunidad de mejora está en separar mejor dominio, transporte e infraestructura, y en normalizar nomenclatura y ubicación de archivos.

Si se siguen las recomendares anteriores, el proyecto será más legible, más mantenible y más fácil de extender sin introducir acoplamientos accidentales.

---

## 10. Propuesta de refinamiento mínimo viable

Si se quiere hacer un refactor gradual sin romper mucho código, una versión prudente sería:

```text
src/lib/
  core/
    apiClient.ts
  features/
    customers/
    products/
    sellers/
    vouchers/
    catalogos/
    auth/
  integrations/
    supabase.ts
    tauri/
  stores/
    legacy/   # solo temporal si hace falta compatibilidad
  utilities/
```

Esto permite mover el proyecto a una arquitectura más limpia sin una reescritura completa en una sola etapa.
