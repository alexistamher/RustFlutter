# Flutter + Rust (FRB v2) Showcase 🚀

Este repositorio es una demostración práctica de cómo integrar **Rust** en aplicaciones **Flutter** utilizando [flutter_rust_bridge](https://cjycode.com/flutter_rust_bridge/) (v2). 

El proyecto demuestra la potencia de Rust para manejar lógica de negocio pesada, networking y persistencia, delegando estas tareas fuera de Dart para obtener mayor rendimiento y seguridad de memoria.

## 🌟 Características

- **Puente Rust-Flutter (FRB v2):** Comunicación bidireccional eficiente entre Dart y Rust.
- **Networking Asíncrono:** Ejemplo de peticiones HTTP CRUD utilizando la librería `reqwest` en Rust.
- **Persistencia de Datos:** Demostración de manejo de estado y lógica de datos desde el lado de Rust.
- **Servidor de Mock (Actix-web):** Incluye un pequeño servidor en Rust para probar las peticiones HTTP localmente.
- **Multiplataforma:** Configuración optimizada para Android, iOS y Desktop (macOS/Windows/Linux).

## 🏗️ Estructura del Proyecto

El repositorio se divide en tres componentes principales:

- **`flutter_app`**: La interfaz de usuario construida con Flutter.
- **`tasks_plugin`**: El plugin que contiene el core de Rust (`/rust/src`).
- **`tasks_server`**: Un servidor backend de ejemplo construido con Actix-web.

## 🚀 Comenzando

### Requisitos Previos

- [Flutter SDK](https://docs.flutter.dev/get-started/install)
- [Rust](https://www.rust-lang.org/tools/install)
- `flutter_rust_bridge_codegen`:
  ```bash
  cargo install flutter_rust_bridge_codegen
  ```

### Ejecutar el Proyecto

1. **Inicia el servidor backend:**
   ```bash
   cd tasks_server
   cargo run
   ```

2. **Genera el código del puente (si haces cambios en Rust):**
   ```bash
   cd tasks_plugin
   flutter_rust_bridge_codegen generate
   ```

3. **Ejecuta la aplicación Flutter:**
   ```bash
   cd flutter_app
   flutter run
   ```

## 🛠️ Ejemplos Implementados

### 1. Peticiones HTTP (Rust)
Se utiliza `reqwest` y `tokio` para realizar operaciones CRUD asíncronas hacia el servidor Actix. La lógica de red está completamente aislada en Rust, devolviendo objetos serializados a Dart mediante `serde`.

### 2. Base de Datos / Persistencia
*Nota: Este ejemplo demuestra cómo Rust puede gestionar el ciclo de vida de los datos localmente.*

## 📄 Licencia

Este proyecto está bajo la Licencia MIT.
