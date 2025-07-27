#[derive(Debug)]
pub enum WalletError {
    IncorrectPassword,
    InsufficientFunds,
    UserNotFound,
    InvalidAmount,
}
