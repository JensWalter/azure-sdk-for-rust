#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2018-10-01")]
pub mod package_2018_10_01;
#[cfg(all(feature = "package-2018-10-01", not(feature = "no-default-version")))]
pub use package_2018_10_01::{models, operations};
#[cfg(feature = "package-2019-02-01")]
pub mod package_2019_02_01;
#[cfg(all(feature = "package-2019-02-01", not(feature = "no-default-version")))]
pub use package_2019_02_01::{models, operations};
#[cfg(feature = "package-2019-03-11")]
pub mod package_2019_03_11;
#[cfg(all(feature = "package-2019-03-11", not(feature = "no-default-version")))]
pub use package_2019_03_11::{models, operations};
#[cfg(feature = "package-2019-04-30")]
pub mod package_2019_04_30;
#[cfg(all(feature = "package-2019-04-30", not(feature = "no-default-version")))]
pub use package_2019_04_30::{models, operations};
#[cfg(feature = "package-2019-06-01")]
pub mod package_2019_06_01;
#[cfg(all(feature = "package-2019-06-01", not(feature = "no-default-version")))]
pub use package_2019_06_01::{models, operations};
#[cfg(feature = "package-2019-06-04")]
pub mod package_2019_06_04;
#[cfg(all(feature = "package-2019-06-04", not(feature = "no-default-version")))]
pub use package_2019_06_04::{models, operations};
#[cfg(feature = "package-2019-08-01")]
pub mod package_2019_08_01;
#[cfg(all(feature = "package-2019-08-01", not(feature = "no-default-version")))]
pub use package_2019_08_01::{models, operations};
#[cfg(feature = "package-2019-08-15")]
pub mod package_2019_08_15;
#[cfg(all(feature = "package-2019-08-15", not(feature = "no-default-version")))]
pub use package_2019_08_15::{models, operations};
#[cfg(feature = "package-2019-11-01")]
pub mod package_2019_11_01;
#[cfg(all(feature = "package-2019-11-01", not(feature = "no-default-version")))]
pub use package_2019_11_01::{models, operations};
#[cfg(feature = "package-2020-06-01")]
pub mod package_2020_06_01;
#[cfg(all(feature = "package-2020-06-01", not(feature = "no-default-version")))]
pub use package_2020_06_01::{models, operations};
#[cfg(feature = "package-2020-07-15")]
pub mod package_2020_07_15;
#[cfg(all(feature = "package-2020-07-15", not(feature = "no-default-version")))]
pub use package_2020_07_15::{models, operations};
#[cfg(feature = "package-2020-09-01")]
pub mod package_2020_09_01;
#[cfg(all(feature = "package-2020-09-01", not(feature = "no-default-version")))]
pub use package_2020_09_01::{models, operations};
#[cfg(feature = "package-2020-10-01")]
pub mod package_2020_10_01;
#[cfg(all(feature = "package-2020-10-01", not(feature = "no-default-version")))]
pub use package_2020_10_01::{models, operations};
#[cfg(feature = "package-2020-12-01")]
pub mod package_2020_12_01;
#[cfg(all(feature = "package-2020-12-01", not(feature = "no-default-version")))]
pub use package_2020_12_01::{models, operations};
#[cfg(feature = "package-2021-01-01")]
pub mod package_2021_01_01;
#[cfg(all(feature = "package-2021-01-01", not(feature = "no-default-version")))]
pub use package_2021_01_01::{models, operations};
#[cfg(feature = "package-2021-02-01")]
pub mod package_2021_02_01;
#[cfg(all(feature = "package-2021-02-01", not(feature = "no-default-version")))]
pub use package_2021_02_01::{models, operations};
#[cfg(feature = "package-2021-03-01")]
pub mod package_2021_03_01;
#[cfg(all(feature = "package-2021-03-01", not(feature = "no-default-version")))]
pub use package_2021_03_01::{models, operations};
#[cfg(feature = "package-2021-05-01")]
pub mod package_2021_05_01;
use azure_core::setters;
#[cfg(all(feature = "package-2021-05-01", not(feature = "no-default-version")))]
pub use package_2021_05_01::{models, operations};
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
