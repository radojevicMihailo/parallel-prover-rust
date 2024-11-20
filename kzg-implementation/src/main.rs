pub mod kzg;
pub mod utils;

use ark_bls12_381::{Bls12_381, Fr, G1Projective as G1, G2Projective as G2};
use ark_ff::UniformRand;
use kzg::KZG;
use utils::evaluate;

fn main() {
    let mut rng = ark_std::test_rng();
    let degree = 16;
    let mut kzg_instance = KZG::<Bls12_381>::new(G1::rand(&mut rng), G2::rand(&mut rng), degree);

    let secret = Fr::rand(&mut rng);
    kzg_instance.setup(secret);

    let poly = vec![Fr::rand(&mut rng); degree+1];
    let commitment = kzg_instance.commit(&poly);

    test_single_evaluation(&kzg_instance, &poly, commitment);
}

pub fn test_single_evaluation(kzg_instance: &KZG<Bls12_381>, poly: &[Fr], commitment: G1) {
    let mut rng = ark_std::test_rng();

    let point = Fr::rand(&mut rng);
    let pi = kzg_instance.open(&poly, point);

    let value = evaluate(&poly, point);
    assert!(kzg_instance.verify(point, value, commitment, pi));

    println!("Single point evaluation verified!");
}
