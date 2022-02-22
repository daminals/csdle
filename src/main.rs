// Rdle
// Daniel Kogan
// 02.11.2022

use std::io;
use std::io::Write;
use std::fs;
use rand::seq::SliceRandom; 
use std::process;
use std::collections::HashMap;

#[derive(Debug)]
struct LetterPosition {
    letter: char,
    position: usize,
    color: String
}
// helper function, returns the letter position to sort by
fn letter_by_position(letter_pos: &LetterPosition) -> usize {
    letter_pos.position
}
fn sort_letter_position(letter_pos: &mut Vec<LetterPosition>) {
    letter_pos.sort_by_key(letter_by_position);
}

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
    let solution = pick_solution();
    println!("{}", &solution); //don't tell people the answer lol
    for round in 1..7 {
        let guess = request_answer(round);
        let verify_guess = verify_answer(&solution, &guess, &round);
        let check_for_wrong_input = wrong_input_loop(&verify_guess, &round, &solution, &guess);
        println!("{}", check_for_wrong_input);
    }
    println!("The correct word was '{}'", solution);
}
// look through the list 
fn pick_solution() -> String {
    let filename = "solution.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let all_guesses = contents.lines().collect::<Vec<&str>>();
    return pick_solution_loop(all_guesses);
}
fn pick_solution_loop(all_possible_solutions: Vec<&str>) -> String{
    loop {
        let solution = all_possible_solutions.choose(&mut rand::thread_rng()).unwrap().to_string();
        if solution.len() == 5 {
            return String::from(solution);
        }
    }
}
fn request_answer(round: u8) -> String {
    print!("Guess {}. ", round);
    io::stdout().flush().unwrap();
    let guess = return_user_input();
    return guess;
}
fn verify_answer(solution: &String, guess: &String, round: &u8) -> String {
    if guess.len() != 5 {
        if guess.len() > 5 {
            println!("Too many letters");
        } else {
            println!("Not enough letters");
        }
        return "EXIT_REDO_ROUND".to_string();
    }
    if !is_real_word(&guess) {
        println!("Not a word");
        return "EXIT_REDO_ROUND".to_string();
    }
    let mut computed_answer = String::from("");
    // colors
    let black_text = "\u{001b}[30m";
    let clear_format = "\u{001b}[0m";
    // check guess loop
    
    let color_output = return_output_vector(&guess, &solution);

    for item in color_output {
        // print out every element of the output, with their colors
        let col: &str = &String::from(item.color).to_owned()[..];
        let letter: &str = &String::from(item.letter).to_owned()[..];
        computed_answer += &format!("{}{} {} {}", col, &black_text, letter, &clear_format).to_owned()[..];
    }

    if guess == solution {
        println!("{}", computed_answer);
        println!("Yay! You have won in {} guesses!!", round);
        process::exit(0x0100);
    }
    return computed_answer;
}
fn return_user_input() -> String {
    let mut input_output = String::new();
    io::stdin()
    .read_line(&mut input_output)
    .expect("Failed to read line");
    
    return input_output.trim().to_string()
}
fn wrong_input_loop (verify_guess: &String, round: &u8, solution: &String, guess: &String) -> String {
    //println!("{}", verify_guess);
    if verify_guess == &"EXIT_REDO_ROUND".to_string() {
        let guess = request_answer(*round);
        let verify_guess = verify_answer(solution, &guess.to_owned(), round);
        let recursive_check = wrong_input_loop(&verify_guess, round, solution, &guess);
        //println!("\u{001b}[31m {} \u{001b}[0m", recursive_check);
        //println!("{}", verify_answer(solution, &recursive_check));
        return recursive_check;
    }
    return verify_answer(solution, guess, round);
}
fn is_real_word(guess: &String) -> bool {
    return is_in_file(&guess, "solution.txt") || is_in_file(guess, "all.txt")
}
fn is_in_file(guess: &String, filename: &str) -> bool {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let all_words = contents.lines().collect::<Vec<_>>();
    return all_words.contains(&&guess.to_owned()[..]);
}
fn return_green_letters(guess: &str, solution: &str) -> Vec<LetterPosition> {
    let sol_chars = solution.chars().collect::<Vec<_>>();
    let mut correct_letters = vec![];
    for (index, letter) in guess.chars().enumerate() {
        if sol_chars[index] == letter {
            correct_letters.push(LetterPosition{letter: letter, position: index, color: "\u{001b}[42m".to_string()});
        }
    }
    return correct_letters;
}
fn return_gray_letters(guess: &str, solution: &str, mut gray_letters: Vec<LetterPosition>)  -> Vec<LetterPosition> {
    let sol_chars = solution.chars().collect::<Vec<_>>();
    for (index, letter) in guess.chars().enumerate() {
        if !sol_chars.contains(&letter) {
            gray_letters.push(LetterPosition{letter: letter, position: index, color: "\u{001b}[107m".to_string()});
        }
    }
    return gray_letters;
}
fn create_frequency_hashmap(text: &str) -> HashMap<char, i32>{
    let mut char_frequency = HashMap::new();
    for character in text.chars().collect::<Vec<char>>() {
        if !char_frequency.contains_key(&character) {
            char_frequency.insert(
                character,
                1
            );
        } else {
            *char_frequency.get_mut(&character).unwrap() += 1;
            //char_frequency[&character] += 1;
        }
    }
    return char_frequency;
}
fn subtract_hashmap(mut current_hm: HashMap<char, i32>, character: char) -> HashMap<char, i32> {
    if current_hm.contains_key(&character) {
        if current_hm[&character] > 0 {
            *current_hm.get_mut(&character).unwrap() -= 1
        }
    }
    return current_hm;
}
fn return_yellow_letters(guess: &str, solution: &str, mut yellow_letters: Vec<LetterPosition>) -> Vec<LetterPosition> {
    let sol_chars = solution.chars().collect::<Vec<char>>(); // collect solution too
    let mut solution_hashmap = create_frequency_hashmap(solution);
    for used_letter in &yellow_letters { // for all letters already used, subtract them out
        solution_hashmap = subtract_hashmap(solution_hashmap, used_letter.letter)
    }
    for (index, letter) in guess.chars().enumerate() {
        let letter_frequency_in_solution = unwrap_hashmap_value(&solution_hashmap, &letter);
        if (letter_frequency_in_solution > 0) && solution.contains(letter) && (sol_chars[index] != letter) {
            yellow_letters.push(LetterPosition { letter: letter, position: index, color: "\u{001b}[43m".to_string() } );
            solution_hashmap = subtract_hashmap(solution_hashmap, letter); // subtract letters just tracked
        } else if solution.contains(letter) && (sol_chars[index] != letter) {
            yellow_letters.push(LetterPosition { letter: letter, position: index, color: "\u{001b}[107m".to_string() } );
            // return gray letters for letters that would be yellow if they were more frequent
        }
    }
    return yellow_letters;
}
// create the output
fn return_output_vector(guess: &str, solution: &str) -> Vec<LetterPosition>{
    let color_guess = return_green_letters(&guess, &solution);
    let color_guess = return_gray_letters(&guess, &solution, color_guess);
    let mut color_guess = return_yellow_letters(&guess, &solution, color_guess);
    sort_letter_position(&mut color_guess);
    return color_guess;

}
// if a key exists, return its value
fn unwrap_hashmap_value(hm: &HashMap<char, i32>, key: &char) -> i32 {
    if hm.contains_key(&key) {
        return hm[&key]
    } else {
        return 0
    }
}