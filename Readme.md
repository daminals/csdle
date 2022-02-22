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

While running my wordle clone for this variant, I discovered a bug within it. It would output a yellow letter and a green when a user inputted a word with a duplicate a letter and one was at the correct index and the solution only had one of these letters. 

Put simply, it reached an edge case where it returned a yellow letter where a gray one should have been.

I mulled over how to fix this issue and tried implementing various different data structures to no avail. I used linked lists, vectors, structs, arrays and all sorts of other methods unsuccessfully. I also switched from handling all letters together to seperate functions for each implementation case (green, gray, yellow).

Eventually, I was able to use HashMaps to track the frequency of a letter in the solution, and compare it to the used letters.

I also learned how to use sorts on a struct, as when I switched to a custom struct vector rather than a one-by-one approach, my output was out of order. I added an index element to the struct and learned how to implement my own 'sort by key' Rust method

I look forward to using this knowledge of data structures, implementations, and more in my future projects!