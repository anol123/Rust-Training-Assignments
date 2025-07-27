use crate::models::transaction::Transaction;
use crate::services::transaction_service::Processor;

pub struct LoggingProcessor {
    processor: Box<dyn Processor + Send>,
}

impl LoggingProcessor {
    pub fn new(processor: Box<dyn Processor + Send>) -> Self {
        LoggingProcessor { processor }
    }
}

impl Processor for LoggingProcessor {
    fn process(&self, transaction: &Transaction) -> Result<(), String> {
        println!("[LOG] Starting transaction: {:?}", transaction);
        let result = self.processor.process(transaction);
        println!("[LOG] Transaction completed with result: {:?}", result);
        result
    }
}