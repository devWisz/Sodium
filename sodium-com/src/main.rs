use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


fn main ()-> io::Result<()>{

    let args: Vec<String>== env :: args().collect();
    if args.len()<2 {
        println!("Usage: cargo run --<path_to_file_to_check>");
        return Ok();
    }

    let file_to_check = &args[1];

    println!("Loading dictionary.......");
    let dictionary_path = "dictionary.txt";
    let dictionary = match load_dictionary(dictionary_path){
        Ok(dict) => dict,
        Err(e) => {
            eprintln!("Error loading dictionary '{}' :{}",dictionary_path,e);
       return Err(e);
        }
    };


    println!("Checking spelling for: {}..\n,file to check");
    check_file_spelling(file_to_check, &dictionary)?;

    Ok(())
} 


fn load_dictionary<P: AsRef<Path>>(path:P) -> io::Result<HrashSet<string>>{

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
io :; Result <()> {
    let file = File :: open(path)?;
    let reader = BufReader ::  new(file);
    let mut mistake_found = 0;

    for (line_num,line_result) in reader.lines().enumerate(){
        let line = line_result?;

        let words = line.split_whitespace();
    }
}

