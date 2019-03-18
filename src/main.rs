extern crate rand;

use std::env;
use std::ffi::OsString;
use rand::prelude::*;

struct Config {
    password_length: usize,
    use_uppercase_letters: bool,
    use_numbers: bool,
    use_symbols: bool,
}

struct Pool<'a> {
    lowercase_letters: [&'a str; 26],
    uppercase_letters: [&'a str; 26],
    numbers: [&'a str; 10],
    symbols: [&'a str; 8],
}

fn create_password(pool: Pool, config: Config) -> String {
    let mut rng = thread_rng();

    let mut password = String::new();

    while password.len() < config.password_length {
        let mut random_strings = ["", "", "", ""];

        {
            let random_number = rng.gen_range(0, pool.lowercase_letters.len());
            random_strings[0] = pool.lowercase_letters[random_number];
        }

        {
            if config.use_uppercase_letters {
                let random_number = rng.gen_range(0, pool.uppercase_letters.len());
                random_strings[1] = pool.uppercase_letters[random_number];
            }
        }

        {
            if config.use_numbers {
                let random_number = rng.gen_range(0, pool.numbers.len());
                random_strings[2] = pool.numbers[random_number];
            }
        }

        {
            if config.use_symbols {
                let random_number = rng.gen_range(0, pool.symbols.len());
                random_strings[3] = pool.symbols[random_number];
            }
        }

        let random_number = rng.gen_range(0, random_strings.len());
        let random_character = random_strings[random_number];

        password.push_str(random_character);
    }

    password
}

fn main() {

    let args: Vec<OsString> = env::args_os().collect();

    let pool = Pool {
        lowercase_letters: ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"],
        uppercase_letters: ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"],
        numbers: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
        symbols: ["!", "§", "$", "%", "&", "?", "@", "#",],
    };

    let mut config = Config {
        password_length: 10,
        use_uppercase_letters: args.contains(&OsString::from("mixedcase")),
        use_numbers: args.contains(&OsString::from("numbers")),
        use_symbols: args.contains(&OsString::from("symbols")),
    };

    for a in &args {
        let a = a.to_str().unwrap_or("");

        if a.contains(&String::from("length=")) {
            let length: Vec<&str> = a.split('=').collect();

            match length[1].parse::<usize>() {
                Ok(n) => {
                    config.password_length = n
                },
                Err(error) => {
                    // kind is not public, so we've to use the description
                    match error.to_string().as_ref() {
                        "cannot parse integer from empty string" => eprintln!("Pleace enter a password length."),
                        "invalid digit found in string" => eprintln!("Pleace enter a valid password length."),
                        _ => eprintln!("There is something wrong with your password length."),
                    };

                    eprintln!("Password length is set to default value ({}).", config.password_length);
                },
            };
        }
    }

    println!("{}", create_password(pool, config));

}