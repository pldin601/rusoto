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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptInvitationRequest {
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The value that is used to validate the administrator account to the member account.</p>
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    /// <p>The account ID of the GuardDuty administrator account whose invitation you're accepting.</p>
    #[serde(rename = "masterId")]
    pub master_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptInvitationResponse {}

/// <p>Contains information on the current access control policies for the bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccessControlList {
    /// <p>A value that indicates whether public read access for the bucket is enabled through an Access Control List (ACL).</p>
    #[serde(rename = "allowsPublicReadAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_read_access: Option<bool>,
    /// <p>A value that indicates whether public write access for the bucket is enabled through an Access Control List (ACL).</p>
    #[serde(rename = "allowsPublicWriteAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_write_access: Option<bool>,
}

/// <p>Contains information about the access keys.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccessKeyDetails {
    /// <p>The access key ID of the user.</p>
    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// <p>The principal ID of the user.</p>
    #[serde(rename = "principalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// <p>The name of the user.</p>
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// <p>The type of the user.</p>
    #[serde(rename = "userType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
}

/// <p>Contains information about the account.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AccountDetail {
    /// <p>The member account ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The email address of the member account.</p>
    #[serde(rename = "email")]
    pub email: String,
}

/// <p>Contains information about the account level permissions on the S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccountLevelPermissions {
    /// <p>Describes the S3 Block Public Access settings of the bucket's parent account.</p>
    #[serde(rename = "blockPublicAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_access: Option<BlockPublicAccess>,
}

/// <p>Contains information about actions.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Action {
    /// <p>The GuardDuty finding activity type.</p>
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// <p>Information about the AWS_API_CALL action described in this finding.</p>
    #[serde(rename = "awsApiCallAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_call_action: Option<AwsApiCallAction>,
    /// <p>Information about the DNS_REQUEST action described in this finding.</p>
    #[serde(rename = "dnsRequestAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_request_action: Option<DnsRequestAction>,
    /// <p>Information about the NETWORK_CONNECTION action described in this finding.</p>
    #[serde(rename = "networkConnectionAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_connection_action: Option<NetworkConnectionAction>,
    /// <p>Information about the PORT_PROBE action described in this finding.</p>
    #[serde(rename = "portProbeAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_probe_action: Option<PortProbeAction>,
}

/// <p>The account within the organization specified as the GuardDuty delegated administrator.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminAccount {
    /// <p>The AWS account ID for the account.</p>
    #[serde(rename = "adminAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_account_id: Option<String>,
    /// <p>Indicates whether the account is enabled as the delegated administrator.</p>
    #[serde(rename = "adminStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ArchiveFindingsRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to archive.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The IDs of the findings that you want to archive.</p>
    #[serde(rename = "findingIds")]
    pub finding_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ArchiveFindingsResponse {}

/// <p>Contains information about the API action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AwsApiCallAction {
    /// <p>The AWS API name.</p>
    #[serde(rename = "api")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    /// <p>The AWS API caller type.</p>
    #[serde(rename = "callerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_type: Option<String>,
    /// <p>The domain information for the AWS API call.</p>
    #[serde(rename = "domainDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_details: Option<DomainDetails>,
    /// <p>The error code of the failed AWS API action.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The remote IP information of the connection that initiated the AWS API call.</p>
    #[serde(rename = "remoteIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<RemoteIpDetails>,
    /// <p>The AWS service name whose API was invoked.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

/// <p>Contains information on how the bucker owner's S3 Block Public Access settings are being applied to the S3 bucket. See <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html">S3 Block Public Access</a> for more information. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BlockPublicAccess {
    /// <p>Indicates if S3 Block Public Access is set to <code>BlockPublicAcls</code>.</p>
    #[serde(rename = "blockPublicAcls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_acls: Option<bool>,
    /// <p>Indicates if S3 Block Public Access is set to <code>BlockPublicPolicy</code>.</p>
    #[serde(rename = "blockPublicPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_policy: Option<bool>,
    /// <p>Indicates if S3 Block Public Access is set to <code>IgnorePublicAcls</code>.</p>
    #[serde(rename = "ignorePublicAcls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_public_acls: Option<bool>,
    /// <p>Indicates if S3 Block Public Access is set to <code>RestrictPublicBuckets</code>.</p>
    #[serde(rename = "restrictPublicBuckets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_public_buckets: Option<bool>,
}

/// <p>Contains information about the bucket level permissions for the S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BucketLevelPermissions {
    /// <p>Contains information on how Access Control Policies are applied to the bucket.</p>
    #[serde(rename = "accessControlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<AccessControlList>,
    /// <p>Contains information on which account level S3 Block Public Access settings are applied to the S3 bucket.</p>
    #[serde(rename = "blockPublicAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_access: Option<BlockPublicAccess>,
    /// <p>Contains information on the bucket policies for the S3 bucket.</p>
    #[serde(rename = "bucketPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_policy: Option<BucketPolicy>,
}

/// <p>Contains information on the current bucket policies for the S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BucketPolicy {
    /// <p>A value that indicates whether public read access for the bucket is enabled through a bucket policy.</p>
    #[serde(rename = "allowsPublicReadAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_read_access: Option<bool>,
    /// <p>A value that indicates whether public write access for the bucket is enabled through a bucket policy.</p>
    #[serde(rename = "allowsPublicWriteAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_write_access: Option<bool>,
}

/// <p>Contains information about the city associated with the IP address.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct City {
    /// <p>The city name of the remote IP address.</p>
    #[serde(rename = "cityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
}

/// <p>Contains information on the status of CloudTrail as a data source for the detector.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CloudTrailConfigurationResult {
    /// <p>Describes whether CloudTrail is enabled as a data source for the detector.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Contains information about the condition.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Condition {
    /// <p>Represents an <i>equal</i> <b/> condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "equals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<Vec<String>>,
    /// <p>Represents a <i>greater than</i> condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "greaterThan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<i64>,
    /// <p>Represents a <i>greater than or equal</i> condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "greaterThanOrEqual")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than_or_equal: Option<i64>,
    /// <p>Represents a <i>less than</i> condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "lessThan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than: Option<i64>,
    /// <p>Represents a <i>less than or equal</i> condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "lessThanOrEqual")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than_or_equal: Option<i64>,
    /// <p>Represents a <i>not equal</i> <b/> condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "notEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<Vec<String>>,
}

/// <p>Contains information about the country where the remote IP address is located.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Country {
    /// <p>The country code of the remote IP address.</p>
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// <p>The country name of the remote IP address.</p>
    #[serde(rename = "countryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDetectorRequest {
    /// <p>The idempotency token for the create request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Describes which data sources will be enabled for the detector.</p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<DataSourceConfigurations>,
    /// <p>A Boolean value that specifies whether the detector is to be enabled.</p>
    #[serde(rename = "enable")]
    pub enable: bool,
    /// <p>A value that specifies how frequently updated findings are exported.</p>
    #[serde(rename = "findingPublishingFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
    /// <p>The tags to be added to a new detector resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDetectorResponse {
    /// <p>The unique ID of the created detector.</p>
    #[serde(rename = "detectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFilterRequest {
    /// <p>Specifies the action that is to be applied to the findings that match the filter.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The idempotency token for the create request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The description of the filter.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the detector belonging to the GuardDuty account that you want to create a filter for.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p><p>Represents the criteria to be used in the filter for querying findings.</p> <p>You can only use the following attributes to query findings:</p> <ul> <li> <p>accountId</p> </li> <li> <p>region</p> </li> <li> <p>confidence</p> </li> <li> <p>id</p> </li> <li> <p>resource.accessKeyDetails.accessKeyId</p> </li> <li> <p>resource.accessKeyDetails.principalId</p> </li> <li> <p>resource.accessKeyDetails.userName</p> </li> <li> <p>resource.accessKeyDetails.userType</p> </li> <li> <p>resource.instanceDetails.iamInstanceProfile.id</p> </li> <li> <p>resource.instanceDetails.imageId</p> </li> <li> <p>resource.instanceDetails.instanceId</p> </li> <li> <p>resource.instanceDetails.outpostArn</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.ipv6Addresses</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.privateIpAddresses.privateIpAddress</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.publicDnsName</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.publicIp</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.securityGroups.groupId</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.securityGroups.groupName</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.subnetId</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.vpcId</p> </li> <li> <p>resource.instanceDetails.tags.key</p> </li> <li> <p>resource.instanceDetails.tags.value</p> </li> <li> <p>resource.resourceType</p> </li> <li> <p>service.action.actionType</p> </li> <li> <p>service.action.awsApiCallAction.api</p> </li> <li> <p>service.action.awsApiCallAction.callerType</p> </li> <li> <p>service.action.awsApiCallAction.errorCode</p> </li> <li> <p>service.action.awsApiCallAction.remoteIpDetails.city.cityName</p> </li> <li> <p>service.action.awsApiCallAction.remoteIpDetails.country.countryName</p> </li> <li> <p>service.action.awsApiCallAction.remoteIpDetails.ipAddressV4</p> </li> <li> <p>service.action.awsApiCallAction.remoteIpDetails.organization.asn</p> </li> <li> <p>service.action.awsApiCallAction.remoteIpDetails.organization.asnOrg</p> </li> <li> <p>service.action.awsApiCallAction.serviceName</p> </li> <li> <p>service.action.dnsRequestAction.domain</p> </li> <li> <p>service.action.networkConnectionAction.blocked</p> </li> <li> <p>service.action.networkConnectionAction.connectionDirection</p> </li> <li> <p>service.action.networkConnectionAction.localPortDetails.port</p> </li> <li> <p>service.action.networkConnectionAction.protocol</p> </li> <li> <p>service.action.networkConnectionAction.localIpDetails.ipAddressV4</p> </li> <li> <p>service.action.networkConnectionAction.remoteIpDetails.city.cityName</p> </li> <li> <p>service.action.networkConnectionAction.remoteIpDetails.country.countryName</p> </li> <li> <p>service.action.networkConnectionAction.remoteIpDetails.ipAddressV4</p> </li> <li> <p>service.action.networkConnectionAction.remoteIpDetails.organization.asn</p> </li> <li> <p>service.action.networkConnectionAction.remoteIpDetails.organization.asnOrg</p> </li> <li> <p>service.action.networkConnectionAction.remotePortDetails.port</p> </li> <li> <p>service.additionalInfo.threatListName</p> </li> <li> <p>service.archived</p> <p>When this attribute is set to TRUE, only archived findings are listed. When it&#39;s set to FALSE, only unarchived findings are listed. When this attribute is not set, all existing findings are listed.</p> </li> <li> <p>service.resourceRole</p> </li> <li> <p>severity</p> </li> <li> <p>type</p> </li> <li> <p>updatedAt</p> <p>Type: ISO 8601 string format: YYYY-MM-DDTHH:MM:SS.SSSZ or YYYY-MM-DDTHH:MM:SSZ depending on whether the value contains milliseconds.</p> </li> </ul></p>
    #[serde(rename = "findingCriteria")]
    pub finding_criteria: FindingCriteria,
    /// <p>The name of the filter. Minimum length of 3. Maximum length of 64. Valid characters include alphanumeric characters, dot (.), underscore (_), and dash (-). Spaces are not allowed.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.</p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// <p>The tags to be added to a new filter resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFilterResponse {
    /// <p>The name of the successfully created filter.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateIPSetRequest {
    /// <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded IPSet.</p>
    #[serde(rename = "activate")]
    pub activate: bool,
    /// <p>The idempotency token for the create request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The unique ID of the detector of the GuardDuty account that you want to create an IPSet for.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The format of the file that contains the IPSet.</p>
    #[serde(rename = "format")]
    pub format: String,
    /// <p>The URI of the file that contains the IPSet. For example: https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key.</p>
    #[serde(rename = "location")]
    pub location: String,
    /// <p>The user-friendly name to identify the IPSet.</p> <p> Allowed characters are alphanumerics, spaces, hyphens (-), and underscores (_).</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The tags to be added to a new IP set resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIPSetResponse {
    /// <p>The ID of the IPSet resource.</p>
    #[serde(rename = "ipSetId")]
    pub ip_set_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMembersRequest {
    /// <p>A list of account ID and email address pairs of the accounts that you want to associate with the GuardDuty administrator account.</p>
    #[serde(rename = "accountDetails")]
    pub account_details: Vec<AccountDetail>,
    /// <p>The unique ID of the detector of the GuardDuty account that you want to associate member accounts with.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMembersResponse {
    /// <p>A list of objects that include the <code>accountIds</code> of the unprocessed accounts and a result string that explains why each was unprocessed.</p>
    #[serde(rename = "unprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePublishingDestinationRequest {
    /// <p>The idempotency token for the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The properties of the publishing destination, including the ARNs for the destination and the KMS key used for encryption.</p>
    #[serde(rename = "destinationProperties")]
    pub destination_properties: DestinationProperties,
    /// <p>The type of resource for the publishing destination. Currently only Amazon S3 buckets are supported.</p>
    #[serde(rename = "destinationType")]
    pub destination_type: String,
    /// <p>The ID of the GuardDuty detector associated with the publishing destination.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePublishingDestinationResponse {
    /// <p>The ID of the publishing destination that is created.</p>
    #[serde(rename = "destinationId")]
    pub destination_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSampleFindingsRequest {
    /// <p>The ID of the detector to create sample findings for.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The types of sample findings to generate.</p>
    #[serde(rename = "findingTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_types: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSampleFindingsResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateThreatIntelSetRequest {
    /// <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded ThreatIntelSet.</p>
    #[serde(rename = "activate")]
    pub activate: bool,
    /// <p>The idempotency token for the create request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The unique ID of the detector of the GuardDuty account that you want to create a threatIntelSet for.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The format of the file that contains the ThreatIntelSet.</p>
    #[serde(rename = "format")]
    pub format: String,
    /// <p>The URI of the file that contains the ThreatIntelSet. For example: https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key.</p>
    #[serde(rename = "location")]
    pub location: String,
    /// <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The tags to be added to a new threat list resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateThreatIntelSetResponse {
    /// <p>The ID of the ThreatIntelSet resource.</p>
    #[serde(rename = "threatIntelSetId")]
    pub threat_intel_set_id: String,
}

/// <p>Contains information on the status of DNS logs as a data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DNSLogsConfigurationResult {
    /// <p>Denotes whether DNS logs is enabled as a data source.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Contains information about which data sources are enabled.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DataSourceConfigurations {
    /// <p>Describes whether S3 data event logs are enabled as a data source.</p>
    #[serde(rename = "s3Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<S3LogsConfiguration>,
}

