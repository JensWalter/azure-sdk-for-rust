#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::{models, models::*};
pub mod dimensions {
    use super::{models, models::*};
    pub async fn list(
        operation_config: &crate::OperationConfig,
        scope: &str,
        filter: Option<&str>,
        expand: Option<&str>,
        skiptoken: Option<&str>,
        top: Option<i64>,
    ) -> std::result::Result<DimensionsListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.CostManagement/dimensions",
            operation_config.base_path(),
            scope
        );
        let mut url = url::Url::parse(url_str).map_err(list::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        if let Some(filter) = filter {
            url.query_pairs_mut().append_pair("$filter", filter);
        }
        if let Some(expand) = expand {
            url.query_pairs_mut().append_pair("$expand", expand);
        }
        if let Some(skiptoken) = skiptoken {
            url.query_pairs_mut().append_pair("$skiptoken", skiptoken);
        }
        if let Some(top) = top {
            url.query_pairs_mut().append_pair("$top", top.to_string().as_str());
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: DimensionsListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
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
pub mod query {
    use super::{models, models::*};
    pub async fn usage(
        operation_config: &crate::OperationConfig,
        scope: &str,
        parameters: &QueryDefinition,
    ) -> std::result::Result<QueryResult, usage::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.CostManagement/query",
            operation_config.base_path(),
            scope
        );
        let mut url = url::Url::parse(url_str).map_err(usage::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(usage::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(parameters).map_err(usage::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(usage::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(usage::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: QueryResult =
                    serde_json::from_slice(rsp_body).map_err(|source| usage::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| usage::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(usage::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod usage {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
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
pub mod exports {
    use super::{models, models::*};
    pub async fn list(operation_config: &crate::OperationConfig, scope: &str) -> std::result::Result<ExportListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.CostManagement/exports",
            operation_config.base_path(),
            scope
        );
        let mut url = url::Url::parse(url_str).map_err(list::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ExportListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
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
    pub async fn get(operation_config: &crate::OperationConfig, scope: &str, export_name: &str) -> std::result::Result<Export, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.CostManagement/exports/{}",
            operation_config.base_path(),
            scope,
            export_name
        );
        let mut url = url::Url::parse(url_str).map_err(get::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(get::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Export =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
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
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        scope: &str,
        export_name: &str,
        parameters: &Export,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.CostManagement/exports/{}",
            operation_config.base_path(),
            scope,
            export_name
        );
        let mut url = url::Url::parse(url_str).map_err(create_or_update::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(create_or_update::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(parameters).map_err(create_or_update::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(create_or_update::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(create_or_update::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Export = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: Export = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(create_or_update::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod create_or_update {
        use super::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200(Export),
            Created201(Export),
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
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
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        scope: &str,
        export_name: &str,
    ) -> std::result::Result<(), delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.CostManagement/exports/{}",
            operation_config.base_path(),
            scope,
            export_name
        );
        let mut url = url::Url::parse(url_str).map_err(delete::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(delete::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(delete::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(delete::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(()),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| delete::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(delete::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod delete {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
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
    pub async fn execute(
        operation_config: &crate::OperationConfig,
        scope: &str,
        export_name: &str,
    ) -> std::result::Result<(), execute::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.CostManagement/exports/{}/run",
            operation_config.base_path(),
            scope,
            export_name
        );
        let mut url = url::Url::parse(url_str).map_err(execute::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(execute::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(execute::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(execute::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(()),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| execute::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(execute::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod execute {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
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
    pub async fn get_execution_history(
        operation_config: &crate::OperationConfig,
        scope: &str,
        export_name: &str,
    ) -> std::result::Result<ExportExecutionListResult, get_execution_history::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.CostManagement/exports/{}/runHistory",
            operation_config.base_path(),
            scope,
            export_name
        );
        let mut url = url::Url::parse(url_str).map_err(get_execution_history::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get_execution_history::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(get_execution_history::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(get_execution_history::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ExportExecutionListResult = serde_json::from_slice(rsp_body)
                    .map_err(|source| get_execution_history::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse = serde_json::from_slice(rsp_body)
                    .map_err(|source| get_execution_history::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get_execution_history::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get_execution_history {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
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
pub mod operations {
    use super::{models, models::*};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/Microsoft.CostManagement/operations", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(list::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: OperationListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
        use super::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
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
