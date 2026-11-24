mod motif;
mod perceptron;

use motif::{Motif, A, C};
use perceptron::Perceptron;

fn afficher(m: &Motif) {
    for ligne in m.chunks(5) {
        let s: String = ligne.iter().map(|&p| if p > 0.5 { '#' } else { '.' }).collect();
        println!("{s}");
    }
}

fn main() {
    println!("Motif A :");
    afficher(&A);
    println!("Motif C :");
    afficher(&C);

    let mut reseau = Perceptron::new();
    println!("Sortie originale pour A : {:?}", reseau.propager(&A));
    println!("Sortie originale pour C : {:?}", reseau.propager(&C));
    for i in 0..10 {
        reseau.apprendre(&A, [0u8, 1u8], 0.1);
        reseau.apprendre(&C, [0u8, 1u8], 0.1);
        println!("Itération {} pour A : {:?}", i, reseau.propager(&A));
        println!("Itération {} pour C : {:?}", i, reseau.propager(&C));
    }
}