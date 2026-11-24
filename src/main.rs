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

    let reseau = Perceptron::new();
    println!("Sortie pour A : {:?}", reseau.propager(&A));
    println!("Sortie pour C : {:?}", reseau.propager(&C));
}