// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    let mut guess_time = NUM_INCORRECT_GUESSES;
    let mut guess_word: Vec<char> = vec!['-'; secret_word_chars.len()];
    let mut guess_letters: Vec<char> = vec![];
    let mut letter_map: HashMap<char, usize> = HashMap::new();
    println!("Welcome to CS110L Hangman!");

    loop {
        if guess_time == 0 {
            println!("Sorry, you ran out of guesses!");
            break;
        }
        println!("The word so far is {}", guess_word.iter().collect::<String>());
        println!("You have guessed the following letters: {}", guess_letters.iter().collect::<String>());
        println!("You have {} guesses left", guess_time);
        print!("Please guess a letter: ");

        // Get user input
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        println!();

        let char_guess = guess.chars().nth(0).unwrap();
        if !char_guess.is_alphabetic() {
            panic!("Guess is not an alphabetic symbol")
        }

        let mut start_idx = 0;
        let index = if letter_map.contains_key(&char_guess) {
            start_idx = letter_map[&char_guess] + 1;
            secret_word_chars[start_idx..].iter().position(|ch| *ch == char_guess)
        } else {
            secret_word_chars.iter().position(|ch| *ch == char_guess)
        };

        if let Some(idx) = index {
            let modified_idx = idx + start_idx;
            guess_word[modified_idx] = char_guess;
            *letter_map.entry(char_guess).or_insert(modified_idx) = modified_idx;
        } else {
            guess_time -= 1;
        }
        guess_letters.push(char_guess);

        if guess_word.iter().collect::<String>() == secret_word_chars.iter().collect::<String>() {
            println!("Congratulations you guessed the secret word: {}!", guess_word.iter().collect::<String>());
            break;
        }
    }
}
