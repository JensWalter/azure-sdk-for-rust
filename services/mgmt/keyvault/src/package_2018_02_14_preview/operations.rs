#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use snafu::{ResultExt, Snafu};
pub mod vaults {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        vault_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<Vault, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            vault_name
        );
        let mut url = url::Url::parse(url_str).context(get::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(get::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(get::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Vault = serde_json::from_slice(rsp_body).context(get::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                get::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        vault_name: &str,
        parameters: &VaultCreateOrUpdateParameters,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            vault_name
        );
        let mut url = url::Url::parse(url_str).context(create_or_update::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(create_or_update::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(create_or_update::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: Vault =
                    serde_json::from_slice(rsp_body).context(create_or_update::DeserializeError { body: rsp_body.clone() })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Vault =
                    serde_json::from_slice(rsp_body).context(create_or_update::DeserializeError { body: rsp_body.clone() })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                create_or_update::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Created201(Vault),
            Ok200(Vault),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        vault_name: &str,
        parameters: &VaultPatchParameters,
        subscription_id: &str,
    ) -> std::result::Result<update::Response, update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            vault_name
        );
        let mut url = url::Url::parse(url_str).context(update::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PATCH);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(update::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(update::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: Vault = serde_json::from_slice(rsp_body).context(update::DeserializeError { body: rsp_body.clone() })?;
                Ok(update::Response::Created201(rsp_value))
            }
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Vault = serde_json::from_slice(rsp_body).context(update::DeserializeError { body: rsp_body.clone() })?;
                Ok(update::Response::Ok200(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                update::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Created201(Vault),
            Ok200(Vault),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        vault_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<(), delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            vault_name
        );
        let mut url = url::Url::parse(url_str).context(delete::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(delete::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(delete::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(()),
            status_code => {
                let rsp_body = rsp.body();
                delete::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn update_access_policy(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        vault_name: &str,
        operation_kind: &str,
        parameters: &VaultAccessPolicyParameters,
        subscription_id: &str,
    ) -> std::result::Result<update_access_policy::Response, update_access_policy::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}/accessPolicies/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            vault_name,
            operation_kind
        );
        let mut url = url::Url::parse(url_str).context(update_access_policy::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(update_access_policy::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(update_access_policy::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(update_access_policy::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: VaultAccessPolicyParameters =
                    serde_json::from_slice(rsp_body).context(update_access_policy::DeserializeError { body: rsp_body.clone() })?;
                Ok(update_access_policy::Response::Created201(rsp_value))
            }
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: VaultAccessPolicyParameters =
                    serde_json::from_slice(rsp_body).context(update_access_policy::DeserializeError { body: rsp_body.clone() })?;
                Ok(update_access_policy::Response::Ok200(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                update_access_policy::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod update_access_policy {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Created201(VaultAccessPolicyParameters),
            Ok200(VaultAccessPolicyParameters),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        top: Option<i32>,
        subscription_id: &str,
    ) -> std::result::Result<VaultListResult, list_by_resource_group::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults",
            operation_config.base_path(),
            subscription_id,
            resource_group_name
        );
        let mut url = url::Url::parse(url_str).context(list_by_resource_group::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list_by_resource_group::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        if let Some(top) = top {
            url.query_pairs_mut().append_pair("$top", top.to_string().as_str());
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list_by_resource_group::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: VaultListResult =
                    serde_json::from_slice(rsp_body).context(list_by_resource_group::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list_by_resource_group::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_by_subscription(
        operation_config: &crate::OperationConfig,
        top: Option<i32>,
        subscription_id: &str,
    ) -> std::result::Result<VaultListResult, list_by_subscription::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.KeyVault/vaults",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).context(list_by_subscription::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list_by_subscription::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        if let Some(top) = top {
            url.query_pairs_mut().append_pair("$top", top.to_string().as_str());
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list_by_subscription::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(list_by_subscription::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: VaultListResult =
                    serde_json::from_slice(rsp_body).context(list_by_subscription::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list_by_subscription::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod list_by_subscription {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_deleted(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<DeletedVaultListResult, list_deleted::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.KeyVault/deletedVaults",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).context(list_deleted::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list_deleted::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list_deleted::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(list_deleted::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: DeletedVaultListResult =
                    serde_json::from_slice(rsp_body).context(list_deleted::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list_deleted::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod list_deleted {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn get_deleted(
        operation_config: &crate::OperationConfig,
        vault_name: &str,
        location: &str,
        subscription_id: &str,
    ) -> std::result::Result<DeletedVault, get_deleted::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.KeyVault/locations/{}/deletedVaults/{}",
            operation_config.base_path(),
            subscription_id,
            location,
            vault_name
        );
        let mut url = url::Url::parse(url_str).context(get_deleted::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(get_deleted::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(get_deleted::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(get_deleted::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: DeletedVault =
                    serde_json::from_slice(rsp_body).context(get_deleted::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                get_deleted::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod get_deleted {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn purge_deleted(
        operation_config: &crate::OperationConfig,
        vault_name: &str,
        location: &str,
        subscription_id: &str,
    ) -> std::result::Result<purge_deleted::Response, purge_deleted::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.KeyVault/locations/{}/deletedVaults/{}/purge",
            operation_config.base_path(),
            subscription_id,
            location,
            vault_name
        );
        let mut url = url::Url::parse(url_str).context(purge_deleted::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(purge_deleted::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(purge_deleted::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(purge_deleted::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(purge_deleted::Response::Ok200),
            http::StatusCode::ACCEPTED => Ok(purge_deleted::Response::Accepted202),
            status_code => {
                let rsp_body = rsp.body();
                purge_deleted::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod purge_deleted {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        filter: &str,
        top: Option<i32>,
        subscription_id: &str,
    ) -> std::result::Result<ResourceListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/subscriptions/{}/resources", operation_config.base_path(), subscription_id);
        let mut url = url::Url::parse(url_str).context(list::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        url.query_pairs_mut().append_pair("$filter", filter);
        if let Some(top) = top {
            url.query_pairs_mut().append_pair("$top", top.to_string().as_str());
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ResourceListResult =
                    serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn check_name_availability(
        operation_config: &crate::OperationConfig,
        vault_name: &VaultCheckNameAvailabilityParameters,
        subscription_id: &str,
    ) -> std::result::Result<CheckNameAvailabilityResult, check_name_availability::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.KeyVault/checkNameAvailability",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).context(check_name_availability::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(check_name_availability::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(check_name_availability::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(check_name_availability::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CheckNameAvailabilityResult =
                    serde_json::from_slice(rsp_body).context(check_name_availability::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                check_name_availability::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod check_name_availability {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
pub mod operations {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/Microsoft.KeyVault/operations", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).context(list::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: OperationListResult =
                    serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
