// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::{DateTime, Local};

fn main() {
    let local: DateTime<Local> = Local::now();
    println!("{}", local.format("%d/%m-%Y kl. %H:%M").to_string());
}
