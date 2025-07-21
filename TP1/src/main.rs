use std::io;

// La structure du compte bancaire
#[derive(Clone)]
struct CompteBancaire {
    nom: String,
    solde: f64,
}

// Les méthodes de la structure CompteBancaire
impl CompteBancaire {
    fn afficher_solde(&self) {
        println!("Solde de {} : {:.2}€", self.nom, self.solde);
    }

    fn retrait(&mut self, montant: f64) {
        if montant <= 0.0 {
            println!("Montant invalide. Le retrait doit être positif.");
        } else if montant <= self.solde {
            self.solde -= montant;
            println!("{}€ retirés. Nouveau solde : {:.2}€", montant, self.solde);
        } else {
            println!("Fonds insuffisants !");
        }
    }

    fn depot(&mut self, montant: f64) {
        if montant <= 0.0 {
            println!("Montant invalide. Le dépôt doit être positif.");
        } else {
            self.solde += montant;
            println!("{}€ déposés. Nouveau solde : {:.2}€", montant, self.solde);
        }
    }

    // Point bonus : renommer => retourne un nouveau compte avec le nom modifié
    fn renommer(&self, nouveau_nom: String) -> CompteBancaire {
        CompteBancaire {
            nom: nouveau_nom,
            solde: self.solde,
        }
    }
}

// Le main
fn main() {
    // Création de plusieurs comptes bancaires
    let mut comptes: Vec<CompteBancaire> = vec![
        CompteBancaire { nom: String::from("Kevin"), solde: 1000.0 },
        CompteBancaire { nom: String::from("Alice"), solde: 1500.0 },
        CompteBancaire { nom: String::from("Bob"), solde: 500.0 },
    ];

    // Boucle principale
    loop {
        println!("\nMenu:");
        println!("1. Afficher solde");
        println!("2. Retrait");
        println!("3. Dépôt");
        println!("4. Liste comptes");
        println!("5. Renommer un compte");
        println!("6. Quitter");
        println!("Choix :");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");

        match choix.trim() {
            "1" => {
                println!("Entrez le nom du compte :");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).expect("Erreur");
                if let Some(compte) = comptes.iter().find(|c| c.nom == nom.trim()) {
                    compte.afficher_solde();
                } else {
                    println!("Compte introuvable");
                }
            }
            "2" => {
                println!("Entrez le nom du compte :");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).expect("Erreur");
                if let Some(compte) = comptes.iter_mut().find(|c| c.nom == nom.trim()) {
                    println!("Entrez le montant à retirer :");
                    let mut montant = String::new();
                    io::stdin().read_line(&mut montant).expect("Erreur");
                    let montant: f64 = montant.trim().parse().unwrap_or(0.0);
                    compte.retrait(montant);
                } else {
                    println!("Compte introuvable");
                }
            }
            "3" => {
                println!("Entrez le nom du compte :");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).expect("Erreur");
                if let Some(compte) = comptes.iter_mut().find(|c| c.nom == nom.trim()) {
                    println!("Entrez le montant à déposer :");
                    let mut montant = String::new();
                    io::stdin().read_line(&mut montant).expect("Erreur");
                    let montant: f64 = montant.trim().parse().unwrap_or(0.0);
                    compte.depot(montant);
                } else {
                    println!("Compte introuvable");
                }
            }
            "4" => {
                println!("Liste des comptes :");
                for (index, compte) in comptes.iter().enumerate() {
                    println!("{}. {} (Solde: {:.2}€)", index + 1, compte.nom, compte.solde);
                }
            }
            "5" => {
                println!("Entrez le nom du compte à renommer :");
                let mut ancien_nom = String::new();
                io::stdin().read_line(&mut ancien_nom).expect("Erreur");
                if let Some(position) = comptes.iter().position(|c| c.nom == ancien_nom.trim()) {
                    println!("Entrez le nouveau nom :");
                    let mut nouveau_nom = String::new();
                    io::stdin().read_line(&mut nouveau_nom).expect("Erreur");
                    let nouveau_compte = comptes[position].renommer(nouveau_nom.trim().to_string());
                    comptes[position] = nouveau_compte;
                    println!("Compte renommé avec succès !");
                } else {
                    println!("Compte introuvable");
                }
            }
            "6" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Option invalide"),
        }
    }
}
