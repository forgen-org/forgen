1. Module Rust + WASM + UniFFI qui serde string
2. Génération des types via typeshare
3. Ajout des types au module (modification du code généré ?)
4. Création de macros pour simplifier le code
5. Création d'un CLI pour générer les types et les modules

---

FUCK UNIFFI ET WASM

- Générer un codec js/swift/kotlin
- Utiliser serde-generate
- Dans Rust, faire le serde 
- Faire des libs js/swift/kotlin facilement utilisable (via des échanges de string)
- Faire un tooling (CLI) pour générer les types

1. Générer modules natifs avec String
2. Générer les types
3. Bundle les types avec le module
4. Faire un wrapper types + module


OPTION 1 : Rust -> generates protobuf -> generates TS/Swift/Kotlin -> generates glue code (wasm / uniffi...)

OPTION 2 : Rust -> typeshare -> generate a uniffi/wasm runtime with serde -> generate glue code
