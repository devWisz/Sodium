use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let exe_name = env::current_exe()
            .ok()
            .and_then(|p| p.file_name().map(|s| s.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "sodium".to_string());
        eprintln!("Usage: {} <path_to_file_to_check>", exe_name);
        std::process::exit(1);
    }

    let file_to_check = &args[1];

    println!("Loading dictionary.......");
    let dictionary_path = "dictionary.txt";
    let dictionary = match load_dictionary(dictionary_path) {
        Ok(dict) => dict,
        Err(e) => {
            eprintln!("Error: Could not load dictionary file '{}': {}", dictionary_path, e);
            eprintln!("Please make sure '{}' exists in the current working directory.", dictionary_path);
            std::process::exit(1);
        }
    };

    println!("Checking spelling for: {}..\n", file_to_check);
    if let Err(e) = check_file_spelling(file_to_check, &dictionary) {
        eprintln!("Error: Could not read input file '{}': {}", file_to_check, e);
        std::process::exit(1);
    }
} 


fn load_dictionary<P: AsRef<Path>>(path:P) -> io::Result<HashSet<String>>{

    let file = File:: open (path)?;
    let reader = BufReader ::new(file);
    let mut dictionary = HashSet:: new ();

    
    for line in reader.lines(){ 
let word = line?.trim().to_lowercase();
if !word.is_empty(){
    dictionary.insert(word);
}

    }
    Ok(dictionary)
} 

fn check_file_spelling<P: AsRef<Path>>(path:P, dictionary: &HashSet<String>)->
io :: Result <()> {
    let file = File :: open(path)?;
    let reader = BufReader ::  new(file);
    let mut mistake_found = 0;


   for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;

        let words = line.split_whitespace();

        for raw_word in words {

            let cleaned_word: String = raw_word 
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect ::<String>()
            .to_lowercase();

        if cleaned_word.is_empty (){
continue
        }


        if !dictionary.contains(&cleaned_word){
            mistake_found += 1;
            println!(
                "Line {}: Mistaken Word found -> \"{}\"",
                line_num + 1, cleaned_word
            );
        }
        }
 
  }

    if mistake_found == 0 {
        println!(" No spelling mistakes found! Great job.");
    } else {
        println!("\nTotal spelling errors found: {}", mistake_found);
    }

    Ok(())
} 