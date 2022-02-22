// Rdle
// Daniel Kogan
// 02.11.2022

use crate::fs::File;
use std::io;
use std::io::Write;
use std::fs;
use rand::seq::SliceRandom; 
use std::process;
use array_tool::vec::Intersect;

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
fn sort_LetterPosition(letter_pos: &mut Vec<LetterPosition>) {
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
    let sol_chars = solution.chars().collect::<Vec<_>>();
    let mut sol_remaining_letters = solution.chars().collect::<Vec<_>>();
    let mut computed_answer = String::from("");
    // colors
    let black_text = "\u{001b}[30m";
    let green_background = "\u{001b}[42m";
    let yellow_background = "\u{001b}[43m";
    let white_background = "\u{001b}[107m";
    let clear_format = "\u{001b}[0m";
    // check guess loop
    
    let color_guess = return_green_letters(&guess, &solution);
    //println!("{:?}", color_guess);
    let color_guess = return_gray_letters(&guess, &solution, color_guess);
    let mut color_guess = return_yellow_letters(&guess, &solution, color_guess);
    sort_LetterPosition(&mut color_guess);

    let mut computed_answer = "".to_string();
    for item in color_guess {
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
fn return_yellow_letters(guess: &str, solution: &str, mut yellow_letters: Vec<LetterPosition>) -> Vec<LetterPosition> {
    let mut logs = File::create("logs.txt").unwrap();
    
    let mut remaining_letters = guess.chars().collect::<Vec<char>>();
    let sol_chars = solution.chars().collect::<Vec<char>>();
    writeln!(&mut logs, "green + gray array {:?}", yellow_letters);
    writeln!(&mut logs, "initial array {:?}", yellow_letters);
    let mut charr_array: Vec<char> = vec![];
    for letter in &yellow_letters {
        charr_array.push(letter.letter);
    }
    remaining_letters = remaining_letters.intersect(charr_array);
    let final_gray_letters = remaining_letters.clone();
    let mut remaining_letters = remaining_letters.intersect(sol_chars);
    let sol_chars = solution.chars().collect::<Vec<char>>();
    writeln!(&mut logs, "new array {:?}", remaining_letters);

    //println!("{:?}", guess.chars().enumerate());

    for (index, letter) in guess.chars().enumerate() {
        //println!("{}", letter);
        //println!("{:?}", remaining_letters);
        if remaining_letters.contains(&letter) && solution.contains(letter) && (sol_chars[index] != letter) {
            yellow_letters.push(LetterPosition { letter: letter, position: index, color: "\u{001b}[43m".to_string() } );
            remaining_letters = drop_letter(&mut remaining_letters, letter);
        } else if final_gray_letters.contains(&letter) && solution.contains(letter) && (sol_chars[index] != letter) {
            yellow_letters.push(LetterPosition { letter: letter, position: index, color: "\u{001b}[107m".to_string() } );
        }
    }
    println!("{:?}", yellow_letters);
    return yellow_letters;
}
fn drop_letter(char_array: &mut Vec<char>, letter: char) -> Vec<char> {
    let remove_letter = char_array.iter().position(|x| *x == letter).unwrap();
    char_array.remove(remove_letter);    
    return char_array.to_vec();
}