#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2017-07")]
pub mod package_2017_07;
#[cfg(all(feature = "package-2017-07", not(feature = "no-default-version")))]
pub use package_2017_07::{models, operations};
#[cfg(feature = "package-2015-01")]
pub mod package_2015_01;
#[cfg(all(feature = "package-2015-01", not(feature = "no-default-version")))]
pub use package_2015_01::{models, operations};
#[cfg(feature = "package-2018-08-preview")]
pub mod package_2018_08_preview;
#[cfg(all(feature = "package-2018-08-preview", not(feature = "no-default-version")))]
pub use package_2018_08_preview::{models, operations};
#[cfg(feature = "package-2020-05-01-preview")]
pub mod package_2020_05_01_preview;
#[cfg(all(feature = "package-2020-05-01-preview", not(feature = "no-default-version")))]
pub use package_2020_05_01_preview::{models, operations};
#[cfg(feature = "package-2018-07-01")]
pub mod package_2018_07_01;
#[cfg(all(feature = "package-2018-07-01", not(feature = "no-default-version")))]
pub use package_2018_07_01::{models, operations};
#[cfg(feature = "package-2020-05-01")]
pub mod package_2020_05_01;
use azure_core::setters;
#[cfg(all(feature = "package-2020-05-01", not(feature = "no-default-version")))]
pub use package_2020_05_01::{models, operations};
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
