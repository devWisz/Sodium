use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, IsTerminal};
use std::path::Path;


fn pause_and_exit(code: i32) -> ! {
    if std::io::stdout().is_terminal() {
        println!("\nPress Enter to exit...");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
    }
    std::process::exit(code);
}

fn resolve_file_path(path: &str) -> std::path::PathBuf {
    let p = Path::new(path);
    if p.exists() {
        return p.to_path_buf();
    }
    if let Ok(exe_path) = env::current_exe() {
        let mut current_dir = exe_path.parent();
        while let Some(dir) = current_dir {
            let candidate = dir.join(path);
            if candidate.exists() {
                return candidate;
            }
            current_dir = dir.parent();
        }
    }
    p.to_path_buf()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_to_check = if args.len() > 1 { &args[1] } else { "input.txt" };
    let dictionary_path = if args.len() > 2 { &args[2] } else { "dictionary.txt" };

    let resolved_dict_path = resolve_file_path(dictionary_path);
    println!("Loading dictionary.......");
    let dictionary = match load_dictionary(&resolved_dict_path) {
        Ok(dict) => dict,
        Err(e) => {
            eprintln!("Error: Could not load dictionary file '{}': {}", dictionary_path, e);
            eprintln!("Please make sure '{}' exists in the current working directory or near the executable.", dictionary_path);
            pause_and_exit(1);
        }
    };

    let resolved_input_path = resolve_file_path(file_to_check);
    println!("Checking spelling for: {}..\n", resolved_input_path.display());
    if let Err(e) = check_file_spelling(&resolved_input_path, &dictionary) {
        eprintln!("Error: Could not read input file '{}': {}", file_to_check, e);
        pause_and_exit(1);
    }

    if std::io::stdout().is_terminal() {
        println!("\nPress Enter to exit...");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
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