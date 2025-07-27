// src/main.rs
mod models;
mod processors;
mod services;
mod utils;

use std::sync::mpsc;
use std::thread;
use std::io::{self, Write};

use models::transaction::Transaction;
use processors::banking_processor::BankingProcessor;
use processors::logging_processor::LoggingProcessor;
use services::transaction_service::Processor;
use utils::input::get_input;

fn main() {
    let (tx, rx) = mpsc::channel();
    let processor = LoggingProcessor::new(Box::new(BankingProcessor));

    // Start processing thread
    let processor_thread = thread::spawn(move || {
        for received_transaction in rx {
            println!("\nProcessing transaction in background thread...");
            if let Err(e) = processor.process(&received_transaction) {
                eprintln!("Error processing transaction: {}", e);
            }
        }
        println!("Transaction queue closed, processor shutting down.");
    });

    // User interaction loop
    loop {
        println!("\nBanking Transaction Processor");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Transfer");
        println!("4. Exit");
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => handle_deposit(&tx),
            "2" => handle_withdraw(&tx),
            "3" => handle_transfer(&tx),
            "4" => break,
            _ => println!("Invalid option, please try again."),
        }
    }

    // Clean shutdown
    drop(tx);
    processor_thread.join().unwrap();
}

fn handle_deposit(tx: &mpsc::Sender<Transaction>) {
    println!("\nDeposit Transaction");
    let account_id = get_input("Enter account ID: ");
    let amount = get_input("Enter amount: ");

    match (account_id.parse::<u32>(), amount.parse::<f64>()) {
        (Ok(account_id), Ok(amount)) if amount > 0.0 => {
            tx.send(Transaction::Deposit { account_id, amount }).unwrap();
            println!("Deposit transaction queued successfully!");
        }
        _ => println!("Invalid input! Account ID must be a number and amount must be positive."),
    }
}

fn handle_withdraw(tx: &mpsc::Sender<Transaction>) {
    println!("\nWithdraw Transaction");
    let account_id = get_input("Enter account ID: ");
    let amount = get_input("Enter amount: ");

    match (account_id.parse::<u32>(), amount.parse::<f64>()) {
        (Ok(account_id), Ok(amount)) if amount > 0.0 => {
            tx.send(Transaction::Withdraw { account_id, amount }).unwrap();
            println!("Withdrawal transaction queued successfully!");
        }
        _ => println!("Invalid input! Account ID must be a number and amount must be positive."),
    }
}

fn handle_transfer(tx: &mpsc::Sender<Transaction>) {
    println!("\nTransfer Transaction");
    let from_account = get_input("Enter source account ID: ");
    let to_account = get_input("Enter destination account ID: ");
    let amount = get_input("Enter amount: ");

    match (
        from_account.parse::<u32>(),
        to_account.parse::<u32>(),
        amount.parse::<f64>(),
    ) {
        (Ok(from_account), Ok(to_account), Ok(amount)) if amount > 0.0 => {
            tx.send(Transaction::Transfer {
                from_account,
                to_account,
                amount,
            })
            .unwrap();
            println!("Transfer transaction queued successfully!");
        }
        _ => println!(
            "Invalid input! Account IDs must be numbers and amount must be positive."
        ),
    }
}