//! Shared logic used by `hello-cli`.

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
