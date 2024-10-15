use rand::seq::SliceRandom;
use std::io::{self, Write};

fn main() {
    let words: [&str; 5] = ["rustisgod", "programming", "language", "compiler", "ownership"];
    let secret_word = words.choose(&mut rand::thread_rng()).unwrap();
    let mut guesses = 6;
    let mut guessed_letters = vec!["_", secret_word.len()];
    let mut incorrect_guesses = Vec::new();
}
