[English](README.md) | [正體中文](README.zh-TW.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md)

# Westend送金サンプル

このプロジェクトは、Polkadot SDKと`subxt`を使用して、Westendテストネットワークで残高転送トランザクションを作成および送信する方法を示します。

## 仕組み

1.  **Rustプロジェクトの作成**: `cargo new westend-transfer-example`を使用して新しいRustプロジェクトが作成されました。
2.  **`subxt-cli`のインストール**: Westendノードからメタデータを取得するために`subxt-cli`ツールがインストールされました。
3.  **Rustインターフェースの生成**: `subxt codegen`コマンドを使用して、WestendノードのRPCエンドポイントからメタデータを取得し、Westendチェーンとの対話に必要な型と関数を含む`src/westend_interface.rs`ファイルを生成しました。
4.  **依存関係の追加**: `subxt`、`subxt-signer`、`tokio`、`sp-core`などの依存関係が`Cargo.toml`に追加されました。
5.  **送金コードの記述**: `src/main.rs`で、生成されたインターフェースを使用して残高転送トランザクションを作成、署名、送信するコードが記述されました。
6.  **実行と検証**: `cargo run`を使用してプロジェクトがコンパイルおよび実行されました。プログラムは正常にWestendネットワークに接続し、トランザクションを送信し、完了時に成功メッセージを出力しました。

## 実行方法

このサンプルを実行するには、プロジェクトのルートディレクトリで次のコマンドを実行します。

```bash
cargo run
```

---

## このプロジェクトについて

このプロジェクトのコードは、Google Geminiの支援を受けて生成されました。

Geminiは、コードの作成、ドキュメントの生成、クリエイティブなブレインストーミングなど、さまざまなタスクを支援できる強力な大規模言語モデルです。

### 関連リンク

*   [Geminiを探索する](https://gemini.google.com/)
*   [開発者向けGemini](https://ai.google.dev/)
