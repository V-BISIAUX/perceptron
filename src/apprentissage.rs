use crate::motif::{Motif, A, C};
use crate::perceptron::{Perceptron, N_SORTIES};

const CIBLE_A: [u8; N_SORTIES] = [1, 0];
const CIBLE_C: [u8; N_SORTIES] = [0, 1];
const MAX_ITERATIONS: usize = 1000;

fn erreur(cible: [u8; N_SORTIES], resultat: [u8; N_SORTIES]) -> usize {
    cible.iter().zip(resultat.iter()).filter(|(c, r)| c != r).count()
}

fn erreur_motif(reseau: &Perceptron, x: &Motif, cible: [u8; N_SORTIES]) -> usize {
    erreur(cible, reseau.propager(x))
}

pub fn est_correct(reseau: &Perceptron) -> bool {
    erreur_motif(reseau, &A, CIBLE_A) == 0 && erreur_motif(reseau, &C, CIBLE_C) == 0
}

fn mesurer(reseau: &Perceptron) -> (usize, usize) {
    (
        erreur_motif(reseau, &A, CIBLE_A),
        erreur_motif(reseau, &C, CIBLE_C),
    )
}

pub fn methode_sequentielle (mu: f32) -> (Perceptron, Vec<(usize, usize)>) {
    let mut reseau = Perceptron::new();
    let mut historique = vec! [mesurer(&reseau)];

    for (motif, cible) in [(&A, CIBLE_A), (&C, CIBLE_C)] {
        for _ in 0..MAX_ITERATIONS {
            if erreur_motif(&reseau, motif, cible) == 0 {
                break;
            }
            reseau.apprendre(motif, cible, mu);
            historique.push(mesurer(&reseau));
        }
    }
    (reseau, historique)
}

pub fn methode_alterne(mu: f32) -> (Perceptron, Vec<(usize, usize)>) {
    let mut reseau = Perceptron::new();
    let mut historique = vec![mesurer(&reseau)];

    for _ in 0..MAX_ITERATIONS {
        if est_correct(&reseau) {
            break;
        }
        reseau.apprendre(&A, CIBLE_A, mu);
        historique.push(mesurer(&reseau));
        reseau.apprendre(&C, CIBLE_C, mu);
        historique.push(mesurer(&reseau));
    }
    (reseau, historique)
}