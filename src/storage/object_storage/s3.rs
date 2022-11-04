use std::env;

use super::common::KeyType;
use super::common::ObjectStorage;
use super::common::OpError;
use super::common::SmallValueType;

use async_trait::async_trait;

use aws_sdk_s3::types::ByteStream;
use aws_smithy_http::body::SdkBody;

#[derive(Debug)]
pub struct AwsS3Client {
    bucket_name: String,
    s3_client: aws_sdk_s3::Client,
}

impl AwsS3Client {
    #[allow(dead_code)]
    async fn new() -> AwsS3Client {
        let bucket_name = env::var("BUCKET_NAME").unwrap_or("default".to_string());

        let shared_config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&shared_config);

        // head_bucket returns Ok(()) if found and you have permission else error.
        let rsp = client.list_buckets().send().await;
        match rsp {
            Ok(output) => println!("buckets:{:?}", output.buckets()),
            Err(err) => println!("new client error: {}", err.to_string()),
        }

        AwsS3Client {
            bucket_name,
            s3_client: client,
        }
    }
}

#[async_trait]
impl ObjectStorage for AwsS3Client {
    async fn get(&self, _key: KeyType) -> Result<Option<&SmallValueType>, OpError> {
        unimplemented!()
    }

    async fn put(&self, key: KeyType, value: &SmallValueType) -> Result<(), OpError> {
        let body = ByteStream::new(SdkBody::from(value));

        let rsp = self
            .s3_client
            .put_object()
            .bucket(self.bucket_name.clone())
            .key(key.to_owned())
            .body(body)
            .send()
            .await;

        match rsp {
            Ok(_) => Ok(()),
            Err(e) => Err(OpError::new(&e.to_string())),
        }
    }

    async fn rename(&self, _src_key: KeyType, _dst_key: KeyType) -> Result<(), OpError> {
        unimplemented!()
    }

    async fn copy(&self, _src_key: KeyType, _dst_key: KeyType) -> Result<(), OpError> {
        unimplemented!()
    }

    async fn remove(&self, _key: KeyType) -> Result<(), OpError> {
        unimplemented!()
    }
}
