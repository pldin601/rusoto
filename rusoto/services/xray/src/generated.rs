// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>An alias for an edge.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Alias {
    /// <p>The canonical name of the alias.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of names for the alias, including the canonical name.</p>
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The type of the alias.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Value of a segment annotation. Has one of three value types: Number, Boolean, or String.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnnotationValue {
    /// <p>Value for a Boolean annotation.</p>
    #[serde(rename = "booleanValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    /// <p>Value for a Number annotation.</p>
    #[serde(rename = "numberValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_value: Option<f64>,
    /// <p>Value for a String annotation.</p>
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

/// <p>The service within the service graph that has anomalously high fault rates. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnomalousService {
    #[serde(rename = "serviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<ServiceId>,
}

/// <p>A list of Availability Zones corresponding to the segments in a trace.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AvailabilityZoneDetail {
    /// <p>The name of a corresponding Availability Zone.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p><p/></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BackendConnectionErrors {
    /// <p><p/></p>
    #[serde(rename = "connectionRefusedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_refused_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "hTTPCode4XXCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code_4xx_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "hTTPCode5XXCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code_5xx_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "otherCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "timeoutCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "unknownHostCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_host_count: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetTracesRequest {
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specify the trace IDs of requests for which to retrieve segments.</p>
    #[serde(rename = "traceIds")]
    pub trace_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetTracesResult {
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Full traces for the specified requests.</p>
    #[serde(rename = "traces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traces: Option<Vec<Trace>>,
    /// <p>Trace IDs of requests that haven't been processed.</p>
    #[serde(rename = "unprocessedTraceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_trace_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGroupRequest {
    /// <p>The filter expression defining criteria by which to group traces.</p>
    #[serde(rename = "filterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>The case-sensitive name of the new group. Default is a reserved name and names must be unique.</p>
    #[serde(rename = "groupName")]
    pub group_name: String,
    /// <p><p>The structure containing configurations related to insights.</p> <ul> <li> <p>The InsightsEnabled boolean can be set to true to enable insights for the new group or false to disable insights for the new group.</p> </li> <li> <p>The NotifcationsEnabled boolean can be set to true to enable insights notifications for the new group. Notifications may only be enabled on a group with InsightsEnabled set to true.</p> </li> </ul></p>
    #[serde(rename = "insightsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_configuration: Option<InsightsConfiguration>,
    /// <p><p>A map that contains one or more tag keys and tag values to attach to an X-Ray group. For more information about ways to use tags, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS resources</a> in the <i>AWS General Reference</i>.</p> <p>The following restrictions apply to tags:</p> <ul> <li> <p>Maximum number of user-applied tags per resource: 50</p> </li> <li> <p>Maximum tag key length: 128 Unicode characters</p> </li> <li> <p>Maximum tag value length: 256 Unicode characters</p> </li> <li> <p>Valid values for key and value: a-z, A-Z, 0-9, space, and the following characters: _ . : / = + - and @</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Don&#39;t use <code>aws:</code> as a prefix for keys; it&#39;s reserved for AWS use.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGroupResult {
    /// <p>The group that was created. Contains the name of the group that was created, the Amazon Resource Name (ARN) of the group that was generated based on the group name, the filter expression, and the insight configuration that was assigned to the group.</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSamplingRuleRequest {
    /// <p>The rule definition.</p>
    #[serde(rename = "samplingRule")]
    pub sampling_rule: SamplingRule,
    /// <p><p>A map that contains one or more tag keys and tag values to attach to an X-Ray sampling rule. For more information about ways to use tags, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS resources</a> in the <i>AWS General Reference</i>.</p> <p>The following restrictions apply to tags:</p> <ul> <li> <p>Maximum number of user-applied tags per resource: 50</p> </li> <li> <p>Maximum tag key length: 128 Unicode characters</p> </li> <li> <p>Maximum tag value length: 256 Unicode characters</p> </li> <li> <p>Valid values for key and value: a-z, A-Z, 0-9, space, and the following characters: _ . : / = + - and @</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Don&#39;t use <code>aws:</code> as a prefix for keys; it&#39;s reserved for AWS use.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSamplingRuleResult {
    /// <p>The saved rule definition and metadata.</p>
    #[serde(rename = "samplingRuleRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_record: Option<SamplingRuleRecord>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGroupRequest {
    /// <p>The ARN of the group that was generated on creation.</p>
    #[serde(rename = "groupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The case-sensitive name of the group.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGroupResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSamplingRuleRequest {
    /// <p>The ARN of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "ruleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    /// <p>The name of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "ruleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSamplingRuleResult {
    /// <p>The deleted rule definition and metadata.</p>
    #[serde(rename = "samplingRuleRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_record: Option<SamplingRuleRecord>,
}

/// <p>Information about a connection between two services.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Edge {
    /// <p>Aliases for the edge.</p>
    #[serde(rename = "aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
    /// <p>The end time of the last segment on the edge.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Identifier of the edge. Unique within a service map.</p>
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i64>,
    /// <p>A histogram that maps the spread of client response times on an edge.</p>
    #[serde(rename = "responseTimeHistogram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_histogram: Option<Vec<HistogramEntry>>,
    /// <p>The start time of the first segment on the edge.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>Response statistics for segments on the edge.</p>
    #[serde(rename = "summaryStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_statistics: Option<EdgeStatistics>,
}

/// <p>Response statistics for an edge.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EdgeStatistics {
    /// <p>Information about requests that failed with a 4xx Client Error status code.</p>
    #[serde(rename = "errorStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_statistics: Option<ErrorStatistics>,
    /// <p>Information about requests that failed with a 5xx Server Error status code.</p>
    #[serde(rename = "faultStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_statistics: Option<FaultStatistics>,
    /// <p>The number of requests that completed with a 2xx Success status code.</p>
    #[serde(rename = "okCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_count: Option<i64>,
    /// <p>The total number of completed requests.</p>
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// <p>The aggregate response time of completed requests.</p>
    #[serde(rename = "totalResponseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_response_time: Option<f64>,
}

/// <p>A configuration document that specifies encryption configuration settings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EncryptionConfig {
    /// <p>The ID of the customer master key (CMK) used for encryption, if applicable.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The encryption status. While the status is <code>UPDATING</code>, X-Ray may encrypt data with a combination of the new and old settings.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of encryption. Set to <code>KMS</code> for encryption with CMKs. Set to <code>NONE</code> for default encryption.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The root cause of a trace summary error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorRootCause {
    /// <p>A flag that denotes that the root cause impacts the trace client.</p>
    #[serde(rename = "clientImpacting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_impacting: Option<bool>,
    /// <p>A list of services corresponding to an error. A service identifies a segment and it contains a name, account ID, type, and inferred flag.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ErrorRootCauseService>>,
}

/// <p>A collection of segments and corresponding subsegments associated to a trace summary error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorRootCauseEntity {
    /// <p>The types and messages of the exceptions.</p>
    #[serde(rename = "exceptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<Vec<RootCauseException>>,
    /// <p>The name of the entity.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A flag that denotes a remote subsegment.</p>
    #[serde(rename = "remote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
}

/// <p>A collection of fields identifying the services in a trace summary error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorRootCauseService {
    /// <p>The account ID associated to the service.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The path of root cause entities found on the service. </p>
    #[serde(rename = "entityPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<Vec<ErrorRootCauseEntity>>,
    /// <p>A Boolean value indicating if the service is inferred from the trace.</p>
    #[serde(rename = "inferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,
    /// <p>The service name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A collection of associated service names.</p>
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The type associated to the service.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about requests that failed with a 4xx Client Error status code.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorStatistics {
    /// <p>The number of requests that failed with untracked 4xx Client Error status codes.</p>
    #[serde(rename = "otherCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_count: Option<i64>,
    /// <p>The number of requests that failed with a 419 throttling status code.</p>
    #[serde(rename = "throttleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_count: Option<i64>,
    /// <p>The total number of requests that failed with a 4xx Client Error status code.</p>
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// <p>The root cause information for a trace summary fault.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaultRootCause {
    /// <p>A flag that denotes that the root cause impacts the trace client.</p>
    #[serde(rename = "clientImpacting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_impacting: Option<bool>,
    /// <p>A list of corresponding services. A service identifies a segment and it contains a name, account ID, type, and inferred flag.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<FaultRootCauseService>>,
}

/// <p>A collection of segments and corresponding subsegments associated to a trace summary fault error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaultRootCauseEntity {
    /// <p>The types and messages of the exceptions.</p>
    #[serde(rename = "exceptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<Vec<RootCauseException>>,
    /// <p>The name of the entity.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A flag that denotes a remote subsegment.</p>
    #[serde(rename = "remote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
}

