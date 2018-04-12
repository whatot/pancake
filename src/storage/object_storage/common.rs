use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Default)]
pub struct OpError {
    message: String,
}

impl Error for OpError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

pub type KeyType = String;
pub type SmallValueType = [u8];

impl OpError {
    pub fn new(input: &str) -> Self {
        OpError {
            message: format!("ObjectStorage Operation failed: {}", input),
        }
    }
}

pub trait ObjectStorage {
    fn get(&self, key: KeyType) -> Result<Option<&SmallValueType>, OpError>;
    fn put(&self, key: KeyType, value: &SmallValueType) -> Result<(), OpError>;
    fn rename(&self, src_key: KeyType, dst_key: KeyType) -> Result<(), OpError>;
    fn copy(&self, src_key: KeyType, dst_key: KeyType) -> Result<(), OpError>;
    fn remove(&self, key: KeyType) -> Result<(), OpError>;
}
