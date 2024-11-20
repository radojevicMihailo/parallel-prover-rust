use ark_bn254::{ Fr as ScalarField, G1Projective as G1, G2Projective as G2 };
use ark_ec::{ scalar_mul::ScalarMul, AffineRepr, PrimeGroup };
use ark_ff::{ Field, PrimeField };

fn powers_of_g(n_1: u32, n_2: u32, s_bytes: &[u8]) -> 
    (Vec<ark_ec::short_weierstrass::Affine<ark_bn254::g1::Config>>, 
    Vec<ark_ec::short_weierstrass::Affine<ark_bn254::g2::Config>>) {
    let s = ScalarField::from_be_bytes_mod_order(s_bytes);
    let mut powers_of_s_1 = vec![ScalarField::ONE];
    let mut powers_of_s_2 = vec![ScalarField::ONE];
    let mut curr = s;
    println!("{:?}", powers_of_s_1);
    for _ in 0..n_1 {
        powers_of_s_1.push(curr);
        curr *= &s;
    }
    curr = s;
    for _ in 0..n_2 {
        powers_of_s_2.push(curr);
        curr *= &s;
    }

    let g1 = G1::generator();
    let g2 = G2::generator();

    (g1.batch_mul(&powers_of_s_1), g2.batch_mul(&powers_of_s_2))
}

fn main() {
    let (powers_of_g1, powers_of_g2) = powers_of_g(7, 9, b"1237865491825639871263987126");


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