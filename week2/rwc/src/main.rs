// use std::collections::btree_map::Iter;
use std::env;
use std::process;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut str_vec: Vec<String> = vec![];

    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        str_vec.push(line_str);
    }
    Ok(str_vec)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let lines = read_file_lines(filename).unwrap();
    let mut words = lines.iter().map(|pred| pred.split_whitespace());

    let line_num: usize = lines.len();
    let mut word_num: usize = 0;
    let mut char_num: usize = 0;

    while let Some(mut w) = words.next() {
        while let Some(sstr) = w.next() {
            let chars: Vec<char> = sstr.chars().collect();
            char_num += chars.len();
            word_num += 1;
        }
    }

    println!("{} {} {} {}", word_num, line_num, char_num, filename);
}
