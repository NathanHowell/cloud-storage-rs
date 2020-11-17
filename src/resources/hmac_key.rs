#![allow(unused_imports)]
#![allow(dead_code)]

use crate::error::GoogleResponse;

/// The `HmacKey` resource represents an HMAC key within Cloud Storage. The resource consists of a
/// secret and `HmacMeta`. HMAC keys can be used as credentials for service accounts. For more
/// information, see HMAC Keys.
///
/// Note that the `HmacKey` resource is only returned when you use `HmacKey::create`. Other
/// methods, such as `HmacKey::read`, return the metadata portion of the HMAC key resource.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HmacKey {
    /// The kind of item this is. For HMAC keys, this is always `storage#hmacKey`.
    pub kind: String,
    /// HMAC key metadata.
    pub metadata: HmacMeta,
    /// HMAC secret key material.
    pub secret: String,
}

/// Contains information about an Hmac Key.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HmacMeta {
    /// The kind of item this is. For HMAC key metadata, this is always `storage#hmacKeyMetadata`.
    pub kind: String,
    /// The ID of the HMAC key, including the Project ID and the Access ID.
    pub id: String,
    /// The link to this resource.
    pub self_link: String,
    /// The access ID of the HMAC Key.
    pub access_id: String,
    /// The Project ID of the project that owns the service account to which the key authenticates.
    pub project_id: String,
    /// The email address of the key's associated service account.
    pub service_account_email: String,
    /// The state of the key.
    pub state: HmacState,
    /// The creation time of the HMAC key.
    pub time_created: chrono::DateTime<chrono::Utc>,
    /// The last modification time of the HMAC key metadata.
    pub updated: chrono::DateTime<chrono::Utc>,
    /// HTTP 1.1 Entity tag for the HMAC key.
    pub etag: String,
}

/// The state of an Hmac Key.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HmacState {
    /// This Hmac key is currently used.
    Active,
    /// This Hmac key has been set to inactive.
    Inactive,
    /// This Hmac key has been permanently deleted.
    Deleted,
}

#[derive(Debug, serde::Deserialize)]
struct ListResponse {
    items: Vec<HmacMeta>,
}

#[derive(serde::Serialize)]
struct UpdateRequest {
    secret: String,
    metadata: UpdateMeta,
}

#[derive(serde::Serialize)]
struct UpdateMeta {
    state: HmacState,
}

impl HmacKey {
    /// Creates a new HMAC key for the specified service account.
    ///
    /// The authenticated user must have `storage.hmacKeys.create` permission for the project in
    /// which the key will be created.
    ///
    /// For general information about HMAC keys in Cloud Storage, see
    /// [HMAC Keys](https://cloud.google.com/storage/docs/authentication/hmackeys).
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let hmac_key = HmacKey::create().await?;
    /// # use cloud_storage::hmac_key::HmacState;
    /// # HmacKey::update(&hmac_key.metadata.access_id, HmacState::Inactive).await?;
    /// # HmacKey::delete(&hmac_key.metadata.access_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[tracing::instrument]
    pub async fn create(cloud_storage: &crate::Client) -> crate::Result<Self> {
        use reqwest::header::CONTENT_LENGTH;

        let service_account = cloud_storage
            .service_account
            .as_ref()
            .ok_or(crate::Error::MissingServiceAccount)?;

        let url = format!(
            "{}/projects/{}/hmacKeys",
            crate::BASE_URL,
            cloud_storage.project_id
        );
        let query = [("serviceAccountEmail", &service_account.client_email)];
        cloud_storage
            .post(&url)
            .await?
            .header(CONTENT_LENGTH, 0)
            .query(&query)
            .send()
            .await?
            .json::<GoogleResponse<Self>>()
            .await?
            .into_result()
    }

    /// Retrieves a list of HMAC keys matching the criteria. Since the HmacKey is secret, this does
    /// not return a `HmacKey`, but a `HmacMeta`. This is a redacted version of a `HmacKey`, but
    /// with the secret data omitted.
    ///
    /// The authenticated user must have `storage.hmacKeys.list` permission for the project in which
    /// the key exists.
    ///
    /// For general information about HMAC keys in Cloud Storage, see
    /// [HMAC Keys](https://cloud.google.com/storage/docs/authentication/hmackeys).
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let all_hmac_keys = HmacKey::list().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[tracing::instrument]
    pub async fn list(cloud_storage: &crate::Client) -> crate::Result<Vec<HmacMeta>> {
        let url = format!(
            "{}/projects/{}/hmacKeys",
            crate::BASE_URL,
            cloud_storage.project_id
        );

        // This function requires more complicated error handling because when there is only one
        // entry, Google will return the response `{ "kind": "storage#hmacKeysMetadata" }` instead
        // of a list with one element. This breaks the parser.
        cloud_storage
            .get(&url)
            .await?
            .send()
            .await?
            .json::<GoogleResponse<ListResponse>>()
            .await
            .map_or_else(
                |_| Ok(vec![]),
                |parsed| parsed.into_result().map(|s| s.items),
            )
    }

