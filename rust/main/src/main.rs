use rand::seq::SliceRandom;
use rand::Rng;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use rayon::prelude::*;
use std::str;

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

    let mut rng = rand::thread_rng();
    let mut password: Vec<u8> = Vec::with_capacity(PASSWORDS_LENGTH);

    password.push(get_random_char(&lowercase_letters, &mut rng) as u8);
    password.push(get_random_char(&uppercase_letters, &mut rng) as u8);
    password.push(get_random_char(&digits, &mut rng) as u8);
    password.push(get_random_char(&special_characters, &mut rng) as u8);

    for _ in 4..PASSWORDS_LENGTH {
        password.push(get_random_char(&all_characters, &mut rng) as u8);
    }

    password.shuffle(&mut rng);
    password
}

fn get_random_char(characters: &str, rng: &mut rand::rngs::ThreadRng) -> char {
    let random_index = rng.gen_range(0..characters.len());
    characters.chars().nth(random_index).unwrap()
}

fn write_passwords(filename: &str, passwords: &[Vec<u8>]) {
    let file = File::create(filename).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    for password in passwords {
        writer.write_all(password).expect("Failed to write password");
        writer.write_all(b"\n").expect("Failed to write newline");
    }

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