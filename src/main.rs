use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use std::time::Instant;
use rayon::prelude::*;
use structopt::StructOpt;
use std::io::{self, Write};

#[derive(StructOpt)]
struct Options {
    start_exponent: u64,

    end_exponent: u64,

    #[structopt(short, long)]
    verbose: bool,
}

fn mod_mersenne(n: &BigUint, p: u64) -> BigUint {
    let modulus = (&BigUint::one() << p) - 1u32;
    let mut n = n.clone();

    while n.bits() > p {
        let high = &n >> p;
        let low = &n & &modulus;
        n = high + low;
    }

    if n == modulus {
        BigUint::zero()
    } else {
        n
    }
}

fn is_mersenne_prime(p: u64, verbose: bool) -> bool {
    if p < 2 {
        return false;
    }
    if p == 2 {
        return true;
    }

    let total_iterations = p - 2;
    let progress_interval = (total_iterations / 100).max(1); // Ensure progress_interval is at least 1

    let mut s = 4u32.to_biguint().unwrap();

    for i in 1..=total_iterations {
        s = &s * &s - 2u32;
        s = mod_mersenne(&s, p);

        if verbose {
            if i % progress_interval == 0 || i == total_iterations {
                let percent = (i * 100) / total_iterations;
                print!("\rTesting p = {}: Progress: {}%", p, percent);
                io::stdout().flush().unwrap();
            }
        }
    }

    if verbose {
        println!();
    }
    s.is_zero()
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let options = Options::from_args();

    let start_p = options.start_exponent;
    let end_p = options.end_exponent;
    let verbose = options.verbose;

    if start_p > end_p {
        println!("Error: start_exponent should be less than or equal to end_exponent.");
        return;
    }

    let exponents: Vec<u64> = (start_p..=end_p).filter(|&p| is_prime(p)).collect();

    println!(
        "Searching for Mersenne primes in the range p = {} to p = {}...",
        start_p, end_p
    );

    let start_time = Instant::now();

    let results: Vec<u64> = exponents
        .par_iter()
        .filter_map(|&p| {
            if verbose {
                println!("Testing M({}) = 2^{} - 1", p, p);
            }
            let exponent_start_time = Instant::now();
            let is_prime_result = is_mersenne_prime(p, verbose);
            let duration = exponent_start_time.elapsed();

            if is_prime_result {
                println!(
                    "Found Mersenne prime: M({}), tested in {:.2} seconds.",
                    p,
                    duration.as_secs_f64()
                );
                Some(p)
            } else if verbose {
                println!(
                    "M({}) is composite, tested in {:.2} seconds.",
                    p,
                    duration.as_secs_f64()
                );
                None
            } else {
                None
            }
        })
        .collect();

    let total_duration = start_time.elapsed();

    println!("\nMersenne primes found:");
    for p in &results {
        println!("M({}) is a Mersenne prime.", p);
    }

    println!(
        "\nTotal time taken: {:.2} seconds",
        total_duration.as_secs_f64()
    );
}
