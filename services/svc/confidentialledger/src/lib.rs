#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-0.1-preview-ledger")]
pub mod package_0_1_preview_ledger;
#[cfg(all(feature = "package-0.1-preview-ledger", not(feature = "no-default-version")))]
pub use package_0_1_preview_ledger::{models, operations};
#[cfg(feature = "package-0.1-preview-identity")]
pub mod package_0_1_preview_identity;
use azure_core::setters;
#[cfg(all(feature = "package-0.1-preview-identity", not(feature = "no-default-version")))]
pub use package_0_1_preview_identity::{models, operations};
pub fn config(
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    token_credential: Box<dyn azure_core::TokenCredential>,
) -> OperationConfigBuilder {
    OperationConfigBuilder {
        http_client,
        base_path: None,
        token_credential,
        token_credential_resource: None,
    }
}
pub struct OperationConfigBuilder {
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: Option<String>,
    token_credential: Box<dyn azure_core::TokenCredential>,
    token_credential_resource: Option<String>,
}
impl OperationConfigBuilder {
    setters! { base_path : String => Some (base_path) , token_credential_resource : String => Some (token_credential_resource) , }
    pub fn build(self) -> OperationConfig {
        OperationConfig {
            http_client: self.http_client,
            base_path: self.base_path.unwrap_or("https://management.azure.com".to_owned()),
            token_credential: Some(self.token_credential),
            token_credential_resource: self.token_credential_resource.unwrap_or("https://management.azure.com/".to_owned()),
        }
    }
}
pub struct OperationConfig {
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: String,
    token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    token_credential_resource: String,
}
impl OperationConfig {
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.http_client.as_ref()
    }
    pub fn base_path(&self) -> &str {
        self.base_path.as_str()
    }
    pub fn token_credential(&self) -> Option<&dyn azure_core::TokenCredential> {
        self.token_credential.as_deref()
    }
    pub fn token_credential_resource(&self) -> &str {
        self.token_credential_resource.as_str()
    }
}
