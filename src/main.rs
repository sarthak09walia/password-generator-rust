use std::io;
use rand::Rng;

fn get_password_length() -> usize {
  let mut length = String::new();

  loop {
    println!("Enter the desired password length:");
    io::stdin().read_line(&mut length).unwrap();

    match length.trim().parse() {
      Ok(value) if value >= 6 => {
        return value;
      }
      _ => {
        println!(
          "Password length should be a number greater than or equal to 6. Please try again."
        );
        length.clear();
      }
    }
  }
}

fn character_set() -> [bool; 4] {
  let mut choices = [false; 4];

  for i in 0..4 {
    let mut input = String::new();
    loop {
      println!("Do you need ");
      match i {
        0 => println!("uppercase characters (true/T or false/F):"),
        1 => println!("lowercase characters (true/T or false/F):"),
        2 => println!("numbers (true/T or false/F):"),
        3 => println!("special characters (true/T or false/F):"),
        _ => println!("unknown option"),
      }
      io::stdin().read_line(&mut input).unwrap();

      input = input.trim().to_lowercase(); // Convert input to lowercase for easier comparison

      if input == "t" || input == "true" {
        choices[i] = true;
        break;
      } else if input == "f" || input == "false" {
        choices[i] = false;
        break;
      } else {
        println!("Invalid input. Please enter 'T', 't', 'true', 'F', 'f', or 'false'.");
        input.clear();
      }
    }
  }

  choices
}

fn password_generator(input: [bool; 4], length: usize) -> String {
  let uppercase_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  let lowercase_chars = "abcdefghijklmnopqrstuvwxyz";
  let number_chars = "0123456789";
  let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";

  let mut password = String::new();

  let mut character_sets = Vec::<&str>::new();

  if input[0] {
    character_sets.push(uppercase_chars);
  }
  if input[1] {
    character_sets.push(lowercase_chars);
  }
  if input[2] {
    character_sets.push(number_chars);
  }
  if input[3] {
    character_sets.push(special_chars);
  }

  let mut rng = rand::thread_rng();

  for _ in 0..length {
    if character_sets.is_empty() {
      break;
    }

    let random_set_index = rng.gen_range(0..character_sets.len());
    let selected_set = character_sets[random_set_index];

    let random_char_index = rng.gen_range(0..selected_set.len());
    let random_char = selected_set.chars().nth(random_char_index).unwrap();

    password.push(random_char);
  }

  password
}

fn main() {
  let mut password_length = get_password_length();
  if password_length < 6 {
    println!("Password length should be greater than 6");
    password_length = get_password_length();
  } else {
    println!("Password Length: {}", password_length);
  }
  let character_set = character_set();

  let password = password_generator(character_set, password_length);
  println!("Password: {:?}", password)
}
