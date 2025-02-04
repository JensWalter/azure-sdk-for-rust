#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionsList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<ActionResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRequest {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionRequestProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRequestProperties {
    #[serde(flatten)]
    pub action_properties_base: ActionPropertiesBase,
    #[serde(rename = "triggerUri")]
    pub trigger_uri: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionResponse {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionResponseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionResponseProperties {
    #[serde(flatten)]
    pub action_properties_base: ActionPropertiesBase,
    #[serde(rename = "workflowId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionPropertiesBase {
    #[serde(rename = "logicAppResourceId")]
    pub logic_app_resource_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRule {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    pub kind: AlertRuleKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AlertRuleKind {
    Scheduled,
    MicrosoftSecurityIncidentCreation,
    Fusion,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleTemplate {
    #[serde(flatten)]
    pub resource: Resource,
    pub kind: AlertRuleKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleTemplateDataSource {
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(rename = "dataTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub data_types: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AlertRuleTemplateStatus {
    Installed,
    Available,
    NotAvailable,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AlertRuleTriggerOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRulesList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<AlertRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleTemplatesList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<AlertRuleTemplate>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AlertSeverity {
    High,
    Medium,
    Low,
    Informational,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AttackTactic {
    InitialAccess,
    Execution,
    Persistence,
    PrivilegeEscalation,
    DefenseEvasion,
    CredentialAccess,
    Discovery,
    LateralMovement,
    Collection,
    Exfiltration,
    CommandAndControl,
    Impact,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FusionAlertRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionAlertRuleProperties {
    #[serde(rename = "alertRuleTemplateName")]
    pub alert_rule_template_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub enabled: bool,
    #[serde(rename = "lastModifiedUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_utc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tactics: Vec<AttackTactic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FusionAlertRuleTemplateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionAlertRuleTemplateProperties {
    #[serde(rename = "alertRulesCreatedByTemplateCount", default, skip_serializing_if = "Option::is_none")]
    pub alert_rules_created_by_template_count: Option<i32>,
    #[serde(rename = "createdDateUTC", default, skip_serializing_if = "Option::is_none")]
    pub created_date_utc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "requiredDataConnectors", default, skip_serializing_if = "Vec::is_empty")]
    pub required_data_connectors: Vec<AlertRuleTemplateDataSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertRuleTemplateStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tactics: Vec<AttackTactic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MicrosoftSecurityIncidentCreationAlertRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleCommonProperties {
    #[serde(rename = "displayNamesFilter", default, skip_serializing_if = "Vec::is_empty")]
    pub display_names_filter: Vec<String>,
    #[serde(rename = "displayNamesExcludeFilter", default, skip_serializing_if = "Vec::is_empty")]
    pub display_names_exclude_filter: Vec<String>,
    #[serde(rename = "productFilter")]
    pub product_filter: MicrosoftSecurityProductName,
    #[serde(rename = "severitiesFilter", default, skip_serializing_if = "Vec::is_empty")]
    pub severities_filter: Vec<AlertSeverity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleProperties {
    #[serde(flatten)]
    pub microsoft_security_incident_creation_alert_rule_common_properties: MicrosoftSecurityIncidentCreationAlertRuleCommonProperties,
    #[serde(rename = "alertRuleTemplateName", default, skip_serializing_if = "Option::is_none")]
    pub alert_rule_template_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub enabled: bool,
    #[serde(rename = "lastModifiedUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_utc: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MicrosoftSecurityIncidentCreationAlertRuleTemplateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleTemplateProperties {
    #[serde(rename = "alertRulesCreatedByTemplateCount", default, skip_serializing_if = "Option::is_none")]
    pub alert_rules_created_by_template_count: Option<i32>,
    #[serde(rename = "createdDateUTC", default, skip_serializing_if = "Option::is_none")]
    pub created_date_utc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "requiredDataConnectors", default, skip_serializing_if = "Vec::is_empty")]
    pub required_data_connectors: Vec<AlertRuleTemplateDataSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertRuleTemplateStatus>,
    #[serde(rename = "displayNamesFilter", default, skip_serializing_if = "Vec::is_empty")]
    pub display_names_filter: Vec<String>,
    #[serde(rename = "displayNamesExcludeFilter", default, skip_serializing_if = "Vec::is_empty")]
    pub display_names_exclude_filter: Vec<String>,
    #[serde(rename = "productFilter")]
    pub product_filter: MicrosoftSecurityProductName,
    #[serde(rename = "severitiesFilter", default, skip_serializing_if = "Vec::is_empty")]
    pub severities_filter: Vec<AlertSeverity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MicrosoftSecurityProductName {
    #[serde(rename = "Microsoft Cloud App Security")]
    MicrosoftCloudAppSecurity,
    #[serde(rename = "Azure Security Center")]
    AzureSecurityCenter,
    #[serde(rename = "Azure Advanced Threat Protection")]
    AzureAdvancedThreatProtection,
    #[serde(rename = "Azure Active Directory Identity Protection")]
    AzureActiveDirectoryIdentityProtection,
    #[serde(rename = "Azure Security Center for IoT")]
    AzureSecurityCenterForIoT,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduledAlertRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRuleCommonProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "queryFrequency", default, skip_serializing_if = "Option::is_none")]
    pub query_frequency: Option<String>,
    #[serde(rename = "queryPeriod", default, skip_serializing_if = "Option::is_none")]
    pub query_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverity>,
    #[serde(rename = "triggerOperator", default, skip_serializing_if = "Option::is_none")]
    pub trigger_operator: Option<AlertRuleTriggerOperator>,
    #[serde(rename = "triggerThreshold", default, skip_serializing_if = "Option::is_none")]
    pub trigger_threshold: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRuleProperties {
    #[serde(flatten)]
    pub scheduled_alert_rule_common_properties: ScheduledAlertRuleCommonProperties,
    #[serde(rename = "alertRuleTemplateName", default, skip_serializing_if = "Option::is_none")]
    pub alert_rule_template_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub enabled: bool,
    #[serde(rename = "lastModifiedUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_utc: Option<String>,
    #[serde(rename = "suppressionDuration")]
    pub suppression_duration: String,
    #[serde(rename = "suppressionEnabled")]
    pub suppression_enabled: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tactics: Vec<AttackTactic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRuleTemplateProperties {
    #[serde(rename = "alertRulesCreatedByTemplateCount", default, skip_serializing_if = "Option::is_none")]
    pub alert_rules_created_by_template_count: Option<i32>,
    #[serde(rename = "createdDateUTC", default, skip_serializing_if = "Option::is_none")]
    pub created_date_utc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "requiredDataConnectors", default, skip_serializing_if = "Vec::is_empty")]
    pub required_data_connectors: Vec<AlertRuleTemplateDataSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertRuleTemplateStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "queryFrequency", default, skip_serializing_if = "Option::is_none")]
    pub query_frequency: Option<String>,
    #[serde(rename = "queryPeriod", default, skip_serializing_if = "Option::is_none")]
    pub query_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverity>,
    #[serde(rename = "triggerOperator", default, skip_serializing_if = "Option::is_none")]
    pub trigger_operator: Option<AlertRuleTriggerOperator>,
    #[serde(rename = "triggerThreshold", default, skip_serializing_if = "Option::is_none")]
    pub trigger_threshold: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tactics: Vec<AttackTactic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduledAlertRuleTemplateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Incident {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IncidentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAdditionalData {
    #[serde(rename = "alertsCount", default, skip_serializing_if = "Option::is_none")]
    pub alerts_count: Option<i32>,
    #[serde(rename = "bookmarksCount", default, skip_serializing_if = "Option::is_none")]
    pub bookmarks_count: Option<i32>,
    #[serde(rename = "commentsCount", default, skip_serializing_if = "Option::is_none")]
    pub comments_count: Option<i32>,
    #[serde(rename = "alertProductNames", default, skip_serializing_if = "Vec::is_empty")]
    pub alert_product_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tactics: Vec<AttackTactic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentComment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IncidentCommentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentCommentList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<IncidentComment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentCommentProperties {
    #[serde(rename = "createdTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub created_time_utc: Option<String>,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<ClientInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentLabel {
    #[serde(rename = "labelName")]
    pub label_name: String,
    #[serde(rename = "labelType", default, skip_serializing_if = "Option::is_none")]
    pub label_type: Option<incident_label::LabelType>,
}
pub mod incident_label {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LabelType {
        User,
        System,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<Incident>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentOwnerInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "assignedTo", default, skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentProperties {
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<IncidentAdditionalData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<incident_properties::Classification>,
    #[serde(rename = "classificationComment", default, skip_serializing_if = "Option::is_none")]
    pub classification_comment: Option<String>,
    #[serde(rename = "classificationReason", default, skip_serializing_if = "Option::is_none")]
    pub classification_reason: Option<incident_properties::ClassificationReason>,
    #[serde(rename = "createdTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub created_time_utc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "firstActivityTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub first_activity_time_utc: Option<String>,
    #[serde(rename = "incidentUrl", default, skip_serializing_if = "Option::is_none")]
    pub incident_url: Option<String>,
    #[serde(rename = "incidentNumber", default, skip_serializing_if = "Option::is_none")]
    pub incident_number: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<IncidentLabel>,
    #[serde(rename = "lastActivityTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_activity_time_utc: Option<String>,
    #[serde(rename = "lastModifiedTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time_utc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<IncidentOwnerInfo>,
    #[serde(rename = "relatedAnalyticRuleIds", default, skip_serializing_if = "Vec::is_empty")]
    pub related_analytic_rule_ids: Vec<String>,
    pub severity: IncidentSeverityEnum,
    pub status: incident_properties::Status,
    pub title: String,
}
pub mod incident_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Classification {
        Undetermined,
        TruePositive,
        BenignPositive,
        FalsePositive,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ClassificationReason {
        SuspiciousActivity,
        SuspiciousButExpected,
        IncorrectAlertLogic,
        InaccurateData,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        New,
        Active,
        Closed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IncidentSeverityEnum {
    High,
    Medium,
    Low,
    Informational,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeConsent {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeConsentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeConsentList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<OfficeConsent>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeConsentProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceWithEtag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreatIntelligence {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(rename = "providerName", default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "reportLink", default, skip_serializing_if = "Option::is_none")]
    pub report_link: Option<String>,
    #[serde(rename = "threatDescription", default, skip_serializing_if = "Option::is_none")]
    pub threat_description: Option<String>,
    #[serde(rename = "threatName", default, skip_serializing_if = "Option::is_none")]
    pub threat_name: Option<String>,
    #[serde(rename = "threatType", default, skip_serializing_if = "Option::is_none")]
    pub threat_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "objectId")]
    pub object_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bookmark {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BookmarkProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BookmarkList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<Bookmark>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BookmarkProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserInfo>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub query: String,
    #[serde(rename = "queryResult", default, skip_serializing_if = "Option::is_none")]
    pub query_result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<UserInfo>,
    #[serde(rename = "eventTime", default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    #[serde(rename = "queryStartTime", default, skip_serializing_if = "Option::is_none")]
    pub query_start_time: Option<String>,
    #[serde(rename = "queryEndTime", default, skip_serializing_if = "Option::is_none")]
    pub query_end_time: Option<String>,
    #[serde(rename = "incidentInfo", default, skip_serializing_if = "Option::is_none")]
    pub incident_info: Option<IncidentInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentInfo {
    #[serde(rename = "incidentId", default, skip_serializing_if = "Option::is_none")]
    pub incident_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<IncidentSeverityEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "relationName", default, skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
}
pub type Label = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AadDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadDataConnectorProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "dataTypes", default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AlertsDataTypeOfDataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AatpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AatpDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AatpDataConnectorProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "dataTypes", default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AlertsDataTypeOfDataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AscDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AscDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AscDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsDataTypeOfDataConnector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alerts: Option<DataConnectorDataTypeCommon>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AwsCloudTrailDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnectorDataTypes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnectorProperties {
    #[serde(rename = "awsRoleArn", default, skip_serializing_if = "Option::is_none")]
    pub aws_role_arn: Option<String>,
    #[serde(rename = "dataTypes", default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AwsCloudTrailDataConnectorDataTypes>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnector {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    pub kind: DataConnectorKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataConnectorKind {
    AzureActiveDirectory,
    AzureSecurityCenter,
    MicrosoftCloudAppSecurity,
    ThreatIntelligence,
    Office365,
    AmazonWebServicesCloudTrail,
    AzureAdvancedThreatProtection,
    MicrosoftDefenderAdvancedThreatProtection,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<DataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorDataTypeCommon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<data_connector_data_type_common::State>,
}
pub mod data_connector_data_type_common {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorTenantId {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorWithAlertsProperties {
    #[serde(rename = "dataTypes", default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AlertsDataTypeOfDataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<McasDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnectorDataTypes {
    #[serde(flatten)]
    pub alerts_data_type_of_data_connector: AlertsDataTypeOfDataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alerts: Option<DataConnectorDataTypeCommon>,
    #[serde(rename = "discoveryLogs", default, skip_serializing_if = "Option::is_none")]
    pub discovery_logs: Option<DataConnectorDataTypeCommon>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnectorProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "dataTypes", default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<McasDataConnectorDataTypes>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MdatpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MdatpDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MdatpDataConnectorProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "dataTypes", default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AlertsDataTypeOfDataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TiDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnectorDataTypes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicators: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnectorProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "tipLookbackPeriod", default, skip_serializing_if = "Option::is_none")]
    pub tip_lookback_period: Option<String>,
    #[serde(rename = "dataTypes", default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<TiDataConnectorDataTypes>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnectorDataTypes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange: Option<serde_json::Value>,
    #[serde(rename = "sharePoint", default, skip_serializing_if = "Option::is_none")]
    pub share_point: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teams: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnectorProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "dataTypes", default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<OfficeDataConnectorDataTypes>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
