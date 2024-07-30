use std::io;
use std::cmp::Ordering;
use rand::Rng;
use regex::Regex;

fn main() {
    // Step 1 - Generate a Random Number between 1 and 100

    let secret_number: u32 = set_up();
    run_game(secret_number);
    play_again();

    // SCOPE TEST

}



fn set_up() -> u32 {
    println!("I'm thinking of a numer...");
    return rand::thread_rng().gen_range(1..=100)
}

fn run_game(secret_number: u32) {
    // establish escape pattern once outside of loop for the love of resources
    let escape_pattern = Regex::new(r"exit").unwrap();
    println!("Type \"Exit\" at any time to quit (case insensitive).\nEnter your guess:");

    loop {

        let mut guess = String::new();

        // Collect guess from terminal and update reference with
        // a mutable borrow
        io::stdin()
            .read_line(&mut guess) // 
            .expect("Failed to read line");

        // Check that guess does not match an exit pattern
        if escape_pattern.is_match(&guess.to_lowercase()) {
            break;
        }
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("ERROR: {e}\nPlease only guess numbers, letter and symbols are not allowed.");
                continue; // The `continue` keyword impacts loop flow, proceeding to the next iteration
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low, guess bigger."),
            Ordering::Greater => println!("Too high, guess smaller."),
            Ordering::Equal => {
                println!("🎉Correct!🎉");
                break; // The `break` keyword is also loop flow mgmt, ending the loop
            },
        }
        
    }
}

fn play_again() {
    println!("Play again? yes/no:");
    // Get input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Goodbye!");
    // Define replay and quit patterns
    let matches_replay_ptn: bool = Regex::new(r"yes").unwrap().is_match(&input.to_lowercase()) || &input.to_lowercase(). == "y";
    let matches_quit_ptn: bool = Regex::new(r"no").unwrap().is_match(&input.to_lowercase());

    if (matches_replay_ptn && matches_quit_ptn) || (!matches_quit_ptn && !matches_replay_ptn) {
        println!("ERROR - unable to parse input: {}", input);
        play_again();
    } else if matches_replay_ptn {
        main();
    } else if matches_quit_ptn {
        println!("Goodbye!");
    } else {
        println!("You've unlocked the secret win with \"{}\"", input);
        println!("You've also crashed the game. Goodbye!");
    }
}