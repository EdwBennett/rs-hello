//! Random consecutive-prime lookup, shared by `hello-cli` and the Zola site (via WASM).

use rand::RngExt;

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
    let mut rng = rand::rng();
    let n: u32 = rng.random_range(1..=1000);

    let nth = nth_prime(n);
    let next = nth_prime(n + 1);

    (nth, next)
}

/// Builds a display-ready sentence naming a random pair of consecutive primes.
///
/// A fresh random pair is chosen on every call.
pub fn todays_primes_message() -> String {
    let (nth, next) = random_consecutive_primes();
    format!("Today's prime numbers are {nth} and {next}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_prime_matches_known_values() {
        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(2), 3);
        assert_eq!(nth_prime(3), 5);
    }

    #[test]
    fn message_names_two_consecutive_primes() {
        let message = todays_primes_message();
        assert!(message.starts_with("Today's prime numbers are "));
    }
}
