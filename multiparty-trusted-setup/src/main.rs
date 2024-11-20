use ark_bn254::{ Fr as ScalarField, G1Projective as G1, G2Projective as G2 };
use ark_ec::{ scalar_mul::ScalarMul, AffineRepr, PrimeGroup };
use ark_ff::{ Field, UniformRand };
use ark_std::test_rng;

fn main() {
    let mut rng = test_rng();
    let mut s = ScalarField::rand(&mut rng);

    let mut powers_of_s_1 = vec![ScalarField::ONE];
    let mut powers_of_s_2 = vec![ScalarField::ONE];

    let mut curr = s;  
    for _ in 1..7 {
        powers_of_s_1.push(curr);
        curr *= &s;
    }
    curr = s;
    for _ in 1..9 {
        powers_of_s_2.push(curr);
        curr *= &s;
    }

    for _ in 1..10 {
        rng = test_rng();
        s = ScalarField::rand(&mut rng);
        curr = s;
        for i in 0..7 {
            powers_of_s_1[i] = powers_of_s_1[i] * curr;
            curr *= s;
        }
        curr = s;
        for i in 0..9 {
            powers_of_s_2[i] = powers_of_s_2[i] * curr;
            curr *= s;
        }
    }

    let g1 = G1::generator();
    let g2 = G2::generator();

    let powers_of_g1 = g1.batch_mul(&powers_of_s_1);
    let powers_of_g2 = g2.batch_mul(&powers_of_s_2);

    println!("ptau G1:");
    for (i, point) in powers_of_g1.iter().enumerate() {
        println!("  Point {}:", i + 1);
        println!("    x: {:?}", point.x().unwrap());
        println!("    y: {:?}", point.y().unwrap());
    }

    println!("\nptau G2:");
    for (i, point) in powers_of_g2.iter().enumerate() {
        println!("  Point {}:", i + 1);
        println!("    x: ({:?} + {:?})", point.x.c0, point.x.c1);
        println!("    y: ({:?} + {:?})", point.y.c0, point.y.c1);
    }
}