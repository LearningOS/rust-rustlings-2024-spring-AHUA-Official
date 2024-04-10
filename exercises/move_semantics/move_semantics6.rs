// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM 

fn main() {
    let data = "Rust is great!".to_string();

    // Clone the data to avoid moving it
    get_char(data.clone());

    // Pass a reference to data to string_uppercase
    string_uppercase(&data);
}

fn get_char(data: String) -> char {
    // Process the string and return the first character
    data.chars().next().unwrap()
}

fn string_uppercase(data: &String) {
    // Create a new variable to hold the uppercase string
    let uppercase_data = data.to_uppercase();

    // Use the new variable to avoid borrowing while modifying
    println!("{}", uppercase_data);
}
