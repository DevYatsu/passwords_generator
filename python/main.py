import random
import string
import time

PASSWORDS_LENGTH = 10
NUMBER_TO_GENERATE = 500000
TARGET_FILE = "passwords.txt"


def get_random_char(characters):
    random_index = random.randint(0, len(characters) - 1)
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
    all_characters = lowercase_letters + \
        uppercase_letters + digits + special_characters

    if PASSWORDS_LENGTH < 4:
        raise ValueError("passwords_length must be at least 4")

    # Generate a random password
    password = [
        random.choice(lowercase_letters),
        random.choice(uppercase_letters),
        random.choice(digits),
        random.choice(special_characters)
    ] + [random.choice(all_characters) for _ in range(PASSWORDS_LENGTH - 4)]

    random.shuffle(password)
    password_str = ''.join(password)

    return password_str


def write_passwords(filename: str, passwords: list[str]):
    with open(filename, "w") as file:
        writer = file.buffer
        writer.writelines(
            (password.encode() + b"\n" for password in passwords))

    print(f"Passwords successfully written in {filename}")


@timer
def generate_x_passwords(filename: str, num: int):
    passwords = [generate_password() for _ in range(num)]
    write_passwords(filename, passwords)


generate_x_passwords(TARGET_FILE, NUMBER_TO_GENERATE)
