extern crate rand;

use std::env;
use std::ffi::OsString;
use rand::prelude::*;

fn main() {

    let args: Vec<OsString> = env::args_os().collect();

    let mut password_length: usize = 10;

    for a in &args {
        let a = a.to_str().unwrap_or("");

        if a.contains(&String::from("length=")) {
            let length: Vec<&str> = a.split('=').collect();

            match length[1].parse::<usize>() {
                Ok(n) => {
                    password_length = n
                },
                Err(error) => {
                    // kind is not public, so we've to use the description
                    match error.to_string().as_ref() {
                        "cannot parse integer from empty string" => println!("Pleace enter a password length."),
                        "invalid digit found in string" => println!("Pleace enter a valid password length."),
                        _ => println!("There is something wrong with your password length."),
                    };

                    println!("Password length is set to default value ({}).", password_length);
                },
            };
        }
    }

    let use_uppercase_letters = if args.contains(&OsString::from("mixedcase")) {
        true
    } else {
        false
    };

    let use_numbers = if args.contains(&OsString::from("numbers")) {
        true
    } else {
        false
    };

    let use_symbols = if args.contains(&OsString::from("symbols")) {
        true
    } else {
        false
    };

    let lowercase_letters: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let uppercase_letters: [&str; 26] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
    let numbers: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let symbols: [&str; 8] = ["!", "§", "$", "%", "&", "?", "@", "#",];

    let mut rng = thread_rng();

    let mut password = String::new();

    while password.len() < password_length {
        let mut random_strings = ["", "", "", ""];

        {
            let random_number = rng.gen_range(0, lowercase_letters.len());
            random_strings[0] = lowercase_letters[random_number];
        }

        {
            if use_uppercase_letters {
                let random_number = rng.gen_range(0, uppercase_letters.len());
                random_strings[1] = uppercase_letters[random_number];
            }
        }

        {
            if use_numbers {
                let random_number = rng.gen_range(0, numbers.len());
                random_strings[2] = numbers[random_number];
            }
        }

        {
            if use_symbols {
                let random_number = rng.gen_range(0, symbols.len());
                random_strings[3] = symbols[random_number];
            }
        }

        let random_number = rng.gen_range(0, random_strings.len());
        let random_character = random_strings[random_number];

        password.push_str(random_character);
    }

    println!("{}", password);

}
