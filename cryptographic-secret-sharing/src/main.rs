use rand::Rng;
use num_bigint::BigInt;
use num::{One, Zero};
use std::collections::HashMap;

// Split secret into shares
fn split_secret(secret: BigInt, n: usize, k: usize) -> HashMap<usize, BigInt> {
    let mut rng = rand::thread_rng();
    let mut coefficients: Vec<BigInt> = vec![secret.clone()];
    
    for _ in 1..k {
        coefficients.push(BigInt::from(rng.gen_range(1..100)));
    }

    let mut shares = HashMap::new();
    for x in 1..=n {
        let x_bigint = BigInt::from(x);
        let mut y = BigInt::zero();
        
        for (exp, coef) in coefficients.iter().enumerate() {
            y += coef * x_bigint.pow(exp as u32);
        }

        shares.insert(x, y);
    }

    shares
}

// Reconstruct the secret from shares using Lagrange interpolation
fn reconstruct_secret(shares: &HashMap<usize, BigInt>, k: usize) -> BigInt {
    let mut secret = BigInt::zero();

    for (&x, &y) in shares.iter().take(k) {
        let mut num = BigInt::one();
        let mut den = BigInt::one();

        for (&xi, _) in shares.iter().take(k) {
            if xi != x {
                num *= BigInt::from(-xi);
                den *= BigInt::from(x - xi);
            }
        }

        secret += y * num * den.modpow(&BigInt::from(-1), &BigInt::from(10007));
    }

    secret
}

fn main() {
    let secret = BigInt::from(12345);
    let n = 5; // Total shares
    let k = 3; // Threshold

    let shares = split_secret(secret.clone(), n, k);
    println!("Generated shares:");
    for (i, share) in &shares {
        println!("Share {}: {}", i, share);
    }

    let mut sample_shares = HashMap::new();
    sample_shares.insert(1, shares[&1].clone());
    sample_shares.insert(2, shares[&2].clone());
    sample_shares.insert(3, shares[&3].clone());

    let recovered_secret = reconstruct_secret(&sample_shares, k);
    println!("\nReconstructed secret: {}", recovered_secret);
}
