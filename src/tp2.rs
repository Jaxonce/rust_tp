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
            println!("Montant de dépôt invalide ! Le montant doit être positif.");
            false
        }
    }

    // Nouvelle méthode pour renommer le compte (retourne un nouveau compte)
    fn renommer(self, nouveau_nom: String) -> Self {
        CompteBancaire {
            nom: nouveau_nom,
            solde: self.solde,
        }
    }
}

fn main() {
    // Création d'un vecteur de comptes bancaires
    let mut comptes = vec![
        CompteBancaire::nouveau(String::from("Compte Courant"), 1000.0),
        CompteBancaire::nouveau(String::from("Compte Épargne"), 500.0),
        CompteBancaire::nouveau(String::from("Compte Pro"), 2500.0),
    ];

    loop {
        // Affichage du menu
        println!("\n=== SYSTÈME BANCAIRE (avec Structure + Vec) ===");
        let options = [
            "Afficher tous les soldes", 
            "Retrait", 
            "Dépôt", 
            "Détails d'un compte",
            "Renommer un compte",
            "Quitter"
        ];
        
        for (i, option) in options.iter().enumerate() {
            println!("{}: {}", i + 1, option);
        }
        
        println!("Choisissez une option (1-6) :");
        
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
                // Afficher tous les soldes
                println!("\n--- AFFICHAGE DE TOUS LES SOLDES ---");
                for (i, compte) in comptes.iter().enumerate() {
                    println!("{}: {} - {:.2}€", i + 1, compte.nom, compte.solde);
                }
                let total: f64 = comptes.iter().map(|c| c.solde).sum();
                println!("--- Total: {:.2}€ ---", total);
            },
            
            2 => {
                // Retrait
                println!("\n--- RETRAIT ---");
                println!("Quel compte ?");
                for (i, compte) in comptes.iter().enumerate() {
                    println!("{}: {} - {:.2}€", i + 1, compte.nom, compte.solde);
                }
                
                let mut input_compte = String::new();
                io::stdin().read_line(&mut input_compte).expect("Erreur de lecture");
                
                let numero_compte: usize = match input_compte.trim().parse::<usize>() {
                    Ok(num) if num >= 1 && num <= comptes.len() => num - 1,
                    _ => {
                        println!("Numéro de compte invalide !");
                        continue;
                    }
                };
                
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
                comptes[numero_compte].retirer(montant);
            },
            
            3 => {
                // Dépôt
                println!("\n--- DÉPÔT ---");
                println!("Quel compte ?");
                for (i, compte) in comptes.iter().enumerate() {
                    println!("{}: {} - {:.2}€", i + 1, compte.nom, compte.solde);
                }
                
                let mut input_compte = String::new();
                io::stdin().read_line(&mut input_compte).expect("Erreur de lecture");
                
                let numero_compte: usize = match input_compte.trim().parse::<usize>() {
                    Ok(num) if num >= 1 && num <= comptes.len() => num - 1,
                    _ => {
                        println!("Numéro de compte invalide !");
                        continue;
                    }
                };
                
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
                
                // Utilisation de la méthode deposer (empêche les montants négatifs)
                comptes[numero_compte].deposer(montant);
            },
            
            4 => {
                // Détails d'un compte
                println!("\n--- DÉTAILS D'UN COMPTE ---");
                println!("Quel compte ?");
                for (i, compte) in comptes.iter().enumerate() {
                    println!("{}: {}", i + 1, compte.nom);
                }
                
                let mut input_compte = String::new();
                io::stdin().read_line(&mut input_compte).expect("Erreur de lecture");
                
                let numero_compte: usize = match input_compte.trim().parse::<usize>() {
                    Ok(num) if num >= 1 && num <= comptes.len() => num - 1,
                    _ => {
                        println!("Numéro de compte invalide !");
                        continue;
                    }
                };
                
                comptes[numero_compte].afficher_details();
            },
            
            5 => {
                // Renommer un compte
                println!("\n--- RENOMMER UN COMPTE ---");
                println!("Quel compte à renommer ?");
                for (i, compte) in comptes.iter().enumerate() {
                    println!("{}: {}", i + 1, compte.nom);
                }
                
                let mut input_compte = String::new();
                io::stdin().read_line(&mut input_compte).expect("Erreur de lecture");
                
                let numero_compte: usize = match input_compte.trim().parse::<usize>() {
                    Ok(num) if num >= 1 && num <= comptes.len() => num - 1,
                    _ => {
                        println!("Numéro de compte invalide !");
                        continue;
                    }
                };
                
                println!("Nouveau nom du compte :");
                let mut nouveau_nom = String::new();
                io::stdin().read_line(&mut nouveau_nom).expect("Erreur de lecture");
                let nouveau_nom = nouveau_nom.trim().to_string();
                
                // Utilisation de la méthode renommer (prend possession et retourne un nouveau compte)
                let ancien_compte = comptes.remove(numero_compte);
                let nouveau_compte = ancien_compte.renommer(nouveau_nom);
                comptes.insert(numero_compte, nouveau_compte);
                
                println!("Compte renommé avec succès !");
            },
            
            6 => {
                // Quitter
                println!("Merci d'avoir utilisé notre système bancaire !");
                println!("À bientôt !");
                break;
            },
            
            _ => {
                println!("Option invalide ! Veuillez choisir entre 1 et 6.");
            }
        }
    }
}
