mod motif;
mod perceptron;
mod apprentissage;

use motif::{Motif, A, C};

fn afficher(m: &Motif) {
    for ligne in m.chunks(5) {
        let s: String = ligne.iter().map(|&p| if p > 0.5 { '#' } else { '.' }).collect();
        println!("{s}");
    }
}

fn afficher_historique (titre: &str, historique: &[(usize, usize)]) {
    println!("{titre}");
    for (i, (erreur_a, erreur_c)) in historique.iter().enumerate() {
        println!("étape {i} : erreur A = {erreur_a}, erreur C = {erreur_c}");
    }
}

fn main() {
    println!("Motif A :");
    afficher(&A);
    println!("Motif C :");
    afficher(&C);

    let mu = 0.02;

    let (reseau1, historique1) = apprentissage::methode_sequentielle(0.02);
    afficher_historique("Méthode séquentielle", &historique1);
    println!("réseau correct : {}\n", apprentissage::est_correct(&reseau1));

    let (reseau2, historique2) = apprentissage::methode_alterne(mu);
    afficher_historique("Méthode alternée", &historique2);
    println!("réseau correct : {}", apprentissage::est_correct(&reseau2));
}