/// <p>A collection of fields identifying the services in a trace summary fault.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaultRootCauseService {
    /// <p>The account ID associated to the service.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The path of root cause entities found on the service. </p>
    #[serde(rename = "entityPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<Vec<FaultRootCauseEntity>>,
    /// <p>A Boolean value indicating if the service is inferred from the trace.</p>
    #[serde(rename = "inferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,
    /// <p>The service name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A collection of associated service names.</p>
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The type associated to the service.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about requests that failed with a 5xx Server Error status code.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaultStatistics {
    /// <p>The number of requests that failed with untracked 5xx Server Error status codes.</p>
    #[serde(rename = "otherCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_count: Option<i64>,
    /// <p>The total number of requests that failed with a 5xx Server Error status code.</p>
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// <p>The predicted high and low fault count. This is used to determine if a service has become anomalous and if an insight should be created.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ForecastStatistics {
    /// <p>The upper limit of fault counts for a service.</p>
    #[serde(rename = "faultCountHigh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_count_high: Option<i64>,
    /// <p>The lower limit of fault counts for a service.</p>
    #[serde(rename = "faultCountLow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_count_low: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEncryptionConfigRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEncryptionConfigResult {
    /// <p>The encryption configuration document.</p>
    #[serde(rename = "encryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGroupRequest {
    /// <p>The ARN of the group that was generated on creation.</p>
    #[serde(rename = "groupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The case-sensitive name of the group.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupResult {
    /// <p>The group that was requested. Contains the name of the group, the ARN of the group, the filter expression, and the insight configuration assigned to the group.</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGroupsRequest {
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupsResult {
    /// <p>The collection of all active groups.</p>
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupSummary>>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInsightEventsRequest {
    /// <p>The insight's unique identifier. Use the GetInsightSummaries action to retrieve an InsightId.</p>
    #[serde(rename = "insightId")]
    pub insight_id: String,
    /// <p>Used to retrieve at most the specified value of events.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Specify the pagination token returned by a previous request to retrieve the next page of events. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInsightEventsResult {
    /// <p>A detailed description of the event. This includes the time of the event, client and root cause impact statistics, and the top anomalous service at the time of the event.</p>
    #[serde(rename = "insightEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_events: Option<Vec<InsightEvent>>,
    /// <p>Use this token to retrieve the next page of insight events.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInsightImpactGraphRequest {
    /// <p>The estimated end time of the insight, in Unix time seconds. The EndTime is exclusive of the value provided. The time range between the start time and end time can't be more than six hours. </p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The insight's unique identifier. Use the GetInsightSummaries action to retrieve an InsightId.</p>
    #[serde(rename = "insightId")]
    pub insight_id: String,
    /// <p>Specify the pagination token returned by a previous request to retrieve the next page of results. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The estimated start time of the insight, in Unix time seconds. The StartTime is inclusive of the value provided and can't be more than 30 days old.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInsightImpactGraphResult {
    /// <p>The provided end time. </p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The insight's unique identifier.</p>
    #[serde(rename = "insightId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_id: Option<String>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The time, in Unix seconds, at which the service graph ended.</p>
    #[serde(rename = "serviceGraphEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_graph_end_time: Option<f64>,
    /// <p>The time, in Unix seconds, at which the service graph started.</p>
    #[serde(rename = "serviceGraphStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_graph_start_time: Option<f64>,
    /// <p>The AWS instrumented services related to the insight.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<InsightImpactGraphService>>,
    /// <p>The provided start time.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInsightRequest {
    /// <p>The insight's unique identifier. Use the GetInsightSummaries action to retrieve an InsightId.</p>
    #[serde(rename = "insightId")]
    pub insight_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInsightResult {
    /// <p>The summary information of an insight.</p>
    #[serde(rename = "insight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<Insight>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInsightSummariesRequest {
    /// <p>The end of the time frame in which the insights ended. The end time can't be more than 30 days old.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the group. Required if the GroupName isn't provided.</p>
    #[serde(rename = "groupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The name of the group. Required if the GroupARN isn't provided.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The maximum number of results to display.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The beginning of the time frame in which the insights started. The start time can't be more than 30 days old.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>The list of insight states. </p>
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInsightSummariesResult {
    /// <p>The summary of each insight within the group matching the provided filters. The summary contains the InsightID, start and end time, the root cause service, the root cause and client impact statistics, the top anomalous services, and the status of the insight.</p>
    #[serde(rename = "insightSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_summaries: Option<Vec<InsightSummary>>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSamplingRulesRequest {
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSamplingRulesResult {
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Rule definitions and metadata.</p>
    #[serde(rename = "samplingRuleRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_records: Option<Vec<SamplingRuleRecord>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSamplingStatisticSummariesRequest {
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSamplingStatisticSummariesResult {
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the number of requests instrumented for each sampling rule.</p>
    #[serde(rename = "samplingStatisticSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_statistic_summaries: Option<Vec<SamplingStatisticSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSamplingTargetsRequest {
    /// <p>Information about rules that the service is using to sample requests.</p>
    #[serde(rename = "samplingStatisticsDocuments")]
    pub sampling_statistics_documents: Vec<SamplingStatisticsDocument>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSamplingTargetsResult {
    /// <p>The last time a user changed the sampling rule configuration. If the sampling rule configuration changed since the service last retrieved it, the service should call <a>GetSamplingRules</a> to get the latest version.</p>
    #[serde(rename = "lastRuleModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rule_modification: Option<f64>,
    /// <p>Updated rules that the service should use to sample requests.</p>
    #[serde(rename = "samplingTargetDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_target_documents: Option<Vec<SamplingTargetDocument>>,
    /// <p>Information about <a>SamplingStatisticsDocument</a> that X-Ray could not process.</p>
    #[serde(rename = "unprocessedStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_statistics: Option<Vec<UnprocessedStatistics>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetServiceGraphRequest {
    /// <p>The end of the timeframe for which to generate a graph.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The Amazon Resource Name (ARN) of a group based on which you want to generate a graph.</p>
    #[serde(rename = "groupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The name of a group based on which you want to generate a graph.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The start of the time frame for which to generate a graph.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetServiceGraphResult {
    /// <p>A flag indicating whether the group's filter expression has been consistent, or if the returned service graph may show traces from an older version of the group's filter expression.</p>
    #[serde(rename = "containsOldGroupVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_old_group_versions: Option<bool>,
    /// <p>The end of the time frame for which the graph was generated.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The services that have processed a traced request during the specified time frame.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
    /// <p>The start of the time frame for which the graph was generated.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTimeSeriesServiceStatisticsRequest {
    /// <p>The end of the time frame for which to aggregate statistics.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>A filter expression defining entities that will be aggregated for statistics. Supports ID, service, and edge functions. If no selector expression is specified, edge statistics are returned. </p>
    #[serde(rename = "entitySelectorExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_selector_expression: Option<String>,
    /// <p>The forecasted high and low fault count values. Forecast enabled requests require the EntitySelectorExpression ID be provided.</p>
    #[serde(rename = "forecastStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_statistics: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the group for which to pull statistics from.</p>
    #[serde(rename = "groupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The case-sensitive name of the group for which to pull statistics from.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Aggregation period in seconds.</p>
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// <p>The start of the time frame for which to aggregate statistics.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTimeSeriesServiceStatisticsResult {
    /// <p>A flag indicating whether or not a group's filter expression has been consistent, or if a returned aggregation might show statistics from an older version of the group's filter expression.</p>
    #[serde(rename = "containsOldGroupVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_old_group_versions: Option<bool>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The collection of statistics.</p>
    #[serde(rename = "timeSeriesServiceStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_service_statistics: Option<Vec<TimeSeriesServiceStatistics>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTraceGraphRequest {
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Trace IDs of requests for which to generate a service graph.</p>
    #[serde(rename = "traceIds")]
    pub trace_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTraceGraphResult {
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The services that have processed one of the specified requests.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTraceSummariesRequest {
    /// <p>The end of the time frame for which to retrieve traces.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>Specify a filter expression to retrieve trace summaries for services or requests that meet certain requirements.</p>
    #[serde(rename = "filterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>Specify the pagination token returned by a previous request to retrieve the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Set to <code>true</code> to get summaries for only a subset of available traces.</p>
    #[serde(rename = "sampling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<bool>,
    /// <p>A parameter to indicate whether to enable sampling on trace summaries. Input parameters are Name and Value.</p>
    #[serde(rename = "samplingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_strategy: Option<SamplingStrategy>,
    /// <p>The start of the time frame for which to retrieve traces.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>A parameter to indicate whether to query trace summaries by TraceId or Event time.</p>
    #[serde(rename = "timeRangeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTraceSummariesResult {
    /// <p>The start time of this page of results.</p>
    #[serde(rename = "approximateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_time: Option<f64>,
    /// <p>If the requested time frame contained more than one page of results, you can use this token to retrieve the next page. The first page contains the most recent results, closest to the end of the time frame.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Trace IDs and annotations for traces that were found in the specified time frame.</p>
    #[serde(rename = "traceSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_summaries: Option<Vec<TraceSummary>>,
    /// <p>The total number of traces processed, including traces that did not match the specified filter expression.</p>
    #[serde(rename = "tracesProcessedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traces_processed_count: Option<i64>,
}

/// <p>Details and metadata for a group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Group {
    /// <p>The filter expression defining the parameters to include traces.</p>
    #[serde(rename = "filterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the group generated based on the GroupName.</p>
    #[serde(rename = "groupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The unique case-sensitive name of the group.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p><p>The structure containing configurations related to insights.</p> <ul> <li> <p>The InsightsEnabled boolean can be set to true to enable insights for the group or false to disable insights for the group.</p> </li> <li> <p>The NotifcationsEnabled boolean can be set to true to enable insights notifications through Amazon EventBridge for the group.</p> </li> </ul></p>
    #[serde(rename = "insightsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_configuration: Option<InsightsConfiguration>,
}

/// <p>Details for a group without metadata.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GroupSummary {
    /// <p>The filter expression defining the parameters to include traces.</p>
    #[serde(rename = "filterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>The ARN of the group generated based on the GroupName.</p>
    #[serde(rename = "groupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The unique case-sensitive name of the group.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p><p>The structure containing configurations related to insights.</p> <ul> <li> <p>The InsightsEnabled boolean can be set to true to enable insights for the group or false to disable insights for the group.</p> </li> <li> <p>The NotificationsEnabled boolean can be set to true to enable insights notifications. Notifications can only be enabled on a group with InsightsEnabled set to true.</p> </li> </ul></p>
    #[serde(rename = "insightsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_configuration: Option<InsightsConfiguration>,
}

/// <p>An entry in a histogram for a statistic. A histogram maps the range of observed values on the X axis, and the prevalence of each value on the Y axis.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HistogramEntry {
    /// <p>The prevalence of the entry.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The value of the entry.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Information about an HTTP request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Http {
    /// <p>The IP address of the requestor.</p>
    #[serde(rename = "clientIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    /// <p>The request method.</p>
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p>The response status.</p>
    #[serde(rename = "httpStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status: Option<i64>,
    /// <p>The request URL.</p>
    #[serde(rename = "httpURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_url: Option<String>,
    /// <p>The request's user agent string.</p>
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

/// <p>When fault rates go outside of the expected range, X-Ray creates an insight. Insights tracks emergent issues within your applications.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Insight {
    /// <p>The categories that label and describe the type of insight.</p>
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// <p>The impact statistics of the client side service. This includes the number of requests to the client service and whether the requests were faults or okay.</p>
    #[serde(rename = "clientRequestImpactStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_impact_statistics: Option<RequestImpactStatistics>,
    /// <p>The time, in Unix seconds, at which the insight ended.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the group that the insight belongs to.</p>
    #[serde(rename = "groupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The name of the group that the insight belongs to.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The insights unique identifier. </p>
    #[serde(rename = "insightId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_id: Option<String>,
    #[serde(rename = "rootCauseServiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause_service_id: Option<ServiceId>,
    /// <p>The impact statistics of the root cause service. This includes the number of requests to the client service and whether the requests were faults or okay.</p>
    #[serde(rename = "rootCauseServiceRequestImpactStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause_service_request_impact_statistics: Option<RequestImpactStatistics>,
    /// <p>The time, in Unix seconds, at which the insight began.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The current state of the insight.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A brief description of the insight.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// <p>The service within the insight that is most impacted by the incident.</p>
    #[serde(rename = "topAnomalousServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_anomalous_services: Option<Vec<AnomalousService>>,
}

/// <p>X-Ray reevaluates insights periodically until they are resolved, and records each intermediate state in an event. You can review incident events in the Impact Timeline on the Inspect page in the X-Ray console.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InsightEvent {
    /// <p>The impact statistics of the client side service. This includes the number of requests to the client service and whether the requests were faults or okay.</p>
    #[serde(rename = "clientRequestImpactStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_impact_statistics: Option<RequestImpactStatistics>,
    /// <p>The time, in Unix seconds, at which the event was recorded.</p>
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f64>,
    /// <p>The impact statistics of the root cause service. This includes the number of requests to the client service and whether the requests were faults or okay.</p>
    #[serde(rename = "rootCauseServiceRequestImpactStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause_service_request_impact_statistics: Option<RequestImpactStatistics>,
    /// <p>A brief description of the event.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// <p>The service during the event that is most impacted by the incident.</p>
    #[serde(rename = "topAnomalousServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_anomalous_services: Option<Vec<AnomalousService>>,
}

/// <p>The connection between two service in an insight impact graph.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InsightImpactGraphEdge {
    /// <p>Identifier of the edge. Unique within a service map.</p>
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i64>,
}

/// <p>Information about an application that processed requests, users that made requests, or downstream services, resources, and applications that an application used. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InsightImpactGraphService {
    /// <p>Identifier of the AWS account in which the service runs.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Connections to downstream services.</p>
    #[serde(rename = "edges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<InsightImpactGraphEdge>>,
    /// <p>The canonical name of the service.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of names for the service, including the canonical name.</p>
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>Identifier for the service. Unique within the service map.</p>
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i64>,
    /// <p><p>Identifier for the service. Unique within the service map.</p> <ul> <li> <p>AWS Resource - The type of an AWS resource. For example, AWS::EC2::Instance for an application running on Amazon EC2 or AWS::DynamoDB::Table for an Amazon DynamoDB table that the application used. </p> </li> <li> <p>AWS Service - The type of an AWS service. For example, AWS::DynamoDB for downstream calls to Amazon DynamoDB that didn&#39;t target a specific table. </p> </li> <li> <p>AWS Service - The type of an AWS service. For example, AWS::DynamoDB for downstream calls to Amazon DynamoDB that didn&#39;t target a specific table. </p> </li> <li> <p>remote - A downstream service of indeterminate type.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information that describes an insight.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InsightSummary {
    /// <p> Categories The categories that label and describe the type of insight.</p>
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// <p>The impact statistics of the client side service. This includes the number of requests to the client service and whether the requests were faults or okay. </p>
    #[serde(rename = "clientRequestImpactStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_impact_statistics: Option<RequestImpactStatistics>,
    /// <p>The time, in Unix seconds, at which the insight ended.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the group that the insight belongs to.</p>
    #[serde(rename = "groupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The name of the group that the insight belongs to.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The insights unique identifier. </p>
    #[serde(rename = "insightId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_id: Option<String>,
    /// <p>The time, in Unix seconds, that the insight was last updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "rootCauseServiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause_service_id: Option<ServiceId>,
    /// <p>The impact statistics of the root cause service. This includes the number of requests to the client service and whether the requests were faults or okay. </p>
    #[serde(rename = "rootCauseServiceRequestImpactStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause_service_request_impact_statistics: Option<RequestImpactStatistics>,
    /// <p>The time, in Unix seconds, at which the insight began.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The current state of the insight.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A brief description of the insight.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// <p>The service within the insight that is most impacted by the incident.</p>
    #[serde(rename = "topAnomalousServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_anomalous_services: Option<Vec<AnomalousService>>,
}

/// <p>The structure containing configurations related to insights.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InsightsConfiguration {
    /// <p>Set the InsightsEnabled value to true to enable insights or false to disable insights.</p>
    #[serde(rename = "insightsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_enabled: Option<bool>,
    /// <p>Set the NotificationsEnabled value to true to enable insights notifications. Notifications can only be enabled on a group with InsightsEnabled set to true.</p>
    #[serde(rename = "notificationsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
}

/// <p>A list of EC2 instance IDs corresponding to the segments in a trace. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceIdDetail {
    /// <p>The ID of a corresponding EC2 instance.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>A pagination token. If multiple pages of results are returned, use the <code>NextToken</code> value returned with the current page of results as the value of this parameter to get the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Number (ARN) of an X-Ray group or sampling rule.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A pagination token. If multiple pages of results are returned, use the <code>NextToken</code> value returned with the current page of results to get the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tags, as key and value pairs, that is associated with the specified X-Ray group or sampling rule.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEncryptionConfigRequest {
    /// <p>An AWS KMS customer master key (CMK) in one of the following formats:</p> <ul> <li> <p> <b>Alias</b> - The name of the key. For example, <code>alias/MyKey</code>.</p> </li> <li> <p> <b>Key ID</b> - The KMS key ID of the key. For example, <code>ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>. AWS X-Ray does not support asymmetric CMKs.</p> </li> <li> <p> <b>ARN</b> - The full Amazon Resource Name of the key ID or alias. For example, <code>arn:aws:kms:us-east-2:123456789012:key/ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>. Use this format to specify a key in a different account.</p> </li> </ul> <p>Omit this key if you set <code>Type</code> to <code>NONE</code>.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The type of encryption. Set to <code>KMS</code> to use your own key for encryption. Set to <code>NONE</code> for default encryption.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEncryptionConfigResult {
    /// <p>The new encryption configuration.</p>
    #[serde(rename = "encryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutTelemetryRecordsRequest {
    /// <p><p/></p>
    #[serde(rename = "eC2InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_id: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "resourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "telemetryRecords")]
    pub telemetry_records: Vec<TelemetryRecord>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutTelemetryRecordsResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutTraceSegmentsRequest {
    /// <p>A string containing a JSON document defining one or more segments or subsegments.</p>
    #[serde(rename = "traceSegmentDocuments")]
    pub trace_segment_documents: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutTraceSegmentsResult {
    /// <p>Segments that failed processing.</p>
    #[serde(rename = "unprocessedTraceSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_trace_segments: Option<Vec<UnprocessedTraceSegment>>,
}

/// <p>Statistics that describe how the incident has impacted a service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RequestImpactStatistics {
    /// <p>The number of requests that have resulted in a fault,</p>
    #[serde(rename = "faultCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_count: Option<i64>,
    /// <p>The number of successful requests.</p>
    #[serde(rename = "okCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_count: Option<i64>,
    /// <p>The total number of requests to the service.</p>
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// <p>A list of resources ARNs corresponding to the segments in a trace.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceARNDetail {
    /// <p>The ARN of a corresponding resource.</p>
    #[serde(rename = "aRN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// <p>The root cause information for a response time warning.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResponseTimeRootCause {
    /// <p>A flag that denotes that the root cause impacts the trace client.</p>
    #[serde(rename = "clientImpacting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_impacting: Option<bool>,
    /// <p>A list of corresponding services. A service identifies a segment and contains a name, account ID, type, and inferred flag.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ResponseTimeRootCauseService>>,
}

/// <p>A collection of segments and corresponding subsegments associated to a response time warning.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResponseTimeRootCauseEntity {
    /// <p>The type and messages of the exceptions.</p>
    #[serde(rename = "coverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<f64>,
    /// <p>The name of the entity.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A flag that denotes a remote subsegment.</p>
    #[serde(rename = "remote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
}

/// <p>A collection of fields identifying the service in a response time warning.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResponseTimeRootCauseService {
    /// <p>The account ID associated to the service.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The path of root cause entities found on the service. </p>
    #[serde(rename = "entityPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<Vec<ResponseTimeRootCauseEntity>>,
    /// <p>A Boolean value indicating if the service is inferred from the trace.</p>
    #[serde(rename = "inferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,
    /// <p>The service name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A collection of associated service names.</p>
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The type associated to the service.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The exception associated with a root cause.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RootCauseException {
    /// <p>The message of the exception.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The name of the exception.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A sampling rule that services use to decide whether to instrument a request. Rule fields can match properties of the service, or properties of a request. The service can ignore rules that don't match its properties.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SamplingRule {
    /// <p>Matches attributes derived from the request.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The percentage of matching requests to instrument, after the reservoir is exhausted.</p>
    #[serde(rename = "fixedRate")]
    pub fixed_rate: f64,
    /// <p>Matches the HTTP method of a request.</p>
    #[serde(rename = "hTTPMethod")]
    pub http_method: String,
    /// <p>Matches the hostname from a request URL.</p>
    #[serde(rename = "host")]
    pub host: String,
    /// <p>The priority of the sampling rule.</p>
    #[serde(rename = "priority")]
    pub priority: i64,
    /// <p>A fixed number of matching requests to instrument per second, prior to applying the fixed rate. The reservoir is not used directly by services, but applies to all services using the rule collectively.</p>
    #[serde(rename = "reservoirSize")]
    pub reservoir_size: i64,
    /// <p>Matches the ARN of the AWS resource on which the service runs.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The ARN of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "ruleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    /// <p>The name of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "ruleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// <p>Matches the <code>name</code> that the service uses to identify itself in segments.</p>
    #[serde(rename = "serviceName")]
    pub service_name: String,
    /// <p>Matches the <code>origin</code> that the service uses to identify its type in segments.</p>
    #[serde(rename = "serviceType")]
    pub service_type: String,
    /// <p>Matches the path from a request URL.</p>
    #[serde(rename = "uRLPath")]
    pub url_path: String,
    /// <p>The version of the sampling rule format (<code>1</code>).</p>
    #[serde(rename = "version")]
    pub version: i64,
}

/// <p>A <a>SamplingRule</a> and its metadata.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SamplingRuleRecord {
    /// <p>When the rule was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>When the rule was last modified.</p>
    #[serde(rename = "modifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    /// <p>The sampling rule.</p>
    #[serde(rename = "samplingRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule: Option<SamplingRule>,
}

