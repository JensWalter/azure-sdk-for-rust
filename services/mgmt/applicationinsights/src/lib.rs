#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-03-only")]
pub mod package_2021_03_only;
#[cfg(all(feature = "package-2021-03-only", not(feature = "no-default-version")))]
pub use package_2021_03_only::{models, operations};
#[cfg(feature = "package-preview-2021-03-only")]
pub mod package_preview_2021_03_only;
#[cfg(all(feature = "package-preview-2021-03-only", not(feature = "no-default-version")))]
pub use package_preview_2021_03_only::{models, operations};
#[cfg(feature = "package-2020-11-only")]
pub mod package_2020_11_only;
#[cfg(all(feature = "package-2020-11-only", not(feature = "no-default-version")))]
pub use package_2020_11_only::{models, operations};
#[cfg(feature = "package-2020-10-only")]
pub mod package_2020_10_only;
#[cfg(all(feature = "package-2020-10-only", not(feature = "no-default-version")))]
pub use package_2020_10_only::{models, operations};
#[cfg(feature = "package-preview-2020-10-only")]
pub mod package_preview_2020_10_only;
#[cfg(all(feature = "package-preview-2020-10-only", not(feature = "no-default-version")))]
pub use package_preview_2020_10_only::{models, operations};
#[cfg(feature = "package-preview-2020-06-only")]
pub mod package_preview_2020_06_only;
#[cfg(all(feature = "package-preview-2020-06-only", not(feature = "no-default-version")))]
pub use package_preview_2020_06_only::{models, operations};
#[cfg(feature = "package-2020-04")]
pub mod package_2020_04;
#[cfg(all(feature = "package-2020-04", not(feature = "no-default-version")))]
pub use package_2020_04::{models, operations};
#[cfg(feature = "package-2020-03-01-preview")]
pub mod package_2020_03_01_preview;
#[cfg(all(feature = "package-2020-03-01-preview", not(feature = "no-default-version")))]
pub use package_2020_03_01_preview::{models, operations};
#[cfg(feature = "package-preview-2020-02")]
pub mod package_preview_2020_02;
#[cfg(all(feature = "package-preview-2020-02", not(feature = "no-default-version")))]
pub use package_preview_2020_02::{models, operations};
#[cfg(feature = "package-2020-02-02-preview")]
pub mod package_2020_02_02_preview;
#[cfg(all(feature = "package-2020-02-02-preview", not(feature = "no-default-version")))]
pub use package_2020_02_02_preview::{models, operations};
#[cfg(feature = "package-2020-02-02")]
pub mod package_2020_02_02;
#[cfg(all(feature = "package-2020-02-02", not(feature = "no-default-version")))]
pub use package_2020_02_02::{models, operations};
#[cfg(feature = "package-2019-10-17-preview")]
pub mod package_2019_10_17_preview;
#[cfg(all(feature = "package-2019-10-17-preview", not(feature = "no-default-version")))]
pub use package_2019_10_17_preview::{models, operations};
#[cfg(feature = "package-2018-06-17-preview")]
pub mod package_2018_06_17_preview;
#[cfg(all(feature = "package-2018-06-17-preview", not(feature = "no-default-version")))]
pub use package_2018_06_17_preview::{models, operations};
#[cfg(feature = "package-2018-05-01-preview")]
pub mod package_2018_05_01_preview;
#[cfg(all(feature = "package-2018-05-01-preview", not(feature = "no-default-version")))]
pub use package_2018_05_01_preview::{models, operations};
#[cfg(feature = "package-2017-10")]
pub mod package_2017_10;
#[cfg(all(feature = "package-2017-10", not(feature = "no-default-version")))]
pub use package_2017_10::{models, operations};
#[cfg(feature = "package-2015-05")]
pub mod package_2015_05;
use azure_core::setters;
#[cfg(all(feature = "package-2015-05", not(feature = "no-default-version")))]
pub use package_2015_05::{models, operations};
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
