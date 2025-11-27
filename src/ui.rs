use colored::*;
use std::io::{stdin, stdout, Write};
use crate::game::Game;

pub fn display_title() {
    println!("{}", "=== Haunted Mansion Hangman ===".magenta().bold());
}

pub fn display_status(game: &Game) {
    println!("\nCategory: {}", game.category.yellow());
    println!("Word:     {}", game.get_masked_word().cyan());
    println!("Lives:    {}    Points: {}", game.lives, game.points);
    println!("Guessed:  {:?}", game.guessed);

    // Draw hangman
    println!("{}", get_hangman_art(game.lives));
}

pub fn get_input(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed == "!" {
            return "!".to_string();
        }

        if trimmed.len() == 1 {
            return trimmed.to_string();
        }

        println!("{}", "Enter only ONE letter or '!'".red());
    }
}

// =====================
// HANGMAN ASCII STAGES
// =====================
pub fn get_hangman_art(lives: i32) -> &'static str {
    match lives {
        6 => r#"
  +---+
  |   |
      |
      |
      |
      |
========="#,

        5 => r#"
  +---+
  |   |
  O   |
      |
      |
      |
========="#,

        4 => r#"
  +---+
  |   |
  O   |
  |   |
      |
      |
========="#,

        3 => r#"
  +---+
  |   |
  O   |
 /|   |
      |
      |
========="#,

        2 => r#"
  +---+
  |   |
  O   |
 /|\  |
      |
      |
========="#,

        1 => r#"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
========="#,

        0 => r#"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
========="#,

        _ => "",
    }
}
