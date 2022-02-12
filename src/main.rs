// Rdle
// Daniel Kogan
// 02.11.2022

use std::env;
use std::fs;


fn main() {
    println!("Hello, world!");
    let filename = "all.txt";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

}

