# passwords_generator

This repository contains password generator scripts implemented in Rust and Python programming languages. The scripts allow you to generate a chosen number of passwords with a specified length. There are two versions of each script: one that focuses on generating passwords quickly and another that emphasizes generating highly secure passwords using system generation.

## Folder Structure

The repository is organized into two folders:

- `rust`: This folder contains the Rust implementation of the password generator scripts.
- `python`: This folder contains the Python implementation of the password generator scripts.

Each folder contains the following scripts:

### Rust

- `main/main.rs`: Generates a chosen number of passwords with a chosen length as quickly as possible.
- `secure/main.rs`: Generates the most secure passwords using system generation, but it might be slower than the main.rs script.
- `simplified/main.rs`: Generates passwords after prompting for number to generate, length of passwords and target file name (based on main.rs).

### Python

- `main.py`: Generates a chosen number of passwords with a chosen length as quickly as possible.
- `secure.py`: Generates the most secure passwords using system generation, but it might be slower than the main.py script.

## Performance Measurements

Based on performance measurements, the time taken to generate and write passwords to a chosen file was calculated for different scenarios. The following results were obtained:

### Generating 500,000 Passwords of Length 10

- `main.py`: Approximately 3.40 seconds
- `secure.py`: Approximately 11.80 seconds
- `main.rs`: Approximately 2.70 seconds
- `secure.rs`: Approximately 3.80 seconds
- `simplified.rs`: Approximately 9.60 seconds

### Generating 1,000,000 Passwords of Length 10

- `main.py`: Approximately 6.80 seconds
- `secure.py`: Approximately 23.60 seconds
- `main.rs`: Approximately 5.30 seconds
- `secure.rs`: Approximately 7.20 seconds
- `simplified.rs`: Approximately 20 seconds

Please note that these measurements are based on the specific testing conditions and may vary depending on the hardware, software environment, and other factors.

Feel free to explore and utilize these password generator scripts as per your requirements. If you have any questions or suggestions, please don't hesitate to reach out or open an issue in the repository.
