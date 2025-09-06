[English](README.md) | [正體中文](README.zh-TW.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md)

# Westend 轉帳範例

本專案展示如何使用 Polkadot SDK 和 `subxt` 在 Westend 測試網路上建立並傳送一筆餘額轉帳交易。

## 運作方式

1.  **建立 Rust 專案**: 使用 `cargo new westend-transfer-example` 建立一個新的 Rust 專案。
2.  **安裝 `subxt-cli`**: 安裝 `subxt-cli` 工具，用於從 Westend 節點獲取中繼資料 (metadata)。
3.  **產生 Rust 介面**: 使用 `subxt codegen` 指令從 Westend 節點的 RPC 端點獲取中繼資料，並產生 `src/westend_interface.rs` 檔案，其中包含了與 Westend 鏈互動所需的型別和函式。
4.  **加入相依套件**: 在 `Cargo.toml` 中加入了 `subxt`, `subxt-signer`, `tokio`, 和 `sp-core` 等相依套件。
5.  **編寫轉帳程式碼**: 在 `src/main.rs` 中，編寫了使用產生的介面來建立、簽署並提交一筆餘額轉帳交易的程式碼。
6.  **執行並驗證**: 使用 `cargo run` 編譯並執行專案。程式成功連線到 Westend 網路，提交交易，並在交易完成後印出成功訊息。

## 如何執行

若要執行此範例，請在專案的根目錄下執行以下指令：

```bash
cargo run
```

---

## 關於此專案

此專案的程式碼由 Google Gemini 協助生成。

Gemini 是一個功能強大的大型語言模型，可以幫助您完成編寫程式碼、產生文件、創意發想等各種任務。

### 相關連結

*   [探索 Gemini](https://gemini.google.com/)
*   [Gemini 開發者專頁](https://ai.google.dev/)
