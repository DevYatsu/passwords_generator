import random
import string
import time
passwords_length = 10
number_to_generate = 100000
target_file = "passwords.txt"


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

    if passwords_length < 4:
        raise ValueError("passwords_length must be at least 4")

    # Generate a random password
    password = random.choice(lowercase_letters) \
        + random.choice(uppercase_letters) \
        + random.choice(digits) \
        + random.choice(special_characters) \
        + ''.join(random.choice(string.ascii_letters +
                                string.digits + string.punctuation) for _ in range(passwords_length - 4))

    password_list = list(password)
    random.shuffle(password_list)
    password = ''.join(password_list)

    return password


def write_passwords(filename: str, passwords: list[str]):
    with open(filename, "w") as file:
        for password in passwords:
            file.write(f"{password}\n")

    print(f"Passwords successfully written in {filename}")


@timer
def generate_x_passwords(filename: str, num: int):
    passwords = []

    for i in range(num):
        passwords.append(generate_password())

    write_passwords(filename, passwords)


generate_x_passwords(target_file, number_to_generate)
