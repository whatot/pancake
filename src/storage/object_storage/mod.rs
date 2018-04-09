pub mod s3;

pub trait ObjectStorage {
    pub type KeyType = &str;
    pub type ContentValue = &[u8];
    fn get(&self, key: KeyType) -> Result;
    fn put(&self, key: KeyType, value: ContentValue) -> Result;
    fn move(&self, src_key: KeyType, dst_key: KeyType) -> Result;
    fn copy(&self, src_key: KeyType, dst_key: KeyType) -> Result;
    fn remove(&self, key: KeyType) -> Result;
}
