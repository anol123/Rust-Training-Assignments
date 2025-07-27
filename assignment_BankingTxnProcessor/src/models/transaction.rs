#[derive(Debug, Clone)]
pub enum Transaction {
    Deposit { account_id: u32, amount: f64 },
    Withdraw { account_id: u32, amount: f64 },
    Transfer { from_account: u32, to_account: u32, amount: f64 },
}