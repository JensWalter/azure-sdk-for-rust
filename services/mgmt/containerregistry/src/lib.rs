#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2020-11-preview")]
mod package_2020_11_preview;
#[cfg(feature = "package-2020-11-preview")]
pub use package_2020_11_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-12-preview")]
mod package_2019_12_preview;
#[cfg(feature = "package-2019-12-preview")]
pub use package_2019_12_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-06-preview")]
mod package_2019_06_preview;
#[cfg(feature = "package-2019-06-preview")]
pub use package_2019_06_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-06-preview-only")]
mod package_2019_06_preview_only;
#[cfg(feature = "package-2019-06-preview-only")]
pub use package_2019_06_preview_only::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-05")]
mod package_2019_05;
#[cfg(feature = "package-2019-05")]
pub use package_2019_05::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-05-preview")]
mod package_2019_05_preview;
#[cfg(feature = "package-2019-05-preview")]
pub use package_2019_05_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-04")]
mod package_2019_04;
#[cfg(feature = "package-2019-04")]
pub use package_2019_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-04-only")]
mod package_2019_04_only;
#[cfg(feature = "package-2019-04-only")]
pub use package_2019_04_only::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-09")]
mod package_2018_09;
#[cfg(feature = "package-2018-09")]
pub use package_2018_09::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-02-preview")]
mod package_2018_02_preview;
#[cfg(feature = "package-2018-02-preview")]
pub use package_2018_02_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-10")]
mod package_2017_10;
#[cfg(feature = "package-2017-10")]
pub use package_2017_10::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-06-preview")]
mod package_2017_06_preview;
#[cfg(feature = "package-2017-06-preview")]
pub use package_2017_06_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-03")]
mod package_2017_03;
#[cfg(feature = "package-2017-03")]
pub use package_2017_03::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-06-preview")]
mod package_2016_06_preview;
use azure_core::setters;
#[cfg(feature = "package-2016-06-preview")]
pub use package_2016_06_preview::{models, operations, API_VERSION};
pub fn config(
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
    token_credential: Box<dyn azure_core::TokenCredential>,
) -> OperationConfigBuilder {
    OperationConfigBuilder {
        api_version: None,
        http_client,
        base_path: None,
        token_credential,
        token_credential_resource: None,
    }
}
pub struct OperationConfigBuilder {
    api_version: Option<String>,
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
    base_path: Option<String>,
    token_credential: Box<dyn azure_core::TokenCredential>,
    token_credential_resource: Option<String>,
}
impl OperationConfigBuilder {
    setters! { api_version : String => Some (api_version) , base_path : String => Some (base_path) , token_credential_resource : String => Some (token_credential_resource) , }
    pub fn build(self) -> OperationConfig {
        OperationConfig {
            api_version: self.api_version.unwrap_or(API_VERSION.to_owned()),
            http_client: self.http_client,
            base_path: self.base_path.unwrap_or("https://management.azure.com".to_owned()),
            token_credential: Some(self.token_credential),
            token_credential_resource: self.token_credential_resource.unwrap_or("https://management.azure.com/".to_owned()),
        }
    }
}
pub struct OperationConfig {
    api_version: String,
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
    base_path: String,
    token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    token_credential_resource: String,
}
impl OperationConfig {
    pub fn api_version(&self) -> &str {
        self.api_version.as_str()
    }
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.http_client.as_ref().as_ref()
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
