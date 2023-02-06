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
    /// <p>The ARN of the behavior graph that the member account is accepting the invitation for.</p> <p>The member account status in the behavior graph must be <code>INVITED</code>.</p>
    #[serde(rename = "graphArn")]
    pub graph_arn: String,
}

/// <p>An AWS account that is the administrator account of or a member of a behavior graph.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Account {
    /// <p>The account identifier of the AWS account.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The AWS account root user email address for the AWS account.</p>
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGraphRequest {
    /// <p>The tags to assign to the new behavior graph. You can add up to 50 tags. For each tag, you provide the tag key and the tag value. Each tag key can contain up to 128 characters. Each tag value can contain up to 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGraphResponse {
    /// <p>The ARN of the new behavior graph.</p>
    #[serde(rename = "graphArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMembersRequest {
    /// <p>The list of AWS accounts to invite to become member accounts in the behavior graph. You can invite up to 50 accounts at a time. For each invited account, the account list contains the account identifier and the AWS account root user email address.</p>
    #[serde(rename = "accounts")]
    pub accounts: Vec<Account>,
    /// <p>if set to <code>true</code>, then the member accounts do not receive email notifications. By default, this is set to <code>false</code>, and the member accounts receive email notifications.</p>
    #[serde(rename = "disableEmailNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_email_notification: Option<bool>,
    /// <p>The ARN of the behavior graph to invite the member accounts to contribute their data to.</p>
    #[serde(rename = "graphArn")]
    pub graph_arn: String,
    /// <p>Customized message text to include in the invitation email message to the invited member accounts.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMembersResponse {
    /// <p>The set of member account invitation requests that Detective was able to process. This includes accounts that are being verified, that failed verification, and that passed verification and are being sent an invitation.</p>
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<MemberDetail>>,
    /// <p>The list of accounts for which Detective was unable to process the invitation request. For each account, the list provides the reason why the request could not be processed. The list includes accounts that are already member accounts in the behavior graph.</p>
    #[serde(rename = "unprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGraphRequest {
    /// <p>The ARN of the behavior graph to disable.</p>
    #[serde(rename = "graphArn")]
    pub graph_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMembersRequest {
    /// <p>The list of AWS account identifiers for the member accounts to delete from the behavior graph. You can delete up to 50 member accounts at a time.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// <p>The ARN of the behavior graph to delete members from.</p>
    #[serde(rename = "graphArn")]
    pub graph_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMembersResponse {
    /// <p>The list of AWS account identifiers for the member accounts that Detective successfully deleted from the behavior graph.</p>
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>The list of member accounts that Detective was not able to delete from the behavior graph. For each member account, provides the reason that the deletion could not be processed.</p>
    #[serde(rename = "unprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateMembershipRequest {
    /// <p>The ARN of the behavior graph to remove the member account from.</p> <p>The member account's member status in the behavior graph must be <code>ENABLED</code>.</p>
    #[serde(rename = "graphArn")]
    pub graph_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMembersRequest {
    /// <p>The list of AWS account identifiers for the member account for which to return member details. You can request details for up to 50 member accounts at a time.</p> <p>You cannot use <code>GetMembers</code> to retrieve information about member accounts that were removed from the behavior graph.</p>
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// <p>The ARN of the behavior graph for which to request the member details.</p>
    #[serde(rename = "graphArn")]
    pub graph_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMembersResponse {
    /// <p>The member account details that Detective is returning in response to the request.</p>
    #[serde(rename = "memberDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_details: Option<Vec<MemberDetail>>,
    /// <p>The requested member accounts for which Detective was unable to return member details.</p> <p>For each account, provides the reason why the request could not be processed.</p>
    #[serde(rename = "unprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

/// <p>A behavior graph in Detective.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Graph {
    /// <p>The ARN of the behavior graph.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the behavior graph was created. The value is in milliseconds since the epoch.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGraphsRequest {
    /// <p>The maximum number of graphs to return at a time. The total must be less than the overall limit on the number of results to return, which is currently 200.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For requests to get the next page of results, the pagination token that was returned with the previous set of results. The initial request does not include a pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGraphsResponse {
    /// <p>A list of behavior graphs that the account is an administrator account for.</p>
    #[serde(rename = "graphList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_list: Option<Vec<Graph>>,
    /// <p>If there are more behavior graphs remaining in the results, then this is the pagination token to use to request the next page of behavior graphs.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInvitationsRequest {
    /// <p>The maximum number of behavior graph invitations to return in the response. The total must be less than the overall limit on the number of results to return, which is currently 200.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For requests to retrieve the next page of results, the pagination token that was returned with the previous page of results. The initial request does not include a pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInvitationsResponse {
    /// <p>The list of behavior graphs for which the member account has open or accepted invitations.</p>
    #[serde(rename = "invitations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<MemberDetail>>,
    /// <p>If there are more behavior graphs remaining in the results, then this is the pagination token to use to request the next page of behavior graphs.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMembersRequest {
    /// <p>The ARN of the behavior graph for which to retrieve the list of member accounts.</p>
    #[serde(rename = "graphArn")]
    pub graph_arn: String,
    /// <p>The maximum number of member accounts to include in the response. The total must be less than the overall limit on the number of results to return, which is currently 200.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For requests to retrieve the next page of member account results, the pagination token that was returned with the previous page of results. The initial request does not include a pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMembersResponse {
    /// <p>The list of member accounts in the behavior graph.</p> <p>The results include member accounts that did not pass verification and member accounts that have not yet accepted the invitation to the behavior graph. The results do not include member accounts that were removed from the behavior graph.</p>
    #[serde(rename = "memberDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_details: Option<Vec<MemberDetail>>,
    /// <p>If there are more member accounts remaining in the results, then this is the pagination token to use to request the next page of member accounts.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the behavior graph for which to retrieve the tag values.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tag values that are assigned to the behavior graph. The request returns up to 50 tag values.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Details about a member account that was invited to contribute to a behavior graph.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MemberDetail {
    /// <p>The AWS account identifier for the member account.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The AWS account identifier of the administrator account for the behavior graph.</p>
    #[serde(rename = "administratorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator_id: Option<String>,
    /// <p><p>For member accounts with a status of <code>ACCEPTED<em>BUT</em>DISABLED</code>, the reason that the member account is not enabled.</p> <p>The reason can have one of the following values:</p> <ul> <li> <p> <code>VOLUME<em>TOO</em>HIGH</code> - Indicates that adding the member account would cause the data volume for the behavior graph to be too high.</p> </li> <li> <p> <code>VOLUME_UNKNOWN</code> - Indicates that Detective is unable to verify the data volume for the member account. This is usually because the member account is not enrolled in Amazon GuardDuty. </p> </li> </ul></p>
    #[serde(rename = "disabledReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// <p>The AWS account root user email address for the member account.</p>
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>The ARN of the behavior graph that the member account was invited to.</p>
    #[serde(rename = "graphArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_arn: Option<String>,
    /// <p>The date and time that Detective sent the invitation to the member account. The value is in milliseconds since the epoch.</p>
    #[serde(rename = "invitedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_time: Option<f64>,
    /// <p>The current membership status of the member account. The status can have one of the following values:</p> <ul> <li> <p> <code>INVITED</code> - Indicates that the member was sent an invitation but has not yet responded.</p> </li> <li> <p> <code>VERIFICATION_IN_PROGRESS</code> - Indicates that Detective is verifying that the account identifier and email address provided for the member account match. If they do match, then Detective sends the invitation. If the email address and account identifier don't match, then the member cannot be added to the behavior graph.</p> </li> <li> <p> <code>VERIFICATION_FAILED</code> - Indicates that the account and email address provided for the member account do not match, and Detective did not send an invitation to the account.</p> </li> <li> <p> <code>ENABLED</code> - Indicates that the member account accepted the invitation to contribute to the behavior graph.</p> </li> <li> <p> <code>ACCEPTED_BUT_DISABLED</code> - Indicates that the member account accepted the invitation but is prevented from contributing data to the behavior graph. <code>DisabledReason</code> provides the reason why the member account is not enabled.</p> </li> </ul> <p>Member accounts that declined an invitation or that were removed from the behavior graph are not included.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The date and time that the member account was last updated. The value is in milliseconds since the epoch.</p>
    #[serde(rename = "updatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<f64>,
    /// <p>The data volume in bytes per day for the member account.</p>
    #[serde(rename = "volumeUsageInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_usage_in_bytes: Option<i64>,
    /// <p>The data and time when the member account data volume was last updated.</p>
    #[serde(rename = "volumeUsageUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_usage_updated_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RejectInvitationRequest {
    /// <p>The ARN of the behavior graph to reject the invitation to.</p> <p>The member account's current member status in the behavior graph must be <code>INVITED</code>.</p>
    #[serde(rename = "graphArn")]
    pub graph_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartMonitoringMemberRequest {
    /// <p>The account ID of the member account to try to enable.</p> <p>The account must be an invited member account with a status of <code>ACCEPTED_BUT_DISABLED</code>. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The ARN of the behavior graph.</p>
    #[serde(rename = "graphArn")]
    pub graph_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the behavior graph to assign the tags to.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to assign to the behavior graph. You can add up to 50 tags. For each tag, you provide the tag key and the tag value. Each tag key can contain up to 128 characters. Each tag value can contain up to 256 characters.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>A member account that was included in a request but for which the request could not be processed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnprocessedAccount {
    /// <p>The AWS account identifier of the member account that was not processed.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The reason that the member account request could not be processed.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the behavior graph to remove the tags from.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys of the tags to remove from the behavior graph. You can remove up to 50 tags at a time.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// Errors returned by AcceptInvitation
#[derive(Debug, PartialEq)]
pub enum AcceptInvitationError {
    /// <p>The request attempted an invalid action.</p>
    Conflict(String),
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
}

impl AcceptInvitationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(AcceptInvitationError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(AcceptInvitationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AcceptInvitationError::ResourceNotFound(err.msg))
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
            AcceptInvitationError::Conflict(ref cause) => write!(f, "{}", cause),
            AcceptInvitationError::InternalServer(ref cause) => write!(f, "{}", cause),
            AcceptInvitationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AcceptInvitationError {}
/// Errors returned by CreateGraph
#[derive(Debug, PartialEq)]
pub enum CreateGraphError {
    /// <p>The request attempted an invalid action.</p>
    Conflict(String),
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p><p>This request cannot be completed for one of the following reasons.</p> <ul> <li> <p>The request would cause the number of member accounts in the behavior graph to exceed the maximum allowed. A behavior graph cannot have more than 1000 member accounts.</p> </li> <li> <p>The request would cause the data rate for the behavior graph to exceed the maximum allowed.</p> </li> <li> <p>Detective is unable to verify the data rate for the member account. This is usually because the member account is not enrolled in Amazon GuardDuty. </p> </li> </ul></p>
    ServiceQuotaExceeded(String),
}

impl CreateGraphError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGraphError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateGraphError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateGraphError::InternalServer(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateGraphError::ServiceQuotaExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGraphError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGraphError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateGraphError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateGraphError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGraphError {}
/// Errors returned by CreateMembers
#[derive(Debug, PartialEq)]
pub enum CreateMembersError {
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
    /// <p><p>This request cannot be completed for one of the following reasons.</p> <ul> <li> <p>The request would cause the number of member accounts in the behavior graph to exceed the maximum allowed. A behavior graph cannot have more than 1000 member accounts.</p> </li> <li> <p>The request would cause the data rate for the behavior graph to exceed the maximum allowed.</p> </li> <li> <p>Detective is unable to verify the data rate for the member account. This is usually because the member account is not enrolled in Amazon GuardDuty. </p> </li> </ul></p>
    ServiceQuotaExceeded(String),
}

impl CreateMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateMembersError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateMembersError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateMembersError::ServiceQuotaExceeded(err.msg))
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
            CreateMembersError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateMembersError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMembersError {}
/// Errors returned by DeleteGraph
#[derive(Debug, PartialEq)]
pub enum DeleteGraphError {
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
}

impl DeleteGraphError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGraphError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteGraphError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteGraphError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGraphError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGraphError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteGraphError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGraphError {}
/// Errors returned by DeleteMembers
#[derive(Debug, PartialEq)]
pub enum DeleteMembersError {
    /// <p>The request attempted an invalid action.</p>
    Conflict(String),
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
}

impl DeleteMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DeleteMembersError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteMembersError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteMembersError::ResourceNotFound(err.msg))
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
            DeleteMembersError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteMembersError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMembersError {}
/// Errors returned by DisassociateMembership
#[derive(Debug, PartialEq)]
pub enum DisassociateMembershipError {
    /// <p>The request attempted an invalid action.</p>
    Conflict(String),
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
}

impl DisassociateMembershipError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateMembershipError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DisassociateMembershipError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DisassociateMembershipError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateMembershipError::ResourceNotFound(
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
impl fmt::Display for DisassociateMembershipError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateMembershipError::Conflict(ref cause) => write!(f, "{}", cause),
            DisassociateMembershipError::InternalServer(ref cause) => write!(f, "{}", cause),
            DisassociateMembershipError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateMembershipError {}
/// Errors returned by GetMembers
#[derive(Debug, PartialEq)]
pub enum GetMembersError {
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
}

impl GetMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetMembersError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMembersError::ResourceNotFound(err.msg))
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
            GetMembersError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMembersError {}
/// Errors returned by ListGraphs
#[derive(Debug, PartialEq)]
pub enum ListGraphsError {
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
}

impl ListGraphsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGraphsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListGraphsError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGraphsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGraphsError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGraphsError {}
/// Errors returned by ListInvitations
#[derive(Debug, PartialEq)]
pub enum ListInvitationsError {
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
}

impl ListInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListInvitationsError::InternalServer(err.msg))
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
            ListInvitationsError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInvitationsError {}
/// Errors returned by ListMembers
#[derive(Debug, PartialEq)]
pub enum ListMembersError {
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
}

impl ListMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListMembersError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListMembersError::ResourceNotFound(err.msg))
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
            ListMembersError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMembersError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
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
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by RejectInvitation
#[derive(Debug, PartialEq)]
pub enum RejectInvitationError {
    /// <p>The request attempted an invalid action.</p>
    Conflict(String),
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
}

impl RejectInvitationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RejectInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(RejectInvitationError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(RejectInvitationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RejectInvitationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RejectInvitationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RejectInvitationError::Conflict(ref cause) => write!(f, "{}", cause),
            RejectInvitationError::InternalServer(ref cause) => write!(f, "{}", cause),
            RejectInvitationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RejectInvitationError {}
/// Errors returned by StartMonitoringMember
#[derive(Debug, PartialEq)]
pub enum StartMonitoringMemberError {
    /// <p>The request attempted an invalid action.</p>
    Conflict(String),
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
    /// <p><p>This request cannot be completed for one of the following reasons.</p> <ul> <li> <p>The request would cause the number of member accounts in the behavior graph to exceed the maximum allowed. A behavior graph cannot have more than 1000 member accounts.</p> </li> <li> <p>The request would cause the data rate for the behavior graph to exceed the maximum allowed.</p> </li> <li> <p>Detective is unable to verify the data rate for the member account. This is usually because the member account is not enrolled in Amazon GuardDuty. </p> </li> </ul></p>
    ServiceQuotaExceeded(String),
}

impl StartMonitoringMemberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartMonitoringMemberError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(StartMonitoringMemberError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(StartMonitoringMemberError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartMonitoringMemberError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(StartMonitoringMemberError::ServiceQuotaExceeded(
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
impl fmt::Display for StartMonitoringMemberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartMonitoringMemberError::Conflict(ref cause) => write!(f, "{}", cause),
            StartMonitoringMemberError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartMonitoringMemberError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartMonitoringMemberError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartMonitoringMemberError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
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
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The request was valid but failed because of a problem with the service.</p>
    InternalServer(String),
    /// <p>The request refers to a nonexistent resource.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
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
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the Amazon Detective API. Amazon Detective clients implement this trait.
#[async_trait]
pub trait Detective {
    /// <p>Accepts an invitation for the member account to contribute data to a behavior graph. This operation can only be called by an invited member account. </p> <p>The request provides the ARN of behavior graph.</p> <p>The member account status in the graph must be <code>INVITED</code>.</p>
    async fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Result<(), RusotoError<AcceptInvitationError>>;

    /// <p>Creates a new behavior graph for the calling account, and sets that account as the administrator account. This operation is called by the account that is enabling Detective.</p> <p>Before you try to enable Detective, make sure that your account has been enrolled in Amazon GuardDuty for at least 48 hours. If you do not meet this requirement, you cannot enable Detective. If you do meet the GuardDuty prerequisite, then when you make the request to enable Detective, it checks whether your data volume is within the Detective quota. If it exceeds the quota, then you cannot enable Detective. </p> <p>The operation also enables Detective for the calling account in the currently selected Region. It returns the ARN of the new behavior graph.</p> <p> <code>CreateGraph</code> triggers a process to create the corresponding data tables for the new behavior graph.</p> <p>An account can only be the administrator account for one behavior graph within a Region. If the same account calls <code>CreateGraph</code> with the same administrator account, it always returns the same behavior graph ARN. It does not create a new behavior graph.</p>
    async fn create_graph(
        &self,
        input: CreateGraphRequest,
    ) -> Result<CreateGraphResponse, RusotoError<CreateGraphError>>;

    /// <p><p>Sends a request to invite the specified AWS accounts to be member accounts in the behavior graph. This operation can only be called by the administrator account for a behavior graph. </p> <p> <code>CreateMembers</code> verifies the accounts and then invites the verified accounts. The administrator can optionally specify to not send invitation emails to the member accounts. This would be used when the administrator manages their member accounts centrally.</p> <p>The request provides the behavior graph ARN and the list of accounts to invite.</p> <p>The response separates the requested accounts into two lists:</p> <ul> <li> <p>The accounts that <code>CreateMembers</code> was able to start the verification for. This list includes member accounts that are being verified, that have passed verification and are to be invited, and that have failed verification.</p> </li> <li> <p>The accounts that <code>CreateMembers</code> was unable to process. This list includes accounts that were already invited to be member accounts in the behavior graph.</p> </li> </ul></p>
    async fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> Result<CreateMembersResponse, RusotoError<CreateMembersError>>;

    /// <p>Disables the specified behavior graph and queues it to be deleted. This operation removes the graph from each member account's list of behavior graphs.</p> <p> <code>DeleteGraph</code> can only be called by the administrator account for a behavior graph.</p>
    async fn delete_graph(
        &self,
        input: DeleteGraphRequest,
    ) -> Result<(), RusotoError<DeleteGraphError>>;

    /// <p>Deletes one or more member accounts from the administrator account's behavior graph. This operation can only be called by a Detective administrator account. That account cannot use <code>DeleteMembers</code> to delete their own account from the behavior graph. To disable a behavior graph, the administrator account uses the <code>DeleteGraph</code> API method.</p>
    async fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> Result<DeleteMembersResponse, RusotoError<DeleteMembersError>>;

    /// <p>Removes the member account from the specified behavior graph. This operation can only be called by a member account that has the <code>ENABLED</code> status.</p>
    async fn disassociate_membership(
        &self,
        input: DisassociateMembershipRequest,
    ) -> Result<(), RusotoError<DisassociateMembershipError>>;

    /// <p>Returns the membership details for specified member accounts for a behavior graph.</p>
    async fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> Result<GetMembersResponse, RusotoError<GetMembersError>>;

    /// <p>Returns the list of behavior graphs that the calling account is an administrator account of. This operation can only be called by an administrator account.</p> <p>Because an account can currently only be the administrator of one behavior graph within a Region, the results always contain a single behavior graph.</p>
    async fn list_graphs(
        &self,
        input: ListGraphsRequest,
    ) -> Result<ListGraphsResponse, RusotoError<ListGraphsError>>;

    /// <p>Retrieves the list of open and accepted behavior graph invitations for the member account. This operation can only be called by a member account.</p> <p>Open invitations are invitations that the member account has not responded to.</p> <p>The results do not include behavior graphs for which the member account declined the invitation. The results also do not include behavior graphs that the member account resigned from or was removed from.</p>
    async fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> Result<ListInvitationsResponse, RusotoError<ListInvitationsError>>;

    /// <p>Retrieves the list of member accounts for a behavior graph. Does not return member accounts that were removed from the behavior graph.</p>
    async fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> Result<ListMembersResponse, RusotoError<ListMembersError>>;

    /// <p>Returns the tag values that are assigned to a behavior graph.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Rejects an invitation to contribute the account data to a behavior graph. This operation must be called by a member account that has the <code>INVITED</code> status.</p>
    async fn reject_invitation(
        &self,
        input: RejectInvitationRequest,
    ) -> Result<(), RusotoError<RejectInvitationError>>;

    /// <p><p>Sends a request to enable data ingest for a member account that has a status of <code>ACCEPTED<em>BUT</em>DISABLED</code>.</p> <p>For valid member accounts, the status is updated as follows.</p> <ul> <li> <p>If Detective enabled the member account, then the new status is <code>ENABLED</code>.</p> </li> <li> <p>If Detective cannot enable the member account, the status remains <code>ACCEPTED<em>BUT</em>DISABLED</code>. </p> </li> </ul></p>
    async fn start_monitoring_member(
        &self,
        input: StartMonitoringMemberRequest,
    ) -> Result<(), RusotoError<StartMonitoringMemberError>>;

    /// <p>Applies tag values to a behavior graph.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes tags from a behavior graph.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;
}
/// A client for the Amazon Detective API.
#[derive(Clone)]
pub struct DetectiveClient {
    client: Client,
    region: region::Region,
}

impl DetectiveClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DetectiveClient {
        DetectiveClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DetectiveClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DetectiveClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DetectiveClient {
        DetectiveClient { client, region }
    }
}

#[async_trait]
impl Detective for DetectiveClient {
    /// <p>Accepts an invitation for the member account to contribute data to a behavior graph. This operation can only be called by an invited member account. </p> <p>The request provides the ARN of behavior graph.</p> <p>The member account status in the graph must be <code>INVITED</code>.</p>
    #[allow(unused_mut)]
    async fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Result<(), RusotoError<AcceptInvitationError>> {
        let request_uri = "/invitation";

        let mut request = SignedRequest::new("PUT", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AcceptInvitationError::from_response(response))
        }
    }

    /// <p>Creates a new behavior graph for the calling account, and sets that account as the administrator account. This operation is called by the account that is enabling Detective.</p> <p>Before you try to enable Detective, make sure that your account has been enrolled in Amazon GuardDuty for at least 48 hours. If you do not meet this requirement, you cannot enable Detective. If you do meet the GuardDuty prerequisite, then when you make the request to enable Detective, it checks whether your data volume is within the Detective quota. If it exceeds the quota, then you cannot enable Detective. </p> <p>The operation also enables Detective for the calling account in the currently selected Region. It returns the ARN of the new behavior graph.</p> <p> <code>CreateGraph</code> triggers a process to create the corresponding data tables for the new behavior graph.</p> <p>An account can only be the administrator account for one behavior graph within a Region. If the same account calls <code>CreateGraph</code> with the same administrator account, it always returns the same behavior graph ARN. It does not create a new behavior graph.</p>
    #[allow(unused_mut)]
    async fn create_graph(
        &self,
        input: CreateGraphRequest,
    ) -> Result<CreateGraphResponse, RusotoError<CreateGraphError>> {
        let request_uri = "/graph";

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
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
                .deserialize::<CreateGraphResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateGraphError::from_response(response))
        }
    }

    /// <p><p>Sends a request to invite the specified AWS accounts to be member accounts in the behavior graph. This operation can only be called by the administrator account for a behavior graph. </p> <p> <code>CreateMembers</code> verifies the accounts and then invites the verified accounts. The administrator can optionally specify to not send invitation emails to the member accounts. This would be used when the administrator manages their member accounts centrally.</p> <p>The request provides the behavior graph ARN and the list of accounts to invite.</p> <p>The response separates the requested accounts into two lists:</p> <ul> <li> <p>The accounts that <code>CreateMembers</code> was able to start the verification for. This list includes member accounts that are being verified, that have passed verification and are to be invited, and that have failed verification.</p> </li> <li> <p>The accounts that <code>CreateMembers</code> was unable to process. This list includes accounts that were already invited to be member accounts in the behavior graph.</p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> Result<CreateMembersResponse, RusotoError<CreateMembersError>> {
        let request_uri = "/graph/members";

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
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
                .deserialize::<CreateMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMembersError::from_response(response))
        }
    }

    /// <p>Disables the specified behavior graph and queues it to be deleted. This operation removes the graph from each member account's list of behavior graphs.</p> <p> <code>DeleteGraph</code> can only be called by the administrator account for a behavior graph.</p>
    #[allow(unused_mut)]
    async fn delete_graph(
        &self,
        input: DeleteGraphRequest,
    ) -> Result<(), RusotoError<DeleteGraphError>> {
        let request_uri = "/graph/removal";

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGraphError::from_response(response))
        }
    }

    /// <p>Deletes one or more member accounts from the administrator account's behavior graph. This operation can only be called by a Detective administrator account. That account cannot use <code>DeleteMembers</code> to delete their own account from the behavior graph. To disable a behavior graph, the administrator account uses the <code>DeleteGraph</code> API method.</p>
    #[allow(unused_mut)]
    async fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> Result<DeleteMembersResponse, RusotoError<DeleteMembersError>> {
        let request_uri = "/graph/members/removal";

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
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
                .deserialize::<DeleteMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMembersError::from_response(response))
        }
    }

    /// <p>Removes the member account from the specified behavior graph. This operation can only be called by a member account that has the <code>ENABLED</code> status.</p>
    #[allow(unused_mut)]
    async fn disassociate_membership(
        &self,
        input: DisassociateMembershipRequest,
    ) -> Result<(), RusotoError<DisassociateMembershipError>> {
        let request_uri = "/membership/removal";

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateMembershipError::from_response(response))
        }
    }

    /// <p>Returns the membership details for specified member accounts for a behavior graph.</p>
    #[allow(unused_mut)]
    async fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> Result<GetMembersResponse, RusotoError<GetMembersError>> {
        let request_uri = "/graph/members/get";

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
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
                .deserialize::<GetMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMembersError::from_response(response))
        }
    }

    /// <p>Returns the list of behavior graphs that the calling account is an administrator account of. This operation can only be called by an administrator account.</p> <p>Because an account can currently only be the administrator of one behavior graph within a Region, the results always contain a single behavior graph.</p>
    #[allow(unused_mut)]
    async fn list_graphs(
        &self,
        input: ListGraphsRequest,
    ) -> Result<ListGraphsResponse, RusotoError<ListGraphsError>> {
        let request_uri = "/graphs/list";

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
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
                .deserialize::<ListGraphsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListGraphsError::from_response(response))
        }
    }

    /// <p>Retrieves the list of open and accepted behavior graph invitations for the member account. This operation can only be called by a member account.</p> <p>Open invitations are invitations that the member account has not responded to.</p> <p>The results do not include behavior graphs for which the member account declined the invitation. The results also do not include behavior graphs that the member account resigned from or was removed from.</p>
    #[allow(unused_mut)]
    async fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> Result<ListInvitationsResponse, RusotoError<ListInvitationsError>> {
        let request_uri = "/invitations/list";

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
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
                .deserialize::<ListInvitationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInvitationsError::from_response(response))
        }
    }

    /// <p>Retrieves the list of member accounts for a behavior graph. Does not return member accounts that were removed from the behavior graph.</p>
    #[allow(unused_mut)]
    async fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> Result<ListMembersResponse, RusotoError<ListMembersError>> {
        let request_uri = "/graph/members/list";

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
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
                .deserialize::<ListMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMembersError::from_response(response))
        }
    }

    /// <p>Returns the tag values that are assigned to a behavior graph.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());

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

    /// <p>Rejects an invitation to contribute the account data to a behavior graph. This operation must be called by a member account that has the <code>INVITED</code> status.</p>
    #[allow(unused_mut)]
    async fn reject_invitation(
        &self,
        input: RejectInvitationRequest,
    ) -> Result<(), RusotoError<RejectInvitationError>> {
        let request_uri = "/invitation/removal";

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RejectInvitationError::from_response(response))
        }
    }

    /// <p><p>Sends a request to enable data ingest for a member account that has a status of <code>ACCEPTED<em>BUT</em>DISABLED</code>.</p> <p>For valid member accounts, the status is updated as follows.</p> <ul> <li> <p>If Detective enabled the member account, then the new status is <code>ENABLED</code>.</p> </li> <li> <p>If Detective cannot enable the member account, the status remains <code>ACCEPTED<em>BUT</em>DISABLED</code>. </p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn start_monitoring_member(
        &self,
        input: StartMonitoringMemberRequest,
    ) -> Result<(), RusotoError<StartMonitoringMemberError>> {
        let request_uri = "/graph/member/monitoringstate";

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartMonitoringMemberError::from_response(response))
        }
    }

    /// <p>Applies tag values to a behavior graph.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());
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

    /// <p>Removes tags from a behavior graph.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "detective", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.detective".to_string());

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
}