/// <p>A document specifying changes to a sampling rule's configuration.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SamplingRuleUpdate {
    /// <p>Matches attributes derived from the request.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The percentage of matching requests to instrument, after the reservoir is exhausted.</p>
    #[serde(rename = "fixedRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_rate: Option<f64>,
    /// <p>Matches the HTTP method of a request.</p>
    #[serde(rename = "hTTPMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p>Matches the hostname from a request URL.</p>
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// <p>The priority of the sampling rule.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>A fixed number of matching requests to instrument per second, prior to applying the fixed rate. The reservoir is not used directly by services, but applies to all services using the rule collectively.</p>
    #[serde(rename = "reservoirSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservoir_size: Option<i64>,
    /// <p>Matches the ARN of the AWS resource on which the service runs.</p>
    #[serde(rename = "resourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The ARN of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "ruleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    /// <p>The name of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "ruleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// <p>Matches the <code>name</code> that the service uses to identify itself in segments.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>Matches the <code>origin</code> that the service uses to identify its type in segments.</p>
    #[serde(rename = "serviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    /// <p>Matches the path from a request URL.</p>
    #[serde(rename = "uRLPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_path: Option<String>,
}

/// <p>Aggregated request sampling data for a sampling rule across all services for a 10-second window.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SamplingStatisticSummary {
    /// <p>The number of requests recorded with borrowed reservoir quota.</p>
    #[serde(rename = "borrowCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_count: Option<i64>,
    /// <p>The number of requests that matched the rule.</p>
    #[serde(rename = "requestCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_count: Option<i64>,
    /// <p>The name of the sampling rule.</p>
    #[serde(rename = "ruleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// <p>The number of requests recorded.</p>
    #[serde(rename = "sampledCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampled_count: Option<i64>,
    /// <p>The start time of the reporting window.</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

