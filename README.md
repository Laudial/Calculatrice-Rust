# Calculatrice Rust

Une simple calculatrice en ligne de commande écrite en Rust. Cette application permet d'effectuer des opérations de base telles que l'addition, la soustraction, la multiplication et la division.

## Fonctionnalités

- Addition
- Soustraction
- Multiplication
- Division

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install)

## Installation

1. Clonez le dépôt :

    ```sh
    git clone https://github.com/Laudial/Calculatrice-Rust
    cd calculatrice-rust
    ```

2. Compilez le projet :

    ```sh
    cargo build
    ```

## Utilisation

1. Exécutez la calculatrice :

    ```sh
    cargo run
    ```

2. Suivez les instructions à l'écran pour entrer les opérations et les nombres.

## Exemple

```sh
$ cargo run
Entrez la première opérande: 5
Entrez l'opérateur (+, -, *, /): +
Entrez la seconde opérande: 3
Résultat: 8
```

### Structure du projet

```txt
calculatrice-rust/
├── Cargo.toml
└── src
    └── main.rs
```

### Contribuer

Les contributions sont les bienvenues ! Veuillez ouvrir une issue ou soumettre une pull request.

### License

Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus de détails.