    /// Retrieves an HMAC key's metadata. Since the HmacKey is secret, this does not return a
    /// `HmacKey`, but a `HmacMeta`. This is a redacted version of a `HmacKey`, but with the secret
    /// data omitted.
    ///
    /// The authenticated user must have `storage.hmacKeys.get` permission for the project in which
    /// the key exists.
    ///
    /// For general information about HMAC keys in Cloud Storage, see
    /// [HMAC Keys](https://cloud.google.com/storage/docs/authentication/hmackeys).
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let key = HmacKey::read("some identifier").await?;
    /// # Ok(())
    /// # }
    #[tracing::instrument]
    pub async fn read(cloud_storage: &crate::Client, access_id: &str) -> crate::Result<HmacMeta> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            cloud_storage.project_id,
            access_id
        );
        cloud_storage
            .get(&url)
            .await?
            .send()
            .await?
            .json::<GoogleResponse<HmacMeta>>()
            .await?
            .into_result()
    }

    /// Updates the state of an HMAC key. See the HMAC Key resource descriptor for valid states.
    /// Since the HmacKey is secret, this does not return a `HmacKey`, but a `HmacMeta`. This is a
    /// redacted version of a `HmacKey`, but with the secret data omitted.
    ///
    /// The authenticated user must have `storage.hmacKeys.update` permission for the project in
    /// which the key exists.
    ///
    /// For general information about HMAC keys in Cloud Storage, see
    /// [HMAC Keys](https://cloud.google.com/storage/docs/authentication/hmackeys).
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::hmac_key::{HmacKey, HmacState};
    ///
    /// let key = HmacKey::update("your key", HmacState::Active).await?;
    /// # Ok(())
    /// # }
    #[tracing::instrument]
    pub async fn update(
        cloud_storage: &crate::Client,
        access_id: &str,
        state: HmacState,
    ) -> crate::Result<HmacMeta> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            cloud_storage.project_id,
            access_id
        );
        cloud_storage
            .put(&url)
            .await?
            .json(&UpdateMeta { state })
            .send()
            .await?
            .json::<GoogleResponse<HmacMeta>>()
            .await?
            .into_result()
    }

    /// Deletes an HMAC key. Note that a key must be set to `Inactive` first.
    ///
    /// The authenticated user must have storage.hmacKeys.delete permission for the project in which
    /// the key exists.
    ///
    /// For general information about HMAC keys in Cloud Storage, see
    /// [HMAC Keys](https://cloud.google.com/storage/docs/authentication/hmackeys).
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::hmac_key::{HmacKey, HmacState};
    ///
    /// let key = HmacKey::update("your key", HmacState::Inactive).await?; // this is required.
    /// HmacKey::delete(&key.access_id).await?;
    /// # Ok(())
    /// # }
    #[tracing::instrument]
    pub async fn delete(cloud_storage: &crate::Client, access_id: &str) -> crate::Result<()> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            cloud_storage.project_id,
            access_id
        );
        cloud_storage
            .delete(&url)
            .await?
            .send()
            .await?
            .json::<GoogleResponse<()>>()
            .await?
            .into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_test_hmac() -> HmacMeta {
        let client = crate::Client::new().unwrap();
        match HmacKey::create(&client).await {
            Ok(key) => key.metadata,
            Err(_) => HmacKey::list(&client).await.unwrap().pop().unwrap(),
        }
    }

    async fn remove_test_hmac(access_id: &str) {
        let client = crate::Client::new().unwrap();
        HmacKey::update(&client, access_id, HmacState::Inactive)
            .await
            .unwrap();
        HmacKey::delete(&client, access_id).await.unwrap();
    }

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new().unwrap();
        let key = HmacKey::create(&client).await?;
        remove_test_hmac(&key.metadata.access_id).await;
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new().unwrap();
        HmacKey::list(&client).await?;
        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new().unwrap();
        let key = get_test_hmac().await;
        HmacKey::read(&client, &key.access_id).await?;
        remove_test_hmac(&key.access_id).await;
        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new().unwrap();
        let key = get_test_hmac().await;
        HmacKey::update(&client, &key.access_id, HmacState::Inactive).await?;
        HmacKey::delete(&client, &key.access_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new().unwrap();
        let key = get_test_hmac().await;
        HmacKey::update(&client, &key.access_id, HmacState::Inactive).await?;
        HmacKey::delete(&client, &key.access_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn clear_keys() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new().unwrap();
        let keys = HmacKey::list(&client).await?;
        for key in &keys {
            if key.state != HmacState::Inactive {
                HmacKey::update(&client, &key.access_id, HmacState::Inactive).await?;
            }
            HmacKey::delete(&client, &key.access_id).await?;
        }
        Ok(())
    }
}
