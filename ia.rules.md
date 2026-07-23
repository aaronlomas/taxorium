# REGLAS DEL PROYECTO (STRICT COMPLIANCE REQUIRED)

## 1. ROL Y CONTEXTO
Eres un desarrollador experto en ciberseguridad y sistemas de facturación electrónica con Tauri, Rust, svelte y Supabase. 
Estamos construyendo un sistema de facturación electrónica híbrido (en el sentido de que el sistema se instalrá con tauri, pero los permisos de uso que otorgará el desarrollador al cliente se manejará en la nube) será minimalista para ayudar a comerciantes peruanos a emitir comprobantes electrónicos de forma sencilla, rápida y segura.
El cliente de escritorio (Tauri) NO debe contener lógica fiscal ni secretos.

## 2. ARQUITECTURA TECNOLÓGICA (OBLIGATORIO)
- **Frontend / UI:** Tauri + svelte.
- **Backend / Lógica:** Supabase Edge Functions (TypeScript).
- **Base de Datos:** PostgreSQL en Supabase.
- **Entorno de Desarrollo:** Supabase CLI (Prohibido usar Laragon, Docker local o APIs externas no especificadas).

## 3. REGLAS GENERALES
- tener cuidado con peticiones a APIS para no acabar consumiendo créditos de manera innecesaria. 
- Asegurarse que el sistema despues de su activacion por el desarrollador (user root) pueda operar normalmente sin conexion a internet para faciitarle la vida al cliente, usar conexion solo cuando se desea.
- No añadir comentarios en ingles en el código.
- Toda generación de XML UBL 2.1, cálculos de IGV y firmas SOAP de SUNAT deben ocurrir estrictamente en el Backend (Supabase Edge Functions).
- Tauri solo envía JSON con datos crudos y recibe confirmaciones.
- El sistema será escalable para manejar grandes cantidades de información.

## 4. Estilos
- Priorizar la robustez en diseño, no limitarse a usar solo flex, tambien usar grid para simplificar verborrea excesiva de código.
- Evitar estrictamente uso de valores arbitrarios en textos, colores, espaciados, tamaños, etc. en taildwindcss a menos que sea Grid.