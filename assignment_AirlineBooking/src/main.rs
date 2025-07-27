mod model;
mod errors;
mod engine;

use engine::Plane;
use std::cell::RefCell;
use std::io::{self, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn main() {
    let plane = RefCell::new(Plane::new(5));

    loop {
        println!("\n--- Airline Booking System ---");
        println!("1. Display Seat Status");
        println!("2. Book Specific Seat");
        println!("3. Auto-Book First Available Seat");
        println!("4. Cancel Booking");
        println!("5. Simulate Overbooking Panic");
        println!("6. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                plane.borrow().display();
            }

            "2" => {
                let (seat_number, name) = get_seat_and_name();
                match plane.borrow_mut().book_seat(seat_number, name) {
                    Ok(_) => println!("Seat {} booked successfully!", seat_number),
                    Err(e) => println!("{}", e),
                }
            }

            "3" => {
                let name = get_name_only();
                match plane.borrow_mut().auto_book(name) {
                    Ok(seat) => println!("Auto-booked seat number: {}", seat),
                    Err(e) => println!("{}", e),
                }
            }

            "4" => {
                let seat = get_seat_number_only();
                match plane.borrow_mut().cancel_booking(seat) {
                    Ok(_) => println!("Seat {} cancelled.", seat),
                    Err(e) => println!("{}", e),
                }
            }

            "5" => {
                println!("Simulating Overbooking...");
                let result = catch_unwind(AssertUnwindSafe(|| {
                    for i in 0..10 {
                        let name = format!("User{}", i);
                        let _ = plane.borrow_mut().auto_book(name);
                    }
                }));

                if result.is_err() {
                    println!("Panic occurred and was recovered!");
                }
            }

            "6" => {
                println!("Exiting. Thank you for using the Airline Booking System.");
                break;
            }

            _ => println!("Invalid option. Please choose 1–6."),
        }
    }
}

fn get_seat_and_name() -> (usize, String) {
    let seat = get_seat_number_only();
    let name = get_name_only();
    (seat, name)
}

fn get_seat_number_only() -> usize {
    print!("Enter seat number: ");
    io::stdout().flush().unwrap();
    let mut seat_input = String::new();
    io::stdin().read_line(&mut seat_input).unwrap();
    seat_input.trim().parse().unwrap_or(0)
}

fn get_name_only() -> String {
    print!("Enter passenger name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    name.trim().to_string()
}
