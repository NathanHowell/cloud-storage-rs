//! This crate aims to simplify interacting with the Google Cloud Storage JSON API. Use it until
//! Google releases a Cloud Storage Client Library for Rust. Shoutout to
//! [MyEmma](https://myemma.io/) for funding this free and open source project.
//!
//! Google Cloud Storage is a product by Google that allows for cheap file storage, with a
//! relatively sophisticated API. The idea is that storage happens in `Bucket`s, which are
//! filesystems with a globally unique name. You can make as many of these `Bucket`s as you like!
//!
//! This project talks to Google using a `Service Account`. A service account is an account that you
//! must create in the [cloud storage console](https://console.cloud.google.com/). When the account
//! is created, you can download the file `service-account-********.json`. Store this file somewhere
//! on your machine, and place the path to this file in the environment parameter `SERVICE_ACCOUNT`.
//! Environment parameters declared in the `.env` file are also registered. The service account can
//! then be granted `Roles` in the cloud storage console. The roles required for this project to
//! function are `Service Account Token Creator` and `Storage Object Admin`.
//!
//! # Quickstart
//! Add the following line to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! cloud-storage = "0.6"
//! ```
//! The two most important concepts are [Buckets](bucket/struct.Bucket.html), which represent
//! file systems, and [Objects](object/struct.Object.html), which represent files.
//!
//! ## Examples:
//! Creating a new Bucket in Google Cloud Storage:
//! ```rust
//! # use cloud_storage::{Bucket, NewBucket};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let bucket = Bucket::create(&NewBucket {
//!     name: "doctest-bucket".to_string(),
//!     ..Default::default()
//! }).await?;
//! # bucket.delete().await?;
//! # Ok(())
//! # }
//! ```
//! Connecting to an existing Bucket in Google Cloud Storage:
//! ```no_run
//! # use cloud_storage::Bucket;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let bucket = Bucket::read("mybucket").await?;
//! # Ok(())
//! # }
//! ```
//! Read a file from disk and store it on googles server:
//! ```rust,no_run
//! # use cloud_storage::Object;
//! # use std::fs::File;
//! # use std::io::Read;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut bytes: Vec<u8> = Vec::new();
//! for byte in File::open("myfile.txt")?.bytes() {
//!     bytes.push(byte?)
//! }
//! Object::create("mybucket", bytes, "myfile.txt", "text/plain").await?;
//! # Ok(())
//! # }
//! ```
//! Renaming/moving a file
//! ```rust,no_run
//! # use cloud_storage::Object;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut object = Object::read("mybucket", "myfile").await?;
//! object.name = "mybetterfile".to_string();
//! object.update().await?;
//! # Ok(())
//! # }
//! ```
//! Removing a file
//! ```rust,no_run
//! # use cloud_storage::Object;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! Object::delete("mybucket", "myfile").await?;
//! # Ok(())
//! # }
//! ```
#![forbid(unsafe_code, missing_docs)]

mod download_options;
mod error;
/// Contains objects as represented by Google, to be used for serialization and deserialization.
mod resources;

pub use crate::error::*;
use crate::resources::service_account::ServiceAccount;
pub use crate::resources::{
    bucket::{Bucket, NewBucket},
    object::Object,
    *,
};
pub use download_options::DownloadOptions;
use gouth::Token;
use reqwest::{IntoUrl, RequestBuilder};
use std::fmt::{self, Debug, Formatter};

///
///
pub struct Client {
    /// Static `Token` struct that caches
    token: Token,

    /// Project ID
    project_id: String,

    /// The struct is the parsed service account json file. It is publicly exported to enable easier
    /// debugging of which service account is currently used. It is of the type
    /// [ServiceAccount](service_account/struct.ServiceAccount.html).
    pub service_account: Option<ServiceAccount>,

    client: reqwest::Client,
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("token", &"...")
            .field("project_id", &self.project_id)
            .field("service_account", &self.service_account)
            .field("client", &self.client)
            .finish()
    }
}

