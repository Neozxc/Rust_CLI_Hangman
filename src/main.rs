mod game;
mod words;

fn main() {
    let words: [&str; 5] = [
        "rustisgod",
        "programming",
        "language",
        "compiler",
        "ownership",
    ];

    let secret_word = words::choose_secret_word();
    let mut guesses = 6;
    let mut guessed_letters = vec!['_'; secret_word.len()];
    let mut incorrect_guesses = Vec::new();

    println!("Welcome to the Hangman Game! 🔥");

    while guesses > 0 && guessed_letters.contains(&'_') {
        game::display_game_state(&guessed_letters, &incorrect_guesses, guesses);
        let guess = game::get_guess();

        if secret_word.contains(guess) {
            game::update_guessed_letters(&secret_word, guess, &mut guessed_letters);
        } else {
            guesses -= 1;
            incorrect_guesses.push(guess);
        }
    }

    if guessed_letters.contains(&'_') {
        println!("Game over! The words was: {}", secret_word);
    } else {
        println!("You have won! The word you guessed was: {}", secret_word);
    }
}
