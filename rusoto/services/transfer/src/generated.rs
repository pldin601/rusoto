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

impl TransferClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "transfer", &self.region, request_uri);

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
pub struct CreateAccessRequest {
    /// <p>A unique identifier that is required to identify specific groups within your directory. The users of the group that you associate have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Amazon Web Services Transfer Family. If you know the group name, you can view the SID values by running the following command using Windows PowerShell.</p> <p> <code>Get-ADGroup -Filter {samAccountName -like "<i>YourGroupName</i>*"} -Properties * | Select SamAccountName,ObjectSid</code> </p> <p>In that command, replace <i>YourGroupName</i> with the name of your Active Directory group.</p> <p>The regex used to validate this parameter is a string of characters consisting of uppercase and lowercase alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p>
    #[serde(rename = "externalId")]
    pub external_id: String,
    /// <p>The landing directory (folder) for a user when they log in to the server using the client.</p> <p>A <code>HomeDirectory</code> example is <code>/bucket_name/home/mydirectory</code>.</p>
    #[serde(rename = "homeDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    /// <p><p>Logical directory mappings that specify what Amazon S3 or Amazon EFS paths and keys should be visible to your user and how you want to make them visible. You must specify the <code>Entry</code> and <code>Target</code> pair, where <code>Entry</code> shows how the path is made visible and <code>Target</code> is the actual Amazon S3 or Amazon EFS path. If you only specify a target, it is displayed as is. You also must ensure that your Amazon Web Services Identity and Access Management (IAM) role provides access to paths in <code>Target</code>. This value can only be set when <code>HomeDirectoryType</code> is set to <i>LOGICAL</i>.</p> <p>The following is an <code>Entry</code> and <code>Target</code> pair example.</p> <p> <code>[ { &quot;Entry&quot;: &quot;your-personal-report.pdf&quot;, &quot;Target&quot;: &quot;/bucket3/customized-reports/${transfer:UserName}.pdf&quot; } ]</code> </p> <p>In most cases, you can use this value instead of the scope-down policy to lock down your user to the designated home directory (&quot;<code>chroot</code>&quot;). To do this, you can set <code>Entry</code> to <code>/</code> and set <code>Target</code> to the <code>HomeDirectory</code> parameter value.</p> <p>The following is an <code>Entry</code> and <code>Target</code> pair example for <code>chroot</code>.</p> <p> <code>[ { &quot;Entry:&quot;: &quot;/&quot;, &quot;Target&quot;: &quot;/bucket_name/home/mydirectory&quot; } ]</code> </p> <note> <p>If the target of a logical directory entry does not exist in Amazon S3 or EFS, the entry is ignored. As a workaround, you can use the Amazon S3 API or EFS API to create 0 byte objects as place holders for your directory. If using the CLI, use the <code>s3api</code> or <code>efsapi</code> call instead of <code>s3</code> or <code>efs</code> so you can use the put-object operation. For example, you use the following: <code>aws s3api put-object --bucket bucketname --key path/to/folder/</code>. Make sure that the end of the key name ends in a <code>/</code> for it to be considered a folder.</p> </note></p>
    #[serde(rename = "homeDirectoryMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    /// <p>The type of landing directory (folder) you want your users' home directory to be when they log into the server. If you set it to <code>PATH</code>, the user will see the absolute Amazon S3 bucket or EFS paths as is in their file transfer protocol clients. If you set it <code>LOGICAL</code>, you will need to provide mappings in the <code>HomeDirectoryMappings</code> for how you want to make Amazon S3 or EFS paths visible to your users.</p>
    #[serde(rename = "homeDirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    /// <p><p>A scope-down policy for your user so that you can use the same IAM role across multiple users. This policy scopes down user access to portions of their Amazon S3 bucket. Variables that you can use inside this policy include <code>${Transfer:UserName}</code>, <code>${Transfer:HomeDirectory}</code>, and <code>${Transfer:HomeBucket}</code>.</p> <note> <p>This only applies when domain of <code>ServerId</code> is S3. Amazon EFS does not use scope-down policies.</p> <p>For scope-down policies, Amazon Web Services Transfer Family stores the policy as a JSON blob, instead of the Amazon Resource Name (ARN) of the policy. You save the policy as a JSON blob and pass it in the <code>Policy</code> argument.</p> <p>For an example of a scope-down policy, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/scope-down-policy.html">Example scope-down policy</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html">AssumeRole</a> in the <i>Amazon Web Services Security Token Service API Reference</i>.</p> </note></p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "posixProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    /// <p>Specifies the Amazon Resource Name (ARN) of the IAM role that controls your users' access to your Amazon S3 bucket or EFS file system. The policies attached to this role determine the level of access that you want to provide your users when transferring files into and out of your Amazon S3 bucket or EFS file system. The IAM role should also contain a trust relationship that allows the server to access your resources when servicing your users' transfer requests.</p>
    #[serde(rename = "role")]
    pub role: String,
    /// <p>A system-assigned unique identifier for a server instance. This is the specific server that you added your user to.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAccessResponse {
    /// <p>The external ID of the group whose users have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Amazon Web Services Transfer Family.</p>
    #[serde(rename = "externalId")]
    pub external_id: String,
    /// <p>The ID of the server that the user is attached to.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateServerRequest {
    /// <p><p>The Amazon Resource Name (ARN) of the Amazon Web Services Certificate Manager (ACM) certificate. Required when <code>Protocols</code> is set to <code>FTPS</code>.</p> <p>To request a new public certificate, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-request-public.html">Request a public certificate</a> in the <i> Amazon Web Services Certificate Manager User Guide</i>.</p> <p>To import an existing certificate into ACM, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/import-certificate.html">Importing certificates into ACM</a> in the <i> Amazon Web Services Certificate Manager User Guide</i>.</p> <p>To request a private certificate to use FTPS through private IP addresses, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-request-private.html">Request a private certificate</a> in the <i> Amazon Web Services Certificate Manager User Guide</i>.</p> <p>Certificates with the following cryptographic algorithms and key sizes are supported:</p> <ul> <li> <p>2048-bit RSA (RSA<em>2048)</p> </li> <li> <p>4096-bit RSA (RSA</em>4096)</p> </li> <li> <p>Elliptic Prime Curve 256 bit (EC<em>prime256v1)</p> </li> <li> <p>Elliptic Prime Curve 384 bit (EC</em>secp384r1)</p> </li> <li> <p>Elliptic Prime Curve 521 bit (EC_secp521r1)</p> </li> </ul> <note> <p>The certificate must be a valid SSL/TLS X.509 version 3 certificate with FQDN or IP address specified and information about the issuer.</p> </note></p>
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p><p>The domain of the storage system that is used for file transfers. There are two domains available: Amazon Simple Storage Service (Amazon S3) and Amazon Elastic File System (Amazon EFS). The default value is S3.</p> <note> <p>After the server is created, the domain cannot be changed.</p> </note></p>
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The virtual private cloud (VPC) endpoint settings that are configured for your server. When you host your endpoint within your VPC, you can make it accessible only to resources within your VPC, or you can attach Elastic IP addresses and make it accessible to clients over the internet. Your VPC's default security groups are automatically assigned to your endpoint.</p>
    #[serde(rename = "endpointDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_details: Option<EndpointDetails>,
    /// <p><p>The type of endpoint that you want your server to use. You can choose to make your server&#39;s endpoint publicly accessible (PUBLIC) or host it inside your VPC. With an endpoint that is hosted in a VPC, you can restrict access to your server and resources only within your VPC or choose to make it internet facing by attaching Elastic IP addresses directly to it.</p> <note> <p> After May 19, 2021, you won&#39;t be able to create a server using <code>EndpointType=VPC<em>ENDPOINT</code> in your Amazon Web Services account if your account hasn&#39;t already done so before May 19, 2021. If you have already created servers with <code>EndpointType=VPC</em>ENDPOINT</code> in your Amazon Web Services account on or before May 19, 2021, you will not be affected. After this date, use <code>EndpointType</code>=<code>VPC</code>.</p> <p>For more information, see https://docs.aws.amazon.com/transfer/latest/userguide/create-server-in-vpc.html#deprecate-vpc-endpoint.</p> <p>It is recommended that you use <code>VPC</code> as the <code>EndpointType</code>. With this endpoint type, you have the option to directly associate up to three Elastic IPv4 addresses (BYO IP included) with your server&#39;s endpoint and use VPC security groups to restrict traffic by the client&#39;s public IP address. This is not possible with <code>EndpointType</code> set to <code>VPC_ENDPOINT</code>.</p> </note></p>
    #[serde(rename = "endpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>The RSA private key as generated by the <code>ssh-keygen -N "" -m PEM -f my-new-server-key</code> command.</p> <important> <p>If you aren't planning to migrate existing users from an existing SFTP-enabled server to a new server, don't update the host key. Accidentally changing a server's host key can be disruptive.</p> </important> <p>For more information, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/edit-server-config.html#configuring-servers-change-host-key">Change the host key for your SFTP-enabled server</a> in the <i>Amazon Web Services Transfer Family User Guide</i>.</p>
    #[serde(rename = "hostKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key: Option<String>,
    /// <p>Required when <code>IdentityProviderType</code> is set to <code>AWS_DIRECTORY_SERVICE</code> or <code>API_GATEWAY</code>. Accepts an array containing all of the information required to use a directory in <code>AWS_DIRECTORY_SERVICE</code> or invoke a customer-supplied authentication API, including the API Gateway URL. Not required when <code>IdentityProviderType</code> is set to <code>SERVICE_MANAGED</code>.</p>
    #[serde(rename = "identityProviderDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_details: Option<IdentityProviderDetails>,
    /// <p>Specifies the mode of authentication for a server. The default value is <code>SERVICE_MANAGED</code>, which allows you to store and access user credentials within the Amazon Web Services Transfer Family service.</p> <p>Use <code>AWS_DIRECTORY_SERVICE</code> to provide access to Active Directory groups in Amazon Web Services Managed Active Directory or Microsoft Active Directory in your on-premises environment or in Amazon Web Services using AD Connectors. This option also requires you to provide a Directory ID using the <code>IdentityProviderDetails</code> parameter.</p> <p>Use the <code>API_GATEWAY</code> value to integrate with an identity provider of your choosing. The <code>API_GATEWAY</code> setting requires you to provide an API Gateway endpoint URL to call for authentication using the <code>IdentityProviderDetails</code> parameter.</p>
    #[serde(rename = "identityProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_type: Option<String>,
    /// <p>Specifies the Amazon Resource Name (ARN) of the Amazon Web Services Identity and Access Management (IAM) role that allows a server to turn on Amazon CloudWatch logging for Amazon S3 or Amazon EFS events. When set, user activity can be viewed in your CloudWatch logs.</p>
    #[serde(rename = "loggingRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
    /// <p><p>Specifies the file transfer protocol or protocols over which your file transfer protocol client can connect to your server&#39;s endpoint. The available protocols are:</p> <ul> <li> <p> <code>SFTP</code> (Secure Shell (SSH) File Transfer Protocol): File transfer over SSH</p> </li> <li> <p> <code>FTPS</code> (File Transfer Protocol Secure): File transfer with TLS encryption</p> </li> <li> <p> <code>FTP</code> (File Transfer Protocol): Unencrypted file transfer</p> </li> </ul> <note> <p>If you select <code>FTPS</code>, you must choose a certificate stored in Amazon Web Services Certificate Manager (ACM) which is used to identify your server when clients connect to it over FTPS.</p> <p>If <code>Protocol</code> includes either <code>FTP</code> or <code>FTPS</code>, then the <code>EndpointType</code> must be <code>VPC</code> and the <code>IdentityProviderType</code> must be <code>AWS<em>DIRECTORY</em>SERVICE</code> or <code>API<em>GATEWAY</code>.</p> <p>If <code>Protocol</code> includes <code>FTP</code>, then <code>AddressAllocationIds</code> cannot be associated.</p> <p>If <code>Protocol</code> is set only to <code>SFTP</code>, the <code>EndpointType</code> can be set to <code>PUBLIC</code> and the <code>IdentityProviderType</code> can be set to <code>SERVICE</em>MANAGED</code>.</p> </note></p>
    #[serde(rename = "protocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    /// <p>Specifies the name of the security policy that is attached to the server.</p>
    #[serde(rename = "securityPolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_name: Option<String>,
    /// <p>Key-value pairs that can be used to group and search for servers.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateServerResponse {
    /// <p>The service-assigned ID of the server that is created.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserRequest {
    /// <p>The landing directory (folder) for a user when they log in to the server using the client.</p> <p>A <code>HomeDirectory</code> example is <code>/bucket_name/home/mydirectory</code>.</p>
    #[serde(rename = "homeDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    /// <p><p>Logical directory mappings that specify what Amazon S3 or Amazon EFS paths and keys should be visible to your user and how you want to make them visible. You must specify the <code>Entry</code> and <code>Target</code> pair, where <code>Entry</code> shows how the path is made visible and <code>Target</code> is the actual Amazon S3 or Amazon EFS path. If you only specify a target, it is displayed as is. You also must ensure that your Amazon Web Services Identity and Access Management (IAM) role provides access to paths in <code>Target</code>. This value can only be set when <code>HomeDirectoryType</code> is set to <i>LOGICAL</i>.</p> <p>The following is an <code>Entry</code> and <code>Target</code> pair example.</p> <p> <code>[ { &quot;Entry&quot;: &quot;your-personal-report.pdf&quot;, &quot;Target&quot;: &quot;/bucket3/customized-reports/${transfer:UserName}.pdf&quot; } ]</code> </p> <p>In most cases, you can use this value instead of the scope-down policy to lock your user down to the designated home directory (&quot;<code>chroot</code>&quot;). To do this, you can set <code>Entry</code> to <code>/</code> and set <code>Target</code> to the HomeDirectory parameter value.</p> <p>The following is an <code>Entry</code> and <code>Target</code> pair example for <code>chroot</code>.</p> <p> <code>[ { &quot;Entry:&quot;: &quot;/&quot;, &quot;Target&quot;: &quot;/bucket_name/home/mydirectory&quot; } ]</code> </p> <note> <p>If the target of a logical directory entry does not exist in Amazon S3 or EFS, the entry is ignored. As a workaround, you can use the Amazon S3 API or EFS API to create 0 byte objects as place holders for your directory. If using the CLI, use the <code>s3api</code> or <code>efsapi</code> call instead of <code>s3</code> or <code>efs</code> so you can use the put-object operation. For example, you use the following: <code>aws s3api put-object --bucket bucketname --key path/to/folder/</code>. Make sure that the end of the key name ends in a <code>/</code> for it to be considered a folder.</p> </note></p>
    #[serde(rename = "homeDirectoryMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    /// <p>The type of landing directory (folder) you want your users' home directory to be when they log into the server. If you set it to <code>PATH</code>, the user will see the absolute Amazon S3 bucket or EFS paths as is in their file transfer protocol clients. If you set it <code>LOGICAL</code>, you will need to provide mappings in the <code>HomeDirectoryMappings</code> for how you want to make Amazon S3 or EFS paths visible to your users.</p>
    #[serde(rename = "homeDirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    /// <p><p>A scope-down policy for your user so that you can use the same IAM role across multiple users. This policy scopes down user access to portions of their Amazon S3 bucket. Variables that you can use inside this policy include <code>${Transfer:UserName}</code>, <code>${Transfer:HomeDirectory}</code>, and <code>${Transfer:HomeBucket}</code>.</p> <note> <p>This only applies when domain of ServerId is S3. EFS does not use scope down policy.</p> <p>For scope-down policies, Amazon Web Services Transfer Family stores the policy as a JSON blob, instead of the Amazon Resource Name (ARN) of the policy. You save the policy as a JSON blob and pass it in the <code>Policy</code> argument.</p> <p>For an example of a scope-down policy, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/scope-down-policy.html">Example scope-down policy</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html">AssumeRole</a> in the <i>Amazon Web Services Security Token Service API Reference</i>.</p> </note></p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>Specifies the full POSIX identity, including user ID (<code>Uid</code>), group ID (<code>Gid</code>), and any secondary groups IDs (<code>SecondaryGids</code>), that controls your users' access to your Amazon EFS file systems. The POSIX permissions that are set on files and directories in Amazon EFS determine the level of access your users get when transferring files into and out of your Amazon EFS file systems.</p>
    #[serde(rename = "posixProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    /// <p>Specifies the Amazon Resource Name (ARN) of the IAM role that controls your users' access to your Amazon S3 bucket or EFS file system. The policies attached to this role determine the level of access that you want to provide your users when transferring files into and out of your Amazon S3 bucket or EFS file system. The IAM role should also contain a trust relationship that allows the server to access your resources when servicing your users' transfer requests.</p>
    #[serde(rename = "role")]
    pub role: String,
    /// <p>A system-assigned unique identifier for a server instance. This is the specific server that you added your user to.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p>The public portion of the Secure Shell (SSH) key used to authenticate the user to the server.</p>
    #[serde(rename = "sshPublicKeyBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key_body: Option<String>,
    /// <p>Key-value pairs that can be used to group and search for users. Tags are metadata attached to users for any purpose.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A unique string that identifies a user and is associated with a as specified by the <code>ServerId</code>. This user name must be a minimum of 3 and a maximum of 100 characters long. The following are valid characters: a-z, A-Z, 0-9, underscore '_', hyphen '-', period '.', and at sign '@'. The user name can't start with a hyphen, period, or at sign.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserResponse {
    /// <p>The ID of the server that the user is attached to.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p>A unique string that identifies a user account associated with a server.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAccessRequest {
    /// <p>A unique identifier that is required to identify specific groups within your directory. The users of the group that you associate have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Amazon Web Services Transfer Family. If you know the group name, you can view the SID values by running the following command using Windows PowerShell.</p> <p> <code>Get-ADGroup -Filter {samAccountName -like "<i>YourGroupName</i>*"} -Properties * | Select SamAccountName,ObjectSid</code> </p> <p>In that command, replace <i>YourGroupName</i> with the name of your Active Directory group.</p> <p>The regex used to validate this parameter is a string of characters consisting of uppercase and lowercase alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p>
    #[serde(rename = "externalId")]
    pub external_id: String,
    /// <p>A system-assigned unique identifier for a server that has this user assigned.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteServerRequest {
    /// <p>A unique system-assigned identifier for a server instance.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSshPublicKeyRequest {
    /// <p>A system-assigned unique identifier for a file transfer protocol-enabled server instance that has the user assigned to it.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p>A unique identifier used to reference your user's specific SSH key.</p>
    #[serde(rename = "sshPublicKeyId")]
    pub ssh_public_key_id: String,
    /// <p>A unique string that identifies a user whose public key is being deleted.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserRequest {
    /// <p>A system-assigned unique identifier for a server instance that has the user assigned to it.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p>A unique string that identifies a user that is being deleted from a server.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAccessRequest {
    /// <p>A unique identifier that is required to identify specific groups within your directory. The users of the group that you associate have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Amazon Web Services Transfer Family. If you know the group name, you can view the SID values by running the following command using Windows PowerShell.</p> <p> <code>Get-ADGroup -Filter {samAccountName -like "<i>YourGroupName</i>*"} -Properties * | Select SamAccountName,ObjectSid</code> </p> <p>In that command, replace <i>YourGroupName</i> with the name of your Active Directory group.</p> <p>The regex used to validate this parameter is a string of characters consisting of uppercase and lowercase alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p>
    #[serde(rename = "externalId")]
    pub external_id: String,
    /// <p>A system-assigned unique identifier for a server that has this access assigned.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAccessResponse {
    /// <p>The external ID of the server that the access is attached to.</p>
    #[serde(rename = "access")]
    pub access: DescribedAccess,
    /// <p>A system-assigned unique identifier for a server that has this access assigned.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSecurityPolicyRequest {
    /// <p>Specifies the name of the security policy that is attached to the server.</p>
    #[serde(rename = "securityPolicyName")]
    pub security_policy_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSecurityPolicyResponse {
    /// <p>An array containing the properties of the security policy.</p>
    #[serde(rename = "securityPolicy")]
    pub security_policy: DescribedSecurityPolicy,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeServerRequest {
    /// <p>A system-assigned unique identifier for a server.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeServerResponse {
    /// <p>An array containing the properties of a server with the <code>ServerID</code> you specified.</p>
    #[serde(rename = "server")]
    pub server: DescribedServer,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserRequest {
    /// <p>A system-assigned unique identifier for a server that has this user assigned.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p>The name of the user assigned to one or more servers. User names are part of the sign-in credentials to use the Amazon Web Services Transfer Family service and perform file transfer tasks.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserResponse {
    /// <p>A system-assigned unique identifier for a server that has this user assigned.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p>An array containing the properties of the user account for the <code>ServerID</code> value that you specified.</p>
    #[serde(rename = "user")]
    pub user: DescribedUser,
}

/// <p>Describes the properties of the access that was specified.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribedAccess {
    /// <p>A unique identifier that is required to identify specific groups within your directory. The users of the group that you associate have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Amazon Web Services Transfer Family. If you know the group name, you can view the SID values by running the following command using Windows PowerShell.</p> <p> <code>Get-ADGroup -Filter {samAccountName -like "<i>YourGroupName</i>*"} -Properties * | Select SamAccountName,ObjectSid</code> </p> <p>In that command, replace <i>YourGroupName</i> with the name of your Active Directory group.</p> <p>The regex used to validate this parameter is a string of characters consisting of uppercase and lowercase alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p>
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The landing directory (folder) for a user when they log in to the server using the client.</p> <p>A <code>HomeDirectory</code> example is <code>/bucket_name/home/mydirectory</code>.</p>
    #[serde(rename = "homeDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    /// <p>Logical directory mappings that specify what Amazon S3 or Amazon EFS paths and keys should be visible to your user and how you want to make them visible. You must specify the <code>Entry</code> and <code>Target</code> pair, where <code>Entry</code> shows how the path is made visible and <code>Target</code> is the actual Amazon S3 or Amazon EFS path. If you only specify a target, it is displayed as is. You also must ensure that your Amazon Web Services Identity and Access Management (IAM) role provides access to paths in <code>Target</code>. This value can only be set when <code>HomeDirectoryType</code> is set to <i>LOGICAL</i>.</p> <p>In most cases, you can use this value instead of the scope-down policy to lock down the associated access to the designated home directory ("<code>chroot</code>"). To do this, you can set <code>Entry</code> to '/' and set <code>Target</code> to the <code>HomeDirectory</code> parameter value.</p>
    #[serde(rename = "homeDirectoryMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    /// <p>The type of landing directory (folder) you want your users' home directory to be when they log into the server. If you set it to <code>PATH</code>, the user will see the absolute Amazon S3 bucket or EFS paths as is in their file transfer protocol clients. If you set it <code>LOGICAL</code>, you will need to provide mappings in the <code>HomeDirectoryMappings</code> for how you want to make Amazon S3 or EFS paths visible to your users.</p>
    #[serde(rename = "homeDirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    /// <p>A scope-down policy for your user so that you can use the same IAM role across multiple users. This policy scopes down user access to portions of their Amazon S3 bucket. Variables that you can use inside this policy include <code>${Transfer:UserName}</code>, <code>${Transfer:HomeDirectory}</code>, and <code>${Transfer:HomeBucket}</code>.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "posixProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    /// <p>Specifies the Amazon Resource Name (ARN) of the IAM role that controls your users' access to your Amazon S3 bucket or EFS file system. The policies attached to this role determine the level of access that you want to provide your users when transferring files into and out of your Amazon S3 bucket or EFS file system. The IAM role should also contain a trust relationship that allows the server to access your resources when servicing your users' transfer requests.</p>
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// <p>Describes the properties of a security policy that was specified. For more information about security policies, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/security-policies.html">Working with security policies</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribedSecurityPolicy {
    /// <p>Specifies whether this policy enables Federal Information Processing Standards (FIPS).</p>
    #[serde(rename = "fips")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fips: Option<bool>,
    /// <p>Specifies the name of the security policy that is attached to the server.</p>
    #[serde(rename = "securityPolicyName")]
    pub security_policy_name: String,
    /// <p>Specifies the enabled Secure Shell (SSH) cipher encryption algorithms in the security policy that is attached to the server.</p>
    #[serde(rename = "sshCiphers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_ciphers: Option<Vec<String>>,
    /// <p>Specifies the enabled SSH key exchange (KEX) encryption algorithms in the security policy that is attached to the server.</p>
    #[serde(rename = "sshKexs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_kexs: Option<Vec<String>>,
    /// <p>Specifies the enabled SSH message authentication code (MAC) encryption algorithms in the security policy that is attached to the server.</p>
    #[serde(rename = "sshMacs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_macs: Option<Vec<String>>,
    /// <p>Specifies the enabled Transport Layer Security (TLS) cipher encryption algorithms in the security policy that is attached to the server.</p>
    #[serde(rename = "tlsCiphers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_ciphers: Option<Vec<String>>,
}

/// <p>Describes the properties of a file transfer protocol-enabled server that was specified.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribedServer {
    /// <p>Specifies the unique Amazon Resource Name (ARN) of the server.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>Specifies the ARN of the Amazon Web ServicesCertificate Manager (ACM) certificate. Required when <code>Protocols</code> is set to <code>FTPS</code>.</p>
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>Specifies the domain of the storage system that is used for file transfers.</p>
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The virtual private cloud (VPC) endpoint settings that are configured for your server. When you host your endpoint within your VPC, you can make it accessible only to resources within your VPC, or you can attach Elastic IP addresses and make it accessible to clients over the internet. Your VPC's default security groups are automatically assigned to your endpoint.</p>
    #[serde(rename = "endpointDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_details: Option<EndpointDetails>,
    /// <p>Defines the type of endpoint that your server is connected to. If your server is connected to a VPC endpoint, your server isn't accessible over the public internet.</p>
    #[serde(rename = "endpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>Specifies the Base64-encoded SHA256 fingerprint of the server's host key. This value is equivalent to the output of the <code>ssh-keygen -l -f my-new-server-key</code> command.</p>
    #[serde(rename = "hostKeyFingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key_fingerprint: Option<String>,
    /// <p>Specifies information to call a customer-supplied authentication API. This field is not populated when the <code>IdentityProviderType</code> of a server is <code>AWS_DIRECTORY_SERVICE</code> or <code>SERVICE_MANAGED</code>.</p>
    #[serde(rename = "identityProviderDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_details: Option<IdentityProviderDetails>,
    /// <p>Specifies the mode of authentication for a server. The default value is <code>SERVICE_MANAGED</code>, which allows you to store and access user credentials within the Amazon Web Services Transfer Family service.</p> <p>Use <code>AWS_DIRECTORY_SERVICE</code> to provide access to Active Directory groups in Amazon Web Services Managed Active Directory or Microsoft Active Directory in your on-premises environment or in Amazon Web Services using AD Connectors. This option also requires you to provide a Directory ID using the <code>IdentityProviderDetails</code> parameter.</p> <p>Use the <code>API_GATEWAY</code> value to integrate with an identity provider of your choosing. The <code>API_GATEWAY</code> setting requires you to provide an API Gateway endpoint URL to call for authentication using the <code>IdentityProviderDetails</code> parameter.</p>
    #[serde(rename = "identityProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_type: Option<String>,
    /// <p>Specifies the Amazon Resource Name (ARN) of the Amazon Web Services Identity and Access Management (IAM) role that allows a server to turn on Amazon CloudWatch logging for Amazon S3 or Amazon EFS events. When set, user activity can be viewed in your CloudWatch logs.</p>
    #[serde(rename = "loggingRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
    /// <p> The protocol settings that are configured for your server. </p> <p> Use the <code>PassiveIp</code> parameter to indicate passive mode. Enter a single dotted-quad IPv4 address, such as the external IP address of a firewall, router, or load balancer. </p>
    #[serde(rename = "protocolDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_details: Option<ProtocolDetails>,
    /// <p><p>Specifies the file transfer protocol or protocols over which your file transfer protocol client can connect to your server&#39;s endpoint. The available protocols are:</p> <ul> <li> <p> <code>SFTP</code> (Secure Shell (SSH) File Transfer Protocol): File transfer over SSH</p> </li> <li> <p> <code>FTPS</code> (File Transfer Protocol Secure): File transfer with TLS encryption</p> </li> <li> <p> <code>FTP</code> (File Transfer Protocol): Unencrypted file transfer</p> </li> </ul></p>
    #[serde(rename = "protocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    /// <p>Specifies the name of the security policy that is attached to the server.</p>
    #[serde(rename = "securityPolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_name: Option<String>,
    /// <p>Specifies the unique system-assigned identifier for a server that you instantiate.</p>
    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// <p>Specifies the condition of a server for the server that was described. A value of <code>ONLINE</code> indicates that the server can accept jobs and transfer files. A <code>State</code> value of <code>OFFLINE</code> means that the server cannot perform file transfer operations.</p> <p>The states of <code>STARTING</code> and <code>STOPPING</code> indicate that the server is in an intermediate state, either not fully able to respond, or not fully offline. The values of <code>START_FAILED</code> or <code>STOP_FAILED</code> can indicate an error condition.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Specifies the key-value pairs that you can use to search for and group servers that were assigned to the server that was described.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Specifies the number of users that are assigned to a server you specified with the <code>ServerId</code>.</p>
    #[serde(rename = "userCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i64>,
}

/// <p>Describes the properties of a user that was specified.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribedUser {
    /// <p>Specifies the unique Amazon Resource Name (ARN) for the user that was requested to be described.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The landing directory (folder) for a user when they log in to the server using the client.</p> <p>A <code>HomeDirectory</code> example is <code>/bucket_name/home/mydirectory</code>.</p>
    #[serde(rename = "homeDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    /// <p>Logical directory mappings that specify what Amazon S3 or Amazon EFS paths and keys should be visible to your user and how you want to make them visible. You must specify the <code>Entry</code> and <code>Target</code> pair, where <code>Entry</code> shows how the path is made visible and <code>Target</code> is the actual Amazon S3 or Amazon EFS path. If you only specify a target, it is displayed as is. You also must ensure that your Amazon Web Services Identity and Access Management (IAM) role provides access to paths in <code>Target</code>. This value can only be set when <code>HomeDirectoryType</code> is set to <i>LOGICAL</i>.</p> <p>In most cases, you can use this value instead of the scope-down policy to lock your user down to the designated home directory ("<code>chroot</code>"). To do this, you can set <code>Entry</code> to '/' and set <code>Target</code> to the HomeDirectory parameter value.</p>
    #[serde(rename = "homeDirectoryMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    /// <p>The type of landing directory (folder) you want your users' home directory to be when they log into the server. If you set it to <code>PATH</code>, the user will see the absolute Amazon S3 bucket or EFS paths as is in their file transfer protocol clients. If you set it <code>LOGICAL</code>, you will need to provide mappings in the <code>HomeDirectoryMappings</code> for how you want to make Amazon S3 or EFS paths visible to your users.</p>
    #[serde(rename = "homeDirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    /// <p>A scope-down policy for your user so that you can use the same IAM role across multiple users. This policy scopes down user access to portions of their Amazon S3 bucket. Variables that you can use inside this policy include <code>${Transfer:UserName}</code>, <code>${Transfer:HomeDirectory}</code>, and <code>${Transfer:HomeBucket}</code>.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>Specifies the full POSIX identity, including user ID (<code>Uid</code>), group ID (<code>Gid</code>), and any secondary groups IDs (<code>SecondaryGids</code>), that controls your users' access to your Amazon Elastic File System (Amazon EFS) file systems. The POSIX permissions that are set on files and directories in your file system determine the level of access your users get when transferring files into and out of your Amazon EFS file systems.</p>
    #[serde(rename = "posixProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    /// <p>Specifies the Amazon Resource Name (ARN) of the IAM role that controls your users' access to your Amazon S3 bucket or EFS file system. The policies attached to this role determine the level of access that you want to provide your users when transferring files into and out of your Amazon S3 bucket or EFS file system. The IAM role should also contain a trust relationship that allows the server to access your resources when servicing your users' transfer requests.</p>
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>Specifies the public key portion of the Secure Shell (SSH) keys stored for the described user.</p>
    #[serde(rename = "sshPublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_keys: Option<Vec<SshPublicKey>>,
    /// <p>Specifies the key-value pairs for the user requested. Tag can be used to search for and group users for a variety of purposes.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Specifies the name of the user that was requested to be described. User names are used for authentication purposes. This is the string that will be used by your user when they log in to your server.</p>
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p><p>The virtual private cloud (VPC) endpoint settings that are configured for your file transfer protocol-enabled server. With a VPC endpoint, you can restrict access to your server and resources only within your VPC. To control incoming internet traffic, invoke the <code>UpdateServer</code> API and attach an Elastic IP address to your server&#39;s endpoint.</p> <note> <p> After May 19, 2021, you won&#39;t be able to create a server using <code>EndpointType=VPC<em>ENDPOINT</code> in your Amazon Web Servicesaccount if your account hasn&#39;t already done so before May 19, 2021. If you have already created servers with <code>EndpointType=VPC</em>ENDPOINT</code> in your Amazon Web Servicesaccount on or before May 19, 2021, you will not be affected. After this date, use <code>EndpointType</code>=<code>VPC</code>.</p> <p>For more information, see https://docs.aws.amazon.com/transfer/latest/userguide/create-server-in-vpc.html#deprecate-vpc-endpoint.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EndpointDetails {
    /// <p><p>A list of address allocation IDs that are required to attach an Elastic IP address to your server&#39;s endpoint.</p> <note> <p>This property can only be set when <code>EndpointType</code> is set to <code>VPC</code> and it is only valid in the <code>UpdateServer</code> API.</p> </note></p>
    #[serde(rename = "addressAllocationIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_allocation_ids: Option<Vec<String>>,
    /// <p><p>A list of security groups IDs that are available to attach to your server&#39;s endpoint.</p> <note> <p>This property can only be set when <code>EndpointType</code> is set to <code>VPC</code>.</p> <p>You can edit the <code>SecurityGroupIds</code> property in the <a href="https://docs.aws.amazon.com/transfer/latest/userguide/API_UpdateServer.html">UpdateServer</a> API only if you are changing the <code>EndpointType</code> from <code>PUBLIC</code> or <code>VPC<em>ENDPOINT</code> to <code>VPC</code>. To change security groups associated with your server&#39;s VPC endpoint after creation, use the Amazon EC2 &lt;a href=&quot;https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API</em>ModifyVpcEndpoint.html&quot;&gt;ModifyVpcEndpoint</a> API.</p> </note></p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p><p>A list of subnet IDs that are required to host your server endpoint in your VPC.</p> <note> <p>This property can only be set when <code>EndpointType</code> is set to <code>VPC</code>.</p> </note></p>
    #[serde(rename = "subnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p><p>The ID of the VPC endpoint.</p> <note> <p>This property can only be set when <code>EndpointType</code> is set to <code>VPC_ENDPOINT</code>.</p> <p>For more information, see https://docs.aws.amazon.com/transfer/latest/userguide/create-server-in-vpc.html#deprecate-vpc-endpoint.</p> </note></p>
    #[serde(rename = "vpcEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
    /// <p><p>The VPC ID of the VPC in which a server&#39;s endpoint will be hosted.</p> <note> <p>This property can only be set when <code>EndpointType</code> is set to <code>VPC</code>.</p> </note></p>
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p><p>Represents an object that contains entries and targets for <code>HomeDirectoryMappings</code>.</p> <p>The following is an <code>Entry</code> and <code>Target</code> pair example for <code>chroot</code>.</p> <p> <code>[ { &quot;Entry:&quot;: &quot;/&quot;, &quot;Target&quot;: &quot;/bucket_name/home/mydirectory&quot; } ]</code> </p> <note> <p>If the target of a logical directory entry does not exist in Amazon S3 or EFS, the entry is ignored. As a workaround, you can use the Amazon S3 API or EFS API to create 0 byte objects as place holders for your directory. If using the CLI, use the <code>s3api</code> or <code>efsapi</code> call instead of <code>s3</code> or <code>efs</code> so you can use the put-object operation. For example, you use the following: <code>aws s3api put-object --bucket bucketname --key path/to/folder/</code>. Make sure that the end of the key name ends in a <code>/</code> for it to be considered a folder.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HomeDirectoryMapEntry {
    /// <p>Represents an entry for <code>HomeDirectoryMappings</code>.</p>
    #[serde(rename = "entry")]
    pub entry: String,
    /// <p>Represents the map target that is used in a <code>HomeDirectorymapEntry</code>.</p>
    #[serde(rename = "target")]
    pub target: String,
}

/// <p>Returns information related to the type of user authentication that is in use for a file transfer protocol-enabled server's users. A server can have only one method of authentication.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IdentityProviderDetails {
    /// <p>The identifier of the Amazon Web ServicesDirectory Service directory that you want to stop sharing.</p>
    #[serde(rename = "directoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>Provides the type of <code>InvocationRole</code> used to authenticate the user account.</p>
    #[serde(rename = "invocationRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_role: Option<String>,
    /// <p>Provides the location of the service endpoint used to authenticate users.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportSshPublicKeyRequest {
    /// <p>A system-assigned unique identifier for a server.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p>The public key portion of an SSH key pair.</p>
    #[serde(rename = "sshPublicKeyBody")]
    pub ssh_public_key_body: String,
    /// <p>The name of the user account that is assigned to one or more servers.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

/// <p>Identifies the user, the server they belong to, and the identifier of the SSH public key associated with that user. A user can have more than one key on each server that they are associated with.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportSshPublicKeyResponse {
    /// <p>A system-assigned unique identifier for a server.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p>The name given to a public key by the system that was imported.</p>
    #[serde(rename = "sshPublicKeyId")]
    pub ssh_public_key_id: String,
    /// <p>A user name assigned to the <code>ServerID</code> value that you specified.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAccessesRequest {
    /// <p>Specifies the maximum number of access SIDs to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>When you can get additional results from the <code>ListAccesses</code> call, a <code>NextToken</code> parameter is returned in the output. You can then pass in a subsequent command to the <code>NextToken</code> parameter to continue listing additional accesses.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A system-assigned unique identifier for a server that has users assigned to it.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAccessesResponse {
    /// <p>Returns the accesses and their properties for the <code>ServerId</code> value that you specify.</p>
    #[serde(rename = "accesses")]
    pub accesses: Vec<ListedAccess>,
    /// <p>When you can get additional results from the <code>ListAccesses</code> call, a <code>NextToken</code> parameter is returned in the output. You can then pass in a subsequent command to the <code>NextToken</code> parameter to continue listing additional accesses.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A system-assigned unique identifier for a server that has users assigned to it.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSecurityPoliciesRequest {
    /// <p>Specifies the number of security policies to return as a response to the <code>ListSecurityPolicies</code> query.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>When additional results are obtained from the <code>ListSecurityPolicies</code> command, a <code>NextToken</code> parameter is returned in the output. You can then pass the <code>NextToken</code> parameter in a subsequent command to continue listing additional security policies.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSecurityPoliciesResponse {
    /// <p>When you can get additional results from the <code>ListSecurityPolicies</code> operation, a <code>NextToken</code> parameter is returned in the output. In a following command, you can pass in the <code>NextToken</code> parameter to continue listing security policies.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of security policies that were listed.</p>
    #[serde(rename = "securityPolicyNames")]
    pub security_policy_names: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServersRequest {
    /// <p>Specifies the number of servers to return as a response to the <code>ListServers</code> query.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>When additional results are obtained from the <code>ListServers</code> command, a <code>NextToken</code> parameter is returned in the output. You can then pass the <code>NextToken</code> parameter in a subsequent command to continue listing additional servers.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServersResponse {
    /// <p>When you can get additional results from the <code>ListServers</code> operation, a <code>NextToken</code> parameter is returned in the output. In a following command, you can pass in the <code>NextToken</code> parameter to continue listing additional servers.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of servers that were listed.</p>
    #[serde(rename = "servers")]
    pub servers: Vec<ListedServer>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>Requests the tags associated with a particular Amazon Resource Name (ARN). An ARN is an identifier for a specific Amazon Web Services resource, such as a server, user, or role.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>Specifies the number of tags to return as a response to the <code>ListTagsForResource</code> request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>When you request additional results from the <code>ListTagsForResource</code> operation, a <code>NextToken</code> parameter is returned in the input. You can then pass in a subsequent command to the <code>NextToken</code> parameter to continue listing additional tags.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The ARN you specified to list the tags of.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>When you can get additional results from the <code>ListTagsForResource</code> call, a <code>NextToken</code> parameter is returned in the output. You can then pass in a subsequent command to the <code>NextToken</code> parameter to continue listing additional tags.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Key-value pairs that are assigned to a resource, usually for the purpose of grouping and searching for items. Tags are metadata that you define.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUsersRequest {
    /// <p>Specifies the number of users to return as a response to the <code>ListUsers</code> request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>When you can get additional results from the <code>ListUsers</code> call, a <code>NextToken</code> parameter is returned in the output. You can then pass in a subsequent command to the <code>NextToken</code> parameter to continue listing additional users.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A system-assigned unique identifier for a server that has users assigned to it.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUsersResponse {
    /// <p>When you can get additional results from the <code>ListUsers</code> call, a <code>NextToken</code> parameter is returned in the output. You can then pass in a subsequent command to the <code>NextToken</code> parameter to continue listing additional users.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A system-assigned unique identifier for a server that the users are assigned to.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p>Returns the user accounts and their properties for the <code>ServerId</code> value that you specify.</p>
    #[serde(rename = "users")]
    pub users: Vec<ListedUser>,
}

/// <p>Lists the properties for one or more specified associated accesses.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListedAccess {
    /// <p>A unique identifier that is required to identify specific groups within your directory. The users of the group that you associate have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Amazon Web Services Transfer Family. If you know the group name, you can view the SID values by running the following command using Windows PowerShell.</p> <p> <code>Get-ADGroup -Filter {samAccountName -like "<i>YourGroupName</i>*"} -Properties * | Select SamAccountName,ObjectSid</code> </p> <p>In that command, replace <i>YourGroupName</i> with the name of your Active Directory group.</p> <p>The regex used to validate this parameter is a string of characters consisting of uppercase and lowercase alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p>
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The landing directory (folder) for a user when they log in to the server using the client.</p> <p>A <code>HomeDirectory</code> example is <code>/bucket_name/home/mydirectory</code>.</p>
    #[serde(rename = "homeDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    /// <p>The type of landing directory (folder) you want your users' home directory to be when they log into the server. If you set it to <code>PATH</code>, the user will see the absolute Amazon S3 bucket or EFS paths as is in their file transfer protocol clients. If you set it <code>LOGICAL</code>, you will need to provide mappings in the <code>HomeDirectoryMappings</code> for how you want to make Amazon S3 or EFS paths visible to your users.</p>
    #[serde(rename = "homeDirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    /// <p>Specifies the Amazon Resource Name (ARN) of the IAM role that controls your users' access to your Amazon S3 bucket or EFS file system. The policies attached to this role determine the level of access that you want to provide your users when transferring files into and out of your Amazon S3 bucket or EFS file system. The IAM role should also contain a trust relationship that allows the server to access your resources when servicing your users' transfer requests.</p>
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// <p>Returns properties of a file transfer protocol-enabled server that was specified.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListedServer {
    /// <p>Specifies the unique Amazon Resource Name (ARN) for a server to be listed.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>Specifies the domain of the storage system that is used for file transfers.</p>
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>Specifies the type of VPC endpoint that your server is connected to. If your server is connected to a VPC endpoint, your server isn't accessible over the public internet.</p>
    #[serde(rename = "endpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>Specifies the mode of authentication for a server. The default value is <code>SERVICE_MANAGED</code>, which allows you to store and access user credentials within the Amazon Web Services Transfer Family service.</p> <p>Use <code>AWS_DIRECTORY_SERVICE</code> to provide access to Active Directory groups in Amazon Web Services Managed Active Directory or Microsoft Active Directory in your on-premises environment or in Amazon Web Services using AD Connectors. This option also requires you to provide a Directory ID using the <code>IdentityProviderDetails</code> parameter.</p> <p>Use the <code>API_GATEWAY</code> value to integrate with an identity provider of your choosing. The <code>API_GATEWAY</code> setting requires you to provide an API Gateway endpoint URL to call for authentication using the <code>IdentityProviderDetails</code> parameter.</p>
    #[serde(rename = "identityProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_type: Option<String>,
    /// <p>Specifies the Amazon Resource Name (ARN) of the Amazon Web Services Identity and Access Management (IAM) role that allows a server to turn on Amazon CloudWatch logging for Amazon S3 or Amazon EFS events. When set, user activity can be viewed in your CloudWatch logs.</p>
    #[serde(rename = "loggingRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
    /// <p>Specifies the unique system assigned identifier for the servers that were listed.</p>
    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// <p>Specifies the condition of a server for the server that was described. A value of <code>ONLINE</code> indicates that the server can accept jobs and transfer files. A <code>State</code> value of <code>OFFLINE</code> means that the server cannot perform file transfer operations.</p> <p>The states of <code>STARTING</code> and <code>STOPPING</code> indicate that the server is in an intermediate state, either not fully able to respond, or not fully offline. The values of <code>START_FAILED</code> or <code>STOP_FAILED</code> can indicate an error condition.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Specifies the number of users that are assigned to a server you specified with the <code>ServerId</code>.</p>
    #[serde(rename = "userCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i64>,
}

/// <p>Returns properties of the user that you specify.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListedUser {
    /// <p>Provides the unique Amazon Resource Name (ARN) for the user that you want to learn about.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The landing directory (folder) for a user when they log in to the server using the client.</p> <p>A <code>HomeDirectory</code> example is <code>/bucket_name/home/mydirectory</code>.</p>
    #[serde(rename = "homeDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    /// <p>The type of landing directory (folder) you want your users' home directory to be when they log into the server. If you set it to <code>PATH</code>, the user will see the absolute Amazon S3 bucket or EFS paths as is in their file transfer protocol clients. If you set it <code>LOGICAL</code>, you will need to provide mappings in the <code>HomeDirectoryMappings</code> for how you want to make Amazon S3 or EFS paths visible to your users.</p>
    #[serde(rename = "homeDirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    /// <p><p>Specifies the Amazon Resource Name (ARN) of the IAM role that controls your users&#39; access to your Amazon S3 bucket or EFS file system. The policies attached to this role determine the level of access that you want to provide your users when transferring files into and out of your Amazon S3 bucket or EFS file system. The IAM role should also contain a trust relationship that allows the server to access your resources when servicing your users&#39; transfer requests.</p> <note> <p>The IAM role that controls your users&#39; access to your Amazon S3 bucket for servers with <code>Domain=S3</code>, or your EFS file system for servers with <code>Domain=EFS</code>. </p> <p>The policies attached to this role determine the level of access you want to provide your users when transferring files into and out of your S3 buckets or EFS file systems.</p> </note></p>
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>Specifies the number of SSH public keys stored for the user you specified.</p>
    #[serde(rename = "sshPublicKeyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key_count: Option<i64>,
    /// <p>Specifies the name of the user whose ARN was specified. User names are used for authentication purposes.</p>
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>The full POSIX identity, including user ID (<code>Uid</code>), group ID (<code>Gid</code>), and any secondary groups IDs (<code>SecondaryGids</code>), that controls your users' access to your Amazon EFS file systems. The POSIX permissions that are set on files and directories in your file system determine the level of access your users get when transferring files into and out of your Amazon EFS file systems.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PosixProfile {
    /// <p>The POSIX group ID used for all EFS operations by this user.</p>
    #[serde(rename = "gid")]
    pub gid: i64,
    /// <p>The secondary POSIX group IDs used for all EFS operations by this user.</p>
    #[serde(rename = "secondaryGids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_gids: Option<Vec<i64>>,
    /// <p>The POSIX user ID used for all EFS operations by this user.</p>
    #[serde(rename = "uid")]
    pub uid: i64,
}

