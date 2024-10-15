use std::io::{self, Write};

pub fn display_game_state(guessed_letters: &[char], incorrect_guesses: &[char], guesses: usize) {
    println!("\nWord: {} ", guessed_letters.iter().collect::<String>());
    println!("Incorrect guesses: {:?}", incorrect_guesses);
    println!("Guesses left: {} 😬", guesses);
}

pub fn get_guess() -> char {
    let mut guess = String::new();
    print!("Enter your guess: ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to red input");

    let trimmed_guess = guess.trim();

    if trimmed_guess.len() != 1 {
        println!("Please enter exactly one character.");
        return get_guess();
    }

    trimmed_guess.chars().next().unwrap()
}

pub fn update_guessed_letters(secret_word: &str, guess: char, guessed_letters: &mut Vec<char>) {
    for (i, letter) in secret_word.chars().enumerate() {
        if letter == guess {
            guessed_letters[i] = guess;
        }
    }
}
