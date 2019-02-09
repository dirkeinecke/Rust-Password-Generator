extern crate rand;

use rand::prelude::*;

fn main() {

    let lowercase_letters = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let uppercase_letters = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
    let numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let symbols = ["!", "§", "$", "%", "&", "?", "@", "#",];

    let mut rng = thread_rng();

    let mut password = String::new();

    let password_length = 10;

    let use_uppercase_letters = true;
    let use_numbers = true;
    let use_symbols = true;

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
