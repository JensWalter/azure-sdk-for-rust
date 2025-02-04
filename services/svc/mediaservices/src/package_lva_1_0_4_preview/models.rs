#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphInstance {
    pub name: String,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<MediaGraphSystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MediaGraphInstanceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphInstanceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "topologyName", default, skip_serializing_if = "Option::is_none")]
    pub topology_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<MediaGraphParameterDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<media_graph_instance_properties::State>,
}
pub mod media_graph_instance_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Inactive,
        Activating,
        Active,
        Deactivating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphParameterDefinition {
    pub name: String,
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphInstanceCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MediaGraphInstance>,
    #[serde(rename = "@continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphTopologyCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MediaGraphTopology>,
    #[serde(rename = "@continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphTopology {
    pub name: String,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<MediaGraphSystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MediaGraphTopologyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphTopologyProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<MediaGraphParameterDeclaration>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<MediaGraphSource>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processors: Vec<MediaGraphProcessor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sinks: Vec<MediaGraphSink>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphSystemData {
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphParameterDeclaration {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: media_graph_parameter_declaration::Type,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}
pub mod media_graph_parameter_declaration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        String,
        SecretString,
        Int,
        Double,
        Bool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphSource {
    #[serde(rename = "@type")]
    pub type_: String,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphRtspSource {
    #[serde(flatten)]
    pub media_graph_source: MediaGraphSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<media_graph_rtsp_source::Transport>,
    pub endpoint: MediaGraphEndpoint,
}
pub mod media_graph_rtsp_source {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Transport {
        Http,
        Tcp,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphIoTHubMessageSource {
    #[serde(flatten)]
    pub media_graph_source: MediaGraphSource,
    #[serde(rename = "hubInputName", default, skip_serializing_if = "Option::is_none")]
    pub hub_input_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphIoTHubMessageSink {
    #[serde(flatten)]
    pub media_graph_sink: MediaGraphSink,
    #[serde(rename = "hubOutputName")]
    pub hub_output_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphEndpoint {
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<MediaGraphCredentials>,
    pub url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphCredentials {
    #[serde(rename = "@type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphUsernamePasswordCredentials {
    #[serde(flatten)]
    pub media_graph_credentials: MediaGraphCredentials,
    pub username: String,
    pub password: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphHttpHeaderCredentials {
    #[serde(flatten)]
    pub media_graph_credentials: MediaGraphCredentials,
    #[serde(rename = "headerName")]
    pub header_name: String,
    #[serde(rename = "headerValue")]
    pub header_value: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphUnsecuredEndpoint {
    #[serde(flatten)]
    pub media_graph_endpoint: MediaGraphEndpoint,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphTlsEndpoint {
    #[serde(flatten)]
    pub media_graph_endpoint: MediaGraphEndpoint,
    #[serde(rename = "trustedCertificates", default, skip_serializing_if = "Option::is_none")]
    pub trusted_certificates: Option<MediaGraphCertificateSource>,
    #[serde(rename = "validationOptions", default, skip_serializing_if = "Option::is_none")]
    pub validation_options: Option<MediaGraphTlsValidationOptions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphCertificateSource {
    #[serde(rename = "@type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphTlsValidationOptions {
    #[serde(rename = "ignoreHostname", default, skip_serializing_if = "Option::is_none")]
    pub ignore_hostname: Option<String>,
    #[serde(rename = "ignoreSignature", default, skip_serializing_if = "Option::is_none")]
    pub ignore_signature: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphPemCertificateList {
    #[serde(flatten)]
    pub media_graph_certificate_source: MediaGraphCertificateSource,
    pub certificates: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphSink {
    #[serde(rename = "@type")]
    pub type_: String,
    pub name: String,
    pub inputs: Vec<MediaGraphNodeInput>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphNodeInput {
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(rename = "outputSelectors", default, skip_serializing_if = "Vec::is_empty")]
    pub output_selectors: Vec<MediaGraphOutputSelector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphOutputSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<media_graph_output_selector::Property>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<media_graph_output_selector::Operator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
pub mod media_graph_output_selector {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Property {
        #[serde(rename = "mediaType")]
        MediaType,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        #[serde(rename = "is")]
        Is,
        #[serde(rename = "isNot")]
        IsNot,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphFileSink {
    #[serde(flatten)]
    pub media_graph_sink: MediaGraphSink,
    #[serde(rename = "baseDirectoryPath")]
    pub base_directory_path: String,
    #[serde(rename = "fileNamePattern")]
    pub file_name_pattern: String,
    #[serde(rename = "maximumSizeMiB")]
    pub maximum_size_mi_b: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphAssetSink {
    #[serde(flatten)]
    pub media_graph_sink: MediaGraphSink,
    #[serde(rename = "assetNamePattern")]
    pub asset_name_pattern: String,
    #[serde(rename = "segmentLength", default, skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<String>,
    #[serde(rename = "localMediaCachePath")]
    pub local_media_cache_path: String,
    #[serde(rename = "localMediaCacheMaximumSizeMiB")]
    pub local_media_cache_maximum_size_mi_b: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphProcessor {
    #[serde(rename = "@type")]
    pub type_: String,
    pub name: String,
    pub inputs: Vec<MediaGraphNodeInput>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphMotionDetectionProcessor {
    #[serde(flatten)]
    pub media_graph_processor: MediaGraphProcessor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<media_graph_motion_detection_processor::Sensitivity>,
    #[serde(rename = "outputMotionRegion", default, skip_serializing_if = "Option::is_none")]
    pub output_motion_region: Option<bool>,
    #[serde(rename = "eventAggregationWindow", default, skip_serializing_if = "Option::is_none")]
    pub event_aggregation_window: Option<String>,
}
pub mod media_graph_motion_detection_processor {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Sensitivity {
        Low,
        Medium,
        High,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphExtensionProcessorBase {
    #[serde(flatten)]
    pub media_graph_processor: MediaGraphProcessor,
    pub endpoint: MediaGraphEndpoint,
    pub image: MediaGraphImage,
    #[serde(rename = "samplingOptions", default, skip_serializing_if = "Option::is_none")]
    pub sampling_options: Option<MediaGraphSamplingOptions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphCognitiveServicesVisionExtension {
    #[serde(flatten)]
    pub media_graph_extension_processor_base: MediaGraphExtensionProcessorBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphGrpcExtension {
    #[serde(flatten)]
    pub media_graph_extension_processor_base: MediaGraphExtensionProcessorBase,
    #[serde(rename = "dataTransfer")]
    pub data_transfer: MediaGraphGrpcExtensionDataTransfer,
    #[serde(rename = "extensionConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub extension_configuration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphGrpcExtensionDataTransfer {
    #[serde(rename = "sharedMemorySizeMiB", default, skip_serializing_if = "Option::is_none")]
    pub shared_memory_size_mi_b: Option<String>,
    pub mode: media_graph_grpc_extension_data_transfer::Mode,
}
pub mod media_graph_grpc_extension_data_transfer {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Embedded,
        SharedMemory,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphHttpExtension {
    #[serde(flatten)]
    pub media_graph_extension_processor_base: MediaGraphExtensionProcessorBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<MediaGraphImageScale>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<MediaGraphImageFormat>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphSamplingOptions {
    #[serde(rename = "skipSamplesWithoutAnnotation", default, skip_serializing_if = "Option::is_none")]
    pub skip_samples_without_annotation: Option<String>,
    #[serde(rename = "maximumSamplesPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub maximum_samples_per_second: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphImageScale {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<media_graph_image_scale::Mode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
}
pub mod media_graph_image_scale {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        PreserveAspectRatio,
        Pad,
        Stretch,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphImageFormat {
    #[serde(rename = "@type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphImageFormatRaw {
    #[serde(flatten)]
    pub media_graph_image_format: MediaGraphImageFormat,
    #[serde(rename = "pixelFormat")]
    pub pixel_format: media_graph_image_format_raw::PixelFormat,
}
pub mod media_graph_image_format_raw {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PixelFormat {
        Yuv420p,
        Rgb565be,
        Rgb565le,
        Rgb555be,
        Rgb555le,
        Rgb24,
        Bgr24,
        Argb,
        Rgba,
        Abgr,
        Bgra,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphImageFormatJpeg {
    #[serde(flatten)]
    pub media_graph_image_format: MediaGraphImageFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphImageFormatBmp {
    #[serde(flatten)]
    pub media_graph_image_format: MediaGraphImageFormat,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphImageFormatPng {
    #[serde(flatten)]
    pub media_graph_image_format: MediaGraphImageFormat,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphSignalGateProcessor {
    #[serde(flatten)]
    pub media_graph_processor: MediaGraphProcessor,
    #[serde(rename = "activationEvaluationWindow", default, skip_serializing_if = "Option::is_none")]
    pub activation_evaluation_window: Option<String>,
    #[serde(rename = "activationSignalOffset", default, skip_serializing_if = "Option::is_none")]
    pub activation_signal_offset: Option<String>,
    #[serde(rename = "minimumActivationTime", default, skip_serializing_if = "Option::is_none")]
    pub minimum_activation_time: Option<String>,
    #[serde(rename = "maximumActivationTime", default, skip_serializing_if = "Option::is_none")]
    pub maximum_activation_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MethodRequest {
    #[serde(rename = "methodName")]
    pub method_name: String,
    #[serde(rename = "@apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<method_request::ApiVersion>,
}
pub mod method_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ApiVersion {
        #[serde(rename = "2.0")]
        _2_0,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphTopologySetRequest {
    #[serde(flatten)]
    pub method_request: MethodRequest,
    pub graph: MediaGraphTopology,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphTopologySetRequestBody {
    #[serde(flatten)]
    pub method_request: MethodRequest,
    #[serde(flatten)]
    pub media_graph_topology: MediaGraphTopology,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphInstanceSetRequest {
    #[serde(flatten)]
    pub method_request: MethodRequest,
    pub instance: MediaGraphInstance,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphInstanceSetRequestBody {
    #[serde(flatten)]
    pub method_request: MethodRequest,
    #[serde(flatten)]
    pub media_graph_instance: MediaGraphInstance,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemNonSetRequestBase {
    #[serde(flatten)]
    pub method_request: MethodRequest,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphTopologyListRequest {
    #[serde(flatten)]
    pub method_request: MethodRequest,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphTopologyGetRequest {
    #[serde(flatten)]
    pub item_non_set_request_base: ItemNonSetRequestBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphTopologyDeleteRequest {
    #[serde(flatten)]
    pub item_non_set_request_base: ItemNonSetRequestBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphInstanceListRequest {
    #[serde(flatten)]
    pub method_request: MethodRequest,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphInstanceGetRequest {
    #[serde(flatten)]
    pub item_non_set_request_base: ItemNonSetRequestBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphInstanceActivateRequest {
    #[serde(flatten)]
    pub item_non_set_request_base: ItemNonSetRequestBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphInstanceDeActivateRequest {
    #[serde(flatten)]
    pub item_non_set_request_base: ItemNonSetRequestBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGraphInstanceDeleteRequest {
    #[serde(flatten)]
    pub item_non_set_request_base: ItemNonSetRequestBase,
}
