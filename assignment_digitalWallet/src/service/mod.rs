use crate::domain::User;
use crate::error::WalletError;
use crate::logger::Logger;
use crate::domain::Transaction;

pub fn transfer_funds(
    sender: &mut User,
    receiver: &mut User,
    amount: f64,
    logger: &dyn Logger,
) -> Result<(), WalletError> {
    if amount <= 0.0 {
        return Err(WalletError::InvalidAmount);
    }
    if sender.balance < amount {
        return Err(WalletError::InsufficientFunds);
    }

    sender.balance -= amount;
    receiver.balance += amount;

    let txn = Transaction {
        from: sender.username.clone(),
        to: receiver.username.clone(),
        amount,
        message: format!("Transferred ₹{}", amount),
    };

    sender.transaction_history.push(txn.clone());
    receiver.transaction_history.push(txn);

    logger.log("\"Transaction completed.\"");
    Ok(())
}
