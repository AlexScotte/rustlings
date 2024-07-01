// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


/*
    Carefully reason about the range in which each mutable reference is in
    scope. Does it help to update the value of `x` immediately after
    the mutable reference is taken? Read more about 'Mutable References'
    in the book's section 'References and Borrowing':
    https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references.
*/

#[test]
fn main() {
    let mut x = 100;
    let mut y = &mut x;
    *y += 100;
    let mut z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
