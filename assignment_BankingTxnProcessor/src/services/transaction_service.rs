use crate::models::transaction::Transaction;

pub trait Processor {
    fn process(&self, transaction: &Transaction) -> Result<(), String>;
}