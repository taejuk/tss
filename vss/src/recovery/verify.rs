use crate::{generate::polynomials::Polynomial, generate::polynomials::Commit, setup::PublicKey};
use ark_ff::{Field, PrimeField, UniformRand, Zero}; 
use ark_ec::{CurveGroup, PrimeGroup,pairing::Pairing};
use ark_std::rand::Rng;
use ark_bls12_381::{Fr, G1Projective as G1,Bls12_381, G2Projective as G2};


pub fn verify_poly(pk: &PublicKey,C: &G1, poly: &Polynomial) -> bool {
    let c_verify = poly.coeffs.iter()
    .zip(pk.pks.iter())
    .map(|(coeff, base)| {
        *base * *coeff
    })
    .sum::<G1>();
    *C == c_verify
}


pub fn verify_eval(pk: &PublicKey, C: &G1, commit: &Commit) -> bool {
    let x = commit.x;
    let y = commit.y;
    let witness = commit.witness;

    let g1 = pk.pks[0];
    let g2 = pk.pks2[0];

    let c_minus_y = C - (g1 * y);
    
    let lhs = Bls12_381::pairing(c_minus_y, g2);

    let g2_alpha = pk.pks2[1];
    let g2_x = g2 * x;
    
    let rhs_element = g2_alpha - g2_x;

    let rhs = Bls12_381::pairing(witness, rhs_element);

    lhs == rhs
}