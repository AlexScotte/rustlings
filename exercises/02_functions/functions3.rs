// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

/*
This time, the function *declaration* is okay, but there's something wrong
with the place where we're calling the function.

As a reminder, you can freely play around with different solutions in Rustlings!
Watch mode will only jump to the next exercise if you remove the `I AM NOT
DONE` comment.
*/

fn main() {
    call_me(1);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
