use crate::domain::Transaction;
use crate::error::WalletError;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub balance: f64,
    pub transaction_history: Vec<Transaction>,
}

impl User {
    pub fn new(username: String, password: String, balance: f64) -> Self {
        User {
            username,
            password,
            balance,
            transaction_history: Vec::new(),
        }
    }
    pub fn authenticate(&self, password: &str) -> Result<(), WalletError> {
        if self.password == password {
            Ok(())
        } else {
            Err(WalletError::IncorrectPassword)
        }
    }

    pub fn check_balance(&self) -> f64 {
        self.balance
    }

    pub fn show_history(&self, count: usize) {
        let len = self.transaction_history.len();
        let slice = if count > len {
            &self.transaction_history[..]
        } else {
            &self.transaction_history[len - count..]
        };

        for txn in slice {
            println!("{:?}", txn);
        }
    }
}
