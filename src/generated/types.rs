//! Generated types from OpenAPI specification
//!
//! This file contains all the generated types for the API.
//! Do not edit manually - regenerate using the appropriate script.
#![allow(clippy::large_enum_variant)]
#![allow(clippy::format_in_format_args)]
#![allow(clippy::let_unit_value)]
#![allow(unreachable_patterns)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inwardIssue", skip_serializing_if = "Option::is_none")]
    pub inward_issue: Option<IssueRefJsonBean>,
    #[serde(rename = "outwardIssue", skip_serializing_if = "Option::is_none")]
    pub outward_issue: Option<IssueRefJsonBean>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<IssueLinkTypeJsonBean>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorklogWithPaginationBean {
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worklogs: Option<Vec<Worklog>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Worklog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "issueId", skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
    #[serde(rename = "timeSpent", skip_serializing_if = "Option::is_none")]
    pub time_spent: Option<String>,
    #[serde(rename = "timeSpentSeconds", skip_serializing_if = "Option::is_none")]
    pub time_spent_seconds: Option<i64>,
    #[serde(rename = "updateAuthor", skip_serializing_if = "Option::is_none")]
    pub update_author: Option<UserJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<VisibilityJsonBean>,
}
pub type WorklogList = Vec<Worklog>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorklogChangedSinceBean {
    #[serde(rename = "isLastPage", skip_serializing_if = "Option::is_none")]
    pub is_last_page: Option<bool>,
    #[serde(rename = "lastPage", skip_serializing_if = "Option::is_none")]
    pub last_page: Option<bool>,
    #[serde(rename = "nextPage", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<WorklogChangeBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorklogChangeBean {
    #[serde(rename = "updatedTime", skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
    #[serde(rename = "worklogId", skip_serializing_if = "Option::is_none")]
    pub worklog_id: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowSchemeMigrationRequest {
    #[serde(rename = "schemeId", skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<i64>,
    #[serde(rename = "statusMappings", skip_serializing_if = "Option::is_none")]
    pub status_mappings: Option<Vec<StatusMapping>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusMapping {
    #[serde(rename = "issueTypeName", skip_serializing_if = "Option::is_none")]
    pub issue_type_name: Option<String>,
    #[serde(rename = "sourceStatusName", skip_serializing_if = "Option::is_none")]
    pub source_status_name: Option<String>,
    #[serde(rename = "targetStatusName", skip_serializing_if = "Option::is_none")]
    pub target_status_name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowSchemeBean {
    #[serde(rename = "defaultWorkflow", skip_serializing_if = "Option::is_none")]
    pub default_workflow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "issueTypeMappings", skip_serializing_if = "Option::is_none")]
    pub issue_type_mappings: Option<WorkflowSchemeBeanIssueTypeMappings>,
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<WorkflowSchemeBeanIssueTypes>,
    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "lastModifiedUser", skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<UserBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "originalDefaultWorkflow", skip_serializing_if = "Option::is_none")]
    pub original_default_workflow: Option<String>,
    #[serde(
        rename = "originalIssueTypeMappings",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_issue_type_mappings: Option<
        WorkflowSchemeBeanOriginalIssueTypeMappings,
    >,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "updateDraftIfNeeded", skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowSchemeBeanOriginalIssueTypeMappings {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowSchemeBeanIssueTypes {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowSchemeBeanIssueTypeMappings {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionMoveBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<VersionMoveBeanPosition>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum VersionMoveBeanPosition {
    #[default]
    #[serde(rename = "Earlier")]
    Earlier,
    #[serde(rename = "Later")]
    Later,
    #[serde(rename = "First")]
    First,
    #[serde(rename = "Last")]
    Last,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionIssueCountsBean {
    #[serde(rename = "customFieldNames", skip_serializing_if = "Option::is_none")]
    pub custom_field_names: Option<Vec<VersionUsageInCustomFields>>,
    #[serde(
        rename = "issueCountWithCustomFieldsShowingVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub issue_count_with_custom_fields_showing_version: Option<i64>,
    #[serde(rename = "issuesAffectedCount", skip_serializing_if = "Option::is_none")]
    pub issues_affected_count: Option<i64>,
    #[serde(rename = "issuesFixedCount", skip_serializing_if = "Option::is_none")]
    pub issues_fixed_count: Option<i64>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionUsageInCustomFields {
    #[serde(rename = "customFieldId", skip_serializing_if = "Option::is_none")]
    pub custom_field_id: Option<i64>,
    #[serde(rename = "fieldName", skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(
        rename = "issueCountWithVersionInCustomField",
        skip_serializing_if = "Option::is_none"
    )]
    pub issue_count_with_version_in_custom_field: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UsersAndGroupsBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<GroupSuggestionsBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<UserPickerResultsBean>,
}
pub type UserPickerResultsBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserIssueRelevanceBean {
    #[serde(rename = "avatarUrls", skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<UserIssueRelevanceBeanAvatarUrls>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(
        rename = "highestIssueInvolvementRank",
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_issue_involvement_rank: Option<i32>,
    #[serde(rename = "issueInvolvements", skip_serializing_if = "Option::is_none")]
    pub issue_involvements: Option<Vec<IssueInvolvementBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(
        rename = "latestCommentCreationTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_comment_creation_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserIssueRelevanceBeanAvatarUrls {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueInvolvementBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAnonymizationValidationBean {
    #[serde(rename = "affectedEntities", skip_serializing_if = "Option::is_none")]
    pub affected_entities: Option<UserAnonymizationValidationBeanAffectedEntities>,
    #[serde(
        rename = "businessLogicValidationFailed",
        skip_serializing_if = "Option::is_none"
    )]
    pub business_logic_validation_failed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<UserAnonymizationValidationBeanErrors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "userKey", skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<UserAnonymizationValidationBeanWarnings>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAnonymizationValidationBeanWarnings {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAnonymizationValidationBeanErrors {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAnonymizationValidationBeanAffectedEntities {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransitionsMetaBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<TransitionBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamPageBean {
    #[serde(rename = "isLast", skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "nextPage", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<StreamPageBeanValuesItem>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamPageBeanValuesItem {}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletRegistration {
    #[serde(rename = "className", skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(rename = "initParameters", skip_serializing_if = "Option::is_none")]
    pub init_parameters: Option<ServletRegistrationInitParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "runAsRole", skip_serializing_if = "Option::is_none")]
    pub run_as_role: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletRegistrationInitParameters {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecuritySchemesJsonBean {
    #[serde(rename = "issueSecuritySchemes", skip_serializing_if = "Option::is_none")]
    pub issue_security_schemes: Option<Vec<SecuritySchemeJsonBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecuritySchemeJsonBean {
    #[serde(rename = "defaultSecurityLevelId", skip_serializing_if = "Option::is_none")]
    pub default_security_level_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<SecurityLevelJsonBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecurityListLevelJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<SecurityLevelJsonBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecurityLevelJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResultsBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<IssueBean>>,
    ///The maximum number of results that can be retrieved from the underlying search engine, or null if unlimited
    #[serde(rename = "maxResultWindow", skip_serializing_if = "Option::is_none")]
    pub max_result_window: Option<i32>,
    ///The number of results to return in this page.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<SearchResultsBeanNames>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SearchResultsBeanSchema>,
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(rename = "warningMessages", skip_serializing_if = "Option::is_none")]
    pub warning_messages: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResultsBeanSchema {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResultsBeanNames {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RestWebhookConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<RestWebhookCredentials>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scopeType", skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    #[serde(rename = "sslVerificationRequired", skip_serializing_if = "Option::is_none")]
    pub ssl_verification_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<RestWebhookStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestWebhookStatistics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestWebhookCredentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestWebhookConfiguration {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoteReciprocalIssueLinkCreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<RemoteIssueLinkCreateOrUpdateResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<RemoteIssueLinkCreateOrUpdateResponse>,
}
pub type RemoteIssueLinkCreateOrUpdateResponse = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoteReciprocalIssueLinkCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<RemoteIssueLinkCreateOrUpdateRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<RemoteIssueLinkCreateOrUpdateRequest>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoteIssueLinkCreateOrUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
    #[serde(rename = "globalId", skip_serializing_if = "Option::is_none")]
    pub global_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<RemoteObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoteObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Icon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url16x16: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Application {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoteEntityLinksJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<RemoteEntityLinkJsonBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoteEntityLinkJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReindexRequestBean {
    #[serde(rename = "completionTime", skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReindexRequestBeanStatus>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<ReindexRequestBeanType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ReindexRequestBeanType {
    #[default]
    #[serde(rename = "IMMEDIATE")]
    Immediate,
    #[serde(rename = "DELAYED")]
    Delayed,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ReindexRequestBeanStatus {
    #[default]
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "COMPLETE")]
    Complete,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReindexBean {
    #[serde(rename = "currentProgress", skip_serializing_if = "Option::is_none")]
    pub current_progress: Option<i64>,
    #[serde(rename = "currentSubTask", skip_serializing_if = "Option::is_none")]
    pub current_sub_task: Option<String>,
    #[serde(rename = "finishTime", skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    #[serde(rename = "progressUrl", skip_serializing_if = "Option::is_none")]
    pub progress_url: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "submittedTime", skip_serializing_if = "Option::is_none")]
    pub submitted_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<ReindexBeanType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ReindexBeanType {
    #[default]
    #[serde(rename = "FOREGROUND")]
    Foreground,
    #[serde(rename = "BACKGROUND")]
    Background,
    #[serde(rename = "BACKGROUND_PREFFERED")]
    BackgroundPreffered,
    #[serde(rename = "BACKGROUND_PREFERRED")]
    BackgroundPreferred,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectUpdateBean {
    #[serde(rename = "assigneeType", skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<ProjectUpdateBeanAssigneeType>,
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "issueSecurityScheme", skip_serializing_if = "Option::is_none")]
    pub issue_security_scheme: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "notificationScheme", skip_serializing_if = "Option::is_none")]
    pub notification_scheme: Option<i64>,
    #[serde(rename = "permissionScheme", skip_serializing_if = "Option::is_none")]
    pub permission_scheme: Option<i64>,
    #[serde(rename = "projectTemplateKey", skip_serializing_if = "Option::is_none")]
    pub project_template_key: Option<String>,
    #[serde(rename = "projectTypeKey", skip_serializing_if = "Option::is_none")]
    pub project_type_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ProjectUpdateBeanAssigneeType {
    #[default]
    #[serde(rename = "PROJECT_LEAD")]
    ProjectLead,
    #[serde(rename = "UNASSIGNED")]
    Unassigned,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectRoleActorsUpdateBean {
    #[serde(rename = "categorisedActors", skip_serializing_if = "Option::is_none")]
    pub categorised_actors: Option<ProjectRoleActorsUpdateBeanCategorisedActors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectRoleActorsUpdateBeanCategorisedActors {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectRoleActorsBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actors: Option<Vec<RoleActorBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectPickerResultWrapper {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectPickerItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectPickerItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectJsonBean {
    #[serde(rename = "avatarUrls", skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<ProjectJsonBeanAvatarUrls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectCategory", skip_serializing_if = "Option::is_none")]
    pub project_category: Option<ProjectCategoryJsonBean>,
    #[serde(rename = "projectTypeKey", skip_serializing_if = "Option::is_none")]
    pub project_type_key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectJsonBeanAvatarUrls {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectCategoryJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectInputBean {
    #[serde(rename = "assigneeType", skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<ProjectInputBeanAssigneeType>,
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "issueSecurityScheme", skip_serializing_if = "Option::is_none")]
    pub issue_security_scheme: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "notificationScheme", skip_serializing_if = "Option::is_none")]
    pub notification_scheme: Option<i64>,
    #[serde(rename = "permissionScheme", skip_serializing_if = "Option::is_none")]
    pub permission_scheme: Option<i64>,
    #[serde(rename = "projectTemplateKey", skip_serializing_if = "Option::is_none")]
    pub project_template_key: Option<String>,
    #[serde(rename = "projectTypeKey", skip_serializing_if = "Option::is_none")]
    pub project_type_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "workflowSchemeId", skip_serializing_if = "Option::is_none")]
    pub workflow_scheme_id: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ProjectInputBeanAssigneeType {
    #[default]
    #[serde(rename = "PROJECT_LEAD")]
    ProjectLead,
    #[serde(rename = "UNASSIGNED")]
    Unassigned,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductLicenseTO {
    #[serde(
        rename = "isUnlimitedNumberOfUsers",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_unlimited_number_of_users: Option<bool>,
    #[serde(rename = "licenseKey", skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<ProductLicenseTOLocale>,
    #[serde(rename = "numberOfUsers", skip_serializing_if = "Option::is_none")]
    pub number_of_users: Option<i32>,
    #[serde(rename = "productDisplayName", skip_serializing_if = "Option::is_none")]
    pub product_display_name: Option<String>,
    #[serde(rename = "productKey", skip_serializing_if = "Option::is_none")]
    pub product_key: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductLicenseTOLocale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "displayCountry", skip_serializing_if = "Option::is_none")]
    pub display_country: Option<String>,
    #[serde(rename = "displayLanguage", skip_serializing_if = "Option::is_none")]
    pub display_language: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayScript", skip_serializing_if = "Option::is_none")]
    pub display_script: Option<String>,
    #[serde(rename = "displayVariant", skip_serializing_if = "Option::is_none")]
    pub display_variant: Option<String>,
    #[serde(rename = "extensionKeys", skip_serializing_if = "Option::is_none")]
    pub extension_keys: Option<Vec<String>>,
    #[serde(rename = "iso3Country", skip_serializing_if = "Option::is_none")]
    pub iso3_country: Option<String>,
    #[serde(rename = "iso3Language", skip_serializing_if = "Option::is_none")]
    pub iso3_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(rename = "unicodeLocaleAttributes", skip_serializing_if = "Option::is_none")]
    pub unicode_locale_attributes: Option<Vec<String>>,
    #[serde(rename = "unicodeLocaleKeys", skip_serializing_if = "Option::is_none")]
    pub unicode_locale_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrioritySchemeListBean {
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemes: Option<Vec<PrioritySchemeBean>>,
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrioritySchemeBean {
    #[serde(rename = "defaultOptionId", skip_serializing_if = "Option::is_none")]
    pub default_option_id: Option<String>,
    #[serde(rename = "defaultScheme", skip_serializing_if = "Option::is_none")]
    pub default_scheme: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "optionIds", skip_serializing_if = "Option::is_none")]
    pub option_ids: Option<Vec<String>>,
    #[serde(rename = "projectKeys", skip_serializing_if = "Option::is_none")]
    pub project_keys: Option<Vec<String>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PinnedCommentJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<CommentJsonBean>,
    #[serde(rename = "pinnedBy", skip_serializing_if = "Option::is_none")]
    pub pinned_by: Option<String>,
    #[serde(rename = "pinnedDate", skip_serializing_if = "Option::is_none")]
    pub pinned_date: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionsJsonBean {
    ///A map of permission keys to permission objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsJsonBeanPermissions>,
}
///A map of permission keys to permission objects.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionsJsonBeanPermissions {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionsInputBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<GrantToPermissionInputBean>>,
    #[serde(rename = "permissionKeys", skip_serializing_if = "Option::is_none")]
    pub permission_keys: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionSchemesBean {
    #[serde(rename = "permissionSchemes", skip_serializing_if = "Option::is_none")]
    pub permission_schemes: Option<Vec<PermissionSchemeBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionSchemeBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionGrantBean>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionGrantsBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionGrantBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionGrantBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<PermissionHolderBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionHolderBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<FieldBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(rename = "projectRole", skip_serializing_if = "Option::is_none")]
    pub project_role: Option<ProjectRoleBean>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserJsonBean>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartialSuccessBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<Entry>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    #[serde(rename = "issueId", skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<i64>,
    #[serde(rename = "issueKey", skip_serializing_if = "Option::is_none")]
    pub issue_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageBean {
    #[serde(rename = "isLast", skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextPage", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<PageBeanValuesItem>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageBeanValuesItem {}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderByOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<OrderByOption>>,
    #[serde(rename = "matchesCount", skip_serializing_if = "Option::is_none")]
    pub matches_count: Option<i32>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderByOption {
    #[serde(rename = "fieldId", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(rename = "fieldName", skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "sortJql", skip_serializing_if = "Option::is_none")]
    pub sort_jql: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OptionsSettings {
    #[serde(rename = "issueContext", skip_serializing_if = "Option::is_none")]
    pub issue_context: Option<IssueContextParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Box<OptionBasic>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueContextParam {
    #[serde(rename = "issueTypeId", skip_serializing_if = "Option::is_none")]
    pub issue_type_id: Option<String>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OptionBasic {
    #[serde(rename = "childOptions", skip_serializing_if = "Option::is_none")]
    pub child_options: Option<Vec<Box<OptionBasic>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "optionId", skip_serializing_if = "Option::is_none")]
    pub option_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotificationJsonBean {
    #[serde(rename = "htmlBody", skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict: Option<RestrictJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "textBody", skip_serializing_if = "Option::is_none")]
    pub text_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<ToJsonBean>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupJsonBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserJsonBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchers: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestrictJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupJsonBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionJsonBean>>,
}
///A map of permission keys to permission objects.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<PermissionJsonBeanType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum PermissionJsonBeanType {
    #[default]
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "PROJECT")]
    Project,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alive: Option<bool>,
    #[serde(rename = "cacheListenerPort", skip_serializing_if = "Option::is_none")]
    pub cache_listener_port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(
        rename = "lastStateChangeTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_state_change_timestamp: Option<i64>,
    #[serde(rename = "nodeBuildNumber", skip_serializing_if = "Option::is_none")]
    pub node_build_number: Option<i64>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "nodeVersion", skip_serializing_if = "Option::is_none")]
    pub node_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<NodeBeanState>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NodeBeanState {
    #[default]
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "PASSIVE")]
    Passive,
    #[serde(rename = "ACTIVATING")]
    Activating,
    #[serde(rename = "PASSIVATING")]
    Passivating,
    #[serde(rename = "OFFLINE")]
    Offline,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MoveFieldBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<MoveFieldBeanPosition>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MoveFieldBeanPosition {
    #[default]
    #[serde(rename = "Earlier")]
    Earlier,
    #[serde(rename = "Later")]
    Later,
    #[serde(rename = "First")]
    First,
    #[serde(rename = "Last")]
    Last,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinkIssueRequestJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<CommentJsonBean>,
    #[serde(rename = "inwardIssue", skip_serializing_if = "Option::is_none")]
    pub inward_issue: Option<IssueRefJsonBean>,
    #[serde(rename = "outwardIssue", skip_serializing_if = "Option::is_none")]
    pub outward_issue: Option<IssueRefJsonBean>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<IssueLinkTypeJsonBean>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueLinkTypeJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inward: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outward: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LicenseValidationResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<LicenseValidationResultsErrors>,
    #[serde(rename = "licenseString", skip_serializing_if = "Option::is_none")]
    pub license_string: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LicenseValidationResultsErrors {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssuesUpdateBean {
    #[serde(rename = "issueUpdates", skip_serializing_if = "Option::is_none")]
    pub issue_updates: Option<Vec<IssueUpdateBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssuesCreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BulkOperationErrorResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<IssueCreateResponse>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueCreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueUpdateBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<IssueUpdateBeanFields>,
    #[serde(rename = "historyMetadata", skip_serializing_if = "Option::is_none")]
    pub history_metadata: Option<HistoryMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<EntityPropertyBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<TransitionBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<IssueUpdateBeanUpdate>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueUpdateBeanUpdate {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueUpdateBeanFields {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueTypeWithStatusJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<StatusJsonBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueTypeSchemeListBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemes: Option<Vec<IssueTypeSchemeBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueTypeSchemeBean {
    #[serde(rename = "defaultIssueType", skip_serializing_if = "Option::is_none")]
    pub default_issue_type: Option<IssueTypeJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<IssueTypeJsonBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueTypeCreateBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<IssueTypeCreateBeanType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum IssueTypeCreateBeanType {
    #[default]
    #[serde(rename = "subtask")]
    Subtask,
    #[serde(rename = "standard")]
    Standard,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueRefJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Fields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssuePickerResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<IssueSection>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<IssuePickerIssue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssuePickerIssue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "keyHtml", skip_serializing_if = "Option::is_none")]
    pub key_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "summaryText", skip_serializing_if = "Option::is_none")]
    pub summary_text: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changelog: Option<ChangelogBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editmeta: Option<EditMetaBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<IssueBeanFields>,
    #[serde(rename = "fieldsToInclude", skip_serializing_if = "Option::is_none")]
    pub fields_to_include: Option<IncludedFields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<IssueBeanNames>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<OpsbarBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesBean>,
    #[serde(rename = "renderedFields", skip_serializing_if = "Option::is_none")]
    pub rendered_fields: Option<IssueBeanRenderedFields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<IssueBeanSchema>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "transitionBeans", skip_serializing_if = "Option::is_none")]
    pub transition_beans: Option<Vec<TransitionBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<TransitionBean>>,
    #[serde(
        rename = "versionedRepresentations",
        skip_serializing_if = "Option::is_none"
    )]
    pub versioned_representations: Option<IssueBeanVersionedRepresentations>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueBeanVersionedRepresentations {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransitionBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<TransitionBeanFields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "opsbarSequence", skip_serializing_if = "Option::is_none")]
    pub opsbar_sequence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<StatusJsonBean>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransitionBeanFields {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueBeanSchema {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueBeanRenderedFields {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertiesBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesBeanProperties>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertiesBeanProperties {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpsbarBean {
    #[serde(rename = "linkGroups", skip_serializing_if = "Option::is_none")]
    pub link_groups: Option<Vec<Box<LinkGroupBean>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinkGroupBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Box<LinkGroupBean>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<SimpleLinkBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<SimpleLinkBean>>,
    #[serde(rename = "styleClass", skip_serializing_if = "Option::is_none")]
    pub style_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleLinkBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "iconClass", skip_serializing_if = "Option::is_none")]
    pub icon_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<SimpleLinkBeanParams>,
    #[serde(rename = "styleClass", skip_serializing_if = "Option::is_none")]
    pub style_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleLinkBeanParams {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueBeanNames {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueBeanFields {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncludedFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<String>>,
}
pub type EditMetaBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexSummaryBean {
    #[serde(
        rename = "externalPlatformIndexReplay",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_platform_index_replay: Option<
        IndexSummaryBeanExternalPlatformIndexReplay,
    >,
    #[serde(rename = "issueIndex", skip_serializing_if = "Option::is_none")]
    pub issue_index: Option<IssueIndexSummaryBean>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "replicationQueues", skip_serializing_if = "Option::is_none")]
    pub replication_queues: Option<IndexSummaryBeanReplicationQueues>,
    #[serde(rename = "reportTime", skip_serializing_if = "Option::is_none")]
    pub report_time: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexSummaryBeanReplicationQueues {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueIndexSummaryBean {
    #[serde(rename = "countInArchive", skip_serializing_if = "Option::is_none")]
    pub count_in_archive: Option<i64>,
    #[serde(rename = "countInDatabase", skip_serializing_if = "Option::is_none")]
    pub count_in_database: Option<i64>,
    #[serde(rename = "countInIndex", skip_serializing_if = "Option::is_none")]
    pub count_in_index: Option<i64>,
    #[serde(rename = "indexReadable", skip_serializing_if = "Option::is_none")]
    pub index_readable: Option<bool>,
    #[serde(rename = "lastUpdatedInDatabase", skip_serializing_if = "Option::is_none")]
    pub last_updated_in_database: Option<String>,
    #[serde(rename = "lastUpdatedInIndex", skip_serializing_if = "Option::is_none")]
    pub last_updated_in_index: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexSummaryBeanExternalPlatformIndexReplay {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexReplicationQueueSummaryBean {
    #[serde(rename = "lastConsumedOperation", skip_serializing_if = "Option::is_none")]
    pub last_consumed_operation: Option<IndexReplicationQueueEntryBean>,
    #[serde(rename = "lastOperationInQueue", skip_serializing_if = "Option::is_none")]
    pub last_operation_in_queue: Option<IndexReplicationQueueEntryBean>,
    #[serde(rename = "queueSize", skip_serializing_if = "Option::is_none")]
    pub queue_size: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexReplicationQueueEntryBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "replicationTime", skip_serializing_if = "Option::is_none")]
    pub replication_time: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpServletResponse {
    #[serde(rename = "bufferSize", skip_serializing_if = "Option::is_none")]
    pub buffer_size: Option<i32>,
    #[serde(rename = "characterEncoding", skip_serializing_if = "Option::is_none")]
    pub character_encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committed: Option<bool>,
    #[serde(rename = "contentLength", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i32>,
    #[serde(rename = "contentLengthLong", skip_serializing_if = "Option::is_none")]
    pub content_length_long: Option<i64>,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "headerNames", skip_serializing_if = "Option::is_none")]
    pub header_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<HttpServletResponseLocale>,
    #[serde(rename = "outputStream", skip_serializing_if = "Option::is_none")]
    pub output_stream: Option<ServletOutputStream>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "trailerFields", skip_serializing_if = "Option::is_none")]
    pub trailer_fields: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writer: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpServletResponseLocale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "displayCountry", skip_serializing_if = "Option::is_none")]
    pub display_country: Option<String>,
    #[serde(rename = "displayLanguage", skip_serializing_if = "Option::is_none")]
    pub display_language: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayScript", skip_serializing_if = "Option::is_none")]
    pub display_script: Option<String>,
    #[serde(rename = "displayVariant", skip_serializing_if = "Option::is_none")]
    pub display_variant: Option<String>,
    #[serde(rename = "extensionKeys", skip_serializing_if = "Option::is_none")]
    pub extension_keys: Option<Vec<String>>,
    #[serde(rename = "iso3Country", skip_serializing_if = "Option::is_none")]
    pub iso3_country: Option<String>,
    #[serde(rename = "iso3Language", skip_serializing_if = "Option::is_none")]
    pub iso3_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(rename = "unicodeLocaleAttributes", skip_serializing_if = "Option::is_none")]
    pub unicode_locale_attributes: Option<Vec<String>>,
    #[serde(rename = "unicodeLocaleKeys", skip_serializing_if = "Option::is_none")]
    pub unicode_locale_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpServletRequest {
    #[serde(rename = "asyncContext", skip_serializing_if = "Option::is_none")]
    pub async_context: Option<Box<AsyncContext>>,
    #[serde(rename = "asyncStarted", skip_serializing_if = "Option::is_none")]
    pub async_started: Option<bool>,
    #[serde(rename = "asyncSupported", skip_serializing_if = "Option::is_none")]
    pub async_supported: Option<bool>,
    #[serde(rename = "attributeNames", skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<serde_json::Value>,
    #[serde(rename = "authType", skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "characterEncoding", skip_serializing_if = "Option::is_none")]
    pub character_encoding: Option<String>,
    #[serde(rename = "contentLength", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i32>,
    #[serde(rename = "contentLengthLong", skip_serializing_if = "Option::is_none")]
    pub content_length_long: Option<i64>,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "contextPath", skip_serializing_if = "Option::is_none")]
    pub context_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<Cookie>>,
    #[serde(rename = "dispatcherType", skip_serializing_if = "Option::is_none")]
    pub dispatcher_type: Option<HttpServletRequestDispatcherType>,
    #[serde(rename = "headerNames", skip_serializing_if = "Option::is_none")]
    pub header_names: Option<serde_json::Value>,
    #[serde(rename = "httpServletMapping", skip_serializing_if = "Option::is_none")]
    pub http_servlet_mapping: Option<HttpServletMapping>,
    #[serde(rename = "inputStream", skip_serializing_if = "Option::is_none")]
    pub input_stream: Option<ServletInputStream>,
    #[serde(rename = "localAddr", skip_serializing_if = "Option::is_none")]
    pub local_addr: Option<String>,
    #[serde(rename = "localName", skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    #[serde(rename = "localPort", skip_serializing_if = "Option::is_none")]
    pub local_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<HttpServletRequestLocale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locales: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "parameterMap", skip_serializing_if = "Option::is_none")]
    pub parameter_map: Option<HttpServletRequestParameterMap>,
    #[serde(rename = "parameterNames", skip_serializing_if = "Option::is_none")]
    pub parameter_names: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Part>>,
    #[serde(rename = "pathInfo", skip_serializing_if = "Option::is_none")]
    pub path_info: Option<String>,
    #[serde(rename = "pathTranslated", skip_serializing_if = "Option::is_none")]
    pub path_translated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "protocolRequestId", skip_serializing_if = "Option::is_none")]
    pub protocol_request_id: Option<String>,
    #[serde(rename = "queryString", skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader: Option<serde_json::Value>,
    #[serde(rename = "remoteAddr", skip_serializing_if = "Option::is_none")]
    pub remote_addr: Option<String>,
    #[serde(rename = "remoteHost", skip_serializing_if = "Option::is_none")]
    pub remote_host: Option<String>,
    #[serde(rename = "remotePort", skip_serializing_if = "Option::is_none")]
    pub remote_port: Option<i32>,
    #[serde(rename = "remoteUser", skip_serializing_if = "Option::is_none")]
    pub remote_user: Option<String>,
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "requestURI", skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[serde(rename = "requestURL", skip_serializing_if = "Option::is_none")]
    pub request_url: Option<HttpServletRequestRequestURL>,
    #[serde(rename = "requestedSessionId", skip_serializing_if = "Option::is_none")]
    pub requested_session_id: Option<String>,
    #[serde(
        rename = "requestedSessionIdFromCookie",
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_session_id_from_cookie: Option<bool>,
    #[serde(
        rename = "requestedSessionIdFromURL",
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_session_id_from_url: Option<bool>,
    #[serde(rename = "requestedSessionIdValid", skip_serializing_if = "Option::is_none")]
    pub requested_session_id_valid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(rename = "serverName", skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "serverPort", skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
    #[serde(rename = "servletConnection", skip_serializing_if = "Option::is_none")]
    pub servlet_connection: Option<ServletConnection>,
    #[serde(rename = "servletContext", skip_serializing_if = "Option::is_none")]
    pub servlet_context: Option<ServletContext>,
    #[serde(rename = "servletPath", skip_serializing_if = "Option::is_none")]
    pub servlet_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<HttpSession>,
    #[serde(rename = "trailerFields", skip_serializing_if = "Option::is_none")]
    pub trailer_fields: Option<HttpServletRequestTrailerFields>,
    #[serde(rename = "trailerFieldsReady", skip_serializing_if = "Option::is_none")]
    pub trailer_fields_ready: Option<bool>,
    #[serde(rename = "userPrincipal", skip_serializing_if = "Option::is_none")]
    pub user_principal: Option<HttpServletRequestUserPrincipal>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpServletRequestUserPrincipal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpServletRequestTrailerFields {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpSession {
    #[serde(rename = "attributeNames", skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<serde_json::Value>,
    #[serde(rename = "creationTime", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastAccessedTime", skip_serializing_if = "Option::is_none")]
    pub last_accessed_time: Option<i64>,
    #[serde(rename = "maxInactiveInterval", skip_serializing_if = "Option::is_none")]
    pub max_inactive_interval: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(rename = "servletContext", skip_serializing_if = "Option::is_none")]
    pub servlet_context: Option<ServletContext>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpServletRequestRequestURL {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Part {
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "headerNames", skip_serializing_if = "Option::is_none")]
    pub header_names: Option<Vec<String>>,
    #[serde(rename = "inputStream", skip_serializing_if = "Option::is_none")]
    pub input_stream: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "submittedFileName", skip_serializing_if = "Option::is_none")]
    pub submitted_file_name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpServletRequestParameterMap {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpServletRequestLocale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "displayCountry", skip_serializing_if = "Option::is_none")]
    pub display_country: Option<String>,
    #[serde(rename = "displayLanguage", skip_serializing_if = "Option::is_none")]
    pub display_language: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayScript", skip_serializing_if = "Option::is_none")]
    pub display_script: Option<String>,
    #[serde(rename = "displayVariant", skip_serializing_if = "Option::is_none")]
    pub display_variant: Option<String>,
    #[serde(rename = "extensionKeys", skip_serializing_if = "Option::is_none")]
    pub extension_keys: Option<Vec<String>>,
    #[serde(rename = "iso3Country", skip_serializing_if = "Option::is_none")]
    pub iso3_country: Option<String>,
    #[serde(rename = "iso3Language", skip_serializing_if = "Option::is_none")]
    pub iso3_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(rename = "unicodeLocaleAttributes", skip_serializing_if = "Option::is_none")]
    pub unicode_locale_attributes: Option<Vec<String>>,
    #[serde(rename = "unicodeLocaleKeys", skip_serializing_if = "Option::is_none")]
    pub unicode_locale_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum HttpServletRequestDispatcherType {
    #[default]
    #[serde(rename = "FORWARD")]
    Forward,
    #[serde(rename = "INCLUDE")]
    Include,
    #[serde(rename = "REQUEST")]
    Request,
    #[serde(rename = "ASYNC")]
    Async_,
    #[serde(rename = "ERROR")]
    Error,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpServletMapping {
    #[serde(rename = "mappingMatch", skip_serializing_if = "Option::is_none")]
    pub mapping_match: Option<HttpServletMappingMappingMatch>,
    #[serde(rename = "matchValue", skip_serializing_if = "Option::is_none")]
    pub match_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "servletName", skip_serializing_if = "Option::is_none")]
    pub servlet_name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum HttpServletMappingMappingMatch {
    #[default]
    #[serde(rename = "CONTEXT_ROOT")]
    ContextRoot,
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "EXACT")]
    Exact,
    #[serde(rename = "EXTENSION")]
    Extension,
    #[serde(rename = "PATH")]
    Path,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupSuggestionsBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupSuggestionBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupSuggestionBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<GroupLabelBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupLabelBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<GroupLabelBeanType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum GroupLabelBeanType {
    #[default]
    #[serde(rename = "ADMIN")]
    Admin,
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "MULTIPLE")]
    Multiple,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<PagedListWrapperUserJsonBeanApplicationUser>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PagedListWrapperUserJsonBeanApplicationUser {
    #[serde(rename = "backingListSize", skip_serializing_if = "Option::is_none")]
    pub backing_list_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackUserJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UserJsonBean>>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "pagingCallback", skip_serializing_if = "Option::is_none")]
    pub paging_callback: Option<ListWrapperCallbackUserJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
pub type ListWrapperCallbackUserJsonBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrantToPermissionInputBean {
    #[serde(rename = "securityType", skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<OptionString>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OptionString {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(rename = "orNull", skip_serializing_if = "Option::is_none")]
    pub or_null: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Gadget {
    #[serde(rename = "filterId", skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<i64>,
    #[serde(rename = "filterName", skip_serializing_if = "Option::is_none")]
    pub filter_name: Option<String>,
    #[serde(rename = "gadgetUri", skip_serializing_if = "Option::is_none")]
    pub gadget_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    #[serde(rename = "portalId", skip_serializing_if = "Option::is_none")]
    pub portal_id: Option<i64>,
    #[serde(rename = "userPrefs", skip_serializing_if = "Option::is_none")]
    pub user_prefs: Option<GadgetUserPrefs>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GadgetUserPrefs {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilterRegistration {
    #[serde(rename = "className", skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(rename = "initParameters", skip_serializing_if = "Option::is_none")]
    pub init_parameters: Option<FilterRegistrationInitParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "servletNameMappings", skip_serializing_if = "Option::is_none")]
    pub servlet_name_mappings: Option<Vec<String>>,
    #[serde(rename = "urlPatternMappings", skip_serializing_if = "Option::is_none")]
    pub url_pattern_mappings: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilterRegistrationInitParameters {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilterBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserBean>,
    #[serde(rename = "searchUrl", skip_serializing_if = "Option::is_none")]
    pub search_url: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "sharePermissions", skip_serializing_if = "Option::is_none")]
    pub share_permissions: Option<Vec<FilterPermissionBean>>,
    #[serde(rename = "sharedUsers", skip_serializing_if = "Option::is_none")]
    pub shared_users: Option<UserBeanListWrapper>,
    #[serde(rename = "viewUrl", skip_serializing_if = "Option::is_none")]
    pub view_url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserBeanListWrapper {
    #[serde(rename = "backingListSize", skip_serializing_if = "Option::is_none")]
    pub backing_list_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackUserBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UserBean>>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "pagingCallback", skip_serializing_if = "Option::is_none")]
    pub paging_callback: Option<ListWrapperCallbackUserBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
pub type ListWrapperCallbackUserBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilterPermissionBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<ProjectRoleBean>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectRoleBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actors: Option<Vec<RoleActorBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoleActorBean {
    #[serde(rename = "avatarUrl", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuetype: Option<IssueTypeJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<PriorityJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "statusCategory", skip_serializing_if = "Option::is_none")]
    pub status_category: Option<StatusCategoryJsonBean>,
    #[serde(rename = "statusColor", skip_serializing_if = "Option::is_none")]
    pub status_color: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusCategoryJsonBean {
    #[serde(rename = "colorName", skip_serializing_if = "Option::is_none")]
    pub color_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriorityJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "statusColor", skip_serializing_if = "Option::is_none")]
    pub status_color: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldMetaBean {
    #[serde(rename = "allowedValues", skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<FieldMetaBeanAllowedValuesItem>>,
    #[serde(rename = "autoCompleteUrl", skip_serializing_if = "Option::is_none")]
    pub auto_complete_url: Option<String>,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(rename = "fieldId", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(rename = "hasDefaultValue", skip_serializing_if = "Option::is_none")]
    pub has_default_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<JsonTypeBean>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldMetaBeanAllowedValuesItem {}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldConfigSchemeBean {
    #[serde(rename = "allIssueTypes", skip_serializing_if = "Option::is_none")]
    pub all_issue_types: Option<bool>,
    #[serde(rename = "allProjects", skip_serializing_if = "Option::is_none")]
    pub all_projects: Option<bool>,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<FieldBean>,
    #[serde(rename = "fieldConfigIds", skip_serializing_if = "Option::is_none")]
    pub field_config_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<IssueTypeJsonBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectBean>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueTypeJsonBean {
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldBean {
    #[serde(rename = "clauseNames", skip_serializing_if = "Option::is_none")]
    pub clause_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<JsonTypeBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonTypeBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
    #[serde(rename = "customId", skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExternalPlatformIndexReplaySummaryBean {
    #[serde(rename = "lastConsumedOperation", skip_serializing_if = "Option::is_none")]
    pub last_consumed_operation: Option<ExternalPlatformIndexReplayEntryBean>,
    #[serde(rename = "lastOperationInQueue", skip_serializing_if = "Option::is_none")]
    pub last_operation_in_queue: Option<ExternalPlatformIndexReplayEntryBean>,
    #[serde(rename = "queueSize", skip_serializing_if = "Option::is_none")]
    pub queue_size: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExternalPlatformIndexReplayEntryBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "journalWriteTime", skip_serializing_if = "Option::is_none")]
    pub journal_write_time: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpicUpdateBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpicBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityPropertiesKeysBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<EntityPropertyKeyBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityPropertyKeyBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<EntityRefBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityRefBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<EntityTypeBean>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityTypeBean {
    #[serde(
        rename = "applicationTypeClassName",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_type_class_name: Option<String>,
    #[serde(rename = "i18nKey", skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "pluralizedI18nKey", skip_serializing_if = "Option::is_none")]
    pub pluralized_i18n_key: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteAndReplaceVersionBean {
    #[serde(
        rename = "customFieldReplacementList",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_field_replacement_list: Option<Vec<CustomFieldReplacement>>,
    #[serde(rename = "moveAffectedIssuesTo", skip_serializing_if = "Option::is_none")]
    pub move_affected_issues_to: Option<i64>,
    #[serde(rename = "moveFixIssuesTo", skip_serializing_if = "Option::is_none")]
    pub move_fix_issues_to: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomFieldReplacement {
    #[serde(rename = "customFieldId", skip_serializing_if = "Option::is_none")]
    pub custom_field_id: Option<i64>,
    #[serde(rename = "moveTo", skip_serializing_if = "Option::is_none")]
    pub move_to: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DefaultShareScopeBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<DefaultShareScopeBeanScope>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum DefaultShareScopeBeanScope {
    #[default]
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "AUTHENTICATED")]
    Authenticated,
    #[serde(rename = "PRIVATE")]
    Private,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DashboardsBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<DashboardBean>>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DashboardBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DarkFeaturesBean {
    #[serde(rename = "siteFeatures", skip_serializing_if = "Option::is_none")]
    pub site_features: Option<DarkFeaturesBeanSiteFeatures>,
    #[serde(rename = "systemFeatures", skip_serializing_if = "Option::is_none")]
    pub system_features: Option<DarkFeaturesBeanSystemFeatures>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DarkFeaturesBeanSystemFeatures {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DarkFeaturesBeanSiteFeatures {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomFieldOptionsBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CustomFieldOptionBean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomFieldOptionBean {
    #[serde(rename = "childrenIds", skip_serializing_if = "Option::is_none")]
    pub children_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateMetaIssueTypeBean {
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<CreateMetaIssueTypeBeanFields>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateMetaIssueTypeBeanFields {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateIndexBean {
    #[serde(rename = "entityName", skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    #[serde(rename = "fieldNameToColumnName", skip_serializing_if = "Option::is_none")]
    pub field_name_to_column_name: Option<CreateIndexBeanFieldNameToColumnName>,
    #[serde(rename = "indexName", skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "tableName", skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateIndexBeanFieldNameToColumnName {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cookie {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CookieAttributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "httpOnly", skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    #[serde(rename = "maxAge", skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CookieAttributes {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigurationBean {
    #[serde(rename = "attachmentsEnabled", skip_serializing_if = "Option::is_none")]
    pub attachments_enabled: Option<bool>,
    #[serde(rename = "issueLinkingEnabled", skip_serializing_if = "Option::is_none")]
    pub issue_linking_enabled: Option<bool>,
    #[serde(rename = "subTasksEnabled", skip_serializing_if = "Option::is_none")]
    pub sub_tasks_enabled: Option<bool>,
    #[serde(
        rename = "timeTrackingConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_tracking_configuration: Option<TimeTrackingConfigurationBean>,
    #[serde(rename = "timeTrackingEnabled", skip_serializing_if = "Option::is_none")]
    pub time_tracking_enabled: Option<bool>,
    #[serde(rename = "unassignedIssuesAllowed", skip_serializing_if = "Option::is_none")]
    pub unassigned_issues_allowed: Option<bool>,
    #[serde(rename = "votingEnabled", skip_serializing_if = "Option::is_none")]
    pub voting_enabled: Option<bool>,
    #[serde(rename = "watchingEnabled", skip_serializing_if = "Option::is_none")]
    pub watching_enabled: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeTrackingConfigurationBean {
    #[serde(rename = "defaultUnit", skip_serializing_if = "Option::is_none")]
    pub default_unit: Option<TimeTrackingConfigurationBeanDefaultUnit>,
    #[serde(rename = "timeFormat", skip_serializing_if = "Option::is_none")]
    pub time_format: Option<TimeTrackingConfigurationBeanTimeFormat>,
    #[serde(rename = "workingDaysPerWeek", skip_serializing_if = "Option::is_none")]
    pub working_days_per_week: Option<f64>,
    #[serde(rename = "workingHoursPerDay", skip_serializing_if = "Option::is_none")]
    pub working_hours_per_day: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TimeTrackingConfigurationBeanTimeFormat {
    #[default]
    #[serde(rename = "pretty")]
    Pretty,
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "hours")]
    Hours,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TimeTrackingConfigurationBeanDefaultUnit {
    #[default]
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComponentBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "assigneeType", skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<ComponentBeanAssigneeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead: Option<UserBean>,
    #[serde(rename = "leadUserName", skip_serializing_if = "Option::is_none")]
    pub lead_user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "applicationRoles", skip_serializing_if = "Option::is_none")]
    pub application_roles: Option<SimpleListWrapperApplicationRoleBean>,
    #[serde(rename = "avatarUrls", skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<UserBeanAvatarUrls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<SimpleListWrapperGroupJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "lastLoginTime", skip_serializing_if = "Option::is_none")]
    pub last_login_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserBeanAvatarUrls {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleListWrapperGroupJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackGroupJsonBean>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "pagingCallback", skip_serializing_if = "Option::is_none")]
    pub paging_callback: Option<ListWrapperCallbackGroupJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
pub type ListWrapperCallbackGroupJsonBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleListWrapperApplicationRoleBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackApplicationRoleBean>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "pagingCallback", skip_serializing_if = "Option::is_none")]
    pub paging_callback: Option<ListWrapperCallbackApplicationRoleBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
pub type ListWrapperCallbackApplicationRoleBean = serde_json::Value;
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ComponentBeanAssigneeType {
    #[default]
    #[serde(rename = "PROJECT_DEFAULT")]
    ProjectDefault,
    #[serde(rename = "COMPONENT_LEAD")]
    ComponentLead,
    #[serde(rename = "PROJECT_LEAD")]
    ProjectLead,
    #[serde(rename = "UNASSIGNED")]
    Unassigned,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentsWithPaginationJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<CommentJsonBean>>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<EntityPropertyBean>>,
    #[serde(rename = "renderedBody", skip_serializing_if = "Option::is_none")]
    pub rendered_body: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "updateAuthor", skip_serializing_if = "Option::is_none")]
    pub update_author: Option<UserJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<VisibilityJsonBean>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VisibilityJsonBean {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<VisibilityJsonBeanType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum VisibilityJsonBeanType {
    #[default]
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "role")]
    Role,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityPropertyBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColumnLayout {
    #[serde(rename = "columnConfig", skip_serializing_if = "Option::is_none")]
    pub column_config: Option<ColumnLayoutColumnConfig>,
    #[serde(rename = "columnLayoutItems", skip_serializing_if = "Option::is_none")]
    pub column_layout_items: Option<Vec<ColumnLayoutItem>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColumnLayoutItem {
    #[serde(rename = "columnHeadingKey", skip_serializing_if = "Option::is_none")]
    pub column_heading_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "navigableField", skip_serializing_if = "Option::is_none")]
    pub navigable_field: Option<NavigableField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NavigableField {
    #[serde(rename = "columnCssClass", skip_serializing_if = "Option::is_none")]
    pub column_css_class: Option<String>,
    #[serde(rename = "columnHeadingKey", skip_serializing_if = "Option::is_none")]
    pub column_heading_key: Option<String>,
    #[serde(rename = "defaultSortOrder", skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<String>,
    #[serde(rename = "hiddenFieldId", skip_serializing_if = "Option::is_none")]
    pub hidden_field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nameKey", skip_serializing_if = "Option::is_none")]
    pub name_key: Option<String>,
    #[serde(rename = "valueLoader", skip_serializing_if = "Option::is_none")]
    pub value_loader: Option<FieldValueLoader>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldValueLoader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ColumnLayoutColumnConfig {
    #[default]
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "EXPLICIT")]
    Explicit,
    #[serde(rename = "FILTER")]
    Filter,
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "NONE")]
    None,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColorBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<ColorBeanKey>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ColorBeanKey {
    #[default]
    #[serde(rename = "color_1")]
    Color1,
    #[serde(rename = "color_2")]
    Color2,
    #[serde(rename = "color_3")]
    Color3,
    #[serde(rename = "color_4")]
    Color4,
    #[serde(rename = "color_5")]
    Color5,
    #[serde(rename = "color_6")]
    Color6,
    #[serde(rename = "color_7")]
    Color7,
    #[serde(rename = "color_8")]
    Color8,
    #[serde(rename = "color_9")]
    Color9,
    #[serde(rename = "color_10")]
    Color10,
    #[serde(rename = "color_11")]
    Color11,
    #[serde(rename = "color_12")]
    Color12,
    #[serde(rename = "color_13")]
    Color13,
    #[serde(rename = "color_14")]
    Color14,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClusterState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<NodeBuildInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ClusterStateState>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeBuildInfo {
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ClusterStateState {
    #[default]
    #[serde(rename = "STABLE")]
    Stable,
    #[serde(rename = "READY_TO_UPGRADE")]
    ReadyToUpgrade,
    #[serde(rename = "MIXED")]
    Mixed,
    #[serde(rename = "READY_TO_RUN_UPGRADE_TASKS")]
    ReadyToRunUpgradeTasks,
    #[serde(rename = "RUNNING_UPGRADE_TASKS")]
    RunningUpgradeTasks,
    #[serde(rename = "UPGRADE_TASKS_FAILED")]
    UpgradeTasksFailed,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClusterLockStatusesBean {
    #[serde(rename = "clusterLocks", skip_serializing_if = "Option::is_none")]
    pub cluster_locks: Option<Vec<ClusterLockStatusBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClusterLockStatusBean {
    #[serde(rename = "holdingLockSec", skip_serializing_if = "Option::is_none")]
    pub holding_lock_sec: Option<String>,
    #[serde(rename = "lockName", skip_serializing_if = "Option::is_none")]
    pub lock_name: Option<String>,
    #[serde(rename = "lockedByNode", skip_serializing_if = "Option::is_none")]
    pub locked_by_node: Option<String>,
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangelogBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub histories: Option<Vec<ChangeHistoryBean>>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangeHistoryBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "historyMetadata", skip_serializing_if = "Option::is_none")]
    pub history_metadata: Option<HistoryMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ChangeItemBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HistoryMetadata {
    #[serde(rename = "activityDescription", skip_serializing_if = "Option::is_none")]
    pub activity_description: Option<String>,
    #[serde(rename = "activityDescriptionKey", skip_serializing_if = "Option::is_none")]
    pub activity_description_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<HistoryMetadataParticipant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<HistoryMetadataParticipant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "descriptionKey", skip_serializing_if = "Option::is_none")]
    pub description_key: Option<String>,
    #[serde(rename = "emailDescription", skip_serializing_if = "Option::is_none")]
    pub email_description: Option<String>,
    #[serde(rename = "emailDescriptionKey", skip_serializing_if = "Option::is_none")]
    pub email_description_key: Option<String>,
    #[serde(rename = "extraData", skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<HistoryMetadataExtraData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<HistoryMetadataParticipant>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HistoryMetadataExtraData {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HistoryMetadataParticipant {
    #[serde(rename = "avatarUrl", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayNameKey", skip_serializing_if = "Option::is_none")]
    pub display_name_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangeItemBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fieldtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "fromString", skip_serializing_if = "Option::is_none")]
    pub from_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "toString", skip_serializing_if = "Option::is_none")]
    pub to_string: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BulkOperationErrorResult {
    #[serde(rename = "elementErrors", skip_serializing_if = "Option::is_none")]
    pub element_errors: Option<ErrorCollection>,
    #[serde(rename = "failedElementNumber", skip_serializing_if = "Option::is_none")]
    pub failed_element_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorCollection {
    #[serde(rename = "errorMessages", skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<ErrorCollectionErrors>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorCollectionErrors {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BulkDeleteResponseBean {
    #[serde(rename = "deletedCustomFields", skip_serializing_if = "Option::is_none")]
    pub deleted_custom_fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "notDeletedCustomFields", skip_serializing_if = "Option::is_none")]
    pub not_deleted_custom_fields: Option<BulkDeleteResponseBeanNotDeletedCustomFields>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BulkDeleteResponseBeanNotDeletedCustomFields {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoardConfigBean {
    #[serde(rename = "columnConfig", skip_serializing_if = "Option::is_none")]
    pub column_config: Option<ColumnConfigBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimation: Option<EstimationConfigBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<RelationBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking: Option<RankingConfigBean>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "subQuery", skip_serializing_if = "Option::is_none")]
    pub sub_query: Option<SubqueryBean>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubqueryBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RankingConfigBean {
    #[serde(rename = "rankCustomFieldId", skip_serializing_if = "Option::is_none")]
    pub rank_custom_field_id: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstimationConfigBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<EstimationFieldBean>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstimationFieldBean {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "fieldId", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColumnConfigBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<ColumnBean>>,
    #[serde(rename = "constraintType", skip_serializing_if = "Option::is_none")]
    pub constraint_type: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColumnBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<RelationBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AvailableProjectsPaginatedBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<PageBeanProjectBean>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageBeanProjectBean {
    #[serde(rename = "isLast", skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextPage", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<ProjectBean>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "avatarUrls", skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<ProjectBeanAvatarUrls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectBeanAvatarUrls {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthSuccess {
    #[serde(rename = "loginInfo", skip_serializing_if = "Option::is_none")]
    pub login_info: Option<LoginInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<SessionInfo>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginInfo {
    #[serde(rename = "failedLoginCount", skip_serializing_if = "Option::is_none")]
    pub failed_login_count: Option<i64>,
    #[serde(rename = "lastFailedLoginTime", skip_serializing_if = "Option::is_none")]
    pub last_failed_login_time: Option<String>,
    #[serde(rename = "loginCount", skip_serializing_if = "Option::is_none")]
    pub login_count: Option<i64>,
    #[serde(rename = "previousLoginTime", skip_serializing_if = "Option::is_none")]
    pub previous_login_time: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserJsonBean>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "avatarUrls", skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<UserJsonBeanAvatarUrls>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserJsonBeanAvatarUrls {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentArchiveImpl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AttachmentArchiveEntry>>,
    ///Total number of entries available (can be larger that what was asked for)
    #[serde(rename = "totalEntryCount", skip_serializing_if = "Option::is_none")]
    pub total_entry_count: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentArchiveEntry {
    #[serde(rename = "abbreviatedName", skip_serializing_if = "Option::is_none")]
    pub abbreviated_name: Option<String>,
    #[serde(rename = "entryIndex", skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<i64>,
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AsyncContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<Box<ServletRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ServletResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletResponse {
    #[serde(rename = "bufferSize", skip_serializing_if = "Option::is_none")]
    pub buffer_size: Option<i32>,
    #[serde(rename = "characterEncoding", skip_serializing_if = "Option::is_none")]
    pub character_encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committed: Option<bool>,
    #[serde(rename = "contentLength", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i32>,
    #[serde(rename = "contentLengthLong", skip_serializing_if = "Option::is_none")]
    pub content_length_long: Option<i64>,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<ServletResponseLocale>,
    #[serde(rename = "outputStream", skip_serializing_if = "Option::is_none")]
    pub output_stream: Option<ServletOutputStream>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writer: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletResponseLocale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "displayCountry", skip_serializing_if = "Option::is_none")]
    pub display_country: Option<String>,
    #[serde(rename = "displayLanguage", skip_serializing_if = "Option::is_none")]
    pub display_language: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayScript", skip_serializing_if = "Option::is_none")]
    pub display_script: Option<String>,
    #[serde(rename = "displayVariant", skip_serializing_if = "Option::is_none")]
    pub display_variant: Option<String>,
    #[serde(rename = "extensionKeys", skip_serializing_if = "Option::is_none")]
    pub extension_keys: Option<Vec<String>>,
    #[serde(rename = "iso3Country", skip_serializing_if = "Option::is_none")]
    pub iso3_country: Option<String>,
    #[serde(rename = "iso3Language", skip_serializing_if = "Option::is_none")]
    pub iso3_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(rename = "unicodeLocaleAttributes", skip_serializing_if = "Option::is_none")]
    pub unicode_locale_attributes: Option<Vec<String>>,
    #[serde(rename = "unicodeLocaleKeys", skip_serializing_if = "Option::is_none")]
    pub unicode_locale_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletOutputStream {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[serde(rename = "writeListener", skip_serializing_if = "Option::is_none")]
    pub write_listener: Option<WriteListener>,
}
pub type WriteListener = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletRequest {
    #[serde(rename = "asyncContext", skip_serializing_if = "Option::is_none")]
    pub async_context: Option<Box<AsyncContext>>,
    #[serde(rename = "asyncStarted", skip_serializing_if = "Option::is_none")]
    pub async_started: Option<bool>,
    #[serde(rename = "asyncSupported", skip_serializing_if = "Option::is_none")]
    pub async_supported: Option<bool>,
    #[serde(rename = "attributeNames", skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<serde_json::Value>,
    #[serde(rename = "characterEncoding", skip_serializing_if = "Option::is_none")]
    pub character_encoding: Option<String>,
    #[serde(rename = "contentLength", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i32>,
    #[serde(rename = "contentLengthLong", skip_serializing_if = "Option::is_none")]
    pub content_length_long: Option<i64>,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "dispatcherType", skip_serializing_if = "Option::is_none")]
    pub dispatcher_type: Option<ServletRequestDispatcherType>,
    #[serde(rename = "inputStream", skip_serializing_if = "Option::is_none")]
    pub input_stream: Option<ServletInputStream>,
    #[serde(rename = "localAddr", skip_serializing_if = "Option::is_none")]
    pub local_addr: Option<String>,
    #[serde(rename = "localName", skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    #[serde(rename = "localPort", skip_serializing_if = "Option::is_none")]
    pub local_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<Box<ServletRequestLocale>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locales: Option<serde_json::Value>,
    #[serde(rename = "parameterMap", skip_serializing_if = "Option::is_none")]
    pub parameter_map: Option<Box<ServletRequestParameterMap>>,
    #[serde(rename = "parameterNames", skip_serializing_if = "Option::is_none")]
    pub parameter_names: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "protocolRequestId", skip_serializing_if = "Option::is_none")]
    pub protocol_request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader: Option<serde_json::Value>,
    #[serde(rename = "remoteAddr", skip_serializing_if = "Option::is_none")]
    pub remote_addr: Option<String>,
    #[serde(rename = "remoteHost", skip_serializing_if = "Option::is_none")]
    pub remote_host: Option<String>,
    #[serde(rename = "remotePort", skip_serializing_if = "Option::is_none")]
    pub remote_port: Option<i32>,
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(rename = "serverName", skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "serverPort", skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
    #[serde(rename = "servletConnection", skip_serializing_if = "Option::is_none")]
    pub servlet_connection: Option<ServletConnection>,
    #[serde(rename = "servletContext", skip_serializing_if = "Option::is_none")]
    pub servlet_context: Option<ServletContext>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletRequestParameterMap {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletRequestLocale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "displayCountry", skip_serializing_if = "Option::is_none")]
    pub display_country: Option<String>,
    #[serde(rename = "displayLanguage", skip_serializing_if = "Option::is_none")]
    pub display_language: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayScript", skip_serializing_if = "Option::is_none")]
    pub display_script: Option<String>,
    #[serde(rename = "displayVariant", skip_serializing_if = "Option::is_none")]
    pub display_variant: Option<String>,
    #[serde(rename = "extensionKeys", skip_serializing_if = "Option::is_none")]
    pub extension_keys: Option<Vec<String>>,
    #[serde(rename = "iso3Country", skip_serializing_if = "Option::is_none")]
    pub iso3_country: Option<String>,
    #[serde(rename = "iso3Language", skip_serializing_if = "Option::is_none")]
    pub iso3_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(rename = "unicodeLocaleAttributes", skip_serializing_if = "Option::is_none")]
    pub unicode_locale_attributes: Option<Vec<String>>,
    #[serde(rename = "unicodeLocaleKeys", skip_serializing_if = "Option::is_none")]
    pub unicode_locale_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ServletRequestDispatcherType {
    #[default]
    #[serde(rename = "FORWARD")]
    Forward,
    #[serde(rename = "INCLUDE")]
    Include,
    #[serde(rename = "REQUEST")]
    Request,
    #[serde(rename = "ASYNC")]
    Async_,
    #[serde(rename = "ERROR")]
    Error,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletInputStream {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    #[serde(rename = "readListener", skip_serializing_if = "Option::is_none")]
    pub read_listener: Option<ReadListener>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}
pub type ReadListener = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletContext {
    #[serde(rename = "attributeNames", skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<serde_json::Value>,
    #[serde(rename = "classLoader", skip_serializing_if = "Option::is_none")]
    pub class_loader: Option<ServletContextClassLoader>,
    #[serde(rename = "contextPath", skip_serializing_if = "Option::is_none")]
    pub context_path: Option<String>,
    #[serde(
        rename = "defaultSessionTrackingModes",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_session_tracking_modes: Option<Vec<String>>,
    #[serde(rename = "effectiveMajorVersion", skip_serializing_if = "Option::is_none")]
    pub effective_major_version: Option<i32>,
    #[serde(rename = "effectiveMinorVersion", skip_serializing_if = "Option::is_none")]
    pub effective_minor_version: Option<i32>,
    #[serde(
        rename = "effectiveSessionTrackingModes",
        skip_serializing_if = "Option::is_none"
    )]
    pub effective_session_tracking_modes: Option<Vec<String>>,
    #[serde(rename = "filterRegistrations", skip_serializing_if = "Option::is_none")]
    pub filter_registrations: Option<ServletContextFilterRegistrations>,
    #[serde(rename = "initParameterNames", skip_serializing_if = "Option::is_none")]
    pub init_parameter_names: Option<serde_json::Value>,
    #[serde(rename = "jspConfigDescriptor", skip_serializing_if = "Option::is_none")]
    pub jsp_config_descriptor: Option<JspConfigDescriptor>,
    #[serde(rename = "majorVersion", skip_serializing_if = "Option::is_none")]
    pub major_version: Option<i32>,
    #[serde(rename = "minorVersion", skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<i32>,
    #[serde(
        rename = "requestCharacterEncoding",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_character_encoding: Option<String>,
    #[serde(
        rename = "responseCharacterEncoding",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_character_encoding: Option<String>,
    #[serde(rename = "serverInfo", skip_serializing_if = "Option::is_none")]
    pub server_info: Option<String>,
    #[serde(rename = "servletContextName", skip_serializing_if = "Option::is_none")]
    pub servlet_context_name: Option<String>,
    #[serde(rename = "servletRegistrations", skip_serializing_if = "Option::is_none")]
    pub servlet_registrations: Option<ServletContextServletRegistrations>,
    #[serde(rename = "sessionCookieConfig", skip_serializing_if = "Option::is_none")]
    pub session_cookie_config: Option<SessionCookieConfig>,
    #[serde(rename = "sessionTimeout", skip_serializing_if = "Option::is_none")]
    pub session_timeout: Option<i32>,
    #[serde(rename = "sessionTrackingModes", skip_serializing_if = "Option::is_none")]
    pub session_tracking_modes: Option<Vec<String>>,
    #[serde(rename = "virtualServerName", skip_serializing_if = "Option::is_none")]
    pub virtual_server_name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionCookieConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SessionCookieConfigAttributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "httpOnly", skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    #[serde(rename = "maxAge", skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionCookieConfigAttributes {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletContextServletRegistrations {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletContextFilterRegistrations {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletContextClassLoader {
    #[serde(rename = "defaultAssertionStatus", skip_serializing_if = "Option::is_none")]
    pub default_assertion_status: Option<bool>,
    #[serde(rename = "definedPackages", skip_serializing_if = "Option::is_none")]
    pub defined_packages: Option<Vec<Box<ServletContextDefinedPackagesItem>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<ServletContextParent>>,
    #[serde(
        rename = "registeredAsParallelCapable",
        skip_serializing_if = "Option::is_none"
    )]
    pub registered_as_parallel_capable: Option<bool>,
    #[serde(rename = "unnamedModule", skip_serializing_if = "Option::is_none")]
    pub unnamed_module: Option<Box<ServletContextUnnamedModule>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletContextAnnotationsItem {}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletContextDeclaredAnnotationsItem {}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletContextDescriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletContextParent {
    #[serde(rename = "defaultAssertionStatus", skip_serializing_if = "Option::is_none")]
    pub default_assertion_status: Option<bool>,
    #[serde(rename = "definedPackages", skip_serializing_if = "Option::is_none")]
    pub defined_packages: Option<Vec<Box<ServletContextDefinedPackagesItem>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "registeredAsParallelCapable",
        skip_serializing_if = "Option::is_none"
    )]
    pub registered_as_parallel_capable: Option<bool>,
    #[serde(rename = "unnamedModule", skip_serializing_if = "Option::is_none")]
    pub unnamed_module: Option<Box<ServletContextUnnamedModule>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletContextUnnamedModule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Box<ServletContextAnnotationsItem>>>,
    #[serde(rename = "declaredAnnotations", skip_serializing_if = "Option::is_none")]
    pub declared_annotations: Option<Vec<Box<ServletContextDeclaredAnnotationsItem>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<Box<ServletContextDescriptor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named: Option<bool>,
    #[serde(rename = "nativeAccessEnabled", skip_serializing_if = "Option::is_none")]
    pub native_access_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletContextDefinedPackagesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Box<ServletContextAnnotationsItem>>>,
    #[serde(rename = "declaredAnnotations", skip_serializing_if = "Option::is_none")]
    pub declared_annotations: Option<Vec<Box<ServletContextDeclaredAnnotationsItem>>>,
    #[serde(rename = "implementationTitle", skip_serializing_if = "Option::is_none")]
    pub implementation_title: Option<String>,
    #[serde(rename = "implementationVendor", skip_serializing_if = "Option::is_none")]
    pub implementation_vendor: Option<String>,
    #[serde(rename = "implementationVersion", skip_serializing_if = "Option::is_none")]
    pub implementation_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sealed: Option<bool>,
    #[serde(rename = "specificationTitle", skip_serializing_if = "Option::is_none")]
    pub specification_title: Option<String>,
    #[serde(rename = "specificationVendor", skip_serializing_if = "Option::is_none")]
    pub specification_vendor: Option<String>,
    #[serde(rename = "specificationVersion", skip_serializing_if = "Option::is_none")]
    pub specification_version: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JspConfigDescriptor {
    #[serde(rename = "jspPropertyGroups", skip_serializing_if = "Option::is_none")]
    pub jsp_property_groups: Option<Vec<JspPropertyGroupDescriptor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taglibs: Option<Vec<TaglibDescriptor>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaglibDescriptor {
    #[serde(rename = "taglibLocation", skip_serializing_if = "Option::is_none")]
    pub taglib_location: Option<String>,
    #[serde(rename = "taglibURI", skip_serializing_if = "Option::is_none")]
    pub taglib_uri: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JspPropertyGroupDescriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer: Option<String>,
    #[serde(rename = "defaultContentType", skip_serializing_if = "Option::is_none")]
    pub default_content_type: Option<String>,
    #[serde(
        rename = "deferredSyntaxAllowedAsLiteral",
        skip_serializing_if = "Option::is_none"
    )]
    pub deferred_syntax_allowed_as_literal: Option<String>,
    #[serde(rename = "elIgnored", skip_serializing_if = "Option::is_none")]
    pub el_ignored: Option<String>,
    #[serde(rename = "errorOnELNotFound", skip_serializing_if = "Option::is_none")]
    pub error_on_elnot_found: Option<String>,
    #[serde(
        rename = "errorOnUndeclaredNamespace",
        skip_serializing_if = "Option::is_none"
    )]
    pub error_on_undeclared_namespace: Option<String>,
    #[serde(rename = "includeCodas", skip_serializing_if = "Option::is_none")]
    pub include_codas: Option<Vec<String>>,
    #[serde(rename = "includePreludes", skip_serializing_if = "Option::is_none")]
    pub include_preludes: Option<Vec<String>>,
    #[serde(rename = "isXml", skip_serializing_if = "Option::is_none")]
    pub is_xml: Option<String>,
    #[serde(rename = "pageEncoding", skip_serializing_if = "Option::is_none")]
    pub page_encoding: Option<String>,
    #[serde(rename = "scriptingInvalid", skip_serializing_if = "Option::is_none")]
    pub scripting_invalid: Option<String>,
    #[serde(
        rename = "trimDirectiveWhitespaces",
        skip_serializing_if = "Option::is_none"
    )]
    pub trim_directive_whitespaces: Option<String>,
    #[serde(rename = "urlPatterns", skip_serializing_if = "Option::is_none")]
    pub url_patterns: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServletConnection {
    #[serde(rename = "connectionId", skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "protocolConnectionId", skip_serializing_if = "Option::is_none")]
    pub protocol_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AffectedEntityBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "numberOfOccurrences", skip_serializing_if = "Option::is_none")]
    pub number_of_occurrences: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<AffectedEntityBeanType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "uriDisplayName", skip_serializing_if = "Option::is_none")]
    pub uri_display_name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AffectedEntityBeanType {
    #[default]
    #[serde(rename = "ANONYMIZE")]
    Anonymize,
    #[serde(rename = "TRANSFER_OWNERSHIP")]
    TransferOwnership,
    #[serde(rename = "REMOVE")]
    Remove,
    #[serde(rename = "MANUAL")]
    Manual,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct A11yPersonalSettingBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActiveCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActorInputBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActorsMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddFieldBean {
    #[serde(rename = "fieldId", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddGroupBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddReactionRequestBean {
    #[serde(rename = "commentId", skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<i64>,
    #[serde(rename = "emojiId", skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<String>,
}
pub type AddWatcher1Request = String;
pub type AdminHistoryLink = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppMonitoringRestEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppTypeBean {
    #[serde(rename = "i18nKey", skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApplicationPropertyBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApplicationRoleBean {
    #[serde(rename = "defaultGroups", skip_serializing_if = "Option::is_none")]
    pub default_groups: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "hasUnlimitedSeats", skip_serializing_if = "Option::is_none")]
    pub has_unlimited_seats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numberOfSeats", skip_serializing_if = "Option::is_none")]
    pub number_of_seats: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<bool>,
    #[serde(rename = "remainingSeats", skip_serializing_if = "Option::is_none")]
    pub remaining_seats: Option<i32>,
    #[serde(rename = "selectedByDefault", skip_serializing_if = "Option::is_none")]
    pub selected_by_default: Option<bool>,
    #[serde(rename = "userCount", skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i32>,
    #[serde(rename = "userCountDescription", skip_serializing_if = "Option::is_none")]
    pub user_count_description: Option<String>,
}
pub type AreMetricsExposedResponse = bool;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssignIssueTypesRequest {
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssociateProjectsBean {
    #[serde(rename = "idsOrKeys", skip_serializing_if = "Option::is_none")]
    pub ids_or_keys: Option<Vec<String>>,
}
pub type AttachmentBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentMetaBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///Upload limit in bytes
    #[serde(rename = "uploadLimit", skip_serializing_if = "Option::is_none")]
    pub upload_limit: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoCompleteResponseBean {
    #[serde(rename = "jqlReservedWords", skip_serializing_if = "Option::is_none")]
    pub jql_reserved_words: Option<Vec<String>>,
    #[serde(rename = "visibleFieldNames", skip_serializing_if = "Option::is_none")]
    pub visible_field_names: Option<Vec<String>>,
    #[serde(rename = "visibleFunctionNames", skip_serializing_if = "Option::is_none")]
    pub visible_function_names: Option<Vec<String>>,
}
pub type AutoCompleteResultWrapper = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AvailableIssueTypesRequestBean {
    #[serde(rename = "ignoredIssueTypeIds", skip_serializing_if = "Option::is_none")]
    pub ignored_issue_type_ids: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AvailableProjectsRequestBean {
    #[serde(rename = "ignoredProjectIds", skip_serializing_if = "Option::is_none")]
    pub ignored_project_ids: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AvatarBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AvatarCroppingBean {
    #[serde(rename = "cropperOffsetX", skip_serializing_if = "Option::is_none")]
    pub cropper_offset_x: Option<i32>,
    #[serde(rename = "cropperOffsetY", skip_serializing_if = "Option::is_none")]
    pub cropper_offset_y: Option<i32>,
    #[serde(rename = "cropperWidth", skip_serializing_if = "Option::is_none")]
    pub cropper_width: Option<i32>,
    #[serde(rename = "needsCropping", skip_serializing_if = "Option::is_none")]
    pub needs_cropping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
pub type BlogBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoardBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoardCreateBean {
    #[serde(rename = "filterId", skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BooleanSettingBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}
pub type CanMoveSubTaskResponse = bool;
pub type ColumnOptions = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColumnsBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComponentIssueCountsBean {
    #[serde(rename = "issueCount", skip_serializing_if = "Option::is_none")]
    pub issue_count: Option<i64>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
pub type ContentToRender = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateProjectShortcutBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUpdateRoleRequestBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type CurrentUser = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomFieldBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isAllProjects", skip_serializing_if = "Option::is_none")]
    pub is_all_projects: Option<bool>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isManaged", skip_serializing_if = "Option::is_none")]
    pub is_managed: Option<bool>,
    #[serde(rename = "isTrusted", skip_serializing_if = "Option::is_none")]
    pub is_trusted: Option<bool>,
    #[serde(rename = "issueTypeIds", skip_serializing_if = "Option::is_none")]
    pub issue_type_ids: Option<Vec<String>>,
    #[serde(rename = "issuesWithValue", skip_serializing_if = "Option::is_none")]
    pub issues_with_value: Option<i64>,
    #[serde(rename = "lastValueUpdate", skip_serializing_if = "Option::is_none")]
    pub last_value_update: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numericId", skip_serializing_if = "Option::is_none")]
    pub numeric_id: Option<i64>,
    #[serde(rename = "projectIds", skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,
    #[serde(rename = "projectsCount", skip_serializing_if = "Option::is_none")]
    pub projects_count: Option<i32>,
    #[serde(rename = "screensCount", skip_serializing_if = "Option::is_none")]
    pub screens_count: Option<i32>,
    #[serde(rename = "searcherKey", skip_serializing_if = "Option::is_none")]
    pub searcher_key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomFieldDefinitionJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "issueTypeIds", skip_serializing_if = "Option::is_none")]
    pub issue_type_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectIds", skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,
    #[serde(rename = "searcherKey", skip_serializing_if = "Option::is_none")]
    pub searcher_key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DarkFeaturePropertyBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DefaultBean {
    #[serde(rename = "updateDraftIfNeeded", skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EditorMarkupParameters {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "fieldName", skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "issueKey", skip_serializing_if = "Option::is_none")]
    pub issue_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityVersionBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<i64>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(rename = "entityVersion", skip_serializing_if = "Option::is_none")]
    pub entity_version: Option<i64>,
    #[serde(rename = "hasVersion", skip_serializing_if = "Option::is_none")]
    pub has_version: Option<bool>,
    #[serde(rename = "parentIssueId", skip_serializing_if = "Option::is_none")]
    pub parent_issue_id: Option<i64>,
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpicRankRequestBean {
    #[serde(rename = "rankAfterEpic", skip_serializing_if = "Option::is_none")]
    pub rank_after_epic: Option<String>,
    #[serde(rename = "rankBeforeEpic", skip_serializing_if = "Option::is_none")]
    pub rank_before_epic: Option<String>,
    #[serde(rename = "rankCustomFieldId", skip_serializing_if = "Option::is_none")]
    pub rank_custom_field_id: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldEditBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldOperation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldValueBean {
    #[serde(rename = "fieldId", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilePart {
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "formField", skip_serializing_if = "Option::is_none")]
    pub form_field: Option<bool>,
    #[serde(rename = "inputStream", skip_serializing_if = "Option::is_none")]
    pub input_stream: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
pub type GetAvailableMetricsResponse = String;
pub type GetMaxAggregationBucketsResponse = i64;
pub type GetMaxResultWindowResponse = i64;
pub type GetPasswordPolicyResponse = String;
pub type GetPreferenceResponse = String;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetReactionsRequestBean {
    #[serde(rename = "commentIds", skip_serializing_if = "Option::is_none")]
    pub comment_ids: Option<Vec<i64>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HumanReadableArchive {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "totalEntryCount", skip_serializing_if = "Option::is_none")]
    pub total_entry_count: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexSnapshotBean {
    #[serde(rename = "absolutePath", skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexSnapshotPromiseBean {
    #[serde(rename = "futureAbsolutePath", skip_serializing_if = "Option::is_none")]
    pub future_absolute_path: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexSnapshotStatusBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IpdMonitoringRestEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueAssignRequestBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueLinkTypeOrderUpdateRequest {
    #[serde(rename = "newPosition", skip_serializing_if = "Option::is_none")]
    pub new_position: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueLinkTypeResetOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}
pub type IssueLinkTypesBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueRankRequestBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<String>>,
    #[serde(rename = "rankAfterIssue", skip_serializing_if = "Option::is_none")]
    pub rank_after_issue: Option<String>,
    #[serde(rename = "rankBeforeIssue", skip_serializing_if = "Option::is_none")]
    pub rank_before_issue: Option<String>,
    #[serde(rename = "rankCustomFieldId", skip_serializing_if = "Option::is_none")]
    pub rank_custom_field_id: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueSubTaskMovePositionBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueTypeMappingBean {
    #[serde(rename = "issueType", skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,
    #[serde(rename = "updateDraftIfNeeded", skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueTypeSchemeCreateUpdateBean {
    #[serde(rename = "defaultIssueTypeId", skip_serializing_if = "Option::is_none")]
    pub default_issue_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "issueTypeIDs", skip_serializing_if = "Option::is_none")]
    pub issue_type_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueTypeUpdateBean {
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobBean {
    #[serde(rename = "cronExpression", skip_serializing_if = "Option::is_none")]
    pub cron_expression: Option<String>,
    #[serde(rename = "firstRunTime", skip_serializing_if = "Option::is_none")]
    pub first_run_time: Option<i64>,
    #[serde(rename = "intervalInMillis", skip_serializing_if = "Option::is_none")]
    pub interval_in_millis: Option<i64>,
    #[serde(rename = "jobId", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobRunnerKey", skip_serializing_if = "Option::is_none")]
    pub job_runner_key: Option<String>,
    #[serde(rename = "nextRunTime", skip_serializing_if = "Option::is_none")]
    pub next_run_time: Option<i64>,
    #[serde(rename = "runMode", skip_serializing_if = "Option::is_none")]
    pub run_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runnable: Option<bool>,
    #[serde(rename = "scheduleType", skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<String>,
    #[serde(rename = "timeZoneId", skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobRunBean {
    #[serde(rename = "durationInMillis", skip_serializing_if = "Option::is_none")]
    pub duration_in_millis: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "runOutcome", skip_serializing_if = "Option::is_none")]
    pub run_outcome: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
pub type LastVisitedItem = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotificationSchemeBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "notificationSchemeEvents",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_scheme_events: Option<serde_json::Value>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderByPreferences {
    #[serde(rename = "orderByOption", skip_serializing_if = "Option::is_none")]
    pub order_by_option: Option<String>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderByRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OriginalOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<i64>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PasswordBean {
    #[serde(rename = "currentPassword", skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PasswordPolicyCreateUserBean {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PasswordPolicyUpdateUserBean {
    #[serde(rename = "newPassword", skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[serde(rename = "oldPassword", skip_serializing_if = "Option::is_none")]
    pub old_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PatternRepresentation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "permType", skip_serializing_if = "Option::is_none")]
    pub perm_type: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionSchemeAttributeBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
pub type PolicyCheckCreateUserResponse = String;
pub type PolicyCheckUpdateUserResponse = String;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrioritySchemeResponse {
    #[serde(rename = "defaultPriority", skip_serializing_if = "Option::is_none")]
    pub default_priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priorities: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrioritySchemeUpdateBean {
    #[serde(rename = "defaultOptionId", skip_serializing_if = "Option::is_none")]
    pub default_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "optionIds", skip_serializing_if = "Option::is_none")]
    pub option_ids: Option<Vec<String>>,
}
pub type ProcessRequestsResponse = i64;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectCategoryBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectDeleteInstructionsBean {
    #[serde(rename = "grantsToDelete", skip_serializing_if = "Option::is_none")]
    pub grants_to_delete: Option<Vec<i64>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectTypeBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "descriptionI18nKey", skip_serializing_if = "Option::is_none")]
    pub description_i18n_key: Option<String>,
    #[serde(rename = "formattedKey", skip_serializing_if = "Option::is_none")]
    pub formatted_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Property {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterItemHolder {
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isManaged", skip_serializing_if = "Option::is_none")]
    pub is_managed: Option<bool>,
}
pub type ReleaseRequest = String;
pub type RemoteIssueLinkBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoveWorkflowRequest {
    #[serde(rename = "nextDefaultWorkflow", skip_serializing_if = "Option::is_none")]
    pub next_default_workflow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}
pub type ResolutionBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResolutionJsonBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseValue {
    #[serde(rename = "errorStringI18n", skip_serializing_if = "Option::is_none")]
    pub error_string_i18n: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestInvocationHistory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScreenableFieldBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "showWhenEmpty", skip_serializing_if = "Option::is_none")]
    pub show_when_empty: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScreenableTabBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchRequestBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    #[serde(rename = "validateQuery", skip_serializing_if = "Option::is_none")]
    pub validate_query: Option<bool>,
}
pub type ServerInfoBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceAccountBean {
    #[serde(rename = "clientConfigurationId", skip_serializing_if = "Option::is_none")]
    pub client_configuration_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectKeys", skip_serializing_if = "Option::is_none")]
    pub project_keys: Option<Vec<String>>,
}
pub type SetBaseUrlRequest = String;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SetColumnsUrlEncodedRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SetIssueNavigatorDefaultColumnsFormRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
}
pub type SetPinCommentRequest = bool;
pub type SetPreferenceRequest = String;
pub type SetProperty2Request = String;
pub type SetProperty5Request = String;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShareBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usernames: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SharePermissionInputBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupname: Option<String>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "projectRoleId", skip_serializing_if = "Option::is_none")]
    pub project_role_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "userKey", skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SortByBean {
    #[serde(rename = "fieldId", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(rename = "fieldName", skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "toggleJql", skip_serializing_if = "Option::is_none")]
    pub toggle_jql: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SprintBean {
    #[serde(rename = "activatedDate", skip_serializing_if = "Option::is_none")]
    pub activated_date: Option<String>,
    #[serde(rename = "autoStartStop", skip_serializing_if = "Option::is_none")]
    pub auto_start_stop: Option<bool>,
    #[serde(rename = "completeDate", skip_serializing_if = "Option::is_none")]
    pub complete_date: Option<String>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(
        rename = "incompleteIssuesDestinationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub incomplete_issues_destination_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "originBoardId", skip_serializing_if = "Option::is_none")]
    pub origin_board_id: Option<i64>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SprintCreateBean {
    #[serde(rename = "autoStartStop", skip_serializing_if = "Option::is_none")]
    pub auto_start_stop: Option<bool>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(
        rename = "incompleteIssuesDestinationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub incomplete_issues_destination_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "originBoardId", skip_serializing_if = "Option::is_none")]
    pub origin_board_id: Option<i64>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced: Option<bool>,
    #[serde(rename = "userProfileTimeZone", skip_serializing_if = "Option::is_none")]
    pub user_profile_time_zone: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SprintSwapBean {
    #[serde(rename = "sprintToSwapWith", skip_serializing_if = "Option::is_none")]
    pub sprint_to_swap_with: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<i64>,
}
pub type StreamingOutput = serde_json::Value;
pub type StringList = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TerminologyRequestBean {
    #[serde(rename = "newName", skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(rename = "newNamePlural", skip_serializing_if = "Option::is_none")]
    pub new_name_plural: Option<String>,
    #[serde(rename = "originalName", skip_serializing_if = "Option::is_none")]
    pub original_name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TerminologyResponseBean {
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "newName", skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(rename = "newNamePlural", skip_serializing_if = "Option::is_none")]
    pub new_name_plural: Option<String>,
    #[serde(rename = "originalName", skip_serializing_if = "Option::is_none")]
    pub original_name: Option<String>,
    #[serde(rename = "originalNamePlural", skip_serializing_if = "Option::is_none")]
    pub original_name_plural: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tooltip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
pub type TypeParameter = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnmapSprintsBean {
    #[serde(rename = "sprintIds", skip_serializing_if = "Option::is_none")]
    pub sprint_ids: Option<Vec<i64>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserToGroupBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpgradeResultBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAnonymizationRequestBean {
    #[serde(rename = "newOwnerKey", skip_serializing_if = "Option::is_none")]
    pub new_owner_key: Option<String>,
    #[serde(rename = "userKey", skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAnonymizationRerunRequestBean {
    #[serde(rename = "newOwnerKey", skip_serializing_if = "Option::is_none")]
    pub new_owner_key: Option<String>,
    #[serde(rename = "oldUserKey", skip_serializing_if = "Option::is_none")]
    pub old_user_key: Option<String>,
    #[serde(rename = "oldUserName", skip_serializing_if = "Option::is_none")]
    pub old_user_name: Option<String>,
    #[serde(rename = "userKey", skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserWriteBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "applicationKeys", skip_serializing_if = "Option::is_none")]
    pub application_keys: Option<Vec<String>>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
pub type ValidateRequest = String;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValidationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "moveUnfixedIssuesTo", skip_serializing_if = "Option::is_none")]
    pub move_unfixed_issues_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overdue: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    #[serde(rename = "releaseDate", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(rename = "releaseDateSet", skip_serializing_if = "Option::is_none")]
    pub release_date_set: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released: Option<bool>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "startDateSet", skip_serializing_if = "Option::is_none")]
    pub start_date_set: Option<bool>,
    #[serde(rename = "userReleaseDate", skip_serializing_if = "Option::is_none")]
    pub user_release_date: Option<String>,
    #[serde(rename = "userStartDate", skip_serializing_if = "Option::is_none")]
    pub user_start_date: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionUnresolvedIssueCountsBean {
    #[serde(rename = "issuesUnresolvedCount", skip_serializing_if = "Option::is_none")]
    pub issues_unresolved_count: Option<i64>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
pub type VoteBean = serde_json::Value;
pub type VoteWatchResult = serde_json::Value;
pub type WatchersBean = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowMappingBean {
    #[serde(rename = "defaultMapping", skip_serializing_if = "Option::is_none")]
    pub default_mapping: Option<bool>,
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<String>>,
    #[serde(rename = "updateDraftIfNeeded", skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowSchemeRequest {
    #[serde(rename = "defaultWorkflow", skip_serializing_if = "Option::is_none")]
    pub default_workflow: Option<bool>,
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}
pub type WorkflowTransitionResource = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorklogIdsRequestBean {
    ///List of worklog ids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
}