/// <p>Contains information on the status of data sources for the detector.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSourceConfigurationsResult {
    /// <p>An object that contains information on the status of CloudTrail as a data source.</p>
    #[serde(rename = "cloudTrail")]
    pub cloud_trail: CloudTrailConfigurationResult,
    /// <p>An object that contains information on the status of DNS logs as a data source.</p>
    #[serde(rename = "dNSLogs")]
    pub dns_logs: DNSLogsConfigurationResult,
    /// <p>An object that contains information on the status of VPC flow logs as a data source.</p>
    #[serde(rename = "flowLogs")]
    pub flow_logs: FlowLogsConfigurationResult,
    /// <p>An object that contains information on the status of S3 Data event logs as a data source.</p>
    #[serde(rename = "s3Logs")]
    pub s3_logs: S3LogsConfigurationResult,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeclineInvitationsRequest {
    /// <p>A list of account IDs of the AWS accounts that sent invitations to the current member account that you want to decline invitations from.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeclineInvitationsResponse {
    /// <p>A list of objects that contain the unprocessed account and a result string that explains why it was unprocessed.</p>
    #[serde(rename = "unprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

/// <p>Contains information on the server side encryption method used in the S3 bucket. See <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/serv-side-encryption.html">S3 Server-Side Encryption</a> for more information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DefaultServerSideEncryption {
    /// <p>The type of encryption used for objects within the S3 bucket.</p>
    #[serde(rename = "encryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the KMS encryption key. Only available if the bucket <code>EncryptionType</code> is <code>aws:kms</code>.</p>
    #[serde(rename = "kmsMasterKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDetectorRequest {
    /// <p>The unique ID of the detector that you want to delete.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDetectorResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFilterRequest {
    /// <p>The unique ID of the detector that the filter is associated with.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The name of the filter that you want to delete.</p>
    #[serde(rename = "filterName")]
    pub filter_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFilterResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIPSetRequest {
    /// <p>The unique ID of the detector associated with the IPSet.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The unique ID of the IPSet to delete.</p>
    #[serde(rename = "ipSetId")]
    pub ip_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteIPSetResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInvitationsRequest {
    /// <p>A list of account IDs of the AWS accounts that sent invitations to the current member account that you want to delete invitations from.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInvitationsResponse {
    /// <p>A list of objects that contain the unprocessed account and a result string that explains why it was unprocessed.</p>
    #[serde(rename = "unprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMembersRequest {
    /// <p>A list of account IDs of the GuardDuty member accounts that you want to delete.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account whose members you want to delete.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMembersResponse {
    /// <p>The accounts that could not be processed.</p>
    #[serde(rename = "unprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePublishingDestinationRequest {
    /// <p>The ID of the publishing destination to delete.</p>
    #[serde(rename = "destinationId")]
    pub destination_id: String,
    /// <p>The unique ID of the detector associated with the publishing destination to delete.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePublishingDestinationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteThreatIntelSetRequest {
    /// <p>The unique ID of the detector that the threatIntelSet is associated with.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The unique ID of the threatIntelSet that you want to delete.</p>
    #[serde(rename = "threatIntelSetId")]
    pub threat_intel_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteThreatIntelSetResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOrganizationConfigurationRequest {
    /// <p>The ID of the detector to retrieve information about the delegated administrator from.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOrganizationConfigurationResponse {
    /// <p>Indicates whether GuardDuty is automatically enabled for accounts added to the organization.</p>
    #[serde(rename = "autoEnable")]
    pub auto_enable: bool,
    /// <p>Describes which data sources are enabled automatically for member accounts.</p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<OrganizationDataSourceConfigurationsResult>,
    /// <p>Indicates whether the maximum number of allowed member accounts are already associated with the delegated administrator account for your organization.</p>
    #[serde(rename = "memberAccountLimitReached")]
    pub member_account_limit_reached: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePublishingDestinationRequest {
    /// <p>The ID of the publishing destination to retrieve.</p>
    #[serde(rename = "destinationId")]
    pub destination_id: String,
    /// <p>The unique ID of the detector associated with the publishing destination to retrieve.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePublishingDestinationResponse {
    /// <p>The ID of the publishing destination.</p>
    #[serde(rename = "destinationId")]
    pub destination_id: String,
    /// <p>A <code>DestinationProperties</code> object that includes the <code>DestinationArn</code> and <code>KmsKeyArn</code> of the publishing destination.</p>
    #[serde(rename = "destinationProperties")]
    pub destination_properties: DestinationProperties,
    /// <p>The type of publishing destination. Currently, only Amazon S3 buckets are supported.</p>
    #[serde(rename = "destinationType")]
    pub destination_type: String,
    /// <p>The time, in epoch millisecond format, at which GuardDuty was first unable to publish findings to the destination.</p>
    #[serde(rename = "publishingFailureStartTimestamp")]
    pub publishing_failure_start_timestamp: i64,
    /// <p>The status of the publishing destination.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Contains information about the publishing destination, including the ID, type, and status.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Destination {
    /// <p>The unique ID of the publishing destination.</p>
    #[serde(rename = "destinationId")]
    pub destination_id: String,
    /// <p>The type of resource used for the publishing destination. Currently, only Amazon S3 buckets are supported.</p>
    #[serde(rename = "destinationType")]
    pub destination_type: String,
    /// <p>The status of the publishing destination.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Contains the Amazon Resource Name (ARN) of the resource to publish to, such as an S3 bucket, and the ARN of the KMS key to use to encrypt published findings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DestinationProperties {
    /// <p>The ARN of the resource to publish to.</p>
    #[serde(rename = "destinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    /// <p>The ARN of the KMS key to use for encryption.</p>
    #[serde(rename = "kmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableOrganizationAdminAccountRequest {
    /// <p>The AWS Account ID for the organizations account to be disabled as a GuardDuty delegated administrator.</p>
    #[serde(rename = "adminAccountId")]
    pub admin_account_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableOrganizationAdminAccountResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateFromMasterAccountRequest {
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateFromMasterAccountResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateMembersRequest {
    /// <p>A list of account IDs of the GuardDuty member accounts that you want to disassociate from the administrator account.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account whose members you want to disassociate from the administrator account.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateMembersResponse {
    /// <p>A list of objects that contain the unprocessed account and a result string that explains why it was unprocessed.</p>
    #[serde(rename = "unprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

/// <p>Contains information about the DNS_REQUEST action described in this finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DnsRequestAction {
    /// <p>The domain information for the API request.</p>
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

/// <p>Contains information about the domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainDetails {
    /// <p>The domain information for the AWS API call.</p>
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableOrganizationAdminAccountRequest {
    /// <p>The AWS Account ID for the organization account to be enabled as a GuardDuty delegated administrator.</p>
    #[serde(rename = "adminAccountId")]
    pub admin_account_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableOrganizationAdminAccountResponse {}

/// <p>Contains information about the reason that the finding was generated.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Evidence {
    /// <p>A list of threat intelligence details related to the evidence.</p>
    #[serde(rename = "threatIntelligenceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intelligence_details: Option<Vec<ThreatIntelligenceDetail>>,
}

/// <p>Contains information about the finding, which is generated when abnormal or suspicious activity is detected.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Finding {
    /// <p>The ID of the account in which the finding was generated.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The ARN of the finding.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The confidence score for the finding.</p>
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    /// <p>The time and date when the finding was created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// <p>The description of the finding.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the finding.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The partition associated with the finding.</p>
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    /// <p>The Region where the finding was generated.</p>
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "resource")]
    pub resource: Resource,
    /// <p>The version of the schema used for the finding.</p>
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    /// <p>The severity of the finding.</p>
    #[serde(rename = "severity")]
    pub severity: f64,
    /// <p>The title of the finding.</p>
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The type of finding.</p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p>The time and date when the finding was last updated.</p>
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// <p>Contains information about the criteria used for querying findings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FindingCriteria {
    /// <p>Represents a map of finding properties that match specified conditions and values when querying findings.</p>
    #[serde(rename = "criterion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criterion: Option<::std::collections::HashMap<String, Condition>>,
}

/// <p>Contains information about finding statistics.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FindingStatistics {
    /// <p>Represents a map of severity to count statistics for a set of findings.</p>
    #[serde(rename = "countBySeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_by_severity: Option<::std::collections::HashMap<String, i64>>,
}

/// <p>Contains information on the status of VPC flow logs as a data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FlowLogsConfigurationResult {
    /// <p>Denotes whether VPC flow logs is enabled as a data source.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Contains information about the location of the remote IP address.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GeoLocation {
    /// <p>The latitude information of the remote IP address.</p>
    #[serde(rename = "lat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    /// <p>The longitude information of the remote IP address.</p>
    #[serde(rename = "lon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDetectorRequest {
    /// <p>The unique ID of the detector that you want to get.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDetectorResponse {
    /// <p>The timestamp of when the detector was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>Describes which data sources are enabled for the detector.</p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<DataSourceConfigurationsResult>,
    /// <p>The publishing frequency of the finding.</p>
    #[serde(rename = "findingPublishingFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
    /// <p>The GuardDuty service role.</p>
    #[serde(rename = "serviceRole")]
    pub service_role: String,
    /// <p>The detector status.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>The tags of the detector resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The last-updated timestamp for the detector.</p>
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFilterRequest {
    /// <p>The unique ID of the detector that the filter is associated with.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The name of the filter you want to get.</p>
    #[serde(rename = "filterName")]
    pub filter_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFilterResponse {
    /// <p>Specifies the action that is to be applied to the findings that match the filter.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>The description of the filter.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Represents the criteria to be used in the filter for querying findings.</p>
    #[serde(rename = "findingCriteria")]
    pub finding_criteria: FindingCriteria,
    /// <p>The name of the filter.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.</p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// <p>The tags of the filter resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFindingsRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to retrieve.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The IDs of the findings that you want to retrieve.</p>
    #[serde(rename = "findingIds")]
    pub finding_ids: Vec<String>,
    /// <p>Represents the criteria used for sorting findings.</p>
    #[serde(rename = "sortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SortCriteria>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFindingsResponse {
    /// <p>A list of findings.</p>
    #[serde(rename = "findings")]
    pub findings: Vec<Finding>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFindingsStatisticsRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings' statistics you want to retrieve.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>Represents the criteria that is used for querying findings.</p>
    #[serde(rename = "findingCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    /// <p>The types of finding statistics to retrieve.</p>
    #[serde(rename = "findingStatisticTypes")]
    pub finding_statistic_types: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFindingsStatisticsResponse {
    /// <p>The finding statistics object.</p>
    #[serde(rename = "findingStatistics")]
    pub finding_statistics: FindingStatistics,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIPSetRequest {
    /// <p>The unique ID of the detector that the IPSet is associated with.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The unique ID of the IPSet to retrieve.</p>
    #[serde(rename = "ipSetId")]
    pub ip_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetIPSetResponse {
    /// <p>The format of the file that contains the IPSet.</p>
    #[serde(rename = "format")]
    pub format: String,
    /// <p>The URI of the file that contains the IPSet. For example: https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key.</p>
    #[serde(rename = "location")]
    pub location: String,
    /// <p>The user-friendly name for the IPSet.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The status of IPSet file that was uploaded.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>The tags of the IPSet resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInvitationsCountRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInvitationsCountResponse {
    /// <p>The number of received invitations.</p>
    #[serde(rename = "invitationsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations_count: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMasterAccountRequest {
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMasterAccountResponse {
    /// <p>The administrator account details.</p>
    #[serde(rename = "master")]
    pub master: Master,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMemberDetectorsRequest {
    /// <p>The account ID of the member account.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// <p>The detector ID for the administrator account.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMemberDetectorsResponse {
    /// <p>An object that describes which data sources are enabled for a member account.</p>
    #[serde(rename = "memberDataSourceConfigurations")]
    pub member_data_source_configurations: Vec<MemberDataSourceConfiguration>,
    /// <p>A list of member account IDs that were unable to be processed along with an explanation for why they were not processed.</p>
    #[serde(rename = "unprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMembersRequest {
    /// <p>A list of account IDs of the GuardDuty member accounts that you want to describe.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account whose members you want to retrieve.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMembersResponse {
    /// <p>A list of members.</p>
    #[serde(rename = "members")]
    pub members: Vec<Member>,
    /// <p>A list of objects that contain the unprocessed account and a result string that explains why it was unprocessed.</p>
    #[serde(rename = "unprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetThreatIntelSetRequest {
    /// <p>The unique ID of the detector that the threatIntelSet is associated with.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The unique ID of the threatIntelSet that you want to get.</p>
    #[serde(rename = "threatIntelSetId")]
    pub threat_intel_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetThreatIntelSetResponse {
    /// <p>The format of the threatIntelSet.</p>
    #[serde(rename = "format")]
    pub format: String,
    /// <p>The URI of the file that contains the ThreatIntelSet. For example: https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key.</p>
    #[serde(rename = "location")]
    pub location: String,
    /// <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The status of threatIntelSet file uploaded.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>The tags of the threat list resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUsageStatisticsRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service whose usage statistics you want to retrieve.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. For subsequent calls, use the NextToken value returned from the previous request to continue listing results after the first page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The currency unit you would like to view your usage statistics in. Current valid values are USD.</p>
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>Represents the criteria used for querying usage.</p>
    #[serde(rename = "usageCriteria")]
    pub usage_criteria: UsageCriteria,
    /// <p>The type of usage statistics to retrieve.</p>
    #[serde(rename = "usageStatisticType")]
    pub usage_statistic_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUsageStatisticsResponse {
    /// <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The usage statistics object. If a UsageStatisticType was provided, the objects representing other types will be null.</p>
    #[serde(rename = "usageStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_statistics: Option<UsageStatistics>,
}

/// <p>Contains information about the EC2 instance profile.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IamInstanceProfile {
    /// <p>The profile ARN of the EC2 instance.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The profile ID of the EC2 instance.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Contains information about the details of an instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceDetails {
    /// <p>The Availability Zone of the EC2 instance.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The profile information of the EC2 instance.</p>
    #[serde(rename = "iamInstanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<IamInstanceProfile>,
    /// <p>The image description of the EC2 instance.</p>
    #[serde(rename = "imageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_description: Option<String>,
    /// <p>The image ID of the EC2 instance.</p>
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// <p>The ID of the EC2 instance.</p>
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The state of the EC2 instance.</p>
    #[serde(rename = "instanceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_state: Option<String>,
    /// <p>The type of the EC2 instance.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The launch time of the EC2 instance.</p>
    #[serde(rename = "launchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<String>,
    /// <p>The elastic network interface information of the EC2 instance.</p>
    #[serde(rename = "networkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Outpost. Only applicable to AWS Outposts instances.</p>
    #[serde(rename = "outpostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    /// <p>The platform of the EC2 instance.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The product code of the EC2 instance.</p>
    #[serde(rename = "productCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_codes: Option<Vec<ProductCode>>,
    /// <p>The tags of the EC2 instance.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Contains information about the invitation to become a member account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Invitation {
    /// <p>The ID of the account that the invitation was sent from.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The ID of the invitation. This value is used to validate the inviter account to the member account.</p>
    #[serde(rename = "invitationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    /// <p>The timestamp when the invitation was sent.</p>
    #[serde(rename = "invitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    /// <p>The status of the relationship between the inviter and invitee accounts.</p>
    #[serde(rename = "relationshipStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InviteMembersRequest {
    /// <p>A list of account IDs of the accounts that you want to invite to GuardDuty as members.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account that you want to invite members with.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>A Boolean value that specifies whether you want to disable email notification to the accounts that you are inviting to GuardDuty as members.</p>
    #[serde(rename = "disableEmailNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_email_notification: Option<bool>,
    /// <p>The invitation message that you want to send to the accounts that you're inviting to GuardDuty as members.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InviteMembersResponse {
    /// <p>A list of objects that contain the unprocessed account and a result string that explains why it was unprocessed.</p>
    #[serde(rename = "unprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDetectorsRequest {
    /// <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action, fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDetectorsResponse {
    /// <p>A list of detector IDs.</p>
    #[serde(rename = "detectorIds")]
    pub detector_ids: Vec<String>,
    /// <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFiltersRequest {
    /// <p>The unique ID of the detector that the filter is associated with.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action, fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFiltersResponse {
    /// <p>A list of filter names.</p>
    #[serde(rename = "filterNames")]
    pub filter_names: Vec<String>,
    /// <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFindingsRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to list.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p><p>Represents the criteria used for querying findings. Valid values include:</p> <ul> <li> <p>JSON field name</p> </li> <li> <p>accountId</p> </li> <li> <p>region</p> </li> <li> <p>confidence</p> </li> <li> <p>id</p> </li> <li> <p>resource.accessKeyDetails.accessKeyId</p> </li> <li> <p>resource.accessKeyDetails.principalId</p> </li> <li> <p>resource.accessKeyDetails.userName</p> </li> <li> <p>resource.accessKeyDetails.userType</p> </li> <li> <p>resource.instanceDetails.iamInstanceProfile.id</p> </li> <li> <p>resource.instanceDetails.imageId</p> </li> <li> <p>resource.instanceDetails.instanceId</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.ipv6Addresses</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.privateIpAddresses.privateIpAddress</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.publicDnsName</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.publicIp</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.securityGroups.groupId</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.securityGroups.groupName</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.subnetId</p> </li> <li> <p>resource.instanceDetails.networkInterfaces.vpcId</p> </li> <li> <p>resource.instanceDetails.tags.key</p> </li> <li> <p>resource.instanceDetails.tags.value</p> </li> <li> <p>resource.resourceType</p> </li> <li> <p>service.action.actionType</p> </li> <li> <p>service.action.awsApiCallAction.api</p> </li> <li> <p>service.action.awsApiCallAction.callerType</p> </li> <li> <p>service.action.awsApiCallAction.remoteIpDetails.city.cityName</p> </li> <li> <p>service.action.awsApiCallAction.remoteIpDetails.country.countryName</p> </li> <li> <p>service.action.awsApiCallAction.remoteIpDetails.ipAddressV4</p> </li> <li> <p>service.action.awsApiCallAction.remoteIpDetails.organization.asn</p> </li> <li> <p>service.action.awsApiCallAction.remoteIpDetails.organization.asnOrg</p> </li> <li> <p>service.action.awsApiCallAction.serviceName</p> </li> <li> <p>service.action.dnsRequestAction.domain</p> </li> <li> <p>service.action.networkConnectionAction.blocked</p> </li> <li> <p>service.action.networkConnectionAction.connectionDirection</p> </li> <li> <p>service.action.networkConnectionAction.localPortDetails.port</p> </li> <li> <p>service.action.networkConnectionAction.protocol</p> </li> <li> <p>service.action.networkConnectionAction.remoteIpDetails.city.cityName</p> </li> <li> <p>service.action.networkConnectionAction.remoteIpDetails.country.countryName</p> </li> <li> <p>service.action.networkConnectionAction.remoteIpDetails.ipAddressV4</p> </li> <li> <p>service.action.networkConnectionAction.remoteIpDetails.organization.asn</p> </li> <li> <p>service.action.networkConnectionAction.remoteIpDetails.organization.asnOrg</p> </li> <li> <p>service.action.networkConnectionAction.remotePortDetails.port</p> </li> <li> <p>service.additionalInfo.threatListName</p> </li> <li> <p>service.archived</p> <p>When this attribute is set to &#39;true&#39;, only archived findings are listed. When it&#39;s set to &#39;false&#39;, only unarchived findings are listed. When this attribute is not set, all existing findings are listed.</p> </li> <li> <p>service.resourceRole</p> </li> <li> <p>severity</p> </li> <li> <p>type</p> </li> <li> <p>updatedAt</p> <p>Type: Timestamp in Unix Epoch millisecond format: 1486685375000</p> </li> </ul></p>
    #[serde(rename = "findingCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action, fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Represents the criteria used for sorting findings.</p>
    #[serde(rename = "sortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SortCriteria>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFindingsResponse {
    /// <p>The IDs of the findings that you're listing.</p>
    #[serde(rename = "findingIds")]
    pub finding_ids: Vec<String>,
    /// <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIPSetsRequest {
    /// <p>The unique ID of the detector that the IPSet is associated with.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action, fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListIPSetsResponse {
    /// <p>The IDs of the IPSet resources.</p>
    #[serde(rename = "ipSetIds")]
    pub ip_set_ids: Vec<String>,
    /// <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInvitationsRequest {
    /// <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action, fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInvitationsResponse {
    /// <p>A list of invitation descriptions.</p>
    #[serde(rename = "invitations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<Invitation>>,
    /// <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMembersRequest {
    /// <p>The unique ID of the detector the member is associated with.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action, fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies whether to only return associated members or to return all members (including members who haven't been invited yet or have been disassociated).</p>
    #[serde(rename = "onlyAssociated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_associated: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMembersResponse {
    /// <p>A list of members.</p>
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    /// <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOrganizationAdminAccountsRequest {
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. For subsequent calls, use the <code>NextToken</code> value returned from the previous request to continue listing results after the first page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOrganizationAdminAccountsResponse {
    /// <p>A list of accounts configured as GuardDuty delegated administrators.</p>
    #[serde(rename = "adminAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_accounts: Option<Vec<AdminAccount>>,
    /// <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPublishingDestinationsRequest {
    /// <p>The ID of the detector to retrieve publishing destinations for.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. For subsequent calls, use the <code>NextToken</code> value returned from the previous request to continue listing results after the first page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPublishingDestinationsResponse {
    /// <p>A <code>Destinations</code> object that includes information about each publishing destination returned.</p>
    #[serde(rename = "destinations")]
    pub destinations: Vec<Destination>,
    /// <p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. For subsequent calls, use the <code>NextToken</code> value returned from the previous request to continue listing results after the first page.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the given GuardDuty resource. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags associated with the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListThreatIntelSetsRequest {
    /// <p>The unique ID of the detector that the threatIntelSet is associated with.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter to paginate results in the response. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action, fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListThreatIntelSetsResponse {
    /// <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The IDs of the ThreatIntelSet resources.</p>
    #[serde(rename = "threatIntelSetIds")]
    pub threat_intel_set_ids: Vec<String>,
}

/// <p>Contains information about the local IP address of the connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LocalIpDetails {
    /// <p>The IPv4 local address of the connection.</p>
    #[serde(rename = "ipAddressV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v4: Option<String>,
}

/// <p>Contains information about the port for the local connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LocalPortDetails {
    /// <p>The port number of the local connection.</p>
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The port name of the local connection.</p>
    #[serde(rename = "portName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

/// <p>Contains information about the administrator account and invitation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Master {
    /// <p>The ID of the account used as the administrator account.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The value used to validate the administrator account to the member account.</p>
    #[serde(rename = "invitationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    /// <p>The timestamp when the invitation was sent.</p>
    #[serde(rename = "invitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    /// <p>The status of the relationship between the administrator and member accounts.</p>
    #[serde(rename = "relationshipStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
}

/// <p>Contains information about the member account. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Member {
    /// <p>The ID of the member account.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The detector ID of the member account.</p>
    #[serde(rename = "detectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    /// <p>The email address of the member account.</p>
    #[serde(rename = "email")]
    pub email: String,
    /// <p>The timestamp when the invitation was sent.</p>
    #[serde(rename = "invitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    /// <p>The administrator account ID.</p>
    #[serde(rename = "masterId")]
    pub master_id: String,
    /// <p>The status of the relationship between the member and the administrator.</p>
    #[serde(rename = "relationshipStatus")]
    pub relationship_status: String,
    /// <p>The last-updated timestamp of the member.</p>
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// <p>Contains information on which data sources are enabled for a member account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MemberDataSourceConfiguration {
    /// <p>The account ID for the member account.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>Contains information on the status of data sources for the account.</p>
    #[serde(rename = "dataSources")]
    pub data_sources: DataSourceConfigurationsResult,
}

/// <p>Contains information about the NETWORK_CONNECTION action described in the finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkConnectionAction {
    /// <p>Indicates whether EC2 blocked the network connection to your instance.</p>
    #[serde(rename = "blocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    /// <p>The network connection direction.</p>
    #[serde(rename = "connectionDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_direction: Option<String>,
    /// <p>The local IP information of the connection.</p>
    #[serde(rename = "localIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip_details: Option<LocalIpDetails>,
    /// <p>The local port information of the connection.</p>
    #[serde(rename = "localPortDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port_details: Option<LocalPortDetails>,
    /// <p>The network connection protocol.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The remote IP information of the connection.</p>
    #[serde(rename = "remoteIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<RemoteIpDetails>,
    /// <p>The remote port information of the connection.</p>
    #[serde(rename = "remotePortDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_port_details: Option<RemotePortDetails>,
}

/// <p>Contains information about the elastic network interface of the EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkInterface {
    /// <p>A list of IPv6 addresses for the EC2 instance.</p>
    #[serde(rename = "ipv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_addresses: Option<Vec<String>>,
    /// <p>The ID of the network interface.</p>
    #[serde(rename = "networkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The private DNS name of the EC2 instance.</p>
    #[serde(rename = "privateDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// <p>The private IP address of the EC2 instance.</p>
    #[serde(rename = "privateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// <p>Other private IP address information of the EC2 instance.</p>
    #[serde(rename = "privateIpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_addresses: Option<Vec<PrivateIpAddressDetails>>,
    /// <p>The public DNS name of the EC2 instance.</p>
    #[serde(rename = "publicDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_dns_name: Option<String>,
    /// <p>The public IP address of the EC2 instance.</p>
    #[serde(rename = "publicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// <p>The security groups associated with the EC2 instance.</p>
    #[serde(rename = "securityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<SecurityGroup>>,
    /// <p>The subnet ID of the EC2 instance.</p>
    #[serde(rename = "subnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The VPC ID of the EC2 instance.</p>
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Contains information about the ISP organization of the remote IP address.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Organization {
    /// <p>The Autonomous System Number (ASN) of the internet provider of the remote IP address.</p>
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<String>,
    /// <p>The organization that registered this ASN.</p>
    #[serde(rename = "asnOrg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_org: Option<String>,
    /// <p>The ISP information for the internet provider.</p>
    #[serde(rename = "isp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
    /// <p>The name of the internet provider.</p>
    #[serde(rename = "org")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
}

/// <p>An object that contains information on which data sources will be configured to be automatically enabled for new members within the organization.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OrganizationDataSourceConfigurations {
    /// <p>Describes whether S3 data event logs are enabled for new members of the organization.</p>
    #[serde(rename = "s3Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<OrganizationS3LogsConfiguration>,
}

/// <p>An object that contains information on which data sources are automatically enabled for new members within the organization.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationDataSourceConfigurationsResult {
    /// <p>Describes whether S3 data event logs are enabled as a data source.</p>
    #[serde(rename = "s3Logs")]
    pub s3_logs: OrganizationS3LogsConfigurationResult,
}

/// <p>Describes whether S3 data event logs will be automatically enabled for new members of the organization.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OrganizationS3LogsConfiguration {
    /// <p>A value that contains information on whether S3 data event logs will be enabled automatically as a data source for the organization.</p>
    #[serde(rename = "autoEnable")]
    pub auto_enable: bool,
}

/// <p>The current configuration of S3 data event logs as a data source for the organization.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationS3LogsConfigurationResult {
    /// <p>A value that describes whether S3 data event logs are automatically enabled for new members of the organization.</p>
    #[serde(rename = "autoEnable")]
    pub auto_enable: bool,
}

/// <p>Contains information on the owner of the bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Owner {
    /// <p>The canonical user ID of the bucket owner. For information about locating your canonical user ID see <a href="https://docs.aws.amazon.com/general/latest/gr/acct-identifiers.html#FindingCanonicalId">Finding Your Account Canonical User ID.</a> </p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Contains information about how permissions are configured for the S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PermissionConfiguration {
    /// <p>Contains information about the account level permissions on the S3 bucket.</p>
    #[serde(rename = "accountLevelPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_level_permissions: Option<AccountLevelPermissions>,
    /// <p>Contains information about the bucket level permissions for the S3 bucket.</p>
    #[serde(rename = "bucketLevelPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_level_permissions: Option<BucketLevelPermissions>,
}

/// <p>Contains information about the PORT_PROBE action described in the finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PortProbeAction {
    /// <p>Indicates whether EC2 blocked the port probe to the instance, such as with an ACL.</p>
    #[serde(rename = "blocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    /// <p>A list of objects related to port probe details.</p>
    #[serde(rename = "portProbeDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_probe_details: Option<Vec<PortProbeDetail>>,
}

/// <p>Contains information about the port probe details.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PortProbeDetail {
    /// <p>The local IP information of the connection.</p>
    #[serde(rename = "localIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip_details: Option<LocalIpDetails>,
    /// <p>The local port information of the connection.</p>
    #[serde(rename = "localPortDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port_details: Option<LocalPortDetails>,
    /// <p>The remote IP information of the connection.</p>
    #[serde(rename = "remoteIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<RemoteIpDetails>,
}

