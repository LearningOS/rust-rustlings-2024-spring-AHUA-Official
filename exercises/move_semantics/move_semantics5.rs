// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// I AM 

fn main() {
    let mut x = 10;
    let y = &mut x;
    *y += 100;
    println!("y: {}", y);

    // Remove the following line to fix the issue
    // let z = &mut x;
}