/// <p><p> The protocol settings that are configured for your server. </p> <note> <p> This type is only valid in the <code>UpdateServer</code> API. </p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProtocolDetails {
    /// <p> Indicates passive mode, for FTP and FTPS protocols. Enter a single dotted-quad IPv4 address, such as the external IP address of a firewall, router, or load balancer. For example: </p> <p> <code> aws transfer update-server --protocol-details PassiveIp=<i>0.0.0.0</i> </code> </p> <p>Replace <code> <i>0.0.0.0</i> </code> in the example above with the actual IP address you want to use.</p>
    #[serde(rename = "passiveIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passive_ip: Option<String>,
}

/// <p>Provides information about the public Secure Shell (SSH) key that is associated with a user account for the specific file transfer protocol-enabled server (as identified by <code>ServerId</code>). The information returned includes the date the key was imported, the public key contents, and the public key ID. A user can store more than one SSH public key associated with their user name on a specific server.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SshPublicKey {
    /// <p>Specifies the date that the public key was added to the user account.</p>
    #[serde(rename = "dateImported")]
    pub date_imported: f64,
    /// <p>Specifies the content of the SSH public key as specified by the <code>PublicKeyId</code>.</p>
    #[serde(rename = "sshPublicKeyBody")]
    pub ssh_public_key_body: String,
    /// <p>Specifies the <code>SshPublicKeyId</code> parameter contains the identifier of the public key.</p>
    #[serde(rename = "sshPublicKeyId")]
    pub ssh_public_key_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartServerRequest {
    /// <p>A system-assigned unique identifier for a server that you start.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopServerRequest {
    /// <p>A system-assigned unique identifier for a server that you stopped.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

/// <p>Creates a key-value pair for a specific resource. Tags are metadata that you can use to search for and group a resource for various purposes. You can apply tags to servers, users, and roles. A tag key can take more than one value. For example, to group servers for accounting purposes, you might create a tag called <code>Group</code> and assign the values <code>Research</code> and <code>Accounting</code> to that group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The name assigned to the tag that you create.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>Contains one or more values that you assigned to the key name you create.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>An Amazon Resource Name (ARN) for a specific Amazon Web Services resource, such as a server, user, or role.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>Key-value pairs assigned to ARNs that you can use to group and search for resources by type. You can attach this metadata to user accounts for any purpose.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TestIdentityProviderRequest {
    /// <p>A system-assigned identifier for a specific server. That server's user authentication method is tested with a user name and password.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p><p>The type of file transfer protocol to be tested.</p> <p>The available protocols are:</p> <ul> <li> <p>Secure Shell (SSH) File Transfer Protocol (SFTP)</p> </li> <li> <p>File Transfer Protocol Secure (FTPS)</p> </li> <li> <p>File Transfer Protocol (FTP)</p> </li> </ul></p>
    #[serde(rename = "serverProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_protocol: Option<String>,
    /// <p>The source IP address of the user account to be tested.</p>
    #[serde(rename = "sourceIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    /// <p>The name of the user account to be tested.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
    /// <p>The password of the user account to be tested.</p>
    #[serde(rename = "userPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_password: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestIdentityProviderResponse {
    /// <p>A message that indicates whether the test was successful or not.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The response that is returned from your API Gateway.</p>
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// <p>The HTTP status code that is the response from your API Gateway.</p>
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    /// <p>The endpoint of the service used to authenticate a user.</p>
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The value of the resource that will have the tag removed. An Amazon Resource Name (ARN) is an identifier for a specific Amazon Web Services resource, such as a server, user, or role.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>TagKeys are key-value pairs assigned to ARNs that can be used to group and search for resources by type. This metadata can be attached to resources for any purpose.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAccessRequest {
    /// <p>A unique identifier that is required to identify specific groups within your directory. The users of the group that you associate have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Amazon Web Services Transfer Family. If you know the group name, you can view the SID values by running the following command using Windows PowerShell.</p> <p> <code>Get-ADGroup -Filter {samAccountName -like "<i>YourGroupName</i>*"} -Properties * | Select SamAccountName,ObjectSid</code> </p> <p>In that command, replace <i>YourGroupName</i> with the name of your Active Directory group.</p> <p>The regex used to validate this parameter is a string of characters consisting of uppercase and lowercase alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p>
    #[serde(rename = "externalId")]
    pub external_id: String,
    /// <p>The landing directory (folder) for a user when they log in to the server using the client.</p> <p>A <code>HomeDirectory</code> example is <code>/bucket_name/home/mydirectory</code>.</p>
    #[serde(rename = "homeDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    /// <p><p>Logical directory mappings that specify what Amazon S3 or Amazon EFS paths and keys should be visible to your user and how you want to make them visible. You must specify the <code>Entry</code> and <code>Target</code> pair, where <code>Entry</code> shows how the path is made visible and <code>Target</code> is the actual Amazon S3 or Amazon EFS path. If you only specify a target, it is displayed as is. You also must ensure that your Amazon Web Services Identity and Access Management (IAM) role provides access to paths in <code>Target</code>. This value can only be set when <code>HomeDirectoryType</code> is set to <i>LOGICAL</i>.</p> <p>The following is an <code>Entry</code> and <code>Target</code> pair example.</p> <p> <code>[ { &quot;Entry&quot;: &quot;your-personal-report.pdf&quot;, &quot;Target&quot;: &quot;/bucket3/customized-reports/${transfer:UserName}.pdf&quot; } ]</code> </p> <p>In most cases, you can use this value instead of the scope-down policy to lock down your user to the designated home directory (&quot;<code>chroot</code>&quot;). To do this, you can set <code>Entry</code> to <code>/</code> and set <code>Target</code> to the <code>HomeDirectory</code> parameter value.</p> <p>The following is an <code>Entry</code> and <code>Target</code> pair example for <code>chroot</code>.</p> <p> <code>[ { &quot;Entry:&quot;: &quot;/&quot;, &quot;Target&quot;: &quot;/bucket_name/home/mydirectory&quot; } ]</code> </p> <note> <p>If the target of a logical directory entry does not exist in Amazon S3 or EFS, the entry is ignored. As a workaround, you can use the Amazon S3 API or EFS API to create 0 byte objects as place holders for your directory. If using the CLI, use the <code>s3api</code> or <code>efsapi</code> call instead of <code>s3</code> or <code>efs</code> so you can use the put-object operation. For example, you use the following: <code>aws s3api put-object --bucket bucketname --key path/to/folder/</code>. Make sure that the end of the key name ends in a <code>/</code> for it to be considered a folder.</p> </note></p>
    #[serde(rename = "homeDirectoryMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    /// <p>The type of landing directory (folder) you want your users' home directory to be when they log into the server. If you set it to <code>PATH</code>, the user will see the absolute Amazon S3 bucket or EFS paths as is in their file transfer protocol clients. If you set it <code>LOGICAL</code>, you will need to provide mappings in the <code>HomeDirectoryMappings</code> for how you want to make Amazon S3 or EFS paths visible to your users.</p>
    #[serde(rename = "homeDirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    /// <p><p>A scope-down policy for your user so that you can use the same IAM role across multiple users. This policy scopes down user access to portions of their Amazon S3 bucket. Variables that you can use inside this policy include <code>${Transfer:UserName}</code>, <code>${Transfer:HomeDirectory}</code>, and <code>${Transfer:HomeBucket}</code>.</p> <note> <p>This only applies when domain of <code>ServerId</code> is S3. Amazon EFS does not use scope down policy.</p> <p>For scope-down policies, Amazon Web ServicesTransfer Family stores the policy as a JSON blob, instead of the Amazon Resource Name (ARN) of the policy. You save the policy as a JSON blob and pass it in the <code>Policy</code> argument.</p> <p>For an example of a scope-down policy, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/scope-down-policy.html">Example scope-down policy</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html">AssumeRole</a> in the <i>Amazon Web ServicesSecurity Token Service API Reference</i>.</p> </note></p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "posixProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    /// <p>Specifies the Amazon Resource Name (ARN) of the IAM role that controls your users' access to your Amazon S3 bucket or EFS file system. The policies attached to this role determine the level of access that you want to provide your users when transferring files into and out of your Amazon S3 bucket or EFS file system. The IAM role should also contain a trust relationship that allows the server to access your resources when servicing your users' transfer requests.</p>
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>A system-assigned unique identifier for a server instance. This is the specific server that you added your user to.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAccessResponse {
    /// <p>The external ID of the group whose users have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Amazon Web ServicesTransfer Family.</p>
    #[serde(rename = "externalId")]
    pub external_id: String,
    /// <p>The ID of the server that the user is attached to.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateServerRequest {
    /// <p><p>The Amazon Resource Name (ARN) of the Amazon Web ServicesCertificate Manager (ACM) certificate. Required when <code>Protocols</code> is set to <code>FTPS</code>.</p> <p>To request a new public certificate, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-request-public.html">Request a public certificate</a> in the <i> Amazon Web ServicesCertificate Manager User Guide</i>.</p> <p>To import an existing certificate into ACM, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/import-certificate.html">Importing certificates into ACM</a> in the <i> Amazon Web ServicesCertificate Manager User Guide</i>.</p> <p>To request a private certificate to use FTPS through private IP addresses, see <a href="https://docs.aws.amazon.com/acm/latest/userguide/gs-acm-request-private.html">Request a private certificate</a> in the <i> Amazon Web ServicesCertificate Manager User Guide</i>.</p> <p>Certificates with the following cryptographic algorithms and key sizes are supported:</p> <ul> <li> <p>2048-bit RSA (RSA<em>2048)</p> </li> <li> <p>4096-bit RSA (RSA</em>4096)</p> </li> <li> <p>Elliptic Prime Curve 256 bit (EC<em>prime256v1)</p> </li> <li> <p>Elliptic Prime Curve 384 bit (EC</em>secp384r1)</p> </li> <li> <p>Elliptic Prime Curve 521 bit (EC_secp521r1)</p> </li> </ul> <note> <p>The certificate must be a valid SSL/TLS X.509 version 3 certificate with FQDN or IP address specified and information about the issuer.</p> </note></p>
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The virtual private cloud (VPC) endpoint settings that are configured for your server. When you host your endpoint within your VPC, you can make it accessible only to resources within your VPC, or you can attach Elastic IP addresses and make it accessible to clients over the internet. Your VPC's default security groups are automatically assigned to your endpoint.</p>
    #[serde(rename = "endpointDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_details: Option<EndpointDetails>,
    /// <p><p>The type of endpoint that you want your server to use. You can choose to make your server&#39;s endpoint publicly accessible (PUBLIC) or host it inside your VPC. With an endpoint that is hosted in a VPC, you can restrict access to your server and resources only within your VPC or choose to make it internet facing by attaching Elastic IP addresses directly to it.</p> <note> <p> After May 19, 2021, you won&#39;t be able to create a server using <code>EndpointType=VPC<em>ENDPOINT</code> in your Amazon Web Servicesaccount if your account hasn&#39;t already done so before May 19, 2021. If you have already created servers with <code>EndpointType=VPC</em>ENDPOINT</code> in your Amazon Web Servicesaccount on or before May 19, 2021, you will not be affected. After this date, use <code>EndpointType</code>=<code>VPC</code>.</p> <p>For more information, see https://docs.aws.amazon.com/transfer/latest/userguide/create-server-in-vpc.html#deprecate-vpc-endpoint.</p> <p>It is recommended that you use <code>VPC</code> as the <code>EndpointType</code>. With this endpoint type, you have the option to directly associate up to three Elastic IPv4 addresses (BYO IP included) with your server&#39;s endpoint and use VPC security groups to restrict traffic by the client&#39;s public IP address. This is not possible with <code>EndpointType</code> set to <code>VPC_ENDPOINT</code>.</p> </note></p>
    #[serde(rename = "endpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>The RSA private key as generated by <code>ssh-keygen -N "" -m PEM -f my-new-server-key</code>.</p> <important> <p>If you aren't planning to migrate existing users from an existing server to a new server, don't update the host key. Accidentally changing a server's host key can be disruptive.</p> </important> <p>For more information, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/edit-server-config.html#configuring-servers-change-host-key">Change the host key for your SFTP-enabled server</a> in the <i>Amazon Web ServicesTransfer Family User Guide</i>.</p>
    #[serde(rename = "hostKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key: Option<String>,
    /// <p>An array containing all of the information required to call a customer's authentication API method.</p>
    #[serde(rename = "identityProviderDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_details: Option<IdentityProviderDetails>,
    /// <p>Specifies the Amazon Resource Name (ARN) of the Amazon Web Services Identity and Access Management (IAM) role that allows a server to turn on Amazon CloudWatch logging for Amazon S3 or Amazon EFS events. When set, user activity can be viewed in your CloudWatch logs.</p>
    #[serde(rename = "loggingRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<String>,
    /// <p> The protocol settings that are configured for your server. </p> <p> Use the <code>PassiveIp</code> parameter to indicate passive mode (for FTP and FTPS protocols). Enter a single dotted-quad IPv4 address, such as the external IP address of a firewall, router, or load balancer. </p>
    #[serde(rename = "protocolDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_details: Option<ProtocolDetails>,
    /// <p><p>Specifies the file transfer protocol or protocols over which your file transfer protocol client can connect to your server&#39;s endpoint. The available protocols are:</p> <ul> <li> <p>Secure Shell (SSH) File Transfer Protocol (SFTP): File transfer over SSH</p> </li> <li> <p>File Transfer Protocol Secure (FTPS): File transfer with TLS encryption</p> </li> <li> <p>File Transfer Protocol (FTP): Unencrypted file transfer</p> </li> </ul> <note> <p>If you select <code>FTPS</code>, you must choose a certificate stored in Amazon Web ServicesCertificate Manager (ACM) which will be used to identify your server when clients connect to it over FTPS.</p> <p>If <code>Protocol</code> includes either <code>FTP</code> or <code>FTPS</code>, then the <code>EndpointType</code> must be <code>VPC</code> and the <code>IdentityProviderType</code> must be <code>AWS<em>DIRECTORY</em>SERVICE</code> or <code>API<em>GATEWAY</code>.</p> <p>If <code>Protocol</code> includes <code>FTP</code>, then <code>AddressAllocationIds</code> cannot be associated.</p> <p>If <code>Protocol</code> is set only to <code>SFTP</code>, the <code>EndpointType</code> can be set to <code>PUBLIC</code> and the <code>IdentityProviderType</code> can be set to <code>SERVICE</em>MANAGED</code>.</p> </note></p>
    #[serde(rename = "protocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    /// <p>Specifies the name of the security policy that is attached to the server.</p>
    #[serde(rename = "securityPolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_name: Option<String>,
    /// <p>A system-assigned unique identifier for a server instance that the user account is assigned to.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateServerResponse {
    /// <p>A system-assigned unique identifier for a server that the user account is assigned to.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserRequest {
    /// <p>The landing directory (folder) for a user when they log in to the server using the client.</p> <p>A <code>HomeDirectory</code> example is <code>/bucket_name/home/mydirectory</code>.</p>
    #[serde(rename = "homeDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<String>,
    /// <p><p>Logical directory mappings that specify what Amazon S3 or Amazon EFS paths and keys should be visible to your user and how you want to make them visible. You must specify the <code>Entry</code> and <code>Target</code> pair, where <code>Entry</code> shows how the path is made visible and <code>Target</code> is the actual Amazon S3 or Amazon EFS path. If you only specify a target, it is displayed as is. You also must ensure that your Amazon Web Services Identity and Access Management (IAM) role provides access to paths in <code>Target</code>. This value can only be set when <code>HomeDirectoryType</code> is set to <i>LOGICAL</i>.</p> <p>The following is an <code>Entry</code> and <code>Target</code> pair example.</p> <p> <code>[ { &quot;Entry&quot;: &quot;your-personal-report.pdf&quot;, &quot;Target&quot;: &quot;/bucket3/customized-reports/${transfer:UserName}.pdf&quot; } ]</code> </p> <p>In most cases, you can use this value instead of the scope-down policy to lock down your user to the designated home directory (&quot;<code>chroot</code>&quot;). To do this, you can set <code>Entry</code> to &#39;/&#39; and set <code>Target</code> to the HomeDirectory parameter value.</p> <p>The following is an <code>Entry</code> and <code>Target</code> pair example for <code>chroot</code>.</p> <p> <code>[ { &quot;Entry:&quot;: &quot;/&quot;, &quot;Target&quot;: &quot;/bucket_name/home/mydirectory&quot; } ]</code> </p> <note> <p>If the target of a logical directory entry does not exist in Amazon S3 or EFS, the entry is ignored. As a workaround, you can use the Amazon S3 API or EFS API to create 0 byte objects as place holders for your directory. If using the CLI, use the <code>s3api</code> or <code>efsapi</code> call instead of <code>s3</code> or <code>efs</code> so you can use the put-object operation. For example, you use the following: <code>aws s3api put-object --bucket bucketname --key path/to/folder/</code>. Make sure that the end of the key name ends in a <code>/</code> for it to be considered a folder.</p> </note></p>
    #[serde(rename = "homeDirectoryMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    /// <p>The type of landing directory (folder) you want your users' home directory to be when they log into the server. If you set it to <code>PATH</code>, the user will see the absolute Amazon S3 bucket or EFS paths as is in their file transfer protocol clients. If you set it <code>LOGICAL</code>, you will need to provide mappings in the <code>HomeDirectoryMappings</code> for how you want to make Amazon S3 or EFS paths visible to your users.</p>
    #[serde(rename = "homeDirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<String>,
    /// <p><p>A scope-down policy for your user so that you can use the same IAM role across multiple users. This policy scopes down user access to portions of their Amazon S3 bucket. Variables that you can use inside this policy include <code>${Transfer:UserName}</code>, <code>${Transfer:HomeDirectory}</code>, and <code>${Transfer:HomeBucket}</code>.</p> <note> <p>This only applies when domain of <code>ServerId</code> is S3. Amazon EFS does not use scope-down policies.</p> <p>For scope-down policies, Amazon Web ServicesTransfer Family stores the policy as a JSON blob, instead of the Amazon Resource Name (ARN) of the policy. You save the policy as a JSON blob and pass it in the <code>Policy</code> argument.</p> <p>For an example of a scope-down policy, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/users.html#users-policies-scope-down">Creating a scope-down policy</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html">AssumeRole</a> in the <i>Amazon Web Services Security Token Service API Reference</i>.</p> </note></p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>Specifies the full POSIX identity, including user ID (<code>Uid</code>), group ID (<code>Gid</code>), and any secondary groups IDs (<code>SecondaryGids</code>), that controls your users' access to your Amazon Elastic File Systems (Amazon EFS). The POSIX permissions that are set on files and directories in your file system determines the level of access your users get when transferring files into and out of your Amazon EFS file systems.</p>
    #[serde(rename = "posixProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,
    /// <p>Specifies the Amazon Resource Name (ARN) of the IAM role that controls your users' access to your Amazon S3 bucket or EFS file system. The policies attached to this role determine the level of access that you want to provide your users when transferring files into and out of your Amazon S3 bucket or EFS file system. The IAM role should also contain a trust relationship that allows the server to access your resources when servicing your users' transfer requests.</p>
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>A system-assigned unique identifier for a server instance that the user account is assigned to.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p>A unique string that identifies a user and is associated with a server as specified by the <code>ServerId</code>. This user name must be a minimum of 3 and a maximum of 100 characters long. The following are valid characters: a-z, A-Z, 0-9, underscore '_', hyphen '-', period '.', and at sign '@'. The user name can't start with a hyphen, period, or at sign.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

/// <p> <code>UpdateUserResponse</code> returns the user name and identifier for the request to update a user's properties.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserResponse {
    /// <p>A system-assigned unique identifier for a server instance that the user account is assigned to.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
    /// <p>The unique identifier for a user that is assigned to a server instance that was specified in the request.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

/// Errors returned by CreateAccess
#[derive(Debug, PartialEq)]
pub enum CreateAccessError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>The requested resource does not exist.</p>
    ResourceExists(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl CreateAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(CreateAccessError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateAccessError::InvalidRequest(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateAccessError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateAccessError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateAccessError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAccessError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateAccessError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateAccessError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateAccessError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateAccessError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAccessError {}
/// Errors returned by CreateServer
#[derive(Debug, PartialEq)]
pub enum CreateServerError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>The requested resource does not exist.</p>
    ResourceExists(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p> <p> HTTP Status Code: 400</p>
    Throttling(String),
}

impl CreateServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateServerError::AccessDenied(err.msg))
                }
                "InternalServiceError" => {
                    return RusotoError::Service(CreateServerError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateServerError::InvalidRequest(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateServerError::ResourceExists(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateServerError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateServerError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateServerError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateServerError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateServerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateServerError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateServerError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateServerError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateServerError {}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>The requested resource does not exist.</p>
    ResourceExists(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(CreateUserError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateUserError::InvalidRequest(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(CreateUserError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateUserError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateUserError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreateUserError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateUserError::ResourceExists(ref cause) => write!(f, "{}", cause),
            CreateUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserError {}
/// Errors returned by DeleteAccess
#[derive(Debug, PartialEq)]
pub enum DeleteAccessError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl DeleteAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DeleteAccessError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteAccessError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteAccessError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteAccessError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAccessError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeleteAccessError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteAccessError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAccessError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAccessError {}
/// Errors returned by DeleteServer
#[derive(Debug, PartialEq)]
pub enum DeleteServerError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl DeleteServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteServerError::AccessDenied(err.msg))
                }
                "InternalServiceError" => {
                    return RusotoError::Service(DeleteServerError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteServerError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteServerError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteServerError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteServerError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteServerError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeleteServerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteServerError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteServerError {}
/// Errors returned by DeleteSshPublicKey
#[derive(Debug, PartialEq)]
pub enum DeleteSshPublicKeyError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p> <p> HTTP Status Code: 400</p>
    Throttling(String),
}

impl DeleteSshPublicKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSshPublicKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DeleteSshPublicKeyError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteSshPublicKeyError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteSshPublicKeyError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteSshPublicKeyError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteSshPublicKeyError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSshPublicKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSshPublicKeyError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeleteSshPublicKeyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteSshPublicKeyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteSshPublicKeyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteSshPublicKeyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSshPublicKeyError {}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DeleteUserError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteUserError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteUserError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserError {}
/// Errors returned by DescribeAccess
#[derive(Debug, PartialEq)]
pub enum DescribeAccessError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl DescribeAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DescribeAccessError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeAccessError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeAccessError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeAccessError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAccessError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DescribeAccessError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeAccessError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAccessError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAccessError {}
/// Errors returned by DescribeSecurityPolicy
#[derive(Debug, PartialEq)]
pub enum DescribeSecurityPolicyError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl DescribeSecurityPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSecurityPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DescribeSecurityPolicyError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeSecurityPolicyError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeSecurityPolicyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeSecurityPolicyError::ServiceUnavailable(
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
impl fmt::Display for DescribeSecurityPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSecurityPolicyError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DescribeSecurityPolicyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeSecurityPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeSecurityPolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSecurityPolicyError {}
/// Errors returned by DescribeServer
#[derive(Debug, PartialEq)]
pub enum DescribeServerError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl DescribeServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DescribeServerError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeServerError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeServerError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeServerError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServerError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DescribeServerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeServerError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeServerError {}
/// Errors returned by DescribeUser
#[derive(Debug, PartialEq)]
pub enum DescribeUserError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl DescribeUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DescribeUserError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeUserError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeUserError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DescribeUserError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserError {}
/// Errors returned by ImportSshPublicKey
#[derive(Debug, PartialEq)]
pub enum ImportSshPublicKeyError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>The requested resource does not exist.</p>
    ResourceExists(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p> <p> HTTP Status Code: 400</p>
    Throttling(String),
}

impl ImportSshPublicKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportSshPublicKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(ImportSshPublicKeyError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ImportSshPublicKeyError::InvalidRequest(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(ImportSshPublicKeyError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ImportSshPublicKeyError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ImportSshPublicKeyError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ImportSshPublicKeyError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ImportSshPublicKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportSshPublicKeyError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ImportSshPublicKeyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ImportSshPublicKeyError::ResourceExists(ref cause) => write!(f, "{}", cause),
            ImportSshPublicKeyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ImportSshPublicKeyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ImportSshPublicKeyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportSshPublicKeyError {}
/// Errors returned by ListAccesses
#[derive(Debug, PartialEq)]
pub enum ListAccessesError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>The <code>NextToken</code> parameter that was passed is invalid.</p>
    InvalidNextToken(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl ListAccessesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAccessesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(ListAccessesError::InternalServiceError(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListAccessesError::InvalidNextToken(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListAccessesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListAccessesError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListAccessesError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAccessesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAccessesError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListAccessesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListAccessesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListAccessesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListAccessesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAccessesError {}
/// Errors returned by ListSecurityPolicies
#[derive(Debug, PartialEq)]
pub enum ListSecurityPoliciesError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>The <code>NextToken</code> parameter that was passed is invalid.</p>
    InvalidNextToken(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl ListSecurityPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSecurityPoliciesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(ListSecurityPoliciesError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListSecurityPoliciesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListSecurityPoliciesError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListSecurityPoliciesError::ServiceUnavailable(
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
impl fmt::Display for ListSecurityPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSecurityPoliciesError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListSecurityPoliciesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListSecurityPoliciesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListSecurityPoliciesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSecurityPoliciesError {}
/// Errors returned by ListServers
#[derive(Debug, PartialEq)]
pub enum ListServersError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>The <code>NextToken</code> parameter that was passed is invalid.</p>
    InvalidNextToken(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl ListServersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListServersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(ListServersError::InternalServiceError(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListServersError::InvalidNextToken(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListServersError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListServersError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListServersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListServersError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListServersError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListServersError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListServersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListServersError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>The <code>NextToken</code> parameter that was passed is invalid.</p>
    InvalidNextToken(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListTagsForResourceError::ServiceUnavailable(
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
            ListTagsForResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>The <code>NextToken</code> parameter that was passed is invalid.</p>
    InvalidNextToken(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl ListUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUsersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(ListUsersError::InternalServiceError(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListUsersError::InvalidNextToken(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListUsersError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUsersError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListUsersError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUsersError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListUsersError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListUsersError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListUsersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUsersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUsersError {}
/// Errors returned by StartServer
#[derive(Debug, PartialEq)]
pub enum StartServerError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p> <p> HTTP Status Code: 400</p>
    Throttling(String),
}

impl StartServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(StartServerError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartServerError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartServerError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StartServerError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartServerError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartServerError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            StartServerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartServerError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            StartServerError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartServerError {}
/// Errors returned by StopServer
#[derive(Debug, PartialEq)]
pub enum StopServerError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p> <p> HTTP Status Code: 400</p>
    Throttling(String),
}

impl StopServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(StopServerError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopServerError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopServerError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StopServerError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StopServerError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopServerError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            StopServerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StopServerError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            StopServerError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopServerError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(TagResourceError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(TagResourceError::ServiceUnavailable(err.msg))
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
            TagResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by TestIdentityProvider
#[derive(Debug, PartialEq)]
pub enum TestIdentityProviderError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl TestIdentityProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestIdentityProviderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(TestIdentityProviderError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TestIdentityProviderError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TestIdentityProviderError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(TestIdentityProviderError::ServiceUnavailable(
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
impl fmt::Display for TestIdentityProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestIdentityProviderError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            TestIdentityProviderError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TestIdentityProviderError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TestIdentityProviderError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TestIdentityProviderError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(UntagResourceError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UntagResourceError::ServiceUnavailable(err.msg))
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
            UntagResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateAccess
#[derive(Debug, PartialEq)]
pub enum UpdateAccessError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>The requested resource does not exist.</p>
    ResourceExists(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
}

impl UpdateAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(UpdateAccessError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateAccessError::InvalidRequest(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(UpdateAccessError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAccessError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateAccessError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAccessError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UpdateAccessError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateAccessError::ResourceExists(ref cause) => write!(f, "{}", cause),
            UpdateAccessError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAccessError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAccessError {}
/// Errors returned by UpdateServer
#[derive(Debug, PartialEq)]
pub enum UpdateServerError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>This exception is thrown when the <code>UpdatServer</code> is called for a file transfer protocol-enabled server that has VPC as the endpoint type and the server's <code>VpcEndpointID</code> is not in the available state.</p>
    Conflict(String),
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>The requested resource does not exist.</p>
    ResourceExists(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p> <p> HTTP Status Code: 400</p>
    Throttling(String),
}

impl UpdateServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateServerError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateServerError::Conflict(err.msg))
                }
                "InternalServiceError" => {
                    return RusotoError::Service(UpdateServerError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateServerError::InvalidRequest(err.msg))
                }
                "ResourceExistsException" => {
                    return RusotoError::Service(UpdateServerError::ResourceExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateServerError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateServerError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateServerError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateServerError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateServerError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateServerError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UpdateServerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateServerError::ResourceExists(ref cause) => write!(f, "{}", cause),
            UpdateServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateServerError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateServerError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateServerError {}
/// Errors returned by UpdateUser
#[derive(Debug, PartialEq)]
pub enum UpdateUserError {
    /// <p>This exception is thrown when an error occurs in the Amazon Web ServicesTransfer Family service.</p>
    InternalServiceError(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
    /// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
    ResourceNotFound(String),
    /// <p>The request has failed because the Amazon Web ServicesTransfer Family service is not available.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p> <p> HTTP Status Code: 400</p>
    Throttling(String),
}

impl UpdateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(UpdateUserError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateUserError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UpdateUserError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserError {}
/// Trait representing the capabilities of the AWS Transfer API. AWS Transfer clients implement this trait.
#[async_trait]
pub trait Transfer {
    /// <p>Used by administrators to choose which groups in the directory should have access to upload and download files over the enabled protocols using Amazon Web Services Transfer Family. For example, a Microsoft Active Directory might contain 50,000 users, but only a small fraction might need the ability to transfer files to the server. An administrator can use <code>CreateAccess</code> to limit the access to the correct set of users who need this ability.</p>
    async fn create_access(
        &self,
        input: CreateAccessRequest,
    ) -> Result<CreateAccessResponse, RusotoError<CreateAccessError>>;

    /// <p>Instantiates an auto-scaling virtual server based on the selected file transfer protocol in Amazon Web Services. When you make updates to your file transfer protocol-enabled server or when you work with users, use the service-generated <code>ServerId</code> property that is assigned to the newly created server.</p>
    async fn create_server(
        &self,
        input: CreateServerRequest,
    ) -> Result<CreateServerResponse, RusotoError<CreateServerError>>;

    /// <p>Creates a user and associates them with an existing file transfer protocol-enabled server. You can only create and associate users with servers that have the <code>IdentityProviderType</code> set to <code>SERVICE_MANAGED</code>. Using parameters for <code>CreateUser</code>, you can specify the user name, set the home directory, store the user's public key, and assign the user's Amazon Web Services Identity and Access Management (IAM) role. You can also optionally add a scope-down policy, and assign metadata with tags that can be used to group and search for users.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>>;

    /// <p>Allows you to delete the access specified in the <code>ServerID</code> and <code>ExternalID</code> parameters.</p>
    async fn delete_access(
        &self,
        input: DeleteAccessRequest,
    ) -> Result<(), RusotoError<DeleteAccessError>>;

    /// <p>Deletes the file transfer protocol-enabled server that you specify.</p> <p>No response returns from this operation.</p>
    async fn delete_server(
        &self,
        input: DeleteServerRequest,
    ) -> Result<(), RusotoError<DeleteServerError>>;

    /// <p>Deletes a user's Secure Shell (SSH) public key.</p> <p>No response is returned from this operation.</p>
    async fn delete_ssh_public_key(
        &self,
        input: DeleteSshPublicKeyRequest,
    ) -> Result<(), RusotoError<DeleteSshPublicKeyError>>;

    /// <p><p>Deletes the user belonging to a file transfer protocol-enabled server you specify.</p> <p>No response returns from this operation.</p> <note> <p>When you delete a user from a server, the user&#39;s information is lost.</p> </note></p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<(), RusotoError<DeleteUserError>>;

    /// <p>Describes the access that is assigned to the specific file transfer protocol-enabled server, as identified by its <code>ServerId</code> property and its <code>ExternalID</code>.</p> <p>The response from this call returns the properties of the access that is associated with the <code>ServerId</code> value that was specified.</p>
    async fn describe_access(
        &self,
        input: DescribeAccessRequest,
    ) -> Result<DescribeAccessResponse, RusotoError<DescribeAccessError>>;

    /// <p>Describes the security policy that is attached to your file transfer protocol-enabled server. The response contains a description of the security policy's properties. For more information about security policies, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/security-policies.html">Working with security policies</a>.</p>
    async fn describe_security_policy(
        &self,
        input: DescribeSecurityPolicyRequest,
    ) -> Result<DescribeSecurityPolicyResponse, RusotoError<DescribeSecurityPolicyError>>;

    /// <p>Describes a file transfer protocol-enabled server that you specify by passing the <code>ServerId</code> parameter.</p> <p>The response contains a description of a server's properties. When you set <code>EndpointType</code> to VPC, the response will contain the <code>EndpointDetails</code>.</p>
    async fn describe_server(
        &self,
        input: DescribeServerRequest,
    ) -> Result<DescribeServerResponse, RusotoError<DescribeServerError>>;

    /// <p>Describes the user assigned to the specific file transfer protocol-enabled server, as identified by its <code>ServerId</code> property.</p> <p>The response from this call returns the properties of the user associated with the <code>ServerId</code> value that was specified.</p>
    async fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> Result<DescribeUserResponse, RusotoError<DescribeUserError>>;

    /// <p>Adds a Secure Shell (SSH) public key to a user account identified by a <code>UserName</code> value assigned to the specific file transfer protocol-enabled server, identified by <code>ServerId</code>.</p> <p>The response returns the <code>UserName</code> value, the <code>ServerId</code> value, and the name of the <code>SshPublicKeyId</code>.</p>
    async fn import_ssh_public_key(
        &self,
        input: ImportSshPublicKeyRequest,
    ) -> Result<ImportSshPublicKeyResponse, RusotoError<ImportSshPublicKeyError>>;

    /// <p>Lists the details for all the accesses you have on your server.</p>
    async fn list_accesses(
        &self,
        input: ListAccessesRequest,
    ) -> Result<ListAccessesResponse, RusotoError<ListAccessesError>>;

    /// <p>Lists the security policies that are attached to your file transfer protocol-enabled servers.</p>
    async fn list_security_policies(
        &self,
        input: ListSecurityPoliciesRequest,
    ) -> Result<ListSecurityPoliciesResponse, RusotoError<ListSecurityPoliciesError>>;

    /// <p>Lists the file transfer protocol-enabled servers that are associated with your Amazon Web Services account.</p>
    async fn list_servers(
        &self,
        input: ListServersRequest,
    ) -> Result<ListServersResponse, RusotoError<ListServersError>>;

    /// <p>Lists all of the tags associated with the Amazon Resource Name (ARN) that you specify. The resource can be a user, server, or role.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists the users for a file transfer protocol-enabled server that you specify by passing the <code>ServerId</code> parameter.</p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>>;

    /// <p>Changes the state of a file transfer protocol-enabled server from <code>OFFLINE</code> to <code>ONLINE</code>. It has no impact on a server that is already <code>ONLINE</code>. An <code>ONLINE</code> server can accept and process file transfer jobs.</p> <p>The state of <code>STARTING</code> indicates that the server is in an intermediate state, either not fully able to respond, or not fully online. The values of <code>START_FAILED</code> can indicate an error condition.</p> <p>No response is returned from this call.</p>
    async fn start_server(
        &self,
        input: StartServerRequest,
    ) -> Result<(), RusotoError<StartServerError>>;

    /// <p>Changes the state of a file transfer protocol-enabled server from <code>ONLINE</code> to <code>OFFLINE</code>. An <code>OFFLINE</code> server cannot accept and process file transfer jobs. Information tied to your server, such as server and user properties, are not affected by stopping your server.</p> <note> <p>Stopping the server will not reduce or impact your file transfer protocol endpoint billing; you must delete the server to stop being billed.</p> </note> <p>The state of <code>STOPPING</code> indicates that the server is in an intermediate state, either not fully able to respond, or not fully offline. The values of <code>STOP_FAILED</code> can indicate an error condition.</p> <p>No response is returned from this call.</p>
    async fn stop_server(
        &self,
        input: StopServerRequest,
    ) -> Result<(), RusotoError<StopServerError>>;

    /// <p>Attaches a key-value pair to a resource, as identified by its Amazon Resource Name (ARN). Resources are users, servers, roles, and other entities.</p> <p>There is no response returned from this call.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>If the <code>IdentityProviderType</code> of a file transfer protocol-enabled server is <code>AWS_DIRECTORY_SERVICE</code> or <code>API_Gateway</code>, tests whether your identity provider is set up successfully. We highly recommend that you call this operation to test your authentication method as soon as you create your server. By doing so, you can troubleshoot issues with the identity provider integration to ensure that your users can successfully use the service.</p>
    async fn test_identity_provider(
        &self,
        input: TestIdentityProviderRequest,
    ) -> Result<TestIdentityProviderResponse, RusotoError<TestIdentityProviderError>>;

    /// <p>Detaches a key-value pair from a resource, as identified by its Amazon Resource Name (ARN). Resources are users, servers, roles, and other entities.</p> <p>No response is returned from this call.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Allows you to update parameters for the access specified in the <code>ServerID</code> and <code>ExternalID</code> parameters.</p>
    async fn update_access(
        &self,
        input: UpdateAccessRequest,
    ) -> Result<UpdateAccessResponse, RusotoError<UpdateAccessError>>;

    /// <p>Updates the file transfer protocol-enabled server's properties after that server has been created.</p> <p>The <code>UpdateServer</code> call returns the <code>ServerId</code> of the server you updated.</p>
    async fn update_server(
        &self,
        input: UpdateServerRequest,
    ) -> Result<UpdateServerResponse, RusotoError<UpdateServerError>>;

    /// <p>Assigns new properties to a user. Parameters you pass modify any or all of the following: the home directory, role, and policy for the <code>UserName</code> and <code>ServerId</code> you specify.</p> <p>The response returns the <code>ServerId</code> and the <code>UserName</code> for the updated user.</p>
    async fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, RusotoError<UpdateUserError>>;
}
/// A client for the AWS Transfer API.
#[derive(Clone)]
pub struct TransferClient {
    client: Client,
    region: region::Region,
}

impl TransferClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> TransferClient {
        TransferClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> TransferClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        TransferClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> TransferClient {
        TransferClient { client, region }
    }
}

#[async_trait]
impl Transfer for TransferClient {
    /// <p>Used by administrators to choose which groups in the directory should have access to upload and download files over the enabled protocols using Amazon Web Services Transfer Family. For example, a Microsoft Active Directory might contain 50,000 users, but only a small fraction might need the ability to transfer files to the server. An administrator can use <code>CreateAccess</code> to limit the access to the correct set of users who need this ability.</p>
    async fn create_access(
        &self,
        input: CreateAccessRequest,
    ) -> Result<CreateAccessResponse, RusotoError<CreateAccessError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.CreateAccess");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateAccessError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateAccessResponse, _>()
    }

    /// <p>Instantiates an auto-scaling virtual server based on the selected file transfer protocol in Amazon Web Services. When you make updates to your file transfer protocol-enabled server or when you work with users, use the service-generated <code>ServerId</code> property that is assigned to the newly created server.</p>
    async fn create_server(
        &self,
        input: CreateServerRequest,
    ) -> Result<CreateServerResponse, RusotoError<CreateServerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.CreateServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateServerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateServerResponse, _>()
    }

    /// <p>Creates a user and associates them with an existing file transfer protocol-enabled server. You can only create and associate users with servers that have the <code>IdentityProviderType</code> set to <code>SERVICE_MANAGED</code>. Using parameters for <code>CreateUser</code>, you can specify the user name, set the home directory, store the user's public key, and assign the user's Amazon Web Services Identity and Access Management (IAM) role. You can also optionally add a scope-down policy, and assign metadata with tags that can be used to group and search for users.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.CreateUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateUserError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateUserResponse, _>()
    }

    /// <p>Allows you to delete the access specified in the <code>ServerID</code> and <code>ExternalID</code> parameters.</p>
    async fn delete_access(
        &self,
        input: DeleteAccessRequest,
    ) -> Result<(), RusotoError<DeleteAccessError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.DeleteAccess");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAccessError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes the file transfer protocol-enabled server that you specify.</p> <p>No response returns from this operation.</p>
    async fn delete_server(
        &self,
        input: DeleteServerRequest,
    ) -> Result<(), RusotoError<DeleteServerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.DeleteServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteServerError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a user's Secure Shell (SSH) public key.</p> <p>No response is returned from this operation.</p>
    async fn delete_ssh_public_key(
        &self,
        input: DeleteSshPublicKeyRequest,
    ) -> Result<(), RusotoError<DeleteSshPublicKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.DeleteSshPublicKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteSshPublicKeyError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes the user belonging to a file transfer protocol-enabled server you specify.</p> <p>No response returns from this operation.</p> <note> <p>When you delete a user from a server, the user&#39;s information is lost.</p> </note></p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<(), RusotoError<DeleteUserError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.DeleteUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteUserError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Describes the access that is assigned to the specific file transfer protocol-enabled server, as identified by its <code>ServerId</code> property and its <code>ExternalID</code>.</p> <p>The response from this call returns the properties of the access that is associated with the <code>ServerId</code> value that was specified.</p>
    async fn describe_access(
        &self,
        input: DescribeAccessRequest,
    ) -> Result<DescribeAccessResponse, RusotoError<DescribeAccessError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.DescribeAccess");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeAccessError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeAccessResponse, _>()
    }

    /// <p>Describes the security policy that is attached to your file transfer protocol-enabled server. The response contains a description of the security policy's properties. For more information about security policies, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/security-policies.html">Working with security policies</a>.</p>
    async fn describe_security_policy(
        &self,
        input: DescribeSecurityPolicyRequest,
    ) -> Result<DescribeSecurityPolicyResponse, RusotoError<DescribeSecurityPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.DescribeSecurityPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeSecurityPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeSecurityPolicyResponse, _>()
    }

    /// <p>Describes a file transfer protocol-enabled server that you specify by passing the <code>ServerId</code> parameter.</p> <p>The response contains a description of a server's properties. When you set <code>EndpointType</code> to VPC, the response will contain the <code>EndpointDetails</code>.</p>
    async fn describe_server(
        &self,
        input: DescribeServerRequest,
    ) -> Result<DescribeServerResponse, RusotoError<DescribeServerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.DescribeServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeServerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeServerResponse, _>()
    }

    /// <p>Describes the user assigned to the specific file transfer protocol-enabled server, as identified by its <code>ServerId</code> property.</p> <p>The response from this call returns the properties of the user associated with the <code>ServerId</code> value that was specified.</p>
    async fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> Result<DescribeUserResponse, RusotoError<DescribeUserError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.DescribeUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeUserError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeUserResponse, _>()
    }

    /// <p>Adds a Secure Shell (SSH) public key to a user account identified by a <code>UserName</code> value assigned to the specific file transfer protocol-enabled server, identified by <code>ServerId</code>.</p> <p>The response returns the <code>UserName</code> value, the <code>ServerId</code> value, and the name of the <code>SshPublicKeyId</code>.</p>
    async fn import_ssh_public_key(
        &self,
        input: ImportSshPublicKeyRequest,
    ) -> Result<ImportSshPublicKeyResponse, RusotoError<ImportSshPublicKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.ImportSshPublicKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ImportSshPublicKeyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ImportSshPublicKeyResponse, _>()
    }

    /// <p>Lists the details for all the accesses you have on your server.</p>
    async fn list_accesses(
        &self,
        input: ListAccessesRequest,
    ) -> Result<ListAccessesResponse, RusotoError<ListAccessesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.ListAccesses");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAccessesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAccessesResponse, _>()
    }

    /// <p>Lists the security policies that are attached to your file transfer protocol-enabled servers.</p>
    async fn list_security_policies(
        &self,
        input: ListSecurityPoliciesRequest,
    ) -> Result<ListSecurityPoliciesResponse, RusotoError<ListSecurityPoliciesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.ListSecurityPolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListSecurityPoliciesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListSecurityPoliciesResponse, _>()
    }

    /// <p>Lists the file transfer protocol-enabled servers that are associated with your Amazon Web Services account.</p>
    async fn list_servers(
        &self,
        input: ListServersRequest,
    ) -> Result<ListServersResponse, RusotoError<ListServersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.ListServers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListServersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListServersResponse, _>()
    }

    /// <p>Lists all of the tags associated with the Amazon Resource Name (ARN) that you specify. The resource can be a user, server, or role.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Lists the users for a file transfer protocol-enabled server that you specify by passing the <code>ServerId</code> parameter.</p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.ListUsers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListUsersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListUsersResponse, _>()
    }

    /// <p>Changes the state of a file transfer protocol-enabled server from <code>OFFLINE</code> to <code>ONLINE</code>. It has no impact on a server that is already <code>ONLINE</code>. An <code>ONLINE</code> server can accept and process file transfer jobs.</p> <p>The state of <code>STARTING</code> indicates that the server is in an intermediate state, either not fully able to respond, or not fully online. The values of <code>START_FAILED</code> can indicate an error condition.</p> <p>No response is returned from this call.</p>
    async fn start_server(
        &self,
        input: StartServerRequest,
    ) -> Result<(), RusotoError<StartServerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.StartServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartServerError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Changes the state of a file transfer protocol-enabled server from <code>ONLINE</code> to <code>OFFLINE</code>. An <code>OFFLINE</code> server cannot accept and process file transfer jobs. Information tied to your server, such as server and user properties, are not affected by stopping your server.</p> <note> <p>Stopping the server will not reduce or impact your file transfer protocol endpoint billing; you must delete the server to stop being billed.</p> </note> <p>The state of <code>STOPPING</code> indicates that the server is in an intermediate state, either not fully able to respond, or not fully offline. The values of <code>STOP_FAILED</code> can indicate an error condition.</p> <p>No response is returned from this call.</p>
    async fn stop_server(
        &self,
        input: StopServerRequest,
    ) -> Result<(), RusotoError<StopServerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.StopServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopServerError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Attaches a key-value pair to a resource, as identified by its Amazon Resource Name (ARN). Resources are users, servers, roles, and other entities.</p> <p>There is no response returned from this call.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>If the <code>IdentityProviderType</code> of a file transfer protocol-enabled server is <code>AWS_DIRECTORY_SERVICE</code> or <code>API_Gateway</code>, tests whether your identity provider is set up successfully. We highly recommend that you call this operation to test your authentication method as soon as you create your server. By doing so, you can troubleshoot issues with the identity provider integration to ensure that your users can successfully use the service.</p>
    async fn test_identity_provider(
        &self,
        input: TestIdentityProviderRequest,
    ) -> Result<TestIdentityProviderResponse, RusotoError<TestIdentityProviderError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.TestIdentityProvider");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TestIdentityProviderError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<TestIdentityProviderResponse, _>()
    }

    /// <p>Detaches a key-value pair from a resource, as identified by its Amazon Resource Name (ARN). Resources are users, servers, roles, and other entities.</p> <p>No response is returned from this call.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Allows you to update parameters for the access specified in the <code>ServerID</code> and <code>ExternalID</code> parameters.</p>
    async fn update_access(
        &self,
        input: UpdateAccessRequest,
    ) -> Result<UpdateAccessResponse, RusotoError<UpdateAccessError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.UpdateAccess");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateAccessError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateAccessResponse, _>()
    }

    /// <p>Updates the file transfer protocol-enabled server's properties after that server has been created.</p> <p>The <code>UpdateServer</code> call returns the <code>ServerId</code> of the server you updated.</p>
    async fn update_server(
        &self,
        input: UpdateServerRequest,
    ) -> Result<UpdateServerResponse, RusotoError<UpdateServerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.UpdateServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateServerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateServerResponse, _>()
    }

    /// <p>Assigns new properties to a user. Parameters you pass modify any or all of the following: the home directory, role, and policy for the <code>UserName</code> and <code>ServerId</code> you specify.</p> <p>The response returns the <code>ServerId</code> and the <code>UserName</code> for the updated user.</p>
    async fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, RusotoError<UpdateUserError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "TransferService.UpdateUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateUserError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateUserResponse, _>()
    }
}
