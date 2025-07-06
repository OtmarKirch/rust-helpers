//! Rust Helpers Library

use rand::prelude::*;
use std::collections::HashSet;

/// Sends greetings from the Rust Helpers library.
pub fn greet_rust_helpers(name: &str) -> String {
    format!("Hello, {}! Welcome to Rust Helpers.", name)
}
/// Randomizes the order of a vec and splits it into smaller vectors of a specified size.
///
/// # Arguments
/// * `vec` - The vector to be split and randomized. It must implement the `Clone` trait.
/// * `chunk_size` - The size of each chunk in the resulting vector. The last chunk may be smaller if the total number of elements is not divisible by `chunk_size`.
///
/// # Returns
/// A vector of vectors, where each inner vector is a chunk of the original vector, randomized in order.
///
/// # Example
/// ```
/// use rust_helpers::split_rand_vec;
/// let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
/// let chunked = split_rand_vec(vec, 3);
/// assert_eq!(chunked.len(), 3);
/// assert!(chunked.iter().all(|chunk| chunk.len() <= 3));
/// /// The elements in the chunks are randomized, so the order may vary.
/// ```
pub fn split_rand_vec<T: Clone>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    let mut rng = rand::rng();
    let mut vec: Vec<T> = vec;
    vec.shuffle(&mut rng);
    vec.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect()
}

/// Randomizes the order of a vec and splits it into smaller vectors of equal size.
/// # Arguments
/// * `vec` - The vector to be split and randomized. It must implement the `Clone` trait.
/// * `parts` - The number of parts to split the vector into. A number of parts may contain fewer elements than others if the total number of elements is not divisible by `parts`.
///
/// # Returns
/// A vector of vectors, where each inner vector is a part of the original vector, randomized in order.
/// # Example
/// ```
/// use rust_helpers::split_rand_vec_eq;
/// let vec = vec![1, 2, 3, 4, 5, 6, 7];
/// let chunked = split_rand_vec_eq(vec, 3);
/// assert_eq!(chunked.len(), 3);
/// assert_eq!(chunked[0].len(), 3);
/// assert_eq!(chunked[1].len(), 2);
/// assert_eq!(chunked[2].len(), 2);
/// /// The elements in the chunks are randomized, so the order may vary.
/// ```
pub fn split_rand_vec_eq<T: Clone>(vec: Vec<T>, parts: usize) -> Vec<Vec<T>> {
    let mut rng = rand::rng();
    let mut vec: Vec<T> = vec;
    vec.shuffle(&mut rng);
    let mut result = vec![Vec::new(); parts];
    for (i, e) in vec.iter().enumerate() {
        result[i % parts].push(e.clone());
    }
    return result;
}

/// Randomizes the order of a HashSet and splits it into smaller vectors of equal size.
/// # Arguments
/// * `vec` - The vector to be split and randomized. It must implement the `Clone` trait.
/// * `parts` - The number of parts to split the vector into. A number of parts may contain fewer elements than others if the total number of elements is not divisible by `parts`.
///
/// # Returns
/// A vector of vectors, where each inner vector is a part of the original vector, randomized in order.
/// # Example
/// ```
/// use rust_helpers::split_rand_vec_eq;
/// let vec = vec![1, 2, 3, 4, 5, 6, 7];
/// let chunked = split_rand_vec_eq(vec, 3);
/// assert_eq!(chunked.len(), 3);
/// assert_eq!(chunked[0].len(), 3);
/// assert_eq!(chunked[1].len(), 2);
/// assert_eq!(chunked[2].len(), 2);
/// /// The elements in the chunks are randomized, so the order may vary.
/// ```
pub fn split_rand_hashset_eq<T: Clone>(vec: HashSet<T>, parts: usize) -> Vec<Vec<T>> {
    let mut rng = rand::rng();
    let mut vec: Vec<T> = vec.into_iter().collect();
    vec.shuffle(&mut rng);
    let mut result = vec![Vec::new(); parts];
    for (i, e) in vec.iter().enumerate() {
        result[i % parts].push(e.clone());
    }
    return result;
}

pub fn check_sufficient_items<T: PartialEq>(req_items: &[(T, usize)], checked_vector: &[T]) -> bool {
    for (item, req_number_item) in req_items {
        let cnt_check = dbg!(checked_vector.iter().filter(|x| *x == item).count());
        if cnt_check < *req_number_item {
            return false;
        }
    }
    true
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

    #[test]
    fn test_split_rand_vec_eq() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let chunked = split_rand_vec_eq(vec, 3);
        assert_eq!(chunked.len(), 3);
        assert_eq!(chunked[0].len(), 3);
        assert_eq!(chunked[1].len(), 2);
        assert_eq!(chunked[2].len(), 2);
    }

    #[test]
    fn test_split_hashset_eq() {
        let vec: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7].iter().cloned().collect();
        let chunked = split_rand_hashset_eq(vec, 3);
        assert_eq!(chunked.len(), 3);
        assert_eq!(chunked[0].len(), 3);
        assert_eq!(chunked[1].len(), 2);
        assert_eq!(chunked[2].len(), 2);
    }

    #[test]
    fn test_check_sufficient_items() {
        let req_items = [("apple", 2), ("banana", 1), ("orange", 3)];
        let checked_vector = vec!["apple", "apple", "apple", "banana", "orange", "orange", "orange", "orange"];
        assert!(check_sufficient_items(&req_items, &checked_vector));

        let insufficient_items = vec![("apple", 3), ("banana", 1)];
        let insufficient_vector = vec!["apple", "banana", "orange"];
        assert!(!check_sufficient_items(&insufficient_items, &insufficient_vector));
    }    
}
