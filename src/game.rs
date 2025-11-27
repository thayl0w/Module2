use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

pub struct Game {
    pub category: String,
    pub word: Vec<char>,
    pub guessed: Vec<char>,
    pub lives: i32,
    pub points: u32,
}

impl Game {
    pub fn new(word_lists: &HashMap<String, Vec<String>>) -> Self {
        let mut rng = thread_rng();

        // Pick random category
        let categories: Vec<_> = word_lists.keys().cloned().collect();
        let category = categories.choose(&mut rng).unwrap().clone();

        // Pick random word
        let words = word_lists.get(&category).unwrap();
        let word_string = words.choose(&mut rng).unwrap().clone();

        Game {
            category,
            word: word_string.chars().collect(),
            guessed: vec![],
            lives: 6,
            points: 0,
        }
    }

    pub fn guess_letter(&mut self, ch: char) {
        if self.guessed.contains(&ch) {
            return;
        }

        self.guessed.push(ch);

        if self.word.contains(&ch) {
            self.points += 10;
        } else {
            self.lives -= 1;
        }
    }

    pub fn reveal_random_letter(&mut self) {
        let remaining: Vec<char> = self
            .word
            .iter()
            .cloned()
            .filter(|c| !self.guessed.contains(c))
            .collect();

        if remaining.is_empty() {
            return;
        }

        let mut rng = thread_rng();
        let letter = remaining.choose(&mut rng).unwrap();

        self.guessed.push(*letter);
        self.points = self.points.saturating_sub(5);
    }

    pub fn get_masked_word(&self) -> String {
        self.word
            .iter()
            .map(|c| if self.guessed.contains(c) { *c } else { '_' })
            .collect()
    }

    pub fn is_won(&self) -> bool {
        self.word.iter().all(|c| self.guessed.contains(c))
    }

    pub fn is_lost(&self) -> bool {
        self.lives <= 0
    }
}
