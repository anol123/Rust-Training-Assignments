#[derive(Debug)]
pub struct Record {
    pub key: String,
    pub value: String,
}

impl Record {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