/// <p>Request sampling results for a single rule from a service. Results are for the last 10 seconds unless the service has been assigned a longer reporting interval after a previous call to <a>GetSamplingTargets</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SamplingStatisticsDocument {
    /// <p>The number of requests recorded with borrowed reservoir quota.</p>
    #[serde(rename = "borrowCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_count: Option<i64>,
    /// <p>A unique identifier for the service in hexadecimal.</p>
    #[serde(rename = "clientID")]
    pub client_id: String,
    /// <p>The number of requests that matched the rule.</p>
    #[serde(rename = "requestCount")]
    pub request_count: i64,
    /// <p>The name of the sampling rule.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    /// <p>The number of requests recorded.</p>
    #[serde(rename = "sampledCount")]
    pub sampled_count: i64,
    /// <p>The current time.</p>
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}

/// <p>The name and value of a sampling rule to apply to a trace summary.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SamplingStrategy {
    /// <p>The name of a sampling rule.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of a sampling rule.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Temporary changes to a sampling rule configuration. To meet the global sampling target for a rule, X-Ray calculates a new reservoir for each service based on the recent sampling results of all services that called <a>GetSamplingTargets</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SamplingTargetDocument {
    /// <p>The percentage of matching requests to instrument, after the reservoir is exhausted.</p>
    #[serde(rename = "fixedRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_rate: Option<f64>,
    /// <p>The number of seconds for the service to wait before getting sampling targets again.</p>
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// <p>The number of requests per second that X-Ray allocated for this service.</p>
    #[serde(rename = "reservoirQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservoir_quota: Option<i64>,
    /// <p>When the reservoir quota expires.</p>
    #[serde(rename = "reservoirQuotaTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservoir_quota_ttl: Option<f64>,
    /// <p>The name of the sampling rule.</p>
    #[serde(rename = "ruleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

/// <p>A segment from a trace that has been ingested by the X-Ray service. The segment can be compiled from documents uploaded with <a>PutTraceSegments</a>, or an <code>inferred</code> segment for a downstream service, generated from a subsegment sent by the service that called it.</p> <p>For the full segment document schema, see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html">AWS X-Ray Segment Documents</a> in the <i>AWS X-Ray Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Segment {
    /// <p>The segment document.</p>
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    /// <p>The segment's ID.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Information about an application that processed requests, users that made requests, or downstream services, resources, and applications that an application used.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Service {
    /// <p>Identifier of the AWS account in which the service runs.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>A histogram that maps the spread of service durations.</p>
    #[serde(rename = "durationHistogram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_histogram: Option<Vec<HistogramEntry>>,
    /// <p>Connections to downstream services.</p>
    #[serde(rename = "edges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<Edge>>,
    /// <p>The end time of the last segment that the service generated.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The canonical name of the service.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of names for the service, including the canonical name.</p>
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>Identifier for the service. Unique within the service map.</p>
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i64>,
    /// <p>A histogram that maps the spread of service response times.</p>
    #[serde(rename = "responseTimeHistogram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_histogram: Option<Vec<HistogramEntry>>,
    /// <p>Indicates that the service was the first service to process a request.</p>
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<bool>,
    /// <p>The start time of the first segment that the service generated.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The service's state.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Aggregated statistics for the service.</p>
    #[serde(rename = "summaryStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_statistics: Option<ServiceStatistics>,
    /// <p><p>The type of service.</p> <ul> <li> <p>AWS Resource - The type of an AWS resource. For example, <code>AWS::EC2::Instance</code> for an application running on Amazon EC2 or <code>AWS::DynamoDB::Table</code> for an Amazon DynamoDB table that the application used.</p> </li> <li> <p>AWS Service - The type of an AWS service. For example, <code>AWS::DynamoDB</code> for downstream calls to Amazon DynamoDB that didn&#39;t target a specific table.</p> </li> <li> <p> <code>client</code> - Represents the clients that sent requests to a root service.</p> </li> <li> <p> <code>remote</code> - A downstream service of indeterminate type.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p><p/></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceId {
    /// <p><p/></p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p><p/></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Response statistics for a service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceStatistics {
    /// <p>Information about requests that failed with a 4xx Client Error status code.</p>
    #[serde(rename = "errorStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_statistics: Option<ErrorStatistics>,
    /// <p>Information about requests that failed with a 5xx Server Error status code.</p>
    #[serde(rename = "faultStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_statistics: Option<FaultStatistics>,
    /// <p>The number of requests that completed with a 2xx Success status code.</p>
    #[serde(rename = "okCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_count: Option<i64>,
    /// <p>The total number of completed requests.</p>
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// <p>The aggregate response time of completed requests.</p>
    #[serde(rename = "totalResponseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_response_time: Option<f64>,
}

/// <p><p>A map that contains tag keys and tag values to attach to an AWS X-Ray group or sampling rule. For more information about ways to use tags, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS resources</a> in the <i>AWS General Reference</i>.</p> <p>The following restrictions apply to tags:</p> <ul> <li> <p>Maximum number of user-applied tags per resource: 50</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Don&#39;t use <code>aws:</code> as a prefix for keys; it&#39;s reserved for AWS use. You cannot edit or delete system tags.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>A tag key, such as <code>Stage</code> or <code>Name</code>. A tag key cannot be empty. The key can be a maximum of 128 characters, and can contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>An optional tag value, such as <code>Production</code> or <code>test-only</code>. The value can be a maximum of 255 characters, and contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Number (ARN) of an X-Ray group or sampling rule.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p><p>A map that contains one or more tag keys and tag values to attach to an X-Ray group or sampling rule. For more information about ways to use tags, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS resources</a> in the <i>AWS General Reference</i>.</p> <p>The following restrictions apply to tags:</p> <ul> <li> <p>Maximum number of user-applied tags per resource: 50</p> </li> <li> <p>Maximum tag key length: 128 Unicode characters</p> </li> <li> <p>Maximum tag value length: 256 Unicode characters</p> </li> <li> <p>Valid values for key and value: a-z, A-Z, 0-9, space, and the following characters: _ . : / = + - and @</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Don&#39;t use <code>aws:</code> as a prefix for keys; it&#39;s reserved for AWS use. You cannot edit or delete system tags.</p> </li> </ul></p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p><p/></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TelemetryRecord {
    /// <p><p/></p>
    #[serde(rename = "backendConnectionErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_connection_errors: Option<BackendConnectionErrors>,
    /// <p><p/></p>
    #[serde(rename = "segmentsReceivedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_received_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "segmentsRejectedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_rejected_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "segmentsSentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_sent_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "segmentsSpilloverCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_spillover_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}

