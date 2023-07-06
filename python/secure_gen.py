import random
import string
import time
import secrets
PASSWORDS_LENGTH = 10
NUMBER_TO_GENERATE = 500000
TARGET_FILE = "passwords.txt"

secure_random = secrets.SystemRandom()


def get_random_char(characters):
    random_index = secure_random.randint(0, len(characters) - 1)
    return characters[random_index]


def timer(fn):
    def wrapper(*args, **kwargs):
        start = time.time()
        result = fn(*args, **kwargs)
        end = time.time()
        print(f"function execution: {end - start} seconds")
        return result
    return wrapper


def generate_password():
    # Define the characters to be used in the password
    lowercase_letters = string.ascii_lowercase
    uppercase_letters = string.ascii_uppercase
    digits = string.digits
    special_characters = string.punctuation
    all_characters = "".join(
        [lowercase_letters, uppercase_letters, digits, special_characters])

    if PASSWORDS_LENGTH < 4:
        raise ValueError("passwords_length must be at least 4")

    # Generate a random password
    password = [
        get_random_char(lowercase_letters),
        get_random_char(uppercase_letters),
        get_random_char(digits),
        get_random_char(special_characters)
    ] + [get_random_char(all_characters) for _ in range(4, PASSWORDS_LENGTH)]

    random.shuffle(password)
    password_str = ''.join(password)

    return password_str


def write_passwords(filename: str, passwords: list[str]):
    with open(filename, "w") as file:
        writer = file.buffer
        for password in passwords:
            writer.write(password.encode() + b"\n")

    print(f"Passwords successfully written in {filename}")


@timer
def generate_x_passwords(filename: str, num: int):
    passwords = [generate_password() for _ in range(num)]

    write_passwords(filename, passwords)


generate_x_passwords(TARGET_FILE, NUMBER_TO_GENERATE)
