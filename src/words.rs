use rand::seq::SliceRandom;
use rand::thread_rng;

/**
 * Since the word list is fixed size and won't change, we use a static array for efficiency
 * @returns performance benefit allocated on the stack and requires less overhead than a Vec.
 */

const WORDS: [&str; 5] = [
    "rustisgod",
    "programming",
    "language",
    "compiler",
    "ownership",
];

/**
 * Method to choose a secret word randomly from the list
 */

pub fn choose_secret_word() -> &'static str {
    WORDS
        .choose(&mut thread_rng())
        .expect("Word list is empty.")
}
