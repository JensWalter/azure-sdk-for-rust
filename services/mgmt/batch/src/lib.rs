#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "no-default-version")))]
pub use package_2021_06::{models, operations};
#[cfg(feature = "package-2021-01")]
pub mod package_2021_01;
#[cfg(all(feature = "package-2021-01", not(feature = "no-default-version")))]
pub use package_2021_01::{models, operations};
#[cfg(feature = "package-2020-09")]
pub mod package_2020_09;
#[cfg(all(feature = "package-2020-09", not(feature = "no-default-version")))]
pub use package_2020_09::{models, operations};
#[cfg(feature = "package-2020-05")]
pub mod package_2020_05;
#[cfg(all(feature = "package-2020-05", not(feature = "no-default-version")))]
pub use package_2020_05::{models, operations};
#[cfg(feature = "package-2020-03")]
pub mod package_2020_03;
#[cfg(all(feature = "package-2020-03", not(feature = "no-default-version")))]
pub use package_2020_03::{models, operations};
#[cfg(feature = "package-2019-08")]
pub mod package_2019_08;
#[cfg(all(feature = "package-2019-08", not(feature = "no-default-version")))]
pub use package_2019_08::{models, operations};
#[cfg(feature = "package-2019-04")]
pub mod package_2019_04;
#[cfg(all(feature = "package-2019-04", not(feature = "no-default-version")))]
pub use package_2019_04::{models, operations};
#[cfg(feature = "package-2018-12")]
pub mod package_2018_12;
#[cfg(all(feature = "package-2018-12", not(feature = "no-default-version")))]
pub use package_2018_12::{models, operations};
#[cfg(feature = "package-2017-09")]
pub mod package_2017_09;
use azure_core::setters;
#[cfg(all(feature = "package-2017-09", not(feature = "no-default-version")))]
pub use package_2017_09::{models, operations};
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
