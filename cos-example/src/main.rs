/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

// https://docs.aws.amazon.com/sdk-for-rust/latest/dg/s3-object-lambda.html

use aws_config::meta::region::RegionProviderChain;
use aws_config::provider_config::ProviderConfig;
use aws_sdk_s3::{ByteStream, Client, Credentials, Endpoint, Error, Region, PKG_VERSION};
use aws_types::credentials::{ProvideCredentials, SharedCredentialsProvider};

use aws_config::web_identity_token::StaticConfiguration;
use aws_endpoint::partition::endpoint;
use aws_endpoint::CredentialScope;
use aws_sdk_s3::client::Builder;
use aws_types::config::Config;
use std::path::Path;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The name of the bucket.
    #[structopt(short, long)]
    bucket: String,

    /// The name of the object in the bucket.
    #[structopt(short, long)]
    key: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

/// Lists your buckets and uploads a file to a bucket.
/// # Arguments
///
/// * `-b BUCKET` - The bucket to which the file is uploaded.
/// * `-k KEY` - The name of the file to upload to the bucket.
/// * `[-r REGION]` - The Region in which the client is created.
///    If not supplied, uses the value of the **AWS_REGION** environment variable.
///    If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    // let req = client.list_tables().limit(10);
    // let resp = req.send().await?;
    // println!("Current DynamoDB tables: {:?}", resp.table_names);

    // let Opt {
    //     bucket,
    //     region,
    //     key,
    //     verbose,
    // } = Opt::from_args();

    let key = "Cargo.toml";
    let bucket = std::env::var("BUCKET").expect("BUCKET is not found");
    let region = "ap-beijing";

    // let region_provider = RegionProviderChain::first_try()
    //     .or_default_provider()
    //     .or_else(Region::new("us-west-2"));

    // let region = region.map(Region::new);

    let access_key_id = std::env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found");
    let secret_access_key =
        std::env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found");

    let credential = Credentials::new(&access_key_id, &secret_access_key, None, None, "");

    let credential = SharedCredentialsProvider::new(credential);

    let region = Region::new(region);
    println!("region: {:?}", region);

    let endpoint = endpoint::Metadata {
        // Important: Replace the $accountNumber below with your AWS account number,
        // and $accessPoint with the name of your S3 Object Lambda access point.
        // For example, "my-access-point-123123123123.s3-object-lambda.{region}.amazonaws.com".
        // If you're using a FIPs region, add `-fips` after `s3-object-lambda`.
        uri_template: "cos.{region}.myqcloud.com",
        protocol: endpoint::Protocol::Https,
        signature_versions: endpoint::SignatureVersion::V4,
        // Important: The following overrides the credential scope so that request signing works.
        credential_scope: CredentialScope::builder()
            .service("s3-object-lambda")
            .build(),
    };

    // let config = Config::builder()
    //     .region(region)
    //     .credentials_provider(credential)
    //     .build();

    let config = aws_sdk_s3::Config::builder()
        .endpoint_resolver(endpoint)
        .region(region)
        .credentials_provider(credential)
        .build();

    // let config = aws_config::ConfigLoader::default()
    //     .region(region)
    //     .load()
    //     .await;

    // let provider_config = aws_config::provider_config::ProviderConfig::empty()
    //     .with_region(Some(region))
    //     .load_default_region()
    //     .await;

    // ProviderConfig::empty().with_region(None);

    // let shared_config = aws_config::from_env().region(region_provider).load().await;
    // aws_config::Config::builder().set_credentials_provider(None);
    let client = Client::from_conf(config);

    println!();

    if true {
        println!("S3 client version: {}", PKG_VERSION);
        // println!("Region:            {}", config.region().unwrap());
        println!("Bucket:            {}", &bucket);
        println!("Key:               {}", &key);
        println!();
    }

    let resp = client.list_buckets().send().await?;

    for bucket in resp.buckets.unwrap_or_default() {
        println!("bucket: {:?}", bucket.name.as_deref().unwrap_or_default())
    }

    let body = ByteStream::from_path(Path::new("Cargo.toml")).await;

    match body {
        Ok(b) => {
            let resp = client
                .put_object()
                .bucket(&bucket)
                .key(key)
                .body(b)
                .send()
                .await?;

            println!("Upload success. Version: {:?}", resp.version_id);

            let resp = client.get_object().bucket(&bucket).key(key).send().await?;
            let data = resp.body.collect().await;
            println!("data: {:?}", data.unwrap().into_bytes());
        }
        Err(e) => {
            println!("Got an error DOING SOMETHING:");
            println!("{}", e);
            process::exit(1);
        }
    }

    Ok(())
}
