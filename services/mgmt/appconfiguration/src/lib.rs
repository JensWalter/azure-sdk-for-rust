#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-03-01-preview")]
pub mod package_2021_03_01_preview;
#[cfg(all(feature = "package-2021-03-01-preview", not(feature = "no-default-version")))]
pub use package_2021_03_01_preview::{models, operations};
#[cfg(feature = "package-2020-07-01-preview")]
pub mod package_2020_07_01_preview;
#[cfg(all(feature = "package-2020-07-01-preview", not(feature = "no-default-version")))]
pub use package_2020_07_01_preview::{models, operations};
#[cfg(feature = "package-2020-06-01")]
pub mod package_2020_06_01;
#[cfg(all(feature = "package-2020-06-01", not(feature = "no-default-version")))]
pub use package_2020_06_01::{models, operations};
#[cfg(feature = "package-2019-11-01-preview")]
pub mod package_2019_11_01_preview;
#[cfg(all(feature = "package-2019-11-01-preview", not(feature = "no-default-version")))]
pub use package_2019_11_01_preview::{models, operations};
#[cfg(feature = "package-2019-02-01-preview")]
pub mod package_2019_02_01_preview;
#[cfg(all(feature = "package-2019-02-01-preview", not(feature = "no-default-version")))]
pub use package_2019_02_01_preview::{models, operations};
#[cfg(feature = "package-2019-10-01")]
pub mod package_2019_10_01;
use azure_core::setters;
#[cfg(all(feature = "package-2019-10-01", not(feature = "no-default-version")))]
pub use package_2019_10_01::{models, operations};
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