fn gcloud_project() -> Result<String> {
    let output = std::process::Command::new("gcloud")
        .args(&["config", "get-value", "project", "--format=json"])
        .envs(std::env::vars())
        .output()?;
    Ok(serde_json::from_slice(output.stdout.as_slice())?)
}

fn project_id() -> Result<String> {
    std::env::var("GOOGLE_CLOUD_PROJECT")
        .or_else(|_| gcloud_project())
        .or_else(|_| Ok(gcemeta::project_id()?))
}

impl Client {
    ///
    pub fn new() -> Result<Self> {
        Ok(Client {
            token: gouth::Builder::new()
                .scopes(&["https://www.googleapis.com/auth/devstorage.full_control"])
                .build()?,
            project_id: project_id()?,
            service_account: ServiceAccount::from_env().ok(),
            client: reqwest::Client::new(),
        })
    }
}

/// A type alias where the error is set to be `cloud_storage::Error`.
pub type Result<T> = std::result::Result<T, crate::Error>;

const BASE_URL: &str = "https://www.googleapis.com/storage/v1";

impl Client {
    async fn get_headers(&self) -> Result<reqwest::header::HeaderMap> {
        let mut result = reqwest::header::HeaderMap::new();
        let token = self.token.header_value()?;
        result.insert(reqwest::header::AUTHORIZATION, token.parse()?);
        Ok(result)
    }

    async fn delete<U: IntoUrl>(&self, url: U) -> Result<RequestBuilder> {
        Ok(self.client.delete(url).headers(self.get_headers().await?))
    }

    async fn get<U: IntoUrl>(&self, url: U) -> Result<RequestBuilder> {
        Ok(self.client.get(url).headers(self.get_headers().await?))
    }

    async fn post<U: IntoUrl>(&self, url: U) -> Result<RequestBuilder> {
        Ok(self.client.post(url).headers(self.get_headers().await?))
    }

    async fn put<U: IntoUrl>(&self, url: U) -> Result<RequestBuilder> {
        Ok(self.client.put(url).headers(self.get_headers().await?))
    }
}

fn from_str<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

fn from_str_opt<'de, T, D>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: serde::Deserializer<'de>,
{
    let s: std::result::Result<serde_json::Value, _> =
        serde::Deserialize::deserialize(deserializer);
    match s {
        Ok(serde_json::Value::String(s)) => T::from_str(&s)
            .map_err(serde::de::Error::custom)
            .map(Option::from),
        Ok(serde_json::Value::Number(num)) => T::from_str(&num.to_string())
            .map_err(serde::de::Error::custom)
            .map(Option::from),
        Ok(_value) => Err(serde::de::Error::custom("Incorrect type")),
        Err(_) => Ok(None),
    }
}

#[cfg(test)]
async fn read_test_bucket() -> Bucket {
    dotenv::dotenv().ok();
    let name = std::env::var("TEST_BUCKET").unwrap();
    let client = crate::Client::new().unwrap();
    match Bucket::read(&client, &name).await {
        Ok(bucket) => bucket,
        Err(_not_found) => Bucket::create(
            &client,
            &NewBucket {
                name,
                ..NewBucket::default()
            },
        )
        .await
        .unwrap(),
    }
}

// since all tests run in parallel, we need to make sure we do not create multiple buckets with
// the same name in each test.
#[cfg(test)]
async fn create_test_bucket(client: &crate::Client, name: &str) -> Bucket {
    std::thread::sleep(std::time::Duration::from_millis(1500)); // avoid getting rate limited

    dotenv::dotenv().ok();
    let base_name = std::env::var("TEST_BUCKET").unwrap();
    let name = format!("{}-{}", base_name, name);
    let new_bucket = NewBucket {
        name,
        ..NewBucket::default()
    };
    match Bucket::create(&client, &new_bucket).await {
        Ok(bucket) => bucket,
        Err(_alread_exists) => Bucket::read(&client, &new_bucket.name).await.unwrap(),
    }
}