/// <p>Contains other private IP address information of the EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PrivateIpAddressDetails {
    /// <p>The private DNS name of the EC2 instance.</p>
    #[serde(rename = "privateDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// <p>The private IP address of the EC2 instance.</p>
    #[serde(rename = "privateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}

/// <p>Contains information about the product code for the EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProductCode {
    /// <p>The product code information.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The product code type.</p>
    #[serde(rename = "productType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
}

/// <p>Describes the public access policies that apply to the S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PublicAccess {
    /// <p>Describes the effective permission on this bucket after factoring all attached policies.</p>
    #[serde(rename = "effectivePermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_permission: Option<String>,
    /// <p>Contains information about how permissions are configured for the S3 bucket.</p>
    #[serde(rename = "permissionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_configuration: Option<PermissionConfiguration>,
}

/// <p>Contains information about the remote IP address of the connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoteIpDetails {
    /// <p>The city information of the remote IP address.</p>
    #[serde(rename = "city")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<City>,
    /// <p>The country code of the remote IP address.</p>
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    /// <p>The location information of the remote IP address.</p>
    #[serde(rename = "geoLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_location: Option<GeoLocation>,
    /// <p>The IPv4 remote address of the connection.</p>
    #[serde(rename = "ipAddressV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v4: Option<String>,
    /// <p>The ISP organization information of the remote IP address.</p>
    #[serde(rename = "organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
}

