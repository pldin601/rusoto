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

impl CloudHsmv2Client {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "cloudhsm", &self.region, request_uri);
        request.set_endpoint_prefix("cloudhsmv2".to_string());

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
/// <p>Contains information about a backup of an AWS CloudHSM cluster. All backup objects contain the <code>BackupId</code>, <code>BackupState</code>, <code>ClusterId</code>, and <code>CreateTimestamp</code> parameters. Backups that were copied into a destination region additionally contain the <code>CopyTimestamp</code>, <code>SourceBackup</code>, <code>SourceCluster</code>, and <code>SourceRegion</code> parameters. A backup that is pending deletion will include the <code>DeleteTimestamp</code> parameter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Backup {
    /// <p>The identifier (ID) of the backup.</p>
    #[serde(rename = "backupId")]
    pub backup_id: String,
    /// <p>The state of the backup.</p>
    #[serde(rename = "backupState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_state: Option<String>,
    /// <p>The identifier (ID) of the cluster that was backed up.</p>
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The date and time when the backup was copied from a source backup.</p>
    #[serde(rename = "copyTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_timestamp: Option<f64>,
    /// <p>The date and time when the backup was created.</p>
    #[serde(rename = "createTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    /// <p>The date and time when the backup will be permanently deleted.</p>
    #[serde(rename = "deleteTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_timestamp: Option<f64>,
    /// <p>Specifies whether the service should exempt a backup from the retention policy for the cluster. <code>True</code> exempts a backup from the retention policy. <code>False</code> means the service applies the backup retention policy defined at the cluster.</p>
    #[serde(rename = "neverExpires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_expires: Option<bool>,
    /// <p>The identifier (ID) of the source backup from which the new backup was copied.</p>
    #[serde(rename = "sourceBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup: Option<String>,
    /// <p>The identifier (ID) of the cluster containing the source backup from which the new backup was copied.</p>
    #[serde(rename = "sourceCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cluster: Option<String>,
    /// <p>The AWS Region that contains the source backup from which the new backup was copied.</p>
    #[serde(rename = "sourceRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
    /// <p>The list of tags for the backup.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

/// <p>A policy that defines the number of days to retain backups.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BackupRetentionPolicy {
    /// <p>The type of backup retention policy. For the <code>DAYS</code> type, the value is the number of days to retain backups.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>Use a value between 7 - 379.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Contains one or more certificates or a certificate signing request (CSR).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Certificates {
    /// <p>The HSM hardware certificate issued (signed) by AWS CloudHSM.</p>
    #[serde(rename = "awsHardwareCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_hardware_certificate: Option<String>,
    /// <p>The cluster certificate issued (signed) by the issuing certificate authority (CA) of the cluster's owner.</p>
    #[serde(rename = "clusterCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_certificate: Option<String>,
    /// <p>The cluster's certificate signing request (CSR). The CSR exists only when the cluster's state is <code>UNINITIALIZED</code>.</p>
    #[serde(rename = "clusterCsr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_csr: Option<String>,
    /// <p>The HSM certificate issued (signed) by the HSM hardware.</p>
    #[serde(rename = "hsmCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_certificate: Option<String>,
    /// <p>The HSM hardware certificate issued (signed) by the hardware manufacturer.</p>
    #[serde(rename = "manufacturerHardwareCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_hardware_certificate: Option<String>,
}

