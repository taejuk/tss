use ark_ff::{Field, PrimeField, UniformRand, Zero}; 
use ark_ec::{CurveGroup, PrimeGroup};
use ark_std::rand::Rng;
use ark_bls12_381::{Fr, G1Projective as G1};
use vss::{generate::polynomials::Polynomial, recovery::verify::{verify_eval, verify_poly}, setup::PublicKey};


fn main() {
    let mut rng = ark_std::test_rng();
    let pk = PublicKey::generate(10);
    let secret = Fr::rand(&mut rng);
    let threshold:usize = 4;
    println!("secret: {}", secret);
    let poly = Polynomial::new(secret, threshold);
    let c = poly.commit(&pk);
    println!("commit verify is {}", verify_poly(&pk, &c, &poly));
    let x = Fr::rand(&mut rng);
    let commit = poly.create_commit(&x, &pk);
    println!("witness verify is {}", verify_eval(&pk, &c, &commit));
}