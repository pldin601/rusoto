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
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl EventBridgeClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "events", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ActivateEventSourceRequest {
    /// <p>The name of the partner event source to activate.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Contains details about an API destination.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApiDestination {
    /// <p>The ARN of the API destination.</p>
    #[serde(rename = "apiDestinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_arn: Option<String>,
    /// <p>The state of the API destination.</p>
    #[serde(rename = "apiDestinationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_state: Option<String>,
    /// <p>The ARN of the connection specified for the API destination.</p>
    #[serde(rename = "connectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>A time stamp for the time that the API destination was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The method to use to connect to the HTTP endpoint.</p>
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p>The URL to the endpoint for the API destination.</p>
    #[serde(rename = "invocationEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_endpoint: Option<String>,
    /// <p>The maximum number of invocations per second to send to the HTTP endpoint.</p>
    #[serde(rename = "invocationRateLimitPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_rate_limit_per_second: Option<i64>,
    /// <p>A time stamp for the time that the API destination was last modified.</p>
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the API destination.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>An <code>Archive</code> object that contains details about an archive.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Archive {
    /// <p>The name of the archive.</p>
    #[serde(rename = "archiveName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_name: Option<String>,
    /// <p>The time stamp for the time that the archive was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The number of events in the archive.</p>
    #[serde(rename = "eventCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i64>,
    /// <p>The ARN of the event bus associated with the archive. Only events from this event bus are sent to the archive.</p>
    #[serde(rename = "eventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>The number of days to retain events in the archive before they are deleted.</p>
    #[serde(rename = "retentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i64>,
    /// <p>The size of the archive, in bytes.</p>
    #[serde(rename = "sizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    /// <p>The current state of the archive.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description for the reason that the archive is in the current state.</p>
    #[serde(rename = "stateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

/// <p>This structure specifies the VPC subnets and security groups for the task, and whether a public IP address is to be used. This structure is relevant only for ECS tasks that use the <code>awsvpc</code> network mode.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AwsVpcConfiguration {
    /// <p>Specifies whether the task's elastic network interface receives a public IP address. You can specify <code>ENABLED</code> only when <code>LaunchType</code> in <code>EcsParameters</code> is set to <code>FARGATE</code>.</p>
    #[serde(rename = "assignPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
    /// <p>Specifies the security groups associated with the task. These security groups must all be in the same VPC. You can specify as many as five security groups. If you do not specify a security group, the default security group for the VPC is used.</p>
    #[serde(rename = "securityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>Specifies the subnets associated with the task. These subnets must all be in the same VPC. You can specify as many as 16 subnets.</p>
    #[serde(rename = "subnets")]
    pub subnets: Vec<String>,
}

/// <p>The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000. If you specify array properties for a job, it becomes an array job. This parameter is used only if the target is an AWS Batch job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BatchArrayProperties {
    /// <p>The size of the array, if this is an array batch job. Valid values are integers between 2 and 10,000.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// <p>The custom parameters to be used when the target is an AWS Batch job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BatchParameters {
    /// <p>The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000. If you specify array properties for a job, it becomes an array job. This parameter is used only if the target is an AWS Batch job.</p>
    #[serde(rename = "arrayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<BatchArrayProperties>,
    /// <p>The ARN or name of the job definition to use if the event target is an AWS Batch job. This job definition must already exist.</p>
    #[serde(rename = "jobDefinition")]
    pub job_definition: String,
    /// <p>The name to use for this execution of the job, if the target is an AWS Batch job.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p>The retry strategy to use for failed jobs, if the target is an AWS Batch job. The retry strategy is the number of times to retry the failed job execution. Valid values are 1–10. When you specify a retry strategy here, it overrides the retry strategy defined in the job definition.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<BatchRetryStrategy>,
}

/// <p>The retry strategy to use for failed jobs, if the target is an AWS Batch job. If you specify a retry strategy here, it overrides the retry strategy defined in the job definition.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BatchRetryStrategy {
    /// <p>The number of times to attempt to retry, if the job fails. Valid values are 1–10.</p>
    #[serde(rename = "attempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelReplayRequest {
    /// <p>The name of the replay to cancel.</p>
    #[serde(rename = "replayName")]
    pub replay_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelReplayResponse {
    /// <p>The ARN of the replay to cancel.</p>
    #[serde(rename = "replayArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_arn: Option<String>,
    /// <p>The current state of the replay.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason that the replay is in the current state.</p>
    #[serde(rename = "stateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

/// <p>The details of a capacity provider strategy. To learn more, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CapacityProviderStrategyItem.html">CapacityProviderStrategyItem</a> in the Amazon ECS API Reference.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CapacityProviderStrategyItem {
    /// <p>The base value designates how many tasks, at a minimum, to run on the specified capacity provider. Only one capacity provider in a capacity provider strategy can have a base defined. If no value is specified, the default value of 0 is used. </p>
    #[serde(rename = "base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i64>,
    /// <p>The short name of the capacity provider.</p>
    #[serde(rename = "capacityProvider")]
    pub capacity_provider: String,
    /// <p>The weight value designates the relative percentage of the total number of tasks launched that should use the specified capacity provider. The weight value is taken into consideration after the base value, if defined, is satisfied.</p>
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// <p>A JSON string which you can use to limit the event bus permissions you are granting to only accounts that fulfill the condition. Currently, the only supported condition is membership in a certain AWS organization. The string must contain <code>Type</code>, <code>Key</code>, and <code>Value</code> fields. The <code>Value</code> field specifies the ID of the AWS organization. Following is an example value for <code>Condition</code>:</p> <p> <code>'{"Type" : "StringEquals", "Key": "aws:PrincipalOrgID", "Value": "o-1234567890"}'</code> </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Condition {
    /// <p>Specifies the key for the condition. Currently the only supported key is <code>aws:PrincipalOrgID</code>.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>Specifies the type of condition. Currently the only supported value is <code>StringEquals</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p>Specifies the value for the key. Currently, this must be the ID of the organization.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>Contains information about a connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Connection {
    /// <p>The authorization type specified for the connection.</p>
    #[serde(rename = "authorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The ARN of the connection.</p>
    #[serde(rename = "connectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The state of the connection.</p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>A time stamp for the time that the connection was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A time stamp for the time that the connection was last authorized.</p>
    #[serde(rename = "lastAuthorizedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authorized_time: Option<f64>,
    /// <p>A time stamp for the time that the connection was last modified.</p>
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the connection.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The reason that the connection is in the connection state.</p>
    #[serde(rename = "stateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

/// <p>Contains the authorization parameters for the connection if API Key is specified as the authorization type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConnectionApiKeyAuthResponseParameters {
    /// <p>The name of the header to use for the <code>APIKeyValue</code> used for authorization.</p>
    #[serde(rename = "apiKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_name: Option<String>,
}

/// <p>Contains the authorization parameters to use for the connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConnectionAuthResponseParameters {
    /// <p>The API Key parameters to use for authorization.</p>
    #[serde(rename = "apiKeyAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_auth_parameters: Option<ConnectionApiKeyAuthResponseParameters>,
    /// <p>The authorization parameters for Basic authorization.</p>
    #[serde(rename = "basicAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_parameters: Option<ConnectionBasicAuthResponseParameters>,
    /// <p>Additional parameters for the connection that are passed through with every invocation to the HTTP endpoint.</p>
    #[serde(rename = "invocationHttpParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_http_parameters: Option<ConnectionHttpParameters>,
    /// <p>The OAuth parameters to use for authorization.</p>
    #[serde(rename = "oAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_parameters: Option<ConnectionOAuthResponseParameters>,
}

/// <p>Contains the authorization parameters for the connection if Basic is specified as the authorization type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConnectionBasicAuthResponseParameters {
    /// <p>The user name to use for Basic authorization.</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Additional parameter included in the body. You can include up to 100 additional body parameters per request. An event payload cannot exceed 64 KB.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConnectionBodyParameter {
    /// <p>Specified whether the value is secret.</p>
    #[serde(rename = "isValueSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_value_secret: Option<bool>,
    /// <p>The key for the parameter.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value associated with the key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Additional parameter included in the header. You can include up to 100 additional header parameters per request. An event payload cannot exceed 64 KB.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConnectionHeaderParameter {
    /// <p>Specified whether the value is a secret.</p>
    #[serde(rename = "isValueSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_value_secret: Option<bool>,
    /// <p>The key for the parameter.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value associated with the key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Contains additional parameters for the connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConnectionHttpParameters {
    /// <p>Contains additional body string parameters for the connection.</p>
    #[serde(rename = "bodyParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_parameters: Option<Vec<ConnectionBodyParameter>>,
    /// <p>Contains additional header parameters for the connection.</p>
    #[serde(rename = "headerParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_parameters: Option<Vec<ConnectionHeaderParameter>>,
    /// <p>Contains additional query string parameters for the connection.</p>
    #[serde(rename = "queryStringParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string_parameters: Option<Vec<ConnectionQueryStringParameter>>,
}

/// <p>Contains the client response parameters for the connection when OAuth is specified as the authorization type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConnectionOAuthClientResponseParameters {
    /// <p>The client ID associated with the response to the connection request.</p>
    #[serde(rename = "clientID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

/// <p>Contains the response parameters when OAuth is specified as the authorization type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConnectionOAuthResponseParameters {
    /// <p>The URL to the HTTP endpoint that authorized the request.</p>
    #[serde(rename = "authorizationEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    /// <p>A <code>ConnectionOAuthClientResponseParameters</code> object that contains details about the client parameters returned when OAuth is specified as the authorization type.</p>
    #[serde(rename = "clientParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_parameters: Option<ConnectionOAuthClientResponseParameters>,
    /// <p>The method used to connect to the HTTP endpoint.</p>
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p>The additional HTTP parameters used for the OAuth authorization request.</p>
    #[serde(rename = "oAuthHttpParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_http_parameters: Option<ConnectionHttpParameters>,
}

/// <p>Additional query string parameter for the connection. You can include up to 100 additional query string parameters per request. Each additional parameter counts towards the event payload size, which cannot exceed 64 KB.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConnectionQueryStringParameter {
    /// <p>Specifies whether the value is secret.</p>
    #[serde(rename = "isValueSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_value_secret: Option<bool>,
    /// <p>The key for a query string parameter.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value associated with the key for the query string parameter.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApiDestinationRequest {
    /// <p>The ARN of the connection to use for the API destination. The destination endpoint must support the authorization type specified for the connection.</p>
    #[serde(rename = "connectionArn")]
    pub connection_arn: String,
    /// <p>A description for the API destination to create.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The method to use for the request to the HTTP invocation endpoint.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>The URL to the HTTP invocation endpoint for the API destination.</p>
    #[serde(rename = "invocationEndpoint")]
    pub invocation_endpoint: String,
    /// <p>The maximum number of requests per second to send to the HTTP invocation endpoint.</p>
    #[serde(rename = "invocationRateLimitPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_rate_limit_per_second: Option<i64>,
    /// <p>The name for the API destination to create.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateApiDestinationResponse {
    /// <p>The ARN of the API destination that was created by the request.</p>
    #[serde(rename = "apiDestinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_arn: Option<String>,
    /// <p>The state of the API destination that was created by the request.</p>
    #[serde(rename = "apiDestinationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_state: Option<String>,
    /// <p>A time stamp indicating the time that the API destination was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A time stamp indicating the time that the API destination was last modified.</p>
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateArchiveRequest {
    /// <p>The name for the archive to create.</p>
    #[serde(rename = "archiveName")]
    pub archive_name: String,
    /// <p>A description for the archive.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An event pattern to use to filter events sent to the archive.</p>
    #[serde(rename = "eventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    /// <p>The ARN of the event source associated with the archive.</p>
    #[serde(rename = "eventSourceArn")]
    pub event_source_arn: String,
    /// <p>The number of days to retain events for. Default value is 0. If set to 0, events are retained indefinitely</p>
    #[serde(rename = "retentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateArchiveResponse {
    /// <p>The ARN of the archive that was created.</p>
    #[serde(rename = "archiveArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_arn: Option<String>,
    /// <p>The time at which the archive was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The state of the archive that was created.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason that the archive is in the state.</p>
    #[serde(rename = "stateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

/// <p>Contains the API key authorization parameters for the connection.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConnectionApiKeyAuthRequestParameters {
    /// <p>The name of the API key to use for authorization.</p>
    #[serde(rename = "apiKeyName")]
    pub api_key_name: String,
    /// <p>The value for the API key to use for authorization.</p>
    #[serde(rename = "apiKeyValue")]
    pub api_key_value: String,
}

/// <p>Contains the authorization parameters for the connection.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConnectionAuthRequestParameters {
    /// <p>A <code>CreateConnectionApiKeyAuthRequestParameters</code> object that contains the API key authorization parameters to use for the connection.</p>
    #[serde(rename = "apiKeyAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_auth_parameters: Option<CreateConnectionApiKeyAuthRequestParameters>,
    /// <p>A <code>CreateConnectionBasicAuthRequestParameters</code> object that contains the Basic authorization parameters to use for the connection.</p>
    #[serde(rename = "basicAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_parameters: Option<CreateConnectionBasicAuthRequestParameters>,
    /// <p>A <code>ConnectionHttpParameters</code> object that contains the API key authorization parameters to use for the connection. Note that if you include additional parameters for the target of a rule via <code>HttpParameters</code>, including query strings, the parameters added for the connection take precedence.</p>
    #[serde(rename = "invocationHttpParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_http_parameters: Option<ConnectionHttpParameters>,
    /// <p>A <code>CreateConnectionOAuthRequestParameters</code> object that contains the OAuth authorization parameters to use for the connection.</p>
    #[serde(rename = "oAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_parameters: Option<CreateConnectionOAuthRequestParameters>,
}

/// <p>Contains the Basic authorization parameters to use for the connection.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConnectionBasicAuthRequestParameters {
    /// <p>The password associated with the user name to use for Basic authorization.</p>
    #[serde(rename = "password")]
    pub password: String,
    /// <p>The user name to use for Basic authorization.</p>
    #[serde(rename = "username")]
    pub username: String,
}

/// <p>Contains the Basic authorization parameters to use for the connection.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConnectionOAuthClientRequestParameters {
    /// <p>The client ID to use for OAuth authorization for the connection.</p>
    #[serde(rename = "clientID")]
    pub client_id: String,
    /// <p>The client secret associated with the client ID to use for OAuth authorization for the connection.</p>
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
}

