use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

/// Loads word lists by category from a file (format: "Category: word1 word2 ...").
pub fn load_word_lists(filename: &str) -> Result<HashMap<String, Vec<String>>> {
    // Open the file, handling errors with a match on the Result.
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let reader = BufReader::new(file);

    let mut categories = HashMap::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
        if let Some((cat, words)) = line.split_once(':') {
            let category = cat.trim().to_string();
            let list: Vec<String> = words
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            categories.insert(category, list);
        }
    }
    Ok(categories)
}
