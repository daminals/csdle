![github repo badge: Language](https://img.shields.io/badge/Language-Rust-181717?color=red)    ![github repo badge: Based On](https://img.shields.io/badge/Based%20On-Wordle-181717?color=green)

# Csdle

I decided to expand a previous project I made, [Rdle](https://github.com/daminals/Rdle), which was a plain wordle clone. 

Recently however, I have been getting very into Wordle varients, such as [Quordle](https://www.quordle.com/#/), [Octordle](https://octordle.com), [Jewdle](https://www.jewdle.app), and others. I was inspired to create my own variant, where all the solutions would be Computer Science words.

Like the real game, there is a seperate list of acceptable words and solution words, where you can guess from any acceptable but all the solution words will be related to CS in some way

## How to play Csdle

The rules for Csdle are the same as Wordle, except all the solutions will be related to computer science

The rules for Wordle are as follows:
- You must guess a 5 letter word the program has chosen
- You have 6 guesses.
- If a letter is in a solution and the guess you made, the letter will be highlighted yellow
- If a letter is in a solution and the guess you made, in the order you guessed it, the letter will be highlighted green

## What I learned

I learned about reading through text files, parsing and unwrapping, error handling, and while loops (although I replaced them with recursion) in Rust.
