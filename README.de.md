[English](README.md) | [正體中文](README.zh-TW.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md)

# Westend Überweisungsbeispiel

Dieses Projekt zeigt, wie man mit dem Polkadot SDK und `subxt` eine Guthabenüberweisung im Westend-Testnetzwerk erstellt und sendet.

## Wie es funktioniert

1.  **Rust-Projekt erstellen**: Ein neues Rust-Projekt wurde mit `cargo new westend-transfer-example` erstellt.
2.  **`subxt-cli` installieren**: Das `subxt-cli`-Tool wurde installiert, um Metadaten von einem Westend-Knoten abzurufen.
3.  **Rust-Schnittstelle generieren**: Der Befehl `subxt codegen` wurde verwendet, um Metadaten vom RPC-Endpunkt des Westend-Knotens abzurufen und die Datei `src/westend_interface.rs` zu generieren, die die für die Interaktion mit der Westend-Chain erforderlichen Typen und Funktionen enthält.
4.  **Abhängigkeiten hinzufügen**: Abhängigkeiten wie `subxt`, `subxt-signer`, `tokio` und `sp-core` wurden zur `Cargo.toml` hinzugefügt.
5.  **Überweisungscode schreiben**: In `src/main.rs` wurde Code geschrieben, um die generierte Schnittstelle zum Erstellen, Signieren und Senden einer Guthabenüberweisung zu verwenden.
6.  **Ausführen und überprüfen**: Das Projekt wurde mit `cargo run` kompiliert und ausgeführt. Das Programm hat sich erfolgreich mit dem Westend-Netzwerk verbunden, die Transaktion übermittelt und nach Abschluss eine Erfolgsmeldung ausgegeben.

## Wie man es ausführt

Um dieses Beispiel auszuführen, führen Sie den folgenden Befehl im Stammverzeichnis des Projekts aus:

```bash
cargo run
```

---

## Über dieses Projekt

Der Code für dieses Projekt wurde mit Unterstützung von Google Gemini generiert.

Gemini ist ein leistungsstarkes großes Sprachmodell, das Ihnen bei verschiedenen Aufgaben wie dem Schreiben von Code, dem Erstellen von Dokumentationen und dem kreativen Brainstorming helfen kann.

### Verwandte Links

*   [Entdecken Sie Gemini](https://gemini.google.com/)
*   [Gemini für Entwickler](https://ai.google.dev/)
