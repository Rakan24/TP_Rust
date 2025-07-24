use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read};
use chrono::Utc;

struct FileManager {
    nom_fichier: String,
}

impl FileManager {
    fn new(nom: &str) -> Self {
        FileManager {
            nom_fichier: nom.to_string(),
        }
    }

    fn creer(&self, contenu: &str) {
        if fs::write(&self.nom_fichier, contenu).is_ok() {
            println!("Fichier '{}' créé avec succès", self.nom_fichier);
        } else {
            println!("Erreur lors de la création");
        }
    }

    fn lire(&self) {
        match fs::read_to_string(&self.nom_fichier) {
            Ok(contenu) => {
                println!("--- Contenu du fichier ---");
                println!("{}", contenu);
            }
            Err(_) => println!("Impossible de lire le fichier."),
        }
    }

    fn modifier(&mut self, nouveau_contenu: &str) {
        match OpenOptions::new().write(true).append(true).open(&self.nom_fichier) {
            Ok(mut fichier) => {
                if writeln!(fichier, "\n{}", nouveau_contenu).is_ok() {
                    println!("Contenu ajouté avec succès");
                } else {
                    println!("Erreur lors de l'écriture");
                }
            }
            Err(_) => println!("Fichier introuvable"),
        }
    }

    fn supprimer(self) {
        match fs::remove_file(&self.nom_fichier) {
            Ok(_) => println!("Fichier '{}' supprimé", self.nom_fichier),
            Err(_) => println!("Erreur lors de la suppression"),
        }
    }

    fn afficher_date(&self) {
        let maintenant = Utc::now();
        println!("Date et heure actuelles : {}", maintenant.format("%d/%m/%Y %H:%M:%S"));
    }
}

fn main() {
    let mut nom_fichier = String::new();
    println!("Entrez le nom du fichier à gérer (ex: fichier_test.txt) :");
    io::stdin().read_line(&mut nom_fichier).expect("Erreur de lecture");
    let nom_fichier = nom_fichier.trim();

    let mut gestionnaire = FileManager::new(nom_fichier);

    loop {
        println!("\n----- MENU -----");
        let options = [
            "Créer le fichier",
            "Lire le fichier",
            "Modifier le fichier",
            "Supprimer le fichier",
            "Afficher la date",
            "Quitter",
        ];

        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        let mut choix = String::new();
        println!("Entrez votre choix :");
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim().parse::<usize>().unwrap_or(0);

        match choix {
            1 => {
                println!("Entrez le contenu du fichier :");
                let mut contenu = String::new();
                io::stdin().read_line(&mut contenu).expect("Erreur de lecture");
                gestionnaire.creer(contenu.trim());
            }
            2 => {
                gestionnaire.lire();
            }
            3 => {
                println!("Entrez le nouveau contenu à ajouter :");
                let mut ajout = String::new();
                io::stdin().read_line(&mut ajout).expect("Erreur de lecture");
                gestionnaire.modifier(ajout.trim());
            }
            4 => {
                gestionnaire.supprimer();
                break; // On break après suppression
            }
            5 => {
                gestionnaire.afficher_date();
            }
            6 => {
                println!("Au revoir");
                break;
            }
            _ => println!("Option invalide. Réessayez."),
        }
    }
}
