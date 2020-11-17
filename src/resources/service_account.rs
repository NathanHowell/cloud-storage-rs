use std::fmt::{self, Debug, Formatter};

/// A deserialized `service-account-********.json`-file.
#[derive(serde::Deserialize)]
pub struct ServiceAccount {
    /// The type of authentication, this should always be `service_account`.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The name of the current project.
    pub project_id: String,
    /// A unique identifier for the private key.
    pub private_key_id: String,
    /// The private key in RSA format.
    pub private_key: String,
    /// The email address associated with the service account.
    pub client_email: String,
    /// The unique identifier for this client.
    pub client_id: String,
    /// The endpoint where authentication happens.
    pub auth_uri: String,
    /// The endpoint where OAuth2 tokens are issued.
    pub token_uri: String,
    /// The url of the cert provider.
    pub auth_provider_x509_cert_url: String,
    /// The url of a static file containing metadata for this certificate.
    pub client_x509_cert_url: String,
}

impl Debug for ServiceAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServiceAccount")
            .field("type", &self.r#type)
            .field("project_id", &self.project_id)
            .field("private_key_id", &self.private_key_id)
            .field("private_key", &"...")
            .field("client_email", &self.client_email)
            .field("client_id", &self.client_id)
            .field("auth_uri", &self.auth_uri)
            .field("token_uri", &self.token_uri)
            .field(
                "auth_provider_x509_cert_url",
                &self.auth_provider_x509_cert_url,
            )
            .field("client_x509_cert_url", &self.client_x509_cert_url)
            .finish()
    }
}

impl ServiceAccount {
    pub(crate) fn from_env() -> Result<Self, crate::Error> {
        dotenv::dotenv().ok();
        let path = std::env::var("SERVICE_ACCOUNT")
            .or_else(|_| std::env::var("GOOGLE_APPLICATION_CREDENTIALS"))
            .map_err(|e| crate::Error::Other(e.to_string()))?;
        let file = std::fs::read_to_string(path).expect("SERVICE_ACCOUNT file not found");
        let account: Self = serde_json::from_str(&file).expect("service account file not valid");
        if account.r#type != "service_account" {
            panic!("`type` parameter of `SERVICE_ACCOUNT` variable is not 'service_account'");
        }
        Ok(account)
    }
}
