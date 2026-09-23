// script logic that will be used in hello-core library
// fn random_consecutive_primes() -> (u64, u64)
//   will be called by both hello-cli and by zola code
//   which will display something like "today's consecutive primes are x and y"

use rand::Rng;

/// Returns true when `number` is prime.
fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }

    if number == 2 {
        return true;
    }

    if number % 2 == 0 {
        return false;
    }

    let limit = (number as f64).sqrt() as u64;

    for divisor in (3..=limit).step_by(2) {
        if number % divisor == 0 {
            return false;
        }
    }

    true
}

/// Returns the prime at a one-based position.
///
/// For example:
/// - nth_prime(1) returns 2
/// - nth_prime(2) returns 3
/// - nth_prime(3) returns 5
fn nth_prime(n: u32) -> u64 {
    assert!(n >= 1, "n must be at least 1");

    let mut primes_found = 0;
    let mut candidate = 2;

    loop {
        if is_prime(candidate) {
            primes_found += 1;

            if primes_found == n {
                return candidate;
            }
        }

        candidate += 1;
    }
}

/// Generates a random n in the range 1..=1000, then returns:
/// `(the nth prime, the (n + 1)th prime)`.
fn random_consecutive_primes() -> (u64, u64) {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..=1000);

    let nth = nth_prime(n);
    let next = nth_prime(n + 1);

    (nth, next)
}

fn main() {
    let (nth_prime_value, next_prime_value) = random_consecutive_primes();

    println!("The randomly selected n produced these consecutive indexed primes:");
    println!("nth prime: {}", nth_prime_value);
    println!("(n + 1)th prime: {}", next_prime_value);
}