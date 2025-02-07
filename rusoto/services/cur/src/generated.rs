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

impl CostAndUsageReportClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "cur", &self.region, request_uri);

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
/// <p>Deletes the specified report.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReportDefinitionRequest {
    /// <p>The name of the report that you want to delete. The name must be unique, is case sensitive, and can't include spaces.</p>
    #[serde(rename = "reportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteReportDefinitionResponse {
    #[serde(rename = "responseMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message: Option<String>,
}

/// <p>Requests a list of AWS Cost and Usage reports owned by the account.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReportDefinitionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReportDefinitionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of AWS Cost and Usage reports owned by the account.</p>
    #[serde(rename = "reportDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_definitions: Option<Vec<ReportDefinition>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyReportDefinitionRequest {
    #[serde(rename = "reportDefinition")]
    pub report_definition: ReportDefinition,
    #[serde(rename = "reportName")]
    pub report_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyReportDefinitionResponse {}

/// <p>Creates a Cost and Usage Report.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutReportDefinitionRequest {
    /// <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed metadata and data file information. </p>
    #[serde(rename = "reportDefinition")]
    pub report_definition: ReportDefinition,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutReportDefinitionResponse {}

/// <p>The definition of AWS Cost and Usage Report. You can specify the report name, time unit, report format, compression format, S3 bucket, additional artifacts, and schema elements in the definition. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ReportDefinition {
    /// <p>A list of manifests that you want Amazon Web Services to create for this report.</p>
    #[serde(rename = "additionalArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_artifacts: Option<Vec<String>>,
    /// <p>A list of strings that indicate additional content that Amazon Web Services includes in the report, such as individual resource IDs. </p>
    #[serde(rename = "additionalSchemaElements")]
    pub additional_schema_elements: Vec<String>,
    /// <p> The Amazon resource name of the billing view. You can get this value by using the billing view service public APIs. </p>
    #[serde(rename = "billingViewArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "compression")]
    pub compression: String,
    #[serde(rename = "format")]
    pub format: String,
    /// <p>Whether you want Amazon Web Services to update your reports after they have been finalized if Amazon Web Services detects charges related to previous months. These charges can include refunds, credits, or support fees.</p>
    #[serde(rename = "refreshClosedReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_closed_reports: Option<bool>,
    #[serde(rename = "reportName")]
    pub report_name: String,
    /// <p>Whether you want Amazon Web Services to overwrite the previous version of each report or to deliver the report in addition to the previous versions.</p>
    #[serde(rename = "reportVersioning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_versioning: Option<String>,
    #[serde(rename = "s3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "s3Prefix")]
    pub s3_prefix: String,
    #[serde(rename = "s3Region")]
    pub s3_region: String,
    #[serde(rename = "timeUnit")]
    pub time_unit: String,
}

/// Errors returned by DeleteReportDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteReportDefinitionError {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
}

