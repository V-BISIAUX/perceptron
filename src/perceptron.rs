use crate::motif::{Motif, N_ENTREES};

pub const N_SORTIES: usize = 2;

pub struct Perceptron {
    pub poids: [[f32; N_ENTREES]; N_SORTIES],
    pub theta: f32,
}

impl Perceptron {
    pub fn new() -> Self {
        Perceptron {
            poids: [[0.0; N_ENTREES]; N_SORTIES],
            theta: 1.0,
        }
    }

    pub fn propager(&self, x: &Motif) -> [u8; N_SORTIES] {
        let mut y = [0u8; N_SORTIES];

        for i in 0..N_SORTIES {
            let somme: f32 = x.iter().zip(self.poids[0]).map(|(pixel, poids)| pixel * poids).sum();
            y[i] = if (somme - self.theta) >= 0.0 {1} else {0};
        }
        y
    }
}