use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng(); // Random number generator
    let mut dice: [i32; 5] = [0; 5]; // Array to hold 5 dice values
    let labels = ['A', 'B', 'C', 'D', 'E']; // Labels for each die
    let mut input = String::new();
    let mut reroll_count = 0; // Counter for rerolls or no rolls

    println!("Rolling five dice to start the game...");
    roll_all_dice(&mut dice, &mut rng);
    print_dice(&dice, &labels);

    loop {
        if reroll_count >= 2 {
            println!("\n2 rerolls or actions reached. Starting a new turn...");
            println!("Press Enter to roll all dice for the new turn.");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to wait for Enter.");
            roll_all_dice(&mut dice, &mut rng);
            print_dice(&dice, &labels);
            reroll_count = 0; // Reset reroll count
            continue;
        }

        println!("\nEnter the letters (A-E) of the dice you want to reroll, or press Enter for no reroll, or type 'exit' to quit:");
        input.clear(); // Clear the input buffer
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        // Check if the input is "exit"
        if input.trim().eq_ignore_ascii_case("exit") {
            println!("Exiting the program.");
            break;
        }

        // If the user presses Enter without entering any letters, count it as an action
        if input.trim().is_empty() {
            println!("No dice rerolled. Counting this as one action.");
            reroll_count += 1; // Increment the reroll count even if no dice are rerolled
            continue;
        }

        // Parse input and reroll selected dice
        let choices: Vec<char> = input
            .trim()
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect();

        if !choices.is_empty() {
            reroll_dice(&mut dice, &choices, &mut rng, &labels);
            print_dice(&dice, &labels);
            reroll_count += 1; // Increment the reroll count
        } else {
            println!("No valid dice selected for reroll. Try again.");
        }
    }
}

// Rolls all dice at once for a new turn
fn roll_all_dice(dice: &mut [i32; 5], rng: &mut rand::rngs::ThreadRng) {
    for die in dice.iter_mut() {
        *die = rng.random_range(1..=6);
    }
}

// Prints the current state of the dice
fn print_dice(dice: &[i32; 5], labels: &[char; 5]) {
    println!("\nCurrent dice:");
    for (i, &die) in dice.iter().enumerate() {
        println!("{}: {}", labels[i], die);
    }
}

// Rerolls the selected dice specified by the user
fn reroll_dice(
    dice: &mut [i32; 5],
    choices: &[char],
    rng: &mut rand::rngs::ThreadRng,
    labels: &[char; 5],
) {
    for &choice in choices {
        if let Some(index) = labels.iter().position(|&label| label.eq_ignore_ascii_case(&choice)) {
            dice[index] = rng.random_range(1..=6);
            println!("Rerolled {}...", labels[index]);
        } else {
            println!("Invalid selection: {}", choice);
        }
    }
}
