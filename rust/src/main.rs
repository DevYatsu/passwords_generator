use getrandom::getrandom;

use rand::thread_rng;
use rand::seq::SliceRandom;

use std::fs::File;
use std::io::Write;
use std::time::Instant;

const PASSWORDS_LENGTH: usize = 10;
const NUMBER_TO_GENERATE: usize = 100000;
const FILE_NAME: &str = "passwords.txt";

fn main() {
    timer(|| {
        generate_x_passwords(FILE_NAME, NUMBER_TO_GENERATE);
    });
}


fn timer<F: Fn()>(f: F) {
    let start = Instant::now();
    f();
    let end = Instant::now();
    println!("function execution: {} seconds", (end - start).as_secs_f64());
}

fn generate_password() -> String {
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = "0123456789";
    let special_characters = "!@#$%^&*()";

    let mut password = String::with_capacity(PASSWORDS_LENGTH);

    password.push(get_random_char(lowercase_letters));
    password.push(get_random_char(uppercase_letters));
    password.push(get_random_char(digits));
    password.push(get_random_char(special_characters));


    for _ in 4..PASSWORDS_LENGTH {
        password.push(get_random_char(
            &(lowercase_letters.to_owned()
                + uppercase_letters
                + digits
                + special_characters),
        ));
    }

    let mut password_vec: Vec<char> = password.chars().collect();
    password_vec.shuffle(&mut thread_rng());

    password_vec.into_iter().collect()
}

fn get_random_char(characters: &str) -> char {
    let random_index: usize = getrandom_index(characters.len());
    characters.chars().nth(random_index).unwrap()
}

fn getrandom_index(upper_bound: usize) -> usize {
    let mut buffer = [0u8; 1];
    getrandom(&mut buffer).expect("Failed to generate random number");
    buffer[0] as usize % upper_bound
}

fn write_passwords(filename: &str, passwords: &[String]) {
    let mut file = File::create(filename).expect("Failed to create file");

    for password in passwords {
        writeln!(file, "{}", password).expect("Failed to write password");
    }

    println!("Passwords successfully written in {}", filename);
}

fn generate_x_passwords(filename: &str, num: usize) {
    let mut passwords: Vec<String> = Vec::with_capacity(num);

    for _ in 0..num {
        passwords.push(generate_password());
    }

    write_passwords(filename, &passwords);
}
