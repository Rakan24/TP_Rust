# TP_Rust


## Séance 1 :

# Créer un nouveau projet
cargo new nom_du_projet

=> Cela crée une arborescence complète de projet.

# Compiler le projet
cargo build

=> Cela compile le projet en mode debug.

# Exécuter le projet
cargo run

=> Compile et exécute directement le projet.

# Fonctions
Déclaration d'une fonction:

fn ma_fonction() {
    println!("Bonjour");
}


# Afficher du texte
Afficher du texte dans le terminal :

println!("Hello, world!");


# Conventions d'écriture
- Ecriture en snake case :
  - `let age_utilisateur = 25;`
  - `fn afficher_message() { ... }`
- Les noms de variables :
  - Ne commencent jamais par un chiffre
  - N’utilisent pas de majuscules (sauf pour les constantes)
  - Pas de tirets ("-"), uniquement des underscores ("_")




## Séance 2 :

# Les structures
Permettent de modéliser des objets.

struct CompteBancaire {
    nom: String,
    solde: f64,
}


# Les fonctions

Les méthodes des structures

impl CompteBancaire {
    fn afficher_solde(&self) {
        println!("Solde: {}€", self.solde);
    }

    fn retrait(&mut self, montant: f64) {
        self.solde -= montant;
    }
}


# Boucle infinie :

loop {
    println!("Je tourne en boucle !");
}


# Boucle avec condition :

while condition {
    // Instructions
}


# Boucle for :

for i in 0..5 {
    println!("i vaut : {}", i);
}


# Compteurs

Utilisés souvent avec les boucles "for" ou "while".


let mut compteur = 0;
while compteur < 3 {
    println!("Compteur : {}", compteur);
    compteur += 1;
}


# Les références

# "&self" => Lecture seule

On peut lire les champs mais pas les modifier :

fn afficher(&self) {
    println!("Nom : {}", self.nom);
}


# "&mut self" => Modification possible

Permet de modifier les champs :


fn changer_nom(&mut self, nouveau_nom: String) {
    self.nom = nouveau_nom;
}


# "self" => Transfert de propriété (ownership)

La méthode prend la possession complète de la structure :


fn consommer(self) {
    println!("Le compte de {} est consommé", self.nom);
}



## Séance 3 :

# Gestion de fichiers avec Rust

Permet de lire, écrire, modifier et supprimer des fichiers.

use std::fs;

fs::write("fichier.txt", "Contenu");
let contenu = fs::read_to_string("fichier.txt").unwrap();
fs::remove_file("fichier.txt").unwrap();


# Structures : Exemples complémentaires


struct FileManager {
    nom_fichier: String,
}


# Implémentation avec `impl`

Utiliser des méthodes associées à une structure.


impl FileManager {
    fn creer(&self) {
        // code de création
    }

    fn lire(&self) {
        // code de lecture
    }
}


# Match

Permet de tester plusieurs cas.


let choix = 2;
match choix {
    1 => println!("Créer"),
    2 => println!("Lire"),
    _ => println!("Autre"),
}


# Lecture utilisateur

Lire une ligne saisie au clavier :


use std::io;

let mut entree = String::new();
io::stdin().read_line(&mut entree).expect("Erreur");


# Date & Heure avec Chrono

Afficher la date actuelle au format UTC ou français :


use chrono::Utc;

let maintenant = Utc::now();
println!("{}", maintenant.format("%d/%m/%Y %H:%M:%S"));


# Ownership (propriétaire)

Chaque variable possède sa donnée. Une seule référence propriétaire à la fois.


let nom = String::from("Kevin");
let copie = nom.clone(); // nom encore accessible


# Membership (appartenance)

Les données font partie d’une structure :


struct User {
    nom: String,
    secu: String,
}


# Menu interactif avec match et boucle

Afficher un menu, lire un choix, et exécuter une action :


let options = ["Créer", "Lire", "Modifier", "Supprimer"];

for (i, option) in options.iter().enumerate() {
    println!("{}. {}", i + 1, option);
}

let mut choix = String::new();
io::stdin().read_line(&mut choix).unwrap();

let choix: usize = choix.trim().parse().unwrap();

match choix {
    1 => println!("Créer fichier"),
    2 => println!("Lire fichier"),
    _ => println!("Option invalide"),
}

