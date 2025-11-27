mod words;
mod game;
mod ui;

use std::fs;
use game::Game;

fn main() {
    // Load words from file (words.txt with categories)
    let word_lists = match words::load_word_lists("words.txt") {
        Ok(w) => w,
        Err(e) => { eprintln!("Error loading words: {}", e); return; }
    };
    // Load high score from file
    let highscore: u32 = match fs::read_to_string("highscore.txt") {
        Ok(contents) => match contents.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => 0,
        },
        Err(_) => 0,
    };

    println!("Welcome to Haunted Mansion Hangman!");
    ui::display_title();

    loop {
        let mut game = Game::new(&word_lists);
        // Main game loop
        while !game.is_won() && !game.is_lost() {
            ui::display_status(&game);
            let input = ui::get_input("Enter a letter or '!' for a hint: ");
            if input == "!" {
                if game.points >= 5 {
                    game.reveal_random_letter();
                } else {
                    println!("Not enough points for a hint! (5 points needed)");
                }
                continue;
            }
            if let Some(ch) = input.chars().next() {
                game.guess_letter(ch);
            }
        }
        // End of round
        if game.is_won() {
            println!("\nYou won! The word was \"{}\".", game.word.iter().collect::<String>());
        } else {
            println!("\nGame over! The word was \"{}\".", game.word.iter().collect::<String>());
        }
        println!("Your score: {}", game.points);
        // Check high score
        if game.points > highscore {
            println!("New high score!");
            match fs::write("highscore.txt", game.points.to_string()) {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to save highscore: {}", e),
            }
        }
        let again = ui::get_input("Play again? (y/n): ");
        if again.to_lowercase() != "y" {
            println!("Thanks for playing!");
            break;
        }
    }
}
