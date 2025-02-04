#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::{models, models::*, API_VERSION};
pub mod service {
    use super::{models, models::*, API_VERSION};
    pub async fn get_properties(
        operation_config: &crate::OperationConfig,
        restype: &str,
        comp: &str,
        timeout: Option<i64>,
        x_ms_version: &str,
        x_ms_client_request_id: Option<&str>,
    ) -> std::result::Result<TableServiceProperties, get_properties::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/?ServiceProperties", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(get_properties::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get_properties::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("restype", restype);
        url.query_pairs_mut().append_pair("comp", comp);
        if let Some(timeout) = timeout {
            url.query_pairs_mut().append_pair("timeout", timeout.to_string().as_str());
        }
        req_builder = req_builder.header("x-ms-version", x_ms_version);
        if let Some(x_ms_client_request_id) = x_ms_client_request_id {
            req_builder = req_builder.header("x-ms-client-request-id", x_ms_client_request_id);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get_properties::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(get_properties::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: TableServiceProperties =
                    serde_json::from_slice(rsp_body).map_err(|source| get_properties::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: TableServiceError =
                    serde_json::from_slice(rsp_body).map_err(|source| get_properties::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get_properties::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get_properties {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::TableServiceError,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn set_properties(
        operation_config: &crate::OperationConfig,
        restype: &str,
        comp: &str,
        table_service_properties: &TableServiceProperties,
        timeout: Option<i64>,
        x_ms_version: &str,
        x_ms_client_request_id: Option<&str>,
    ) -> std::result::Result<(), set_properties::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/?ServiceProperties", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(set_properties::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(set_properties::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("restype", restype);
        url.query_pairs_mut().append_pair("comp", comp);
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(table_service_properties).map_err(set_properties::Error::SerializeError)?;
        if let Some(timeout) = timeout {
            url.query_pairs_mut().append_pair("timeout", timeout.to_string().as_str());
        }
        req_builder = req_builder.header("x-ms-version", x_ms_version);
        if let Some(x_ms_client_request_id) = x_ms_client_request_id {
            req_builder = req_builder.header("x-ms-client-request-id", x_ms_client_request_id);
        }
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(set_properties::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(set_properties::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::ACCEPTED => Ok(()),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: TableServiceError =
                    serde_json::from_slice(rsp_body).map_err(|source| set_properties::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(set_properties::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod set_properties {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::TableServiceError,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn get_statistics(
        operation_config: &crate::OperationConfig,
        restype: &str,
        comp: &str,
        timeout: Option<i64>,
        x_ms_version: &str,
        x_ms_client_request_id: Option<&str>,
    ) -> std::result::Result<TableServiceStats, get_statistics::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/?ServiceStats", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(get_statistics::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get_statistics::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("restype", restype);
        url.query_pairs_mut().append_pair("comp", comp);
        if let Some(timeout) = timeout {
            url.query_pairs_mut().append_pair("timeout", timeout.to_string().as_str());
        }
        req_builder = req_builder.header("x-ms-version", x_ms_version);
        if let Some(x_ms_client_request_id) = x_ms_client_request_id {
            req_builder = req_builder.header("x-ms-client-request-id", x_ms_client_request_id);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get_statistics::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(get_statistics::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: TableServiceStats =
                    serde_json::from_slice(rsp_body).map_err(|source| get_statistics::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: TableServiceError =
                    serde_json::from_slice(rsp_body).map_err(|source| get_statistics::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get_statistics::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get_statistics {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::TableServiceError,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
}
