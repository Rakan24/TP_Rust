use std::io;



//La structure du compte bancaire
struct CompteBancaire {
    nom: String,
    solde: f64,
}


//Les méthodes de la structure CompteBancaire
impl CompteBancaire {
    fn afficher_solde(&self) {
        println!("Solde de {} : {:.2}€", self.nom, self.solde);
    }

    fn retrait(&mut self, montant: f64) {
        if montant <= self.solde {
            self.solde -= montant;
            println!("{}€ retirés. Nouveau solde : {:.2}€", montant, self.solde);
        } else {
            println!("Fonds insuffisants !");
        }
    }
}


//Le main
fn main() {


    //Création de 2 comptes bancaires pour tests 
    let mut comptes = vec![
        CompteBancaire { nom: String::from("Kevin"), solde: 1000.0 },
        CompteBancaire { nom: String::from("Alice"), solde: 1500.0 },
    ];



    //Boucle principale
    loop {
        println!("\nMenu:");
        println!("1. Afficher solde");
        println!("2. Retrait");
        println!("3. Liste comptes");
        println!("4. Quitter");
        println!("Choix :");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");


        //Equivalent du switch en Rust
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
                println!("Liste des comptes :");
                for compte in &comptes {
                    println!("- {}", compte.nom);
                }
            }
            "4" => {
                println!("Au revoir");
                break;
            }
            _ => println!("Option invalide"),
        }
    }
}


