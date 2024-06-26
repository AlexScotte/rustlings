// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

/*
Rust requires that all parts of a function's signature have type annotations,
but `call_me` is missing the type annotation of `num`.
*/

fn main() {
    call_me(3);
}

fn call_me(num:i8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
