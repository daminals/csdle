// Rdle
// Daniel Kogan
// 02.11.2022

use std::env;
use std::fs;
use rand::seq::SliceRandom; 

fn main() {
    let filename = "all.txt";

    //println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let all_guesses = contents.lines().collect::<Vec<_>>();


    println!("{:?}", all_guesses.choose(&mut rand::thread_rng()).unwrap());

}