/// <p>A list of TimeSeriesStatistic structures.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TimeSeriesServiceStatistics {
    #[serde(rename = "edgeSummaryStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_summary_statistics: Option<EdgeStatistics>,
    /// <p>The response time histogram for the selected entities.</p>
    #[serde(rename = "responseTimeHistogram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_histogram: Option<Vec<HistogramEntry>>,
    /// <p>The forecasted high and low fault count values.</p>
    #[serde(rename = "serviceForecastStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_forecast_statistics: Option<ForecastStatistics>,
    #[serde(rename = "serviceSummaryStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_summary_statistics: Option<ServiceStatistics>,
    /// <p>Timestamp of the window for which statistics are aggregated.</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

/// <p>A collection of segment documents with matching trace IDs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Trace {
    /// <p>The length of time in seconds between the start time of the root segment and the end time of the last segment that completed.</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// <p>The unique identifier for the request that generated the trace's segments and subsegments.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>LimitExceeded is set to true when the trace has exceeded one of the defined quotas. For more information about quotas, see <a href="https://docs.aws.amazon.com/general/latest/gr/xray.html">AWS X-Ray endpoints and quotas</a>.</p>
    #[serde(rename = "limitExceeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_exceeded: Option<bool>,
    /// <p>Segment documents for the segments and subsegments that comprise the trace.</p>
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<Segment>>,
}

/// <p>Metadata generated from the segment documents in a trace.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TraceSummary {
    /// <p>Annotations from the trace's segment documents.</p>
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, Vec<ValueWithServiceIds>>>,
    /// <p>A list of Availability Zones for any zone corresponding to the trace segments.</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<AvailabilityZoneDetail>>,
    /// <p>The length of time in seconds between the start time of the root segment and the end time of the last segment that completed.</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// <p>The root of a trace.</p>
    #[serde(rename = "entryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<ServiceId>,
    /// <p>A collection of ErrorRootCause structures corresponding to the trace segments.</p>
    #[serde(rename = "errorRootCauses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_root_causes: Option<Vec<ErrorRootCause>>,
    /// <p>A collection of FaultRootCause structures corresponding to the trace segments.</p>
    #[serde(rename = "faultRootCauses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_root_causes: Option<Vec<FaultRootCause>>,
    /// <p>The root segment document has a 400 series error.</p>
    #[serde(rename = "hasError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    /// <p>The root segment document has a 500 series error.</p>
    #[serde(rename = "hasFault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_fault: Option<bool>,
    /// <p>One or more of the segment documents has a 429 throttling error.</p>
    #[serde(rename = "hasThrottle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_throttle: Option<bool>,
    /// <p>Information about the HTTP request served by the trace.</p>
    #[serde(rename = "http")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<Http>,
    /// <p>The unique identifier for the request that generated the trace's segments and subsegments.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A list of EC2 instance IDs for any instance corresponding to the trace segments.</p>
    #[serde(rename = "instanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<InstanceIdDetail>>,
    /// <p>One or more of the segment documents is in progress.</p>
    #[serde(rename = "isPartial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_partial: Option<bool>,
    /// <p>The matched time stamp of a defined event.</p>
    #[serde(rename = "matchedEventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_event_time: Option<f64>,
    /// <p>A list of resource ARNs for any resource corresponding to the trace segments.</p>
    #[serde(rename = "resourceARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ar_ns: Option<Vec<ResourceARNDetail>>,
    /// <p>The length of time in seconds between the start and end times of the root segment. If the service performs work asynchronously, the response time measures the time before the response is sent to the user, while the duration measures the amount of time before the last traced activity completes.</p>
    #[serde(rename = "responseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time: Option<f64>,
    /// <p>A collection of ResponseTimeRootCause structures corresponding to the trace segments.</p>
    #[serde(rename = "responseTimeRootCauses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_root_causes: Option<Vec<ResponseTimeRootCause>>,
    /// <p>The revision number of a trace.</p>
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    /// <p>Service IDs from the trace's segment documents.</p>
    #[serde(rename = "serviceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<ServiceId>>,
    /// <p>Users from the trace's segment documents.</p>
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<TraceUser>>,
}

/// <p>Information about a user recorded in segment documents.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TraceUser {
    /// <p>Services that the user's request hit.</p>
    #[serde(rename = "serviceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<ServiceId>>,
    /// <p>The user's name.</p>
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>Sampling statistics from a call to <a>GetSamplingTargets</a> that X-Ray could not process.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnprocessedStatistics {
    /// <p>The error code.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The name of the sampling rule.</p>
    #[serde(rename = "ruleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

/// <p>Information about a segment that failed processing.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnprocessedTraceSegment {
    /// <p>The error that caused processing to fail.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The segment's ID.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Number (ARN) of an X-Ray group or sampling rule.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>Keys for one or more tags that you want to remove from an X-Ray group or sampling rule.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGroupRequest {
    /// <p>The updated filter expression defining criteria by which to group traces.</p>
    #[serde(rename = "filterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>The ARN that was generated upon creation.</p>
    #[serde(rename = "groupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The case-sensitive name of the group.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p><p>The structure containing configurations related to insights.</p> <ul> <li> <p>The InsightsEnabled boolean can be set to true to enable insights for the group or false to disable insights for the group.</p> </li> <li> <p>The NotifcationsEnabled boolean can be set to true to enable insights notifications for the group. Notifications can only be enabled on a group with InsightsEnabled set to true.</p> </li> </ul></p>
    #[serde(rename = "insightsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_configuration: Option<InsightsConfiguration>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGroupResult {
    /// <p>The group that was updated. Contains the name of the group that was updated, the ARN of the group that was updated, the updated filter expression, and the updated insight configuration assigned to the group.</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSamplingRuleRequest {
    /// <p>The rule and fields to change.</p>
    #[serde(rename = "samplingRuleUpdate")]
    pub sampling_rule_update: SamplingRuleUpdate,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSamplingRuleResult {
    /// <p>The updated rule definition and metadata.</p>
    #[serde(rename = "samplingRuleRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_record: Option<SamplingRuleRecord>,
}

/// <p>Information about a segment annotation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ValueWithServiceIds {
    /// <p>Values of the annotation.</p>
    #[serde(rename = "annotationValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_value: Option<AnnotationValue>,
    /// <p>Services to which the annotation applies.</p>
    #[serde(rename = "serviceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<ServiceId>>,
}

