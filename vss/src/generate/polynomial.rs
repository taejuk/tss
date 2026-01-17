use ark_ff::{Field, PrimeField, UniformRand, Zero}; 
use ark_ec::{CurveGroup, PrimeGroup};
use ark_std::rand::Rng;
use ark_bls12_381::{Fr, G1Projective as G1};


use crate::setup::PublicKey;
#[derive(Clone)]
pub struct Polynomial {
    coeffs: Vec<Fr>
}

pub struct Commit {
    pub x: Fr,
    pub y: Fr,
    pub witness: G1
}

impl Polynomial {
    
    pub fn new(secret: Fr, degree: usize) -> Self {
        assert!(degree > 0, "degree should be bigger than 0.");
        let mut coeffs = Vec::with_capacity(degree + 1);
        let mut rng = ark_std::test_rng();
        coeffs.push(secret);

        for _ in 0..degree {
            coeffs.push(Fr::rand(&mut rng));
        }
        Polynomial { coeffs: coeffs}
    }

    pub fn zero() -> Self {
        Polynomial { coeffs: vec![Fr::zero()] }
    }

    pub fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }

    pub fn evaluate(&self, x: &Fr) -> Fr {
        self.coeffs.iter().rev().fold(Fr::zero(), |acc, coeff| {
            acc * *x + *coeff 
        })
    }
    // polynomial을 x-i로 나눈다. 이 때는 조립제법을 이용하면 된다.
    pub fn div_by_one_degree(&self, i: &Fr) -> Self {
        let mut quotient = Vec::with_capacity(self.coeffs.len()-1);
        let mut carry = Fr::zero();
        for coeff in self.coeffs.iter().rev() {
            let value = *coeff  + carry;
            quotient.push(value);
            carry = value * i;
        }
        
        quotient.pop();
        quotient.reverse();

        Polynomial { coeffs:quotient }
    }

    pub fn commit(&self, pk: &PublicKey) -> G1 {
        assert!(self.coeffs.len() <= pk.pks.len(), "Polynomial degree is too large for the Public Key");
        
        self.coeffs.iter()
            .zip(pk.pks.iter())
            .map(|(coeff, base)| {
                *base * *coeff
            })
            .sum::<G1>()
    }

    pub fn open(&self) -> &Vec<Fr> {
        &self.coeffs
    }

    pub fn minus_constant(&self, y: &Fr) -> Self {
        let mut coeffs = self.coeffs.clone();
        coeffs[0] = coeffs[0] - y;
        Polynomial { coeffs }
    }
    pub fn create_commit(&self, x: &Fr, pk: &PublicKey) -> Commit {
        let mut psi = self.clone();
        let y = self.evaluate(x);
        // 상수항을 뺀다.
        
        psi = psi.minus_constant(&y);
        psi = psi.div_by_one_degree(x);
        let value = psi.commit(pk);
        
        Commit { x: x.clone(), y, witness: value }
        
    }
}