/// <p>Contains information about the remote port.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemotePortDetails {
    /// <p>The port number of the remote connection.</p>
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The port name of the remote connection.</p>
    #[serde(rename = "portName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

/// <p>Contains information about the AWS resource associated with the activity that prompted GuardDuty to generate a finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Resource {
    /// <p>The IAM access key details (IAM user information) of a user that engaged in the activity that prompted GuardDuty to generate a finding.</p>
    #[serde(rename = "accessKeyDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_details: Option<AccessKeyDetails>,
    /// <p>The information about the EC2 instance associated with the activity that prompted GuardDuty to generate a finding.</p>
    #[serde(rename = "instanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_details: Option<InstanceDetails>,
    /// <p>The type of AWS resource.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Contains information on the S3 bucket.</p>
    #[serde(rename = "s3BucketDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_details: Option<Vec<S3BucketDetail>>,
}

/// <p>Contains information on the S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3BucketDetail {
    /// <p>The Amazon Resource Name (ARN) of the S3 bucket.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time the bucket was created at.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>Describes the server side encryption method used in the S3 bucket.</p>
    #[serde(rename = "defaultServerSideEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_server_side_encryption: Option<DefaultServerSideEncryption>,
    /// <p>The name of the S3 bucket.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the S3 bucket.</p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    /// <p>Describes the public access policies that apply to the S3 bucket.</p>
    #[serde(rename = "publicAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<PublicAccess>,
    /// <p>All tags attached to the S3 bucket</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Describes whether the bucket is a source or destination bucket.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Describes whether S3 data event logs will be enabled as a data source.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3LogsConfiguration {
    /// <p> The status of S3 data event logs as a data source.</p>
    #[serde(rename = "enable")]
    pub enable: bool,
}

/// <p>Describes whether S3 data event logs will be enabled as a data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3LogsConfigurationResult {
    /// <p>A value that describes whether S3 data event logs are automatically enabled for new members of the organization.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Contains information about the security groups associated with the EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityGroup {
    /// <p>The security group ID of the EC2 instance.</p>
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The security group name of the EC2 instance.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>Contains additional information about the generated finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Service {
    /// <p>Information about the activity that is described in a finding.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// <p>Indicates whether this finding is archived.</p>
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// <p>The total count of the occurrences of this finding type.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The detector ID for the GuardDuty service.</p>
    #[serde(rename = "detectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    /// <p>The first-seen timestamp of the activity that prompted GuardDuty to generate this finding.</p>
    #[serde(rename = "eventFirstSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_first_seen: Option<String>,
    /// <p>The last-seen timestamp of the activity that prompted GuardDuty to generate this finding.</p>
    #[serde(rename = "eventLastSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_last_seen: Option<String>,
    /// <p>An evidence object associated with the service.</p>
    #[serde(rename = "evidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Evidence>,
    /// <p>The resource role information for this finding.</p>
    #[serde(rename = "resourceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_role: Option<String>,
    /// <p>The name of the AWS service (GuardDuty) that generated a finding.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>Feedback that was submitted about the finding.</p>
    #[serde(rename = "userFeedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_feedback: Option<String>,
}

/// <p>Contains information about the criteria used for sorting findings.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SortCriteria {
    /// <p>Represents the finding attribute (for example, accountId) to sort findings by.</p>
    #[serde(rename = "attributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The order by which the sorted findings are to be displayed.</p>
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartMonitoringMembersRequest {
    /// <p>A list of account IDs of the GuardDuty member accounts to start monitoring.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty administrator account associated with the member accounts to monitor.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartMonitoringMembersResponse {
    /// <p>A list of objects that contain the unprocessed account and a result string that explains why it was unprocessed.</p>
    #[serde(rename = "unprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopMonitoringMembersRequest {
    /// <p>A list of account IDs for the member accounts to stop monitoring.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector associated with the GuardDuty administrator account that is monitoring member accounts.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopMonitoringMembersResponse {
    /// <p>A list of objects that contain an accountId for each account that could not be processed, and a result string that indicates why the account was not processed. </p>
    #[serde(rename = "unprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

/// <p>Contains information about a tag associated with the EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Tag {
    /// <p>The EC2 instance tag key.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The EC2 instance tag value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the GuardDuty resource to apply a tag to.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to be added to a resource.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>An instance of a threat intelligence detail that constitutes evidence for the finding.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ThreatIntelligenceDetail {
    /// <p>The name of the threat intelligence list that triggered the finding.</p>
    #[serde(rename = "threatListName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_list_name: Option<String>,
    /// <p>A list of names of the threats in the threat intelligence list that triggered the finding.</p>
    #[serde(rename = "threatNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_names: Option<Vec<String>>,
}

/// <p>Contains the total usage with the corresponding currency unit for that value.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Total {
    /// <p>The total usage.</p>
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// <p>The currency unit that the amount is given in.</p>
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UnarchiveFindingsRequest {
    /// <p>The ID of the detector associated with the findings to unarchive.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The IDs of the findings to unarchive.</p>
    #[serde(rename = "findingIds")]
    pub finding_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnarchiveFindingsResponse {}

/// <p>Contains information about the accounts that weren't processed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnprocessedAccount {
    /// <p>The AWS account ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>A reason why the account hasn't been processed.</p>
    #[serde(rename = "result")]
    pub result: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the resource to remove tags from.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys to remove from the resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDetectorRequest {
    /// <p>Describes which data sources will be updated.</p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<DataSourceConfigurations>,
    /// <p>The unique ID of the detector to update.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>Specifies whether the detector is enabled or not enabled.</p>
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// <p>An enum value that specifies how frequently findings are exported, such as to CloudWatch Events.</p>
    #[serde(rename = "findingPublishingFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDetectorResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFilterRequest {
    /// <p>Specifies the action that is to be applied to the findings that match the filter.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The description of the filter.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique ID of the detector that specifies the GuardDuty service where you want to update a filter.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The name of the filter.</p>
    #[serde(rename = "filterName")]
    pub filter_name: String,
    /// <p>Represents the criteria to be used in the filter for querying findings.</p>
    #[serde(rename = "findingCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    /// <p>Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.</p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFilterResponse {
    /// <p>The name of the filter.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFindingsFeedbackRequest {
    /// <p>Additional feedback about the GuardDuty findings.</p>
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// <p>The ID of the detector associated with the findings to update feedback for.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The feedback for the finding.</p>
    #[serde(rename = "feedback")]
    pub feedback: String,
    /// <p>The IDs of the findings that you want to mark as useful or not useful.</p>
    #[serde(rename = "findingIds")]
    pub finding_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFindingsFeedbackResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateIPSetRequest {
    /// <p>The updated Boolean value that specifies whether the IPSet is active or not.</p>
    #[serde(rename = "activate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate: Option<bool>,
    /// <p>The detectorID that specifies the GuardDuty service whose IPSet you want to update.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The unique ID that specifies the IPSet that you want to update.</p>
    #[serde(rename = "ipSetId")]
    pub ip_set_id: String,
    /// <p>The updated URI of the file that contains the IPSet. For example: https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The unique ID that specifies the IPSet that you want to update.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateIPSetResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMemberDetectorsRequest {
    /// <p>A list of member account IDs to be updated.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// <p>Describes which data sources will be updated.</p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<DataSourceConfigurations>,
    /// <p>The detector ID of the administrator account.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMemberDetectorsResponse {
    /// <p>A list of member account IDs that were unable to be processed along with an explanation for why they were not processed.</p>
    #[serde(rename = "unprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateOrganizationConfigurationRequest {
    /// <p>Indicates whether to automatically enable member accounts in the organization.</p>
    #[serde(rename = "autoEnable")]
    pub auto_enable: bool,
    /// <p>Describes which data sources will be updated.</p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<OrganizationDataSourceConfigurations>,
    /// <p>The ID of the detector to update the delegated administrator for.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateOrganizationConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePublishingDestinationRequest {
    /// <p>The ID of the publishing destination to update.</p>
    #[serde(rename = "destinationId")]
    pub destination_id: String,
    /// <p>A <code>DestinationProperties</code> object that includes the <code>DestinationArn</code> and <code>KmsKeyArn</code> of the publishing destination.</p>
    #[serde(rename = "destinationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_properties: Option<DestinationProperties>,
    /// <p>The ID of the detector associated with the publishing destinations to update.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePublishingDestinationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateThreatIntelSetRequest {
    /// <p>The updated Boolean value that specifies whether the ThreateIntelSet is active or not.</p>
    #[serde(rename = "activate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate: Option<bool>,
    /// <p>The detectorID that specifies the GuardDuty service whose ThreatIntelSet you want to update.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The updated URI of the file that contains the ThreateIntelSet.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The unique ID that specifies the ThreatIntelSet that you want to update.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique ID that specifies the ThreatIntelSet that you want to update.</p>
    #[serde(rename = "threatIntelSetId")]
    pub threat_intel_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateThreatIntelSetResponse {}

/// <p>Contains information on the total of usage based on account IDs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UsageAccountResult {
    /// <p>The Account ID that generated usage.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Represents the total of usage for the Account ID.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Total>,
}

/// <p>Contains information about the criteria used to query usage statistics.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UsageCriteria {
    /// <p>The account IDs to aggregate usage statistics from.</p>
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>The data sources to aggregate usage statistics from.</p>
    #[serde(rename = "dataSources")]
    pub data_sources: Vec<String>,
    /// <p>The resources to aggregate usage statistics from. Only accepts exact resource names.</p>
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

/// <p>Contains information on the result of usage based on data source type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UsageDataSourceResult {
    /// <p>The data source type that generated usage.</p>
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    /// <p>Represents the total of usage for the specified data source.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Total>,
}

/// <p>Contains information on the sum of usage based on an AWS resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UsageResourceResult {
    /// <p>The AWS resource that generated usage.</p>
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// <p>Represents the sum total of usage for the specified resource type.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Total>,
}

/// <p>Contains the result of GuardDuty usage. If a UsageStatisticType is provided the result for other types will be null. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UsageStatistics {
    /// <p>The usage statistic sum organized by account ID.</p>
    #[serde(rename = "sumByAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_by_account: Option<Vec<UsageAccountResult>>,
    /// <p>The usage statistic sum organized by on data source.</p>
    #[serde(rename = "sumByDataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_by_data_source: Option<Vec<UsageDataSourceResult>>,
    /// <p>The usage statistic sum organized by resource.</p>
    #[serde(rename = "sumByResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_by_resource: Option<Vec<UsageResourceResult>>,
    /// <p>Lists the top 50 resources that have generated the most GuardDuty usage, in order from most to least expensive.</p>
    #[serde(rename = "topResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_resources: Option<Vec<UsageResourceResult>>,
}