/// Errors returned by BatchGetTraces
#[derive(Debug, PartialEq)]
pub enum BatchGetTracesError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl BatchGetTracesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetTracesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchGetTracesError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(BatchGetTracesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetTracesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetTracesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchGetTracesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetTracesError {}
/// Errors returned by CreateGroup
#[derive(Debug, PartialEq)]
pub enum CreateGroupError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl CreateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateGroupError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(CreateGroupError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateGroupError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGroupError {}
/// Errors returned by CreateSamplingRule
#[derive(Debug, PartialEq)]
pub enum CreateSamplingRuleError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>You have reached the maximum number of sampling rules.</p>
    RuleLimitExceeded(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl CreateSamplingRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSamplingRuleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateSamplingRuleError::InvalidRequest(err.msg))
                }
                "RuleLimitExceededException" => {
                    return RusotoError::Service(CreateSamplingRuleError::RuleLimitExceeded(
                        err.msg,
                    ))
                }
                "ThrottledException" => {
                    return RusotoError::Service(CreateSamplingRuleError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSamplingRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSamplingRuleError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateSamplingRuleError::RuleLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateSamplingRuleError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSamplingRuleError {}
/// Errors returned by DeleteGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGroupError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl DeleteGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteGroupError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(DeleteGroupError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGroupError {}
/// Errors returned by DeleteSamplingRule
#[derive(Debug, PartialEq)]
pub enum DeleteSamplingRuleError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl DeleteSamplingRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSamplingRuleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteSamplingRuleError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(DeleteSamplingRuleError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSamplingRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSamplingRuleError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteSamplingRuleError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSamplingRuleError {}
/// Errors returned by GetEncryptionConfig
#[derive(Debug, PartialEq)]
pub enum GetEncryptionConfigError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetEncryptionConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEncryptionConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetEncryptionConfigError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetEncryptionConfigError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEncryptionConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEncryptionConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetEncryptionConfigError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEncryptionConfigError {}
/// Errors returned by GetGroup
#[derive(Debug, PartialEq)]
pub enum GetGroupError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetGroupError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetGroupError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetGroupError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGroupError {}
/// Errors returned by GetGroups
#[derive(Debug, PartialEq)]
pub enum GetGroupsError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetGroupsError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetGroupsError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGroupsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetGroupsError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGroupsError {}
/// Errors returned by GetInsight
#[derive(Debug, PartialEq)]
pub enum GetInsightError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetInsightError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInsightError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetInsightError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetInsightError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInsightError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInsightError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetInsightError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInsightError {}
/// Errors returned by GetInsightEvents
#[derive(Debug, PartialEq)]
pub enum GetInsightEventsError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetInsightEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInsightEventsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetInsightEventsError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetInsightEventsError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInsightEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInsightEventsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetInsightEventsError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInsightEventsError {}
/// Errors returned by GetInsightImpactGraph
#[derive(Debug, PartialEq)]
pub enum GetInsightImpactGraphError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetInsightImpactGraphError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInsightImpactGraphError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetInsightImpactGraphError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetInsightImpactGraphError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInsightImpactGraphError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInsightImpactGraphError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetInsightImpactGraphError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInsightImpactGraphError {}
/// Errors returned by GetInsightSummaries
#[derive(Debug, PartialEq)]
pub enum GetInsightSummariesError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetInsightSummariesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInsightSummariesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetInsightSummariesError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetInsightSummariesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInsightSummariesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInsightSummariesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetInsightSummariesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInsightSummariesError {}
/// Errors returned by GetSamplingRules
#[derive(Debug, PartialEq)]
pub enum GetSamplingRulesError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetSamplingRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSamplingRulesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetSamplingRulesError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetSamplingRulesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSamplingRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSamplingRulesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetSamplingRulesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSamplingRulesError {}
/// Errors returned by GetSamplingStatisticSummaries
#[derive(Debug, PartialEq)]
pub enum GetSamplingStatisticSummariesError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetSamplingStatisticSummariesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetSamplingStatisticSummariesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        GetSamplingStatisticSummariesError::InvalidRequest(err.msg),
                    )
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetSamplingStatisticSummariesError::Throttled(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSamplingStatisticSummariesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSamplingStatisticSummariesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetSamplingStatisticSummariesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSamplingStatisticSummariesError {}
/// Errors returned by GetSamplingTargets
#[derive(Debug, PartialEq)]
pub enum GetSamplingTargetsError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetSamplingTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSamplingTargetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetSamplingTargetsError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetSamplingTargetsError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSamplingTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSamplingTargetsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetSamplingTargetsError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSamplingTargetsError {}
/// Errors returned by GetServiceGraph
#[derive(Debug, PartialEq)]
pub enum GetServiceGraphError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetServiceGraphError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetServiceGraphError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetServiceGraphError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetServiceGraphError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetServiceGraphError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetServiceGraphError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetServiceGraphError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetServiceGraphError {}
/// Errors returned by GetTimeSeriesServiceStatistics
#[derive(Debug, PartialEq)]
pub enum GetTimeSeriesServiceStatisticsError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetTimeSeriesServiceStatisticsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetTimeSeriesServiceStatisticsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        GetTimeSeriesServiceStatisticsError::InvalidRequest(err.msg),
                    )
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetTimeSeriesServiceStatisticsError::Throttled(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTimeSeriesServiceStatisticsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTimeSeriesServiceStatisticsError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetTimeSeriesServiceStatisticsError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTimeSeriesServiceStatisticsError {}
/// Errors returned by GetTraceGraph
#[derive(Debug, PartialEq)]
pub enum GetTraceGraphError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetTraceGraphError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTraceGraphError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetTraceGraphError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetTraceGraphError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTraceGraphError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTraceGraphError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetTraceGraphError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTraceGraphError {}
/// Errors returned by GetTraceSummaries
#[derive(Debug, PartialEq)]
pub enum GetTraceSummariesError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl GetTraceSummariesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTraceSummariesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetTraceSummariesError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetTraceSummariesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTraceSummariesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTraceSummariesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetTraceSummariesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTraceSummariesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The resource was not found. Verify that the name or Amazon Resource Name (ARN) of the resource is correct.</p>
    ResourceNotFound(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottledException" => {
                    return RusotoError::Service(ListTagsForResourceError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutEncryptionConfig
#[derive(Debug, PartialEq)]
pub enum PutEncryptionConfigError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl PutEncryptionConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEncryptionConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(PutEncryptionConfigError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(PutEncryptionConfigError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEncryptionConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEncryptionConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            PutEncryptionConfigError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEncryptionConfigError {}
/// Errors returned by PutTelemetryRecords
#[derive(Debug, PartialEq)]
pub enum PutTelemetryRecordsError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl PutTelemetryRecordsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutTelemetryRecordsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(PutTelemetryRecordsError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(PutTelemetryRecordsError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutTelemetryRecordsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutTelemetryRecordsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            PutTelemetryRecordsError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutTelemetryRecordsError {}
/// Errors returned by PutTraceSegments
#[derive(Debug, PartialEq)]
pub enum PutTraceSegmentsError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl PutTraceSegmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutTraceSegmentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(PutTraceSegmentsError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(PutTraceSegmentsError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutTraceSegmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutTraceSegmentsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            PutTraceSegmentsError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutTraceSegmentsError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The resource was not found. Verify that the name or Amazon Resource Name (ARN) of the resource is correct.</p>
    ResourceNotFound(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
    /// <p>You have exceeded the maximum number of tags you can apply to this resource.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(TagResourceError::Throttled(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(TagResourceError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttled(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The resource was not found. Verify that the name or Amazon Resource Name (ARN) of the resource is correct.</p>
    ResourceNotFound(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(UntagResourceError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateGroup
#[derive(Debug, PartialEq)]
pub enum UpdateGroupError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl UpdateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateGroupError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(UpdateGroupError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGroupError {}
/// Errors returned by UpdateSamplingRule
#[derive(Debug, PartialEq)]
pub enum UpdateSamplingRuleError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
}

impl UpdateSamplingRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSamplingRuleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateSamplingRuleError::InvalidRequest(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(UpdateSamplingRuleError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSamplingRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSamplingRuleError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateSamplingRuleError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSamplingRuleError {}
/// Trait representing the capabilities of the AWS X-Ray API. AWS X-Ray clients implement this trait.
#[async_trait]
pub trait XRay {
    /// <p>Retrieves a list of traces specified by ID. Each trace is a collection of segment documents that originates from a single request. Use <code>GetTraceSummaries</code> to get a list of trace IDs.</p>
    async fn batch_get_traces(
        &self,
        input: BatchGetTracesRequest,
    ) -> Result<BatchGetTracesResult, RusotoError<BatchGetTracesError>>;

    /// <p>Creates a group resource with a name and a filter expression. </p>
    async fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> Result<CreateGroupResult, RusotoError<CreateGroupError>>;

    /// <p>Creates a rule to control sampling behavior for instrumented applications. Services retrieve rules with <a>GetSamplingRules</a>, and evaluate each rule in ascending order of <i>priority</i> for each request. If a rule matches, the service records a trace, borrowing it from the reservoir size. After 10 seconds, the service reports back to X-Ray with <a>GetSamplingTargets</a> to get updated versions of each in-use rule. The updated rule contains a trace quota that the service can use instead of borrowing from the reservoir.</p>
    async fn create_sampling_rule(
        &self,
        input: CreateSamplingRuleRequest,
    ) -> Result<CreateSamplingRuleResult, RusotoError<CreateSamplingRuleError>>;

    /// <p>Deletes a group resource.</p>
    async fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> Result<DeleteGroupResult, RusotoError<DeleteGroupError>>;

    /// <p>Deletes a sampling rule.</p>
    async fn delete_sampling_rule(
        &self,
        input: DeleteSamplingRuleRequest,
    ) -> Result<DeleteSamplingRuleResult, RusotoError<DeleteSamplingRuleError>>;

    /// <p>Retrieves the current encryption configuration for X-Ray data.</p>
    async fn get_encryption_config(
        &self,
    ) -> Result<GetEncryptionConfigResult, RusotoError<GetEncryptionConfigError>>;

    /// <p>Retrieves group resource details.</p>
    async fn get_group(
        &self,
        input: GetGroupRequest,
    ) -> Result<GetGroupResult, RusotoError<GetGroupError>>;

    /// <p>Retrieves all active group details.</p>
    async fn get_groups(
        &self,
        input: GetGroupsRequest,
    ) -> Result<GetGroupsResult, RusotoError<GetGroupsError>>;

    /// <p>Retrieves the summary information of an insight. This includes impact to clients and root cause services, the top anomalous services, the category, the state of the insight, and the start and end time of the insight.</p>
    async fn get_insight(
        &self,
        input: GetInsightRequest,
    ) -> Result<GetInsightResult, RusotoError<GetInsightError>>;

    /// <p>X-Ray reevaluates insights periodically until they're resolved, and records each intermediate state as an event. You can review an insight's events in the Impact Timeline on the Inspect page in the X-Ray console.</p>
    async fn get_insight_events(
        &self,
        input: GetInsightEventsRequest,
    ) -> Result<GetInsightEventsResult, RusotoError<GetInsightEventsError>>;

    /// <p>Retrieves a service graph structure filtered by the specified insight. The service graph is limited to only structural information. For a complete service graph, use this API with the GetServiceGraph API.</p>
    async fn get_insight_impact_graph(
        &self,
        input: GetInsightImpactGraphRequest,
    ) -> Result<GetInsightImpactGraphResult, RusotoError<GetInsightImpactGraphError>>;

    /// <p>Retrieves the summaries of all insights in the specified group matching the provided filter values.</p>
    async fn get_insight_summaries(
        &self,
        input: GetInsightSummariesRequest,
    ) -> Result<GetInsightSummariesResult, RusotoError<GetInsightSummariesError>>;

    /// <p>Retrieves all sampling rules.</p>
    async fn get_sampling_rules(
        &self,
        input: GetSamplingRulesRequest,
    ) -> Result<GetSamplingRulesResult, RusotoError<GetSamplingRulesError>>;

    /// <p>Retrieves information about recent sampling results for all sampling rules.</p>
    async fn get_sampling_statistic_summaries(
        &self,
        input: GetSamplingStatisticSummariesRequest,
    ) -> Result<GetSamplingStatisticSummariesResult, RusotoError<GetSamplingStatisticSummariesError>>;

    /// <p>Requests a sampling quota for rules that the service is using to sample requests. </p>
    async fn get_sampling_targets(
        &self,
        input: GetSamplingTargetsRequest,
    ) -> Result<GetSamplingTargetsResult, RusotoError<GetSamplingTargetsError>>;

    /// <p>Retrieves a document that describes services that process incoming requests, and downstream services that they call as a result. Root services process incoming requests and make calls to downstream services. Root services are applications that use the <a href="https://docs.aws.amazon.com/xray/index.html">AWS X-Ray SDK</a>. Downstream services can be other applications, AWS resources, HTTP web APIs, or SQL databases.</p>
    async fn get_service_graph(
        &self,
        input: GetServiceGraphRequest,
    ) -> Result<GetServiceGraphResult, RusotoError<GetServiceGraphError>>;

    /// <p>Get an aggregation of service statistics defined by a specific time range.</p>
    async fn get_time_series_service_statistics(
        &self,
        input: GetTimeSeriesServiceStatisticsRequest,
    ) -> Result<
        GetTimeSeriesServiceStatisticsResult,
        RusotoError<GetTimeSeriesServiceStatisticsError>,
    >;

    /// <p>Retrieves a service graph for one or more specific trace IDs.</p>
    async fn get_trace_graph(
        &self,
        input: GetTraceGraphRequest,
    ) -> Result<GetTraceGraphResult, RusotoError<GetTraceGraphError>>;

    /// <p>Retrieves IDs and annotations for traces available for a specified time frame using an optional filter. To get the full traces, pass the trace IDs to <code>BatchGetTraces</code>.</p> <p>A filter expression can target traced requests that hit specific service nodes or edges, have errors, or come from a known user. For example, the following filter expression targets traces that pass through <code>api.example.com</code>:</p> <p> <code>service("api.example.com")</code> </p> <p>This filter expression finds traces that have an annotation named <code>account</code> with the value <code>12345</code>:</p> <p> <code>annotation.account = "12345"</code> </p> <p>For a full list of indexed fields and keywords that you can use in filter expressions, see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-console-filters.html">Using Filter Expressions</a> in the <i>AWS X-Ray Developer Guide</i>.</p>
    async fn get_trace_summaries(
        &self,
        input: GetTraceSummariesRequest,
    ) -> Result<GetTraceSummariesResult, RusotoError<GetTraceSummariesError>>;

    /// <p>Returns a list of tags that are applied to the specified AWS X-Ray group or sampling rule.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Updates the encryption configuration for X-Ray data.</p>
    async fn put_encryption_config(
        &self,
        input: PutEncryptionConfigRequest,
    ) -> Result<PutEncryptionConfigResult, RusotoError<PutEncryptionConfigError>>;

    /// <p>Used by the AWS X-Ray daemon to upload telemetry.</p>
    async fn put_telemetry_records(
        &self,
        input: PutTelemetryRecordsRequest,
    ) -> Result<PutTelemetryRecordsResult, RusotoError<PutTelemetryRecordsError>>;

    /// <p><p>Uploads segment documents to AWS X-Ray. The <a href="https://docs.aws.amazon.com/xray/index.html">X-Ray SDK</a> generates segment documents and sends them to the X-Ray daemon, which uploads them in batches. A segment document can be a completed segment, an in-progress segment, or an array of subsegments.</p> <p>Segments must include the following fields. For the full segment document schema, see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html">AWS X-Ray Segment Documents</a> in the <i>AWS X-Ray Developer Guide</i>.</p> <p class="title"> <b>Required segment document fields</b> </p> <ul> <li> <p> <code>name</code> - The name of the service that handled the request.</p> </li> <li> <p> <code>id</code> - A 64-bit identifier for the segment, unique among segments in the same trace, in 16 hexadecimal digits.</p> </li> <li> <p> <code>trace<em>id</code> - A unique identifier that connects all segments and subsegments originating from a single client request.</p> </li> <li> <p> <code>start</em>time</code> - Time the segment or subsegment was created, in floating point seconds in epoch time, accurate to milliseconds. For example, <code>1480615200.010</code> or <code>1.480615200010E9</code>.</p> </li> <li> <p> <code>end<em>time</code> - Time the segment or subsegment was closed. For example, <code>1480615200.090</code> or <code>1.480615200090E9</code>. Specify either an <code>end</em>time</code> or <code>in<em>progress</code>.</p> </li> <li> <p> <code>in</em>progress</code> - Set to <code>true</code> instead of specifying an <code>end<em>time</code> to record that a segment has been started, but is not complete. Send an in-progress segment when your application receives a request that will take a long time to serve, to trace that the request was received. When the response is sent, send the complete segment to overwrite the in-progress segment.</p> </li> </ul> <p>A <code>trace</em>id</code> consists of three numbers separated by hyphens. For example, 1-58406520-a006649127e371903a2de979. This includes:</p> <p class="title"> <b>Trace ID Format</b> </p> <ul> <li> <p>The version number, for instance, <code>1</code>.</p> </li> <li> <p>The time of the original request, in Unix epoch time, in 8 hexadecimal digits. For example, 10:00AM December 2nd, 2016 PST in epoch time is <code>1480615200</code> seconds, or <code>58406520</code> in hexadecimal.</p> </li> <li> <p>A 96-bit identifier for the trace, globally unique, in 24 hexadecimal digits.</p> </li> </ul></p>
    async fn put_trace_segments(
        &self,
        input: PutTraceSegmentsRequest,
    ) -> Result<PutTraceSegmentsResult, RusotoError<PutTraceSegmentsError>>;

    /// <p>Applies tags to an existing AWS X-Ray group or sampling rule.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes tags from an AWS X-Ray group or sampling rule. You cannot edit or delete system tags (those with an <code>aws:</code> prefix).</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates a group resource.</p>
    async fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> Result<UpdateGroupResult, RusotoError<UpdateGroupError>>;

    /// <p>Modifies a sampling rule's configuration.</p>
    async fn update_sampling_rule(
        &self,
        input: UpdateSamplingRuleRequest,
    ) -> Result<UpdateSamplingRuleResult, RusotoError<UpdateSamplingRuleError>>;
}
/// A client for the AWS X-Ray API.
#[derive(Clone)]
pub struct XRayClient {
    client: Client,
    region: region::Region,
}

impl XRayClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> XRayClient {
        XRayClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> XRayClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        XRayClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> XRayClient {
        XRayClient { client, region }
    }
}

#[async_trait]
impl XRay for XRayClient {
    /// <p>Retrieves a list of traces specified by ID. Each trace is a collection of segment documents that originates from a single request. Use <code>GetTraceSummaries</code> to get a list of trace IDs.</p>
    #[allow(unused_mut)]
    async fn batch_get_traces(
        &self,
        input: BatchGetTracesRequest,
    ) -> Result<BatchGetTracesResult, RusotoError<BatchGetTracesError>> {
        let request_uri = "/Traces";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchGetTracesResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetTracesError::from_response(response))
        }
    }

    /// <p>Creates a group resource with a name and a filter expression. </p>
    #[allow(unused_mut)]
    async fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> Result<CreateGroupResult, RusotoError<CreateGroupError>> {
        let request_uri = "/CreateGroup";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateGroupResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateGroupError::from_response(response))
        }
    }

    /// <p>Creates a rule to control sampling behavior for instrumented applications. Services retrieve rules with <a>GetSamplingRules</a>, and evaluate each rule in ascending order of <i>priority</i> for each request. If a rule matches, the service records a trace, borrowing it from the reservoir size. After 10 seconds, the service reports back to X-Ray with <a>GetSamplingTargets</a> to get updated versions of each in-use rule. The updated rule contains a trace quota that the service can use instead of borrowing from the reservoir.</p>
    #[allow(unused_mut)]
    async fn create_sampling_rule(
        &self,
        input: CreateSamplingRuleRequest,
    ) -> Result<CreateSamplingRuleResult, RusotoError<CreateSamplingRuleError>> {
        let request_uri = "/CreateSamplingRule";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateSamplingRuleResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSamplingRuleError::from_response(response))
        }
    }

    /// <p>Deletes a group resource.</p>
    #[allow(unused_mut)]
    async fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> Result<DeleteGroupResult, RusotoError<DeleteGroupError>> {
        let request_uri = "/DeleteGroup";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteGroupResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGroupError::from_response(response))
        }
    }

    /// <p>Deletes a sampling rule.</p>
    #[allow(unused_mut)]
    async fn delete_sampling_rule(
        &self,
        input: DeleteSamplingRuleRequest,
    ) -> Result<DeleteSamplingRuleResult, RusotoError<DeleteSamplingRuleError>> {
        let request_uri = "/DeleteSamplingRule";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSamplingRuleResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSamplingRuleError::from_response(response))
        }
    }

    /// <p>Retrieves the current encryption configuration for X-Ray data.</p>
    #[allow(unused_mut)]
    async fn get_encryption_config(
        &self,
    ) -> Result<GetEncryptionConfigResult, RusotoError<GetEncryptionConfigError>> {
        let request_uri = "/EncryptionConfig";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetEncryptionConfigResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEncryptionConfigError::from_response(response))
        }
    }

    /// <p>Retrieves group resource details.</p>
    #[allow(unused_mut)]
    async fn get_group(
        &self,
        input: GetGroupRequest,
    ) -> Result<GetGroupResult, RusotoError<GetGroupError>> {
        let request_uri = "/GetGroup";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetGroupResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetGroupError::from_response(response))
        }
    }

    /// <p>Retrieves all active group details.</p>
    #[allow(unused_mut)]
    async fn get_groups(
        &self,
        input: GetGroupsRequest,
    ) -> Result<GetGroupsResult, RusotoError<GetGroupsError>> {
        let request_uri = "/Groups";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetGroupsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetGroupsError::from_response(response))
        }
    }

    /// <p>Retrieves the summary information of an insight. This includes impact to clients and root cause services, the top anomalous services, the category, the state of the insight, and the start and end time of the insight.</p>
    #[allow(unused_mut)]
    async fn get_insight(
        &self,
        input: GetInsightRequest,
    ) -> Result<GetInsightResult, RusotoError<GetInsightError>> {
        let request_uri = "/Insight";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInsightResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInsightError::from_response(response))
        }
    }

    /// <p>X-Ray reevaluates insights periodically until they're resolved, and records each intermediate state as an event. You can review an insight's events in the Impact Timeline on the Inspect page in the X-Ray console.</p>
    #[allow(unused_mut)]
    async fn get_insight_events(
        &self,
        input: GetInsightEventsRequest,
    ) -> Result<GetInsightEventsResult, RusotoError<GetInsightEventsError>> {
        let request_uri = "/InsightEvents";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInsightEventsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInsightEventsError::from_response(response))
        }
    }

    /// <p>Retrieves a service graph structure filtered by the specified insight. The service graph is limited to only structural information. For a complete service graph, use this API with the GetServiceGraph API.</p>
    #[allow(unused_mut)]
    async fn get_insight_impact_graph(
        &self,
        input: GetInsightImpactGraphRequest,
    ) -> Result<GetInsightImpactGraphResult, RusotoError<GetInsightImpactGraphError>> {
        let request_uri = "/InsightImpactGraph";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInsightImpactGraphResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInsightImpactGraphError::from_response(response))
        }
    }

    /// <p>Retrieves the summaries of all insights in the specified group matching the provided filter values.</p>
    #[allow(unused_mut)]
    async fn get_insight_summaries(
        &self,
        input: GetInsightSummariesRequest,
    ) -> Result<GetInsightSummariesResult, RusotoError<GetInsightSummariesError>> {
        let request_uri = "/InsightSummaries";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInsightSummariesResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInsightSummariesError::from_response(response))
        }
    }

    /// <p>Retrieves all sampling rules.</p>
    #[allow(unused_mut)]
    async fn get_sampling_rules(
        &self,
        input: GetSamplingRulesRequest,
    ) -> Result<GetSamplingRulesResult, RusotoError<GetSamplingRulesError>> {
        let request_uri = "/GetSamplingRules";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSamplingRulesResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSamplingRulesError::from_response(response))
        }
    }

    /// <p>Retrieves information about recent sampling results for all sampling rules.</p>
    #[allow(unused_mut)]
    async fn get_sampling_statistic_summaries(
        &self,
        input: GetSamplingStatisticSummariesRequest,
    ) -> Result<GetSamplingStatisticSummariesResult, RusotoError<GetSamplingStatisticSummariesError>>
    {
        let request_uri = "/SamplingStatisticSummaries";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSamplingStatisticSummariesResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSamplingStatisticSummariesError::from_response(response))
        }
    }

    /// <p>Requests a sampling quota for rules that the service is using to sample requests. </p>
    #[allow(unused_mut)]
    async fn get_sampling_targets(
        &self,
        input: GetSamplingTargetsRequest,
    ) -> Result<GetSamplingTargetsResult, RusotoError<GetSamplingTargetsError>> {
        let request_uri = "/SamplingTargets";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSamplingTargetsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSamplingTargetsError::from_response(response))
        }
    }

    /// <p>Retrieves a document that describes services that process incoming requests, and downstream services that they call as a result. Root services process incoming requests and make calls to downstream services. Root services are applications that use the <a href="https://docs.aws.amazon.com/xray/index.html">AWS X-Ray SDK</a>. Downstream services can be other applications, AWS resources, HTTP web APIs, or SQL databases.</p>
    #[allow(unused_mut)]
    async fn get_service_graph(
        &self,
        input: GetServiceGraphRequest,
    ) -> Result<GetServiceGraphResult, RusotoError<GetServiceGraphError>> {
        let request_uri = "/ServiceGraph";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetServiceGraphResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetServiceGraphError::from_response(response))
        }
    }

    /// <p>Get an aggregation of service statistics defined by a specific time range.</p>
    #[allow(unused_mut)]
    async fn get_time_series_service_statistics(
        &self,
        input: GetTimeSeriesServiceStatisticsRequest,
    ) -> Result<
        GetTimeSeriesServiceStatisticsResult,
        RusotoError<GetTimeSeriesServiceStatisticsError>,
    > {
        let request_uri = "/TimeSeriesServiceStatistics";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetTimeSeriesServiceStatisticsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetTimeSeriesServiceStatisticsError::from_response(response))
        }
    }

    /// <p>Retrieves a service graph for one or more specific trace IDs.</p>
    #[allow(unused_mut)]
    async fn get_trace_graph(
        &self,
        input: GetTraceGraphRequest,
    ) -> Result<GetTraceGraphResult, RusotoError<GetTraceGraphError>> {
        let request_uri = "/TraceGraph";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetTraceGraphResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetTraceGraphError::from_response(response))
        }
    }

    /// <p>Retrieves IDs and annotations for traces available for a specified time frame using an optional filter. To get the full traces, pass the trace IDs to <code>BatchGetTraces</code>.</p> <p>A filter expression can target traced requests that hit specific service nodes or edges, have errors, or come from a known user. For example, the following filter expression targets traces that pass through <code>api.example.com</code>:</p> <p> <code>service("api.example.com")</code> </p> <p>This filter expression finds traces that have an annotation named <code>account</code> with the value <code>12345</code>:</p> <p> <code>annotation.account = "12345"</code> </p> <p>For a full list of indexed fields and keywords that you can use in filter expressions, see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-console-filters.html">Using Filter Expressions</a> in the <i>AWS X-Ray Developer Guide</i>.</p>
    #[allow(unused_mut)]
    async fn get_trace_summaries(
        &self,
        input: GetTraceSummariesRequest,
    ) -> Result<GetTraceSummariesResult, RusotoError<GetTraceSummariesError>> {
        let request_uri = "/TraceSummaries";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetTraceSummariesResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetTraceSummariesError::from_response(response))
        }
    }

    /// <p>Returns a list of tags that are applied to the specified AWS X-Ray group or sampling rule.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = "/ListTagsForResource";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Updates the encryption configuration for X-Ray data.</p>
    #[allow(unused_mut)]
    async fn put_encryption_config(
        &self,
        input: PutEncryptionConfigRequest,
    ) -> Result<PutEncryptionConfigResult, RusotoError<PutEncryptionConfigError>> {
        let request_uri = "/PutEncryptionConfig";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutEncryptionConfigResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEncryptionConfigError::from_response(response))
        }
    }

    /// <p>Used by the AWS X-Ray daemon to upload telemetry.</p>
    #[allow(unused_mut)]
    async fn put_telemetry_records(
        &self,
        input: PutTelemetryRecordsRequest,
    ) -> Result<PutTelemetryRecordsResult, RusotoError<PutTelemetryRecordsError>> {
        let request_uri = "/TelemetryRecords";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutTelemetryRecordsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutTelemetryRecordsError::from_response(response))
        }
    }

    /// <p><p>Uploads segment documents to AWS X-Ray. The <a href="https://docs.aws.amazon.com/xray/index.html">X-Ray SDK</a> generates segment documents and sends them to the X-Ray daemon, which uploads them in batches. A segment document can be a completed segment, an in-progress segment, or an array of subsegments.</p> <p>Segments must include the following fields. For the full segment document schema, see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html">AWS X-Ray Segment Documents</a> in the <i>AWS X-Ray Developer Guide</i>.</p> <p class="title"> <b>Required segment document fields</b> </p> <ul> <li> <p> <code>name</code> - The name of the service that handled the request.</p> </li> <li> <p> <code>id</code> - A 64-bit identifier for the segment, unique among segments in the same trace, in 16 hexadecimal digits.</p> </li> <li> <p> <code>trace<em>id</code> - A unique identifier that connects all segments and subsegments originating from a single client request.</p> </li> <li> <p> <code>start</em>time</code> - Time the segment or subsegment was created, in floating point seconds in epoch time, accurate to milliseconds. For example, <code>1480615200.010</code> or <code>1.480615200010E9</code>.</p> </li> <li> <p> <code>end<em>time</code> - Time the segment or subsegment was closed. For example, <code>1480615200.090</code> or <code>1.480615200090E9</code>. Specify either an <code>end</em>time</code> or <code>in<em>progress</code>.</p> </li> <li> <p> <code>in</em>progress</code> - Set to <code>true</code> instead of specifying an <code>end<em>time</code> to record that a segment has been started, but is not complete. Send an in-progress segment when your application receives a request that will take a long time to serve, to trace that the request was received. When the response is sent, send the complete segment to overwrite the in-progress segment.</p> </li> </ul> <p>A <code>trace</em>id</code> consists of three numbers separated by hyphens. For example, 1-58406520-a006649127e371903a2de979. This includes:</p> <p class="title"> <b>Trace ID Format</b> </p> <ul> <li> <p>The version number, for instance, <code>1</code>.</p> </li> <li> <p>The time of the original request, in Unix epoch time, in 8 hexadecimal digits. For example, 10:00AM December 2nd, 2016 PST in epoch time is <code>1480615200</code> seconds, or <code>58406520</code> in hexadecimal.</p> </li> <li> <p>A 96-bit identifier for the trace, globally unique, in 24 hexadecimal digits.</p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn put_trace_segments(
        &self,
        input: PutTraceSegmentsRequest,
    ) -> Result<PutTraceSegmentsResult, RusotoError<PutTraceSegmentsError>> {
        let request_uri = "/TraceSegments";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutTraceSegmentsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutTraceSegmentsError::from_response(response))
        }
    }

    /// <p>Applies tags to an existing AWS X-Ray group or sampling rule.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = "/TagResource";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes tags from an AWS X-Ray group or sampling rule. You cannot edit or delete system tags (those with an <code>aws:</code> prefix).</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = "/UntagResource";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates a group resource.</p>
    #[allow(unused_mut)]
    async fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> Result<UpdateGroupResult, RusotoError<UpdateGroupError>> {
        let request_uri = "/UpdateGroup";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateGroupResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGroupError::from_response(response))
        }
    }

    /// <p>Modifies a sampling rule's configuration.</p>
    #[allow(unused_mut)]
    async fn update_sampling_rule(
        &self,
        input: UpdateSamplingRuleRequest,
    ) -> Result<UpdateSamplingRuleResult, RusotoError<UpdateSamplingRuleError>> {
        let request_uri = "/UpdateSamplingRule";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateSamplingRuleResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSamplingRuleError::from_response(response))
        }
    }
}
