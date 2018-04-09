extern crate aws_sdk_rust;

use aws_sdk_rust::aws::common::credentials::DefaultCredentialsProvider;
use aws_sdk_rust::aws::common::region::Region;
use aws_sdk_rust::aws::s3::endpoint::{Endpoint, Signature};
use aws_sdk_rust::aws::s3::s3client::S3Client;
use super::ObjectStorage;

struct AwsS3Client {
    bucket_name: String,
    s3_client: S3Client,
}

pub struct S3Config {
    access_key_id: String,
    secret_access_key: String,
    bucket_name: String,
    endpoint_url: String,
    use_https: bool,
    use_v4_signature: bool,
    region_name: String,
}

impl ObjectStorage for AwsS3Client {}