/// Errors returned by AcceptInvitation
#[derive(Debug, PartialEq)]
pub enum AcceptInvitationError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl AcceptInvitationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(AcceptInvitationError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(AcceptInvitationError::InternalServerError(
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
impl fmt::Display for AcceptInvitationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptInvitationError::BadRequest(ref cause) => write!(f, "{}", cause),
            AcceptInvitationError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AcceptInvitationError {}
/// Errors returned by ArchiveFindings
#[derive(Debug, PartialEq)]
pub enum ArchiveFindingsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl ArchiveFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ArchiveFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ArchiveFindingsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ArchiveFindingsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ArchiveFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArchiveFindingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ArchiveFindingsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ArchiveFindingsError {}
/// Errors returned by CreateDetector
#[derive(Debug, PartialEq)]
pub enum CreateDetectorError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl CreateDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDetectorError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateDetectorError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDetectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDetectorError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDetectorError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDetectorError {}
/// Errors returned by CreateFilter
#[derive(Debug, PartialEq)]
pub enum CreateFilterError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl CreateFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFilterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateFilterError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateFilterError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateFilterError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFilterError {}
/// Errors returned by CreateIPSet
#[derive(Debug, PartialEq)]
pub enum CreateIPSetError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl CreateIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIPSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateIPSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateIPSetError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIPSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIPSetError {}
/// Errors returned by CreateMembers
#[derive(Debug, PartialEq)]
pub enum CreateMembersError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl CreateMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateMembersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMembersError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateMembersError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMembersError {}
/// Errors returned by CreatePublishingDestination
#[derive(Debug, PartialEq)]
pub enum CreatePublishingDestinationError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl CreatePublishingDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreatePublishingDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreatePublishingDestinationError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        CreatePublishingDestinationError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePublishingDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePublishingDestinationError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreatePublishingDestinationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreatePublishingDestinationError {}
/// Errors returned by CreateSampleFindings
#[derive(Debug, PartialEq)]
pub enum CreateSampleFindingsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl CreateSampleFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSampleFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateSampleFindingsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateSampleFindingsError::InternalServerError(
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
impl fmt::Display for CreateSampleFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSampleFindingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateSampleFindingsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSampleFindingsError {}
/// Errors returned by CreateThreatIntelSet
#[derive(Debug, PartialEq)]
pub enum CreateThreatIntelSetError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl CreateThreatIntelSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateThreatIntelSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateThreatIntelSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateThreatIntelSetError::InternalServerError(
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
impl fmt::Display for CreateThreatIntelSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateThreatIntelSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateThreatIntelSetError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateThreatIntelSetError {}
/// Errors returned by DeclineInvitations
#[derive(Debug, PartialEq)]
pub enum DeclineInvitationsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DeclineInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeclineInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeclineInvitationsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeclineInvitationsError::InternalServerError(
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
impl fmt::Display for DeclineInvitationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeclineInvitationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeclineInvitationsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeclineInvitationsError {}
/// Errors returned by DeleteDetector
#[derive(Debug, PartialEq)]
pub enum DeleteDetectorError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DeleteDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDetectorError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteDetectorError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDetectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDetectorError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteDetectorError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDetectorError {}
/// Errors returned by DeleteFilter
#[derive(Debug, PartialEq)]
pub enum DeleteFilterError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DeleteFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFilterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteFilterError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteFilterError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteFilterError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFilterError {}
/// Errors returned by DeleteIPSet
#[derive(Debug, PartialEq)]
pub enum DeleteIPSetError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DeleteIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIPSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteIPSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteIPSetError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIPSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteIPSetError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIPSetError {}
/// Errors returned by DeleteInvitations
#[derive(Debug, PartialEq)]
pub enum DeleteInvitationsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DeleteInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteInvitationsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteInvitationsError::InternalServerError(
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
impl fmt::Display for DeleteInvitationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInvitationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteInvitationsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInvitationsError {}
/// Errors returned by DeleteMembers
#[derive(Debug, PartialEq)]
pub enum DeleteMembersError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DeleteMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteMembersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMembersError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteMembersError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMembersError {}
/// Errors returned by DeletePublishingDestination
#[derive(Debug, PartialEq)]
pub enum DeletePublishingDestinationError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DeletePublishingDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeletePublishingDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeletePublishingDestinationError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DeletePublishingDestinationError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePublishingDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePublishingDestinationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeletePublishingDestinationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeletePublishingDestinationError {}
/// Errors returned by DeleteThreatIntelSet
#[derive(Debug, PartialEq)]
pub enum DeleteThreatIntelSetError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DeleteThreatIntelSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteThreatIntelSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteThreatIntelSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteThreatIntelSetError::InternalServerError(
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
impl fmt::Display for DeleteThreatIntelSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteThreatIntelSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteThreatIntelSetError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteThreatIntelSetError {}
/// Errors returned by DescribeOrganizationConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationConfigurationError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DescribeOrganizationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOrganizationConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigurationError::BadRequest(err.msg),
                    )
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigurationError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeOrganizationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOrganizationConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeOrganizationConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeOrganizationConfigurationError {}
/// Errors returned by DescribePublishingDestination
#[derive(Debug, PartialEq)]
pub enum DescribePublishingDestinationError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DescribePublishingDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribePublishingDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribePublishingDestinationError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribePublishingDestinationError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribePublishingDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePublishingDestinationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribePublishingDestinationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribePublishingDestinationError {}
/// Errors returned by DisableOrganizationAdminAccount
#[derive(Debug, PartialEq)]
pub enum DisableOrganizationAdminAccountError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DisableOrganizationAdminAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisableOrganizationAdminAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DisableOrganizationAdminAccountError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DisableOrganizationAdminAccountError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableOrganizationAdminAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableOrganizationAdminAccountError::BadRequest(ref cause) => write!(f, "{}", cause),
            DisableOrganizationAdminAccountError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisableOrganizationAdminAccountError {}
/// Errors returned by DisassociateFromMasterAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateFromMasterAccountError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DisassociateFromMasterAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateFromMasterAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DisassociateFromMasterAccountError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DisassociateFromMasterAccountError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateFromMasterAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateFromMasterAccountError::BadRequest(ref cause) => write!(f, "{}", cause),
            DisassociateFromMasterAccountError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateFromMasterAccountError {}
/// Errors returned by DisassociateMembers
#[derive(Debug, PartialEq)]
pub enum DisassociateMembersError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl DisassociateMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DisassociateMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DisassociateMembersError::InternalServerError(
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
impl fmt::Display for DisassociateMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateMembersError::BadRequest(ref cause) => write!(f, "{}", cause),
            DisassociateMembersError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateMembersError {}
/// Errors returned by EnableOrganizationAdminAccount
#[derive(Debug, PartialEq)]
pub enum EnableOrganizationAdminAccountError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl EnableOrganizationAdminAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<EnableOrganizationAdminAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(EnableOrganizationAdminAccountError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        EnableOrganizationAdminAccountError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableOrganizationAdminAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableOrganizationAdminAccountError::BadRequest(ref cause) => write!(f, "{}", cause),
            EnableOrganizationAdminAccountError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for EnableOrganizationAdminAccountError {}
/// Errors returned by GetDetector
#[derive(Debug, PartialEq)]
pub enum GetDetectorError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl GetDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDetectorError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetDetectorError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDetectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDetectorError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDetectorError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDetectorError {}
/// Errors returned by GetFilter
#[derive(Debug, PartialEq)]
pub enum GetFilterError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl GetFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFilterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetFilterError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetFilterError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetFilterError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFilterError {}
/// Errors returned by GetFindings
#[derive(Debug, PartialEq)]
pub enum GetFindingsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl GetFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetFindingsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetFindingsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFindingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetFindingsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFindingsError {}
/// Errors returned by GetFindingsStatistics
#[derive(Debug, PartialEq)]
pub enum GetFindingsStatisticsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl GetFindingsStatisticsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFindingsStatisticsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetFindingsStatisticsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetFindingsStatisticsError::InternalServerError(
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
impl fmt::Display for GetFindingsStatisticsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFindingsStatisticsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetFindingsStatisticsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFindingsStatisticsError {}
/// Errors returned by GetIPSet
#[derive(Debug, PartialEq)]
pub enum GetIPSetError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl GetIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIPSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetIPSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetIPSetError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIPSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetIPSetError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIPSetError {}
/// Errors returned by GetInvitationsCount
#[derive(Debug, PartialEq)]
pub enum GetInvitationsCountError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl GetInvitationsCountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInvitationsCountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetInvitationsCountError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetInvitationsCountError::InternalServerError(
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
impl fmt::Display for GetInvitationsCountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInvitationsCountError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetInvitationsCountError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInvitationsCountError {}
/// Errors returned by GetMasterAccount
#[derive(Debug, PartialEq)]
pub enum GetMasterAccountError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl GetMasterAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMasterAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetMasterAccountError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetMasterAccountError::InternalServerError(
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
impl fmt::Display for GetMasterAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMasterAccountError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetMasterAccountError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMasterAccountError {}
/// Errors returned by GetMemberDetectors
#[derive(Debug, PartialEq)]
pub enum GetMemberDetectorsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl GetMemberDetectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMemberDetectorsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetMemberDetectorsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetMemberDetectorsError::InternalServerError(
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
impl fmt::Display for GetMemberDetectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMemberDetectorsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetMemberDetectorsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMemberDetectorsError {}
/// Errors returned by GetMembers
#[derive(Debug, PartialEq)]
pub enum GetMembersError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl GetMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetMembersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMembersError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetMembersError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMembersError {}
/// Errors returned by GetThreatIntelSet
#[derive(Debug, PartialEq)]
pub enum GetThreatIntelSetError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl GetThreatIntelSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetThreatIntelSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetThreatIntelSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetThreatIntelSetError::InternalServerError(
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
impl fmt::Display for GetThreatIntelSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetThreatIntelSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetThreatIntelSetError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetThreatIntelSetError {}
/// Errors returned by GetUsageStatistics
#[derive(Debug, PartialEq)]
pub enum GetUsageStatisticsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl GetUsageStatisticsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUsageStatisticsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetUsageStatisticsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetUsageStatisticsError::InternalServerError(
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
impl fmt::Display for GetUsageStatisticsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUsageStatisticsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetUsageStatisticsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUsageStatisticsError {}
/// Errors returned by InviteMembers
#[derive(Debug, PartialEq)]
pub enum InviteMembersError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl InviteMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InviteMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(InviteMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(InviteMembersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InviteMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InviteMembersError::BadRequest(ref cause) => write!(f, "{}", cause),
            InviteMembersError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InviteMembersError {}
/// Errors returned by ListDetectors
#[derive(Debug, PartialEq)]
pub enum ListDetectorsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl ListDetectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDetectorsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDetectorsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListDetectorsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDetectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDetectorsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListDetectorsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDetectorsError {}
/// Errors returned by ListFilters
#[derive(Debug, PartialEq)]
pub enum ListFiltersError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl ListFiltersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFiltersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListFiltersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListFiltersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFiltersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFiltersError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListFiltersError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFiltersError {}
/// Errors returned by ListFindings
#[derive(Debug, PartialEq)]
pub enum ListFindingsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl ListFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListFindingsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListFindingsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFindingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListFindingsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFindingsError {}
/// Errors returned by ListIPSets
#[derive(Debug, PartialEq)]
pub enum ListIPSetsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl ListIPSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIPSetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListIPSetsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListIPSetsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListIPSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIPSetsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListIPSetsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIPSetsError {}
/// Errors returned by ListInvitations
#[derive(Debug, PartialEq)]
pub enum ListInvitationsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl ListInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListInvitationsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListInvitationsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInvitationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInvitationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInvitationsError {}
/// Errors returned by ListMembers
#[derive(Debug, PartialEq)]
pub enum ListMembersError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl ListMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListMembersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMembersError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListMembersError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMembersError {}
/// Errors returned by ListOrganizationAdminAccounts
#[derive(Debug, PartialEq)]
pub enum ListOrganizationAdminAccountsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl ListOrganizationAdminAccountsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListOrganizationAdminAccountsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListOrganizationAdminAccountsError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        ListOrganizationAdminAccountsError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOrganizationAdminAccountsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOrganizationAdminAccountsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListOrganizationAdminAccountsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListOrganizationAdminAccountsError {}
/// Errors returned by ListPublishingDestinations
#[derive(Debug, PartialEq)]
pub enum ListPublishingDestinationsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl ListPublishingDestinationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListPublishingDestinationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListPublishingDestinationsError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        ListPublishingDestinationsError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPublishingDestinationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPublishingDestinationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListPublishingDestinationsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListPublishingDestinationsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
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
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListThreatIntelSets
#[derive(Debug, PartialEq)]
pub enum ListThreatIntelSetsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl ListThreatIntelSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListThreatIntelSetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListThreatIntelSetsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListThreatIntelSetsError::InternalServerError(
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
impl fmt::Display for ListThreatIntelSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListThreatIntelSetsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListThreatIntelSetsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListThreatIntelSetsError {}
/// Errors returned by StartMonitoringMembers
#[derive(Debug, PartialEq)]
pub enum StartMonitoringMembersError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl StartMonitoringMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartMonitoringMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartMonitoringMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StartMonitoringMembersError::InternalServerError(
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
impl fmt::Display for StartMonitoringMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartMonitoringMembersError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartMonitoringMembersError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartMonitoringMembersError {}
/// Errors returned by StopMonitoringMembers
#[derive(Debug, PartialEq)]
pub enum StopMonitoringMembersError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl StopMonitoringMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopMonitoringMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StopMonitoringMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StopMonitoringMembersError::InternalServerError(
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
impl fmt::Display for StopMonitoringMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopMonitoringMembersError::BadRequest(ref cause) => write!(f, "{}", cause),
            StopMonitoringMembersError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopMonitoringMembersError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
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
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UnarchiveFindings
#[derive(Debug, PartialEq)]
pub enum UnarchiveFindingsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl UnarchiveFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnarchiveFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UnarchiveFindingsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UnarchiveFindingsError::InternalServerError(
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
impl fmt::Display for UnarchiveFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnarchiveFindingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            UnarchiveFindingsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UnarchiveFindingsError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
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
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDetector
#[derive(Debug, PartialEq)]
pub enum UpdateDetectorError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl UpdateDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDetectorError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateDetectorError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDetectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDetectorError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateDetectorError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDetectorError {}
/// Errors returned by UpdateFilter
#[derive(Debug, PartialEq)]
pub enum UpdateFilterError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl UpdateFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFilterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFilterError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFilterError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFilterError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFilterError {}
/// Errors returned by UpdateFindingsFeedback
#[derive(Debug, PartialEq)]
pub enum UpdateFindingsFeedbackError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl UpdateFindingsFeedbackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFindingsFeedbackError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFindingsFeedbackError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFindingsFeedbackError::InternalServerError(
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
impl fmt::Display for UpdateFindingsFeedbackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFindingsFeedbackError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFindingsFeedbackError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFindingsFeedbackError {}
/// Errors returned by UpdateIPSet
#[derive(Debug, PartialEq)]
pub enum UpdateIPSetError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl UpdateIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIPSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateIPSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateIPSetError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateIPSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateIPSetError {}
/// Errors returned by UpdateMemberDetectors
#[derive(Debug, PartialEq)]
pub enum UpdateMemberDetectorsError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl UpdateMemberDetectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMemberDetectorsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateMemberDetectorsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateMemberDetectorsError::InternalServerError(
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
impl fmt::Display for UpdateMemberDetectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMemberDetectorsError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateMemberDetectorsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateMemberDetectorsError {}
/// Errors returned by UpdateOrganizationConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateOrganizationConfigurationError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl UpdateOrganizationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateOrganizationConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateOrganizationConfigurationError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateOrganizationConfigurationError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateOrganizationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateOrganizationConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateOrganizationConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateOrganizationConfigurationError {}
/// Errors returned by UpdatePublishingDestination
#[derive(Debug, PartialEq)]
pub enum UpdatePublishingDestinationError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl UpdatePublishingDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdatePublishingDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdatePublishingDestinationError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdatePublishingDestinationError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdatePublishingDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePublishingDestinationError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdatePublishingDestinationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdatePublishingDestinationError {}
/// Errors returned by UpdateThreatIntelSet
#[derive(Debug, PartialEq)]
pub enum UpdateThreatIntelSetError {
    /// <p>A bad request exception object.</p>
    BadRequest(String),
    /// <p>An internal server error exception object.</p>
    InternalServerError(String),
}

impl UpdateThreatIntelSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateThreatIntelSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateThreatIntelSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateThreatIntelSetError::InternalServerError(
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
impl fmt::Display for UpdateThreatIntelSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateThreatIntelSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateThreatIntelSetError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateThreatIntelSetError {}
/// Trait representing the capabilities of the Amazon GuardDuty API. Amazon GuardDuty clients implement this trait.
#[async_trait]
pub trait GuardDuty {
    /// <p>Accepts the invitation to be monitored by a GuardDuty administrator account.</p>
    async fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Result<AcceptInvitationResponse, RusotoError<AcceptInvitationError>>;

    /// <p><p>Archives GuardDuty findings that are specified by the list of finding IDs.</p> <note> <p>Only the administrator account can archive findings. Member accounts don&#39;t have permission to archive findings from their accounts.</p> </note></p>
    async fn archive_findings(
        &self,
        input: ArchiveFindingsRequest,
    ) -> Result<ArchiveFindingsResponse, RusotoError<ArchiveFindingsError>>;

    /// <p>Creates a single Amazon GuardDuty detector. A detector is a resource that represents the GuardDuty service. To start using GuardDuty, you must create a detector in each Region where you enable the service. You can have only one detector per account per Region. All data sources are enabled in a new detector by default.</p>
    async fn create_detector(
        &self,
        input: CreateDetectorRequest,
    ) -> Result<CreateDetectorResponse, RusotoError<CreateDetectorError>>;

    /// <p>Creates a filter using the specified finding criteria.</p>
    async fn create_filter(
        &self,
        input: CreateFilterRequest,
    ) -> Result<CreateFilterResponse, RusotoError<CreateFilterError>>;

    /// <p>Creates a new IPSet, which is called a trusted IP list in the console user interface. An IPSet is a list of IP addresses that are trusted for secure communication with AWS infrastructure and applications. GuardDuty doesn't generate findings for IP addresses that are included in IPSets. Only users from the administrator account can use this operation.</p>
    async fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> Result<CreateIPSetResponse, RusotoError<CreateIPSetError>>;

    /// <p>Creates member accounts of the current AWS account by specifying a list of AWS account IDs. This step is a prerequisite for managing the associated member accounts either by invitation or through an organization.</p> <p>When using <code>Create Members</code> as an organizations delegated administrator this action will enable GuardDuty in the added member accounts, with the exception of the organization delegated administrator account, which must enable GuardDuty prior to being added as a member.</p> <p>If you are adding accounts by invitation use this action after GuardDuty has been enabled in potential member accounts and before using <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_InviteMembers.html"> <code>Invite Members</code> </a>.</p>
    async fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> Result<CreateMembersResponse, RusotoError<CreateMembersError>>;

    /// <p>Creates a publishing destination to export findings to. The resource to export findings to must exist before you use this operation.</p>
    async fn create_publishing_destination(
        &self,
        input: CreatePublishingDestinationRequest,
    ) -> Result<CreatePublishingDestinationResponse, RusotoError<CreatePublishingDestinationError>>;

    /// <p>Generates example findings of types specified by the list of finding types. If 'NULL' is specified for <code>findingTypes</code>, the API generates example findings of all supported finding types.</p>
    async fn create_sample_findings(
        &self,
        input: CreateSampleFindingsRequest,
    ) -> Result<CreateSampleFindingsResponse, RusotoError<CreateSampleFindingsError>>;

    /// <p>Creates a new ThreatIntelSet. ThreatIntelSets consist of known malicious IP addresses. GuardDuty generates findings based on ThreatIntelSets. Only users of the administrator account can use this operation.</p>
    async fn create_threat_intel_set(
        &self,
        input: CreateThreatIntelSetRequest,
    ) -> Result<CreateThreatIntelSetResponse, RusotoError<CreateThreatIntelSetError>>;

    /// <p>Declines invitations sent to the current member account by AWS accounts specified by their account IDs.</p>
    async fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> Result<DeclineInvitationsResponse, RusotoError<DeclineInvitationsError>>;

    /// <p>Deletes an Amazon GuardDuty detector that is specified by the detector ID.</p>
    async fn delete_detector(
        &self,
        input: DeleteDetectorRequest,
    ) -> Result<DeleteDetectorResponse, RusotoError<DeleteDetectorError>>;

    /// <p>Deletes the filter specified by the filter name.</p>
    async fn delete_filter(
        &self,
        input: DeleteFilterRequest,
    ) -> Result<DeleteFilterResponse, RusotoError<DeleteFilterError>>;

    /// <p>Deletes the IPSet specified by the <code>ipSetId</code>. IPSets are called trusted IP lists in the console user interface.</p>
    async fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> Result<DeleteIPSetResponse, RusotoError<DeleteIPSetError>>;

    /// <p>Deletes invitations sent to the current member account by AWS accounts specified by their account IDs.</p>
    async fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> Result<DeleteInvitationsResponse, RusotoError<DeleteInvitationsError>>;

    /// <p>Deletes GuardDuty member accounts (to the current GuardDuty administrator account) specified by the account IDs.</p>
    async fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> Result<DeleteMembersResponse, RusotoError<DeleteMembersError>>;

    /// <p>Deletes the publishing definition with the specified <code>destinationId</code>.</p>
    async fn delete_publishing_destination(
        &self,
        input: DeletePublishingDestinationRequest,
    ) -> Result<DeletePublishingDestinationResponse, RusotoError<DeletePublishingDestinationError>>;

    /// <p>Deletes the ThreatIntelSet specified by the ThreatIntelSet ID.</p>
    async fn delete_threat_intel_set(
        &self,
        input: DeleteThreatIntelSetRequest,
    ) -> Result<DeleteThreatIntelSetResponse, RusotoError<DeleteThreatIntelSetError>>;

    /// <p>Returns information about the account selected as the delegated administrator for GuardDuty.</p>
    async fn describe_organization_configuration(
        &self,
        input: DescribeOrganizationConfigurationRequest,
    ) -> Result<
        DescribeOrganizationConfigurationResponse,
        RusotoError<DescribeOrganizationConfigurationError>,
    >;

    /// <p>Returns information about the publishing destination specified by the provided <code>destinationId</code>.</p>
    async fn describe_publishing_destination(
        &self,
        input: DescribePublishingDestinationRequest,
    ) -> Result<
        DescribePublishingDestinationResponse,
        RusotoError<DescribePublishingDestinationError>,
    >;

    /// <p>Disables an AWS account within the Organization as the GuardDuty delegated administrator.</p>
    async fn disable_organization_admin_account(
        &self,
        input: DisableOrganizationAdminAccountRequest,
    ) -> Result<
        DisableOrganizationAdminAccountResponse,
        RusotoError<DisableOrganizationAdminAccountError>,
    >;

    /// <p>Disassociates the current GuardDuty member account from its administrator account.</p>
    async fn disassociate_from_master_account(
        &self,
        input: DisassociateFromMasterAccountRequest,
    ) -> Result<
        DisassociateFromMasterAccountResponse,
        RusotoError<DisassociateFromMasterAccountError>,
    >;

    /// <p>Disassociates GuardDuty member accounts (to the current GuardDuty administrator account) specified by the account IDs.</p>
    async fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> Result<DisassociateMembersResponse, RusotoError<DisassociateMembersError>>;

    /// <p>Enables an AWS account within the organization as the GuardDuty delegated administrator.</p>
    async fn enable_organization_admin_account(
        &self,
        input: EnableOrganizationAdminAccountRequest,
    ) -> Result<
        EnableOrganizationAdminAccountResponse,
        RusotoError<EnableOrganizationAdminAccountError>,
    >;

    /// <p>Retrieves an Amazon GuardDuty detector specified by the detectorId.</p>
    async fn get_detector(
        &self,
        input: GetDetectorRequest,
    ) -> Result<GetDetectorResponse, RusotoError<GetDetectorError>>;

    /// <p>Returns the details of the filter specified by the filter name.</p>
    async fn get_filter(
        &self,
        input: GetFilterRequest,
    ) -> Result<GetFilterResponse, RusotoError<GetFilterError>>;

    /// <p>Describes Amazon GuardDuty findings specified by finding IDs.</p>
    async fn get_findings(
        &self,
        input: GetFindingsRequest,
    ) -> Result<GetFindingsResponse, RusotoError<GetFindingsError>>;

    /// <p>Lists Amazon GuardDuty findings statistics for the specified detector ID.</p>
    async fn get_findings_statistics(
        &self,
        input: GetFindingsStatisticsRequest,
    ) -> Result<GetFindingsStatisticsResponse, RusotoError<GetFindingsStatisticsError>>;

    /// <p>Retrieves the IPSet specified by the <code>ipSetId</code>.</p>
    async fn get_ip_set(
        &self,
        input: GetIPSetRequest,
    ) -> Result<GetIPSetResponse, RusotoError<GetIPSetError>>;

    /// <p>Returns the count of all GuardDuty membership invitations that were sent to the current member account except the currently accepted invitation.</p>
    async fn get_invitations_count(
        &self,
    ) -> Result<GetInvitationsCountResponse, RusotoError<GetInvitationsCountError>>;

    /// <p>Provides the details for the GuardDuty administrator account associated with the current GuardDuty member account.</p>
    async fn get_master_account(
        &self,
        input: GetMasterAccountRequest,
    ) -> Result<GetMasterAccountResponse, RusotoError<GetMasterAccountError>>;

    /// <p>Describes which data sources are enabled for the member account's detector.</p>
    async fn get_member_detectors(
        &self,
        input: GetMemberDetectorsRequest,
    ) -> Result<GetMemberDetectorsResponse, RusotoError<GetMemberDetectorsError>>;

    /// <p>Retrieves GuardDuty member accounts (of the current GuardDuty administrator account) specified by the account IDs.</p>
    async fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> Result<GetMembersResponse, RusotoError<GetMembersError>>;

    /// <p>Retrieves the ThreatIntelSet that is specified by the ThreatIntelSet ID.</p>
    async fn get_threat_intel_set(
        &self,
        input: GetThreatIntelSetRequest,
    ) -> Result<GetThreatIntelSetResponse, RusotoError<GetThreatIntelSetError>>;

    /// <p>Lists Amazon GuardDuty usage statistics over the last 30 days for the specified detector ID. For newly enabled detectors or data sources the cost returned will include only the usage so far under 30 days, this may differ from the cost metrics in the console, which projects usage over 30 days to provide a monthly cost estimate. For more information see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/monitoring_costs.html#usage-calculations">Understanding How Usage Costs are Calculated</a>.</p>
    async fn get_usage_statistics(
        &self,
        input: GetUsageStatisticsRequest,
    ) -> Result<GetUsageStatisticsResponse, RusotoError<GetUsageStatisticsError>>;

    /// <p>Invites other AWS accounts (created as members of the current AWS account by CreateMembers) to enable GuardDuty, and allow the current AWS account to view and manage these accounts' findings on their behalf as the GuardDuty administrator account.</p>
    async fn invite_members(
        &self,
        input: InviteMembersRequest,
    ) -> Result<InviteMembersResponse, RusotoError<InviteMembersError>>;

    /// <p>Lists detectorIds of all the existing Amazon GuardDuty detector resources.</p>
    async fn list_detectors(
        &self,
        input: ListDetectorsRequest,
    ) -> Result<ListDetectorsResponse, RusotoError<ListDetectorsError>>;

    /// <p>Returns a paginated list of the current filters.</p>
    async fn list_filters(
        &self,
        input: ListFiltersRequest,
    ) -> Result<ListFiltersResponse, RusotoError<ListFiltersError>>;

    /// <p>Lists Amazon GuardDuty findings for the specified detector ID.</p>
    async fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> Result<ListFindingsResponse, RusotoError<ListFindingsError>>;

    /// <p>Lists the IPSets of the GuardDuty service specified by the detector ID. If you use this operation from a member account, the IPSets returned are the IPSets from the associated administrator account.</p>
    async fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> Result<ListIPSetsResponse, RusotoError<ListIPSetsError>>;

    /// <p>Lists all GuardDuty membership invitations that were sent to the current AWS account.</p>
    async fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> Result<ListInvitationsResponse, RusotoError<ListInvitationsError>>;

    /// <p>Lists details about all member accounts for the current GuardDuty administrator account.</p>
    async fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> Result<ListMembersResponse, RusotoError<ListMembersError>>;

    /// <p>Lists the accounts configured as GuardDuty delegated administrators.</p>
    async fn list_organization_admin_accounts(
        &self,
        input: ListOrganizationAdminAccountsRequest,
    ) -> Result<
        ListOrganizationAdminAccountsResponse,
        RusotoError<ListOrganizationAdminAccountsError>,
    >;

    /// <p>Returns a list of publishing destinations associated with the specified <code>dectectorId</code>.</p>
    async fn list_publishing_destinations(
        &self,
        input: ListPublishingDestinationsRequest,
    ) -> Result<ListPublishingDestinationsResponse, RusotoError<ListPublishingDestinationsError>>;

    /// <p>Lists tags for a resource. Tagging is currently supported for detectors, finding filters, IP sets, and threat intel sets, with a limit of 50 tags per resource. When invoked, this operation returns all assigned tags for a given resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists the ThreatIntelSets of the GuardDuty service specified by the detector ID. If you use this operation from a member account, the ThreatIntelSets associated with the administrator account are returned.</p>
    async fn list_threat_intel_sets(
        &self,
        input: ListThreatIntelSetsRequest,
    ) -> Result<ListThreatIntelSetsResponse, RusotoError<ListThreatIntelSetsError>>;

    /// <p>Turns on GuardDuty monitoring of the specified member accounts. Use this operation to restart monitoring of accounts that you stopped monitoring with the <code>StopMonitoringMembers</code> operation.</p>
    async fn start_monitoring_members(
        &self,
        input: StartMonitoringMembersRequest,
    ) -> Result<StartMonitoringMembersResponse, RusotoError<StartMonitoringMembersError>>;

    /// <p>Stops GuardDuty monitoring for the specified member accounts. Use the <code>StartMonitoringMembers</code> operation to restart monitoring for those accounts.</p>
    async fn stop_monitoring_members(
        &self,
        input: StopMonitoringMembersRequest,
    ) -> Result<StopMonitoringMembersResponse, RusotoError<StopMonitoringMembersError>>;

    /// <p>Adds tags to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Unarchives GuardDuty findings specified by the <code>findingIds</code>.</p>
    async fn unarchive_findings(
        &self,
        input: UnarchiveFindingsRequest,
    ) -> Result<UnarchiveFindingsResponse, RusotoError<UnarchiveFindingsError>>;

    /// <p>Removes tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the Amazon GuardDuty detector specified by the detectorId.</p>
    async fn update_detector(
        &self,
        input: UpdateDetectorRequest,
    ) -> Result<UpdateDetectorResponse, RusotoError<UpdateDetectorError>>;

    /// <p>Updates the filter specified by the filter name.</p>
    async fn update_filter(
        &self,
        input: UpdateFilterRequest,
    ) -> Result<UpdateFilterResponse, RusotoError<UpdateFilterError>>;

    /// <p>Marks the specified GuardDuty findings as useful or not useful.</p>
    async fn update_findings_feedback(
        &self,
        input: UpdateFindingsFeedbackRequest,
    ) -> Result<UpdateFindingsFeedbackResponse, RusotoError<UpdateFindingsFeedbackError>>;

    /// <p>Updates the IPSet specified by the IPSet ID.</p>
    async fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> Result<UpdateIPSetResponse, RusotoError<UpdateIPSetError>>;

    /// <p>Contains information on member accounts to be updated.</p>
    async fn update_member_detectors(
        &self,
        input: UpdateMemberDetectorsRequest,
    ) -> Result<UpdateMemberDetectorsResponse, RusotoError<UpdateMemberDetectorsError>>;

    /// <p>Updates the delegated administrator account with the values provided.</p>
    async fn update_organization_configuration(
        &self,
        input: UpdateOrganizationConfigurationRequest,
    ) -> Result<
        UpdateOrganizationConfigurationResponse,
        RusotoError<UpdateOrganizationConfigurationError>,
    >;

    /// <p>Updates information about the publishing destination specified by the <code>destinationId</code>.</p>
    async fn update_publishing_destination(
        &self,
        input: UpdatePublishingDestinationRequest,
    ) -> Result<UpdatePublishingDestinationResponse, RusotoError<UpdatePublishingDestinationError>>;

    /// <p>Updates the ThreatIntelSet specified by the ThreatIntelSet ID.</p>
    async fn update_threat_intel_set(
        &self,
        input: UpdateThreatIntelSetRequest,
    ) -> Result<UpdateThreatIntelSetResponse, RusotoError<UpdateThreatIntelSetError>>;
}
/// A client for the Amazon GuardDuty API.
#[derive(Clone)]
pub struct GuardDutyClient {
    client: Client,
    region: region::Region,
}

impl GuardDutyClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> GuardDutyClient {
        GuardDutyClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> GuardDutyClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        GuardDutyClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> GuardDutyClient {
        GuardDutyClient { client, region }
    }
}

#[async_trait]
impl GuardDuty for GuardDutyClient {
    /// <p>Accepts the invitation to be monitored by a GuardDuty administrator account.</p>
    #[allow(unused_mut)]
    async fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Result<AcceptInvitationResponse, RusotoError<AcceptInvitationError>> {
        let request_uri = format!(
            "/detector/{detector_id}/master",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AcceptInvitationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AcceptInvitationError::from_response(response))
        }
    }

    /// <p><p>Archives GuardDuty findings that are specified by the list of finding IDs.</p> <note> <p>Only the administrator account can archive findings. Member accounts don&#39;t have permission to archive findings from their accounts.</p> </note></p>
    #[allow(unused_mut)]
    async fn archive_findings(
        &self,
        input: ArchiveFindingsRequest,
    ) -> Result<ArchiveFindingsResponse, RusotoError<ArchiveFindingsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/archive",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ArchiveFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ArchiveFindingsError::from_response(response))
        }
    }

    /// <p>Creates a single Amazon GuardDuty detector. A detector is a resource that represents the GuardDuty service. To start using GuardDuty, you must create a detector in each Region where you enable the service. You can have only one detector per account per Region. All data sources are enabled in a new detector by default.</p>
    #[allow(unused_mut)]
    async fn create_detector(
        &self,
        input: CreateDetectorRequest,
    ) -> Result<CreateDetectorResponse, RusotoError<CreateDetectorError>> {
        let request_uri = "/detector";

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDetectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDetectorError::from_response(response))
        }
    }

    /// <p>Creates a filter using the specified finding criteria.</p>
    #[allow(unused_mut)]
    async fn create_filter(
        &self,
        input: CreateFilterRequest,
    ) -> Result<CreateFilterResponse, RusotoError<CreateFilterError>> {
        let request_uri = format!(
            "/detector/{detector_id}/filter",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateFilterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFilterError::from_response(response))
        }
    }

    /// <p>Creates a new IPSet, which is called a trusted IP list in the console user interface. An IPSet is a list of IP addresses that are trusted for secure communication with AWS infrastructure and applications. GuardDuty doesn't generate findings for IP addresses that are included in IPSets. Only users from the administrator account can use this operation.</p>
    #[allow(unused_mut)]
    async fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> Result<CreateIPSetResponse, RusotoError<CreateIPSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateIPSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIPSetError::from_response(response))
        }
    }

    /// <p>Creates member accounts of the current AWS account by specifying a list of AWS account IDs. This step is a prerequisite for managing the associated member accounts either by invitation or through an organization.</p> <p>When using <code>Create Members</code> as an organizations delegated administrator this action will enable GuardDuty in the added member accounts, with the exception of the organization delegated administrator account, which must enable GuardDuty prior to being added as a member.</p> <p>If you are adding accounts by invitation use this action after GuardDuty has been enabled in potential member accounts and before using <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_InviteMembers.html"> <code>Invite Members</code> </a>.</p>
    #[allow(unused_mut)]
    async fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> Result<CreateMembersResponse, RusotoError<CreateMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMembersError::from_response(response))
        }
    }

    /// <p>Creates a publishing destination to export findings to. The resource to export findings to must exist before you use this operation.</p>
    #[allow(unused_mut)]
    async fn create_publishing_destination(
        &self,
        input: CreatePublishingDestinationRequest,
    ) -> Result<CreatePublishingDestinationResponse, RusotoError<CreatePublishingDestinationError>>
    {
        let request_uri = format!(
            "/detector/{detector_id}/publishingDestination",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreatePublishingDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePublishingDestinationError::from_response(response))
        }
    }

    /// <p>Generates example findings of types specified by the list of finding types. If 'NULL' is specified for <code>findingTypes</code>, the API generates example findings of all supported finding types.</p>
    #[allow(unused_mut)]
    async fn create_sample_findings(
        &self,
        input: CreateSampleFindingsRequest,
    ) -> Result<CreateSampleFindingsResponse, RusotoError<CreateSampleFindingsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/create",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateSampleFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSampleFindingsError::from_response(response))
        }
    }

    /// <p>Creates a new ThreatIntelSet. ThreatIntelSets consist of known malicious IP addresses. GuardDuty generates findings based on ThreatIntelSets. Only users of the administrator account can use this operation.</p>
    #[allow(unused_mut)]
    async fn create_threat_intel_set(
        &self,
        input: CreateThreatIntelSetRequest,
    ) -> Result<CreateThreatIntelSetResponse, RusotoError<CreateThreatIntelSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateThreatIntelSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateThreatIntelSetError::from_response(response))
        }
    }

    /// <p>Declines invitations sent to the current member account by AWS accounts specified by their account IDs.</p>
    #[allow(unused_mut)]
    async fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> Result<DeclineInvitationsResponse, RusotoError<DeclineInvitationsError>> {
        let request_uri = "/invitation/decline";

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeclineInvitationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeclineInvitationsError::from_response(response))
        }
    }

    /// <p>Deletes an Amazon GuardDuty detector that is specified by the detector ID.</p>
    #[allow(unused_mut)]
    async fn delete_detector(
        &self,
        input: DeleteDetectorRequest,
    ) -> Result<DeleteDetectorResponse, RusotoError<DeleteDetectorError>> {
        let request_uri = format!("/detector/{detector_id}", detector_id = input.detector_id);

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDetectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDetectorError::from_response(response))
        }
    }

    /// <p>Deletes the filter specified by the filter name.</p>
    #[allow(unused_mut)]
    async fn delete_filter(
        &self,
        input: DeleteFilterRequest,
    ) -> Result<DeleteFilterResponse, RusotoError<DeleteFilterError>> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = input.detector_id,
            filter_name = input.filter_name
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteFilterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFilterError::from_response(response))
        }
    }

    /// <p>Deletes the IPSet specified by the <code>ipSetId</code>. IPSets are called trusted IP lists in the console user interface.</p>
    #[allow(unused_mut)]
    async fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> Result<DeleteIPSetResponse, RusotoError<DeleteIPSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = input.detector_id,
            ip_set_id = input.ip_set_id
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteIPSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIPSetError::from_response(response))
        }
    }

    /// <p>Deletes invitations sent to the current member account by AWS accounts specified by their account IDs.</p>
    #[allow(unused_mut)]
    async fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> Result<DeleteInvitationsResponse, RusotoError<DeleteInvitationsError>> {
        let request_uri = "/invitation/delete";

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteInvitationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInvitationsError::from_response(response))
        }
    }

    /// <p>Deletes GuardDuty member accounts (to the current GuardDuty administrator account) specified by the account IDs.</p>
    #[allow(unused_mut)]
    async fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> Result<DeleteMembersResponse, RusotoError<DeleteMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/delete",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMembersError::from_response(response))
        }
    }

    /// <p>Deletes the publishing definition with the specified <code>destinationId</code>.</p>
    #[allow(unused_mut)]
    async fn delete_publishing_destination(
        &self,
        input: DeletePublishingDestinationRequest,
    ) -> Result<DeletePublishingDestinationResponse, RusotoError<DeletePublishingDestinationError>>
    {
        let request_uri = format!(
            "/detector/{detector_id}/publishingDestination/{destination_id}",
            destination_id = input.destination_id,
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeletePublishingDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePublishingDestinationError::from_response(response))
        }
    }

    /// <p>Deletes the ThreatIntelSet specified by the ThreatIntelSet ID.</p>
    #[allow(unused_mut)]
    async fn delete_threat_intel_set(
        &self,
        input: DeleteThreatIntelSetRequest,
    ) -> Result<DeleteThreatIntelSetResponse, RusotoError<DeleteThreatIntelSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = input.detector_id,
            threat_intel_set_id = input.threat_intel_set_id
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteThreatIntelSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteThreatIntelSetError::from_response(response))
        }
    }

    /// <p>Returns information about the account selected as the delegated administrator for GuardDuty.</p>
    #[allow(unused_mut)]
    async fn describe_organization_configuration(
        &self,
        input: DescribeOrganizationConfigurationRequest,
    ) -> Result<
        DescribeOrganizationConfigurationResponse,
        RusotoError<DescribeOrganizationConfigurationError>,
    > {
        let request_uri = format!(
            "/detector/{detector_id}/admin",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeOrganizationConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeOrganizationConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns information about the publishing destination specified by the provided <code>destinationId</code>.</p>
    #[allow(unused_mut)]
    async fn describe_publishing_destination(
        &self,
        input: DescribePublishingDestinationRequest,
    ) -> Result<
        DescribePublishingDestinationResponse,
        RusotoError<DescribePublishingDestinationError>,
    > {
        let request_uri = format!(
            "/detector/{detector_id}/publishingDestination/{destination_id}",
            destination_id = input.destination_id,
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribePublishingDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribePublishingDestinationError::from_response(response))
        }
    }

    /// <p>Disables an AWS account within the Organization as the GuardDuty delegated administrator.</p>
    #[allow(unused_mut)]
    async fn disable_organization_admin_account(
        &self,
        input: DisableOrganizationAdminAccountRequest,
    ) -> Result<
        DisableOrganizationAdminAccountResponse,
        RusotoError<DisableOrganizationAdminAccountError>,
    > {
        let request_uri = "/admin/disable";

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisableOrganizationAdminAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisableOrganizationAdminAccountError::from_response(
                response,
            ))
        }
    }

    /// <p>Disassociates the current GuardDuty member account from its administrator account.</p>
    #[allow(unused_mut)]
    async fn disassociate_from_master_account(
        &self,
        input: DisassociateFromMasterAccountRequest,
    ) -> Result<
        DisassociateFromMasterAccountResponse,
        RusotoError<DisassociateFromMasterAccountError>,
    > {
        let request_uri = format!(
            "/detector/{detector_id}/master/disassociate",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateFromMasterAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateFromMasterAccountError::from_response(response))
        }
    }

    /// <p>Disassociates GuardDuty member accounts (to the current GuardDuty administrator account) specified by the account IDs.</p>
    #[allow(unused_mut)]
    async fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> Result<DisassociateMembersResponse, RusotoError<DisassociateMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/disassociate",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateMembersError::from_response(response))
        }
    }

    /// <p>Enables an AWS account within the organization as the GuardDuty delegated administrator.</p>
    #[allow(unused_mut)]
    async fn enable_organization_admin_account(
        &self,
        input: EnableOrganizationAdminAccountRequest,
    ) -> Result<
        EnableOrganizationAdminAccountResponse,
        RusotoError<EnableOrganizationAdminAccountError>,
    > {
        let request_uri = "/admin/enable";

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<EnableOrganizationAdminAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(EnableOrganizationAdminAccountError::from_response(response))
        }
    }

    /// <p>Retrieves an Amazon GuardDuty detector specified by the detectorId.</p>
    #[allow(unused_mut)]
    async fn get_detector(
        &self,
        input: GetDetectorRequest,
    ) -> Result<GetDetectorResponse, RusotoError<GetDetectorError>> {
        let request_uri = format!("/detector/{detector_id}", detector_id = input.detector_id);

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDetectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDetectorError::from_response(response))
        }
    }

    /// <p>Returns the details of the filter specified by the filter name.</p>
    #[allow(unused_mut)]
    async fn get_filter(
        &self,
        input: GetFilterRequest,
    ) -> Result<GetFilterResponse, RusotoError<GetFilterError>> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = input.detector_id,
            filter_name = input.filter_name
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFilterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFilterError::from_response(response))
        }
    }

    /// <p>Describes Amazon GuardDuty findings specified by finding IDs.</p>
    #[allow(unused_mut)]
    async fn get_findings(
        &self,
        input: GetFindingsRequest,
    ) -> Result<GetFindingsResponse, RusotoError<GetFindingsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/get",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFindingsError::from_response(response))
        }
    }

    /// <p>Lists Amazon GuardDuty findings statistics for the specified detector ID.</p>
    #[allow(unused_mut)]
    async fn get_findings_statistics(
        &self,
        input: GetFindingsStatisticsRequest,
    ) -> Result<GetFindingsStatisticsResponse, RusotoError<GetFindingsStatisticsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/statistics",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFindingsStatisticsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFindingsStatisticsError::from_response(response))
        }
    }

    /// <p>Retrieves the IPSet specified by the <code>ipSetId</code>.</p>
    #[allow(unused_mut)]
    async fn get_ip_set(
        &self,
        input: GetIPSetRequest,
    ) -> Result<GetIPSetResponse, RusotoError<GetIPSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = input.detector_id,
            ip_set_id = input.ip_set_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetIPSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetIPSetError::from_response(response))
        }
    }

    /// <p>Returns the count of all GuardDuty membership invitations that were sent to the current member account except the currently accepted invitation.</p>
    #[allow(unused_mut)]
    async fn get_invitations_count(
        &self,
    ) -> Result<GetInvitationsCountResponse, RusotoError<GetInvitationsCountError>> {
        let request_uri = "/invitation/count";

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInvitationsCountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInvitationsCountError::from_response(response))
        }
    }

    /// <p>Provides the details for the GuardDuty administrator account associated with the current GuardDuty member account.</p>
    #[allow(unused_mut)]
    async fn get_master_account(
        &self,
        input: GetMasterAccountRequest,
    ) -> Result<GetMasterAccountResponse, RusotoError<GetMasterAccountError>> {
        let request_uri = format!(
            "/detector/{detector_id}/master",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetMasterAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMasterAccountError::from_response(response))
        }
    }

    /// <p>Describes which data sources are enabled for the member account's detector.</p>
    #[allow(unused_mut)]
    async fn get_member_detectors(
        &self,
        input: GetMemberDetectorsRequest,
    ) -> Result<GetMemberDetectorsResponse, RusotoError<GetMemberDetectorsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/detector/get",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetMemberDetectorsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMemberDetectorsError::from_response(response))
        }
    }

    /// <p>Retrieves GuardDuty member accounts (of the current GuardDuty administrator account) specified by the account IDs.</p>
    #[allow(unused_mut)]
    async fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> Result<GetMembersResponse, RusotoError<GetMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/get",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMembersError::from_response(response))
        }
    }

    /// <p>Retrieves the ThreatIntelSet that is specified by the ThreatIntelSet ID.</p>
    #[allow(unused_mut)]
    async fn get_threat_intel_set(
        &self,
        input: GetThreatIntelSetRequest,
    ) -> Result<GetThreatIntelSetResponse, RusotoError<GetThreatIntelSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = input.detector_id,
            threat_intel_set_id = input.threat_intel_set_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetThreatIntelSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetThreatIntelSetError::from_response(response))
        }
    }

    /// <p>Lists Amazon GuardDuty usage statistics over the last 30 days for the specified detector ID. For newly enabled detectors or data sources the cost returned will include only the usage so far under 30 days, this may differ from the cost metrics in the console, which projects usage over 30 days to provide a monthly cost estimate. For more information see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/monitoring_costs.html#usage-calculations">Understanding How Usage Costs are Calculated</a>.</p>
    #[allow(unused_mut)]
    async fn get_usage_statistics(
        &self,
        input: GetUsageStatisticsRequest,
    ) -> Result<GetUsageStatisticsResponse, RusotoError<GetUsageStatisticsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/usage/statistics",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetUsageStatisticsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetUsageStatisticsError::from_response(response))
        }
    }

    /// <p>Invites other AWS accounts (created as members of the current AWS account by CreateMembers) to enable GuardDuty, and allow the current AWS account to view and manage these accounts' findings on their behalf as the GuardDuty administrator account.</p>
    #[allow(unused_mut)]
    async fn invite_members(
        &self,
        input: InviteMembersRequest,
    ) -> Result<InviteMembersResponse, RusotoError<InviteMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/invite",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<InviteMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InviteMembersError::from_response(response))
        }
    }

    /// <p>Lists detectorIds of all the existing Amazon GuardDuty detector resources.</p>
    #[allow(unused_mut)]
    async fn list_detectors(
        &self,
        input: ListDetectorsRequest,
    ) -> Result<ListDetectorsResponse, RusotoError<ListDetectorsError>> {
        let request_uri = "/detector";

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDetectorsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDetectorsError::from_response(response))
        }
    }

    /// <p>Returns a paginated list of the current filters.</p>
    #[allow(unused_mut)]
    async fn list_filters(
        &self,
        input: ListFiltersRequest,
    ) -> Result<ListFiltersResponse, RusotoError<ListFiltersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/filter",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFiltersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFiltersError::from_response(response))
        }
    }

    /// <p>Lists Amazon GuardDuty findings for the specified detector ID.</p>
    #[allow(unused_mut)]
    async fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> Result<ListFindingsResponse, RusotoError<ListFindingsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFindingsError::from_response(response))
        }
    }

    /// <p>Lists the IPSets of the GuardDuty service specified by the detector ID. If you use this operation from a member account, the IPSets returned are the IPSets from the associated administrator account.</p>
    #[allow(unused_mut)]
    async fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> Result<ListIPSetsResponse, RusotoError<ListIPSetsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListIPSetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListIPSetsError::from_response(response))
        }
    }

    /// <p>Lists all GuardDuty membership invitations that were sent to the current AWS account.</p>
    #[allow(unused_mut)]
    async fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> Result<ListInvitationsResponse, RusotoError<ListInvitationsError>> {
        let request_uri = "/invitation";

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInvitationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInvitationsError::from_response(response))
        }
    }

    /// <p>Lists details about all member accounts for the current GuardDuty administrator account.</p>
    #[allow(unused_mut)]
    async fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> Result<ListMembersResponse, RusotoError<ListMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.only_associated {
            params.put("onlyAssociated", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMembersError::from_response(response))
        }
    }

    /// <p>Lists the accounts configured as GuardDuty delegated administrators.</p>
    #[allow(unused_mut)]
    async fn list_organization_admin_accounts(
        &self,
        input: ListOrganizationAdminAccountsRequest,
    ) -> Result<
        ListOrganizationAdminAccountsResponse,
        RusotoError<ListOrganizationAdminAccountsError>,
    > {
        let request_uri = "/admin";

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListOrganizationAdminAccountsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListOrganizationAdminAccountsError::from_response(response))
        }
    }

    /// <p>Returns a list of publishing destinations associated with the specified <code>dectectorId</code>.</p>
    #[allow(unused_mut)]
    async fn list_publishing_destinations(
        &self,
        input: ListPublishingDestinationsRequest,
    ) -> Result<ListPublishingDestinationsResponse, RusotoError<ListPublishingDestinationsError>>
    {
        let request_uri = format!(
            "/detector/{detector_id}/publishingDestination",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPublishingDestinationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPublishingDestinationsError::from_response(response))
        }
    }

    /// <p>Lists tags for a resource. Tagging is currently supported for detectors, finding filters, IP sets, and threat intel sets, with a limit of 50 tags per resource. When invoked, this operation returns all assigned tags for a given resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Lists the ThreatIntelSets of the GuardDuty service specified by the detector ID. If you use this operation from a member account, the ThreatIntelSets associated with the administrator account are returned.</p>
    #[allow(unused_mut)]
    async fn list_threat_intel_sets(
        &self,
        input: ListThreatIntelSetsRequest,
    ) -> Result<ListThreatIntelSetsResponse, RusotoError<ListThreatIntelSetsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListThreatIntelSetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListThreatIntelSetsError::from_response(response))
        }
    }

    /// <p>Turns on GuardDuty monitoring of the specified member accounts. Use this operation to restart monitoring of accounts that you stopped monitoring with the <code>StopMonitoringMembers</code> operation.</p>
    #[allow(unused_mut)]
    async fn start_monitoring_members(
        &self,
        input: StartMonitoringMembersRequest,
    ) -> Result<StartMonitoringMembersResponse, RusotoError<StartMonitoringMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/start",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartMonitoringMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartMonitoringMembersError::from_response(response))
        }
    }

    /// <p>Stops GuardDuty monitoring for the specified member accounts. Use the <code>StartMonitoringMembers</code> operation to restart monitoring for those accounts.</p>
    #[allow(unused_mut)]
    async fn stop_monitoring_members(
        &self,
        input: StopMonitoringMembersRequest,
    ) -> Result<StopMonitoringMembersResponse, RusotoError<StopMonitoringMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/stop",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StopMonitoringMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopMonitoringMembersError::from_response(response))
        }
    }

    /// <p>Adds tags to a resource.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Unarchives GuardDuty findings specified by the <code>findingIds</code>.</p>
    #[allow(unused_mut)]
    async fn unarchive_findings(
        &self,
        input: UnarchiveFindingsRequest,
    ) -> Result<UnarchiveFindingsResponse, RusotoError<UnarchiveFindingsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/unarchive",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UnarchiveFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UnarchiveFindingsError::from_response(response))
        }
    }

    /// <p>Removes tags from a resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates the Amazon GuardDuty detector specified by the detectorId.</p>
    #[allow(unused_mut)]
    async fn update_detector(
        &self,
        input: UpdateDetectorRequest,
    ) -> Result<UpdateDetectorResponse, RusotoError<UpdateDetectorError>> {
        let request_uri = format!("/detector/{detector_id}", detector_id = input.detector_id);

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDetectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDetectorError::from_response(response))
        }
    }

    /// <p>Updates the filter specified by the filter name.</p>
    #[allow(unused_mut)]
    async fn update_filter(
        &self,
        input: UpdateFilterRequest,
    ) -> Result<UpdateFilterResponse, RusotoError<UpdateFilterError>> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = input.detector_id,
            filter_name = input.filter_name
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFilterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFilterError::from_response(response))
        }
    }

    /// <p>Marks the specified GuardDuty findings as useful or not useful.</p>
    #[allow(unused_mut)]
    async fn update_findings_feedback(
        &self,
        input: UpdateFindingsFeedbackRequest,
    ) -> Result<UpdateFindingsFeedbackResponse, RusotoError<UpdateFindingsFeedbackError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/feedback",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFindingsFeedbackResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFindingsFeedbackError::from_response(response))
        }
    }

    /// <p>Updates the IPSet specified by the IPSet ID.</p>
    #[allow(unused_mut)]
    async fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> Result<UpdateIPSetResponse, RusotoError<UpdateIPSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = input.detector_id,
            ip_set_id = input.ip_set_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateIPSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateIPSetError::from_response(response))
        }
    }

    /// <p>Contains information on member accounts to be updated.</p>
    #[allow(unused_mut)]
    async fn update_member_detectors(
        &self,
        input: UpdateMemberDetectorsRequest,
    ) -> Result<UpdateMemberDetectorsResponse, RusotoError<UpdateMemberDetectorsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/detector/update",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateMemberDetectorsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateMemberDetectorsError::from_response(response))
        }
    }

    /// <p>Updates the delegated administrator account with the values provided.</p>
    #[allow(unused_mut)]
    async fn update_organization_configuration(
        &self,
        input: UpdateOrganizationConfigurationRequest,
    ) -> Result<
        UpdateOrganizationConfigurationResponse,
        RusotoError<UpdateOrganizationConfigurationError>,
    > {
        let request_uri = format!(
            "/detector/{detector_id}/admin",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateOrganizationConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateOrganizationConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates information about the publishing destination specified by the <code>destinationId</code>.</p>
    #[allow(unused_mut)]
    async fn update_publishing_destination(
        &self,
        input: UpdatePublishingDestinationRequest,
    ) -> Result<UpdatePublishingDestinationResponse, RusotoError<UpdatePublishingDestinationError>>
    {
        let request_uri = format!(
            "/detector/{detector_id}/publishingDestination/{destination_id}",
            destination_id = input.destination_id,
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdatePublishingDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePublishingDestinationError::from_response(response))
        }
    }

    /// <p>Updates the ThreatIntelSet specified by the ThreatIntelSet ID.</p>
    #[allow(unused_mut)]
    async fn update_threat_intel_set(
        &self,
        input: UpdateThreatIntelSetRequest,
    ) -> Result<UpdateThreatIntelSetResponse, RusotoError<UpdateThreatIntelSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = input.detector_id,
            threat_intel_set_id = input.threat_intel_set_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateThreatIntelSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateThreatIntelSetError::from_response(response))
        }
    }
}
