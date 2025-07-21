use std::io;

fn main() {
    // Variable pour simuler un compte bancaire simple
    let mut solde_compte: f64 = 1000.0;
    let nom_compte = "Mon Compte Bancaire";
    
    loop {
        // Affichage du menu
        println!("\n=== SYSTÈME BANCAIRE ===");
        let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];
        
        for (i, option) in options.iter().enumerate() {
            println!("{}: {}", i + 1, option);
        }
        
        println!("Choisissez une option (1-4) :");
        
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
                // Afficher solde
                println!("\n--- AFFICHAGE DU SOLDE ---");
                println!("{}: {:.2}€", nom_compte, solde_compte);
            },
            
            2 => {
                // Retrait
                println!("\n--- RETRAIT ---");
                println!("Compte: {}", nom_compte);
                println!("Solde actuel: {:.2}€", solde_compte);
                
                println!("Montant à retirer :");
                let mut input_montant = String::new();
                io::stdin().read_line(&mut input_montant).expect("Erreur de lecture");
                
                let montant: f64 = match input_montant.trim().parse() {
                    Ok(num) if num > 0.0 => num,
                    _ => {
                        println!("Montant invalide !");
                        continue;
                    }
                };
                
                // Effectuer le retrait
                if solde_compte >= montant {
                    solde_compte -= montant;
                    println!("Retrait de {:.2}€ effectué sur {}. Nouveau solde: {:.2}€", 
                           montant, nom_compte, solde_compte);
                } else {
                    println!("Solde insuffisant ! Solde actuel: {:.2}€", solde_compte);
                }
            },
            
            3 => {
                // Liste comptes
                println!("\n--- DÉTAILS DU COMPTE ---");
                println!("Nom: {}", nom_compte);
                println!("Solde: {:.2}€", solde_compte);
            },
            
            4 => {
                // Quitter
                println!("Merci d'avoir utilisé notre système bancaire !");
                println!("À bientôt !");
                break;
            },
            
            _ => {
                println!("Option invalide ! Veuillez choisir entre 1 et 4.");
            }
        }
    }
}
