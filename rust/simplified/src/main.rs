use dialoguer::theme::ColorfulTheme;
use rand::seq::SliceRandom;
use rand::Rng;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use dialoguer::Input;
use rayon::prelude::*;
use std::str;

fn main() -> Result<(), std::io::Error> {
    let number_to_generate: String = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("How many passwords do you want to generate?")
        .default("100000".into())
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<usize>() {
                Err(_) => {
                    return Err("Invalid number of passwords to generate!");
                }
                Ok(_) => {
                    Ok(())
                }
            }
        })        
        .interact_text()?;

    let number_to_generate: usize = number_to_generate.parse::<usize>().unwrap();

    let passwords_length: String = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("What length should these passwords have?")
        .default("8".into())
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<usize>() {
                Err(_) => {
                    return Err("Invalid passwords length!");
                }
                Ok(val) => {
                    if val < 4 && number_to_generate > 0 {
                        return Err("Passwords must be at least 4 characters long!");
                    }
                    Ok(())
                }
            }
        })
        .interact_text()?;

    let passwords_length: usize = passwords_length.parse::<usize>().unwrap();

    let file_name: String = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("How to name the file to put the passwords in?")
        .default("passwords.txt".into())
        .interact_text()?;

    timer(|| {
        generate_x_passwords(&file_name, number_to_generate, passwords_length);
    });

    Ok(())
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

fn generate_password(length: &usize) -> Vec<u8> {
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = "0123456789";
    let special_characters = "!@#$%^&*()";
    let all_characters: String =
        lowercase_letters.to_owned() + uppercase_letters + digits + special_characters;

    let mut rng = rand::thread_rng();
    let mut password = vec![0; *length];

    password[0] = get_random_char(&lowercase_letters, &mut rng) as u8;
    password[1] = get_random_char(&uppercase_letters, &mut rng) as u8;
    password[2] = get_random_char(&digits, &mut rng) as u8;
    password[3] = get_random_char(&special_characters, &mut rng) as u8;

    for i in 4..*length {
        password[i] = get_random_char(&all_characters, &mut rng) as u8;
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
    let mut writer: BufWriter<File> = BufWriter::with_capacity(65536, file);

    for password in passwords {
        writer
            .write_all(password)
            .expect("Failed to write password");
        writer.write_all(b"\n").expect("Failed to write newline");
    }

    writer.flush().expect("Failed to flush buffer");

    println!("Passwords successfully written in {}", filename);
}

fn generate_x_passwords(filename: &str, num: usize, length: usize) {
    if num == 0 {
        println!("No passwords generated.");
        return;
    }

    let passwords: Vec<Vec<u8>> = (0..num)
        .into_par_iter()
        .map(|_| generate_password(&length))
        .collect();

    write_passwords(filename, &passwords);
}