/// <p>Contains information about an AWS CloudHSM cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Cluster {
    /// <p>The cluster's backup policy.</p>
    #[serde(rename = "backupPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_policy: Option<String>,
    /// <p>A policy that defines how the service retains backups.</p>
    #[serde(rename = "backupRetentionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_policy: Option<BackupRetentionPolicy>,
    /// <p>Contains one or more certificates or a certificate signing request (CSR).</p>
    #[serde(rename = "certificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Certificates>,
    /// <p>The cluster's identifier (ID).</p>
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The date and time when the cluster was created.</p>
    #[serde(rename = "createTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    /// <p>The type of HSM that the cluster contains.</p>
    #[serde(rename = "hsmType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_type: Option<String>,
    /// <p>Contains information about the HSMs in the cluster.</p>
    #[serde(rename = "hsms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsms: Option<Vec<Hsm>>,
    /// <p>The default password for the cluster's Pre-Crypto Officer (PRECO) user.</p>
    #[serde(rename = "preCoPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_co_password: Option<String>,
    /// <p>The identifier (ID) of the cluster's security group.</p>
    #[serde(rename = "securityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<String>,
    /// <p>The identifier (ID) of the backup used to create the cluster. This value exists only when the cluster was created from a backup.</p>
    #[serde(rename = "sourceBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    /// <p>The cluster's state.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of the cluster's state.</p>
    #[serde(rename = "stateMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
    /// <p>A map from availability zone to the cluster’s subnet in that availability zone.</p>
    #[serde(rename = "subnetMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mapping: Option<::std::collections::HashMap<String, String>>,
    /// <p>The list of tags for the cluster.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
    /// <p>The identifier (ID) of the virtual private cloud (VPC) that contains the cluster.</p>
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CopyBackupToRegionRequest {
    /// <p>The ID of the backup that will be copied to the destination region. </p>
    #[serde(rename = "backupId")]
    pub backup_id: String,
    /// <p>The AWS region that will contain your copied CloudHSM cluster backup.</p>
    #[serde(rename = "destinationRegion")]
    pub destination_region: String,
    /// <p>Tags to apply to the destination backup during creation. If you specify tags, only these tags will be applied to the destination backup. If you do not specify tags, the service copies tags from the source backup to the destination backup.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CopyBackupToRegionResponse {
    /// <p>Information on the backup that will be copied to the destination region, including CreateTimestamp, SourceBackup, SourceCluster, and Source Region. CreateTimestamp of the destination backup will be the same as that of the source backup.</p> <p>You will need to use the <code>sourceBackupID</code> returned in this operation to use the <a>DescribeBackups</a> operation on the backup that will be copied to the destination region.</p>
    #[serde(rename = "destinationBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_backup: Option<DestinationBackup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateClusterRequest {
    /// <p>A policy that defines how the service retains backups.</p>
    #[serde(rename = "backupRetentionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_policy: Option<BackupRetentionPolicy>,
    /// <p>The type of HSM to use in the cluster. Currently the only allowed value is <code>hsm1.medium</code>.</p>
    #[serde(rename = "hsmType")]
    pub hsm_type: String,
    /// <p>The identifier (ID) of the cluster backup to restore. Use this value to restore the cluster from a backup instead of creating a new cluster. To find the backup ID, use <a>DescribeBackups</a>.</p>
    #[serde(rename = "sourceBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    /// <p><p>The identifiers (IDs) of the subnets where you are creating the cluster. You must specify at least one subnet. If you specify multiple subnets, they must meet the following criteria:</p> <ul> <li> <p>All subnets must be in the same virtual private cloud (VPC).</p> </li> <li> <p>You can specify only one subnet per Availability Zone.</p> </li> </ul></p>
    #[serde(rename = "subnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>Tags to apply to the CloudHSM cluster during creation.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateClusterResponse {
    /// <p>Information about the cluster that was created.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateHsmRequest {
    /// <p>The Availability Zone where you are creating the HSM. To find the cluster's Availability Zones, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The identifier (ID) of the HSM's cluster. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "clusterId")]
    pub cluster_id: String,
    /// <p>The HSM's IP address. If you specify an IP address, use an available address from the subnet that maps to the Availability Zone where you are creating the HSM. If you don't specify an IP address, one is chosen for you from that subnet.</p>
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateHsmResponse {
    /// <p>Information about the HSM that was created.</p>
    #[serde(rename = "hsm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm: Option<Hsm>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBackupRequest {
    /// <p>The ID of the backup to be deleted. To find the ID of a backup, use the <a>DescribeBackups</a> operation.</p>
    #[serde(rename = "backupId")]
    pub backup_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBackupResponse {
    /// <p>Information on the <code>Backup</code> object deleted.</p>
    #[serde(rename = "backup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteClusterRequest {
    /// <p>The identifier (ID) of the cluster that you are deleting. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "clusterId")]
    pub cluster_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteClusterResponse {
    /// <p>Information about the cluster that was deleted.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteHsmRequest {
    /// <p>The identifier (ID) of the cluster that contains the HSM that you are deleting.</p>
    #[serde(rename = "clusterId")]
    pub cluster_id: String,
    /// <p>The identifier (ID) of the elastic network interface (ENI) of the HSM that you are deleting.</p>
    #[serde(rename = "eniId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
    /// <p>The IP address of the elastic network interface (ENI) of the HSM that you are deleting.</p>
    #[serde(rename = "eniIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip: Option<String>,
    /// <p>The identifier (ID) of the HSM that you are deleting.</p>
    #[serde(rename = "hsmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteHsmResponse {
    /// <p>The identifier (ID) of the HSM that was deleted.</p>
    #[serde(rename = "hsmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBackupsRequest {
    /// <p>One or more filters to limit the items returned in the response.</p> <p>Use the <code>backupIds</code> filter to return only the specified backups. Specify backups by their backup identifier (ID).</p> <p>Use the <code>sourceBackupIds</code> filter to return only the backups created from a source backup. The <code>sourceBackupID</code> of a source backup is returned by the <a>CopyBackupToRegion</a> operation.</p> <p>Use the <code>clusterIds</code> filter to return only the backups for the specified clusters. Specify clusters by their cluster identifier (ID).</p> <p>Use the <code>states</code> filter to return only backups that match the specified state.</p> <p>Use the <code>neverExpires</code> filter to return backups filtered by the value in the <code>neverExpires</code> parameter. <code>True</code> returns all backups exempt from the backup retention policy. <code>False</code> returns all backups with a backup retention policy defined at the cluster.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of backups to return in the response. When there are more backups than the number you specify, the response contains a <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> value that you received in the previous response. Use this value to get more backups.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Designates whether or not to sort the return backups by ascending chronological order of generation.</p>
    #[serde(rename = "sortAscending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_ascending: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBackupsResponse {
    /// <p>A list of backups.</p>
    #[serde(rename = "backups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<Backup>>,
    /// <p>An opaque string that indicates that the response contains only a subset of backups. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeClustersRequest {
    /// <p>One or more filters to limit the items returned in the response.</p> <p>Use the <code>clusterIds</code> filter to return only the specified clusters. Specify clusters by their cluster identifier (ID).</p> <p>Use the <code>vpcIds</code> filter to return only the clusters in the specified virtual private clouds (VPCs). Specify VPCs by their VPC identifier (ID).</p> <p>Use the <code>states</code> filter to return only clusters that match the specified state.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of clusters to return in the response. When there are more clusters than the number you specify, the response contains a <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> value that you received in the previous response. Use this value to get more clusters.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeClustersResponse {
    /// <p>A list of clusters.</p>
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<Cluster>>,
    /// <p>An opaque string that indicates that the response contains only a subset of clusters. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains information about the backup that will be copied and created by the <a>CopyBackupToRegion</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DestinationBackup {
    /// <p>The date and time when both the source backup was created.</p>
    #[serde(rename = "createTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    /// <p>The identifier (ID) of the source backup from which the new backup was copied.</p>
    #[serde(rename = "sourceBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup: Option<String>,
    /// <p>The identifier (ID) of the cluster containing the source backup from which the new backup was copied.</p>
    #[serde(rename = "sourceCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cluster: Option<String>,
    /// <p>The AWS region that contains the source backup from which the new backup was copied.</p>
    #[serde(rename = "sourceRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
}

/// <p>Contains information about a hardware security module (HSM) in an AWS CloudHSM cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Hsm {
    /// <p>The Availability Zone that contains the HSM.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The identifier (ID) of the cluster that contains the HSM.</p>
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The identifier (ID) of the HSM's elastic network interface (ENI).</p>
    #[serde(rename = "eniId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
    /// <p>The IP address of the HSM's elastic network interface (ENI).</p>
    #[serde(rename = "eniIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip: Option<String>,
    /// <p>The HSM's identifier (ID).</p>
    #[serde(rename = "hsmId")]
    pub hsm_id: String,
    /// <p>The HSM's state.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of the HSM's state.</p>
    #[serde(rename = "stateMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
    /// <p>The subnet that contains the HSM's elastic network interface (ENI).</p>
    #[serde(rename = "subnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InitializeClusterRequest {
    /// <p>The identifier (ID) of the cluster that you are claiming. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "clusterId")]
    pub cluster_id: String,
    /// <p>The cluster certificate issued (signed) by your issuing certificate authority (CA). The certificate must be in PEM format and can contain a maximum of 5000 characters.</p>
    #[serde(rename = "signedCert")]
    pub signed_cert: String,
    /// <p>The issuing certificate of the issuing certificate authority (CA) that issued (signed) the cluster certificate. You must use a self-signed certificate. The certificate used to sign the HSM CSR must be directly available, and thus must be the root certificate. The certificate must be in PEM format and can contain a maximum of 5000 characters.</p>
    #[serde(rename = "trustAnchor")]
    pub trust_anchor: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InitializeClusterResponse {
    /// <p>The cluster's state.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of the cluster's state.</p>
    #[serde(rename = "stateMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsRequest {
    /// <p>The maximum number of tags to return in the response. When there are more tags than the number you specify, the response contains a <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> value that you received in the previous response. Use this value to get more tags.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The cluster identifier (ID) for the cluster whose tags you are getting. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsResponse {
    /// <p>An opaque string that indicates that the response contains only a subset of tags. Use this value in a subsequent <code>ListTags</code> request to get more tags.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tags.</p>
    #[serde(rename = "tagList")]
    pub tag_list: Vec<Tag>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyBackupAttributesRequest {
    /// <p>The identifier (ID) of the backup to modify. To find the ID of a backup, use the <a>DescribeBackups</a> operation.</p>
    #[serde(rename = "backupId")]
    pub backup_id: String,
    /// <p>Specifies whether the service should exempt a backup from the retention policy for the cluster. <code>True</code> exempts a backup from the retention policy. <code>False</code> means the service applies the backup retention policy defined at the cluster.</p>
    #[serde(rename = "neverExpires")]
    pub never_expires: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyBackupAttributesResponse {
    #[serde(rename = "backup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyClusterRequest {
    /// <p>A policy that defines how the service retains backups.</p>
    #[serde(rename = "backupRetentionPolicy")]
    pub backup_retention_policy: BackupRetentionPolicy,
    /// <p>The identifier (ID) of the cluster that you want to modify. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "clusterId")]
    pub cluster_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyClusterResponse {
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestoreBackupRequest {
    /// <p>The ID of the backup to be restored. To find the ID of a backup, use the <a>DescribeBackups</a> operation.</p>
    #[serde(rename = "backupId")]
    pub backup_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RestoreBackupResponse {
    /// <p>Information on the <code>Backup</code> object created.</p>
    #[serde(rename = "backup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

/// <p>Contains a tag. A tag is a key-value pair.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The cluster identifier (ID) for the cluster that you are tagging. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>A list of one or more tags.</p>
    #[serde(rename = "tagList")]
    pub tag_list: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The cluster identifier (ID) for the cluster whose tags you are removing. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>A list of one or more tag keys for the tags that you are removing. Specify only the tag keys, not the tag values.</p>
    #[serde(rename = "tagKeyList")]
    pub tag_key_list: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// Errors returned by CopyBackupToRegion
#[derive(Debug, PartialEq)]
pub enum CopyBackupToRegionError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// <p>The request was rejected because of a tagging failure. Verify the tag conditions in all applicable policies, and then retry the request.</p>
    CloudHsmTag(String),
}

impl CopyBackupToRegionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopyBackupToRegionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(CopyBackupToRegionError::CloudHsmAccessDenied(
                        err.msg,
                    ))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(CopyBackupToRegionError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(CopyBackupToRegionError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(CopyBackupToRegionError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(CopyBackupToRegionError::CloudHsmService(err.msg))
                }
                "CloudHsmTagException" => {
                    return RusotoError::Service(CopyBackupToRegionError::CloudHsmTag(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CopyBackupToRegionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CopyBackupToRegionError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            CopyBackupToRegionError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            CopyBackupToRegionError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            CopyBackupToRegionError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            CopyBackupToRegionError::CloudHsmService(ref cause) => write!(f, "{}", cause),
            CopyBackupToRegionError::CloudHsmTag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CopyBackupToRegionError {}
/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// <p>The request was rejected because of a tagging failure. Verify the tag conditions in all applicable policies, and then retry the request.</p>
    CloudHsmTag(String),
}

impl CreateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(CreateClusterError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(CreateClusterError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(CreateClusterError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(CreateClusterError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(CreateClusterError::CloudHsmService(err.msg))
                }
                "CloudHsmTagException" => {
                    return RusotoError::Service(CreateClusterError::CloudHsmTag(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateClusterError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            CreateClusterError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            CreateClusterError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateClusterError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateClusterError::CloudHsmService(ref cause) => write!(f, "{}", cause),
            CreateClusterError::CloudHsmTag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateClusterError {}
/// Errors returned by CreateHsm
#[derive(Debug, PartialEq)]
pub enum CreateHsmError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl CreateHsmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHsmError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmInternalFailure(err.msg))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmInvalidRequest(err.msg))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmResourceNotFound(err.msg))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateHsmError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHsmError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            CreateHsmError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            CreateHsmError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateHsmError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateHsmError::CloudHsmService(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateHsmError {}
/// Errors returned by DeleteBackup
#[derive(Debug, PartialEq)]
pub enum DeleteBackupError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl DeleteBackupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(DeleteBackupError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(DeleteBackupError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(DeleteBackupError::CloudHsmInvalidRequest(err.msg))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(DeleteBackupError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DeleteBackupError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBackupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBackupError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteBackupError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteBackupError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteBackupError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteBackupError::CloudHsmService(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBackupError {}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// <p>The request was rejected because of a tagging failure. Verify the tag conditions in all applicable policies, and then retry the request.</p>
    CloudHsmTag(String),
}

impl DeleteClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(DeleteClusterError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(DeleteClusterError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(DeleteClusterError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(DeleteClusterError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DeleteClusterError::CloudHsmService(err.msg))
                }
                "CloudHsmTagException" => {
                    return RusotoError::Service(DeleteClusterError::CloudHsmTag(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteClusterError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::CloudHsmService(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::CloudHsmTag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteClusterError {}
/// Errors returned by DeleteHsm
#[derive(Debug, PartialEq)]
pub enum DeleteHsmError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl DeleteHsmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteHsmError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmInternalFailure(err.msg))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmInvalidRequest(err.msg))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmResourceNotFound(err.msg))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteHsmError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteHsmError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteHsmError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteHsmError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteHsmError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteHsmError::CloudHsmService(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteHsmError {}
/// Errors returned by DescribeBackups
#[derive(Debug, PartialEq)]
pub enum DescribeBackupsError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// <p>The request was rejected because of a tagging failure. Verify the tag conditions in all applicable policies, and then retry the request.</p>
    CloudHsmTag(String),
}

impl DescribeBackupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBackupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(DescribeBackupsError::CloudHsmAccessDenied(
                        err.msg,
                    ))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(DescribeBackupsError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(DescribeBackupsError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(DescribeBackupsError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DescribeBackupsError::CloudHsmService(err.msg))
                }
                "CloudHsmTagException" => {
                    return RusotoError::Service(DescribeBackupsError::CloudHsmTag(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeBackupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBackupsError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeBackupsError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeBackupsError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeBackupsError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeBackupsError::CloudHsmService(ref cause) => write!(f, "{}", cause),
            DescribeBackupsError::CloudHsmTag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBackupsError {}
/// Errors returned by DescribeClusters
#[derive(Debug, PartialEq)]
pub enum DescribeClustersError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// <p>The request was rejected because of a tagging failure. Verify the tag conditions in all applicable policies, and then retry the request.</p>
    CloudHsmTag(String),
}

impl DescribeClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClustersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(DescribeClustersError::CloudHsmAccessDenied(
                        err.msg,
                    ))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(DescribeClustersError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(DescribeClustersError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DescribeClustersError::CloudHsmService(err.msg))
                }
                "CloudHsmTagException" => {
                    return RusotoError::Service(DescribeClustersError::CloudHsmTag(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeClustersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeClustersError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeClustersError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeClustersError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeClustersError::CloudHsmService(ref cause) => write!(f, "{}", cause),
            DescribeClustersError::CloudHsmTag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeClustersError {}
/// Errors returned by InitializeCluster
#[derive(Debug, PartialEq)]
pub enum InitializeClusterError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl InitializeClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InitializeClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(InitializeClusterError::CloudHsmAccessDenied(
                        err.msg,
                    ))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(InitializeClusterError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(InitializeClusterError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(InitializeClusterError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(InitializeClusterError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InitializeClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InitializeClusterError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            InitializeClusterError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            InitializeClusterError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            InitializeClusterError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            InitializeClusterError::CloudHsmService(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InitializeClusterError {}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// <p>The request was rejected because of a tagging failure. Verify the tag conditions in all applicable policies, and then retry the request.</p>
    CloudHsmTag(String),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(ListTagsError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(ListTagsError::CloudHsmInternalFailure(err.msg))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(ListTagsError::CloudHsmInvalidRequest(err.msg))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsError::CloudHsmResourceNotFound(err.msg))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ListTagsError::CloudHsmService(err.msg))
                }
                "CloudHsmTagException" => {
                    return RusotoError::Service(ListTagsError::CloudHsmTag(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            ListTagsError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            ListTagsError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTagsError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsError::CloudHsmService(ref cause) => write!(f, "{}", cause),
            ListTagsError::CloudHsmTag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsError {}
/// Errors returned by ModifyBackupAttributes
#[derive(Debug, PartialEq)]
pub enum ModifyBackupAttributesError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl ModifyBackupAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyBackupAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(ModifyBackupAttributesError::CloudHsmAccessDenied(
                        err.msg,
                    ))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(
                        ModifyBackupAttributesError::CloudHsmInternalFailure(err.msg),
                    )
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(
                        ModifyBackupAttributesError::CloudHsmInvalidRequest(err.msg),
                    )
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(
                        ModifyBackupAttributesError::CloudHsmResourceNotFound(err.msg),
                    )
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ModifyBackupAttributesError::CloudHsmService(
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
impl fmt::Display for ModifyBackupAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyBackupAttributesError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            ModifyBackupAttributesError::CloudHsmInternalFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyBackupAttributesError::CloudHsmInvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyBackupAttributesError::CloudHsmResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyBackupAttributesError::CloudHsmService(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyBackupAttributesError {}
/// Errors returned by ModifyCluster
#[derive(Debug, PartialEq)]
pub enum ModifyClusterError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl ModifyClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(ModifyClusterError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(ModifyClusterError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(ModifyClusterError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(ModifyClusterError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ModifyClusterError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ModifyClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyClusterError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            ModifyClusterError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            ModifyClusterError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            ModifyClusterError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            ModifyClusterError::CloudHsmService(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyClusterError {}
/// Errors returned by RestoreBackup
#[derive(Debug, PartialEq)]
pub enum RestoreBackupError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl RestoreBackupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RestoreBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(RestoreBackupError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(RestoreBackupError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(RestoreBackupError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(RestoreBackupError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(RestoreBackupError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RestoreBackupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestoreBackupError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            RestoreBackupError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            RestoreBackupError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            RestoreBackupError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            RestoreBackupError::CloudHsmService(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RestoreBackupError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// <p>The request was rejected because of a tagging failure. Verify the tag conditions in all applicable policies, and then retry the request.</p>
    CloudHsmTag(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(TagResourceError::CloudHsmInternalFailure(err.msg))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::CloudHsmInvalidRequest(err.msg))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(TagResourceError::CloudHsmService(err.msg))
                }
                "CloudHsmTagException" => {
                    return RusotoError::Service(TagResourceError::CloudHsmTag(err.msg))
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
            TagResourceError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            TagResourceError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::CloudHsmService(ref cause) => write!(f, "{}", cause),
            TagResourceError::CloudHsmTag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// <p>The request was rejected because of a tagging failure. Verify the tag conditions in all applicable policies, and then retry the request.</p>
    CloudHsmTag(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(UntagResourceError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(UntagResourceError::CloudHsmService(err.msg))
                }
                "CloudHsmTagException" => {
                    return RusotoError::Service(UntagResourceError::CloudHsmTag(err.msg))
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
            UntagResourceError::CloudHsmAccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::CloudHsmInternalFailure(ref cause) => write!(f, "{}", cause),
            UntagResourceError::CloudHsmInvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::CloudHsmResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::CloudHsmService(ref cause) => write!(f, "{}", cause),
            UntagResourceError::CloudHsmTag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the CloudHSM V2 API. CloudHSM V2 clients implement this trait.
#[async_trait]
pub trait CloudHsmv2 {
    /// <p>Copy an AWS CloudHSM cluster backup to a different region.</p>
    async fn copy_backup_to_region(
        &self,
        input: CopyBackupToRegionRequest,
    ) -> Result<CopyBackupToRegionResponse, RusotoError<CopyBackupToRegionError>>;

    /// <p>Creates a new AWS CloudHSM cluster.</p>
    async fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> Result<CreateClusterResponse, RusotoError<CreateClusterError>>;

    /// <p>Creates a new hardware security module (HSM) in the specified AWS CloudHSM cluster.</p>
    async fn create_hsm(
        &self,
        input: CreateHsmRequest,
    ) -> Result<CreateHsmResponse, RusotoError<CreateHsmError>>;

    /// <p>Deletes a specified AWS CloudHSM backup. A backup can be restored up to 7 days after the DeleteBackup request is made. For more information on restoring a backup, see <a>RestoreBackup</a>.</p>
    async fn delete_backup(
        &self,
        input: DeleteBackupRequest,
    ) -> Result<DeleteBackupResponse, RusotoError<DeleteBackupError>>;

    /// <p>Deletes the specified AWS CloudHSM cluster. Before you can delete a cluster, you must delete all HSMs in the cluster. To see if the cluster contains any HSMs, use <a>DescribeClusters</a>. To delete an HSM, use <a>DeleteHsm</a>.</p>
    async fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse, RusotoError<DeleteClusterError>>;

    /// <p>Deletes the specified HSM. To specify an HSM, you can use its identifier (ID), the IP address of the HSM's elastic network interface (ENI), or the ID of the HSM's ENI. You need to specify only one of these values. To find these values, use <a>DescribeClusters</a>.</p>
    async fn delete_hsm(
        &self,
        input: DeleteHsmRequest,
    ) -> Result<DeleteHsmResponse, RusotoError<DeleteHsmError>>;

    /// <p>Gets information about backups of AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the backups. When the response contains only a subset of backups, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more backups to get.</p>
    async fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> Result<DescribeBackupsResponse, RusotoError<DescribeBackupsError>>;

    /// <p>Gets information about AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the clusters. When the response contains only a subset of clusters, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more clusters to get.</p>
    async fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> Result<DescribeClustersResponse, RusotoError<DescribeClustersError>>;

    /// <p>Claims an AWS CloudHSM cluster by submitting the cluster certificate issued by your issuing certificate authority (CA) and the CA's root certificate. Before you can claim a cluster, you must sign the cluster's certificate signing request (CSR) with your issuing CA. To get the cluster's CSR, use <a>DescribeClusters</a>.</p>
    async fn initialize_cluster(
        &self,
        input: InitializeClusterRequest,
    ) -> Result<InitializeClusterResponse, RusotoError<InitializeClusterError>>;

    /// <p>Gets a list of tags for the specified AWS CloudHSM cluster.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the tags. When the response contains only a subset of tags, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>ListTags</code> request to get more tags. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more tags to get.</p>
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResponse, RusotoError<ListTagsError>>;

    /// <p>Modifies attributes for AWS CloudHSM backup.</p>
    async fn modify_backup_attributes(
        &self,
        input: ModifyBackupAttributesRequest,
    ) -> Result<ModifyBackupAttributesResponse, RusotoError<ModifyBackupAttributesError>>;

    /// <p>Modifies AWS CloudHSM cluster.</p>
    async fn modify_cluster(
        &self,
        input: ModifyClusterRequest,
    ) -> Result<ModifyClusterResponse, RusotoError<ModifyClusterError>>;

    /// <p>Restores a specified AWS CloudHSM backup that is in the <code>PENDING_DELETION</code> state. For mor information on deleting a backup, see <a>DeleteBackup</a>.</p>
    async fn restore_backup(
        &self,
        input: RestoreBackupRequest,
    ) -> Result<RestoreBackupResponse, RusotoError<RestoreBackupError>>;

    /// <p>Adds or overwrites one or more tags for the specified AWS CloudHSM cluster.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes the specified tag or tags from the specified AWS CloudHSM cluster.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;
}
/// A client for the CloudHSM V2 API.
#[derive(Clone)]
pub struct CloudHsmv2Client {
    client: Client,
    region: region::Region,
}

impl CloudHsmv2Client {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudHsmv2Client {
        CloudHsmv2Client {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudHsmv2Client
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CloudHsmv2Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CloudHsmv2Client {
        CloudHsmv2Client { client, region }
    }
}

#[async_trait]
impl CloudHsmv2 for CloudHsmv2Client {
    /// <p>Copy an AWS CloudHSM cluster backup to a different region.</p>
    async fn copy_backup_to_region(
        &self,
        input: CopyBackupToRegionRequest,
    ) -> Result<CopyBackupToRegionResponse, RusotoError<CopyBackupToRegionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.CopyBackupToRegion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CopyBackupToRegionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CopyBackupToRegionResponse, _>()
    }

    /// <p>Creates a new AWS CloudHSM cluster.</p>
    async fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> Result<CreateClusterResponse, RusotoError<CreateClusterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.CreateCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateClusterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateClusterResponse, _>()
    }

    /// <p>Creates a new hardware security module (HSM) in the specified AWS CloudHSM cluster.</p>
    async fn create_hsm(
        &self,
        input: CreateHsmRequest,
    ) -> Result<CreateHsmResponse, RusotoError<CreateHsmError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.CreateHsm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateHsmError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateHsmResponse, _>()
    }

    /// <p>Deletes a specified AWS CloudHSM backup. A backup can be restored up to 7 days after the DeleteBackup request is made. For more information on restoring a backup, see <a>RestoreBackup</a>.</p>
    async fn delete_backup(
        &self,
        input: DeleteBackupRequest,
    ) -> Result<DeleteBackupResponse, RusotoError<DeleteBackupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.DeleteBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteBackupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteBackupResponse, _>()
    }

    /// <p>Deletes the specified AWS CloudHSM cluster. Before you can delete a cluster, you must delete all HSMs in the cluster. To see if the cluster contains any HSMs, use <a>DescribeClusters</a>. To delete an HSM, use <a>DeleteHsm</a>.</p>
    async fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse, RusotoError<DeleteClusterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.DeleteCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteClusterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteClusterResponse, _>()
    }

    /// <p>Deletes the specified HSM. To specify an HSM, you can use its identifier (ID), the IP address of the HSM's elastic network interface (ENI), or the ID of the HSM's ENI. You need to specify only one of these values. To find these values, use <a>DescribeClusters</a>.</p>
    async fn delete_hsm(
        &self,
        input: DeleteHsmRequest,
    ) -> Result<DeleteHsmResponse, RusotoError<DeleteHsmError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.DeleteHsm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteHsmError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteHsmResponse, _>()
    }

    /// <p>Gets information about backups of AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the backups. When the response contains only a subset of backups, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more backups to get.</p>
    async fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> Result<DescribeBackupsResponse, RusotoError<DescribeBackupsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.DescribeBackups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeBackupsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeBackupsResponse, _>()
    }

    /// <p>Gets information about AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the clusters. When the response contains only a subset of clusters, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more clusters to get.</p>
    async fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> Result<DescribeClustersResponse, RusotoError<DescribeClustersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.DescribeClusters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeClustersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeClustersResponse, _>()
    }

    /// <p>Claims an AWS CloudHSM cluster by submitting the cluster certificate issued by your issuing certificate authority (CA) and the CA's root certificate. Before you can claim a cluster, you must sign the cluster's certificate signing request (CSR) with your issuing CA. To get the cluster's CSR, use <a>DescribeClusters</a>.</p>
    async fn initialize_cluster(
        &self,
        input: InitializeClusterRequest,
    ) -> Result<InitializeClusterResponse, RusotoError<InitializeClusterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.InitializeCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, InitializeClusterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<InitializeClusterResponse, _>()
    }

    /// <p>Gets a list of tags for the specified AWS CloudHSM cluster.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the tags. When the response contains only a subset of tags, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>ListTags</code> request to get more tags. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more tags to get.</p>
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResponse, RusotoError<ListTagsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsResponse, _>()
    }

    /// <p>Modifies attributes for AWS CloudHSM backup.</p>
    async fn modify_backup_attributes(
        &self,
        input: ModifyBackupAttributesRequest,
    ) -> Result<ModifyBackupAttributesResponse, RusotoError<ModifyBackupAttributesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.ModifyBackupAttributes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ModifyBackupAttributesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ModifyBackupAttributesResponse, _>()
    }

    /// <p>Modifies AWS CloudHSM cluster.</p>
    async fn modify_cluster(
        &self,
        input: ModifyClusterRequest,
    ) -> Result<ModifyClusterResponse, RusotoError<ModifyClusterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.ModifyCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ModifyClusterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ModifyClusterResponse, _>()
    }

    /// <p>Restores a specified AWS CloudHSM backup that is in the <code>PENDING_DELETION</code> state. For mor information on deleting a backup, see <a>DeleteBackup</a>.</p>
    async fn restore_backup(
        &self,
        input: RestoreBackupRequest,
    ) -> Result<RestoreBackupResponse, RusotoError<RestoreBackupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.RestoreBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RestoreBackupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RestoreBackupResponse, _>()
    }

    /// <p>Adds or overwrites one or more tags for the specified AWS CloudHSM cluster.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Removes the specified tag or tags from the specified AWS CloudHSM cluster.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "BaldrApiService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }
}
