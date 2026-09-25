//! Shared logic used by `hello-cli` and, via WASM, the Zola site.

mod primes;

pub use primes::random_consecutive_primes;

/// WASM bindings, compiled only when targeting the browser.
#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    /// Browser-callable wrapper around [`crate::random_consecutive_primes`].
    ///
    /// Returns bare numbers (`[first, second]`), not a display-ready sentence — the
    /// page's own JS builds whatever text it wants around them.
    #[wasm_bindgen(js_name = randomConsecutivePrimes)]
    pub fn random_consecutive_primes() -> Vec<u64> {
        let (first, second) = crate::random_consecutive_primes();
        vec![first, second]
    }
}

/// Build a greeting for `name`, falling back to "World" when blank.
///
/// ```
/// assert_eq!(hello_core::greet("Ada"), "Hello, Ada!");
/// assert_eq!(hello_core::greet("  "), "Hello, World!");
/// ```
pub fn greet(name: &str) -> String {
    let name = name.trim();
    let name = if name.is_empty() { "World" } else { name };
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn greets_by_name() {
        assert_eq!(greet("Ferris"), "Hello, Ferris!");
    }

    #[test]
    fn blank_defaults_to_world() {
        assert_eq!(greet(""), "Hello, World!");
    }
}
