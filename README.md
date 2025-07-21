# Introduction à Rust

## Liens utiles
- [Documentation officielle de Rust](https://doc.rust-lang.org/book/)

## Installation de Rust
Pour installer Rust :
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh
```

## Cargo
Cargo -> gestionnaire de paquets + compilation pour Rust. 

gérer les dépendances, compiler le code et créer des paquets.

### Initialisation d'un projet
```bash
cargo new nom_du_projet
cd nom_du_projet
```

### Compilation et exécution
```bash
cargo run
```

## Les bases de Rust

### Variables et types
Rust est un langage statiquement typé:
```rust
let x: i32 = 5; // variable immuable
let mut y: i32 = 10; // variable mutable
y += 5; // modification de la variable mutable
```

### Fonctions
fonctions définies avec le mot-clé `fn`. Voici un exemple :
```rust
fn ajouter(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let resultat = ajouter(5, 10);
    println!("Le résultat est : {}", resultat);
}
```
### IF et boucles
expressions conditionnelles et boucles.
```rust
fn main() {
    for i in 0..5 {
        if i % 2 == 0 {
            println!("{} est pair", i);
        } else {
            println!("{} est impair", i);
        }
    }
}
```
### Iterateurs
parcourir les collections.
```rust
fn main() {
    let options = ["Option 1", "Option 2", "Option 3"];
    for (i, option) in options.iter().enumerate() {
        println!("{}: {}", i+1, option);
    }
}
```
### Input utilisateur
lire l'entrée de l'utilisateur => bibliothèque standard :
```rust
use std::io;

fn main() {
    let mut input = String::new(); // Mut : variable mutable 
    println!("Entrez un nombre :");
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    let input:usize = input.trim().parse().expect("Veuillez entrer un nombre");
    println!("Vous avez entré : {}", input);
    if input % 2 == 0 {
        println!("{} est pair", input);
    } else {
        println!("{} est impair", input);
    }
    println!("Fin du programme");
}
```

### Loop
Boucle infinie jusqu'à `break` :
```rust
fn main() {
    let mut compteur = 0;
    loop {
        println!("Compteur: {}", compteur);
        compteur += 1;
        if compteur == 3 {
            break; // sortie de boucle
        }
    }
}
```

### Struct et impl
Structures + méthodes associées :
```rust
struct Personne {
    nom: String,
    age: u32,
}

impl Personne {
    fn nouveau(nom: String, age: u32) -> Self {
        Personne { nom, age }
    }
    
    fn afficher(&self) {  // emprunt immuable (lecture seule)
        println!("Nom: {}", self.nom);
    }
    
    fn vieillir(&mut self) {  // emprunt mutable (peu etre modifié)
        self.age += 1;
    }
}

fn main() {
    let mut p = Personne::nouveau("Kevin".to_string(), 25);
    p.afficher();
    p.vieillir();
}
```

### Fonctions associées (&self, &mut self)
- `&self` : lecture seule (emprunt immuable)
- `&mut self` : modification possible (emprunt mutable)
- `self` : prend possession (déplace la valeur)

### Le underscore (_)
Ignore une valeur :
```rust
let _ = fonction_qui_retourne_quelque_chose(); // ignore le retour
let (_premier, second) = (1, 2); // ignore premier
```

### Gestion des erreurs
`match` pour gérer `Result` et `Option` :
```rust
fn main() {
    let input = "42";
    let nombre: i32 = match input.parse() {
        Ok(n) => n,           // succès
        Err(_) => {           // erreur
            println!("Erreur de parsing");
            return;
        }
    };
    
    // Ou avec expect() pour paniquer en cas d'erreur
    let nombre2: i32 = input.parse().expect("Nombre invalide");
}
```