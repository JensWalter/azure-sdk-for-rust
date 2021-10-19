#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ResourceIdentityType>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MoveState {
    AssignmentPending,
    PreparePending,
    PrepareInProgress,
    PrepareFailed,
    MovePending,
    MoveInProgress,
    MoveFailed,
    DiscardInProgress,
    DiscardFailed,
    CommitPending,
    CommitInProgress,
    CommitFailed,
    Committed,
    DeleteSourcePending,
    ResourceMoveCompleted,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MoveResourceInputType {
    MoveResourceId,
    MoveResourceSourceId,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Succeeded,
    Updating,
    Creating,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobName {
    InitialSync,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStatus {
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<JobName>,
    #[serde(rename = "jobProgress", default, skip_serializing_if = "Option::is_none")]
    pub job_progress: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResourceStatus {
    #[serde(rename = "moveState", default, skip_serializing_if = "Option::is_none")]
    pub move_state: Option<MoveState>,
    #[serde(rename = "jobStatus", default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<JobStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<MoveResourceError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResolutionType {
    Manual,
    Automatic,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DependencyType {
    RequiredForPrepare,
    RequiredForMove,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResourceDependency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resolutionStatus", default, skip_serializing_if = "Option::is_none")]
    pub resolution_status: Option<String>,
    #[serde(rename = "resolutionType", default, skip_serializing_if = "Option::is_none")]
    pub resolution_type: Option<ResolutionType>,
    #[serde(rename = "dependencyType", default, skip_serializing_if = "Option::is_none")]
    pub dependency_type: Option<DependencyType>,
    #[serde(rename = "manualResolution", default, skip_serializing_if = "Option::is_none")]
    pub manual_resolution: Option<ManualResolutionProperties>,
    #[serde(rename = "automaticResolution", default, skip_serializing_if = "Option::is_none")]
    pub automatic_resolution: Option<AutomaticResolutionProperties>,
    #[serde(rename = "isOptional", default, skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResourceDependencyOverride {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualResolutionProperties {
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomaticResolutionProperties {
    #[serde(rename = "moveResourceId", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResourceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "sourceId")]
    pub source_id: String,
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "existingTargetId", default, skip_serializing_if = "Option::is_none")]
    pub existing_target_id: Option<String>,
    #[serde(rename = "resourceSettings", default, skip_serializing_if = "Option::is_none")]
    pub resource_settings: Option<ResourceSettings>,
    #[serde(rename = "sourceResourceSettings", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_settings: Option<ResourceSettings>,
    #[serde(rename = "moveStatus", default, skip_serializing_if = "Option::is_none")]
    pub move_status: Option<serde_json::Value>,
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<MoveResourceDependency>,
    #[serde(rename = "dependsOnOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on_overrides: Vec<MoveResourceDependencyOverride>,
    #[serde(rename = "isResolveRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_resolve_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MoveResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SummaryCollection {
    #[serde(rename = "fieldName", default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub summary: Vec<Summary>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveCollectionProperties {
    #[serde(rename = "sourceRegion")]
    pub source_region: String,
    #[serde(rename = "targetRegion")]
    pub target_region: String,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveCollection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MoveCollectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateMoveCollectionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrepareRequest {
    #[serde(rename = "validateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[serde(rename = "moveResources")]
    pub move_resources: Vec<String>,
    #[serde(rename = "moveResourceInputType", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_input_type: Option<MoveResourceInputType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceMoveRequest {
    #[serde(rename = "validateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[serde(rename = "moveResources")]
    pub move_resources: Vec<String>,
    #[serde(rename = "moveResourceInputType", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_input_type: Option<MoveResourceInputType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitRequest {
    #[serde(rename = "validateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[serde(rename = "moveResources")]
    pub move_resources: Vec<String>,
    #[serde(rename = "moveResourceInputType", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_input_type: Option<MoveResourceInputType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscardRequest {
    #[serde(rename = "validateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[serde(rename = "moveResources")]
    pub move_resources: Vec<String>,
    #[serde(rename = "moveResourceInputType", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_input_type: Option<MoveResourceInputType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkRemoveRequest {
    #[serde(rename = "validateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[serde(rename = "moveResources", default, skip_serializing_if = "Vec::is_empty")]
    pub move_resources: Vec<String>,
    #[serde(rename = "moveResourceInputType", default, skip_serializing_if = "Option::is_none")]
    pub move_resource_input_type: Option<MoveResourceInputType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResourceFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MoveResourceFilterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResourceFilterProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnresolvedDependenciesFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UnresolvedDependenciesFilterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnresolvedDependenciesFilterProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResourceCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MoveResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(rename = "summaryCollection", default, skip_serializing_if = "Option::is_none")]
    pub summary_collection: Option<SummaryCollection>,
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveCollectionResultList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MoveCollection>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnresolvedDependency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnresolvedDependencyCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UnresolvedDependency>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(rename = "summaryCollection", default, skip_serializing_if = "Option::is_none")]
    pub summary_collection: Option<SummaryCollection>,
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequiredForResourcesCollection {
    #[serde(rename = "sourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub source_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Display {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsDiscoveryProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsDiscovery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationsDiscoveryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsDiscoveryCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationsDiscovery>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResourceError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MoveResourceErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResourceErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<MoveResourceErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSettings {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "targetResourceName")]
    pub target_resource_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<virtual_machine_resource_settings::TargetAvailabilityZone>,
    #[serde(rename = "targetVmSize", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_size: Option<String>,
    #[serde(rename = "targetAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_set_id: Option<String>,
}
pub mod virtual_machine_resource_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetAvailabilityZone {
        #[serde(rename = "1")]
        N1,
        #[serde(rename = "2")]
        N2,
        #[serde(rename = "3")]
        N3,
        #[serde(rename = "NA")]
        Na,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilitySetResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[serde(rename = "faultDomain", default, skip_serializing_if = "Option::is_none")]
    pub fault_domain: Option<i32>,
    #[serde(rename = "updateDomain", default, skip_serializing_if = "Option::is_none")]
    pub update_domain: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubnetResourceSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "addressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub address_prefix: Option<String>,
    #[serde(rename = "networkSecurityGroup", default, skip_serializing_if = "Option::is_none")]
    pub network_security_group: Option<NsgReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[serde(rename = "enableDdosProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_ddos_protection: Option<bool>,
    #[serde(rename = "addressSpace", default, skip_serializing_if = "Vec::is_empty")]
    pub address_space: Vec<String>,
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<SubnetResourceSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceReference {
    #[serde(rename = "sourceArmResourceId")]
    pub source_arm_resource_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResourceReference {
    #[serde(flatten)]
    pub azure_resource_reference: AzureResourceReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterfaceResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[serde(rename = "ipConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_configurations: Vec<NicIpConfigurationResourceSettings>,
    #[serde(rename = "enableAcceleratedNetworking", default, skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_networking: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubnetReference {
    #[serde(flatten)]
    pub proxy_resource_reference: ProxyResourceReference,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerBackendAddressPoolReference {
    #[serde(flatten)]
    pub proxy_resource_reference: ProxyResourceReference,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerNatRuleReference {
    #[serde(flatten)]
    pub proxy_resource_reference: ProxyResourceReference,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NsgReference {
    #[serde(flatten)]
    pub azure_resource_reference: AzureResourceReference,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicIpReference {
    #[serde(flatten)]
    pub azure_resource_reference: AzureResourceReference,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NicIpConfigurationResourceSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "privateIpAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubnetReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "loadBalancerBackendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_backend_address_pools: Vec<LoadBalancerBackendAddressPoolReference>,
    #[serde(rename = "loadBalancerNatRules", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_nat_rules: Vec<LoadBalancerNatRuleReference>,
    #[serde(rename = "publicIp", default, skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<PublicIpReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NsgSecurityRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub destination_address_prefix: Option<String>,
    #[serde(rename = "destinationPortRange", default, skip_serializing_if = "Option::is_none")]
    pub destination_port_range: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "sourceAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub source_address_prefix: Option<String>,
    #[serde(rename = "sourcePortRange", default, skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSecurityGroupResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[serde(rename = "securityRules", default, skip_serializing_if = "Vec::is_empty")]
    pub security_rules: Vec<NsgSecurityRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LbFrontendIpConfigurationResourceSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "privateIpAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubnetReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LbBackendAddressPoolResourceSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "frontendIPConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub frontend_ip_configurations: Vec<LbFrontendIpConfigurationResourceSettings>,
    #[serde(rename = "backendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub backend_address_pools: Vec<LbBackendAddressPoolResourceSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ZoneRedundant {
    Enable,
    Disable,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlElasticPoolResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<ZoneRedundant>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabaseResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<ZoneRedundant>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicIpAddressResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
    #[serde(rename = "domainNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub domain_name_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "publicIpAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_allocation_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskEncryptionSetResourceSettings {
    #[serde(flatten)]
    pub resource_settings: ResourceSettings,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationStatusError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationStatusProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<OperationStatusError>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<OperationErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<MoveErrorInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveErrorInfo {
    #[serde(rename = "moveResources", default, skip_serializing_if = "Vec::is_empty")]
    pub move_resources: Vec<AffectedMoveResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AffectedMoveResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "moveResources", default, skip_serializing_if = "Vec::is_empty")]
    pub move_resources: Vec<AffectedMoveResource>,
}
