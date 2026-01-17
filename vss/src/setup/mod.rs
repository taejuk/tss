use ark_ff::{Field, PrimeField, UniformRand};
use ark_ec::{PrimeGroup};
use ark_std::test_rng;
use ark_bls12_381::{Fr, G1Projective as G1, G2Projective as G2};
#[derive(Clone)]
pub struct PublicKey {
    pub pks: Vec<G1>,
    pub pks2: Vec<G2>
}

impl PublicKey {
    pub fn generate(degree: usize) -> Self {
        let g = G1::generator();
        let g2 = G2::generator();
        let mut rng = test_rng();

        let alpha = Fr::rand(&mut rng);
        let mut powers = Vec::with_capacity(degree+1);
        let mut powers2 = Vec::with_capacity(degree+1);

        let mut current = Fr::from(1u64);
        for _ in 0..=degree {
            let point = g * current;
            let point2 = g2 * current;
            powers.push(point);
            powers2.push(point2);
            current = current * alpha;
        }

        PublicKey { pks: powers, pks2: powers2 }
    }
}