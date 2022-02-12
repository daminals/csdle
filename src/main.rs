// Rdle
// Daniel Kogan
// 02.11.2022

use std::env;
use std::fs;
use rand::seq::SliceRandom; 

fn pick_solution() -> String {
    let filename = "solution.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let all_guesses = contents.lines().collect::<Vec<_>>();
    //println!("{:?}", all_guesses.choose(&mut rand::thread_rng()).unwrap());
    return all_guesses.choose(&mut rand::thread_rng()).unwrap().to_string();
}

fn main() {
    let solution = pick_solution();

    println!("{}", &solution);
}

