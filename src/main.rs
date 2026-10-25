use std::env;
use std::fs;
use std::process;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = match get_text(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("No file {}, {}", file_path, e);
            process::exit(1);
        }
    };

    let word_maps = text_to_map(&contents);

    let mut sorted: Vec<(&String, &usize)> = word_maps.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    for w in sorted {
        println!("word:{}, count:{}", w.0, w.1);
    }
}

fn get_text(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn text_to_map(contents: &String) -> HashMap<String, usize> {
    contents.clone().retain(|c| c != ',' && c != '.' && c != '!' && c != '?');

    let words: Vec<String> = contents
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();

    let mut word_maps: HashMap<String, usize> = HashMap::new();
    for word in words {
        *word_maps.entry(word).or_insert(0) += 1;
        // let score = word_maps.get(&word).copied().unwrap_or(0);
        // word_maps.insert(word, score + 1);
        // println!("{}", word);
    }

    word_maps
}
