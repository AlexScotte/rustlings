// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

/*
    Stop reading whenever you feel like you have enough direction :) Or try
    doing one step and then fixing the compiler errors that result!
    So the end goal is to:
    - get rid of the first line in main that creates the new vector
    - so then `vec0` doesn't exist, so we can't pass it to `fill_vec`
    - `fill_vec` has had its signature changed, which our call should reflect
    - since we're not creating a new vec in `main` anymore, we need to create
        a new vec in `fill_vec`, and fill it with the expected values
*/

#[test]
fn main() {
    let mut vec1 = fill_vec();

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument - don't change this!
fn fill_vec() -> Vec<i32> {
    // Instead, let's create and fill the Vec in here - how do you do that?
    let mut vec = vec![22, 44, 66];

    vec.push(88);

    vec
}