impl DeleteReportDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReportDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteReportDefinitionError::InternalError(
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
impl fmt::Display for DeleteReportDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReportDefinitionError::InternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteReportDefinitionError {}
/// Errors returned by DescribeReportDefinitions
#[derive(Debug, PartialEq)]
pub enum DescribeReportDefinitionsError {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
}

impl DescribeReportDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReportDefinitionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeReportDefinitionsError::InternalError(
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
impl fmt::Display for DescribeReportDefinitionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReportDefinitionsError::InternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeReportDefinitionsError {}
/// Errors returned by ModifyReportDefinition
#[derive(Debug, PartialEq)]
pub enum ModifyReportDefinitionError {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
}

impl ModifyReportDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyReportDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ModifyReportDefinitionError::InternalError(
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
impl fmt::Display for ModifyReportDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyReportDefinitionError::InternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyReportDefinitionError {}
/// Errors returned by PutReportDefinition
#[derive(Debug, PartialEq)]
pub enum PutReportDefinitionError {
    /// <p>A report with the specified name already exists in the account. Specify a different report name.</p>
    DuplicateReportName(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>This account already has five reports defined. To define a new report, you must delete an existing report.</p>
    ReportLimitReached(String),
}

impl PutReportDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutReportDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateReportNameException" => {
                    return RusotoError::Service(PutReportDefinitionError::DuplicateReportName(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(PutReportDefinitionError::InternalError(err.msg))
                }
                "ReportLimitReachedException" => {
                    return RusotoError::Service(PutReportDefinitionError::ReportLimitReached(
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
impl fmt::Display for PutReportDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutReportDefinitionError::DuplicateReportName(ref cause) => write!(f, "{}", cause),
            PutReportDefinitionError::InternalError(ref cause) => write!(f, "{}", cause),
            PutReportDefinitionError::ReportLimitReached(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutReportDefinitionError {}
/// Trait representing the capabilities of the AWS Cost and Usage Report Service API. AWS Cost and Usage Report Service clients implement this trait.
#[async_trait]
pub trait CostAndUsageReport {
    /// <p>Deletes the specified report.</p>
    async fn delete_report_definition(
        &self,
        input: DeleteReportDefinitionRequest,
    ) -> Result<DeleteReportDefinitionResponse, RusotoError<DeleteReportDefinitionError>>;

    /// <p>Lists the AWS Cost and Usage reports available to this account.</p>
    async fn describe_report_definitions(
        &self,
        input: DescribeReportDefinitionsRequest,
    ) -> Result<DescribeReportDefinitionsResponse, RusotoError<DescribeReportDefinitionsError>>;

    /// <p>Allows you to programatically update your report preferences.</p>
    async fn modify_report_definition(
        &self,
        input: ModifyReportDefinitionRequest,
    ) -> Result<ModifyReportDefinitionResponse, RusotoError<ModifyReportDefinitionError>>;

    /// <p>Creates a new report using the description that you provide.</p>
    async fn put_report_definition(
        &self,
        input: PutReportDefinitionRequest,
    ) -> Result<PutReportDefinitionResponse, RusotoError<PutReportDefinitionError>>;
}
/// A client for the AWS Cost and Usage Report Service API.
#[derive(Clone)]
pub struct CostAndUsageReportClient {
    client: Client,
    region: region::Region,
}

impl CostAndUsageReportClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CostAndUsageReportClient {
        CostAndUsageReportClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CostAndUsageReportClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CostAndUsageReportClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CostAndUsageReportClient {
        CostAndUsageReportClient { client, region }
    }
}

#[async_trait]
impl CostAndUsageReport for CostAndUsageReportClient {
    /// <p>Deletes the specified report.</p>
    async fn delete_report_definition(
        &self,
        input: DeleteReportDefinitionRequest,
    ) -> Result<DeleteReportDefinitionResponse, RusotoError<DeleteReportDefinitionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.DeleteReportDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteReportDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteReportDefinitionResponse, _>()
    }

    /// <p>Lists the AWS Cost and Usage reports available to this account.</p>
    async fn describe_report_definitions(
        &self,
        input: DescribeReportDefinitionsRequest,
    ) -> Result<DescribeReportDefinitionsResponse, RusotoError<DescribeReportDefinitionsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.DescribeReportDefinitions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeReportDefinitionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeReportDefinitionsResponse, _>()
    }

    /// <p>Allows you to programatically update your report preferences.</p>
    async fn modify_report_definition(
        &self,
        input: ModifyReportDefinitionRequest,
    ) -> Result<ModifyReportDefinitionResponse, RusotoError<ModifyReportDefinitionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.ModifyReportDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ModifyReportDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ModifyReportDefinitionResponse, _>()
    }

    /// <p>Creates a new report using the description that you provide.</p>
    async fn put_report_definition(
        &self,
        input: PutReportDefinitionRequest,
    ) -> Result<PutReportDefinitionResponse, RusotoError<PutReportDefinitionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.PutReportDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutReportDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutReportDefinitionResponse, _>()
    }
}
