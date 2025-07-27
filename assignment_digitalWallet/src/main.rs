use std::collections::HashMap;
use std::io;

use assignment_digitalWallet::domain::User;
use assignment_digitalWallet::logger::{ConsoleLogger, Logger};
use assignment_digitalWallet::service::transfer_funds;

fn main() {
    let mut users: HashMap<String, User> = HashMap::new();

    users.insert(
        "alice".to_string(),
        User::new("alice".to_string(), "pass123".to_string(), 1000.0),
    );
    users.insert(
        "bob".to_string(),
        User::new("bob".to_string(), "bobpass".to_string(), 500.0),
    );

    let logger: Box<dyn Logger> = Box::new(ConsoleLogger);

    println!("Enter username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim().to_string();

    // Authenticate first
    if !users.contains_key(&username) {
        println!("User not found.");
        return;
    }

    println!("Enter password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    if users.get(&username).unwrap().authenticate(password).is_err() {
        println!("Authentication failed.");
        return;
    }

    println!("Welcome, {}!", username);

    loop {
        println!("1. Check Balance\n2. Transfer Funds\n3. Show History\n4. Exit");

    let mut option = String::new();
    io::stdin().read_line(&mut option).unwrap();
    let option = option.trim();

    match option {
        "1" => {
            let user = users.get(&username).unwrap();
            println!("Balance: ₹{}", user.check_balance());
        }
        "2" => {
            println!("Enter recipient username:");
            let mut recipient = String::new();
            io::stdin().read_line(&mut recipient).unwrap();
            let recipient = recipient.trim().to_string();

            println!("Enter amount:");
            let mut amount_str = String::new();
            io::stdin().read_line(&mut amount_str).unwrap();
            let amount: f64 = amount_str.trim().parse().unwrap_or(0.0);

            // Get mutable references in one scope
            let (sender, receiver) = {
                // Prevent same name transfer
                if recipient == username {
                    println!("Cannot transfer to self.");
                    return;
                }

                let sender_ptr = users.get_mut(&username).unwrap() as *mut User;
                let receiver_ptr = users.get_mut(&recipient);
                if receiver_ptr.is_none() {
                    println!("Recipient not found.");
                    return;
                }

                // SAFETY: sender and receiver are guaranteed not to be the same due to check above
                unsafe { (&mut *sender_ptr, receiver_ptr.unwrap()) }
            };

            match transfer_funds(sender, receiver, amount, &*logger) {
                Ok(_) => println!("Transfer successful!"),
                Err(e) => println!("Transfer failed: {:?}", e),
            }
        }
        "3" => {
            let user = users.get(&username).unwrap();
            user.show_history(5);
        }
        "4" => {
                        println!("Exiting...");
                        break;
                    }
        _ => println!("Invalid option."),
    }
    }
}
