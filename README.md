[English](README.md) | [正體中文](README.zh-TW.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md)

# Westend Transfer Example

This project demonstrates how to create and send a balance transfer transaction on the Westend test network using the Polkadot SDK and `subxt`.

## How It Works

1.  **Create a Rust Project**: A new Rust project was created using `cargo new westend-transfer-example`.
2.  **Install `subxt-cli`**: The `subxt-cli` tool was installed to fetch metadata from a Westend node.
3.  **Generate Rust Interface**: The `subxt codegen` command was used to fetch metadata from the Westend node's RPC endpoint and generate the `src/westend_interface.rs` file, which contains the types and functions needed to interact with the Westend chain.
4.  **Add Dependencies**: Dependencies such as `subxt`, `subxt-signer`, `tokio`, and `sp-core` were added to `Cargo.toml`.
5.  **Write Transfer Code**: In `src/main.rs`, code was written to use the generated interface to create, sign, and submit a balance transfer transaction.
6.  **Run and Verify**: The project was compiled and run using `cargo run`. The program successfully connected to the Westend network, submitted the transaction, and printed a success message upon completion.

## How to Run

To run this example, execute the following command in the project's root directory:

```bash
cargo run
```

---

## About This Project

The code for this project was generated with the assistance of Google Gemini.

Gemini is a powerful large language model that can help you with various tasks such as writing code, generating documentation, and creative brainstorming.

### Related Links

*   [Explore Gemini](https://gemini.google.com/)
*   [Gemini for Developers](https://ai.google.dev/)