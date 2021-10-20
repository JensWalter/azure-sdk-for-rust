#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::{models, models::*, API_VERSION};
pub async fn publish_cloud_event_events(
    operation_config: &crate::OperationConfig,
    events: &Vec<&CloudEventEvent>,
) -> std::result::Result<(), publish_cloud_event_events::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!("{}/api/events?overload=cloudEvent", operation_config.base_path(),);
    let mut url = url::Url::parse(url_str).map_err(publish_cloud_event_events::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(publish_cloud_event_events::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
    req_builder = req_builder.header("content-type", "application/json");
    let req_body = azure_core::to_json(events).map_err(publish_cloud_event_events::Error::SerializeError)?;
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder
        .body(req_body)
        .map_err(publish_cloud_event_events::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(publish_cloud_event_events::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => Ok(()),
        status_code => Err(publish_cloud_event_events::Error::DefaultResponse { status_code }),
    }
}
pub mod publish_cloud_event_events {
    use super::{models, models::*, API_VERSION};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse { status_code: http::StatusCode },
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
pub async fn publish_custom_event_events(
    operation_config: &crate::OperationConfig,
    events: &Vec<&CustomEventEvent>,
) -> std::result::Result<(), publish_custom_event_events::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!("{}/api/events?overload=customEvent", operation_config.base_path(),);
    let mut url = url::Url::parse(url_str).map_err(publish_custom_event_events::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(publish_custom_event_events::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
    req_builder = req_builder.header("content-type", "application/json");
    let req_body = azure_core::to_json(events).map_err(publish_custom_event_events::Error::SerializeError)?;
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder
        .body(req_body)
        .map_err(publish_custom_event_events::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(publish_custom_event_events::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => Ok(()),
        status_code => Err(publish_custom_event_events::Error::DefaultResponse { status_code }),
    }
}
pub mod publish_custom_event_events {
    use super::{models, models::*, API_VERSION};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse { status_code: http::StatusCode },
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
