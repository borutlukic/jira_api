//! Generated HTTP client for regular API requests
//!
//! This file contains the HTTP client implementation for GET, POST, etc.
//! Do not edit manually - regenerate using the appropriate script.
#![allow(clippy::format_in_format_args)]
#![allow(clippy::let_unit_value)]
use super::types::*;
use thiserror::Error;
/// HTTP client errors that can occur during API requests
#[derive(Error, Debug)]
pub enum HttpError {
    /// Network or connection error (from reqwest)
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    /// Middleware error (from reqwest-middleware)
    #[error("Middleware error: {0}")]
    Middleware(#[from] reqwest_middleware::Error),
    /// Request serialization error
    #[error("Failed to serialize request: {0}")]
    Serialization(String),
    /// Response deserialization error
    #[error("Failed to deserialize response: {0}")]
    Deserialization(String),
    /// HTTP error response (4xx, 5xx)
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String, body: Option<String> },
    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),
    /// Request timeout
    #[error("Request timeout")]
    Timeout,
    /// Invalid configuration
    #[error("Configuration error: {0}")]
    Config(String),
    /// Generic error
    #[error("{0}")]
    Other(String),
}
impl HttpError {
    /// Create an HTTP error from a status code and message
    pub fn from_status(
        status: u16,
        message: impl Into<String>,
        body: Option<String>,
    ) -> Self {
        Self::Http {
            status,
            message: message.into(),
            body,
        }
    }
    /// Create a serialization error
    pub fn serialization_error(error: impl std::fmt::Display) -> Self {
        Self::Serialization(error.to_string())
    }
    /// Create a deserialization error
    pub fn deserialization_error(error: impl std::fmt::Display) -> Self {
        Self::Deserialization(error.to_string())
    }
    /// Check if this is a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        matches!(self, Self::Http { status, .. } if * status >= 400 && * status < 500)
    }
    /// Check if this is a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        matches!(self, Self::Http { status, .. } if * status >= 500 && * status < 600)
    }
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Network(_) => true,
            Self::Middleware(_) => true,
            Self::Timeout => true,
            Self::Http { status, .. } => matches!(status, 429 | 500 | 502 | 503 | 504),
            _ => false,
        }
    }
}
/// Result type for HTTP operations
pub type HttpResult<T> = Result<T, HttpError>;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::collections::BTreeMap;
/// HTTP client for making API requests
#[derive(Clone)]
pub struct HttpClient {
    base_url: String,
    api_key: Option<String>,
    http_client: ClientWithMiddleware,
    custom_headers: BTreeMap<String, String>,
}
impl HttpClient {
    /// Create a new HTTP client with default configuration
    pub fn new() -> Self {
        Self::with_config(true)
    }
    /// Create a new HTTP client with custom configuration
    pub fn with_config(enable_tracing: bool) -> Self {
        let reqwest_client = reqwest::Client::new();
        let mut client_builder = ClientBuilder::new(reqwest_client);
        if enable_tracing {
            use reqwest_tracing::TracingMiddleware;
            client_builder = client_builder.with(TracingMiddleware::default());
        }
        let http_client = client_builder.build();
        Self {
            base_url: String::new(),
            api_key: None,
            http_client,
            custom_headers: BTreeMap::new(),
        }
    }
    /// Set the base URL for all requests
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
    /// Set the API key for authentication
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
    /// Add a custom header to all requests
    pub fn with_header(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.custom_headers.insert(name.into(), value.into());
        self
    }
    /// Add multiple custom headers
    pub fn with_headers(mut self, headers: BTreeMap<String, String>) -> Self {
        self.custom_headers.extend(headers);
        self
    }
}
impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
impl HttpClient {
    ///POST /api/2/cluster/zdu/retryUpgrade
    pub async fn acknowledge_errors(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/cluster/zdu/retryUpgrade");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/project/{projectIdOrKey}/role/{id}
    pub async fn add_actor_users(
        &self,
        project_id_or_key: impl AsRef<str>,
        id: i64,
        request: ActorsMap,
    ) -> HttpResult<ProjectRoleBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/role/{}", project_id_or_key
            .as_ref(), id)
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/{issueIdOrKey}/attachments
    pub async fn add_attachment(
        &self,
        issue_id_or_key: impl AsRef<str>,
        form: reqwest::multipart::Form,
    ) -> HttpResult<AttachmentJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/attachments", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.post(url).multipart(form);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/{issueIdOrKey}/comment
    pub async fn add_comment(
        &self,
        issue_id_or_key: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
        request: CommentJsonBean,
    ) -> HttpResult<CommentJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/comment", issue_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/screens/{screenId}/tabs/{tabId}/fields
    pub async fn add_field(
        &self,
        tab_id: i64,
        screen_id: i64,
        request: AddFieldBean,
    ) -> HttpResult<ScreenableFieldBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/screens/{}/tabs/{}/fields", tab_id,
            screen_id)
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/screens/addToDefault/{fieldId}
    pub async fn add_field_to_default_screen(
        &self,
        field_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/screens/addToDefault/{}", field_id
            .as_ref())
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issuetypescheme/{schemeId}/associations
    pub async fn add_project_associations_to_scheme(
        &self,
        scheme_id: impl AsRef<str>,
        request: AssociateProjectsBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetypescheme/{}/associations",
            scheme_id.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/role/{id}/actors
    pub async fn add_project_role_actors_to_role(
        &self,
        id: i64,
        request: ActorInputBean,
    ) -> HttpResult<ProjectRoleActorsBean> {
        let url = format!("{}{}", self.base_url, format!("/api/2/role/{}/actors", id));
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/filter/{id}/permission
    pub async fn add_share_permission(
        &self,
        id: impl AsRef<str>,
        request: SharePermissionInputBean,
    ) -> HttpResult<FilterPermissionBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/filter/{}/permission", id.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/screens/{screenId}/tabs
    pub async fn add_tab(
        &self,
        screen_id: i64,
        request: ScreenableTabBean,
    ) -> HttpResult<ScreenableTabBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/screens/{}/tabs", screen_id)
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/user/application
    pub async fn add_user_to_application_1(
        &self,
        application_key: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/application");
        let mut req = self.http_client.post(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = application_key {
                query_params.push(("applicationKey", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/group/user
    pub async fn add_user_to_group(
        &self,
        groupname: impl AsRef<str>,
        request: UpdateUserToGroupBean,
    ) -> HttpResult<GroupBean> {
        let url = format!("{}{}", self.base_url, "/api/2/group/user");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params.push(("groupname", groupname.as_ref().to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/{issueIdOrKey}/votes
    pub async fn add_vote(&self, issue_id_or_key: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/votes", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/{issueIdOrKey}/watchers
    pub async fn add_watcher_1(
        &self,
        issue_id_or_key: impl AsRef<str>,
        user_name: Option<impl AsRef<str>>,
        request: AddWatcher1Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/watchers", issue_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = user_name {
                query_params.push(("userName", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/{issueIdOrKey}/worklog
    pub async fn add_worklog(
        &self,
        issue_id_or_key: impl AsRef<str>,
        new_estimate: Option<impl AsRef<str>>,
        adjust_estimate: Option<impl AsRef<str>>,
        reduce_by: Option<impl AsRef<str>>,
        request: Worklog,
    ) -> HttpResult<Worklog> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/worklog", issue_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = new_estimate {
                query_params.push(("newEstimate", v.as_ref().to_string()));
            }
            if let Some(v) = adjust_estimate {
                query_params.push(("adjustEstimate", v.as_ref().to_string()));
            }
            if let Some(v) = reduce_by {
                query_params.push(("reduceBy", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/email-templates/apply
    pub async fn apply_email_templates(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/email-templates/apply");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/cluster/zdu/approve
    pub async fn approve_upgrade(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/cluster/zdu/approve");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issue/{issueIdOrKey}/archive
    pub async fn archive_issue(
        &self,
        issue_id_or_key: impl AsRef<str>,
        notify_users: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/archive", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.put(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = notify_users {
                query_params.push(("notifyUsers", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/archive
    pub async fn archive_issues(
        &self,
        notify_users: Option<impl AsRef<str>>,
        body: String,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/issue/archive");
        let mut req = self
            .http_client
            .post(url)
            .body(body)
            .header("content-type", "text/plain");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = notify_users {
                query_params.push(("notifyUsers", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/project/{projectIdOrKey}/archive
    pub async fn archive_project(
        &self,
        project_id_or_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/archive", project_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/monitoring/jmx/areMetricsExposed
    pub async fn are_metrics_exposed(&self) -> HttpResult<AreMetricsExposedResponse> {
        let url = format!(
            "{}{}", self.base_url, "/api/2/monitoring/jmx/areMetricsExposed"
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issue/{issueIdOrKey}/assignee
    pub async fn assign(
        &self,
        issue_id_or_key: impl AsRef<str>,
        request: UserBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/assignee", issue_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/project/{projectKeyOrId}/permissionscheme
    pub async fn assign_permission_scheme(
        &self,
        project_key_or_id: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
        request: IdBean,
    ) -> HttpResult<PermissionSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/permissionscheme",
            project_key_or_id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/project/{projectKeyOrId}/priorityscheme
    pub async fn assign_priority_scheme(
        &self,
        project_key_or_id: impl AsRef<str>,
        request: IdBean,
    ) -> HttpResult<PrioritySchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/priorityscheme",
            project_key_or_id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/customFields
    pub async fn bulk_delete_custom_fields(
        &self,
        ids: impl AsRef<str>,
    ) -> HttpResult<BulkDeleteResponseBean> {
        let url = format!("{}{}", self.base_url, "/api/2/customFields");
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params.push(("ids", ids.as_ref().to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/subtask/move
    pub async fn can_move_sub_task(
        &self,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<CanMoveSubTaskResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/subtask/move",
            issue_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/cluster/zdu/cancel
    pub async fn cancel_upgrade(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/cluster/zdu/cancel");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/myself/password
    pub async fn change_my_password(&self, request: PasswordBean) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/myself/password");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/cluster/node/{nodeId}/offline
    pub async fn change_node_state_to_offline(
        &self,
        node_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/cluster/node/{}/offline", node_id
            .as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/user/password
    pub async fn change_user_password(
        &self,
        key: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
        request: PasswordBean,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/password");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = key {
                query_params.push(("key", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issuetype/{id}/avatar
    pub async fn create_avatar_from_temporary(
        &self,
        id: impl AsRef<str>,
        request: AvatarCroppingBean,
    ) -> HttpResult<AvatarBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetype/{}/avatar", id.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/project/{projectIdOrKey}/avatar
    pub async fn create_avatar_from_temporary_1(
        &self,
        project_id_or_key: impl AsRef<str>,
        request: AvatarCroppingBean,
    ) -> HttpResult<AvatarBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/avatar", project_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/universal_avatar/type/{type}/owner/{owningObjectId}/avatar
    pub async fn create_avatar_from_temporary_2(
        &self,
        type_: impl AsRef<str>,
        owning_object_id: impl AsRef<str>,
        request: AvatarCroppingBean,
    ) -> HttpResult<AvatarBean> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/2/universal_avatar/type/{}/owner/{}/avatar", type_.as_ref(),
            owning_object_id.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/user/avatar
    pub async fn create_avatar_from_temporary_3(
        &self,
        username: Option<impl AsRef<str>>,
        request: AvatarCroppingBean,
    ) -> HttpResult<AvatarBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/avatar");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /agile/1.0/board
    pub async fn create_board(&self, request: BoardCreateBean) -> HttpResult<BoardBean> {
        let url = format!("{}{}", self.base_url, "/agile/1.0/board");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/component
    pub async fn create_component(
        &self,
        request: ComponentBean,
    ) -> HttpResult<ComponentBean> {
        let url = format!("{}{}", self.base_url, "/api/2/component");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/field
    pub async fn create_custom_field(
        &self,
        request: CustomFieldDefinitionJsonBean,
    ) -> HttpResult<FieldBean> {
        let url = format!("{}{}", self.base_url, "/api/2/field");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/workflowscheme/{id}/createdraft
    pub async fn create_draft_for_parent(
        &self,
        id: i64,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/createdraft", id)
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/filter
    pub async fn create_filter(
        &self,
        expand: Option<impl AsRef<str>>,
        request: FilterBean,
    ) -> HttpResult<FilterBean> {
        let url = format!("{}{}", self.base_url, "/api/2/filter");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/group
    pub async fn create_group(&self, request: AddGroupBean) -> HttpResult<GroupBean> {
        let url = format!("{}{}", self.base_url, "/api/2/group");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/index-snapshot
    pub async fn create_index_snapshot(&self) -> HttpResult<IndexSnapshotPromiseBean> {
        let url = format!("{}{}", self.base_url, "/api/2/index-snapshot");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue
    pub async fn create_issue(
        &self,
        update_history: Option<bool>,
        request: IssueUpdateBean,
    ) -> HttpResult<IssueCreateResponse> {
        let url = format!("{}{}", self.base_url, "/api/2/issue");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = update_history {
                query_params.push(("updateHistory", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issueLinkType
    pub async fn create_issue_link_type(
        &self,
        request: IssueLinkTypeJsonBean,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/issueLinkType");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issuetype
    pub async fn create_issue_type(
        &self,
        request: IssueTypeCreateBean,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/issuetype");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issuetypescheme
    pub async fn create_issue_type_scheme(
        &self,
        request: IssueTypeSchemeCreateUpdateBean,
    ) -> HttpResult<IssueTypeSchemeBean> {
        let url = format!("{}{}", self.base_url, "/api/2/issuetypescheme");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/bulk
    pub async fn create_issues(
        &self,
        request: IssuesUpdateBean,
    ) -> HttpResult<IssuesCreateResponse> {
        let url = format!("{}{}", self.base_url, "/api/2/issue/bulk");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/{issueIdOrKey}/remotelink
    pub async fn create_or_update_remote_issue_link(
        &self,
        issue_id_or_key: impl AsRef<str>,
        request: RemoteIssueLinkCreateOrUpdateRequest,
    ) -> HttpResult<RemoteIssueLinkBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/remotelink", issue_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/version/{versionId}/remotelink
    pub async fn create_or_update_remote_version_link(
        &self,
        version_id: impl AsRef<str>,
        request: RemoteEntityLinkJsonBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}/remotelink", version_id
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/version/{versionId}/remotelink/{globalId}
    pub async fn create_or_update_remote_version_link_1(
        &self,
        version_id: impl AsRef<str>,
        global_id: impl AsRef<str>,
        request: RemoteEntityLinkJsonBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}/remotelink/{}", version_id
            .as_ref(), global_id.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/permissionscheme/{schemeId}/permission
    pub async fn create_permission_grant(
        &self,
        scheme_id: i64,
        expand: Option<impl AsRef<str>>,
        request: PermissionGrantBean,
    ) -> HttpResult<PermissionGrantBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/permissionscheme/{}/permission",
            scheme_id)
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/permissionscheme
    pub async fn create_permission_scheme(
        &self,
        expand: Option<impl AsRef<str>>,
        request: PermissionSchemeBean,
    ) -> HttpResult<PermissionSchemeBean> {
        let url = format!("{}{}", self.base_url, "/api/2/permissionscheme");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/priorityschemes
    pub async fn create_priority_scheme(
        &self,
        request: PrioritySchemeUpdateBean,
    ) -> HttpResult<PrioritySchemeBean> {
        let url = format!("{}{}", self.base_url, "/api/2/priorityschemes");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/project
    pub async fn create_project(
        &self,
        request: ProjectInputBean,
    ) -> HttpResult<ProjectIdentity> {
        let url = format!("{}{}", self.base_url, "/api/2/project");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/projectCategory
    pub async fn create_project_category(
        &self,
        request: ProjectCategoryBean,
    ) -> HttpResult<ProjectCategoryJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/projectCategory");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/role
    pub async fn create_project_role(
        &self,
        request: CreateUpdateRoleRequestBean,
    ) -> HttpResult<ProjectRoleBean> {
        let url = format!("{}{}", self.base_url, "/api/2/role");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/remotelink/reciprocal
    pub async fn create_reciprocal_remote_issue_link(
        &self,
        request: RemoteReciprocalIssueLinkCreateRequest,
    ) -> HttpResult<RemoteReciprocalIssueLinkCreateResponse> {
        let url = format!("{}{}", self.base_url, "/api/2/issue/remotelink/reciprocal");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/workflowscheme
    pub async fn create_scheme(&self, request: WorkflowSchemeBean) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/workflowscheme");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /agile/1.0/sprint
    pub async fn create_sprint(
        &self,
        request: SprintCreateBean,
    ) -> HttpResult<SprintBean> {
        let url = format!("{}{}", self.base_url, "/agile/1.0/sprint");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/user
    pub async fn create_user(
        &self,
        request: UserWriteBean,
    ) -> HttpResult<UserWriteBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/version
    pub async fn create_version(&self, request: VersionBean) -> HttpResult<VersionBean> {
        let url = format!("{}{}", self.base_url, "/api/2/version");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /auth/1/session
    pub async fn current_user(&self) -> HttpResult<CurrentUser> {
        let url = format!("{}{}", self.base_url, "/auth/1/session");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/columns
    pub async fn default_columns(
        &self,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<ColumnOptions> {
        let url = format!("{}{}", self.base_url, "/api/2/user/columns");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/filter/{id}/columns
    pub async fn default_columns_1(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<ColumnLayout> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/filter/{}/columns", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/component/{id}
    pub async fn delete(
        &self,
        id: impl AsRef<str>,
        move_issues_to: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/component/{}", id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = move_issues_to {
                query_params.push(("moveIssuesTo", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/project/{projectIdOrKey}/role/{id}
    pub async fn delete_actor(
        &self,
        project_id_or_key: impl AsRef<str>,
        id: i64,
        user: Option<impl AsRef<str>>,
        group: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/role/{}", project_id_or_key
            .as_ref(), id)
        );
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = user {
                query_params.push(("user", v.as_ref().to_string()));
            }
            if let Some(v) = group {
                query_params.push(("group", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/project/{projectIdOrKey}/avatar/{id}
    pub async fn delete_avatar(
        &self,
        project_id_or_key: impl AsRef<str>,
        id: i64,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/avatar/{}",
            project_id_or_key.as_ref(), id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/universal_avatar/type/{type}/owner/{owningObjectId}/avatar/{id}
    pub async fn delete_avatar_1(
        &self,
        id: i64,
        type_: impl AsRef<str>,
        owning_object_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/2/universal_avatar/type/{}/owner/{}/avatar/{}", id, type_
            .as_ref(), owning_object_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/user/avatar/{id}
    pub async fn delete_avatar_2(
        &self,
        id: i64,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, format!("/api/2/user/avatar/{}", id));
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /agile/1.0/board/{boardId}
    pub async fn delete_board(&self, board_id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}", board_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issue/{issueIdOrKey}/comment/{id}
    pub async fn delete_comment(
        &self,
        issue_id_or_key: impl AsRef<str>,
        id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/comment/{}", issue_id_or_key
            .as_ref(), id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/workflowscheme/{id}/default
    pub async fn delete_default(
        &self,
        id: i64,
        update_draft_if_needed: Option<bool>,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/default", id)
        );
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = update_draft_if_needed {
                query_params.push(("updateDraftIfNeeded", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/workflowscheme/{id}/draft
    pub async fn delete_draft_by_id(&self, id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft", id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/workflowscheme/{id}/draft/default
    pub async fn delete_draft_default(&self, id: i64) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft/default", id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/workflowscheme/{id}/draft/issuetype/{issueType}
    pub async fn delete_draft_issue_type(
        &self,
        issue_type: impl AsRef<str>,
        id: i64,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft/issuetype/{}",
            issue_type.as_ref(), id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/workflowscheme/{id}/draft/workflow
    pub async fn delete_draft_workflow_mapping(
        &self,
        id: i64,
        workflow_name: Option<impl AsRef<str>>,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft/workflow", id)
        );
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = workflow_name {
                query_params.push(("workflowName", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/filter/{id}
    pub async fn delete_filter(&self, id: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/filter/{}", id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issue/{issueIdOrKey}
    pub async fn delete_issue(
        &self,
        issue_id_or_key: impl AsRef<str>,
        delete_subtasks: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}", issue_id_or_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = delete_subtasks {
                query_params.push(("deleteSubtasks", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issueLink/{linkId}
    pub async fn delete_issue_link(&self, link_id: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issueLink/{}", link_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issueLinkType/{issueLinkTypeId}
    pub async fn delete_issue_link_type(
        &self,
        issue_link_type_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issueLinkType/{}", issue_link_type_id
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/workflowscheme/{id}/issuetype/{issueType}
    pub async fn delete_issue_type(
        &self,
        issue_type: impl AsRef<str>,
        id: i64,
        update_draft_if_needed: Option<bool>,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/issuetype/{}",
            issue_type.as_ref(), id)
        );
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = update_draft_if_needed {
                query_params.push(("updateDraftIfNeeded", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issuetypescheme/{schemeId}
    pub async fn delete_issue_type_scheme(
        &self,
        scheme_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetypescheme/{}", scheme_id
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issuetype/{id}
    pub async fn delete_issue_type_1(&self, id: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetype/{}", id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/cluster/node/{nodeId}
    pub async fn delete_node(&self, node_id: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/cluster/node/{}", node_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/permissionscheme/{schemeId}
    pub async fn delete_permission_scheme(&self, scheme_id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/permissionscheme/{}", scheme_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/permissionscheme/{schemeId}/permission/{permissionId}
    pub async fn delete_permission_scheme_entity(
        &self,
        permission_id: i64,
        scheme_id: i64,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/permissionscheme/{}/permission/{}",
            permission_id, scheme_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/priorityschemes/{schemeId}
    pub async fn delete_priority_scheme(&self, scheme_id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/priorityschemes/{}", scheme_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/project/{projectIdOrKey}
    pub async fn delete_project(
        &self,
        project_id_or_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}", project_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/role/{id}
    pub async fn delete_project_role(
        &self,
        id: i64,
        swap: Option<i64>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, format!("/api/2/role/{}", id));
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = swap {
                query_params.push(("swap", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/role/{id}/actors
    pub async fn delete_project_role_actors_from_role(
        &self,
        id: i64,
        user: Option<impl AsRef<str>>,
        group: Option<impl AsRef<str>>,
    ) -> HttpResult<ProjectRoleActorsBean> {
        let url = format!("{}{}", self.base_url, format!("/api/2/role/{}/actors", id));
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = user {
                query_params.push(("user", v.as_ref().to_string()));
            }
            if let Some(v) = group {
                query_params.push(("group", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /agile/1.0/board/{boardId}/properties/{propertyKey}
    pub async fn delete_property(
        &self,
        property_key: impl AsRef<str>,
        board_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}/properties/{}",
            property_key.as_ref(), board_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/dashboard/{dashboardId}/items/{itemId}/properties/{propertyKey}
    pub async fn delete_property_1(
        &self,
        property_key: impl AsRef<str>,
        item_id: impl AsRef<str>,
        dashboard_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/dashboard/{}/items/{}/properties/{}",
            property_key.as_ref(), item_id.as_ref(), dashboard_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/comment/{commentId}/properties/{propertyKey}
    pub async fn delete_property_2(
        &self,
        property_key: impl AsRef<str>,
        comment_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/comment/{}/properties/{}",
            property_key.as_ref(), comment_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issue/{issueIdOrKey}/properties/{propertyKey}
    pub async fn delete_property_3(
        &self,
        property_key: impl AsRef<str>,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/properties/{}", property_key
            .as_ref(), issue_id_or_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issuetype/{issueTypeId}/properties/{propertyKey}
    pub async fn delete_property_4(
        &self,
        property_key: impl AsRef<str>,
        issue_type_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetype/{}/properties/{}",
            property_key.as_ref(), issue_type_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/project/{projectIdOrKey}/properties/{propertyKey}
    pub async fn delete_property_5(
        &self,
        property_key: impl AsRef<str>,
        project_id_or_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/properties/{}",
            property_key.as_ref(), project_id_or_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/user/properties/{propertyKey}
    pub async fn delete_property_6(
        &self,
        property_key: impl AsRef<str>,
        user_key: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/user/properties/{}", property_key
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = user_key {
                query_params.push(("userKey", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issue/{issueIdOrKey}/remotelink
    pub async fn delete_remote_issue_link_by_global_id(
        &self,
        issue_id_or_key: impl AsRef<str>,
        global_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/remotelink", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params.push(("globalId", global_id.as_ref().to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issue/{issueIdOrKey}/remotelink/{linkId}
    pub async fn delete_remote_issue_link_by_id(
        &self,
        link_id: impl AsRef<str>,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/remotelink/{}", link_id
            .as_ref(), issue_id_or_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/version/{versionId}/remotelink/{globalId}
    pub async fn delete_remote_version_link(
        &self,
        version_id: impl AsRef<str>,
        global_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}/remotelink/{}", version_id
            .as_ref(), global_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/version/{versionId}/remotelink
    pub async fn delete_remote_version_links_by_version_id(
        &self,
        version_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}/remotelink", version_id
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/workflowscheme/{id}
    pub async fn delete_scheme(&self, id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}", id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/user/session/{username}
    pub async fn delete_session(&self, username: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/user/session/{}", username.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/filter/{id}/permission/{permission-id}
    pub async fn delete_share_permission(
        &self,
        id: impl AsRef<str>,
        permission_id: i64,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/filter/{}/permission/{}", id.as_ref(),
            permission_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /agile/1.0/sprint/{sprintId}
    pub async fn delete_sprint(&self, sprint_id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/sprint/{}", sprint_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/screens/{screenId}/tabs/{tabId}
    pub async fn delete_tab(&self, tab_id: i64, screen_id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/screens/{}/tabs/{}", tab_id,
            screen_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/workflowscheme/{id}/workflow
    pub async fn delete_workflow_mapping(
        &self,
        id: i64,
        update_draft_if_needed: Option<bool>,
        workflow_name: Option<impl AsRef<str>>,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/workflow", id)
        );
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = update_draft_if_needed {
                query_params.push(("updateDraftIfNeeded", v.to_string()));
            }
            if let Some(v) = workflow_name {
                query_params.push(("workflowName", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issue/{issueIdOrKey}/worklog/{id}
    pub async fn delete_worklog(
        &self,
        issue_id_or_key: impl AsRef<str>,
        id: impl AsRef<str>,
        new_estimate: Option<impl AsRef<str>>,
        adjust_estimate: Option<impl AsRef<str>>,
        increase_by: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/worklog/{}", issue_id_or_key
            .as_ref(), id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = new_estimate {
                query_params.push(("newEstimate", v.as_ref().to_string()));
            }
            if let Some(v) = adjust_estimate {
                query_params.push(("adjustEstimate", v.as_ref().to_string()));
            }
            if let Some(v) = increase_by {
                query_params.push(("increaseBy", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/version/{id}/removeAndSwap
    pub async fn delete_1(
        &self,
        id: impl AsRef<str>,
        request: DeleteAndReplaceVersionBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}/removeAndSwap", id
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/{issueIdOrKey}/transitions
    pub async fn do_transition(
        &self,
        issue_id_or_key: impl AsRef<str>,
        request: IssueUpdateBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/transitions", issue_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/email-templates
    pub async fn download_email_templates(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/email-templates");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/filter/{id}
    pub async fn edit_filter(
        &self,
        id: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
        request: FilterBean,
    ) -> HttpResult<FilterBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/filter/{}", id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issue/{issueIdOrKey}
    pub async fn edit_issue(
        &self,
        issue_id_or_key: impl AsRef<str>,
        notify_users: Option<impl AsRef<str>>,
        request: IssueUpdateBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}", issue_id_or_key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = notify_users {
                query_params.push(("notifyUsers", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /agile/1.0/issue/{issueIdOrKey}/estimation
    pub async fn estimate_issue_for_board(
        &self,
        issue_id_or_key: impl AsRef<str>,
        board_id: Option<i64>,
        request: FieldEditBean,
    ) -> HttpResult<FieldValueBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/issue/{}/estimation",
            issue_id_or_key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = board_id {
                query_params.push(("boardId", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/attachment/{id}/expand/human
    pub async fn expand_for_humans(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<HumanReadableArchive> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/attachment/{}/expand/human", id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/attachment/{id}/expand/raw
    pub async fn expand_for_machines(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<AttachmentArchiveImpl> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/attachment/{}/expand/raw", id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/assignable/search
    pub async fn find_assignable_users_1(
        &self,
        issue_key: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        project: Option<impl AsRef<str>>,
        action_descriptor_id: Option<i64>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<UserBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/assignable/search");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = issue_key {
                query_params.push(("issueKey", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = project {
                query_params.push(("project", v.as_ref().to_string()));
            }
            if let Some(v) = action_descriptor_id {
                query_params.push(("actionDescriptorId", v.to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/assignable/multiProjectSearch
    pub async fn find_bulk_assignable_users(
        &self,
        max_results: Option<i64>,
        project_keys: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<UserBean> {
        let url = format!(
            "{}{}", self.base_url, "/api/2/user/assignable/multiProjectSearch"
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = project_keys {
                query_params.push(("projectKeys", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/groups/picker
    pub async fn find_groups(
        &self,
        max_results: Option<impl AsRef<str>>,
        query: Option<impl AsRef<str>>,
        exclude: Option<impl AsRef<str>>,
        user_name: Option<impl AsRef<str>>,
    ) -> HttpResult<GroupSuggestionsBean> {
        let url = format!("{}{}", self.base_url, "/api/2/groups/picker");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.as_ref().to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = exclude {
                query_params.push(("exclude", v.as_ref().to_string()));
            }
            if let Some(v) = user_name {
                query_params.push(("userName", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/search
    pub async fn find_users(
        &self,
        include_inactive: Option<bool>,
        max_results: Option<i64>,
        include_active: Option<bool>,
        start_at: Option<i64>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<UserBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/search");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = include_inactive {
                query_params.push(("includeInactive", v.to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = include_active {
                query_params.push(("includeActive", v.to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/groupuserpicker
    pub async fn find_users_and_groups(
        &self,
        issue_type_id: Option<impl AsRef<str>>,
        max_results: Option<impl AsRef<str>>,
        query: Option<impl AsRef<str>>,
        show_avatar: Option<impl AsRef<str>>,
        project_id: Option<impl AsRef<str>>,
        field_id: Option<impl AsRef<str>>,
    ) -> HttpResult<UsersAndGroupsBean> {
        let url = format!("{}{}", self.base_url, "/api/2/groupuserpicker");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = issue_type_id {
                query_params.push(("issueTypeId", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.as_ref().to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = show_avatar {
                query_params.push(("showAvatar", v.as_ref().to_string()));
            }
            if let Some(v) = project_id {
                query_params.push(("projectId", v.as_ref().to_string()));
            }
            if let Some(v) = field_id {
                query_params.push(("fieldId", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/picker
    pub async fn find_users_for_picker(
        &self,
        max_results: Option<i64>,
        query: Option<impl AsRef<str>>,
        exclude: Option<impl AsRef<str>>,
        show_avatar: Option<bool>,
    ) -> HttpResult<UserPickerResultsBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/picker");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = exclude {
                query_params.push(("exclude", v.as_ref().to_string()));
            }
            if let Some(v) = show_avatar {
                query_params.push(("showAvatar", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/permission/search
    pub async fn find_users_with_all_permissions(
        &self,
        project_key: Option<impl AsRef<str>>,
        issue_key: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        permissions: Option<impl AsRef<str>>,
        start_at: Option<i64>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<UserBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/permission/search");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = project_key {
                query_params.push(("projectKey", v.as_ref().to_string()));
            }
            if let Some(v) = issue_key {
                query_params.push(("issueKey", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = permissions {
                query_params.push(("permissions", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/viewissue/search
    pub async fn find_users_with_browse_permission(
        &self,
        project_key: Option<impl AsRef<str>>,
        issue_key: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<UserBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/viewissue/search");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = project_key {
                query_params.push(("projectKey", v.as_ref().to_string()));
            }
            if let Some(v) = issue_key {
                query_params.push(("issueKey", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/role/{id}
    pub async fn fully_update_project_role(
        &self,
        id: i64,
        request: CreateUpdateRoleRequestBean,
    ) -> HttpResult<ProjectRoleBean> {
        let url = format!("{}{}", self.base_url, format!("/api/2/role/{}", id));
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/a11y/personal-settings
    pub async fn get_a11y_personal_settings(
        &self,
    ) -> HttpResult<A11yPersonalSettingBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/a11y/personal-settings");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/type/{projectTypeKey}/accessible
    pub async fn get_accessible_project_type_by_key(
        &self,
        project_type_key: impl AsRef<str>,
    ) -> HttpResult<ProjectTypeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/type/{}/accessible",
            project_type_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/application-properties/advanced-settings
    pub async fn get_advanced_settings(&self) -> HttpResult<Property> {
        let url = format!(
            "{}{}", self.base_url, "/api/2/application-properties/advanced-settings"
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/applicationrole
    pub async fn get_all(&self) -> HttpResult<ApplicationRoleBean> {
        let url = format!("{}{}", self.base_url, "/api/2/applicationrole");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectIdOrKey}/avatars
    pub async fn get_all_avatars(
        &self,
        project_id_or_key: impl AsRef<str>,
    ) -> HttpResult<AvatarBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/avatars", project_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/avatars
    pub async fn get_all_avatars_1(
        &self,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<AvatarBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/avatars");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board
    pub async fn get_all_boards(
        &self,
        max_results: Option<i64>,
        name: Option<impl AsRef<str>>,
        project_key_or_id: Option<impl AsRef<str>>,
        type_: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<BoardBean> {
        let url = format!("{}{}", self.base_url, "/agile/1.0/board");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if let Some(v) = project_key_or_id {
                query_params.push(("projectKeyOrId", v.as_ref().to_string()));
            }
            if let Some(v) = type_ {
                query_params.push(("type", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/screens/{screenId}/tabs/{tabId}/fields
    pub async fn get_all_fields(
        &self,
        tab_id: i64,
        screen_id: i64,
        project_key: Option<impl AsRef<str>>,
    ) -> HttpResult<ScreenableFieldBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/screens/{}/tabs/{}/fields", tab_id,
            screen_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = project_key {
                query_params.push(("projectKey", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issuetypescheme
    pub async fn get_all_issue_type_schemes(
        &self,
    ) -> HttpResult<IssueTypeSchemeListBean> {
        let url = format!("{}{}", self.base_url, "/api/2/issuetypescheme");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/cluster/nodes
    pub async fn get_all_nodes(&self) -> HttpResult<NodeBean> {
        let url = format!("{}{}", self.base_url, "/api/2/cluster/nodes");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/permissions
    pub async fn get_all_permissions(&self) -> HttpResult<PermissionsJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/permissions");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/projectCategory
    pub async fn get_all_project_categories(
        &self,
    ) -> HttpResult<ProjectCategoryJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/projectCategory");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/type
    pub async fn get_all_project_types(&self) -> HttpResult<ProjectTypeBean> {
        let url = format!("{}{}", self.base_url, "/api/2/project/type");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project
    pub async fn get_all_projects(
        &self,
        include_archived: Option<bool>,
        expand: Option<impl AsRef<str>>,
        recent: Option<i64>,
        browse_archive: Option<bool>,
    ) -> HttpResult<ProjectBean> {
        let url = format!("{}{}", self.base_url, "/api/2/project");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = include_archived {
                query_params.push(("includeArchived", v.to_string()));
            }
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = recent {
                query_params.push(("recent", v.to_string()));
            }
            if let Some(v) = browse_archive {
                query_params.push(("browseArchive", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/screens
    pub async fn get_all_screens(
        &self,
        search: Option<impl AsRef<str>>,
        expand: Option<impl AsRef<str>>,
        max_results: Option<impl AsRef<str>>,
        start_at: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/screens");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = search {
                query_params.push(("search", v.as_ref().to_string()));
            }
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}/sprint
    pub async fn get_all_sprints(
        &self,
        board_id: i64,
        max_results: Option<i64>,
        state: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<SprintBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}/sprint", board_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = state {
                query_params.push(("state", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectIdOrKey}/statuses
    pub async fn get_all_statuses(
        &self,
        project_id_or_key: impl AsRef<str>,
    ) -> HttpResult<IssueTypeWithStatusJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/statuses",
            project_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/avatar/{type}/system
    pub async fn get_all_system_avatars(
        &self,
        type_: impl AsRef<str>,
    ) -> HttpResult<AvatarBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/avatar/{}/system", type_.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/screens/{screenId}/tabs
    pub async fn get_all_tabs(
        &self,
        screen_id: i64,
        project_key: Option<impl AsRef<str>>,
    ) -> HttpResult<ScreenableTabBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/screens/{}/tabs", screen_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = project_key {
                query_params.push(("projectKey", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/terminology/entries
    pub async fn get_all_terminology_entries(
        &self,
    ) -> HttpResult<TerminologyResponseBean> {
        let url = format!("{}{}", self.base_url, "/api/2/terminology/entries");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}/version
    pub async fn get_all_versions(
        &self,
        board_id: i64,
        max_results: Option<i64>,
        released: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<VersionBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}/version", board_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = released {
                query_params.push(("released", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/workflow
    pub async fn get_all_workflows(
        &self,
        workflow_name: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/workflow");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = workflow_name {
                query_params.push(("workflowName", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issuetype/{id}/alternatives
    pub async fn get_alternative_issue_types(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<IssueTypeJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetype/{}/alternatives", id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectKeyOrId}/permissionscheme
    pub async fn get_assigned_permission_scheme(
        &self,
        project_key_or_id: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<PermissionSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/permissionscheme",
            project_key_or_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectKeyOrId}/priorityscheme
    pub async fn get_assigned_priority_scheme(
        &self,
        project_key_or_id: impl AsRef<str>,
    ) -> HttpResult<PrioritySchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/priorityscheme",
            project_key_or_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issuetypescheme/{schemeId}/associations
    pub async fn get_associated_projects(
        &self,
        scheme_id: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<ProjectBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetypescheme/{}/associations",
            scheme_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/attachment/{id}
    pub async fn get_attachment(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<AttachmentBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/attachment/{}", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/attachment/meta
    pub async fn get_attachment_meta(&self) -> HttpResult<AttachmentMetaBean> {
        let url = format!("{}{}", self.base_url, "/api/2/attachment/meta");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/jql/autocompletedata
    pub async fn get_auto_complete(&self) -> HttpResult<AutoCompleteResponseBean> {
        let url = format!("{}{}", self.base_url, "/api/2/jql/autocompletedata");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/monitoring/jmx/getAvailableMetrics
    pub async fn get_available_metrics(
        &self,
    ) -> HttpResult<GetAvailableMetricsResponse> {
        let url = format!(
            "{}{}", self.base_url, "/api/2/monitoring/jmx/getAvailableMetrics"
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/universal_avatar/type/{type}/owner/{owningObjectId}
    pub async fn get_avatars(
        &self,
        type_: impl AsRef<str>,
        owning_object_id: impl AsRef<str>,
    ) -> HttpResult<AvatarBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/universal_avatar/type/{}/owner/{}",
            type_.as_ref(), owning_object_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}
    pub async fn get_board(&self, board_id: i64) -> HttpResult<BoardBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}", board_id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/workflowscheme/{id}
    pub async fn get_by_id(
        &self,
        id: i64,
        return_draft_if_exists: Option<bool>,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}", id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = return_draft_if_exists {
                query_params.push(("returnDraftIfExists", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/comment/{id}
    pub async fn get_comment(
        &self,
        issue_id_or_key: impl AsRef<str>,
        id: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<CommentJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/comment/{}", issue_id_or_key
            .as_ref(), id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/comment
    pub async fn get_comments(
        &self,
        issue_id_or_key: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
        max_results: Option<impl AsRef<str>>,
        order_by: Option<impl AsRef<str>>,
        start_at: Option<impl AsRef<str>>,
    ) -> HttpResult<CommentsWithPaginationJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/comment", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.as_ref().to_string()));
            }
            if let Some(v) = order_by {
                query_params.push(("orderBy", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/component/{id}
    pub async fn get_component(&self, id: impl AsRef<str>) -> HttpResult<ComponentBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/component/{}", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/component/{id}/relatedIssueCounts
    pub async fn get_component_related_issues(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<ComponentIssueCountsBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/component/{}/relatedIssueCounts", id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}/configuration
    pub async fn get_configuration(&self, board_id: i64) -> HttpResult<BoardConfigBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}/configuration", board_id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/configuration
    pub async fn get_configuration_1(&self) -> HttpResult<ConfigurationBean> {
        let url = format!("{}{}", self.base_url, "/api/2/configuration");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/createmeta/{projectIdOrKey}/issuetypes/{issueTypeId}
    pub async fn get_create_issue_meta_fields(
        &self,
        issue_type_id: impl AsRef<str>,
        project_id_or_key: impl AsRef<str>,
        max_results: Option<impl AsRef<str>>,
        start_at: Option<impl AsRef<str>>,
    ) -> HttpResult<FieldMetaBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/createmeta/{}/issuetypes/{}",
            issue_type_id.as_ref(), project_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/createmeta/{projectIdOrKey}/issuetypes
    pub async fn get_create_issue_meta_project_issue_types(
        &self,
        project_id_or_key: impl AsRef<str>,
        max_results: Option<impl AsRef<str>>,
        start_at: Option<impl AsRef<str>>,
    ) -> HttpResult<CreateMetaIssueTypeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/createmeta/{}/issuetypes",
            project_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/customFieldOption/{id}
    pub async fn get_custom_field_option(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<CustomFieldOptionBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/customFieldOption/{}", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/customFields/{customFieldId}/options
    pub async fn get_custom_field_options(
        &self,
        custom_field_id: impl AsRef<str>,
        max_results: Option<impl AsRef<str>>,
        issue_type_ids: Option<impl AsRef<str>>,
        query: Option<impl AsRef<str>>,
        sort_by_option_name: Option<impl AsRef<str>>,
        use_all_contexts: Option<impl AsRef<str>>,
        page: Option<impl AsRef<str>>,
        project_ids: Option<impl AsRef<str>>,
    ) -> HttpResult<CustomFieldOptionsBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/customFields/{}/options",
            custom_field_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.as_ref().to_string()));
            }
            if let Some(v) = issue_type_ids {
                query_params.push(("issueTypeIds", v.as_ref().to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = sort_by_option_name {
                query_params.push(("sortByOptionName", v.as_ref().to_string()));
            }
            if let Some(v) = use_all_contexts {
                query_params.push(("useAllContexts", v.as_ref().to_string()));
            }
            if let Some(v) = page {
                query_params.push(("page", v.as_ref().to_string()));
            }
            if let Some(v) = project_ids {
                query_params.push(("projectIds", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/customFields
    pub async fn get_custom_fields(
        &self,
        sort_column: Option<impl AsRef<str>>,
        types: Option<impl AsRef<str>>,
        search: Option<impl AsRef<str>>,
        max_results: Option<impl AsRef<str>>,
        sort_order: Option<impl AsRef<str>>,
        screen_ids: Option<impl AsRef<str>>,
        last_value_update: Option<impl AsRef<str>>,
        project_ids: Option<impl AsRef<str>>,
        start_at: Option<impl AsRef<str>>,
    ) -> HttpResult<CustomFieldBean> {
        let url = format!("{}{}", self.base_url, "/api/2/customFields");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = sort_column {
                query_params.push(("sortColumn", v.as_ref().to_string()));
            }
            if let Some(v) = types {
                query_params.push(("types", v.as_ref().to_string()));
            }
            if let Some(v) = search {
                query_params.push(("search", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.as_ref().to_string()));
            }
            if let Some(v) = sort_order {
                query_params.push(("sortOrder", v.as_ref().to_string()));
            }
            if let Some(v) = screen_ids {
                query_params.push(("screenIds", v.as_ref().to_string()));
            }
            if let Some(v) = last_value_update {
                query_params.push(("lastValueUpdate", v.as_ref().to_string()));
            }
            if let Some(v) = project_ids {
                query_params.push(("projectIds", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/dashboard/{id}
    pub async fn get_dashboard(&self, id: impl AsRef<str>) -> HttpResult<DashboardBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/dashboard/{}", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/workflowscheme/{id}/default
    pub async fn get_default(
        &self,
        id: i64,
        return_draft_if_exists: Option<bool>,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/default", id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = return_draft_if_exists {
                query_params.push(("returnDraftIfExists", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/filter/defaultShareScope
    pub async fn get_default_share_scope(&self) -> HttpResult<DefaultShareScopeBean> {
        let url = format!("{}{}", self.base_url, "/api/2/filter/defaultShareScope");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/workflowscheme/{id}/draft
    pub async fn get_draft_by_id(&self, id: i64) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft", id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/workflowscheme/{id}/draft/default
    pub async fn get_draft_default(&self, id: i64) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft/default", id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/workflowscheme/{id}/draft/issuetype/{issueType}
    pub async fn get_draft_issue_type(
        &self,
        issue_type: impl AsRef<str>,
        id: i64,
    ) -> HttpResult<IssueTypeMappingBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft/issuetype/{}",
            issue_type.as_ref(), id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/workflowscheme/{id}/draft/workflow
    pub async fn get_draft_workflow(
        &self,
        id: i64,
        workflow_name: Option<impl AsRef<str>>,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft/workflow", id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = workflow_name {
                query_params.push(("workflowName", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/duplicated/count
    pub async fn get_duplicated_users_count(
        &self,
        flush: Option<bool>,
    ) -> HttpResult<UserBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/duplicated/count");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = flush {
                query_params.push(("flush", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/duplicated/list
    pub async fn get_duplicated_users_mapping(
        &self,
        flush: Option<bool>,
    ) -> HttpResult<AvatarBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/duplicated/list");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = flush {
                query_params.push(("flush", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/editmeta
    pub async fn get_edit_issue_meta(
        &self,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<EditMetaBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/editmeta", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/email-templates/types
    pub async fn get_email_types(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/email-templates/types");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/epic/{epicIdOrKey}
    pub async fn get_epic(
        &self,
        epic_id_or_key: impl AsRef<str>,
    ) -> HttpResult<EpicBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/epic/{}", epic_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}/epic
    pub async fn get_epics(
        &self,
        board_id: i64,
        max_results: Option<i64>,
        done: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<EpicBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}/epic", board_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = done {
                query_params.push(("done", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/search/error/lookup
    pub async fn get_error(&self) -> HttpResult<ResponseValue> {
        let url = format!("{}{}", self.base_url, "/api/2/search/error/lookup");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/filter/favourite
    pub async fn get_favourite_filters(
        &self,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<FilterBean> {
        let url = format!("{}{}", self.base_url, "/api/2/filter/favourite");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/jql/autocompletedata/suggestions
    pub async fn get_field_auto_complete_for_query_string(
        &self,
        predicate_value: Option<impl AsRef<str>>,
        predicate_name: Option<impl AsRef<str>>,
        field_name: Option<impl AsRef<str>>,
        field_value: Option<impl AsRef<str>>,
    ) -> HttpResult<AutoCompleteResultWrapper> {
        let url = format!(
            "{}{}", self.base_url, "/api/2/jql/autocompletedata/suggestions"
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = predicate_value {
                query_params.push(("predicateValue", v.as_ref().to_string()));
            }
            if let Some(v) = predicate_name {
                query_params.push(("predicateName", v.as_ref().to_string()));
            }
            if let Some(v) = field_name {
                query_params.push(("fieldName", v.as_ref().to_string()));
            }
            if let Some(v) = field_value {
                query_params.push(("fieldValue", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/field
    pub async fn get_fields(&self) -> HttpResult<FieldBean> {
        let url = format!("{}{}", self.base_url, "/api/2/field");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/screens/{screenId}/availableFields
    pub async fn get_fields_to_add(
        &self,
        screen_id: i64,
    ) -> HttpResult<ScreenableFieldBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/screens/{}/availableFields",
            screen_id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/filter/{id}
    pub async fn get_filter(
        &self,
        id: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<FilterBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/filter/{}", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/worklog/deleted
    pub async fn get_ids_of_worklogs_deleted_since(
        &self,
        since: Option<i64>,
    ) -> HttpResult<WorklogChangedSinceBean> {
        let url = format!("{}{}", self.base_url, "/api/2/worklog/deleted");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = since {
                query_params.push(("since", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/worklog/updated
    pub async fn get_ids_of_worklogs_modified_since(
        &self,
        since: Option<i64>,
    ) -> HttpResult<WorklogChangedSinceBean> {
        let url = format!("{}{}", self.base_url, "/api/2/worklog/updated");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = since {
                query_params.push(("since", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/index/summary
    pub async fn get_index_summary(&self) -> HttpResult<IndexSummaryBean> {
        let url = format!("{}{}", self.base_url, "/api/2/index/summary");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}
    pub async fn get_issue(
        &self,
        issue_id_or_key: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
        fields: Option<impl AsRef<str>>,
        update_history: Option<impl AsRef<str>>,
        properties: Option<impl AsRef<str>>,
    ) -> HttpResult<IssueBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}", issue_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = fields {
                query_params.push(("fields", v.as_ref().to_string()));
            }
            if let Some(v) = update_history {
                query_params.push(("updateHistory", v.as_ref().to_string()));
            }
            if let Some(v) = properties {
                query_params.push(("properties", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issuetype
    pub async fn get_issue_all_types(&self) -> HttpResult<IssueTypeJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/issuetype");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/issue/{issueIdOrKey}/estimation
    pub async fn get_issue_estimation_for_board(
        &self,
        issue_id_or_key: impl AsRef<str>,
        board_id: Option<i64>,
    ) -> HttpResult<FieldValueBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/issue/{}/estimation",
            issue_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = board_id {
                query_params.push(("boardId", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issueLink/{linkId}
    pub async fn get_issue_link(
        &self,
        link_id: impl AsRef<str>,
    ) -> HttpResult<IssueLinks> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issueLink/{}", link_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issueLinkType/{issueLinkTypeId}
    pub async fn get_issue_link_type(
        &self,
        issue_link_type_id: impl AsRef<str>,
    ) -> HttpResult<IssueLinkTypeJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issueLinkType/{}", issue_link_type_id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issueLinkType
    pub async fn get_issue_link_types(&self) -> HttpResult<IssueLinkTypesBean> {
        let url = format!("{}{}", self.base_url, "/api/2/issueLinkType");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/settings/columns
    pub async fn get_issue_navigator_default_columns(
        &self,
    ) -> HttpResult<ColumnOptions> {
        let url = format!("{}{}", self.base_url, "/api/2/settings/columns");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/picker
    pub async fn get_issue_picker_resource(
        &self,
        current_project_id: Option<impl AsRef<str>>,
        query: Option<impl AsRef<str>>,
        current_issue_key: Option<impl AsRef<str>>,
        show_sub_tasks: Option<impl AsRef<str>>,
        current_jql: Option<impl AsRef<str>>,
        show_sub_task_parent: Option<impl AsRef<str>>,
    ) -> HttpResult<IssuePickerResult> {
        let url = format!("{}{}", self.base_url, "/api/2/issue/picker");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = current_project_id {
                query_params.push(("currentProjectId", v.as_ref().to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = current_issue_key {
                query_params.push(("currentIssueKey", v.as_ref().to_string()));
            }
            if let Some(v) = show_sub_tasks {
                query_params.push(("showSubTasks", v.as_ref().to_string()));
            }
            if let Some(v) = current_jql {
                query_params.push(("currentJQL", v.as_ref().to_string()));
            }
            if let Some(v) = show_sub_task_parent {
                query_params.push(("showSubTaskParent", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issuesecurityschemes/{id}
    pub async fn get_issue_security_scheme(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<SecuritySchemeJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuesecurityschemes/{}", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectKeyOrId}/issuesecuritylevelscheme
    pub async fn get_issue_security_scheme_1(
        &self,
        project_key_or_id: impl AsRef<str>,
    ) -> HttpResult<SecuritySchemeJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/issuesecuritylevelscheme",
            project_key_or_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issuesecurityschemes
    pub async fn get_issue_security_schemes(
        &self,
    ) -> HttpResult<SecuritySchemesJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/issuesecurityschemes");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/workflowscheme/{id}/issuetype/{issueType}
    pub async fn get_issue_type(
        &self,
        issue_type: impl AsRef<str>,
        id: i64,
        return_draft_if_exists: Option<bool>,
    ) -> HttpResult<IssueTypeMappingBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/issuetype/{}",
            issue_type.as_ref(), id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = return_draft_if_exists {
                query_params.push(("returnDraftIfExists", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issuetypescheme/{schemeId}
    pub async fn get_issue_type_scheme(
        &self,
        scheme_id: impl AsRef<str>,
    ) -> HttpResult<IssueTypeSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetypescheme/{}", scheme_id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issuetype/{id}
    pub async fn get_issue_type_1(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<IssueTypeJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetype/{}", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/watchers
    pub async fn get_issue_watchers(
        &self,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<WatchersBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/watchers", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/worklog
    pub async fn get_issue_worklog(
        &self,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<WorklogWithPaginationBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/worklog", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}/backlog
    pub async fn get_issues_for_backlog(
        &self,
        board_id: i64,
        expand: Option<impl AsRef<str>>,
        jql: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        validate_query: Option<bool>,
        fields: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<IssueBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}/backlog", board_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = jql {
                query_params.push(("jql", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = validate_query {
                query_params.push(("validateQuery", v.to_string()));
            }
            if let Some(v) = fields {
                query_params.push(("fields", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}/issue
    pub async fn get_issues_for_board(
        &self,
        board_id: i64,
        expand: Option<impl AsRef<str>>,
        jql: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        validate_query: Option<bool>,
        fields: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<IssueBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}/issue", board_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = jql {
                query_params.push(("jql", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = validate_query {
                query_params.push(("validateQuery", v.to_string()));
            }
            if let Some(v) = fields {
                query_params.push(("fields", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}/epic/{epicId}/issue
    pub async fn get_issues_for_epic(
        &self,
        epic_id: i64,
        board_id: i64,
        expand: Option<impl AsRef<str>>,
        jql: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        validate_query: Option<bool>,
        fields: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<IssueBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}/epic/{}/issue", epic_id,
            board_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = jql {
                query_params.push(("jql", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = validate_query {
                query_params.push(("validateQuery", v.to_string()));
            }
            if let Some(v) = fields {
                query_params.push(("fields", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/epic/{epicIdOrKey}/issue
    pub async fn get_issues_for_epic_1(
        &self,
        epic_id_or_key: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
        jql: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        validate_query: Option<bool>,
        fields: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<SearchResultsBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/epic/{}/issue", epic_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = jql {
                query_params.push(("jql", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = validate_query {
                query_params.push(("validateQuery", v.to_string()));
            }
            if let Some(v) = fields {
                query_params.push(("fields", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}/sprint/{sprintId}/issue
    pub async fn get_issues_for_sprint(
        &self,
        sprint_id: i64,
        board_id: i64,
        expand: Option<impl AsRef<str>>,
        jql: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        validate_query: Option<bool>,
        fields: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<SprintBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}/sprint/{}/issue",
            sprint_id, board_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = jql {
                query_params.push(("jql", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = validate_query {
                query_params.push(("validateQuery", v.to_string()));
            }
            if let Some(v) = fields {
                query_params.push(("fields", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/sprint/{sprintId}/issue
    pub async fn get_issues_for_sprint_1(
        &self,
        sprint_id: i64,
        expand: Option<impl AsRef<str>>,
        jql: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        validate_query: Option<bool>,
        fields: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<SearchResultsBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/sprint/{}/issue", sprint_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = jql {
                query_params.push(("jql", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = validate_query {
                query_params.push(("validateQuery", v.to_string()));
            }
            if let Some(v) = fields {
                query_params.push(("fields", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}/epic/none/issue
    pub async fn get_issues_without_epic(
        &self,
        board_id: i64,
        expand: Option<impl AsRef<str>>,
        jql: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        validate_query: Option<bool>,
        fields: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<IssueBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}/epic/none/issue",
            board_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = jql {
                query_params.push(("jql", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = validate_query {
                query_params.push(("validateQuery", v.to_string()));
            }
            if let Some(v) = fields {
                query_params.push(("fields", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/epic/none/issue
    pub async fn get_issues_without_epic_1(
        &self,
        expand: Option<impl AsRef<str>>,
        jql: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        validate_query: Option<bool>,
        fields: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<SearchResultsBean> {
        let url = format!("{}{}", self.base_url, "/agile/1.0/epic/none/issue");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = jql {
                query_params.push(("jql", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = validate_query {
                query_params.push(("validateQuery", v.to_string()));
            }
            if let Some(v) = fields {
                query_params.push(("fields", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/securitylevel/{id}
    pub async fn get_issuesecuritylevel(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<SecurityLevelJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/securitylevel/{}", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/searchLimits/maxAggregationBuckets
    pub async fn get_max_aggregation_buckets(
        &self,
    ) -> HttpResult<GetMaxAggregationBucketsResponse> {
        let url = format!(
            "{}{}", self.base_url, "/api/2/searchLimits/maxAggregationBuckets"
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/searchLimits/maxResultWindow
    pub async fn get_max_result_window(&self) -> HttpResult<GetMaxResultWindowResponse> {
        let url = format!("{}{}", self.base_url, "/api/2/searchLimits/maxResultWindow");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/notificationscheme/{id}
    pub async fn get_notification_scheme(
        &self,
        id: i64,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<NotificationSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/notificationscheme/{}", id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectKeyOrId}/notificationscheme
    pub async fn get_notification_scheme_1(
        &self,
        project_key_or_id: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<NotificationSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/notificationscheme",
            project_key_or_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/notificationscheme
    pub async fn get_notification_schemes(
        &self,
        expand: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        start_at: Option<i64>,
    ) -> HttpResult<PageBean> {
        let url = format!("{}{}", self.base_url, "/api/2/notificationscheme");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/component/page
    pub async fn get_paginated_components(
        &self,
        max_results: Option<impl AsRef<str>>,
        query: Option<impl AsRef<str>>,
        project_ids: Option<impl AsRef<str>>,
        start_at: Option<impl AsRef<str>>,
    ) -> HttpResult<PageBean> {
        let url = format!("{}{}", self.base_url, "/api/2/component/page");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.as_ref().to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = project_ids {
                query_params.push(("projectIds", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issuetype/page
    pub async fn get_paginated_issue_types(
        &self,
        max_results: Option<i64>,
        query: Option<impl AsRef<str>>,
        project_ids: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<IssueTypeJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/issuetype/page");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = project_ids {
                query_params.push(("projectIds", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/resolution/page
    pub async fn get_paginated_resolutions(
        &self,
        max_results: Option<i64>,
        query: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<ResolutionBean> {
        let url = format!("{}{}", self.base_url, "/api/2/resolution/page");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/status/page
    pub async fn get_paginated_statuses(
        &self,
        issue_type_ids: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        query: Option<impl AsRef<str>>,
        project_ids: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<StatusJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/status/page");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = issue_type_ids {
                query_params.push(("issueTypeIds", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = project_ids {
                query_params.push(("projectIds", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/version
    pub async fn get_paginated_versions(
        &self,
        max_results: Option<i64>,
        query: Option<impl AsRef<str>>,
        project_ids: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<VersionBean> {
        let url = format!("{}{}", self.base_url, "/api/2/version");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = project_ids {
                query_params.push(("projectIds", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/password/policy
    pub async fn get_password_policy(
        &self,
        has_old_password: Option<bool>,
    ) -> HttpResult<GetPasswordPolicyResponse> {
        let url = format!("{}{}", self.base_url, "/api/2/password/policy");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = has_old_password {
                query_params.push(("hasOldPassword", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/permissionscheme/{schemeId}
    pub async fn get_permission_scheme(
        &self,
        scheme_id: i64,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<PermissionSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/permissionscheme/{}", scheme_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/permissionscheme/{schemeId}/permission/{permissionId}
    pub async fn get_permission_scheme_grant(
        &self,
        permission_id: i64,
        scheme_id: i64,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<PermissionGrantBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/permissionscheme/{}/permission/{}",
            permission_id, scheme_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/permissionscheme/{schemeId}/permission
    pub async fn get_permission_scheme_grants(
        &self,
        scheme_id: i64,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<PermissionGrantsBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/permissionscheme/{}/permission",
            scheme_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/permissionscheme
    pub async fn get_permission_schemes(
        &self,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<PermissionSchemesBean> {
        let url = format!("{}{}", self.base_url, "/api/2/permissionscheme");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/mypermissions
    pub async fn get_permissions(
        &self,
        issue_id: Option<impl AsRef<str>>,
        project_key: Option<impl AsRef<str>>,
        issue_key: Option<impl AsRef<str>>,
        project_id: Option<impl AsRef<str>>,
    ) -> HttpResult<PermissionsJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/mypermissions");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = issue_id {
                query_params.push(("issueId", v.as_ref().to_string()));
            }
            if let Some(v) = project_key {
                query_params.push(("projectKey", v.as_ref().to_string()));
            }
            if let Some(v) = issue_key {
                query_params.push(("issueKey", v.as_ref().to_string()));
            }
            if let Some(v) = project_id {
                query_params.push(("projectId", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/pinned-comments
    pub async fn get_pinned_comments(
        &self,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<PinnedCommentJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/pinned-comments",
            issue_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/mypreferences
    pub async fn get_preference(
        &self,
        key: Option<impl AsRef<str>>,
    ) -> HttpResult<GetPreferenceResponse> {
        let url = format!("{}{}", self.base_url, "/api/2/mypreferences");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = key {
                query_params.push(("key", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/priority
    pub async fn get_priorities(&self) -> HttpResult<PriorityJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/priority");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/priority/page
    pub async fn get_priorities_1(
        &self,
        max_results: Option<i64>,
        query: Option<impl AsRef<str>>,
        project_ids: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<PriorityJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/priority/page");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = project_ids {
                query_params.push(("projectIds", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/priority/{id}
    pub async fn get_priority(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<PriorityJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/priority/{}", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/priorityschemes/{schemeId}
    pub async fn get_priority_scheme(
        &self,
        scheme_id: i64,
    ) -> HttpResult<PrioritySchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/priorityschemes/{}", scheme_id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/priorityschemes
    pub async fn get_priority_schemes(
        &self,
        max_results: Option<i64>,
        start_at: Option<i64>,
    ) -> HttpResult<PrioritySchemeListBean> {
        let url = format!("{}{}", self.base_url, "/api/2/priorityschemes");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/reindex/request/{requestId}
    pub async fn get_progress(&self, request_id: i64) -> HttpResult<ReindexRequestBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/reindex/request/{}", request_id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/reindex/request/bulk
    pub async fn get_progress_bulk(
        &self,
        request_id: Option<impl AsRef<str>>,
    ) -> HttpResult<ReindexRequestBean> {
        let url = format!("{}{}", self.base_url, "/api/2/reindex/request/bulk");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = request_id {
                query_params.push(("requestId", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/anonymization/progress
    pub async fn get_progress_1(&self, task_id: Option<i64>) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/anonymization/progress");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = task_id {
                query_params.push(("taskId", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectIdOrKey}
    pub async fn get_project(
        &self,
        project_id_or_key: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<ProjectBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}", project_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/projectCategory/{id}
    pub async fn get_project_category_by_id(
        &self,
        id: i64,
    ) -> HttpResult<ProjectCategoryJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/projectCategory/{}", id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectIdOrKey}/components
    pub async fn get_project_components(
        &self,
        project_id_or_key: impl AsRef<str>,
    ) -> HttpResult<ComponentBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/components",
            project_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectIdOrKey}/role/{id}
    pub async fn get_project_role(
        &self,
        project_id_or_key: impl AsRef<str>,
        id: i64,
    ) -> HttpResult<ProjectRoleBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/role/{}", project_id_or_key
            .as_ref(), id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/role/{id}/actors
    pub async fn get_project_role_actors_for_role(
        &self,
        id: i64,
    ) -> HttpResult<ProjectRoleActorsBean> {
        let url = format!("{}{}", self.base_url, format!("/api/2/role/{}/actors", id));
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectIdOrKey}/role
    pub async fn get_project_roles(
        &self,
        project_id_or_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/role", project_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/role/{id}
    pub async fn get_project_roles_by_id(&self, id: i64) -> HttpResult<ProjectRoleBean> {
        let url = format!("{}{}", self.base_url, format!("/api/2/role/{}", id));
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/role
    pub async fn get_project_roles_1(&self) -> HttpResult<ProjectRoleBean> {
        let url = format!("{}{}", self.base_url, "/api/2/role");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/type/{projectTypeKey}
    pub async fn get_project_type_by_key(
        &self,
        project_type_key: impl AsRef<str>,
    ) -> HttpResult<ProjectTypeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/type/{}", project_type_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectIdOrKey}/versions
    pub async fn get_project_versions(
        &self,
        project_id_or_key: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<VersionBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/versions",
            project_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectIdOrKey}/version
    pub async fn get_project_versions_paginated(
        &self,
        project_id_or_key: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        order_by: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<PageBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/version", project_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = order_by {
                query_params.push(("orderBy", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/projectvalidate/key
    pub async fn get_project_1(
        &self,
        key: Option<impl AsRef<str>>,
    ) -> HttpResult<ErrorCollection> {
        let url = format!("{}{}", self.base_url, "/api/2/projectvalidate/key");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = key {
                query_params.push(("key", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}/project
    pub async fn get_projects(
        &self,
        board_id: i64,
        max_results: Option<i64>,
        start_at: Option<i64>,
    ) -> HttpResult<ProjectJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/board/{}/project", board_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/dashboard/{dashboardId}/items/{itemId}/properties
    pub async fn get_properties_keys(
        &self,
        item_id: impl AsRef<str>,
        dashboard_id: impl AsRef<str>,
    ) -> HttpResult<EntityPropertiesKeysBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/dashboard/{}/items/{}/properties",
            item_id.as_ref(), dashboard_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/comment/{commentId}/properties
    pub async fn get_properties_keys_1(
        &self,
        comment_id: impl AsRef<str>,
    ) -> HttpResult<EntityPropertiesKeysBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/comment/{}/properties", comment_id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/properties
    pub async fn get_properties_keys_2(
        &self,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<EntityPropertiesKeysBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/properties", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectIdOrKey}/properties
    pub async fn get_properties_keys_3(
        &self,
        project_id_or_key: impl AsRef<str>,
    ) -> HttpResult<EntityPropertiesKeysBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/properties",
            project_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/properties
    pub async fn get_properties_keys_4(
        &self,
        user_key: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/properties");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = user_key {
                query_params.push(("userKey", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/application-properties
    pub async fn get_property(
        &self,
        permission_level: impl AsRef<str>,
        key_filter: Option<impl AsRef<str>>,
        key: impl AsRef<str>,
    ) -> HttpResult<Property> {
        let url = format!("{}{}", self.base_url, "/api/2/application-properties");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params
                .push(("permissionLevel", permission_level.as_ref().to_string()));
            if let Some(v) = key_filter {
                query_params.push(("keyFilter", v.as_ref().to_string()));
            }
            query_params.push(("key", key.as_ref().to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issuetype/{issueTypeId}/properties
    pub async fn get_property_keys(
        &self,
        issue_type_id: impl AsRef<str>,
    ) -> HttpResult<EntityPropertiesKeysBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetype/{}/properties",
            issue_type_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/dashboard/{dashboardId}/items/{itemId}/properties/{propertyKey}
    pub async fn get_property_1(
        &self,
        property_key: impl AsRef<str>,
        item_id: impl AsRef<str>,
        dashboard_id: impl AsRef<str>,
    ) -> HttpResult<EntityPropertyBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/dashboard/{}/items/{}/properties/{}",
            property_key.as_ref(), item_id.as_ref(), dashboard_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/comment/{commentId}/properties/{propertyKey}
    pub async fn get_property_2(
        &self,
        property_key: impl AsRef<str>,
        comment_id: impl AsRef<str>,
    ) -> HttpResult<EntityPropertyBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/comment/{}/properties/{}",
            property_key.as_ref(), comment_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/properties/{propertyKey}
    pub async fn get_property_3(
        &self,
        property_key: impl AsRef<str>,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<EntityPropertyBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/properties/{}", property_key
            .as_ref(), issue_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issuetype/{issueTypeId}/properties/{propertyKey}
    pub async fn get_property_4(
        &self,
        property_key: impl AsRef<str>,
        issue_type_id: impl AsRef<str>,
    ) -> HttpResult<EntityPropertyBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetype/{}/properties/{}",
            property_key.as_ref(), issue_type_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectIdOrKey}/properties/{propertyKey}
    pub async fn get_property_5(
        &self,
        property_key: impl AsRef<str>,
        project_id_or_key: impl AsRef<str>,
    ) -> HttpResult<EntityPropertyBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/properties/{}",
            property_key.as_ref(), project_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/properties/{propertyKey}
    pub async fn get_property_6(
        &self,
        property_key: impl AsRef<str>,
        user_key: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/user/properties/{}", property_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = user_key {
                query_params.push(("userKey", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/board/{boardId}/settings/refined-velocity
    pub async fn get_refined_velocity(
        &self,
        board_id: i64,
    ) -> HttpResult<BooleanSettingBean> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/agile/1.0/board/{}/settings/refined-velocity", board_id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/reindex
    pub async fn get_reindex_info(
        &self,
        task_id: Option<i64>,
    ) -> HttpResult<ReindexBean> {
        let url = format!("{}{}", self.base_url, "/api/2/reindex");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = task_id {
                query_params.push(("taskId", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/reindex/progress
    pub async fn get_reindex_progress(
        &self,
        task_id: Option<i64>,
    ) -> HttpResult<ReindexBean> {
        let url = format!("{}{}", self.base_url, "/api/2/reindex/progress");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = task_id {
                query_params.push(("taskId", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/remotelink/{linkId}
    pub async fn get_remote_issue_link_by_id(
        &self,
        link_id: impl AsRef<str>,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<RemoteIssueLinkBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/remotelink/{}", link_id
            .as_ref(), issue_id_or_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/remotelink
    pub async fn get_remote_issue_links(
        &self,
        issue_id_or_key: impl AsRef<str>,
        global_id: Option<impl AsRef<str>>,
    ) -> HttpResult<RemoteIssueLinkBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/remotelink", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = global_id {
                query_params.push(("globalId", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/version/{versionId}/remotelink/{globalId}
    pub async fn get_remote_version_link(
        &self,
        version_id: impl AsRef<str>,
        global_id: impl AsRef<str>,
    ) -> HttpResult<RemoteEntityLinkJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}/remotelink/{}", version_id
            .as_ref(), global_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/version/remotelink
    pub async fn get_remote_version_links(
        &self,
        global_id: Option<impl AsRef<str>>,
    ) -> HttpResult<RemoteEntityLinksJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/version/remotelink");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = global_id {
                query_params.push(("globalId", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/version/{versionId}/remotelink
    pub async fn get_remote_version_links_by_version_id(
        &self,
        version_id: impl AsRef<str>,
    ) -> HttpResult<RemoteEntityLinksJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}/remotelink", version_id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/resolution/{id}
    pub async fn get_resolution(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<ResolutionJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/resolution/{}", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/resolution
    pub async fn get_resolutions(&self) -> HttpResult<ResolutionJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/resolution");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/permissionscheme/{permissionSchemeId}/attribute/{attributeKey}
    pub async fn get_scheme_attribute(
        &self,
        permission_scheme_id: i64,
        attribute_key: impl AsRef<str>,
    ) -> HttpResult<PermissionSchemeAttributeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/permissionscheme/{}/attribute/{}",
            permission_scheme_id, attribute_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectKeyOrId}/securitylevel
    pub async fn get_security_levels_for_project(
        &self,
        project_key_or_id: impl AsRef<str>,
    ) -> HttpResult<SecurityListLevelJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/securitylevel",
            project_key_or_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/serverInfo
    pub async fn get_server_info(&self) -> HttpResult<ServerInfoBean> {
        let url = format!("{}{}", self.base_url, "/api/2/serverInfo");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/filter/{id}/permission/{permissionId}
    pub async fn get_share_permission(
        &self,
        permission_id: impl AsRef<str>,
        id: impl AsRef<str>,
    ) -> HttpResult<FilterPermissionBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/filter/{}/permission/{}",
            permission_id.as_ref(), id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/filter/{id}/permission
    pub async fn get_share_permissions(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<FilterPermissionBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/filter/{}/permission", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /agile/1.0/sprint/{sprintId}
    pub async fn get_sprint(&self, sprint_id: i64) -> HttpResult<SprintBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/sprint/{}", sprint_id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/cluster/zdu/state
    pub async fn get_state(&self) -> HttpResult<ClusterState> {
        let url = format!("{}{}", self.base_url, "/api/2/cluster/zdu/state");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/status/{idOrName}
    pub async fn get_status(
        &self,
        id_or_name: impl AsRef<str>,
    ) -> HttpResult<StatusJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/status/{}", id_or_name.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/statuscategory
    pub async fn get_status_categories(
        &self,
        request: Option<impl AsRef<str>>,
        uri_info: Option<impl AsRef<str>>,
    ) -> HttpResult<StatusCategoryJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/statuscategory");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = request {
                query_params.push(("request", v.as_ref().to_string()));
            }
            if let Some(v) = uri_info {
                query_params.push(("uriInfo", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/statuscategory/{idOrKey}
    pub async fn get_status_category(
        &self,
        id_or_key: impl AsRef<str>,
    ) -> HttpResult<StatusCategoryJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/statuscategory/{}", id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/status
    pub async fn get_statuses(&self) -> HttpResult<StatusJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/status");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/subtask
    pub async fn get_sub_tasks(
        &self,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<IssueRefJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/subtask", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/terminology/entries/{originalName}
    pub async fn get_terminology_entry(
        &self,
        original_name: impl AsRef<str>,
    ) -> HttpResult<TerminologyResponseBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/terminology/entries/{}", original_name
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/transitions
    pub async fn get_transitions(
        &self,
        issue_id_or_key: impl AsRef<str>,
        transition_id: Option<impl AsRef<str>>,
    ) -> HttpResult<TransitionsMetaBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/transitions", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = transition_id {
                query_params.push(("transitionId", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/upgrade
    pub async fn get_upgrade_result(&self) -> HttpResult<UpgradeResultBean> {
        let url = format!("{}{}", self.base_url, "/api/2/upgrade");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/myself
    pub async fn get_user(&self) -> HttpResult<UserBean> {
        let url = format!("{}{}", self.base_url, "/api/2/myself");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/list
    pub async fn get_user_list(
        &self,
        cursor: Option<i64>,
        max_results: Option<i64>,
    ) -> HttpResult<StreamPageBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/list");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = cursor {
                query_params.push(("cursor", v.to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user
    pub async fn get_user_1(
        &self,
        include_deleted: Option<bool>,
        key: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<UserBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = include_deleted {
                query_params.push(("includeDeleted", v.to_string()));
            }
            if let Some(v) = key {
                query_params.push(("key", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/group/member
    pub async fn get_users_from_group(
        &self,
        include_inactive_users: Option<impl AsRef<str>>,
        max_results: Option<impl AsRef<str>>,
        groupname: impl AsRef<str>,
        start_at: Option<impl AsRef<str>>,
    ) -> HttpResult<UserJsonBean> {
        let url = format!("{}{}", self.base_url, "/api/2/group/member");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = include_inactive_users {
                query_params.push(("includeInactiveUsers", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.as_ref().to_string()));
            }
            query_params.push(("groupname", groupname.as_ref().to_string()));
            if let Some(v) = start_at {
                query_params.push(("startAt", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/version/{id}
    pub async fn get_version(
        &self,
        id: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<VersionBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/version/{id}/relatedIssueCounts
    pub async fn get_version_related_issues(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<VersionIssueCountsBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}/relatedIssueCounts", id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/version/{id}/unresolvedIssueCount
    pub async fn get_version_unresolved_issues(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<VersionUnresolvedIssueCountsBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}/unresolvedIssueCount", id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/votes
    pub async fn get_votes(
        &self,
        issue_id_or_key: impl AsRef<str>,
    ) -> HttpResult<VoteBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/votes", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/workflowscheme/{id}/workflow
    pub async fn get_workflow(
        &self,
        id: i64,
        workflow_name: Option<impl AsRef<str>>,
        return_draft_if_exists: Option<bool>,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/workflow", id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = workflow_name {
                query_params.push(("workflowName", v.as_ref().to_string()));
            }
            if let Some(v) = return_draft_if_exists {
                query_params.push(("returnDraftIfExists", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/project/{projectKeyOrId}/workflowscheme
    pub async fn get_workflow_scheme_for_project(
        &self,
        project_key_or_id: impl AsRef<str>,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/workflowscheme",
            project_key_or_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/issue/{issueIdOrKey}/worklog/{id}
    pub async fn get_worklog(
        &self,
        issue_id_or_key: impl AsRef<str>,
        id: impl AsRef<str>,
    ) -> HttpResult<Worklog> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/worklog/{}", issue_id_or_key
            .as_ref(), id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/worklog/list
    pub async fn get_worklogs_for_ids(
        &self,
        request: WorklogIdsRequestBean,
    ) -> HttpResult<WorklogList> {
        let url = format!("{}{}", self.base_url, "/api/2/worklog/list");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/applicationrole/{key}
    pub async fn get_4(&self, key: impl AsRef<str>) -> HttpResult<ApplicationRoleBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/applicationrole/{}", key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/monitoring/app
    pub async fn is_app_monitoring_enabled(
        &self,
    ) -> HttpResult<AppMonitoringRestEntity> {
        let url = format!("{}{}", self.base_url, "/api/2/monitoring/app");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/index-snapshot/isRunning
    pub async fn is_index_snapshot_running(
        &self,
    ) -> HttpResult<IndexSnapshotStatusBean> {
        let url = format!("{}{}", self.base_url, "/api/2/index-snapshot/isRunning");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/monitoring/ipd
    pub async fn is_ipd_monitoring_enabled(
        &self,
    ) -> HttpResult<IpdMonitoringRestEntity> {
        let url = format!("{}{}", self.base_url, "/api/2/monitoring/ipd");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issueLink
    pub async fn link_issues(
        &self,
        request: LinkIssueRequestJsonBean,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/issueLink");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/dashboard
    pub async fn list(
        &self,
        filter: Option<impl AsRef<str>>,
        max_results: Option<impl AsRef<str>>,
        start_at: Option<impl AsRef<str>>,
    ) -> HttpResult<DashboardsBean> {
        let url = format!("{}{}", self.base_url, "/api/2/dashboard");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/index-snapshot
    pub async fn list_index_snapshot(&self) -> HttpResult<IndexSnapshotBean> {
        let url = format!("{}{}", self.base_url, "/api/2/index-snapshot");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /auth/1/session
    pub async fn login(&self, request: AuthParams) -> HttpResult<AuthSuccess> {
        let url = format!("{}{}", self.base_url, "/auth/1/session");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /auth/1/session
    pub async fn logout(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/auth/1/session");
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/version/{id}/mergeto/{moveIssuesTo}
    pub async fn merge(
        &self,
        move_issues_to: impl AsRef<str>,
        id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}/mergeto/{}", move_issues_to
            .as_ref(), id.as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/screens/{screenId}/tabs/{tabId}/fields/{id}/move
    pub async fn move_field(
        &self,
        tab_id: i64,
        screen_id: i64,
        id: impl AsRef<str>,
        request: MoveFieldBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/screens/{}/tabs/{}/fields/{}/move",
            tab_id, screen_id, id.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issueLinkType/{issueLinkTypeId}/order
    pub async fn move_issue_link_type(
        &self,
        issue_link_type_id: impl AsRef<str>,
        request: IssueLinkTypeOrderUpdateRequest,
    ) -> HttpResult<IssueLinkTypeJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issueLinkType/{}/order",
            issue_link_type_id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /agile/1.0/backlog/issue
    pub async fn move_issues_to_backlog(
        &self,
        request: IssueAssignRequestBean,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/agile/1.0/backlog/issue");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /agile/1.0/epic/{epicIdOrKey}/issue
    pub async fn move_issues_to_epic(
        &self,
        epic_id_or_key: impl AsRef<str>,
        request: IssueAssignRequestBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/epic/{}/issue", epic_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /agile/1.0/sprint/{sprintId}/issue
    pub async fn move_issues_to_sprint(
        &self,
        sprint_id: i64,
        request: IssueAssignRequestBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/sprint/{}/issue", sprint_id)
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/{issueIdOrKey}/subtask/move
    pub async fn move_sub_tasks(
        &self,
        issue_id_or_key: impl AsRef<str>,
        request: IssueSubTaskMovePositionBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/subtask/move",
            issue_id_or_key.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/screens/{screenId}/tabs/{tabId}/move/{pos}
    pub async fn move_tab(
        &self,
        tab_id: i64,
        screen_id: i64,
        pos: i64,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/screens/{}/tabs/{}/move/{}", tab_id,
            screen_id, pos)
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/version/{id}/move
    pub async fn move_version(
        &self,
        id: impl AsRef<str>,
        request: VersionMoveBean,
    ) -> HttpResult<VersionBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}/move", id.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issue/{issueIdOrKey}/notify
    pub async fn notify(
        &self,
        issue_id_or_key: impl AsRef<str>,
        request: NotificationJsonBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/notify", issue_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/role/{id}
    pub async fn partial_update_project_role(
        &self,
        id: i64,
        request: CreateUpdateRoleRequestBean,
    ) -> HttpResult<ProjectRoleBean> {
        let url = format!("{}{}", self.base_url, format!("/api/2/role/{}", id));
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /agile/1.0/epic/{epicIdOrKey}
    pub async fn partially_update_epic(
        &self,
        epic_id_or_key: impl AsRef<str>,
        request: EpicUpdateBean,
    ) -> HttpResult<EpicBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/epic/{}", epic_id_or_key.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /agile/1.0/sprint/{sprintId}
    pub async fn partially_update_sprint(
        &self,
        sprint_id: i64,
        request: SprintBean,
    ) -> HttpResult<SprintBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/sprint/{}", sprint_id)
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/password/policy/createUser
    pub async fn policy_check_create_user(
        &self,
        request: PasswordPolicyCreateUserBean,
    ) -> HttpResult<PolicyCheckCreateUserResponse> {
        let url = format!("{}{}", self.base_url, "/api/2/password/policy/createUser");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/password/policy/updateUser
    pub async fn policy_check_update_user(
        &self,
        request: PasswordPolicyUpdateUserBean,
    ) -> HttpResult<PolicyCheckUpdateUserResponse> {
        let url = format!("{}{}", self.base_url, "/api/2/password/policy/updateUser");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/reindex/request
    pub async fn process_requests(&self) -> HttpResult<ProcessRequestsResponse> {
        let url = format!("{}{}", self.base_url, "/api/2/reindex/request");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/applicationrole
    pub async fn put_bulk(
        &self,
        request: ApplicationRoleBean,
    ) -> HttpResult<ApplicationRoleBean> {
        let url = format!("{}{}", self.base_url, "/api/2/applicationrole");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/applicationrole/{key}
    pub async fn put_2(
        &self,
        key: impl AsRef<str>,
        request: ApplicationRoleBean,
    ) -> HttpResult<ApplicationRoleBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/applicationrole/{}", key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /agile/1.0/epic/{epicIdOrKey}/rank
    pub async fn rank_epics(
        &self,
        epic_id_or_key: impl AsRef<str>,
        request: EpicRankRequestBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/epic/{}/rank", epic_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /agile/1.0/issue/rank
    pub async fn rank_issues(
        &self,
        request: IssueRankRequestBean,
    ) -> HttpResult<PartialSuccessBean> {
        let url = format!("{}{}", self.base_url, "/agile/1.0/issue/rank");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/reindex
    pub async fn reindex(
        &self,
        index_change_history: Option<bool>,
        type_: Option<impl AsRef<str>>,
        index_worklogs: Option<bool>,
        index_comments: Option<bool>,
    ) -> HttpResult<ReindexBean> {
        let url = format!("{}{}", self.base_url, "/api/2/reindex");
        let mut req = self.http_client.post(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = index_change_history {
                query_params.push(("indexChangeHistory", v.to_string()));
            }
            if let Some(v) = type_ {
                query_params.push(("type", v.as_ref().to_string()));
            }
            if let Some(v) = index_worklogs {
                query_params.push(("indexWorklogs", v.to_string()));
            }
            if let Some(v) = index_comments {
                query_params.push(("indexComments", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/reindex/issue
    pub async fn reindex_issues(
        &self,
        issue_id: Option<impl AsRef<str>>,
        index_change_history: Option<bool>,
        index_worklogs: Option<bool>,
        index_comments: Option<bool>,
    ) -> HttpResult<ReindexBean> {
        let url = format!("{}{}", self.base_url, "/api/2/reindex/issue");
        let mut req = self.http_client.post(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = issue_id {
                query_params.push(("issueId", v.as_ref().to_string()));
            }
            if let Some(v) = index_change_history {
                query_params.push(("indexChangeHistory", v.to_string()));
            }
            if let Some(v) = index_worklogs {
                query_params.push(("indexWorklogs", v.to_string()));
            }
            if let Some(v) = index_comments {
                query_params.push(("indexComments", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /auth/1/websudo
    pub async fn release(&self, request: ReleaseRequest) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/auth/1/websudo");
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issuetypescheme/{schemeId}/associations
    pub async fn remove_all_project_associations(
        &self,
        scheme_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetypescheme/{}/associations",
            scheme_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/attachment/{id}
    pub async fn remove_attachment(&self, id: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/attachment/{}", id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/screens/{screenId}/tabs/{tabId}/fields/{id}
    pub async fn remove_field(
        &self,
        tab_id: i64,
        screen_id: i64,
        id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/screens/{}/tabs/{}/fields/{}", tab_id,
            screen_id, id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/group
    pub async fn remove_group(
        &self,
        groupname: impl AsRef<str>,
        swap_group: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/group");
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params.push(("groupname", groupname.as_ref().to_string()));
            if let Some(v) = swap_group {
                query_params.push(("swapGroup", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /agile/1.0/epic/none/issue
    pub async fn remove_issues_from_epic(
        &self,
        request: IssueAssignRequestBean,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/agile/1.0/epic/none/issue");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/mypreferences
    pub async fn remove_preference(
        &self,
        key: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/mypreferences");
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = key {
                query_params.push(("key", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issuetypescheme/{schemeId}/associations/{projIdOrKey}
    pub async fn remove_project_association(
        &self,
        proj_id_or_key: impl AsRef<str>,
        scheme_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetypescheme/{}/associations/{}",
            proj_id_or_key.as_ref(), scheme_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/projectCategory/{id}
    pub async fn remove_project_category(&self, id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/projectCategory/{}", id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/user
    pub async fn remove_user(
        &self,
        key: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user");
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = key {
                query_params.push(("key", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/user/application
    pub async fn remove_user_from_application_1(
        &self,
        application_key: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/application");
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = application_key {
                query_params.push(("applicationKey", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/group/user
    pub async fn remove_user_from_group(
        &self,
        groupname: impl AsRef<str>,
        username: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/group/user");
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params.push(("groupname", groupname.as_ref().to_string()));
            query_params.push(("username", username.as_ref().to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issue/{issueIdOrKey}/votes
    pub async fn remove_vote(&self, issue_id_or_key: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/votes", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/issue/{issueIdOrKey}/watchers
    pub async fn remove_watcher_1(
        &self,
        issue_id_or_key: impl AsRef<str>,
        user_name: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/watchers", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = user_name {
                query_params.push(("userName", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/screens/{screenId}/tabs/{tabId}
    pub async fn rename_tab(
        &self,
        tab_id: i64,
        screen_id: i64,
        request: ScreenableTabBean,
    ) -> HttpResult<ScreenableTabBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/screens/{}/tabs/{}", tab_id,
            screen_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/cluster/index-snapshot/{nodeId}
    pub async fn request_current_index_from_node(
        &self,
        node_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/cluster/index-snapshot/{}", node_id
            .as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/user/columns
    pub async fn reset_columns(
        &self,
        username: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/columns");
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/filter/{id}/columns
    pub async fn reset_columns_1(&self, id: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/filter/{}/columns", id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issueLinkType/order
    pub async fn reset_order(
        &self,
        request: IssueLinkTypeResetOrderRequest,
    ) -> HttpResult<IssueLinkTypesBean> {
        let url = format!("{}{}", self.base_url, "/api/2/issueLinkType/order");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issue/{issueIdOrKey}/restore
    pub async fn restore_issue(
        &self,
        issue_id_or_key: impl AsRef<str>,
        notify_users: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/restore", issue_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.put(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = notify_users {
                query_params.push(("notifyUsers", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/project/{projectIdOrKey}/restore
    pub async fn restore_project(
        &self,
        project_id_or_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/restore", project_id_or_key
            .as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/email-templates/revert
    pub async fn revert_email_templates_to_default(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/email-templates/revert");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/upgrade
    pub async fn run_upgrades_now(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/upgrade");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/user/anonymization
    pub async fn schedule_user_anonymization(
        &self,
        request: UserAnonymizationRequestBean,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/anonymization");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/user/anonymization/rerun
    pub async fn schedule_user_anonymization_rerun(
        &self,
        request: UserAnonymizationRerunRequestBean,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/anonymization/rerun");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/projects/picker
    pub async fn search_for_projects(
        &self,
        max_results: Option<i64>,
        query: Option<impl AsRef<str>>,
        allow_empty_query: Option<bool>,
    ) -> HttpResult<ProjectPickerResultWrapper> {
        let url = format!("{}{}", self.base_url, "/api/2/projects/picker");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = query {
                query_params.push(("query", v.as_ref().to_string()));
            }
            if let Some(v) = allow_empty_query {
                query_params.push(("allowEmptyQuery", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/search
    pub async fn search_using_search_request(
        &self,
        request: SearchRequestBean,
    ) -> HttpResult<SearchResultsBean> {
        let url = format!("{}{}", self.base_url, "/api/2/search");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/search
    pub async fn search_1(
        &self,
        expand: Option<impl AsRef<str>>,
        jql: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        validate_query: Option<bool>,
        fields: Option<impl AsRef<str>>,
        start_at: Option<i64>,
    ) -> HttpResult<SearchResultsBean> {
        let url = format!("{}{}", self.base_url, "/api/2/search");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = jql {
                query_params.push(("jql", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("maxResults", v.to_string()));
            }
            if let Some(v) = validate_query {
                query_params.push(("validateQuery", v.to_string()));
            }
            if let Some(v) = fields {
                query_params.push(("fields", v.as_ref().to_string()));
            }
            if let Some(v) = start_at {
                query_params.push(("startAt", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/project/{projectIdOrKey}/role/{id}
    pub async fn set_actors(
        &self,
        project_id_or_key: impl AsRef<str>,
        id: i64,
        request: ProjectRoleActorsUpdateBean,
    ) -> HttpResult<ProjectRoleBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/role/{}", project_id_or_key
            .as_ref(), id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/monitoring/app
    pub async fn set_app_monitoring_enabled(
        &self,
        request: AppMonitoringRestEntity,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/monitoring/app");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/monitoring/ipd
    pub async fn set_app_monitoring_enabled_1(
        &self,
        request: IpdMonitoringRestEntity,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/monitoring/ipd");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/settings/baseUrl
    pub async fn set_base_url(&self, request: SetBaseUrlRequest) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/settings/baseUrl");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/user/columns
    pub async fn set_columns_url_encoded(
        &self,
        request: SetColumnsUrlEncodedRequest,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/columns");
        let mut req = self
            .http_client
            .put(url)
            .body(
                serde_urlencoded::to_string(&request)
                    .map_err(HttpError::serialization_error)?,
            )
            .header("content-type", "application/x-www-form-urlencoded");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/filter/{id}/columns
    pub async fn set_columns_1(&self, id: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/filter/{}/columns", id.as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/filter/defaultShareScope
    pub async fn set_default_share_scope(
        &self,
        request: DefaultShareScopeBean,
    ) -> HttpResult<DefaultShareScopeBean> {
        let url = format!("{}{}", self.base_url, "/api/2/filter/defaultShareScope");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/workflowscheme/{id}/draft/issuetype/{issueType}
    pub async fn set_draft_issue_type(
        &self,
        issue_type: impl AsRef<str>,
        id: i64,
        request: IssueTypeMappingBean,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft/issuetype/{}",
            issue_type.as_ref(), id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/settings/columns
    pub async fn set_issue_navigator_default_columns_form(
        &self,
        request: SetIssueNavigatorDefaultColumnsFormRequest,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/settings/columns");
        let mut req = self
            .http_client
            .put(url)
            .body(
                serde_urlencoded::to_string(&request)
                    .map_err(HttpError::serialization_error)?,
            )
            .header("content-type", "application/x-www-form-urlencoded");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/workflowscheme/{id}/issuetype/{issueType}
    pub async fn set_issue_type(
        &self,
        issue_type: impl AsRef<str>,
        id: i64,
        request: IssueTypeMappingBean,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/issuetype/{}",
            issue_type.as_ref(), id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issue/{issueIdOrKey}/comment/{id}/pin
    pub async fn set_pin_comment(
        &self,
        issue_id_or_key: impl AsRef<str>,
        id: impl AsRef<str>,
        request: SetPinCommentRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/comment/{}/pin",
            issue_id_or_key.as_ref(), id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/mypreferences
    pub async fn set_preference(
        &self,
        key: Option<impl AsRef<str>>,
        request: SetPreferenceRequest,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/mypreferences");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = key {
                query_params.push(("key", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issuetypescheme/{schemeId}/associations
    pub async fn set_project_associations_for_scheme(
        &self,
        scheme_id: impl AsRef<str>,
        request: AssociateProjectsBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetypescheme/{}/associations",
            scheme_id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/dashboard/{dashboardId}/items/{itemId}/properties/{propertyKey}
    pub async fn set_property(
        &self,
        property_key: impl AsRef<str>,
        item_id: impl AsRef<str>,
        dashboard_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/dashboard/{}/items/{}/properties/{}",
            property_key.as_ref(), item_id.as_ref(), dashboard_id.as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/application-properties/{id}
    pub async fn set_property_via_restful_table(
        &self,
        id: impl AsRef<str>,
    ) -> HttpResult<Property> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/application-properties/{}", id
            .as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/comment/{commentId}/properties/{propertyKey}
    pub async fn set_property_1(
        &self,
        property_key: impl AsRef<str>,
        comment_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/comment/{}/properties/{}",
            property_key.as_ref(), comment_id.as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issue/{issueIdOrKey}/properties/{propertyKey}
    pub async fn set_property_2(
        &self,
        property_key: impl AsRef<str>,
        issue_id_or_key: impl AsRef<str>,
        request: SetProperty2Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/properties/{}", property_key
            .as_ref(), issue_id_or_key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issuetype/{issueTypeId}/properties/{propertyKey}
    pub async fn set_property_3(
        &self,
        property_key: impl AsRef<str>,
        issue_type_id: impl AsRef<str>,
        request: PropertyBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetype/{}/properties/{}",
            property_key.as_ref(), issue_type_id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/project/{projectIdOrKey}/properties/{propertyKey}
    pub async fn set_property_4(
        &self,
        property_key: impl AsRef<str>,
        project_id_or_key: impl AsRef<str>,
        request: PropertyBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/properties/{}",
            property_key.as_ref(), project_id_or_key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/user/properties/{propertyKey}
    pub async fn set_property_5(
        &self,
        property_key: impl AsRef<str>,
        user_key: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
        request: SetProperty5Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/user/properties/{}", property_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = user_key {
                query_params.push(("userKey", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/cluster/zdu/start
    pub async fn set_ready_to_upgrade(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/cluster/zdu/start");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /agile/1.0/board/{boardId}/settings/refined-velocity
    pub async fn set_refined_velocity(
        &self,
        board_id: i64,
        request: BooleanSettingBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/agile/1.0/board/{}/settings/refined-velocity", board_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/permissionscheme/{permissionSchemeId}/attribute/{key}
    pub async fn set_scheme_attribute(
        &self,
        permission_scheme_id: i64,
        key: impl AsRef<str>,
        body: String,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/permissionscheme/{}/attribute/{}",
            permission_scheme_id, key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(body)
            .header("content-type", "text/plain");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/terminology/entries
    pub async fn set_terminology_entries(
        &self,
        request: TerminologyRequestBean,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/terminology/entries");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/monitoring/jmx/startExposing
    pub async fn start(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/monitoring/jmx/startExposing");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/monitoring/jmx/stopExposing
    pub async fn stop(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/monitoring/jmx/stopExposing");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/issuetype/{id}/avatar/temporary
    pub async fn store_temporary_avatar_using_multi_part(
        &self,
        id: impl AsRef<str>,
        form: reqwest::multipart::Form,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetype/{}/avatar/temporary", id
            .as_ref())
        );
        let mut req = self.http_client.post(url).multipart(form);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/project/{projectIdOrKey}/avatar/temporary
    pub async fn store_temporary_avatar_using_multi_part_1(
        &self,
        project_id_or_key: impl AsRef<str>,
        form: reqwest::multipart::Form,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/avatar/temporary",
            project_id_or_key.as_ref())
        );
        let mut req = self.http_client.post(url).multipart(form);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/universal_avatar/type/{type}/owner/{owningObjectId}/temp
    pub async fn store_temporary_avatar_using_multi_part_2(
        &self,
        type_: impl AsRef<str>,
        owning_object_id: impl AsRef<str>,
        form: reqwest::multipart::Form,
    ) -> HttpResult<AvatarCroppingBean> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/2/universal_avatar/type/{}/owner/{}/temp", type_.as_ref(),
            owning_object_id.as_ref())
        );
        let mut req = self.http_client.post(url).multipart(form);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/user/avatar/temporary
    pub async fn store_temporary_avatar_using_multi_part_3(
        &self,
        username: Option<impl AsRef<str>>,
        form: reqwest::multipart::Form,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/avatar/temporary");
        let mut req = self.http_client.post(url).multipart(form);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /agile/1.0/sprint/{sprintId}/swap
    pub async fn swap_sprint(
        &self,
        sprint_id: i64,
        request: SprintSwapBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/sprint/{}/swap", sprint_id)
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/project/{projectKeyOrId}/priorityscheme/{schemeId}
    pub async fn unassign_priority_scheme(
        &self,
        scheme_id: i64,
        project_key_or_id: impl AsRef<str>,
    ) -> HttpResult<PrioritySchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/priorityscheme/{}",
            scheme_id, project_key_or_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/2/user/anonymization/unlock
    pub async fn unlock_anonymization(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/anonymization/unlock");
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /agile/1.0/sprint/unmap-all
    pub async fn unmap_all_sprints(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/agile/1.0/sprint/unmap-all");
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /agile/1.0/sprint/unmap
    pub async fn unmap_sprints(&self, request: UnmapSprintsBean) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/agile/1.0/sprint/unmap");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/workflowscheme/{id}
    pub async fn update(
        &self,
        id: i64,
        request: WorkflowSchemeBean,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}", id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issue/{issueIdOrKey}/comment/{id}
    pub async fn update_comment(
        &self,
        issue_id_or_key: impl AsRef<str>,
        id: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
        request: CommentJsonBean,
    ) -> HttpResult<CommentJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/comment/{}", issue_id_or_key
            .as_ref(), id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/component/{id}
    pub async fn update_component(
        &self,
        id: impl AsRef<str>,
        request: ComponentBean,
    ) -> HttpResult<ComponentBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/component/{}", id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/workflowscheme/{id}/default
    pub async fn update_default(
        &self,
        id: i64,
        request: DefaultBean,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/default", id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/workflowscheme/{id}/draft
    pub async fn update_draft(
        &self,
        id: i64,
        request: WorkflowSchemeBean,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft", id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/workflowscheme/{id}/draft/default
    pub async fn update_draft_default(
        &self,
        id: i64,
        request: DefaultBean,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft/default", id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/workflowscheme/{id}/draft/workflow
    pub async fn update_draft_workflow_mapping(
        &self,
        id: i64,
        workflow_name: Option<impl AsRef<str>>,
        request: WorkflowMappingBean,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/draft/workflow", id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = workflow_name {
                query_params.push(("workflowName", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issueLinkType/{issueLinkTypeId}
    pub async fn update_issue_link_type(
        &self,
        issue_link_type_id: impl AsRef<str>,
        request: IssueLinkTypeJsonBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issueLinkType/{}", issue_link_type_id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issuetype/{id}
    pub async fn update_issue_type(
        &self,
        id: impl AsRef<str>,
        request: IssueTypeUpdateBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetype/{}", id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issuetypescheme/{schemeId}
    pub async fn update_issue_type_scheme(
        &self,
        scheme_id: impl AsRef<str>,
        request: IssueTypeSchemeCreateUpdateBean,
    ) -> HttpResult<IssueTypeSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issuetypescheme/{}", scheme_id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/permissionscheme/{schemeId}
    pub async fn update_permission_scheme(
        &self,
        scheme_id: i64,
        expand: Option<impl AsRef<str>>,
        request: PermissionSchemeBean,
    ) -> HttpResult<PermissionSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/permissionscheme/{}", scheme_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/priorityschemes/{schemeId}
    pub async fn update_priority_scheme(
        &self,
        scheme_id: i64,
        request: PrioritySchemeUpdateBean,
    ) -> HttpResult<PrioritySchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/priorityschemes/{}", scheme_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/project/{projectIdOrKey}
    pub async fn update_project(
        &self,
        project_id_or_key: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
        request: ProjectUpdateBean,
    ) -> HttpResult<ProjectBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}", project_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/project/{projectIdOrKey}/avatar
    pub async fn update_project_avatar(
        &self,
        project_id_or_key: impl AsRef<str>,
        request: AvatarBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/avatar", project_id_or_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/projectCategory/{id}
    pub async fn update_project_category(
        &self,
        id: i64,
        request: ProjectCategoryBean,
    ) -> HttpResult<ProjectCategoryJsonBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/projectCategory/{}", id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/project/{projectIdOrKey}/type/{newProjectTypeKey}
    pub async fn update_project_type(
        &self,
        project_id_or_key: impl AsRef<str>,
        new_project_type_key: impl AsRef<str>,
    ) -> HttpResult<ProjectBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/project/{}/type/{}", project_id_or_key
            .as_ref(), new_project_type_key.as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issue/{issueIdOrKey}/remotelink/{linkId}
    pub async fn update_remote_issue_link(
        &self,
        link_id: impl AsRef<str>,
        issue_id_or_key: impl AsRef<str>,
        request: RemoteIssueLinkCreateOrUpdateRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/remotelink/{}", link_id
            .as_ref(), issue_id_or_key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/screens/{screenId}/tabs/{tabId}/fields/{id}/updateShowWhenEmptyIndicator/{newValue}
    pub async fn update_show_when_empty_indicator(
        &self,
        tab_id: i64,
        screen_id: i64,
        new_value: bool,
        id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/2/screens/{}/tabs/{}/fields/{}/updateShowWhenEmptyIndicator/{}",
            tab_id, screen_id, new_value, id.as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /agile/1.0/sprint/{sprintId}
    pub async fn update_sprint(
        &self,
        sprint_id: i64,
        request: SprintBean,
    ) -> HttpResult<SprintBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/agile/1.0/sprint/{}", sprint_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/myself
    pub async fn update_user(
        &self,
        request: UserWriteBean,
    ) -> HttpResult<UserWriteBean> {
        let url = format!("{}{}", self.base_url, "/api/2/myself");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/user/avatar
    pub async fn update_user_avatar_1(
        &self,
        username: Option<impl AsRef<str>>,
        request: AvatarBean,
    ) -> HttpResult<AvatarBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/avatar");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/user
    pub async fn update_user_1(
        &self,
        key: Option<impl AsRef<str>>,
        username: Option<impl AsRef<str>>,
        request: UserWriteBean,
    ) -> HttpResult<UserWriteBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = key {
                query_params.push(("key", v.as_ref().to_string()));
            }
            if let Some(v) = username {
                query_params.push(("username", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/version/{id}
    pub async fn update_version(
        &self,
        id: impl AsRef<str>,
        request: VersionBean,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/version/{}", id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/workflowscheme/{id}/workflow
    pub async fn update_workflow_mapping(
        &self,
        id: i64,
        workflow_name: Option<impl AsRef<str>>,
        request: WorkflowMappingBean,
    ) -> HttpResult<WorkflowSchemeBean> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/workflowscheme/{}/workflow", id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = workflow_name {
                query_params.push(("workflowName", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/2/issue/{issueIdOrKey}/worklog/{id}
    pub async fn update_worklog(
        &self,
        issue_id_or_key: impl AsRef<str>,
        id: impl AsRef<str>,
        new_estimate: Option<impl AsRef<str>>,
        adjust_estimate: Option<impl AsRef<str>>,
        request: Worklog,
    ) -> HttpResult<Worklog> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/2/issue/{}/worklog/{}", issue_id_or_key
            .as_ref(), id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = new_estimate {
                query_params.push(("newEstimate", v.as_ref().to_string()));
            }
            if let Some(v) = adjust_estimate {
                query_params.push(("adjustEstimate", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/email-templates
    pub async fn upload_email_templates(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/email-templates");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/2/licenseValidator
    pub async fn validate(
        &self,
        request: ValidateRequest,
    ) -> HttpResult<LicenseValidationResults> {
        let url = format!("{}{}", self.base_url, "/api/2/licenseValidator");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/anonymization
    pub async fn validate_user_anonymization(
        &self,
        expand: Option<impl AsRef<str>>,
        user_key: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/2/user/anonymization");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = user_key {
                query_params.push(("userKey", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/2/user/anonymization/rerun
    pub async fn validate_user_anonymization_rerun(
        &self,
        expand: Option<impl AsRef<str>>,
        old_user_key: Option<impl AsRef<str>>,
        old_user_name: Option<impl AsRef<str>>,
        user_key: Option<impl AsRef<str>>,
    ) -> HttpResult<UserAnonymizationValidationBean> {
        let url = format!("{}{}", self.base_url, "/api/2/user/anonymization/rerun");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = old_user_key {
                query_params.push(("oldUserKey", v.as_ref().to_string()));
            }
            if let Some(v) = old_user_name {
                query_params.push(("oldUserName", v.as_ref().to_string()));
            }
            if let Some(v) = user_key {
                query_params.push(("userKey", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
}
