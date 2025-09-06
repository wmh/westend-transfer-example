[English](README.md) | [正體中文](README.zh-TW.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md)

# Ejemplo de Transferencia en Westend

Este proyecto demuestra cómo crear y enviar una transacción de transferencia de saldo en la red de prueba Westend utilizando el SDK de Polkadot y `subxt`.

## Cómo Funciona

1.  **Crear un Proyecto de Rust**: Se creó un nuevo proyecto de Rust usando `cargo new westend-transfer-example`.
2.  **Instalar `subxt-cli`**: Se instaló la herramienta `subxt-cli` para obtener los metadatos de un nodo de Westend.
3.  **Generar Interfaz de Rust**: Se usó el comando `subxt codegen` para obtener los metadatos del punto de conexión RPC del nodo de Westend y generar el archivo `src/westend_interface.rs`, que contiene los tipos y funciones necesarios para interactuar con la cadena Westend.
4.  **Añadir Dependencias**: Se añadieron dependencias como `subxt`, `subxt-signer`, `tokio` y `sp-core` al `Cargo.toml`.
5.  **Escribir Código de Transferencia**: En `src/main.rs`, se escribió el código para usar la interfaz generada para crear, firmar y enviar una transacción de transferencia de saldo.
6.  **Ejecutar y Verificar**: El proyecto se compiló y ejecutó usando `cargo run`. El programa se conectó con éxito a la red de Westend, envió la transacción e imprimió un mensaje de éxito al completarse.

## Cómo Ejecutar

Para ejecutar este ejemplo, ejecute el siguiente comando en el directorio raíz del proyecto:

```bash
cargo run
```

---

## Sobre Este Proyecto

El código de este proyecto fue generado con la ayuda de Google Gemini.

Gemini es un potente modelo de lenguaje grande que puede ayudarte con diversas tareas como escribir código, generar documentación y lluvia de ideas creativas.

### Enlaces Relacionados

*   [Explora Gemini](https://gemini.google.com/)
*   [Gemini para Desarrolladores](https://ai.google.dev/)
