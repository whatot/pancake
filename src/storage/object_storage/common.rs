use std::error::Error;
use std::fmt;

use async_trait::async_trait;

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
        write!(f, "{}", self.to_string())
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

#[async_trait]
pub trait ObjectStorage {
    async fn get(&self, key: KeyType) -> Result<Option<&SmallValueType>, OpError>;
    async fn put(&self, key: KeyType, value: &SmallValueType) -> Result<(), OpError>;
    async fn rename(&self, src_key: KeyType, dst_key: KeyType) -> Result<(), OpError>;
    async fn copy(&self, src_key: KeyType, dst_key: KeyType) -> Result<(), OpError>;
    async fn remove(&self, key: KeyType) -> Result<(), OpError>;
}
