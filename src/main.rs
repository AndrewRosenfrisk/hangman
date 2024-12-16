use std::io::stdin;

use rand::{thread_rng, Rng};

const HEAD: &str = "O";
const BODY: &str = "|";
const LEFT_ARM: &str = "/";
const LEFT_LEG: &str = "/";
const RIGHT_ARM: &str = "\\";
const RIGHT_LEG: &str = "\\";
const CATEGORY: &str = "ANIMALS";
fn main() {
    let mut tries = 6;
    let mut missed_letters = String::new();
    let mut correct_letters = String::new();
    let mut rng = thread_rng();

    let words = "ANT BABOON BADGER BAT BEAR BEAVER CAMEL CAT CLAM COBRA COUGAR COYOTE CROW DEER DOG DONKEY DUCK EAGLE FERRET FOX FROG GOAT GOOSE HAWK LION LIZARD LLAMA MOLE MONKEY MOOSE MOUSE MULE NEWT OTTER OWL PANDA PARROT PIGEON PYTHON RABBIT RAM RAT RAVEN RHINO SALMON SEAL SHARK SHEEP SKUNK SLOTH SNAKE SPIDER STORK SWAN TIGER TOAD TROUT TURKEY TURTLE WEASEL WHALE WOLF WOMBAT ZEBRA".split(" ").collect::<Vec<&str>>();

    let secret_word = words[rng.gen_range(0..words.len())].to_string();

    let mut remaining_chars = secret_word.clone();

    loop {
        draw_hangman(&missed_letters, &correct_letters, secret_word.clone());

        let guess = get_input(&(missed_letters.clone() + &correct_letters));

        if remaining_chars.contains(&guess) {
            correct_letters.push_str(&guess);
            remaining_chars = remaining_chars.replace(&guess, "");
            if remaining_chars.is_empty() {
                println!("Yes! The secret word is {}", secret_word);
                println!("You win!!!");
                break;
            }
        } else {
            missed_letters.push_str(&guess);
            tries -= 1;
        }
        if tries == 0 {
            draw_hangman(&missed_letters, &correct_letters, secret_word.clone());
            println!("You've run out of guesses!");
            println!("The word was {}", secret_word);
            break;
        }
    }
}

fn draw_hangman(missed_letters: &String, correct_letters: &String, secret_word: String) {
    let mut display = vec![" ", " ", " ", " ", " ", " "];
    let parts = vec![HEAD, BODY, LEFT_ARM, RIGHT_ARM, LEFT_LEG, RIGHT_LEG];

    if missed_letters.len() > 0 {
        for i in 1..=missed_letters.len() {
            display[i - 1] = parts[i - 1];
        }
    }

    println!(
        "
 +--+
 |  |
 {h}  |
{la}{b}{ra} |
{ll} {rl} |
    |
=====",
        h = display[0],
        b = display[1],
        la = display[2],
        ra = display[3],
        ll = display[4],
        rl = display[5]
    );

    println!("The category is: {}", CATEGORY);

    if missed_letters.len() > 0 {
        println!(
            "Missed letters: {}",
            missed_letters
                .split("")
                .collect::<Vec<&str>>()
                .join(", ")
                .trim_start_matches(", ")
        );
    } else {
        println!("No missed letters yet.");
    }

    for letter in secret_word.chars() {
        if correct_letters.contains(&letter.to_string()) {
            print!("{}", letter);
        } else {
            print!(" _ ");
        }
    }
    println!();
}

fn get_input(already_guessed: &String) -> String {
    let alphabet = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    println!("Guess a letter: ");
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_uppercase();

        if input.len() != 1 || !alphabet.contains(&input) {
            println!("Please enter a single letter");
            continue;
        } else if already_guessed.contains(&input) {
            println!("You've already guessed that letter. Choose again.");
            continue;
        } else {
            return input;
        }
    }
}
