//! Rust Helpers Library

use rand::prelude::*;

/// Sends greetings from the Rust Helpers library.
pub fn greet_rust_helpers(name: &str) -> String {
    format!("Hello, {}! Welcome to Rust Helpers.", name)
}
/// Splits a vector into smaller vectors of a specified size.
pub fn split_rand_vec<T: Clone>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    let mut rng = rand::rng();
    // ramdomize the order of the vector
    let mut vec: Vec<T> = vec;
    vec.shuffle(&mut rng);
    vec.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_rust_helpers() {
        let greeting = greet_rust_helpers("Alice");
        assert_eq!(greeting, "Hello, Alice! Welcome to Rust Helpers.");
    }

    #[test]
    fn test_split_rand_vec() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let chunked = split_rand_vec(vec, 3);
        assert_eq!(chunked.len(), 3);
        assert!(chunked.iter().all(|chunk| chunk.len() <= 3));
    }
}