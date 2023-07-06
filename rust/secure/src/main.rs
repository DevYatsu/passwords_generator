use getrandom::getrandom;
use rand::thread_rng;
use rand::seq::SliceRandom;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use std::str;
use rayon::prelude::*;

const PASSWORDS_LENGTH: usize = 10;
const NUMBER_TO_GENERATE: usize = 500000;
const FILE_NAME: &str = "passwords.txt";

fn main() {
    if PASSWORDS_LENGTH < 4 {
        panic!("PASSWORDS_LENGTH must be at least 4")
    }

    timer(|| {
        generate_x_passwords(FILE_NAME, NUMBER_TO_GENERATE);
    });
}

fn timer<F: Fn()>(f: F) {
    let start = Instant::now();
    f();
    let end = Instant::now();
    println!(
        "function execution: {} seconds",
        (end - start).as_secs_f64()
    );
}

fn generate_password() -> Vec<u8> {
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = "0123456789";
    let special_characters = "!@#$%^&*()";
    let all_characters: String =
        lowercase_letters.to_owned() + uppercase_letters + digits + special_characters;

    let mut password: [u8; PASSWORDS_LENGTH] = [0; PASSWORDS_LENGTH];

    password[0] = get_random_char(lowercase_letters) as u8;
    password[1] = get_random_char(uppercase_letters) as u8;
    password[2] = get_random_char(digits) as u8;
    password[3] = get_random_char(special_characters) as u8;

    for i in 4..PASSWORDS_LENGTH {
        password[i] = get_random_char(&all_characters) as u8;
    }

    password.shuffle(&mut thread_rng());    
    password.into_iter().collect()
}

fn get_random_char(characters: &str) -> char {
    let random_index: usize = get_random_index(characters.len());
    characters.chars().nth(random_index).unwrap()
}

fn get_random_index(upper_bound: usize) -> usize {
    let mut buffer = [0u8; 4];
    getrandom(&mut buffer).expect("Failed to generate random bytes");
    let value = u32::from_ne_bytes(buffer);
    (value % upper_bound as u32) as usize
}

fn write_passwords(filename: &str, passwords: &[Vec<u8>]) {
    let file = File::create(filename).expect("Failed to create file");
    let mut writer = BufWriter::with_capacity(65536, file);

    let mut buffer: Vec<u8> = Vec::with_capacity(NUMBER_TO_GENERATE * PASSWORDS_LENGTH);

    for password in passwords {
        buffer.extend(password);
        buffer.push(b'\n');
    }

    let password_str = str::from_utf8(&buffer).expect("Failed to convert to string");

    writer.write_all(password_str.as_bytes()).expect("Failed to write passwords");

    writer.flush().expect("Failed to flush buffer");

    println!("Passwords successfully written in {}", filename);
}

fn generate_x_passwords(filename: &str, num: usize) {
    let passwords: Vec<Vec<u8>> = (0..num)
        .into_par_iter()
        .map(|_| generate_password())
        .collect();

    write_passwords(filename, &passwords);
}
