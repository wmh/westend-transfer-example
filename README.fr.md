[English](README.md) | [正體中文](README.zh-TW.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md)

# Exemple de Transfert Westend

Ce projet montre comment créer et envoyer une transaction de transfert de solde sur le réseau de test Westend en utilisant le SDK Polkadot et `subxt`.

## Comment Ça Marche

1.  **Créer un Projet Rust**: Un nouveau projet Rust a été créé avec `cargo new westend-transfer-example`.
2.  **Installer `subxt-cli`**: L'outil `subxt-cli` a été installé pour récupérer les métadonnées d'un nœud Westend.
3.  **Générer l'Interface Rust**: La commande `subxt codegen` a été utilisée pour récupérer les métadonnées du point de terminaison RPC du nœud Westend et générer le fichier `src/westend_interface.rs`, qui contient les types et les fonctions nécessaires pour interagir avec la chaîne Westend.
4.  **Ajouter des Dépendances**: Des dépendances telles que `subxt`, `subxt-signer`, `tokio` et `sp-core` ont été ajoutées au `Cargo.toml`.
5.  **Écrire le Code de Transfert**: Dans `src/main.rs`, le code a été écrit pour utiliser l'interface générée afin de créer, signer et soumettre une transaction de transfert de solde.
6.  **Exécuter et Vérifier**: Le projet a été compilé et exécuté avec `cargo run`. Le programme s'est connecté avec succès au réseau Westend, a soumis la transaction et a affiché un message de succès à la fin.

## Comment Exécuter

Pour exécuter cet exemple, lancez la commande suivante à la racine du projet :

```bash
cargo run
```

---

## À Propos de ce Projet

Le code de ce projet a été généré avec l'aide de Google Gemini.

Gemini est un grand modèle de langage puissant qui peut vous aider dans diverses tâches telles que l'écriture de code, la génération de documentation et le brainstorming créatif.

### Liens Connexes

*   [Découvrez Gemini](https://gemini.google.com/)
*   [Gemini pour les Développeurs](https://ai.google.dev/)
