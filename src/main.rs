
use std::usize;
use clap::Parser;
use rand::Rng;
use terminal_clipboard;
use rand::prelude::ThreadRng;

mod constants;

use constants::*;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of the password
    #[arg(short = 'L', long, default_value_t = 8)]
    length: u128,

    /// Password will include CAPITAL CASE Letters
    #[arg(short, long, action)]
    upper: bool,

    /// Password will include lowercase Letters
    #[arg(short, long, action)]
    lower: bool,

    /// Password will include numbers
    #[arg(short, long, action)]
    number: bool,

    /// Password will include special characters
    #[arg(short, long, action)]
    special: bool,
}


fn get_random_letter(sets: &Vec<&str>) -> char {
    let mut rng: ThreadRng = rand::thread_rng();

    let set_len: u8 = sets.len().try_into().unwrap();
    let set_idx: usize = rng.gen_range(0..set_len).try_into().unwrap();

    let set_name = sets[set_idx];

    if set_name == "UPPER" {
        let set_len: u8 = UPPERCASE_ALPHABETS_SET.len().try_into().unwrap();
        let set_idx: usize = rng.gen_range(0..set_len).try_into().unwrap();

        let letter = UPPERCASE_ALPHABETS_SET[set_idx];
        let chars: Vec<char> = letter.chars().collect();
        return chars[0]
    }

    if set_name == "LOWER" {
        let set_len: u8 = LOWERCASE_ALPHABETS_SET.len().try_into().unwrap();
        let set_idx: usize = rng.gen_range(0..set_len).try_into().unwrap();

        let letter = LOWERCASE_ALPHABETS_SET[set_idx];
        let chars: Vec<char> = letter.chars().collect();
        return chars[0]
    }

    if set_name == "SPECIAL" {
        let set_len: u8 = SPECIAL_CHARACTERS_SET.len().try_into().unwrap();
        let set_idx: usize = rng.gen_range(0..set_len).try_into().unwrap();

        let letter = SPECIAL_CHARACTERS_SET[set_idx];
        let chars: Vec<char> = letter.chars().collect();
        return chars[0]
    }


    if set_name == "NUMBER" {
        let set_len: u8 = NUMBER_SET.len().try_into().unwrap();
        let set_idx: usize = rng.gen_range(0..set_len).try_into().unwrap();

        let letter = NUMBER_SET[set_idx].to_string();
        let chars: Vec<char> = letter.chars().collect();
        return chars[0]
    }

    return 'X';
}


fn main() {
    let args = Args::parse();

    let mut sets: Vec<&str> = vec![];


    if args.lower {
        sets.push("LOWER");
    }

    if args.upper {
        sets.push("UPPER");
    }

    if args.special {
        sets.push("SPECIAL");
    }

    if args.number {
        sets.push("NUMBER")
    }

    let mut password = String::new();

    for _ in 0..args.length {
        let letter = get_random_letter(&sets);
        password.push(letter)
    }

    println!("Password");
    println!("--------");
    println!("{}", password);
    terminal_clipboard::set_string(password.trim()).unwrap();
}
