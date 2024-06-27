// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

/*
    In Rust, there are two ways to define a Vector.
    1. One way is to use the `Vec::new()` function to create a new vector
    and fill it with the `push()` method.
    2. The second way, which is simpler is to use the `vec![]` macro and
    define your elements inside the square brackets.

    Check this chapter: https://doc.rust-lang.org/stable/book/ch08-01-vectors.html
    of the Rust book to learn more.
*/

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v: Vec<i32> = vec![10, 20, 30, 40]; // TODO: declare your vector here with the macro for vectors
    // OR
    // let mut v = Vec::new();
    // v.push(10); 
    // v.push(20); 
    // v.push(30); 
    // v.push(40); 

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
