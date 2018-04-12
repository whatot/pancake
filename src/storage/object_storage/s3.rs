use super::common::KeyType;
use super::common::ObjectStorage;
use super::common::OpError;
use super::common::SmallValueType;
use aws_sdk_rust::aws::common::credentials::ChainProvider;
use aws_sdk_rust::aws::common::credentials::ParametersProvider;
use aws_sdk_rust::aws::common::region::Region;
use aws_sdk_rust::aws::s3::bucket::*;
use aws_sdk_rust::aws::s3::endpoint::{Endpoint, Signature};
use aws_sdk_rust::aws::s3::object::*;
use aws_sdk_rust::aws::s3::s3client::S3Client;
use hyper::Client as AwsHttpClient;
use std::str::FromStr;
use url::Url;

#[derive(Debug)]
pub struct AwsS3Client {
    bucket_name: String,
    s3_client: S3Client<ChainProvider, AwsHttpClient>,
}

#[derive(Debug)]
pub struct S3Config {
    access_key_id: String,
    secret_access_key: String,
    bucket_name: String,
    endpoint_url: String,
    use_https: bool,
    use_v2_signature: bool,
    is_aws_s3: bool,
    region_name: String,
    proxy_url: String,
}

impl AwsS3Client {
    fn new(s3_config: S3Config) -> AwsS3Client {
        let param_provider: Option<ParametersProvider> = Some(
            ParametersProvider::with_parameters(
                s3_config.access_key_id,
                s3_config.secret_access_key,
                None,
            ).unwrap(),
        );
        let provider = ChainProvider::new(param_provider);

        let mut region: Region = Region::UsEast1;
        if s3_config.is_aws_s3 {
            region = Region::from_str(&s3_config.region_name).unwrap();
        }

        let mut signature = Signature::V4;
        if s3_config.use_v2_signature {
            signature = Signature::V2
        }

        let endpoint_fix_url: String;
        if s3_config.endpoint_url.starts_with("http") {
            endpoint_fix_url = s3_config.endpoint_url
        } else if s3_config.use_https {
            endpoint_fix_url = "https://".to_string() + &s3_config.endpoint_url;
        } else {
            endpoint_fix_url = "http://".to_string() + &s3_config.endpoint_url;
        }

        let endpoint = Endpoint::new(
            region,
            signature,
            Some(Url::parse(endpoint_fix_url.as_str()).unwrap()),
            Some(Url::parse(s3_config.proxy_url.as_str()).unwrap()),
            None,
            Some(false),
        );

        let client = S3Client::new(provider, endpoint);
        // check whether bucket is reachable
        let head_bucket_req = HeadBucketRequest {
            bucket: s3_config.bucket_name.clone(),
        };
        client.head_bucket(&head_bucket_req).unwrap();

        AwsS3Client {
            bucket_name: s3_config.bucket_name.to_owned(),
            s3_client: client,
        }
    }
}

impl ObjectStorage for AwsS3Client {
    fn get(&self, _key: KeyType) -> Result<Option<&SmallValueType>, OpError> {
        unimplemented!()
    }

    fn put(&self, key: KeyType, value: &SmallValueType) -> Result<(), OpError> {
        let mut put_object_req = PutObjectRequest::default();
        put_object_req.bucket = self.bucket_name.to_owned();
        put_object_req.key = key.to_owned();
        put_object_req.body = Some(value);
        match self.s3_client.put_object(&put_object_req, None) {
            Ok(_) => Ok(()),
            Err(e) => Err(OpError::new(e.message.as_str())),
        }
    }

    fn rename(&self, _src_key: KeyType, _dst_key: KeyType) -> Result<(), OpError> {
        unimplemented!()
    }

    fn copy(&self, _src_key: KeyType, _dst_key: KeyType) -> Result<(), OpError> {
        unimplemented!()
    }

    fn remove(&self, _key: KeyType) -> Result<(), OpError> {
        unimplemented!()
    }
}
