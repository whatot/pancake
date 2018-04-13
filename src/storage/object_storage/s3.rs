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

        let mut signature = Signature::V4;
        if s3_config.use_v2_signature {
            signature = Signature::V2
        }

        let endpoint_url: Option<Url>;
        let mut region: Region = Region::UsEast1;
        if s3_config.is_aws_s3 {
            region = Region::from_str(&s3_config.region_name).unwrap();
            endpoint_url = None;
        } else {
            let url_str: String;
            if s3_config.endpoint_url.starts_with("http") {
                url_str = s3_config.endpoint_url
            } else if s3_config.use_https {
                url_str = "https://".to_string() + &s3_config.endpoint_url;
            } else {
                url_str = "http://".to_string() + &s3_config.endpoint_url;
            }
            endpoint_url = Some(Url::parse(url_str.as_str()).unwrap());
        }

        let proxy: Option<Url>;
        if s3_config.proxy_url.is_empty() {
            proxy = None;
        } else {
            proxy = Some(Url::parse(s3_config.proxy_url.as_str()).unwrap());
        }

        let endpoint = Endpoint::new(region, signature, endpoint_url, proxy, None, Some(false));

        let client = S3Client::new(provider, endpoint);
        let head_bucket_req = HeadBucketRequest {
            bucket: s3_config.bucket_name.clone(),
        };
        // head_bucket returns Ok(()) if found and you have permission else error.
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