/// <p>Contains the OAuth authorization parameters to use for the connection.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConnectionOAuthRequestParameters {
    /// <p>The URL to the authorization endpoint when OAuth is specified as the authorization type.</p>
    #[serde(rename = "authorizationEndpoint")]
    pub authorization_endpoint: String,
    /// <p>A <code>CreateConnectionOAuthClientRequestParameters</code> object that contains the client parameters for OAuth authorization.</p>
    #[serde(rename = "clientParameters")]
    pub client_parameters: CreateConnectionOAuthClientRequestParameters,
    /// <p>The method to use for the authorization request.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>A <code>ConnectionHttpParameters</code> object that contains details about the additional parameters to use for the connection.</p>
    #[serde(rename = "oAuthHttpParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_http_parameters: Option<ConnectionHttpParameters>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConnectionRequest {
    /// <p>A <code>CreateConnectionAuthRequestParameters</code> object that contains the authorization parameters to use to authorize with the endpoint. </p>
    #[serde(rename = "authParameters")]
    pub auth_parameters: CreateConnectionAuthRequestParameters,
    /// <p>The type of authorization to use for the connection.</p>
    #[serde(rename = "authorizationType")]
    pub authorization_type: String,
    /// <p>A description for the connection to create.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name for the connection to create.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConnectionResponse {
    /// <p>The ARN of the connection that was created by the request.</p>
    #[serde(rename = "connectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The state of the connection that was created by the request.</p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>A time stamp for the time that the connection was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A time stamp for the time that the connection was last updated.</p>
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEventBusRequest {
    /// <p>If you are creating a partner event bus, this specifies the partner event source that the new event bus will be matched with.</p>
    #[serde(rename = "eventSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_name: Option<String>,
    /// <p>The name of the new event bus. </p> <p>Event bus names cannot contain the / character. You can't use the name <code>default</code> for a custom event bus, as this name is already used for your account's default event bus.</p> <p>If this is a partner event bus, the name must exactly match the name of the partner event source that this event bus is matched to.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Tags to associate with the event bus.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEventBusResponse {
    /// <p>The ARN of the new event bus.</p>
    #[serde(rename = "eventBusArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePartnerEventSourceRequest {
    /// <p>The AWS account ID that is permitted to create a matching partner event bus for this partner event source.</p>
    #[serde(rename = "account")]
    pub account: String,
    /// <p>The name of the partner event source. This name must be unique and must be in the format <code> <i>partner_name</i>/<i>event_namespace</i>/<i>event_name</i> </code>. The AWS account that wants to use this partner event source must create a partner event bus with a name that matches the name of the partner event source.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePartnerEventSourceResponse {
    /// <p>The ARN of the partner event source.</p>
    #[serde(rename = "eventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeactivateEventSourceRequest {
    /// <p>The name of the partner event source to deactivate.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>A <code>DeadLetterConfig</code> object that contains information about a dead-letter queue configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeadLetterConfig {
    /// <p>The ARN of the SQS queue specified as the target for the dead-letter queue.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeauthorizeConnectionRequest {
    /// <p>The name of the connection to remove authorization from.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeauthorizeConnectionResponse {
    /// <p>The ARN of the connection that authorization was removed from.</p>
    #[serde(rename = "connectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The state of the connection.</p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>A time stamp for the time that the connection was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A time stamp for the time that the connection was last authorized.</p>
    #[serde(rename = "lastAuthorizedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authorized_time: Option<f64>,
    /// <p>A time stamp for the time that the connection was last updated.</p>
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApiDestinationRequest {
    /// <p>The name of the destination to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApiDestinationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteArchiveRequest {
    /// <p>The name of the archive to delete.</p>
    #[serde(rename = "archiveName")]
    pub archive_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteArchiveResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConnectionRequest {
    /// <p>The name of the connection to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConnectionResponse {
    /// <p>The ARN of the connection that was deleted.</p>
    #[serde(rename = "connectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The state of the connection before it was deleted.</p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>A time stamp for the time that the connection was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A time stamp for the time that the connection was last authorized before it wa deleted.</p>
    #[serde(rename = "lastAuthorizedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authorized_time: Option<f64>,
    /// <p>A time stamp for the time that the connection was last modified before it was deleted.</p>
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEventBusRequest {
    /// <p>The name of the event bus to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePartnerEventSourceRequest {
    /// <p>The AWS account ID of the AWS customer that the event source was created for.</p>
    #[serde(rename = "account")]
    pub account: String,
    /// <p>The name of the event source to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRuleRequest {
    /// <p>The name or ARN of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>If this is a managed rule, created by an AWS service on your behalf, you must specify <code>Force</code> as <code>True</code> to delete the rule. This parameter is ignored for rules that are not managed rules. You can check whether a rule is a managed rule by using <code>DescribeRule</code> or <code>ListRules</code> and checking the <code>ManagedBy</code> field of the response.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeApiDestinationRequest {
    /// <p>The name of the API destination to retrieve.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeApiDestinationResponse {
    /// <p>The ARN of the API destination retrieved.</p>
    #[serde(rename = "apiDestinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_arn: Option<String>,
    /// <p>The state of the API destination retrieved.</p>
    #[serde(rename = "apiDestinationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_state: Option<String>,
    /// <p>The ARN of the connection specified for the API destination retrieved.</p>
    #[serde(rename = "connectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>A time stamp for the time that the API destination was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The description for the API destination retrieved.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The method to use to connect to the HTTP endpoint.</p>
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p>The URL to use to connect to the HTTP endpoint.</p>
    #[serde(rename = "invocationEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_endpoint: Option<String>,
    /// <p>The maximum number of invocations per second to specified for the API destination. Note that if you set the invocation rate maximum to a value lower the rate necessary to send all events received on to the destination HTTP endpoint, some events may not be delivered within the 24-hour retry window. If you plan to set the rate lower than the rate necessary to deliver all events, consider using a dead-letter queue to catch events that are not delivered within 24 hours.</p>
    #[serde(rename = "invocationRateLimitPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_rate_limit_per_second: Option<i64>,
    /// <p>A time stamp for the time that the API destination was last modified.</p>
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the API destination retrieved.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeArchiveRequest {
    /// <p>The name of the archive to retrieve.</p>
    #[serde(rename = "archiveName")]
    pub archive_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeArchiveResponse {
    /// <p>The ARN of the archive.</p>
    #[serde(rename = "archiveArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_arn: Option<String>,
    /// <p>The name of the archive.</p>
    #[serde(rename = "archiveName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_name: Option<String>,
    /// <p>The time at which the archive was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The description of the archive.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The number of events in the archive.</p>
    #[serde(rename = "eventCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i64>,
    /// <p>The event pattern used to filter events sent to the archive.</p>
    #[serde(rename = "eventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    /// <p>The ARN of the event source associated with the archive.</p>
    #[serde(rename = "eventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>The number of days to retain events for in the archive.</p>
    #[serde(rename = "retentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i64>,
    /// <p>The size of the archive in bytes.</p>
    #[serde(rename = "sizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    /// <p>The state of the archive.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason that the archive is in the state.</p>
    #[serde(rename = "stateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConnectionRequest {
    /// <p>The name of the connection to retrieve.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConnectionResponse {
    /// <p>The parameters to use for authorization for the connection.</p>
    #[serde(rename = "authParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<ConnectionAuthResponseParameters>,
    /// <p>The type of authorization specified for the connection.</p>
    #[serde(rename = "authorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The ARN of the connection retrieved.</p>
    #[serde(rename = "connectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The state of the connection retrieved.</p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>A time stamp for the time that the connection was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The description for the connection retrieved.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A time stamp for the time that the connection was last authorized.</p>
    #[serde(rename = "lastAuthorizedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authorized_time: Option<f64>,
    /// <p>A time stamp for the time that the connection was last modified.</p>
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the connection retrieved.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of the secret created from the authorization parameters specified for the connection.</p>
    #[serde(rename = "secretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    /// <p>The reason that the connection is in the current connection state.</p>
    #[serde(rename = "stateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventBusRequest {
    /// <p>The name or ARN of the event bus to show details for. If you omit this, the default event bus is displayed.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventBusResponse {
    /// <p>The Amazon Resource Name (ARN) of the account permitted to write events to the current account.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the event bus. Currently, this is always <code>default</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The policy that enables the external account to send events to your account.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventSourceRequest {
    /// <p>The name of the partner event source to display the details of.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventSourceResponse {
    /// <p>The ARN of the partner event source.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the SaaS partner that created the event source.</p>
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The date and time that the event source was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The date and time that the event source will expire if you do not create a matching event bus.</p>
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    /// <p>The name of the partner event source.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the event source. If it is ACTIVE, you have already created a matching event bus for this event source, and that event bus is active. If it is PENDING, either you haven't yet created a matching event bus, or that event bus is deactivated. If it is DELETED, you have created a matching event bus, but the event source has since been deleted.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePartnerEventSourceRequest {
    /// <p>The name of the event source to display.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePartnerEventSourceResponse {
    /// <p>The ARN of the event source.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the event source.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReplayRequest {
    /// <p>The name of the replay to retrieve.</p>
    #[serde(rename = "replayName")]
    pub replay_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReplayResponse {
    /// <p>The description of the replay.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A <code>ReplayDestination</code> object that contains details about the replay.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<ReplayDestination>,
    /// <p>The time stamp for the last event that was replayed from the archive.</p>
    #[serde(rename = "eventEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_end_time: Option<f64>,
    /// <p>The time that the event was last replayed.</p>
    #[serde(rename = "eventLastReplayedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_last_replayed_time: Option<f64>,
    /// <p>The ARN of the archive events were replayed from.</p>
    #[serde(rename = "eventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>The time stamp of the first event that was last replayed from the archive.</p>
    #[serde(rename = "eventStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<f64>,
    /// <p>The ARN of the replay.</p>
    #[serde(rename = "replayArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_arn: Option<String>,
    /// <p>A time stamp for the time that the replay stopped.</p>
    #[serde(rename = "replayEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_end_time: Option<f64>,
    /// <p>The name of the replay.</p>
    #[serde(rename = "replayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_name: Option<String>,
    /// <p>A time stamp for the time that the replay started.</p>
    #[serde(rename = "replayStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_start_time: Option<f64>,
    /// <p>The current state of the replay.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason that the replay is in the current state.</p>
    #[serde(rename = "stateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRuleRequest {
    /// <p>The name or ARN of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRuleResponse {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The account ID of the user that created the rule. If you use <code>PutRule</code> to put a rule on an event bus in another account, the other account is the owner of the rule, and the rule ARN includes the account ID for that account. However, the value for <code>CreatedBy</code> is the account ID as the account that created the rule in the other account.</p>
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The description of the rule.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the event bus associated with the rule.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html">Events and Event Patterns</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    #[serde(rename = "eventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    /// <p>If this is a managed rule, created by an AWS service on your behalf, this field displays the principal name of the AWS service that created the rule.</p>
    #[serde(rename = "managedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The scheduling expression. For example, "cron(0 20 * * ? *)", "rate(5 minutes)".</p>
    #[serde(rename = "scheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>Specifies whether the rule is enabled or disabled.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableRuleRequest {
    /// <p>The name or ARN of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>The custom parameters to be used when the target is an Amazon ECS task.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EcsParameters {
    /// <p>The capacity provider strategy to use for the task.</p> <p>If a <code>capacityProviderStrategy</code> is specified, the <code>launchType</code> parameter must be omitted. If no <code>capacityProviderStrategy</code> or launchType is specified, the <code>defaultCapacityProviderStrategy</code> for the cluster is used. </p>
    #[serde(rename = "capacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// <p>Specifies whether to enable Amazon ECS managed tags for the task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html">Tagging Your Amazon ECS Resources</a> in the Amazon Elastic Container Service Developer Guide. </p>
    #[serde(rename = "enableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecs_managed_tags: Option<bool>,
    /// <p>Whether or not to enable the execute command functionality for the containers in this task. If true, this enables execute command functionality on all containers in the task.</p>
    #[serde(rename = "enableExecuteCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    /// <p>Specifies an ECS task group for the task. The maximum length is 255 characters.</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p>Specifies the launch type on which your task is running. The launch type that you specify here must match one of the launch type (compatibilities) of the target task. The <code>FARGATE</code> value is supported only in the Regions where AWS Fargate with Amazon ECS is supported. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/AWS-Fargate.html">AWS Fargate on Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>Use this structure if the ECS task uses the <code>awsvpc</code> network mode. This structure specifies the VPC subnets and security groups associated with the task, and whether a public IP address is to be used. This structure is required if <code>LaunchType</code> is <code>FARGATE</code> because the <code>awsvpc</code> mode is required for Fargate tasks.</p> <p>If you specify <code>NetworkConfiguration</code> when the target ECS task does not use the <code>awsvpc</code> network mode, the task fails.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>An array of placement constraint objects to use for the task. You can specify up to 10 constraints per task (including constraints in the task definition and those specified at runtime).</p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    /// <p>The placement strategy objects to use for the task. You can specify a maximum of five strategy rules per task. </p>
    #[serde(rename = "placementStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    /// <p>Specifies the platform version for the task. Specify only the numeric portion of the platform version, such as <code>1.1.0</code>.</p> <p>This structure is used only if <code>LaunchType</code> is <code>FARGATE</code>. For more information about valid platform versions, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>Specifies whether to propagate the tags from the task definition to the task. If no value is specified, the tags are not propagated. Tags can only be propagated to the task during task creation. To add tags to a task after task creation, use the TagResource API action. </p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    /// <p>The reference ID to use for the task.</p>
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    /// <p>The metadata that you apply to the task to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. To learn more, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html#ECS-RunTask-request-tags">RunTask</a> in the Amazon ECS API Reference.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The number of tasks to create based on <code>TaskDefinition</code>. The default is 1.</p>
    #[serde(rename = "taskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_count: Option<i64>,
    /// <p>The ARN of the task definition to use if the event target is an Amazon ECS task. </p>
    #[serde(rename = "taskDefinitionArn")]
    pub task_definition_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableRuleRequest {
    /// <p>The name or ARN of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>An event bus receives events from a source and routes them to rules associated with that event bus. Your account's default event bus receives events from AWS services. A custom event bus can receive events from your custom applications and services. A partner event bus receives events from an event source created by an SaaS partner. These events come from the partners services or applications.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventBus {
    /// <p>The ARN of the event bus.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the event bus.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The permissions policy of the event bus, describing which other AWS accounts can write events to this event bus.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

/// <p>A partner event source is created by an SaaS partner. If a customer creates a partner event bus that matches this event source, that AWS account can receive events from the partner's applications or services.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventSource {
    /// <p>The ARN of the event source.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the partner that created the event source.</p>
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The date and time the event source was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The date and time that the event source will expire, if the AWS account doesn't create a matching event bus for it.</p>
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    /// <p>The name of the event source.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the event source. If it is ACTIVE, you have already created a matching event bus for this event source, and that event bus is active. If it is PENDING, either you haven't yet created a matching event bus, or that event bus is deactivated. If it is DELETED, you have created a matching event bus, but the event source has since been deleted.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>These are custom parameter to be used when the target is an API Gateway REST APIs or EventBridge ApiDestinations. In the latter case, these are merged with any InvocationParameters specified on the Connection, with any values from the Connection taking precedence.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpParameters {
    /// <p>The headers that need to be sent as part of request invoking the API Gateway REST API or EventBridge ApiDestination.</p>
    #[serde(rename = "headerParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The path parameter values to be used to populate API Gateway REST API or EventBridge ApiDestination path wildcards ("*").</p>
    #[serde(rename = "pathParameterValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_parameter_values: Option<Vec<String>>,
    /// <p>The query string keys/values that need to be sent as part of request invoking the API Gateway REST API or EventBridge ApiDestination.</p>
    #[serde(rename = "queryStringParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string_parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Contains the parameters needed for you to provide custom input to a target based on one or more pieces of data extracted from the event.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputTransformer {
    /// <p>Map of JSON paths to be extracted from the event. You can then insert these in the template in <code>InputTemplate</code> to produce the output you want to be sent to the target.</p> <p> <code>InputPathsMap</code> is an array key-value pairs, where each value is a valid JSON path. You can have as many as 100 key-value pairs. You must use JSON dot notation, not bracket notation.</p> <p>The keys cannot start with "AWS." </p>
    #[serde(rename = "inputPathsMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_paths_map: Option<::std::collections::HashMap<String, String>>,
    /// <p>Input template where you specify placeholders that will be filled with the values of the keys from <code>InputPathsMap</code> to customize the data sent to the target. Enclose each <code>InputPathsMaps</code> value in brackets: &lt;<i>value</i>&gt; The InputTemplate must be valid JSON.</p> <p>If <code>InputTemplate</code> is a JSON object (surrounded by curly braces), the following restrictions apply:</p> <ul> <li> <p>The placeholder cannot be used as an object key.</p> </li> </ul> <p>The following example shows the syntax for using <code>InputPathsMap</code> and <code>InputTemplate</code>.</p> <p> <code> "InputTransformer":</code> </p> <p> <code>{</code> </p> <p> <code>"InputPathsMap": {"instance": "$.detail.instance","status": "$.detail.status"},</code> </p> <p> <code>"InputTemplate": "&lt;instance&gt; is in state &lt;status&gt;"</code> </p> <p> <code>}</code> </p> <p>To have the <code>InputTemplate</code> include quote marks within a JSON string, escape each quote marks with a slash, as in the following example:</p> <p> <code> "InputTransformer":</code> </p> <p> <code>{</code> </p> <p> <code>"InputPathsMap": {"instance": "$.detail.instance","status": "$.detail.status"},</code> </p> <p> <code>"InputTemplate": "&lt;instance&gt; is in state \"&lt;status&gt;\""</code> </p> <p> <code>}</code> </p> <p>The <code>InputTemplate</code> can also be valid JSON with varibles in quotes or out, as in the following example:</p> <p> <code> "InputTransformer":</code> </p> <p> <code>{</code> </p> <p> <code>"InputPathsMap": {"instance": "$.detail.instance","status": "$.detail.status"},</code> </p> <p> <code>"InputTemplate": '{"myInstance": &lt;instance&gt;,"myStatus": "&lt;instance&gt; is in state \"&lt;status&gt;\""}'</code> </p> <p> <code>}</code> </p>
    #[serde(rename = "inputTemplate")]
    pub input_template: String,
}

/// <p>This object enables you to specify a JSON path to extract from the event and use as the partition key for the Amazon Kinesis data stream, so that you can control the shard to which the event goes. If you do not include this parameter, the default is to use the <code>eventId</code> as the partition key.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KinesisParameters {
    /// <p>The JSON path to be extracted from the event and used as the partition key. For more information, see <a href="https://docs.aws.amazon.com/streams/latest/dev/key-concepts.html#partition-key">Amazon Kinesis Streams Key Concepts</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p>
    #[serde(rename = "partitionKeyPath")]
    pub partition_key_path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApiDestinationsRequest {
    /// <p>The ARN of the connection specified for the API destination.</p>
    #[serde(rename = "connectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The maximum number of API destinations to include in the response.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A name prefix to filter results returned. Only API destinations with a name that starts with the prefix are returned.</p>
    #[serde(rename = "namePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListApiDestinationsResponse {
    /// <p>An array of <code>ApiDestination</code> objects that include information about an API destination.</p>
    #[serde(rename = "apiDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destinations: Option<Vec<ApiDestination>>,
    /// <p>A token you can use in a subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListArchivesRequest {
    /// <p>The ARN of the event source associated with the archive.</p>
    #[serde(rename = "eventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A name prefix to filter the archives returned. Only archives with name that match the prefix are returned.</p>
    #[serde(rename = "namePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The state of the archive.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListArchivesResponse {
    /// <p>An array of <code>Archive</code> objects that include details about an archive.</p>
    #[serde(rename = "archives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archives: Option<Vec<Archive>>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConnectionsRequest {
    /// <p>The state of the connection.</p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>The maximum number of connections to return.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A name prefix to filter results returned. Only connections with a name that starts with the prefix are returned.</p>
    #[serde(rename = "namePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConnectionsResponse {
    /// <p>An array of connections objects that include details about the connections.</p>
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    /// <p>A token you can use in a subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEventBusesRequest {
    /// <p>Specifying this limits the number of results returned by this operation. The operation also returns a NextToken which you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Specifying this limits the results to only those event buses with names that start with the specified prefix.</p>
    #[serde(rename = "namePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEventBusesResponse {
    /// <p>This list of event buses.</p>
    #[serde(rename = "eventBuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_buses: Option<Vec<EventBus>>,
    /// <p>A token you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEventSourcesRequest {
    /// <p>Specifying this limits the number of results returned by this operation. The operation also returns a NextToken which you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Specifying this limits the results to only those partner event sources with names that start with the specified prefix.</p>
    #[serde(rename = "namePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEventSourcesResponse {
    /// <p>The list of event sources.</p>
    #[serde(rename = "eventSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_sources: Option<Vec<EventSource>>,
    /// <p>A token you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPartnerEventSourceAccountsRequest {
    /// <p>The name of the partner event source to display account information about.</p>
    #[serde(rename = "eventSourceName")]
    pub event_source_name: String,
    /// <p>Specifying this limits the number of results returned by this operation. The operation also returns a NextToken which you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token returned by a previous call to this operation. Specifying this retrieves the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPartnerEventSourceAccountsResponse {
    /// <p>A token you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of partner event sources returned by the operation.</p>
    #[serde(rename = "partnerEventSourceAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_event_source_accounts: Option<Vec<PartnerEventSourceAccount>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPartnerEventSourcesRequest {
    /// <p>pecifying this limits the number of results returned by this operation. The operation also returns a NextToken which you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify this, the results are limited to only those partner event sources that start with the string you specify.</p>
    #[serde(rename = "namePrefix")]
    pub name_prefix: String,
    /// <p>The token returned by a previous call to this operation. Specifying this retrieves the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPartnerEventSourcesResponse {
    /// <p>A token you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of partner event sources returned by the operation.</p>
    #[serde(rename = "partnerEventSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_event_sources: Option<Vec<PartnerEventSource>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReplaysRequest {
    /// <p>The ARN of the event source associated with the replay.</p>
    #[serde(rename = "eventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>The maximum number of replays to retrieve.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A name prefix to filter the replays returned. Only replays with name that match the prefix are returned.</p>
    #[serde(rename = "namePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The state of the replay.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListReplaysResponse {
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>Replay</code> objects that contain information about the replay.</p>
    #[serde(rename = "replays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replays: Option<Vec<Replay>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRuleNamesByTargetRequest {
    /// <p>The name or ARN of the event bus to list rules for. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the target resource.</p>
    #[serde(rename = "targetArn")]
    pub target_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRuleNamesByTargetResponse {
    /// <p>Indicates whether there are additional results to retrieve. If there are no more results, the value is null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The names of the rules that can invoke the given target.</p>
    #[serde(rename = "ruleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_names: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRulesRequest {
    /// <p>The name or ARN of the event bus to list the rules for. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The prefix matching the rule name.</p>
    #[serde(rename = "namePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRulesResponse {
    /// <p>Indicates whether there are additional results to retrieve. If there are no more results, the value is null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The rules that match the specified criteria.</p>
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the EventBridge resource for which you want to view tags.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The list of tag keys and values associated with the resource you specified</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTargetsByRuleRequest {
    /// <p>The name or ARN of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "rule")]
    pub rule: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTargetsByRuleResponse {
    /// <p>Indicates whether there are additional results to retrieve. If there are no more results, the value is null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The targets assigned to the rule.</p>
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>This structure specifies the network configuration for an ECS task.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NetworkConfiguration {
    /// <p>Use this structure to specify the VPC subnets and security groups for the task, and whether a public IP address is to be used. This structure is relevant only for ECS tasks that use the <code>awsvpc</code> network mode.</p>
    #[serde(rename = "awsvpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

/// <p>A partner event source is created by an SaaS partner. If a customer creates a partner event bus that matches this event source, that AWS account can receive events from the partner's applications or services.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PartnerEventSource {
    /// <p>The ARN of the partner event source.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the partner event source.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The AWS account that a partner event source has been offered to.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PartnerEventSourceAccount {
    /// <p>The AWS account ID that the partner event source was offered to.</p>
    #[serde(rename = "account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// <p>The date and time the event source was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The date and time that the event source will expire, if the AWS account doesn't create a matching event bus for it.</p>
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    /// <p>The state of the event source. If it is ACTIVE, you have already created a matching event bus for this event source, and that event bus is active. If it is PENDING, either you haven't yet created a matching event bus, or that event bus is deactivated. If it is DELETED, you have created a matching event bus, but the event source has since been deleted.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>An object representing a constraint on task placement. To learn more, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html">Task Placement Constraints</a> in the Amazon Elastic Container Service Developer Guide.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlacementConstraint {
    /// <p>A cluster query language expression to apply to the constraint. You cannot specify an expression if the constraint type is <code>distinctInstance</code>. To learn more, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the Amazon Elastic Container Service Developer Guide. </p>
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The type of constraint. Use distinctInstance to ensure that each task in a particular group is running on a different container instance. Use memberOf to restrict the selection to a group of valid candidates. </p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The task placement strategy for a task or service. To learn more, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-strategies.html">Task Placement Strategies</a> in the Amazon Elastic Container Service Developer Guide.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlacementStrategy {
    /// <p>The field to apply the placement strategy against. For the spread placement strategy, valid values are instanceId (or host, which has the same effect), or any platform or custom attribute that is applied to a container instance, such as attribute:ecs.availability-zone. For the binpack placement strategy, valid values are cpu and memory. For the random placement strategy, this field is not used. </p>
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// <p>The type of placement strategy. The random placement strategy randomly places tasks on available candidates. The spread placement strategy spreads placement across available candidates evenly based on the field parameter. The binpack strategy places tasks on available candidates that have the least available amount of the resource that is specified with the field parameter. For example, if you binpack on memory, a task is placed on the instance with the least amount of remaining memory (but still enough to run the task). </p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEventsRequest {
    /// <p>The entry that defines an event in your system. You can specify several parameters for the entry such as the source and type of the event, resources associated with the event, and so on.</p>
    #[serde(rename = "entries")]
    pub entries: Vec<PutEventsRequestEntry>,
}

/// <p>Represents an event to be submitted.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEventsRequestEntry {
    /// <p>A valid JSON string. There is no other schema imposed. The JSON string may contain fields and nested subobjects.</p>
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// <p>Free-form string used to decide what fields to expect in the event detail.</p>
    #[serde(rename = "detailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    /// <p>The name or ARN of the event bus to receive the event. Only the rules that are associated with this event bus are used to match the event. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>AWS resources, identified by Amazon Resource Name (ARN), which the event primarily concerns. Any number, including zero, may be present.</p>
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// <p>The source of the event.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The time stamp of the event, per <a href="https://www.rfc-editor.org/rfc/rfc3339.txt">RFC3339</a>. If no time stamp is provided, the time stamp of the <a>PutEvents</a> call is used.</p>
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    /// <p>An AWS X-Ray trade header, which is an http header (X-Amzn-Trace-Id) that contains the trace-id associated with the event.</p> <p>To learn more about X-Ray trace headers, see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-concepts.html#xray-concepts-tracingheader">Tracing header</a> in the AWS X-Ray Developer Guide.</p>
    #[serde(rename = "traceHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_header: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEventsResponse {
    /// <p>The successfully and unsuccessfully ingested events results. If the ingestion was successful, the entry has the event ID in it. Otherwise, you can use the error code and error message to identify the problem with the entry.</p>
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<PutEventsResultEntry>>,
    /// <p>The number of failed entries.</p>
    #[serde(rename = "failedEntryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i64>,
}

/// <p>Represents an event that failed to be submitted.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEventsResultEntry {
    /// <p>The error code that indicates why the event submission failed.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message that explains why the event submission failed.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the event.</p>
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutPartnerEventsRequest {
    /// <p>The list of events to write to the event bus.</p>
    #[serde(rename = "entries")]
    pub entries: Vec<PutPartnerEventsRequestEntry>,
}

/// <p>The details about an event generated by an SaaS partner.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutPartnerEventsRequestEntry {
    /// <p>A valid JSON string. There is no other schema imposed. The JSON string may contain fields and nested subobjects.</p>
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// <p>A free-form string used to decide what fields to expect in the event detail.</p>
    #[serde(rename = "detailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    /// <p>AWS resources, identified by Amazon Resource Name (ARN), which the event primarily concerns. Any number, including zero, may be present.</p>
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// <p>The event source that is generating the evntry.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The date and time of the event.</p>
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutPartnerEventsResponse {
    /// <p>The list of events from this operation that were successfully written to the partner event bus.</p>
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<PutPartnerEventsResultEntry>>,
    /// <p>The number of events from this operation that could not be written to the partner event bus.</p>
    #[serde(rename = "failedEntryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i64>,
}

/// <p>Represents an event that a partner tried to generate, but failed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutPartnerEventsResultEntry {
    /// <p>The error code that indicates why the event submission failed.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message that explains why the event submission failed.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the event.</p>
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutPermissionRequest {
    /// <p>The action that you are enabling the other account to perform. Currently, this must be <code>events:PutEvents</code>.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>This parameter enables you to limit the permission to accounts that fulfill a certain condition, such as being a member of a certain AWS organization. For more information about AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_introduction.html">What Is AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p> <p>If you specify <code>Condition</code> with an AWS organization ID, and specify "*" as the value for <code>Principal</code>, you grant permission to all the accounts in the named organization.</p> <p>The <code>Condition</code> is a JSON string which must contain <code>Type</code>, <code>Key</code>, and <code>Value</code> fields.</p>
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    /// <p>The name of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>A JSON string that describes the permission policy statement. You can include a <code>Policy</code> parameter in the request instead of using the <code>StatementId</code>, <code>Action</code>, <code>Principal</code>, or <code>Condition</code> parameters.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The 12-digit AWS account ID that you are permitting to put events to your default event bus. Specify "*" to permit any account to put events to your default event bus.</p> <p>If you specify "*" without specifying <code>Condition</code>, avoid creating rules that may match undesirable events. To create more secure rules, make sure that the event pattern for each rule contains an <code>account</code> field with a specific account ID from which to receive events. Rules with an account field do not match any events sent from other accounts.</p>
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    /// <p>An identifier string for the external account that you are granting permissions to. If you later want to revoke the permission for this external account, specify this <code>StatementId</code> when you run <a>RemovePermission</a>.</p>
    #[serde(rename = "statementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRuleRequest {
    /// <p>A description of the rule.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name or ARN of the event bus to associate with this rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html">Events and Event Patterns</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    #[serde(rename = "eventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    /// <p>The name of the rule that you are creating or updating.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The scheduling expression. For example, "cron(0 20 * * ? *)" or "rate(5 minutes)".</p>
    #[serde(rename = "scheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>Indicates whether the rule is enabled or disabled.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The list of key-value pairs to associate with the rule.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRuleResponse {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[serde(rename = "ruleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutTargetsRequest {
    /// <p>The name or ARN of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "rule")]
    pub rule: String,
    /// <p>The targets to update or add to the rule.</p>
    #[serde(rename = "targets")]
    pub targets: Vec<Target>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutTargetsResponse {
    /// <p>The failed target entries.</p>
    #[serde(rename = "failedEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries: Option<Vec<PutTargetsResultEntry>>,
    /// <p>The number of failed entries.</p>
    #[serde(rename = "failedEntryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i64>,
}

/// <p>Represents a target that failed to be added to a rule.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutTargetsResultEntry {
    /// <p>The error code that indicates why the target addition failed. If the value is <code>ConcurrentModificationException</code>, too many requests were made at the same time.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message that explains why the target addition failed.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the target.</p>
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

/// <p>These are custom parameters to be used when the target is a Redshift cluster to invoke the Redshift Data API ExecuteStatement based on EventBridge events.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RedshiftDataParameters {
    /// <p>The name of the database. Required when authenticating using temporary credentials.</p>
    #[serde(rename = "database")]
    pub database: String,
    /// <p>The database user name. Required when authenticating using temporary credentials.</p>
    #[serde(rename = "dbUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    /// <p>The name or ARN of the secret that enables access to the database. Required when authenticating using AWS Secrets Manager.</p>
    #[serde(rename = "secretManagerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_manager_arn: Option<String>,
    /// <p>The SQL statement text to run.</p>
    #[serde(rename = "sql")]
    pub sql: String,
    /// <p>The name of the SQL statement. You can name the SQL statement when you create it to identify the query.</p>
    #[serde(rename = "statementName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_name: Option<String>,
    /// <p>Indicates whether to send an event back to EventBridge after the SQL statement runs.</p>
    #[serde(rename = "withEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_event: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemovePermissionRequest {
    /// <p>The name of the event bus to revoke permissions for. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>Specifies whether to remove all permissions.</p>
    #[serde(rename = "removeAllPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_all_permissions: Option<bool>,
    /// <p>The statement ID corresponding to the account that is no longer allowed to put events to the default event bus.</p>
    #[serde(rename = "statementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTargetsRequest {
    /// <p>The name or ARN of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>If this is a managed rule, created by an AWS service on your behalf, you must specify <code>Force</code> as <code>True</code> to remove targets. This parameter is ignored for rules that are not managed rules. You can check whether a rule is a managed rule by using <code>DescribeRule</code> or <code>ListRules</code> and checking the <code>ManagedBy</code> field of the response.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The IDs of the targets to remove from the rule.</p>
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "rule")]
    pub rule: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveTargetsResponse {
    /// <p>The failed target entries.</p>
    #[serde(rename = "failedEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries: Option<Vec<RemoveTargetsResultEntry>>,
    /// <p>The number of failed entries.</p>
    #[serde(rename = "failedEntryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i64>,
}

/// <p>Represents a target that failed to be removed from a rule.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveTargetsResultEntry {
    /// <p>The error code that indicates why the target removal failed. If the value is <code>ConcurrentModificationException</code>, too many requests were made at the same time.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message that explains why the target removal failed.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the target.</p>
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

/// <p>A <code>Replay</code> object that contains details about a replay.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Replay {
    /// <p>A time stamp for the time to start replaying events. Any event with a creation time prior to the <code>EventEndTime</code> specified is replayed.</p>
    #[serde(rename = "eventEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_end_time: Option<f64>,
    /// <p>A time stamp for the time that the last event was replayed.</p>
    #[serde(rename = "eventLastReplayedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_last_replayed_time: Option<f64>,
    /// <p>The ARN of the archive to replay event from.</p>
    #[serde(rename = "eventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>A time stamp for the time to start replaying events. This is determined by the time in the event as described in <a href="https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_PutEventsRequestEntry.html#eventbridge-Type-PutEventsRequestEntry-Time">Time</a>.</p>
    #[serde(rename = "eventStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<f64>,
    /// <p>A time stamp for the time that the replay completed.</p>
    #[serde(rename = "replayEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_end_time: Option<f64>,
    /// <p>The name of the replay.</p>
    #[serde(rename = "replayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_name: Option<String>,
    /// <p>A time stamp for the time that the replay started.</p>
    #[serde(rename = "replayStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_start_time: Option<f64>,
    /// <p>The current state of the replay.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of why the replay is in the current state.</p>
    #[serde(rename = "stateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

/// <p>A <code>ReplayDestination</code> object that contains details about a replay.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ReplayDestination {
    /// <p>The ARN of the event bus to replay event to. You can replay events only to the event bus specified to create the archive.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>A list of ARNs for rules to replay events to.</p>
    #[serde(rename = "filterArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arns: Option<Vec<String>>,
}

/// <p>A <code>RetryPolicy</code> object that includes information about the retry policy settings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RetryPolicy {
    /// <p>The maximum amount of time, in seconds, to continue to make retry attempts.</p>
    #[serde(rename = "maximumEventAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<i64>,
    /// <p>The maximum number of retry attempts to make before the request fails. Retry attempts continue until either the maximum number of attempts is made or until the duration of the <code>MaximumEventAgeInSeconds</code> is met.</p>
    #[serde(rename = "maximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i64>,
}

/// <p>Contains information about a rule in Amazon EventBridge.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Rule {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the rule.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name or ARN of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The event pattern of the rule. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html">Events and Event Patterns</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    #[serde(rename = "eventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    /// <p>If the rule was created on behalf of your account by an AWS service, this field displays the principal name of the service that created the rule.</p>
    #[serde(rename = "managedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the role that is used for target invocation.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The scheduling expression. For example, "cron(0 20 * * ? *)", "rate(5 minutes)".</p>
    #[serde(rename = "scheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The state of the rule.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>This parameter contains the criteria (either InstanceIds or a tag) used to specify which EC2 instances are to be sent the command. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RunCommandParameters {
    /// <p>Currently, we support including only one RunCommandTarget block, which specifies either an array of InstanceIds or a tag.</p>
    #[serde(rename = "runCommandTargets")]
    pub run_command_targets: Vec<RunCommandTarget>,
}

/// <p>Information about the EC2 instances that are to be sent the command, specified as key-value pairs. Each <code>RunCommandTarget</code> block can include only one key, but this key may specify multiple values.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RunCommandTarget {
    /// <p>Can be either <code>tag:</code> <i>tag-key</i> or <code>InstanceIds</code>.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>If <code>Key</code> is <code>tag:</code> <i>tag-key</i>, <code>Values</code> is a list of tag values. If <code>Key</code> is <code>InstanceIds</code>, <code>Values</code> is a list of Amazon EC2 instance IDs.</p>
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

/// <p>Name/Value pair of a parameter to start execution of a SageMaker Model Building Pipeline.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SageMakerPipelineParameter {
    /// <p>Name of parameter to start execution of a SageMaker Model Building Pipeline.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Value of parameter to start execution of a SageMaker Model Building Pipeline.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>These are custom parameters to use when the target is a SageMaker Model Building Pipeline that starts based on EventBridge events.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SageMakerPipelineParameters {
    /// <p>List of Parameter names and values for SageMaker Model Building Pipeline execution.</p>
    #[serde(rename = "pipelineParameterList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter>>,
}

/// <p>This structure includes the custom parameter to be used when the target is an SQS FIFO queue.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SqsParameters {
    /// <p>The FIFO message group ID to use as the target.</p>
    #[serde(rename = "messageGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartReplayRequest {
    /// <p>A description for the replay to start.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A <code>ReplayDestination</code> object that includes details about the destination for the replay.</p>
    #[serde(rename = "destination")]
    pub destination: ReplayDestination,
    /// <p>A time stamp for the time to stop replaying events. Only events that occurred between the <code>EventStartTime</code> and <code>EventEndTime</code> are replayed.</p>
    #[serde(rename = "eventEndTime")]
    pub event_end_time: f64,
    /// <p>The ARN of the archive to replay events from.</p>
    #[serde(rename = "eventSourceArn")]
    pub event_source_arn: String,
    /// <p>A time stamp for the time to start replaying events. Only events that occurred between the <code>EventStartTime</code> and <code>EventEndTime</code> are replayed.</p>
    #[serde(rename = "eventStartTime")]
    pub event_start_time: f64,
    /// <p>The name of the replay to start.</p>
    #[serde(rename = "replayName")]
    pub replay_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartReplayResponse {
    /// <p>The ARN of the replay.</p>
    #[serde(rename = "replayArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_arn: Option<String>,
    /// <p>The time at which the replay started.</p>
    #[serde(rename = "replayStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_start_time: Option<f64>,
    /// <p>The state of the replay.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason that the replay is in the state.</p>
    #[serde(rename = "stateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

/// <p>A key-value pair associated with an AWS resource. In EventBridge, rules and event buses support tagging.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>A string you can use to assign a value. The combination of tag keys and values can help you organize and categorize your resources.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value for the specified tag key.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the EventBridge resource that you're adding tags to.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The list of key-value pairs to associate with the resource.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Targets are the resources to be invoked when a rule is triggered. For a complete list of services and resources that can be set as a target, see <a>PutTargets</a>.</p> <p>If you are setting the event bus of another account as the target, and that account granted permission to your account through an organization instead of directly by the account ID, then you must specify a <code>RoleArn</code> with proper permissions in the <code>Target</code> structure. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-cross-account-event-delivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon EventBridge User Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Target {
    /// <p>The Amazon Resource Name (ARN) of the target.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>If the event target is an AWS Batch job, this contains the job definition, job name, and other parameters. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/jobs.html">Jobs</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "batchParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_parameters: Option<BatchParameters>,
    /// <p>The <code>DeadLetterConfig</code> that defines the target queue to send dead-letter queue events to.</p>
    #[serde(rename = "deadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    /// <p>Contains the Amazon ECS task definition and task count to be used, if the event target is an Amazon ECS task. For more information about Amazon ECS tasks, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html">Task Definitions </a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>
    #[serde(rename = "ecsParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_parameters: Option<EcsParameters>,
    /// <p>Contains the HTTP parameters to use when the target is a API Gateway REST endpoint or EventBridge ApiDestination.</p> <p>If you specify an API Gateway REST API or EventBridge ApiDestination as a target, you can use this parameter to specify headers, path parameters, and query string keys/values as part of your target invoking request. If you're using ApiDestinations, the corresponding Connection can also have these values configured. In case of any conflicting keys, values from the Connection take precedence.</p>
    #[serde(rename = "httpParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_parameters: Option<HttpParameters>,
    /// <p>The ID of the target.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>Valid JSON text passed to the target. In this case, nothing from the event itself is passed to the target. For more information, see <a href="http://www.rfc-editor.org/rfc/rfc7159.txt">The JavaScript Object Notation (JSON) Data Interchange Format</a>.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The value of the JSONPath that is used for extracting part of the matched event when passing it to the target. You must use JSON dot notation, not bracket notation. For more information about JSON paths, see <a href="http://goessner.net/articles/JsonPath/">JSONPath</a>.</p>
    #[serde(rename = "inputPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_path: Option<String>,
    /// <p>Settings to enable you to provide custom input to a target based on certain event data. You can extract one or more key-value pairs from the event and then use that data to send customized input to the target.</p>
    #[serde(rename = "inputTransformer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_transformer: Option<InputTransformer>,
    /// <p>The custom parameter you can use to control the shard assignment, when the target is a Kinesis data stream. If you do not include this parameter, the default is to use the <code>eventId</code> as the partition key.</p>
    #[serde(rename = "kinesisParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_parameters: Option<KinesisParameters>,
    /// <p>Contains the Redshift Data API parameters to use when the target is a Redshift cluster.</p> <p>If you specify a Redshift Cluster as a Target, you can use this to specify parameters to invoke the Redshift Data API ExecuteStatement based on EventBridge events.</p>
    #[serde(rename = "redshiftDataParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_data_parameters: Option<RedshiftDataParameters>,
    /// <p>The <code>RetryPolicy</code> object that contains the retry policy configuration to use for the dead-letter queue.</p>
    #[serde(rename = "retryPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be used for this target when the rule is triggered. If one rule triggers multiple targets, you can use a different IAM role for each target.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Parameters used when you are using the rule to invoke Amazon EC2 Run Command.</p>
    #[serde(rename = "runCommandParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_command_parameters: Option<RunCommandParameters>,
    /// <p>Contains the SageMaker Model Building Pipeline parameters to start execution of a SageMaker Model Building Pipeline.</p> <p>If you specify a SageMaker Model Building Pipeline as a target, you can use this to specify parameters to start a pipeline execution based on EventBridge events.</p>
    #[serde(rename = "sageMakerPipelineParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_pipeline_parameters: Option<SageMakerPipelineParameters>,
    /// <p>Contains the message group ID to use when the target is a FIFO queue.</p> <p>If you specify an SQS FIFO queue as a target, the queue must have content-based deduplication enabled.</p>
    #[serde(rename = "sqsParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_parameters: Option<SqsParameters>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TestEventPatternRequest {
    /// <p><p>The event, in JSON format, to test against the event pattern. The JSON must follow the format specified in <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/aws-events.html">AWS Events</a>, and the following fields are mandatory:</p> <ul> <li> <p> <code>id</code> </p> </li> <li> <p> <code>account</code> </p> </li> <li> <p> <code>source</code> </p> </li> <li> <p> <code>time</code> </p> </li> <li> <p> <code>region</code> </p> </li> <li> <p> <code>resources</code> </p> </li> <li> <p> <code>detail-type</code> </p> </li> </ul></p>
    #[serde(rename = "event")]
    pub event: String,
    /// <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html">Events and Event Patterns</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    #[serde(rename = "eventPattern")]
    pub event_pattern: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestEventPatternResponse {
    /// <p>Indicates whether the event matches the event pattern.</p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the EventBridge resource from which you are removing tags.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The list of tag keys to remove from the resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApiDestinationRequest {
    /// <p>The ARN of the connection to use for the API destination.</p>
    #[serde(rename = "connectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The name of the API destination to update.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The method to use for the API destination.</p>
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p>The URL to the endpoint to use for the API destination.</p>
    #[serde(rename = "invocationEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_endpoint: Option<String>,
    /// <p>The maximum number of invocations per second to send to the API destination.</p>
    #[serde(rename = "invocationRateLimitPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_rate_limit_per_second: Option<i64>,
    /// <p>The name of the API destination to update.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApiDestinationResponse {
    /// <p>The ARN of the API destination that was updated.</p>
    #[serde(rename = "apiDestinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_arn: Option<String>,
    /// <p>The state of the API destination that was updated.</p>
    #[serde(rename = "apiDestinationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_state: Option<String>,
    /// <p>A time stamp for the time that the API destination was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A time stamp for the time that the API destination was last modified.</p>
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateArchiveRequest {
    /// <p>The name of the archive to update.</p>
    #[serde(rename = "archiveName")]
    pub archive_name: String,
    /// <p>The description for the archive.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The event pattern to use to filter events sent to the archive.</p>
    #[serde(rename = "eventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    /// <p>The number of days to retain events in the archive.</p>
    #[serde(rename = "retentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateArchiveResponse {
    /// <p>The ARN of the archive.</p>
    #[serde(rename = "archiveArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_arn: Option<String>,
    /// <p>The time at which the archive was updated.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The state of the archive.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason that the archive is in the current state.</p>
    #[serde(rename = "stateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

/// <p>Contains the API key authorization parameters to use to update the connection.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConnectionApiKeyAuthRequestParameters {
    /// <p>The name of the API key to use for authorization.</p>
    #[serde(rename = "apiKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_name: Option<String>,
    /// <p>The value associated with teh API key to use for authorization.</p>
    #[serde(rename = "apiKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_value: Option<String>,
}

/// <p>Contains the additional parameters to use for the connection.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConnectionAuthRequestParameters {
    /// <p>A <code>UpdateConnectionApiKeyAuthRequestParameters</code> object that contains the authorization parameters for API key authorization.</p>
    #[serde(rename = "apiKeyAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_auth_parameters: Option<UpdateConnectionApiKeyAuthRequestParameters>,
    /// <p>A <code>UpdateConnectionBasicAuthRequestParameters</code> object that contains the authorization parameters for Basic authorization.</p>
    #[serde(rename = "basicAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_parameters: Option<UpdateConnectionBasicAuthRequestParameters>,
    /// <p>A <code>ConnectionHttpParameters</code> object that contains the additional parameters to use for the connection.</p>
    #[serde(rename = "invocationHttpParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_http_parameters: Option<ConnectionHttpParameters>,
    /// <p>A <code>UpdateConnectionOAuthRequestParameters</code> object that contains the authorization parameters for OAuth authorization.</p>
    #[serde(rename = "oAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_parameters: Option<UpdateConnectionOAuthRequestParameters>,
}

/// <p>Contains the Basic authorization parameters for the connection.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConnectionBasicAuthRequestParameters {
    /// <p>The password associated with the user name to use for Basic authorization.</p>
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The user name to use for Basic authorization.</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Contains the OAuth authorization parameters to use for the connection.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConnectionOAuthClientRequestParameters {
    /// <p>The client ID to use for OAuth authorization.</p>
    #[serde(rename = "clientID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The client secret assciated with the client ID to use for OAuth authorization.</p>
    #[serde(rename = "clientSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

/// <p>Contains the OAuth request parameters to use for the connection.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConnectionOAuthRequestParameters {
    /// <p>The URL to the authorization endpoint when OAuth is specified as the authorization type.</p>
    #[serde(rename = "authorizationEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    /// <p>A <code>UpdateConnectionOAuthClientRequestParameters</code> object that contains the client parameters to use for the connection when OAuth is specified as the authorization type.</p>
    #[serde(rename = "clientParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_parameters: Option<UpdateConnectionOAuthClientRequestParameters>,
    /// <p>The method used to connect to the HTTP endpoint.</p>
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p>The additional HTTP parameters used for the OAuth authorization request.</p>
    #[serde(rename = "oAuthHttpParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_http_parameters: Option<ConnectionHttpParameters>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConnectionRequest {
    /// <p>The authorization parameters to use for the connection.</p>
    #[serde(rename = "authParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<UpdateConnectionAuthRequestParameters>,
    /// <p>The type of authorization to use for the connection.</p>
    #[serde(rename = "authorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>A description for the connection.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the connection to update.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConnectionResponse {
    /// <p>The ARN of the connection that was updated.</p>
    #[serde(rename = "connectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The state of the connection that was updated.</p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>A time stamp for the time that the connection was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A time stamp for the time that the connection was last authorized.</p>
    #[serde(rename = "lastAuthorizedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authorized_time: Option<f64>,
    /// <p>A time stamp for the time that the connection was last modified.</p>
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

/// Errors returned by ActivateEventSource
#[derive(Debug, PartialEq)]
pub enum ActivateEventSourceError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The specified state is not a valid state for an event source.</p>
    InvalidState(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl ActivateEventSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ActivateEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(ActivateEventSourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(ActivateEventSourceError::Internal(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(ActivateEventSourceError::InvalidState(err.msg))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(ActivateEventSourceError::OperationDisabled(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ActivateEventSourceError::ResourceNotFound(
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
impl fmt::Display for ActivateEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ActivateEventSourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            ActivateEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
            ActivateEventSourceError::InvalidState(ref cause) => write!(f, "{}", cause),
            ActivateEventSourceError::OperationDisabled(ref cause) => write!(f, "{}", cause),
            ActivateEventSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ActivateEventSourceError {}
/// Errors returned by CancelReplay
#[derive(Debug, PartialEq)]
pub enum CancelReplayError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>An error occurred because a replay can be canceled only when the state is Running or Starting.</p>
    IllegalStatus(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl CancelReplayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelReplayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CancelReplayError::ConcurrentModification(err.msg))
                }
                "IllegalStatusException" => {
                    return RusotoError::Service(CancelReplayError::IllegalStatus(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(CancelReplayError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelReplayError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelReplayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelReplayError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CancelReplayError::IllegalStatus(ref cause) => write!(f, "{}", cause),
            CancelReplayError::Internal(ref cause) => write!(f, "{}", cause),
            CancelReplayError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelReplayError {}
/// Errors returned by CreateApiDestination
#[derive(Debug, PartialEq)]
pub enum CreateApiDestinationError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The request failed because it attempted to create resource beyond the allowed service quota.</p>
    LimitExceeded(String),
    /// <p>The resource you are trying to create already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl CreateApiDestinationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApiDestinationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateApiDestinationError::Internal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateApiDestinationError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateApiDestinationError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateApiDestinationError::ResourceNotFound(
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
impl fmt::Display for CreateApiDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApiDestinationError::Internal(ref cause) => write!(f, "{}", cause),
            CreateApiDestinationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateApiDestinationError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateApiDestinationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateApiDestinationError {}
/// Errors returned by CreateArchive
#[derive(Debug, PartialEq)]
pub enum CreateArchiveError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The event pattern is not valid.</p>
    InvalidEventPattern(String),
    /// <p>The request failed because it attempted to create resource beyond the allowed service quota.</p>
    LimitExceeded(String),
    /// <p>The resource you are trying to create already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl CreateArchiveError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateArchiveError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateArchiveError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(CreateArchiveError::Internal(err.msg))
                }
                "InvalidEventPatternException" => {
                    return RusotoError::Service(CreateArchiveError::InvalidEventPattern(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateArchiveError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateArchiveError::ResourceAlreadyExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateArchiveError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateArchiveError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateArchiveError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateArchiveError::Internal(ref cause) => write!(f, "{}", cause),
            CreateArchiveError::InvalidEventPattern(ref cause) => write!(f, "{}", cause),
            CreateArchiveError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateArchiveError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateArchiveError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateArchiveError {}
/// Errors returned by CreateConnection
#[derive(Debug, PartialEq)]
pub enum CreateConnectionError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The request failed because it attempted to create resource beyond the allowed service quota.</p>
    LimitExceeded(String),
    /// <p>The resource you are trying to create already exists.</p>
    ResourceAlreadyExists(String),
}

impl CreateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateConnectionError::Internal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateConnectionError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateConnectionError::ResourceAlreadyExists(
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
impl fmt::Display for CreateConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConnectionError::Internal(ref cause) => write!(f, "{}", cause),
            CreateConnectionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateConnectionError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConnectionError {}
/// Errors returned by CreateEventBus
#[derive(Debug, PartialEq)]
pub enum CreateEventBusError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The specified state is not a valid state for an event source.</p>
    InvalidState(String),
    /// <p>The request failed because it attempted to create resource beyond the allowed service quota.</p>
    LimitExceeded(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
    /// <p>The resource you are trying to create already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl CreateEventBusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEventBusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateEventBusError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(CreateEventBusError::Internal(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(CreateEventBusError::InvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateEventBusError::LimitExceeded(err.msg))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(CreateEventBusError::OperationDisabled(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateEventBusError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateEventBusError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEventBusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEventBusError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateEventBusError::Internal(ref cause) => write!(f, "{}", cause),
            CreateEventBusError::InvalidState(ref cause) => write!(f, "{}", cause),
            CreateEventBusError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEventBusError::OperationDisabled(ref cause) => write!(f, "{}", cause),
            CreateEventBusError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateEventBusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEventBusError {}
/// Errors returned by CreatePartnerEventSource
#[derive(Debug, PartialEq)]
pub enum CreatePartnerEventSourceError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The request failed because it attempted to create resource beyond the allowed service quota.</p>
    LimitExceeded(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
    /// <p>The resource you are trying to create already exists.</p>
    ResourceAlreadyExists(String),
}

impl CreatePartnerEventSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePartnerEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreatePartnerEventSourceError::ConcurrentModification(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(CreatePartnerEventSourceError::Internal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreatePartnerEventSourceError::LimitExceeded(
                        err.msg,
                    ))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(CreatePartnerEventSourceError::OperationDisabled(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreatePartnerEventSourceError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePartnerEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePartnerEventSourceError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePartnerEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
            CreatePartnerEventSourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreatePartnerEventSourceError::OperationDisabled(ref cause) => write!(f, "{}", cause),
            CreatePartnerEventSourceError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreatePartnerEventSourceError {}
/// Errors returned by DeactivateEventSource
#[derive(Debug, PartialEq)]
pub enum DeactivateEventSourceError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The specified state is not a valid state for an event source.</p>
    InvalidState(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DeactivateEventSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeactivateEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeactivateEventSourceError::ConcurrentModification(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(DeactivateEventSourceError::Internal(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(DeactivateEventSourceError::InvalidState(err.msg))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(DeactivateEventSourceError::OperationDisabled(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeactivateEventSourceError::ResourceNotFound(
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
impl fmt::Display for DeactivateEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeactivateEventSourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeactivateEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
            DeactivateEventSourceError::InvalidState(ref cause) => write!(f, "{}", cause),
            DeactivateEventSourceError::OperationDisabled(ref cause) => write!(f, "{}", cause),
            DeactivateEventSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeactivateEventSourceError {}
/// Errors returned by DeauthorizeConnection
#[derive(Debug, PartialEq)]
pub enum DeauthorizeConnectionError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DeauthorizeConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeauthorizeConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeauthorizeConnectionError::ConcurrentModification(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(DeauthorizeConnectionError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeauthorizeConnectionError::ResourceNotFound(
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
impl fmt::Display for DeauthorizeConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeauthorizeConnectionError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeauthorizeConnectionError::Internal(ref cause) => write!(f, "{}", cause),
            DeauthorizeConnectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeauthorizeConnectionError {}
/// Errors returned by DeleteApiDestination
#[derive(Debug, PartialEq)]
pub enum DeleteApiDestinationError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DeleteApiDestinationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApiDestinationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteApiDestinationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(DeleteApiDestinationError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteApiDestinationError::ResourceNotFound(
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
impl fmt::Display for DeleteApiDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApiDestinationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteApiDestinationError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteApiDestinationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApiDestinationError {}
/// Errors returned by DeleteArchive
#[derive(Debug, PartialEq)]
pub enum DeleteArchiveError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DeleteArchiveError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteArchiveError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteArchiveError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(DeleteArchiveError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteArchiveError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteArchiveError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteArchiveError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteArchiveError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteArchiveError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteArchiveError {}
/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteConnectionError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(DeleteConnectionError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteConnectionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConnectionError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteConnectionError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteConnectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConnectionError {}
/// Errors returned by DeleteEventBus
#[derive(Debug, PartialEq)]
pub enum DeleteEventBusError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl DeleteEventBusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventBusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteEventBusError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(DeleteEventBusError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEventBusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEventBusError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteEventBusError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEventBusError {}
/// Errors returned by DeletePartnerEventSource
#[derive(Debug, PartialEq)]
pub enum DeletePartnerEventSourceError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
}

impl DeletePartnerEventSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePartnerEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeletePartnerEventSourceError::ConcurrentModification(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(DeletePartnerEventSourceError::Internal(err.msg))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(DeletePartnerEventSourceError::OperationDisabled(
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
impl fmt::Display for DeletePartnerEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePartnerEventSourceError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePartnerEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
            DeletePartnerEventSourceError::OperationDisabled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePartnerEventSourceError {}
/// Errors returned by DeleteRule
#[derive(Debug, PartialEq)]
pub enum DeleteRuleError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DeleteRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteRuleError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(DeleteRuleError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(DeleteRuleError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRuleError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::ManagedRule(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRuleError {}
/// Errors returned by DescribeApiDestination
#[derive(Debug, PartialEq)]
pub enum DescribeApiDestinationError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeApiDestinationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeApiDestinationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeApiDestinationError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeApiDestinationError::ResourceNotFound(
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
impl fmt::Display for DescribeApiDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeApiDestinationError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeApiDestinationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeApiDestinationError {}
/// Errors returned by DescribeArchive
#[derive(Debug, PartialEq)]
pub enum DescribeArchiveError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The resource you are trying to create already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeArchiveError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeArchiveError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeArchiveError::Internal(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(DescribeArchiveError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeArchiveError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeArchiveError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeArchiveError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeArchiveError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            DescribeArchiveError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeArchiveError {}
/// Errors returned by DescribeConnection
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeConnectionError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeConnectionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConnectionError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeConnectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeConnectionError {}
/// Errors returned by DescribeEventBus
#[derive(Debug, PartialEq)]
pub enum DescribeEventBusError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeEventBusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventBusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeEventBusError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeEventBusError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEventBusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventBusError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeEventBusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEventBusError {}
/// Errors returned by DescribeEventSource
#[derive(Debug, PartialEq)]
pub enum DescribeEventSourceError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeEventSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeEventSourceError::Internal(err.msg))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(DescribeEventSourceError::OperationDisabled(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeEventSourceError::ResourceNotFound(
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
impl fmt::Display for DescribeEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeEventSourceError::OperationDisabled(ref cause) => write!(f, "{}", cause),
            DescribeEventSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEventSourceError {}
/// Errors returned by DescribePartnerEventSource
#[derive(Debug, PartialEq)]
pub enum DescribePartnerEventSourceError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DescribePartnerEventSourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribePartnerEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribePartnerEventSourceError::Internal(err.msg))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(
                        DescribePartnerEventSourceError::OperationDisabled(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePartnerEventSourceError::ResourceNotFound(
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
impl fmt::Display for DescribePartnerEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePartnerEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
            DescribePartnerEventSourceError::OperationDisabled(ref cause) => write!(f, "{}", cause),
            DescribePartnerEventSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePartnerEventSourceError {}
/// Errors returned by DescribeReplay
#[derive(Debug, PartialEq)]
pub enum DescribeReplayError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeReplayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReplayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeReplayError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeReplayError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReplayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReplayError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeReplayError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeReplayError {}
/// Errors returned by DescribeRule
#[derive(Debug, PartialEq)]
pub enum DescribeRuleError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeRuleError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRuleError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRuleError {}
/// Errors returned by DisableRule
#[derive(Debug, PartialEq)]
pub enum DisableRuleError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl DisableRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DisableRuleError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(DisableRuleError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(DisableRuleError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisableRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableRuleError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DisableRuleError::Internal(ref cause) => write!(f, "{}", cause),
            DisableRuleError::ManagedRule(ref cause) => write!(f, "{}", cause),
            DisableRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableRuleError {}
/// Errors returned by EnableRule
#[derive(Debug, PartialEq)]
pub enum EnableRuleError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl EnableRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(EnableRuleError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(EnableRuleError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(EnableRuleError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(EnableRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableRuleError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            EnableRuleError::Internal(ref cause) => write!(f, "{}", cause),
            EnableRuleError::ManagedRule(ref cause) => write!(f, "{}", cause),
            EnableRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableRuleError {}
/// Errors returned by ListApiDestinations
#[derive(Debug, PartialEq)]
pub enum ListApiDestinationsError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl ListApiDestinationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApiDestinationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListApiDestinationsError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListApiDestinationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApiDestinationsError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApiDestinationsError {}
/// Errors returned by ListArchives
#[derive(Debug, PartialEq)]
pub enum ListArchivesError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl ListArchivesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListArchivesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListArchivesError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListArchivesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListArchivesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListArchivesError::Internal(ref cause) => write!(f, "{}", cause),
            ListArchivesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListArchivesError {}
/// Errors returned by ListConnections
#[derive(Debug, PartialEq)]
pub enum ListConnectionsError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl ListConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConnectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListConnectionsError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListConnectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListConnectionsError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListConnectionsError {}
/// Errors returned by ListEventBuses
#[derive(Debug, PartialEq)]
pub enum ListEventBusesError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl ListEventBusesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEventBusesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListEventBusesError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEventBusesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEventBusesError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEventBusesError {}
/// Errors returned by ListEventSources
#[derive(Debug, PartialEq)]
pub enum ListEventSourcesError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
}

impl ListEventSourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEventSourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListEventSourcesError::Internal(err.msg))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(ListEventSourcesError::OperationDisabled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEventSourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEventSourcesError::Internal(ref cause) => write!(f, "{}", cause),
            ListEventSourcesError::OperationDisabled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEventSourcesError {}
/// Errors returned by ListPartnerEventSourceAccounts
#[derive(Debug, PartialEq)]
pub enum ListPartnerEventSourceAccountsError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl ListPartnerEventSourceAccountsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListPartnerEventSourceAccountsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListPartnerEventSourceAccountsError::Internal(
                        err.msg,
                    ))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(
                        ListPartnerEventSourceAccountsError::OperationDisabled(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListPartnerEventSourceAccountsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPartnerEventSourceAccountsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPartnerEventSourceAccountsError::Internal(ref cause) => write!(f, "{}", cause),
            ListPartnerEventSourceAccountsError::OperationDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            ListPartnerEventSourceAccountsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListPartnerEventSourceAccountsError {}
/// Errors returned by ListPartnerEventSources
#[derive(Debug, PartialEq)]
pub enum ListPartnerEventSourcesError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
}

impl ListPartnerEventSourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPartnerEventSourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListPartnerEventSourcesError::Internal(err.msg))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(ListPartnerEventSourcesError::OperationDisabled(
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
impl fmt::Display for ListPartnerEventSourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPartnerEventSourcesError::Internal(ref cause) => write!(f, "{}", cause),
            ListPartnerEventSourcesError::OperationDisabled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPartnerEventSourcesError {}
/// Errors returned by ListReplays
#[derive(Debug, PartialEq)]
pub enum ListReplaysError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl ListReplaysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListReplaysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListReplaysError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListReplaysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListReplaysError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListReplaysError {}
/// Errors returned by ListRuleNamesByTarget
#[derive(Debug, PartialEq)]
pub enum ListRuleNamesByTargetError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl ListRuleNamesByTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRuleNamesByTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListRuleNamesByTargetError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRuleNamesByTargetError::ResourceNotFound(
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
impl fmt::Display for ListRuleNamesByTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRuleNamesByTargetError::Internal(ref cause) => write!(f, "{}", cause),
            ListRuleNamesByTargetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRuleNamesByTargetError {}
/// Errors returned by ListRules
#[derive(Debug, PartialEq)]
pub enum ListRulesError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl ListRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListRulesError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRulesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRulesError::Internal(ref cause) => write!(f, "{}", cause),
            ListRulesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRulesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListTagsForResourceError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::Internal(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTargetsByRule
#[derive(Debug, PartialEq)]
pub enum ListTargetsByRuleError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl ListTargetsByRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTargetsByRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListTargetsByRuleError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTargetsByRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTargetsByRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTargetsByRuleError::Internal(ref cause) => write!(f, "{}", cause),
            ListTargetsByRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTargetsByRuleError {}
/// Errors returned by PutEvents
#[derive(Debug, PartialEq)]
pub enum PutEventsError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl PutEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(PutEventsError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEventsError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEventsError {}
/// Errors returned by PutPartnerEvents
#[derive(Debug, PartialEq)]
pub enum PutPartnerEventsError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
}

impl PutPartnerEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPartnerEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(PutPartnerEventsError::Internal(err.msg))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(PutPartnerEventsError::OperationDisabled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutPartnerEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutPartnerEventsError::Internal(ref cause) => write!(f, "{}", cause),
            PutPartnerEventsError::OperationDisabled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutPartnerEventsError {}
/// Errors returned by PutPermission
#[derive(Debug, PartialEq)]
pub enum PutPermissionError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
    /// <p>The event bus policy is too long. For more information, see the limits.</p>
    PolicyLengthExceeded(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl PutPermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPermissionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(PutPermissionError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(PutPermissionError::Internal(err.msg))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(PutPermissionError::OperationDisabled(err.msg))
                }
                "PolicyLengthExceededException" => {
                    return RusotoError::Service(PutPermissionError::PolicyLengthExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutPermissionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutPermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutPermissionError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            PutPermissionError::Internal(ref cause) => write!(f, "{}", cause),
            PutPermissionError::OperationDisabled(ref cause) => write!(f, "{}", cause),
            PutPermissionError::PolicyLengthExceeded(ref cause) => write!(f, "{}", cause),
            PutPermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutPermissionError {}
/// Errors returned by PutRule
#[derive(Debug, PartialEq)]
pub enum PutRuleError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The event pattern is not valid.</p>
    InvalidEventPattern(String),
    /// <p>The request failed because it attempted to create resource beyond the allowed service quota.</p>
    LimitExceeded(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl PutRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(PutRuleError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(PutRuleError::Internal(err.msg))
                }
                "InvalidEventPatternException" => {
                    return RusotoError::Service(PutRuleError::InvalidEventPattern(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutRuleError::LimitExceeded(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(PutRuleError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRuleError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            PutRuleError::Internal(ref cause) => write!(f, "{}", cause),
            PutRuleError::InvalidEventPattern(ref cause) => write!(f, "{}", cause),
            PutRuleError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutRuleError::ManagedRule(ref cause) => write!(f, "{}", cause),
            PutRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRuleError {}
/// Errors returned by PutTargets
#[derive(Debug, PartialEq)]
pub enum PutTargetsError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The request failed because it attempted to create resource beyond the allowed service quota.</p>
    LimitExceeded(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl PutTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutTargetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(PutTargetsError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(PutTargetsError::Internal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutTargetsError::LimitExceeded(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(PutTargetsError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutTargetsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutTargetsError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            PutTargetsError::Internal(ref cause) => write!(f, "{}", cause),
            PutTargetsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutTargetsError::ManagedRule(ref cause) => write!(f, "{}", cause),
            PutTargetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutTargetsError {}
/// Errors returned by RemovePermission
#[derive(Debug, PartialEq)]
pub enum RemovePermissionError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The operation you are attempting is not available in this region.</p>
    OperationDisabled(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl RemovePermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemovePermissionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(RemovePermissionError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(RemovePermissionError::Internal(err.msg))
                }
                "OperationDisabledException" => {
                    return RusotoError::Service(RemovePermissionError::OperationDisabled(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemovePermissionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemovePermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemovePermissionError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::Internal(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::OperationDisabled(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemovePermissionError {}
/// Errors returned by RemoveTargets
#[derive(Debug, PartialEq)]
pub enum RemoveTargetsError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl RemoveTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTargetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(RemoveTargetsError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(RemoveTargetsError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(RemoveTargetsError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemoveTargetsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTargetsError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            RemoveTargetsError::Internal(ref cause) => write!(f, "{}", cause),
            RemoveTargetsError::ManagedRule(ref cause) => write!(f, "{}", cause),
            RemoveTargetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTargetsError {}
/// Errors returned by StartReplay
#[derive(Debug, PartialEq)]
pub enum StartReplayError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The event pattern is not valid.</p>
    InvalidEventPattern(String),
    /// <p>The request failed because it attempted to create resource beyond the allowed service quota.</p>
    LimitExceeded(String),
    /// <p>The resource you are trying to create already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl StartReplayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartReplayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(StartReplayError::Internal(err.msg))
                }
                "InvalidEventPatternException" => {
                    return RusotoError::Service(StartReplayError::InvalidEventPattern(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartReplayError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(StartReplayError::ResourceAlreadyExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartReplayError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartReplayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartReplayError::Internal(ref cause) => write!(f, "{}", cause),
            StartReplayError::InvalidEventPattern(ref cause) => write!(f, "{}", cause),
            StartReplayError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartReplayError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            StartReplayError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartReplayError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(TagResourceError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(TagResourceError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(TagResourceError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            TagResourceError::Internal(ref cause) => write!(f, "{}", cause),
            TagResourceError::ManagedRule(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by TestEventPattern
#[derive(Debug, PartialEq)]
pub enum TestEventPatternError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The event pattern is not valid.</p>
    InvalidEventPattern(String),
}

impl TestEventPatternError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestEventPatternError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(TestEventPatternError::Internal(err.msg))
                }
                "InvalidEventPatternException" => {
                    return RusotoError::Service(TestEventPatternError::InvalidEventPattern(
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
impl fmt::Display for TestEventPatternError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestEventPatternError::Internal(ref cause) => write!(f, "{}", cause),
            TestEventPatternError::InvalidEventPattern(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TestEventPatternError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UntagResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(UntagResourceError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(UntagResourceError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Internal(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ManagedRule(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateApiDestination
#[derive(Debug, PartialEq)]
pub enum UpdateApiDestinationError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The request failed because it attempted to create resource beyond the allowed service quota.</p>
    LimitExceeded(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl UpdateApiDestinationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApiDestinationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateApiDestinationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(UpdateApiDestinationError::Internal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateApiDestinationError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateApiDestinationError::ResourceNotFound(
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
impl fmt::Display for UpdateApiDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApiDestinationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateApiDestinationError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateApiDestinationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateApiDestinationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApiDestinationError {}
/// Errors returned by UpdateArchive
#[derive(Debug, PartialEq)]
pub enum UpdateArchiveError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The event pattern is not valid.</p>
    InvalidEventPattern(String),
    /// <p>The request failed because it attempted to create resource beyond the allowed service quota.</p>
    LimitExceeded(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl UpdateArchiveError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateArchiveError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateArchiveError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(UpdateArchiveError::Internal(err.msg))
                }
                "InvalidEventPatternException" => {
                    return RusotoError::Service(UpdateArchiveError::InvalidEventPattern(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateArchiveError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateArchiveError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateArchiveError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateArchiveError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateArchiveError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateArchiveError::InvalidEventPattern(ref cause) => write!(f, "{}", cause),
            UpdateArchiveError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateArchiveError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateArchiveError {}
/// Errors returned by UpdateConnection
#[derive(Debug, PartialEq)]
pub enum UpdateConnectionError {
    /// <p>There is concurrent modification on a rule, target, archive, or replay.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The request failed because it attempted to create resource beyond the allowed service quota.</p>
    LimitExceeded(String),
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
}

impl UpdateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateConnectionError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(UpdateConnectionError::Internal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateConnectionError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateConnectionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConnectionError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateConnectionError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateConnectionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateConnectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateConnectionError {}
/// Trait representing the capabilities of the Amazon EventBridge API. Amazon EventBridge clients implement this trait.
#[async_trait]
pub trait EventBridge {
    /// <p>Activates a partner event source that has been deactivated. Once activated, your matching event bus will start receiving events from the event source.</p>
    async fn activate_event_source(
        &self,
        input: ActivateEventSourceRequest,
    ) -> Result<(), RusotoError<ActivateEventSourceError>>;

    /// <p>Cancels the specified replay.</p>
    async fn cancel_replay(
        &self,
        input: CancelReplayRequest,
    ) -> Result<CancelReplayResponse, RusotoError<CancelReplayError>>;

    /// <p>Creates an API destination, which is an HTTP invocation endpoint configured as a target for events.</p>
    async fn create_api_destination(
        &self,
        input: CreateApiDestinationRequest,
    ) -> Result<CreateApiDestinationResponse, RusotoError<CreateApiDestinationError>>;

    /// <p>Creates an archive of events with the specified settings. When you create an archive, incoming events might not immediately start being sent to the archive. Allow a short period of time for changes to take effect. If you do not specify a pattern to filter events sent to the archive, all events are sent to the archive except replayed events. Replayed events are not sent to an archive.</p>
    async fn create_archive(
        &self,
        input: CreateArchiveRequest,
    ) -> Result<CreateArchiveResponse, RusotoError<CreateArchiveError>>;

    /// <p>Creates a connection. A connection defines the authorization type and credentials to use for authorization with an API destination HTTP endpoint.</p>
    async fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> Result<CreateConnectionResponse, RusotoError<CreateConnectionError>>;

    /// <p>Creates a new event bus within your account. This can be a custom event bus which you can use to receive events from your custom applications and services, or it can be a partner event bus which can be matched to a partner event source.</p>
    async fn create_event_bus(
        &self,
        input: CreateEventBusRequest,
    ) -> Result<CreateEventBusResponse, RusotoError<CreateEventBusError>>;

    /// <p>Called by an SaaS partner to create a partner event source. This operation is not used by AWS customers.</p> <p>Each partner event source can be used by one AWS account to create a matching partner event bus in that AWS account. A SaaS partner must create one partner event source for each AWS account that wants to receive those event types. </p> <p>A partner event source creates events based on resources within the SaaS partner's service or application.</p> <p>An AWS account that creates a partner event bus that matches the partner event source can use that event bus to receive events from the partner, and then process them using AWS Events rules and targets.</p> <p>Partner event source names follow this format:</p> <p> <code> <i>partner_name</i>/<i>event_namespace</i>/<i>event_name</i> </code> </p> <p> <i>partner_name</i> is determined during partner registration and identifies the partner to AWS customers. <i>event_namespace</i> is determined by the partner and is a way for the partner to categorize their events. <i>event_name</i> is determined by the partner, and should uniquely identify an event-generating resource within the partner system. The combination of <i>event_namespace</i> and <i>event_name</i> should help AWS customers decide whether to create an event bus to receive these events.</p>
    async fn create_partner_event_source(
        &self,
        input: CreatePartnerEventSourceRequest,
    ) -> Result<CreatePartnerEventSourceResponse, RusotoError<CreatePartnerEventSourceError>>;

    /// <p>You can use this operation to temporarily stop receiving events from the specified partner event source. The matching event bus is not deleted. </p> <p>When you deactivate a partner event source, the source goes into PENDING state. If it remains in PENDING state for more than two weeks, it is deleted.</p> <p>To activate a deactivated partner event source, use <a>ActivateEventSource</a>.</p>
    async fn deactivate_event_source(
        &self,
        input: DeactivateEventSourceRequest,
    ) -> Result<(), RusotoError<DeactivateEventSourceError>>;

    /// <p>Removes all authorization parameters from the connection. This lets you remove the secret from the connection so you can reuse it without having to create a new connection.</p>
    async fn deauthorize_connection(
        &self,
        input: DeauthorizeConnectionRequest,
    ) -> Result<DeauthorizeConnectionResponse, RusotoError<DeauthorizeConnectionError>>;

    /// <p>Deletes the specified API destination.</p>
    async fn delete_api_destination(
        &self,
        input: DeleteApiDestinationRequest,
    ) -> Result<DeleteApiDestinationResponse, RusotoError<DeleteApiDestinationError>>;

    /// <p>Deletes the specified archive.</p>
    async fn delete_archive(
        &self,
        input: DeleteArchiveRequest,
    ) -> Result<DeleteArchiveResponse, RusotoError<DeleteArchiveError>>;

    /// <p>Deletes a connection.</p>
    async fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> Result<DeleteConnectionResponse, RusotoError<DeleteConnectionError>>;

    /// <p>Deletes the specified custom event bus or partner event bus. All rules associated with this event bus need to be deleted. You can't delete your account's default event bus.</p>
    async fn delete_event_bus(
        &self,
        input: DeleteEventBusRequest,
    ) -> Result<(), RusotoError<DeleteEventBusError>>;

    /// <p><p>This operation is used by SaaS partners to delete a partner event source. This operation is not used by AWS customers.</p> <p>When you delete an event source, the status of the corresponding partner event bus in the AWS customer account becomes DELETED.</p> <p/></p>
    async fn delete_partner_event_source(
        &self,
        input: DeletePartnerEventSourceRequest,
    ) -> Result<(), RusotoError<DeletePartnerEventSourceError>>;

    /// <p>Deletes the specified rule.</p> <p>Before you can delete the rule, you must remove all targets, using <a>RemoveTargets</a>.</p> <p>When you delete a rule, incoming events might continue to match to the deleted rule. Allow a short period of time for changes to take effect.</p> <p>If you call delete rule multiple times for the same rule, all calls will succeed. When you call delete rule for a non-existent custom eventbus, <code>ResourceNotFoundException</code> is returned.</p> <p>Managed rules are rules created and managed by another AWS service on your behalf. These rules are created by those other AWS services to support functionality in those services. You can delete these rules using the <code>Force</code> option, but you should do so only if you are sure the other service is not still using that rule.</p>
    async fn delete_rule(
        &self,
        input: DeleteRuleRequest,
    ) -> Result<(), RusotoError<DeleteRuleError>>;

    /// <p>Retrieves details about an API destination.</p>
    async fn describe_api_destination(
        &self,
        input: DescribeApiDestinationRequest,
    ) -> Result<DescribeApiDestinationResponse, RusotoError<DescribeApiDestinationError>>;

    /// <p>Retrieves details about an archive.</p>
    async fn describe_archive(
        &self,
        input: DescribeArchiveRequest,
    ) -> Result<DescribeArchiveResponse, RusotoError<DescribeArchiveError>>;

    /// <p>Retrieves details about a connection.</p>
    async fn describe_connection(
        &self,
        input: DescribeConnectionRequest,
    ) -> Result<DescribeConnectionResponse, RusotoError<DescribeConnectionError>>;

    /// <p>Displays details about an event bus in your account. This can include the external AWS accounts that are permitted to write events to your default event bus, and the associated policy. For custom event buses and partner event buses, it displays the name, ARN, policy, state, and creation time.</p> <p> To enable your account to receive events from other accounts on its default event bus, use <a>PutPermission</a>.</p> <p>For more information about partner event buses, see <a>CreateEventBus</a>.</p>
    async fn describe_event_bus(
        &self,
        input: DescribeEventBusRequest,
    ) -> Result<DescribeEventBusResponse, RusotoError<DescribeEventBusError>>;

    /// <p>This operation lists details about a partner event source that is shared with your account.</p>
    async fn describe_event_source(
        &self,
        input: DescribeEventSourceRequest,
    ) -> Result<DescribeEventSourceResponse, RusotoError<DescribeEventSourceError>>;

    /// <p>An SaaS partner can use this operation to list details about a partner event source that they have created. AWS customers do not use this operation. Instead, AWS customers can use <a>DescribeEventSource</a> to see details about a partner event source that is shared with them.</p>
    async fn describe_partner_event_source(
        &self,
        input: DescribePartnerEventSourceRequest,
    ) -> Result<DescribePartnerEventSourceResponse, RusotoError<DescribePartnerEventSourceError>>;

    /// <p>Retrieves details about a replay. Use <code>DescribeReplay</code> to determine the progress of a running replay. A replay processes events to replay based on the time in the event, and replays them using 1 minute intervals. If you use <code>StartReplay</code> and specify an <code>EventStartTime</code> and an <code>EventEndTime</code> that covers a 20 minute time range, the events are replayed from the first minute of that 20 minute range first. Then the events from the second minute are replayed. You can use <code>DescribeReplay</code> to determine the progress of a replay. The value returned for <code>EventLastReplayedTime</code> indicates the time within the specified time range associated with the last event replayed.</p>
    async fn describe_replay(
        &self,
        input: DescribeReplayRequest,
    ) -> Result<DescribeReplayResponse, RusotoError<DescribeReplayError>>;

    /// <p>Describes the specified rule.</p> <p>DescribeRule does not list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    async fn describe_rule(
        &self,
        input: DescribeRuleRequest,
    ) -> Result<DescribeRuleResponse, RusotoError<DescribeRuleError>>;

    /// <p>Disables the specified rule. A disabled rule won't match any events, and won't self-trigger if it has a schedule expression.</p> <p>When you disable a rule, incoming events might continue to match to the disabled rule. Allow a short period of time for changes to take effect.</p>
    async fn disable_rule(
        &self,
        input: DisableRuleRequest,
    ) -> Result<(), RusotoError<DisableRuleError>>;

    /// <p>Enables the specified rule. If the rule does not exist, the operation fails.</p> <p>When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Allow a short period of time for changes to take effect.</p>
    async fn enable_rule(
        &self,
        input: EnableRuleRequest,
    ) -> Result<(), RusotoError<EnableRuleError>>;

    /// <p>Retrieves a list of API destination in the account in the current Region.</p>
    async fn list_api_destinations(
        &self,
        input: ListApiDestinationsRequest,
    ) -> Result<ListApiDestinationsResponse, RusotoError<ListApiDestinationsError>>;

    /// <p>Lists your archives. You can either list all the archives or you can provide a prefix to match to the archive names. Filter parameters are exclusive.</p>
    async fn list_archives(
        &self,
        input: ListArchivesRequest,
    ) -> Result<ListArchivesResponse, RusotoError<ListArchivesError>>;

    /// <p>Retrieves a list of connections from the account.</p>
    async fn list_connections(
        &self,
        input: ListConnectionsRequest,
    ) -> Result<ListConnectionsResponse, RusotoError<ListConnectionsError>>;

    /// <p>Lists all the event buses in your account, including the default event bus, custom event buses, and partner event buses.</p>
    async fn list_event_buses(
        &self,
        input: ListEventBusesRequest,
    ) -> Result<ListEventBusesResponse, RusotoError<ListEventBusesError>>;

    /// <p>You can use this to see all the partner event sources that have been shared with your AWS account. For more information about partner event sources, see <a>CreateEventBus</a>.</p>
    async fn list_event_sources(
        &self,
        input: ListEventSourcesRequest,
    ) -> Result<ListEventSourcesResponse, RusotoError<ListEventSourcesError>>;

    /// <p>An SaaS partner can use this operation to display the AWS account ID that a particular partner event source name is associated with. This operation is not used by AWS customers.</p>
    async fn list_partner_event_source_accounts(
        &self,
        input: ListPartnerEventSourceAccountsRequest,
    ) -> Result<
        ListPartnerEventSourceAccountsResponse,
        RusotoError<ListPartnerEventSourceAccountsError>,
    >;

    /// <p>An SaaS partner can use this operation to list all the partner event source names that they have created. This operation is not used by AWS customers.</p>
    async fn list_partner_event_sources(
        &self,
        input: ListPartnerEventSourcesRequest,
    ) -> Result<ListPartnerEventSourcesResponse, RusotoError<ListPartnerEventSourcesError>>;

    /// <p>Lists your replays. You can either list all the replays or you can provide a prefix to match to the replay names. Filter parameters are exclusive.</p>
    async fn list_replays(
        &self,
        input: ListReplaysRequest,
    ) -> Result<ListReplaysResponse, RusotoError<ListReplaysError>>;

    /// <p>Lists the rules for the specified target. You can see which of the rules in Amazon EventBridge can invoke a specific target in your account.</p>
    async fn list_rule_names_by_target(
        &self,
        input: ListRuleNamesByTargetRequest,
    ) -> Result<ListRuleNamesByTargetResponse, RusotoError<ListRuleNamesByTargetError>>;

    /// <p>Lists your Amazon EventBridge rules. You can either list all the rules or you can provide a prefix to match to the rule names.</p> <p>ListRules does not list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    async fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> Result<ListRulesResponse, RusotoError<ListRulesError>>;

    /// <p>Displays the tags associated with an EventBridge resource. In EventBridge, rules and event buses can be tagged.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists the targets assigned to the specified rule.</p>
    async fn list_targets_by_rule(
        &self,
        input: ListTargetsByRuleRequest,
    ) -> Result<ListTargetsByRuleResponse, RusotoError<ListTargetsByRuleError>>;

    /// <p>Sends custom events to Amazon EventBridge so that they can be matched to rules.</p>
    async fn put_events(
        &self,
        input: PutEventsRequest,
    ) -> Result<PutEventsResponse, RusotoError<PutEventsError>>;

    /// <p>This is used by SaaS partners to write events to a customer's partner event bus. AWS customers do not use this operation.</p>
    async fn put_partner_events(
        &self,
        input: PutPartnerEventsRequest,
    ) -> Result<PutPartnerEventsResponse, RusotoError<PutPartnerEventsError>>;

    /// <p>Running <code>PutPermission</code> permits the specified AWS account or AWS organization to put events to the specified <i>event bus</i>. Amazon EventBridge (CloudWatch Events) rules in your account are triggered by these events arriving to an event bus in your account. </p> <p>For another account to send events to your account, that external account must have an EventBridge rule with your account's event bus as a target.</p> <p>To enable multiple AWS accounts to put events to your event bus, run <code>PutPermission</code> once for each of these accounts. Or, if all the accounts are members of the same AWS organization, you can run <code>PutPermission</code> once specifying <code>Principal</code> as "*" and specifying the AWS organization ID in <code>Condition</code>, to grant permissions to all accounts in that organization.</p> <p>If you grant permissions using an organization, then accounts in that organization must specify a <code>RoleArn</code> with proper permissions when they use <code>PutTarget</code> to add your account's event bus as a target. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-cross-account-event-delivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>The permission policy on the default event bus cannot exceed 10 KB in size.</p>
    async fn put_permission(
        &self,
        input: PutPermissionRequest,
    ) -> Result<(), RusotoError<PutPermissionError>>;

    /// <p>Creates or updates the specified rule. Rules are enabled by default, or based on value of the state. You can disable a rule using <a>DisableRule</a>.</p> <p>A single rule watches for events from a single event bus. Events generated by AWS services go to your account's default event bus. Events generated by SaaS partner services or applications go to the matching partner event bus. If you have custom applications or services, you can specify whether their events go to your default event bus or a custom event bus that you have created. For more information, see <a>CreateEventBus</a>.</p> <p>If you are updating an existing rule, the rule is replaced with what you specify in this <code>PutRule</code> command. If you omit arguments in <code>PutRule</code>, the old values for those arguments are not kept. Instead, they are replaced with null values.</p> <p>When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an EventPattern or ScheduleExpression. Rules with EventPatterns are triggered when a matching event is observed. Rules with ScheduleExpressions self-trigger based on the given schedule. A rule can have both an EventPattern and a ScheduleExpression, in which case the rule triggers on matching events as well as on a schedule.</p> <p>When you initially create a rule, you can optionally assign one or more tags to the rule. Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only rules with certain tag values. To use the <code>PutRule</code> operation and assign tags, you must have both the <code>events:PutRule</code> and <code>events:TagResource</code> permissions.</p> <p>If you are updating an existing rule, any tags you specify in the <code>PutRule</code> operation are ignored. To update the tags of an existing rule, use <a>TagResource</a> and <a>UntagResource</a>.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, EventBridge uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p> <p>In EventBridge, it is possible to create rules that lead to infinite loops, where a rule is fired repeatedly. For example, a rule might detect that ACLs have changed on an S3 bucket, and trigger software to change them to the desired state. If the rule is not written carefully, the subsequent change to the ACLs fires the rule again, creating an infinite loop.</p> <p>To prevent this, write the rules so that the triggered actions do not re-fire the same rule. For example, your rule could fire only if ACLs are found to be in a bad state, instead of after any change. </p> <p>An infinite loop can quickly cause higher than expected charges. We recommend that you use budgeting, which alerts you when charges exceed your specified limit. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/budgets-managing-costs.html">Managing Your Costs with Budgets</a>.</p>
    async fn put_rule(
        &self,
        input: PutRuleRequest,
    ) -> Result<PutRuleResponse, RusotoError<PutRuleError>>;

    /// <p>Adds the specified targets to the specified rule, or updates the targets if they are already associated with the rule.</p> <p>Targets are the resources that are invoked when a rule is triggered.</p> <p>You can configure the following as targets for Events:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-api-destinations.html">API destination</a> </p> </li> <li> <p>Amazon API Gateway REST API endpoints</p> </li> <li> <p>API Gateway</p> </li> <li> <p>AWS Batch job queue</p> </li> <li> <p>CloudWatch Logs group</p> </li> <li> <p>CodeBuild project</p> </li> <li> <p>CodePineline</p> </li> <li> <p>Amazon EC2 <code>CreateSnapshot</code> API call</p> </li> <li> <p>Amazon EC2 <code>RebootInstances</code> API call</p> </li> <li> <p>Amazon EC2 <code>StopInstances</code> API call</p> </li> <li> <p>Amazon EC2 <code>TerminateInstances</code> API call</p> </li> <li> <p>Amazon ECS tasks</p> </li> <li> <p>Event bus in a different AWS account or Region.</p> <p>You can use an event bus in the US East (N. Virginia) us-east-1, US West (Oregon) us-west-2, or Europe (Ireland) eu-west-1 Regions as a target for a rule.</p> </li> <li> <p>Firehose delivery stream (Kinesis Data Firehose)</p> </li> <li> <p>Inspector assessment template (Amazon Inspector)</p> </li> <li> <p>Kinesis stream (Kinesis Data Stream)</p> </li> <li> <p>AWS Lambda function</p> </li> <li> <p>Redshift clusters (Data API statement execution)</p> </li> <li> <p>Amazon SNS topic</p> </li> <li> <p>Amazon SQS queues (includes FIFO queues</p> </li> <li> <p>SSM Automation</p> </li> <li> <p>SSM OpsItem</p> </li> <li> <p>SSM Run Command</p> </li> <li> <p>Step Functions state machines</p> </li> </ul> <p>Creating rules with built-in targets is supported only in the AWS Management Console. The built-in targets are <code>EC2 CreateSnapshot API call</code>, <code>EC2 RebootInstances API call</code>, <code>EC2 StopInstances API call</code>, and <code>EC2 TerminateInstances API call</code>. </p> <p>For some target types, <code>PutTargets</code> provides target-specific parameters. If the target is a Kinesis data stream, you can optionally specify which shard the event goes to by using the <code>KinesisParameters</code> argument. To invoke a command on multiple EC2 instances with one rule, you can use the <code>RunCommandParameters</code> field.</p> <p>To be able to make API calls against the resources that you own, Amazon EventBridge (CloudWatch Events) needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, EventBridge relies on resource-based policies. For EC2 instances, Kinesis data streams, AWS Step Functions state machines and API Gateway REST APIs, EventBridge relies on IAM roles that you specify in the <code>RoleARN</code> argument in <code>PutTargets</code>. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/auth-and-access-control-eventbridge.html">Authentication and Access Control</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>If another AWS account is in the same region and has granted you permission (using <code>PutPermission</code>), you can send events to that account. Set that account's event bus as a target of the rules in your account. To send the matched events to the other account, specify that account's event bus as the <code>Arn</code> value when you run <code>PutTargets</code>. If your account sends events to another account, your account is charged for each sent event. Each event sent to another account is charged as a custom event. The account receiving the event is not charged. For more information, see <a href="https://aws.amazon.com/eventbridge/pricing/">Amazon EventBridge (CloudWatch Events) Pricing</a>.</p> <note> <p> <code>Input</code>, <code>InputPath</code>, and <code>InputTransformer</code> are not available with <code>PutTarget</code> if the target is an event bus of a different AWS account.</p> </note> <p>If you are setting the event bus of another account as the target, and that account granted permission to your account through an organization instead of directly by the account ID, then you must specify a <code>RoleArn</code> with proper permissions in the <code>Target</code> structure. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-cross-account-event-delivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>For more information about enabling cross-account events, see <a>PutPermission</a>.</p> <p> <b>Input</b>, <b>InputPath</b>, and <b>InputTransformer</b> are mutually exclusive and optional parameters of a target. When a rule is triggered due to a matched event:</p> <ul> <li> <p>If none of the following arguments are specified for a target, then the entire event is passed to the target in JSON format (unless the target is Amazon EC2 Run Command or Amazon ECS task, in which case nothing from the event is passed to the target).</p> </li> <li> <p>If <b>Input</b> is specified in the form of valid JSON, then the matched event is overridden with this constant.</p> </li> <li> <p>If <b>InputPath</b> is specified in the form of JSONPath (for example, <code>$.detail</code>), then only the part of the event specified in the path is passed to the target (for example, only the detail part of the event is passed).</p> </li> <li> <p>If <b>InputTransformer</b> is specified, then one or more specified JSONPaths are extracted from the event and used as values in a template that you specify as the input to the target.</p> </li> </ul> <p>When you specify <code>InputPath</code> or <code>InputTransformer</code>, you must use JSON dot notation, not bracket notation.</p> <p>When you add targets to a rule and the associated rule triggers soon after, new or updated targets might not be immediately invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    async fn put_targets(
        &self,
        input: PutTargetsRequest,
    ) -> Result<PutTargetsResponse, RusotoError<PutTargetsError>>;

    /// <p>Revokes the permission of another AWS account to be able to put events to the specified event bus. Specify the account to revoke by the <code>StatementId</code> value that you associated with the account when you granted it permission with <code>PutPermission</code>. You can find the <code>StatementId</code> by using <a>DescribeEventBus</a>.</p>
    async fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> Result<(), RusotoError<RemovePermissionError>>;

    /// <p>Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked.</p> <p>When you remove a target, when the associated rule triggers, removed targets might continue to be invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    async fn remove_targets(
        &self,
        input: RemoveTargetsRequest,
    ) -> Result<RemoveTargetsResponse, RusotoError<RemoveTargetsError>>;

    /// <p>Starts the specified replay. Events are not necessarily replayed in the exact same order that they were added to the archive. A replay processes events to replay based on the time in the event, and replays them using 1 minute intervals. If you specify an <code>EventStartTime</code> and an <code>EventEndTime</code> that covers a 20 minute time range, the events are replayed from the first minute of that 20 minute range first. Then the events from the second minute are replayed. You can use <code>DescribeReplay</code> to determine the progress of a replay. The value returned for <code>EventLastReplayedTime</code> indicates the time within the specified time range associated with the last event replayed.</p>
    async fn start_replay(
        &self,
        input: StartReplayRequest,
    ) -> Result<StartReplayResponse, RusotoError<StartReplayError>>;

    /// <p>Assigns one or more tags (key-value pairs) to the specified EventBridge resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values. In EventBridge, rules and event buses can be tagged.</p> <p>Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.</p> <p>You can use the <code>TagResource</code> action with a resource that already has tags. If you specify a new tag key, this tag is appended to the list of tags associated with the resource. If you specify a tag key that is already associated with the resource, the new tag value that you specify replaces the previous value for that tag.</p> <p>You can associate as many as 50 tags with a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Tests whether the specified event pattern matches the provided event.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, EventBridge uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p>
    async fn test_event_pattern(
        &self,
        input: TestEventPatternRequest,
    ) -> Result<TestEventPatternResponse, RusotoError<TestEventPatternError>>;

    /// <p>Removes one or more tags from the specified EventBridge resource. In Amazon EventBridge (CloudWatch Events), rules and event buses can be tagged.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates an API destination.</p>
    async fn update_api_destination(
        &self,
        input: UpdateApiDestinationRequest,
    ) -> Result<UpdateApiDestinationResponse, RusotoError<UpdateApiDestinationError>>;

    /// <p>Updates the specified archive.</p>
    async fn update_archive(
        &self,
        input: UpdateArchiveRequest,
    ) -> Result<UpdateArchiveResponse, RusotoError<UpdateArchiveError>>;

    /// <p>Updates settings for a connection.</p>
    async fn update_connection(
        &self,
        input: UpdateConnectionRequest,
    ) -> Result<UpdateConnectionResponse, RusotoError<UpdateConnectionError>>;
}
/// A client for the Amazon EventBridge API.
#[derive(Clone)]
pub struct EventBridgeClient {
    client: Client,
    region: region::Region,
}

impl EventBridgeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> EventBridgeClient {
        EventBridgeClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EventBridgeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        EventBridgeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> EventBridgeClient {
        EventBridgeClient { client, region }
    }
}

#[async_trait]
impl EventBridge for EventBridgeClient {
    /// <p>Activates a partner event source that has been deactivated. Once activated, your matching event bus will start receiving events from the event source.</p>
    async fn activate_event_source(
        &self,
        input: ActivateEventSourceRequest,
    ) -> Result<(), RusotoError<ActivateEventSourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ActivateEventSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ActivateEventSourceError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Cancels the specified replay.</p>
    async fn cancel_replay(
        &self,
        input: CancelReplayRequest,
    ) -> Result<CancelReplayResponse, RusotoError<CancelReplayError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.CancelReplay");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CancelReplayError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CancelReplayResponse, _>()
    }

    /// <p>Creates an API destination, which is an HTTP invocation endpoint configured as a target for events.</p>
    async fn create_api_destination(
        &self,
        input: CreateApiDestinationRequest,
    ) -> Result<CreateApiDestinationResponse, RusotoError<CreateApiDestinationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.CreateApiDestination");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateApiDestinationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateApiDestinationResponse, _>()
    }

    /// <p>Creates an archive of events with the specified settings. When you create an archive, incoming events might not immediately start being sent to the archive. Allow a short period of time for changes to take effect. If you do not specify a pattern to filter events sent to the archive, all events are sent to the archive except replayed events. Replayed events are not sent to an archive.</p>
    async fn create_archive(
        &self,
        input: CreateArchiveRequest,
    ) -> Result<CreateArchiveResponse, RusotoError<CreateArchiveError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.CreateArchive");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateArchiveError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateArchiveResponse, _>()
    }

    /// <p>Creates a connection. A connection defines the authorization type and credentials to use for authorization with an API destination HTTP endpoint.</p>
    async fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> Result<CreateConnectionResponse, RusotoError<CreateConnectionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.CreateConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateConnectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateConnectionResponse, _>()
    }

    /// <p>Creates a new event bus within your account. This can be a custom event bus which you can use to receive events from your custom applications and services, or it can be a partner event bus which can be matched to a partner event source.</p>
    async fn create_event_bus(
        &self,
        input: CreateEventBusRequest,
    ) -> Result<CreateEventBusResponse, RusotoError<CreateEventBusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.CreateEventBus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateEventBusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateEventBusResponse, _>()
    }

    /// <p>Called by an SaaS partner to create a partner event source. This operation is not used by AWS customers.</p> <p>Each partner event source can be used by one AWS account to create a matching partner event bus in that AWS account. A SaaS partner must create one partner event source for each AWS account that wants to receive those event types. </p> <p>A partner event source creates events based on resources within the SaaS partner's service or application.</p> <p>An AWS account that creates a partner event bus that matches the partner event source can use that event bus to receive events from the partner, and then process them using AWS Events rules and targets.</p> <p>Partner event source names follow this format:</p> <p> <code> <i>partner_name</i>/<i>event_namespace</i>/<i>event_name</i> </code> </p> <p> <i>partner_name</i> is determined during partner registration and identifies the partner to AWS customers. <i>event_namespace</i> is determined by the partner and is a way for the partner to categorize their events. <i>event_name</i> is determined by the partner, and should uniquely identify an event-generating resource within the partner system. The combination of <i>event_namespace</i> and <i>event_name</i> should help AWS customers decide whether to create an event bus to receive these events.</p>
    async fn create_partner_event_source(
        &self,
        input: CreatePartnerEventSourceRequest,
    ) -> Result<CreatePartnerEventSourceResponse, RusotoError<CreatePartnerEventSourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.CreatePartnerEventSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreatePartnerEventSourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreatePartnerEventSourceResponse, _>()
    }

    /// <p>You can use this operation to temporarily stop receiving events from the specified partner event source. The matching event bus is not deleted. </p> <p>When you deactivate a partner event source, the source goes into PENDING state. If it remains in PENDING state for more than two weeks, it is deleted.</p> <p>To activate a deactivated partner event source, use <a>ActivateEventSource</a>.</p>
    async fn deactivate_event_source(
        &self,
        input: DeactivateEventSourceRequest,
    ) -> Result<(), RusotoError<DeactivateEventSourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DeactivateEventSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeactivateEventSourceError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Removes all authorization parameters from the connection. This lets you remove the secret from the connection so you can reuse it without having to create a new connection.</p>
    async fn deauthorize_connection(
        &self,
        input: DeauthorizeConnectionRequest,
    ) -> Result<DeauthorizeConnectionResponse, RusotoError<DeauthorizeConnectionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DeauthorizeConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeauthorizeConnectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeauthorizeConnectionResponse, _>()
    }

    /// <p>Deletes the specified API destination.</p>
    async fn delete_api_destination(
        &self,
        input: DeleteApiDestinationRequest,
    ) -> Result<DeleteApiDestinationResponse, RusotoError<DeleteApiDestinationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DeleteApiDestination");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteApiDestinationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteApiDestinationResponse, _>()
    }

    /// <p>Deletes the specified archive.</p>
    async fn delete_archive(
        &self,
        input: DeleteArchiveRequest,
    ) -> Result<DeleteArchiveResponse, RusotoError<DeleteArchiveError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DeleteArchive");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteArchiveError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteArchiveResponse, _>()
    }

    /// <p>Deletes a connection.</p>
    async fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> Result<DeleteConnectionResponse, RusotoError<DeleteConnectionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DeleteConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteConnectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteConnectionResponse, _>()
    }

    /// <p>Deletes the specified custom event bus or partner event bus. All rules associated with this event bus need to be deleted. You can't delete your account's default event bus.</p>
    async fn delete_event_bus(
        &self,
        input: DeleteEventBusRequest,
    ) -> Result<(), RusotoError<DeleteEventBusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DeleteEventBus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteEventBusError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>This operation is used by SaaS partners to delete a partner event source. This operation is not used by AWS customers.</p> <p>When you delete an event source, the status of the corresponding partner event bus in the AWS customer account becomes DELETED.</p> <p/></p>
    async fn delete_partner_event_source(
        &self,
        input: DeletePartnerEventSourceRequest,
    ) -> Result<(), RusotoError<DeletePartnerEventSourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DeletePartnerEventSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeletePartnerEventSourceError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes the specified rule.</p> <p>Before you can delete the rule, you must remove all targets, using <a>RemoveTargets</a>.</p> <p>When you delete a rule, incoming events might continue to match to the deleted rule. Allow a short period of time for changes to take effect.</p> <p>If you call delete rule multiple times for the same rule, all calls will succeed. When you call delete rule for a non-existent custom eventbus, <code>ResourceNotFoundException</code> is returned.</p> <p>Managed rules are rules created and managed by another AWS service on your behalf. These rules are created by those other AWS services to support functionality in those services. You can delete these rules using the <code>Force</code> option, but you should do so only if you are sure the other service is not still using that rule.</p>
    async fn delete_rule(
        &self,
        input: DeleteRuleRequest,
    ) -> Result<(), RusotoError<DeleteRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DeleteRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteRuleError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Retrieves details about an API destination.</p>
    async fn describe_api_destination(
        &self,
        input: DescribeApiDestinationRequest,
    ) -> Result<DescribeApiDestinationResponse, RusotoError<DescribeApiDestinationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DescribeApiDestination");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeApiDestinationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeApiDestinationResponse, _>()
    }

    /// <p>Retrieves details about an archive.</p>
    async fn describe_archive(
        &self,
        input: DescribeArchiveRequest,
    ) -> Result<DescribeArchiveResponse, RusotoError<DescribeArchiveError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DescribeArchive");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeArchiveError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeArchiveResponse, _>()
    }

    /// <p>Retrieves details about a connection.</p>
    async fn describe_connection(
        &self,
        input: DescribeConnectionRequest,
    ) -> Result<DescribeConnectionResponse, RusotoError<DescribeConnectionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DescribeConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeConnectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeConnectionResponse, _>()
    }

    /// <p>Displays details about an event bus in your account. This can include the external AWS accounts that are permitted to write events to your default event bus, and the associated policy. For custom event buses and partner event buses, it displays the name, ARN, policy, state, and creation time.</p> <p> To enable your account to receive events from other accounts on its default event bus, use <a>PutPermission</a>.</p> <p>For more information about partner event buses, see <a>CreateEventBus</a>.</p>
    async fn describe_event_bus(
        &self,
        input: DescribeEventBusRequest,
    ) -> Result<DescribeEventBusResponse, RusotoError<DescribeEventBusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DescribeEventBus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEventBusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeEventBusResponse, _>()
    }

    /// <p>This operation lists details about a partner event source that is shared with your account.</p>
    async fn describe_event_source(
        &self,
        input: DescribeEventSourceRequest,
    ) -> Result<DescribeEventSourceResponse, RusotoError<DescribeEventSourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DescribeEventSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEventSourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeEventSourceResponse, _>()
    }

    /// <p>An SaaS partner can use this operation to list details about a partner event source that they have created. AWS customers do not use this operation. Instead, AWS customers can use <a>DescribeEventSource</a> to see details about a partner event source that is shared with them.</p>
    async fn describe_partner_event_source(
        &self,
        input: DescribePartnerEventSourceRequest,
    ) -> Result<DescribePartnerEventSourceResponse, RusotoError<DescribePartnerEventSourceError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DescribePartnerEventSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribePartnerEventSourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribePartnerEventSourceResponse, _>()
    }

    /// <p>Retrieves details about a replay. Use <code>DescribeReplay</code> to determine the progress of a running replay. A replay processes events to replay based on the time in the event, and replays them using 1 minute intervals. If you use <code>StartReplay</code> and specify an <code>EventStartTime</code> and an <code>EventEndTime</code> that covers a 20 minute time range, the events are replayed from the first minute of that 20 minute range first. Then the events from the second minute are replayed. You can use <code>DescribeReplay</code> to determine the progress of a replay. The value returned for <code>EventLastReplayedTime</code> indicates the time within the specified time range associated with the last event replayed.</p>
    async fn describe_replay(
        &self,
        input: DescribeReplayRequest,
    ) -> Result<DescribeReplayResponse, RusotoError<DescribeReplayError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DescribeReplay");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeReplayError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeReplayResponse, _>()
    }

    /// <p>Describes the specified rule.</p> <p>DescribeRule does not list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    async fn describe_rule(
        &self,
        input: DescribeRuleRequest,
    ) -> Result<DescribeRuleResponse, RusotoError<DescribeRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DescribeRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeRuleResponse, _>()
    }

    /// <p>Disables the specified rule. A disabled rule won't match any events, and won't self-trigger if it has a schedule expression.</p> <p>When you disable a rule, incoming events might continue to match to the disabled rule. Allow a short period of time for changes to take effect.</p>
    async fn disable_rule(
        &self,
        input: DisableRuleRequest,
    ) -> Result<(), RusotoError<DisableRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.DisableRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisableRuleError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Enables the specified rule. If the rule does not exist, the operation fails.</p> <p>When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Allow a short period of time for changes to take effect.</p>
    async fn enable_rule(
        &self,
        input: EnableRuleRequest,
    ) -> Result<(), RusotoError<EnableRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.EnableRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, EnableRuleError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Retrieves a list of API destination in the account in the current Region.</p>
    async fn list_api_destinations(
        &self,
        input: ListApiDestinationsRequest,
    ) -> Result<ListApiDestinationsResponse, RusotoError<ListApiDestinationsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListApiDestinations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListApiDestinationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListApiDestinationsResponse, _>()
    }

    /// <p>Lists your archives. You can either list all the archives or you can provide a prefix to match to the archive names. Filter parameters are exclusive.</p>
    async fn list_archives(
        &self,
        input: ListArchivesRequest,
    ) -> Result<ListArchivesResponse, RusotoError<ListArchivesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListArchives");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListArchivesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListArchivesResponse, _>()
    }

    /// <p>Retrieves a list of connections from the account.</p>
    async fn list_connections(
        &self,
        input: ListConnectionsRequest,
    ) -> Result<ListConnectionsResponse, RusotoError<ListConnectionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListConnectionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListConnectionsResponse, _>()
    }

    /// <p>Lists all the event buses in your account, including the default event bus, custom event buses, and partner event buses.</p>
    async fn list_event_buses(
        &self,
        input: ListEventBusesRequest,
    ) -> Result<ListEventBusesResponse, RusotoError<ListEventBusesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListEventBuses");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListEventBusesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListEventBusesResponse, _>()
    }

    /// <p>You can use this to see all the partner event sources that have been shared with your AWS account. For more information about partner event sources, see <a>CreateEventBus</a>.</p>
    async fn list_event_sources(
        &self,
        input: ListEventSourcesRequest,
    ) -> Result<ListEventSourcesResponse, RusotoError<ListEventSourcesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListEventSources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListEventSourcesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListEventSourcesResponse, _>()
    }

    /// <p>An SaaS partner can use this operation to display the AWS account ID that a particular partner event source name is associated with. This operation is not used by AWS customers.</p>
    async fn list_partner_event_source_accounts(
        &self,
        input: ListPartnerEventSourceAccountsRequest,
    ) -> Result<
        ListPartnerEventSourceAccountsResponse,
        RusotoError<ListPartnerEventSourceAccountsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListPartnerEventSourceAccounts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListPartnerEventSourceAccountsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListPartnerEventSourceAccountsResponse, _>()
    }

    /// <p>An SaaS partner can use this operation to list all the partner event source names that they have created. This operation is not used by AWS customers.</p>
    async fn list_partner_event_sources(
        &self,
        input: ListPartnerEventSourcesRequest,
    ) -> Result<ListPartnerEventSourcesResponse, RusotoError<ListPartnerEventSourcesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListPartnerEventSources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListPartnerEventSourcesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListPartnerEventSourcesResponse, _>()
    }

    /// <p>Lists your replays. You can either list all the replays or you can provide a prefix to match to the replay names. Filter parameters are exclusive.</p>
    async fn list_replays(
        &self,
        input: ListReplaysRequest,
    ) -> Result<ListReplaysResponse, RusotoError<ListReplaysError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListReplays");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListReplaysError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListReplaysResponse, _>()
    }

    /// <p>Lists the rules for the specified target. You can see which of the rules in Amazon EventBridge can invoke a specific target in your account.</p>
    async fn list_rule_names_by_target(
        &self,
        input: ListRuleNamesByTargetRequest,
    ) -> Result<ListRuleNamesByTargetResponse, RusotoError<ListRuleNamesByTargetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListRuleNamesByTarget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRuleNamesByTargetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListRuleNamesByTargetResponse, _>()
    }

    /// <p>Lists your Amazon EventBridge rules. You can either list all the rules or you can provide a prefix to match to the rule names.</p> <p>ListRules does not list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    async fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> Result<ListRulesResponse, RusotoError<ListRulesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRulesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListRulesResponse, _>()
    }

    /// <p>Displays the tags associated with an EventBridge resource. In EventBridge, rules and event buses can be tagged.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Lists the targets assigned to the specified rule.</p>
    async fn list_targets_by_rule(
        &self,
        input: ListTargetsByRuleRequest,
    ) -> Result<ListTargetsByRuleResponse, RusotoError<ListTargetsByRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.ListTargetsByRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTargetsByRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTargetsByRuleResponse, _>()
    }

    /// <p>Sends custom events to Amazon EventBridge so that they can be matched to rules.</p>
    async fn put_events(
        &self,
        input: PutEventsRequest,
    ) -> Result<PutEventsResponse, RusotoError<PutEventsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.PutEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutEventsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutEventsResponse, _>()
    }

    /// <p>This is used by SaaS partners to write events to a customer's partner event bus. AWS customers do not use this operation.</p>
    async fn put_partner_events(
        &self,
        input: PutPartnerEventsRequest,
    ) -> Result<PutPartnerEventsResponse, RusotoError<PutPartnerEventsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.PutPartnerEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutPartnerEventsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutPartnerEventsResponse, _>()
    }

    /// <p>Running <code>PutPermission</code> permits the specified AWS account or AWS organization to put events to the specified <i>event bus</i>. Amazon EventBridge (CloudWatch Events) rules in your account are triggered by these events arriving to an event bus in your account. </p> <p>For another account to send events to your account, that external account must have an EventBridge rule with your account's event bus as a target.</p> <p>To enable multiple AWS accounts to put events to your event bus, run <code>PutPermission</code> once for each of these accounts. Or, if all the accounts are members of the same AWS organization, you can run <code>PutPermission</code> once specifying <code>Principal</code> as "*" and specifying the AWS organization ID in <code>Condition</code>, to grant permissions to all accounts in that organization.</p> <p>If you grant permissions using an organization, then accounts in that organization must specify a <code>RoleArn</code> with proper permissions when they use <code>PutTarget</code> to add your account's event bus as a target. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-cross-account-event-delivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>The permission policy on the default event bus cannot exceed 10 KB in size.</p>
    async fn put_permission(
        &self,
        input: PutPermissionRequest,
    ) -> Result<(), RusotoError<PutPermissionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.PutPermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutPermissionError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Creates or updates the specified rule. Rules are enabled by default, or based on value of the state. You can disable a rule using <a>DisableRule</a>.</p> <p>A single rule watches for events from a single event bus. Events generated by AWS services go to your account's default event bus. Events generated by SaaS partner services or applications go to the matching partner event bus. If you have custom applications or services, you can specify whether their events go to your default event bus or a custom event bus that you have created. For more information, see <a>CreateEventBus</a>.</p> <p>If you are updating an existing rule, the rule is replaced with what you specify in this <code>PutRule</code> command. If you omit arguments in <code>PutRule</code>, the old values for those arguments are not kept. Instead, they are replaced with null values.</p> <p>When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an EventPattern or ScheduleExpression. Rules with EventPatterns are triggered when a matching event is observed. Rules with ScheduleExpressions self-trigger based on the given schedule. A rule can have both an EventPattern and a ScheduleExpression, in which case the rule triggers on matching events as well as on a schedule.</p> <p>When you initially create a rule, you can optionally assign one or more tags to the rule. Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only rules with certain tag values. To use the <code>PutRule</code> operation and assign tags, you must have both the <code>events:PutRule</code> and <code>events:TagResource</code> permissions.</p> <p>If you are updating an existing rule, any tags you specify in the <code>PutRule</code> operation are ignored. To update the tags of an existing rule, use <a>TagResource</a> and <a>UntagResource</a>.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, EventBridge uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p> <p>In EventBridge, it is possible to create rules that lead to infinite loops, where a rule is fired repeatedly. For example, a rule might detect that ACLs have changed on an S3 bucket, and trigger software to change them to the desired state. If the rule is not written carefully, the subsequent change to the ACLs fires the rule again, creating an infinite loop.</p> <p>To prevent this, write the rules so that the triggered actions do not re-fire the same rule. For example, your rule could fire only if ACLs are found to be in a bad state, instead of after any change. </p> <p>An infinite loop can quickly cause higher than expected charges. We recommend that you use budgeting, which alerts you when charges exceed your specified limit. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/budgets-managing-costs.html">Managing Your Costs with Budgets</a>.</p>
    async fn put_rule(
        &self,
        input: PutRuleRequest,
    ) -> Result<PutRuleResponse, RusotoError<PutRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.PutRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutRuleResponse, _>()
    }

    /// <p>Adds the specified targets to the specified rule, or updates the targets if they are already associated with the rule.</p> <p>Targets are the resources that are invoked when a rule is triggered.</p> <p>You can configure the following as targets for Events:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-api-destinations.html">API destination</a> </p> </li> <li> <p>Amazon API Gateway REST API endpoints</p> </li> <li> <p>API Gateway</p> </li> <li> <p>AWS Batch job queue</p> </li> <li> <p>CloudWatch Logs group</p> </li> <li> <p>CodeBuild project</p> </li> <li> <p>CodePineline</p> </li> <li> <p>Amazon EC2 <code>CreateSnapshot</code> API call</p> </li> <li> <p>Amazon EC2 <code>RebootInstances</code> API call</p> </li> <li> <p>Amazon EC2 <code>StopInstances</code> API call</p> </li> <li> <p>Amazon EC2 <code>TerminateInstances</code> API call</p> </li> <li> <p>Amazon ECS tasks</p> </li> <li> <p>Event bus in a different AWS account or Region.</p> <p>You can use an event bus in the US East (N. Virginia) us-east-1, US West (Oregon) us-west-2, or Europe (Ireland) eu-west-1 Regions as a target for a rule.</p> </li> <li> <p>Firehose delivery stream (Kinesis Data Firehose)</p> </li> <li> <p>Inspector assessment template (Amazon Inspector)</p> </li> <li> <p>Kinesis stream (Kinesis Data Stream)</p> </li> <li> <p>AWS Lambda function</p> </li> <li> <p>Redshift clusters (Data API statement execution)</p> </li> <li> <p>Amazon SNS topic</p> </li> <li> <p>Amazon SQS queues (includes FIFO queues</p> </li> <li> <p>SSM Automation</p> </li> <li> <p>SSM OpsItem</p> </li> <li> <p>SSM Run Command</p> </li> <li> <p>Step Functions state machines</p> </li> </ul> <p>Creating rules with built-in targets is supported only in the AWS Management Console. The built-in targets are <code>EC2 CreateSnapshot API call</code>, <code>EC2 RebootInstances API call</code>, <code>EC2 StopInstances API call</code>, and <code>EC2 TerminateInstances API call</code>. </p> <p>For some target types, <code>PutTargets</code> provides target-specific parameters. If the target is a Kinesis data stream, you can optionally specify which shard the event goes to by using the <code>KinesisParameters</code> argument. To invoke a command on multiple EC2 instances with one rule, you can use the <code>RunCommandParameters</code> field.</p> <p>To be able to make API calls against the resources that you own, Amazon EventBridge (CloudWatch Events) needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, EventBridge relies on resource-based policies. For EC2 instances, Kinesis data streams, AWS Step Functions state machines and API Gateway REST APIs, EventBridge relies on IAM roles that you specify in the <code>RoleARN</code> argument in <code>PutTargets</code>. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/auth-and-access-control-eventbridge.html">Authentication and Access Control</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>If another AWS account is in the same region and has granted you permission (using <code>PutPermission</code>), you can send events to that account. Set that account's event bus as a target of the rules in your account. To send the matched events to the other account, specify that account's event bus as the <code>Arn</code> value when you run <code>PutTargets</code>. If your account sends events to another account, your account is charged for each sent event. Each event sent to another account is charged as a custom event. The account receiving the event is not charged. For more information, see <a href="https://aws.amazon.com/eventbridge/pricing/">Amazon EventBridge (CloudWatch Events) Pricing</a>.</p> <note> <p> <code>Input</code>, <code>InputPath</code>, and <code>InputTransformer</code> are not available with <code>PutTarget</code> if the target is an event bus of a different AWS account.</p> </note> <p>If you are setting the event bus of another account as the target, and that account granted permission to your account through an organization instead of directly by the account ID, then you must specify a <code>RoleArn</code> with proper permissions in the <code>Target</code> structure. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-cross-account-event-delivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>For more information about enabling cross-account events, see <a>PutPermission</a>.</p> <p> <b>Input</b>, <b>InputPath</b>, and <b>InputTransformer</b> are mutually exclusive and optional parameters of a target. When a rule is triggered due to a matched event:</p> <ul> <li> <p>If none of the following arguments are specified for a target, then the entire event is passed to the target in JSON format (unless the target is Amazon EC2 Run Command or Amazon ECS task, in which case nothing from the event is passed to the target).</p> </li> <li> <p>If <b>Input</b> is specified in the form of valid JSON, then the matched event is overridden with this constant.</p> </li> <li> <p>If <b>InputPath</b> is specified in the form of JSONPath (for example, <code>$.detail</code>), then only the part of the event specified in the path is passed to the target (for example, only the detail part of the event is passed).</p> </li> <li> <p>If <b>InputTransformer</b> is specified, then one or more specified JSONPaths are extracted from the event and used as values in a template that you specify as the input to the target.</p> </li> </ul> <p>When you specify <code>InputPath</code> or <code>InputTransformer</code>, you must use JSON dot notation, not bracket notation.</p> <p>When you add targets to a rule and the associated rule triggers soon after, new or updated targets might not be immediately invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    async fn put_targets(
        &self,
        input: PutTargetsRequest,
    ) -> Result<PutTargetsResponse, RusotoError<PutTargetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.PutTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutTargetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutTargetsResponse, _>()
    }

    /// <p>Revokes the permission of another AWS account to be able to put events to the specified event bus. Specify the account to revoke by the <code>StatementId</code> value that you associated with the account when you granted it permission with <code>PutPermission</code>. You can find the <code>StatementId</code> by using <a>DescribeEventBus</a>.</p>
    async fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> Result<(), RusotoError<RemovePermissionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.RemovePermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RemovePermissionError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked.</p> <p>When you remove a target, when the associated rule triggers, removed targets might continue to be invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    async fn remove_targets(
        &self,
        input: RemoveTargetsRequest,
    ) -> Result<RemoveTargetsResponse, RusotoError<RemoveTargetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.RemoveTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RemoveTargetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RemoveTargetsResponse, _>()
    }

    /// <p>Starts the specified replay. Events are not necessarily replayed in the exact same order that they were added to the archive. A replay processes events to replay based on the time in the event, and replays them using 1 minute intervals. If you specify an <code>EventStartTime</code> and an <code>EventEndTime</code> that covers a 20 minute time range, the events are replayed from the first minute of that 20 minute range first. Then the events from the second minute are replayed. You can use <code>DescribeReplay</code> to determine the progress of a replay. The value returned for <code>EventLastReplayedTime</code> indicates the time within the specified time range associated with the last event replayed.</p>
    async fn start_replay(
        &self,
        input: StartReplayRequest,
    ) -> Result<StartReplayResponse, RusotoError<StartReplayError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.StartReplay");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartReplayError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartReplayResponse, _>()
    }

    /// <p>Assigns one or more tags (key-value pairs) to the specified EventBridge resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values. In EventBridge, rules and event buses can be tagged.</p> <p>Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.</p> <p>You can use the <code>TagResource</code> action with a resource that already has tags. If you specify a new tag key, this tag is appended to the list of tags associated with the resource. If you specify a tag key that is already associated with the resource, the new tag value that you specify replaces the previous value for that tag.</p> <p>You can associate as many as 50 tags with a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Tests whether the specified event pattern matches the provided event.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, EventBridge uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p>
    async fn test_event_pattern(
        &self,
        input: TestEventPatternRequest,
    ) -> Result<TestEventPatternResponse, RusotoError<TestEventPatternError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.TestEventPattern");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TestEventPatternError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TestEventPatternResponse, _>()
    }

    /// <p>Removes one or more tags from the specified EventBridge resource. In Amazon EventBridge (CloudWatch Events), rules and event buses can be tagged.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Updates an API destination.</p>
    async fn update_api_destination(
        &self,
        input: UpdateApiDestinationRequest,
    ) -> Result<UpdateApiDestinationResponse, RusotoError<UpdateApiDestinationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.UpdateApiDestination");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateApiDestinationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateApiDestinationResponse, _>()
    }

    /// <p>Updates the specified archive.</p>
    async fn update_archive(
        &self,
        input: UpdateArchiveRequest,
    ) -> Result<UpdateArchiveResponse, RusotoError<UpdateArchiveError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.UpdateArchive");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateArchiveError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateArchiveResponse, _>()
    }

    /// <p>Updates settings for a connection.</p>
    async fn update_connection(
        &self,
        input: UpdateConnectionRequest,
    ) -> Result<UpdateConnectionResponse, RusotoError<UpdateConnectionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSEvents.UpdateConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateConnectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateConnectionResponse, _>()
    }
}
