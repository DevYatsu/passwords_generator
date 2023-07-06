use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use rayon::prelude::*;

const PASSWORDS_LENGTH: usize = 10;
const NUMBER_TO_GENERATE: usize = 100;
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
    let all_characters: String = lowercase_letters.to_owned() + uppercase_letters + digits + special_characters;

    let mut password: [u8; PASSWORDS_LENGTH] = [0; PASSWORDS_LENGTH];


    password[0] = get_random_char(lowercase_letters) as u8;
    password[1] = get_random_char(uppercase_letters) as u8;
    password[2] = get_random_char(digits) as u8;
    password[3] = get_random_char(special_characters) as u8;

    for i in 4..PASSWORDS_LENGTH {
        password[i] = get_random_char(&all_characters) as u8;
    }

    let mut rng: rand::rngs::ThreadRng = thread_rng();
    password.shuffle(&mut rng);
    String::from_utf8(password.into_iter().collect()).unwrap()
}

fn get_random_char(characters: &str) -> char {
    let random_index: usize = getrandom_index(characters.len());
    characters.chars().nth(random_index).unwrap()
}

fn getrandom_index(upper_bound: usize) -> usize {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    rng.gen_range(0..upper_bound)
}

fn write_passwords(filename: &str, passwords: &[String]) {
    let file: File = File::create(filename).expect("Failed to create file");
    let mut writer: BufWriter<File> = BufWriter::with_capacity(65536, file);

    for password in passwords {
        writeln!(writer, "{}", password).expect("Failed to write password");
    }

    writer.flush().expect("Failed to flush buffer");

    println!("Passwords successfully written in {}", filename);
}

fn generate_x_passwords(filename: &str, num: usize) {
     let passwords: Vec<String> = (0..num)
        .into_par_iter()
        .map(|_| generate_password())
        .collect();

    write_passwords(filename, &passwords);
}
