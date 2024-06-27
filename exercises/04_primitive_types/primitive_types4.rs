// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

/*
    Take a look at the 'Understanding Ownership -> Slices -> Other Slices' section
    of the book: https://doc.rust-lang.org/book/ch04-03-slices.html and use the
    starting and ending (plus one) indices of the items in the `Array` that you
    want to end up in the slice.

    If you're curious why the first argument of `assert_eq!` does not have an
    ampersand for a reference since the second argument is a reference, take a look
    at the coercion chapter of the nomicon:
    https://doc.rust-lang.org/nomicon/coercions.html
*/

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];
    
    // OR
    // let beforeLast = a.len() - 1;
    // let nice_slice = &a[1..beforeLast];

    assert_eq!([2, 3, 4], nice_slice)
}
