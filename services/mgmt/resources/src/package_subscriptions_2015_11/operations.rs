#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use snafu::{ResultExt, Snafu};
pub mod subscriptions {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn list_locations(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<LocationListResult, list_locations::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/subscriptions/{}/locations", operation_config.base_path(), subscription_id);
        let mut url = url::Url::parse(url_str).context(list_locations::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list_locations::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list_locations::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(list_locations::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: LocationListResult =
                    serde_json::from_slice(rsp_body).context(list_locations::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list_locations::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod list_locations {
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
    pub async fn get(operation_config: &crate::OperationConfig, subscription_id: &str) -> std::result::Result<Subscription, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/subscriptions/{}", operation_config.base_path(), subscription_id);
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
                let rsp_value: Subscription = serde_json::from_slice(rsp_body).context(get::DeserializeError { body: rsp_body.clone() })?;
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
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<SubscriptionListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/subscriptions", operation_config.base_path(),);
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
                let rsp_value: SubscriptionListResult =
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
pub mod tenants {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<TenantListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/tenants", operation_config.base_path(),);
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
                let rsp_value: TenantListResult =
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
pub async fn check_resource_name(
    operation_config: &crate::OperationConfig,
    resource_name_definition: Option<&ResourceName>,
) -> std::result::Result<CheckResourceNameResult, check_resource_name::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!("{}/providers/Microsoft.Resources/checkResourceName", operation_config.base_path(),);
    let mut url = url::Url::parse(url_str).context(check_resource_name::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .context(check_resource_name::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
    let req_body = if let Some(resource_name_definition) = resource_name_definition {
        azure_core::to_json(resource_name_definition).context(check_resource_name::SerializeError)?
    } else {
        bytes::Bytes::from_static(azure_core::EMPTY_BODY)
    };
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder.body(req_body).context(check_resource_name::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .context(check_resource_name::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => {
            let rsp_body = rsp.body();
            let rsp_value: CheckResourceNameResult =
                serde_json::from_slice(rsp_body).context(check_resource_name::DeserializeError { body: rsp_body.clone() })?;
            Ok(rsp_value)
        }
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: ErrorResponse =
                serde_json::from_slice(rsp_body).context(check_resource_name::DeserializeError { body: rsp_body.clone() })?;
            check_resource_name::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod check_resource_name {
    use crate::{models, models::*};
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        ParseUrlError {
            source: url::ParseError,
        },
        BuildRequestError {
            source: http::Error,
        },
        ExecuteRequestError {
            source: Box<dyn std::error::Error + Sync + Send>,
        },
        SerializeError {
            source: Box<dyn std::error::Error + Sync + Send>,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
        GetTokenError {
            source: azure_core::errors::AzureError,
        },
    }
}
