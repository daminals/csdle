// Rdle
// Daniel Kogan
// 02.11.2022

use std::env;
use std::io;
use std::io::Write;
use std::fs;
use rand::seq::SliceRandom; 

fn main() {
    let solution = pick_solution();

    println!("{}", &solution);
    for round in 1..7 {
        let guess = request_answer(round);
        let verify_guess = verify_answer(&solution, &guess);
        let check_for_wrong_input = wrong_input_loop(&verify_guess, &round, &solution, &guess);
        println!("{}", check_for_wrong_input);
    }
}
// look through the list 
fn pick_solution() -> String {
    let filename = "solution.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let all_guesses = contents.lines().collect::<Vec<_>>();
    return all_guesses.choose(&mut rand::thread_rng()).unwrap().to_string();
}
fn request_answer(round: u8) -> String {
    print!("Guess {}. ", round);
    io::stdout().flush().unwrap();
    let guess = return_user_input();
    return guess;

}
fn verify_answer(solution: &String, guess: &String) -> String {
    if guess.len() != 5 {
        if guess.len() > 5 {
            println!("Too many letters")
        } else {
            println!("Not enough letters")
        }
        return "EXIT_REDO_ROUND".to_string()
    }

    let sol_chars = solution.chars().collect::<Vec<_>>();
    let mut sol_remaining_letters = solution.chars().collect::<Vec<_>>();
    let mut computed_answer = String::from("");
    for (index, letter) in guess.chars().enumerate() {
        if solution.contains(letter) && sol_remaining_letters.contains(&letter) {
            //println!("index: {} letter: {}", index, letter);
            //println!("{}", sol_chars[index]);
            if sol_chars[index] == letter {
                // return letter as green
                computed_answer += &format!("\u{001b}[32m {} \u{001b}[0m", letter).to_owned();
            } else {
                // return letter as yellow
                computed_answer +=  &format!("\u{001b}[33m {} \u{001b}[0m", letter).to_owned();
            }
            let remove_letter = sol_remaining_letters.iter().position(|x| *x == letter).unwrap();
            sol_remaining_letters.remove(remove_letter);    
        } else {
            computed_answer += &format!(" {} ", letter).to_owned();
        }

    }
    return computed_answer;
}
fn display_info() {

}
fn return_user_input() -> String {
    let mut input_output = String::new();
    io::stdin()
    .read_line(&mut input_output)
    .expect("Failed to read line");
    
    input_output.trim().to_string()
}
fn wrong_input_loop (verify_guess: &String, round: &u8, solution: &String, guess: &String) -> String {
    //println!("{}", verify_guess);
    if verify_guess == &"EXIT_REDO_ROUND".to_string() {
        let guess = request_answer(*round);
        let verify_guess = verify_answer(solution, &guess.to_owned());
        let recursive_check = wrong_input_loop(&verify_guess, round, solution, &guess);
        //println!("\u{001b}[31m {} \u{001b}[0m", recursive_check);
        //println!("{}", verify_answer(solution, &recursive_check));
        return recursive_check;
    }
    return verify_answer(solution, guess);
}
