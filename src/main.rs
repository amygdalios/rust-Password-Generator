use rand::Rng;
use std::io;

fn generate_password(length: usize, use_uppercase: bool, use_numbers: bool, use_special: bool) -> String {
    let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");

    if use_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_numbers {
        charset.push_str("0123456789");
    }
    if use_special {
        charset.push_str("!@#$%^&*()-_=+[]{};:'\",.<>?/\\|");
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    password
}

fn main() {
    println!("Password Generator :)");

    //loop to check password length
    let password_length = loop {
        println!("Choose password length (8-40): ");
    
        //user input
        let mut input = String::new();
    
        
    
        io::stdin().read_line(&mut input).expect("Failed to read line");
    
        let password_length: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Using default length of 12.");
                continue;
            }
        };

        //check if the number is within the valid range
        if password_length >= 8 && password_length <= 40 {
            break password_length;
        } else {
            println!("Password length must be between 8 and 40.");
        }
    };



    let use_uppercase = true;
    let use_numbers = true;
    let use_special = true;

    let password = generate_password(password_length, use_uppercase, use_numbers, use_special);
    println!("Generated password: {}", password);
}
