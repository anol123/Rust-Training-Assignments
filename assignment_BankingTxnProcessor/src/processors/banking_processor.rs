use crate::models::transaction::Transaction;
use crate::services::transaction_service::Processor;

pub struct BankingProcessor;

impl Processor for BankingProcessor {
    fn process(&self, transaction: &Transaction) -> Result<(), String> {
        match transaction {
            Transaction::Deposit { account_id, amount } => {
                println!("Processing deposit of ${:.2} to account {}", amount, account_id);
                Ok(())
            }
            Transaction::Withdraw { account_id, amount } => {
                println!("Processing withdrawal of ${:.2} from account {}", amount, account_id);
                Ok(())
            }
            Transaction::Transfer { from_account, to_account, amount } => {
                println!("Processing transfer of ${:.2} from account {} to account {}", 
                    amount, from_account, to_account);
                Ok(())
            }
        }
    }
}