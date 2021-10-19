#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-preview-2021-09")]
pub mod package_preview_2021_09;
#[cfg(all(feature = "package-preview-2021-09", not(feature = "no-default-version")))]
pub use package_preview_2021_09::{models, operations, API_VERSION};
#[cfg(feature = "package-2021-07")]
pub mod package_2021_07;
#[cfg(all(feature = "package-2021-07", not(feature = "no-default-version")))]
pub use package_2021_07::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-01-23-preview")]
pub mod package_2019_01_23_preview;
#[cfg(all(feature = "package-2019-01-23-preview", not(feature = "no-default-version")))]
pub use package_2019_01_23_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-09-24-preview")]
pub mod package_2019_09_24_preview;
#[cfg(all(feature = "package-2019-09-24-preview", not(feature = "no-default-version")))]
pub use package_2019_09_24_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-12-10-preview")]
pub mod package_2019_12_10_preview;
#[cfg(all(feature = "package-2019-12-10-preview", not(feature = "no-default-version")))]
pub use package_2019_12_10_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-09-21-preview")]
pub mod package_2020_09_21_preview;
#[cfg(all(feature = "package-2020-09-21-preview", not(feature = "no-default-version")))]
pub use package_2020_09_21_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-10-19-preview")]
pub mod package_2020_10_19_preview;
#[cfg(all(feature = "package-2020-10-19-preview", not(feature = "no-default-version")))]
pub use package_2020_10_19_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-11-02-preview")]
pub mod package_2020_11_02_preview;
#[cfg(all(feature = "package-2020-11-02-preview", not(feature = "no-default-version")))]
pub use package_2020_11_02_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-11-10-preview")]
pub mod package_2020_11_10_preview;
#[cfg(all(feature = "package-2020-11-10-preview", not(feature = "no-default-version")))]
pub use package_2020_11_10_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2021-01-14-preview")]
pub mod package_2021_01_14_preview;
#[cfg(all(feature = "package-2021-01-14-preview", not(feature = "no-default-version")))]
pub use package_2021_01_14_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2021-02-01-preview")]
pub mod package_2021_02_01_preview;
#[cfg(all(feature = "package-2021-02-01-preview", not(feature = "no-default-version")))]
pub use package_2021_02_01_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2021-03-09-preview")]
pub mod package_2021_03_09_preview;
#[cfg(all(feature = "package-2021-03-09-preview", not(feature = "no-default-version")))]
pub use package_2021_03_09_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2021-04-01-preview")]
pub mod package_2021_04_01_preview;
use azure_core::setters;
#[cfg(all(feature = "package-2021-04-01-preview", not(feature = "no-default-version")))]
pub use package_2021_04_01_preview::{models, operations, API_VERSION};
#[cfg(not(feature = "no-default-version"))]
fn get_default_feature() -> String {
    API_VERSION.to_owned()
}
#[cfg(feature = "no-default-version")]
fn get_default_feature() -> String {
    "".to_owned()
}
pub fn config(
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
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
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: Option<String>,
    token_credential: Box<dyn azure_core::TokenCredential>,
    token_credential_resource: Option<String>,
}
impl OperationConfigBuilder {
    setters! { api_version : String => Some (api_version) , base_path : String => Some (base_path) , token_credential_resource : String => Some (token_credential_resource) , }
    pub fn build(self) -> OperationConfig {
        OperationConfig {
            api_version: self.api_version.unwrap_or_else(|| get_default_feature()),
            http_client: self.http_client,
            base_path: self.base_path.unwrap_or("https://management.azure.com".to_owned()),
            token_credential: Some(self.token_credential),
            token_credential_resource: self.token_credential_resource.unwrap_or("https://management.azure.com/".to_owned()),
        }
    }
}
pub struct OperationConfig {
    api_version: String,
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: String,
    token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    token_credential_resource: String,
}
impl OperationConfig {
    pub fn api_version(&self) -> &str {
        self.api_version.as_str()
    }
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
