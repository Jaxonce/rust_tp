use std::io;

// Structure pour représenter un compte bancaire
struct CompteBancaire {
    nom: String,
    solde: f64,
}

// Implémentation des méthodes pour la structure CompteBancaire
impl CompteBancaire {
    // Constructeur pour créer un nouveau compte
    fn nouveau(nom: String, solde_initial: f64) -> Self {
        CompteBancaire {
            nom,
            solde: solde_initial,
        }
    }

    // Méthode pour afficher le solde
    fn afficher_solde(&self) {
        println!("\n--- AFFICHAGE DU SOLDE ---");
        println!("{}: {:.2}€", self.nom, self.solde);
    }

    // Méthode pour effectuer un retrait
    fn retirer(&mut self, montant: f64) -> bool {
        if self.solde >= montant && montant > 0.0 {
            self.solde -= montant;
            println!("Retrait de {:.2}€ effectué sur {}. Nouveau solde: {:.2}€", 
                   montant, self.nom, self.solde);
            true
        } else if montant <= 0.0 {
            println!("Montant invalide !");
            false
        } else {
            println!("Solde insuffisant ! Solde actuel: {:.2}€", self.solde);
            false
        }
    }

    // Méthode pour afficher les détails du compte
    fn afficher_details(&self) {
        println!("\n--- DÉTAILS DU COMPTE ---");
        println!("Nom: {}", self.nom);
        println!("Solde: {:.2}€", self.solde);
    }

    // Méthode pour déposer de l'argent (bonus)
    fn deposer(&mut self, montant: f64) -> bool {
        if montant > 0.0 {
            self.solde += montant;
            println!("Dépôt de {:.2}€ effectué sur {}. Nouveau solde: {:.2}€", 
                   montant, self.nom, self.solde);
            true
        } else {
            println!("Montant de dépôt invalide !");
            false
        }
    }
}

fn main() {
    // Création d'un compte bancaire avec la structure
    let mut mon_compte = CompteBancaire::nouveau(
        String::from("Mon Compte Bancaire"), 
        1000.0
    );

    loop {
        // Affichage du menu
        println!("\n=== SYSTÈME BANCAIRE (avec Structure) ===");
        let options = ["Afficher solde", "Retrait", "Dépôt", "Détails du compte", "Quitter"];
        
        for (i, option) in options.iter().enumerate() {
            println!("{}: {}", i + 1, option);
        }
        
        println!("Choisissez une option (1-5) :");
        
        // Lecture de l'entrée utilisateur
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        
        let choix: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide !");
                continue;
            }
        };
        
        match choix {
            1 => {
                // Afficher solde en utilisant la méthode
                mon_compte.afficher_solde();
            },
            
            2 => {
                // Retrait
                println!("\n--- RETRAIT ---");
                println!("Compte: {}", mon_compte.nom);
                println!("Solde actuel: {:.2}€", mon_compte.solde);
                
                println!("Montant à retirer :");
                let mut input_montant = String::new();
                io::stdin().read_line(&mut input_montant).expect("Erreur de lecture");
                
                let montant: f64 = match input_montant.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Montant invalide !");
                        continue;
                    }
                };
                
                // Utilisation de la méthode retirer
                mon_compte.retirer(montant);
            },
            
            3 => {
                // Dépôt (nouvelle fonctionnalité)
                println!("\n--- DÉPÔT ---");
                println!("Compte: {}", mon_compte.nom);
                println!("Solde actuel: {:.2}€", mon_compte.solde);
                
                println!("Montant à déposer :");
                let mut input_montant = String::new();
                io::stdin().read_line(&mut input_montant).expect("Erreur de lecture");
                
                let montant: f64 = match input_montant.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Montant invalide !");
                        continue;
                    }
                };
                
                // Utilisation de la méthode deposer
                mon_compte.deposer(montant);
            },
            
            4 => {
                // Détails du compte en utilisant la méthode
                mon_compte.afficher_details();
            },
            
            5 => {
                // Quitter
                println!("Merci d'avoir utilisé notre système bancaire !");
                println!("À bientôt !");
                break;
            },
            
            _ => {
                println!("Option invalide ! Veuillez choisir entre 1 et 5.");
            }
        }
    }
}
