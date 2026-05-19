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
}