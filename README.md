<h1 align="center">DataLint</h1>

<div align="center">
  <img src="/assets/logo@2x.png" alt="logo">
</div>

<p align="center">
  <a href="https://gitlab.tech.orange/agence-entreprises-grand-est/perfageiae">
    <img src="https://img.shields.io/badge/Perfage-1.0.0-7073f6?style=for-the-badge" alt="DataLint" />
  </a>
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  </a>
</p>


**DataLint** est un modèle de production utilisé sur le serveur de l'application **Perfage**. Ce modèle est optimisé
pour les performances avec le langage Rust et permet de scanner des fichiers CSV afin de vérifier leur conformité, en
détectant les données erronées ou dangereuses.

## Prérequis

### 1. Librairies et outils nécessaires

- [Rust](https://www.rust-lang.org/tools/install) - Le langage de programmation utilisé pour développer le modèle.
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) - L'outil de gestion des dépendances et de
  compilation pour Rust.

### 2. Dépendances externes

- **Modèle d'IA** : Un modèle d'IA pré-entraîné pour la détection de données erronées ou dangereuses, utilisant *
  *PyTorch**.
- **Tokenizer** : Un vecteur d'indexation des données pour la détection d'anomalies au format JSON.

---

## Installation et Configuration

### 1. Cloner ce dépôt

Pour récupérer le dépôt sur votre machine locale, utilisez la commande suivante :

```bash
git clone https://gitlab.tech.orange/agence-entreprises-grand-est/perfageiae.git
cd DataLint
```

### 2. Compilation du projet

Pour compiler le projet, utilisez la commande suivante :

```bash
cargo build --release
```

### 3. Exécution du projet

Pour exécuter le projet, utilisez la commande suivante :

```bash
cargo run --release "nom_du_fichier.csv" "nom_sortie.json"
```

ou

```bash
.\target\release\DataLint.exe "nom_du_fichier.csv" "nom_sortie.json"
```

## Configuration

Créer et ajouter le fichier de configuration `config.json` au même endroit que le fichier exécutable, avec le contenu
suivant :

```json
{
  "model_path": "C:\\Users\\model\\neural\\perfage_ia",
  "vocabulary_path": "C:\\Users\\tokenizer\\tokenizer.json"
}
```

## Dépendances de l'application

- **Torch** : https://pytorch.org/get-started/locally/
- **DLL** : Tous les DLL de PyTorch, à placer au même endroit que le fichier exécutable.