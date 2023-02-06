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
/// <pre><code>        &lt;p&gt;Associates sasl scram secrets to cluster.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchAssociateScramSecretRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster to be updated.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;List of AWS Secrets Manager secret ARNs.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "secretArnList")]
    pub secret_arn_list: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchAssociateScramSecretResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;List of errors when associating secrets to cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "unprocessedScramSecrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_scram_secrets: Option<Vec<UnprocessedScramSecret>>,
}

/// <pre><code>        &lt;p&gt;Disassociates sasl scram secrets to cluster.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDisassociateScramSecretRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster to be updated.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;List of AWS Secrets Manager secret ARNs.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "secretArnList")]
    pub secret_arn_list: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDisassociateScramSecretResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;List of errors when disassociating secrets to cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "unprocessedScramSecrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_scram_secrets: Option<Vec<UnprocessedScramSecret>>,
}

/// <pre><code>        &lt;p&gt;Specifies the EBS volume upgrade information. The broker identifier must be set to the keyword ALL. This means the changes apply to all the brokers in the cluster.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BrokerEBSVolumeInfo {
    /// <pre><code>        &lt;p&gt;The ID of the broker to update.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "kafkaBrokerNodeId")]
    pub kafka_broker_node_id: String,
    /// <pre><code>        &lt;p&gt;Size of the EBS volume to update.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "volumeSizeGB")]
    pub volume_size_gb: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BrokerLogs {
    #[serde(rename = "cloudWatchLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs: Option<CloudWatchLogs>,
    #[serde(rename = "firehose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose: Option<Firehose>,
    #[serde(rename = "s3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3>,
}

/// <pre><code>        &lt;p&gt;Describes the setup to be used for Kafka broker nodes in the cluster.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BrokerNodeGroupInfo {
    /// <pre><code>        &lt;p&gt;The distribution of broker nodes across Availability Zones. This is an optional parameter. If you don&#39;t specify it, Amazon MSK gives it the value DEFAULT. You can also explicitly set this parameter to the value DEFAULT. No other values are currently allowed.&lt;/p&gt;
    /// &lt;p&gt;Amazon MSK distributes the broker nodes evenly across the Availability Zones that correspond to the subnets you provide when you create the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "brokerAZDistribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_az_distribution: Option<String>,
    /// <pre><code>        &lt;p&gt;The list of subnets to connect to in the client virtual private cloud (VPC). AWS creates elastic network interfaces inside these subnets. Client applications use elastic network interfaces to produce and consume data. Client subnets can&#39;t be in Availability Zone us-east-1e.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clientSubnets")]
    pub client_subnets: Vec<String>,
    /// <pre><code>        &lt;p&gt;The type of Amazon EC2 instances to use for Kafka brokers. The following instance types are allowed: kafka.m5.large, kafka.m5.xlarge, kafka.m5.2xlarge,
    /// </code></pre>
    ///
    /// <p>kafka.m5.4xlarge, kafka.m5.12xlarge, and kafka.m5.24xlarge.</p></p>
    #[serde(rename = "instanceType")]
    pub instance_type: String,
    /// <pre><code>        &lt;p&gt;The AWS security groups to associate with the elastic network interfaces in order to specify who can connect to and communicate with the Amazon MSK cluster. If you don&#39;t specify a security group, Amazon MSK uses the default security group associated with the VPC.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "securityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <pre><code>        &lt;p&gt;Contains information about storage volumes attached to MSK broker nodes.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "storageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_info: Option<StorageInfo>,
}

/// <pre><code>        &lt;p&gt;BrokerNodeInfo&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BrokerNodeInfo {
    /// <pre><code>        &lt;p&gt;The attached elastic network interface of the broker.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "attachedENIId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_eni_id: Option<String>,
    /// <pre><code>        &lt;p&gt;The ID of the broker.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "brokerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<f64>,
    /// <pre><code>        &lt;p&gt;The client subnet to which this broker node belongs.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clientSubnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,
    /// <pre><code>        &lt;p&gt;The virtual private cloud (VPC) of the client.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clientVpcIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_vpc_ip_address: Option<String>,
    /// <pre><code>        &lt;p&gt;Information about the version of software currently deployed on the Kafka brokers in the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "currentBrokerSoftwareInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_broker_software_info: Option<BrokerSoftwareInfo>,
    /// <pre><code>        &lt;p&gt;Endpoints for accessing the broker.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<String>>,
}

/// <pre><code>        &lt;p&gt;Information about the current software installed on the cluster.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BrokerSoftwareInfo {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the configuration used for the cluster. This field isn&#39;t visible in this preview release.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "configurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The revision of the configuration to use. This field isn&#39;t visible in this preview release.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "configurationRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_revision: Option<i64>,
    /// <pre><code>        &lt;p&gt;The version of Apache Kafka.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "kafkaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_version: Option<String>,
}

/// <pre><code>        &lt;p&gt;Includes all client authentication information.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ClientAuthentication {
    /// <pre><code>        &lt;p&gt;Details for ClientAuthentication using SASL.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "sasl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl: Option<Sasl>,
    /// <pre><code>        &lt;p&gt;Details for ClientAuthentication using TLS.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "tls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<Tls>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CloudWatchLogs {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "logGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
}

/// <pre><code>        &lt;p&gt;Returns information about a cluster.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClusterInfo {
    /// <pre><code>        &lt;p&gt;Arn of active cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "activeOperationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_operation_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;Information about the broker nodes.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "brokerNodeGroupInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_node_group_info: Option<BrokerNodeGroupInfo>,
    /// <pre><code>        &lt;p&gt;Includes all client authentication information.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clientAuthentication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<ClientAuthentication>,
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The name of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <pre><code>        &lt;p&gt;The time when the cluster was created.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <pre><code>        &lt;p&gt;Information about the version of software currently deployed on the Kafka brokers in the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "currentBrokerSoftwareInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_broker_software_info: Option<BrokerSoftwareInfo>,
    /// <pre><code>        &lt;p&gt;The current version of the MSK cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "currentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    /// <pre><code>        &lt;p&gt;Includes all encryption-related information.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "encryptionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_info: Option<EncryptionInfo>,
    /// <pre><code>        &lt;p&gt;Specifies which metrics are gathered for the MSK cluster. This property has the following possible values: DEFAULT, PER_BROKER, PER_TOPIC_PER_BROKER, and PER_TOPIC_PER_PARTITION. For a list of the metrics associated with each of these levels of monitoring, see &lt;a href=&quot;https://docs.aws.amazon.com/msk/latest/developerguide/monitoring.html&quot;&gt;Monitoring&lt;/a&gt;.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "enhancedMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    #[serde(rename = "loggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <pre><code>        &lt;p&gt;The number of broker nodes in the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "numberOfBrokerNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_broker_nodes: Option<i64>,
    /// <pre><code>        &lt;p&gt;Settings for open monitoring using Prometheus.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "openMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_monitoring: Option<OpenMonitoring>,
    /// <pre><code>        &lt;p&gt;The state of the cluster. The possible states are ACTIVE, CREATING, DELETING, FAILED, HEALING, MAINTENANCE, REBOOTING_BROKER, and UPDATING.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_info: Option<StateInfo>,
    /// <pre><code>        &lt;p&gt;Tags attached to the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <pre><code>        &lt;p&gt;The connection string to use to connect to the Apache ZooKeeper cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "zookeeperConnectString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_connect_string: Option<String>,
    /// <pre><code>        &lt;p&gt;The connection string to use to connect to zookeeper cluster on Tls port.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "zookeeperConnectStringTls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_connect_string_tls: Option<String>,
}

/// <pre><code>        &lt;p&gt;Returns information about a cluster operation.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClusterOperationInfo {
    /// <pre><code>        &lt;p&gt;The ID of the API request that triggered this operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clientRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    /// <pre><code>        &lt;p&gt;ARN of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The time that the operation was created.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <pre><code>        &lt;p&gt;The time at which the operation finished.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <pre><code>        &lt;p&gt;Describes the error if the operation fails.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "errorInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ErrorInfo>,
    /// <pre><code>        &lt;p&gt;ARN of the cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "operationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;State of the cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "operationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_state: Option<String>,
    /// <pre><code>        &lt;p&gt;Steps completed during the operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "operationSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_steps: Option<Vec<ClusterOperationStep>>,
    /// <pre><code>        &lt;p&gt;Type of the cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    /// <pre><code>        &lt;p&gt;Information about cluster attributes before a cluster is updated.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "sourceClusterInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cluster_info: Option<MutableClusterInfo>,
    /// <pre><code>        &lt;p&gt;Information about cluster attributes after a cluster is updated.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "targetClusterInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cluster_info: Option<MutableClusterInfo>,
}

/// <pre><code>        &lt;p&gt;Step taken during a cluster operation.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClusterOperationStep {
    /// <pre><code>        &lt;p&gt;Information about the step and its status.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "stepInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_info: Option<ClusterOperationStepInfo>,
    /// <pre><code>        &lt;p&gt;The name of the step.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "stepName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_name: Option<String>,
}

/// <pre><code>        &lt;p&gt;State information about the operation step.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClusterOperationStepInfo {
    /// <pre><code>        &lt;p&gt;The steps current status.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "stepStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_status: Option<String>,
}

/// <pre><code>        &lt;p&gt;Contains source Kafka versions and compatible target Kafka versions.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CompatibleKafkaVersion {
    /// <pre><code>        &lt;p&gt;A Kafka version.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "sourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    /// <pre><code>        &lt;p&gt;A list of Kafka versions.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "targetVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_versions: Option<Vec<String>>,
}

/// <pre><code>        &lt;p&gt;Represents an MSK Configuration.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Configuration {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <pre><code>        &lt;p&gt;The time when the configuration was created.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "creationTime")]
    pub creation_time: f64,
    /// <pre><code>        &lt;p&gt;The description of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "description")]
    pub description: String,
    /// <pre><code>        &lt;p&gt;An array of the versions of Apache Kafka with which you can use this MSK configuration. You can use this configuration for an MSK cluster only if the Apache Kafka version specified for the cluster appears in this array.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "kafkaVersions")]
    pub kafka_versions: Vec<String>,
    /// <pre><code>        &lt;p&gt;Latest revision of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "latestRevision")]
    pub latest_revision: ConfigurationRevision,
    /// <pre><code>        &lt;p&gt;The name of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "name")]
    pub name: String,
    /// <pre><code>        &lt;p&gt;The state of the configuration. The possible states are ACTIVE, DELETING, and DELETE_FAILED. &lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "state")]
    pub state: String,
}

/// <pre><code>        &lt;p&gt;Specifies the configuration to use for the brokers.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfigurationInfo {
    /// <pre><code>        &lt;p&gt;ARN of the configuration to use.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <pre><code>        &lt;p&gt;The revision of the configuration to use.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "revision")]
    pub revision: i64,
}

/// <pre><code>        &lt;p&gt;Describes a configuration revision.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigurationRevision {
    /// <pre><code>        &lt;p&gt;The time when the configuration revision was created.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "creationTime")]
    pub creation_time: f64,
    /// <pre><code>        &lt;p&gt;The description of the configuration revision.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <pre><code>        &lt;p&gt;The revision number.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "revision")]
    pub revision: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateClusterRequest {
    /// <pre><code>        &lt;p&gt;Information about the broker nodes in the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "brokerNodeGroupInfo")]
    pub broker_node_group_info: BrokerNodeGroupInfo,
    /// <pre><code>        &lt;p&gt;Includes all client authentication related information.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clientAuthentication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<ClientAuthentication>,
    /// <pre><code>        &lt;p&gt;The name of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <pre><code>        &lt;p&gt;Represents the configuration that you want MSK to use for the brokers in a cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "configurationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_info: Option<ConfigurationInfo>,
    /// <pre><code>        &lt;p&gt;Includes all encryption-related information.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "encryptionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_info: Option<EncryptionInfo>,
    /// <pre><code>        &lt;p&gt;Specifies the level of monitoring for the MSK cluster. The possible values are DEFAULT, PER_BROKER, PER_TOPIC_PER_BROKER, and PER_TOPIC_PER_PARTITION.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "enhancedMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    /// <pre><code>        &lt;p&gt;The version of Apache Kafka.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "kafkaVersion")]
    pub kafka_version: String,
    #[serde(rename = "loggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <pre><code>        &lt;p&gt;The number of broker nodes in the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "numberOfBrokerNodes")]
    pub number_of_broker_nodes: i64,
    /// <pre><code>        &lt;p&gt;The settings for open monitoring.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "openMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_monitoring: Option<OpenMonitoringInfo>,
    /// <pre><code>        &lt;p&gt;Create tags when creating the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateClusterResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The name of the MSK cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <pre><code>        &lt;p&gt;The state of the cluster. The possible states are ACTIVE, CREATING, DELETING, FAILED, HEALING, MAINTENANCE, REBOOTING_BROKER, and UPDATING.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConfigurationRequest {
    /// <pre><code>        &lt;p&gt;The description of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <pre><code>        &lt;p&gt;The versions of Apache Kafka with which you can use this MSK configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "kafkaVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_versions: Option<Vec<String>>,
    /// <pre><code>        &lt;p&gt;The name of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "name")]
    pub name: String,
    /// <pre><code>        &lt;p&gt;Contents of the &lt;filename&gt;server.properties&lt;/filename&gt; file. When using the API, you must ensure that the contents of the file are base64 encoded.
    /// When using the AWS Management Console, the SDK, or the AWS CLI, the contents of &lt;filename&gt;server.properties&lt;/filename&gt; can be in plaintext.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "serverProperties")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub server_properties: bytes::Bytes,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConfigurationResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The time when the configuration was created.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <pre><code>        &lt;p&gt;Latest revision of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "latestRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    /// <pre><code>        &lt;p&gt;The name of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <pre><code>        &lt;p&gt;The state of the configuration. The possible states are ACTIVE, DELETING, and DELETE_FAILED. &lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteClusterRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;The current version of the MSK cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "currentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteClusterResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The state of the cluster. The possible states are ACTIVE, CREATING, DELETING, FAILED, HEALING, MAINTENANCE, REBOOTING_BROKER, and UPDATING.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigurationRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies an MSK configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConfigurationResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies an MSK configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The state of the configuration. The possible states are ACTIVE, DELETING, and DELETE_FAILED. &lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeClusterOperationRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the MSK cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterOperationArn")]
    pub cluster_operation_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeClusterOperationResponse {
    /// <pre><code>        &lt;p&gt;Cluster operation information&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterOperationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_info: Option<ClusterOperationInfo>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeClusterRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeClusterResponse {
    /// <pre><code>        &lt;p&gt;The cluster information.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_info: Option<ClusterInfo>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConfigurationRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies an MSK configuration and all of its revisions.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConfigurationResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The time when the configuration was created.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <pre><code>        &lt;p&gt;The description of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <pre><code>        &lt;p&gt;The versions of Apache Kafka with which you can use this MSK configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "kafkaVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_versions: Option<Vec<String>>,
    /// <pre><code>        &lt;p&gt;Latest revision of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "latestRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    /// <pre><code>        &lt;p&gt;The name of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <pre><code>        &lt;p&gt;The state of the configuration. The possible states are ACTIVE, DELETING, and DELETE_FAILED. &lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConfigurationRevisionRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies an MSK configuration and all of its revisions.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <pre><code>        &lt;p&gt;A string that uniquely identifies a revision of an MSK configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "revision")]
    pub revision: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConfigurationRevisionResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The time when the configuration was created.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <pre><code>        &lt;p&gt;The description of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <pre><code>        &lt;p&gt;The revision number.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    /// <pre><code>        &lt;p&gt;Contents of the &lt;filename&gt;server.properties&lt;/filename&gt; file. When using the API, you must ensure that the contents of the file are base64 encoded.
    /// When using the AWS Management Console, the SDK, or the AWS CLI, the contents of &lt;filename&gt;server.properties&lt;/filename&gt; can be in plaintext.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "serverProperties")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_properties: Option<bytes::Bytes>,
}

/// <pre><code>        &lt;p&gt;Contains information about the EBS storage volumes attached to Kafka broker nodes.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EBSStorageInfo {
    /// <pre><code>        &lt;p&gt;The size in GiB of the EBS volume for the data drive on each broker node.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "volumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
}

/// <pre><code>        &lt;p&gt;The data-volume encryption details.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EncryptionAtRest {
    /// <pre><code>        &lt;p&gt;The ARN of the AWS KMS key for encrypting data at rest. If you don&#39;t specify a KMS key, MSK creates one for you and uses it.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "dataVolumeKMSKeyId")]
    pub data_volume_kms_key_id: String,
}

/// <pre><code>        &lt;p&gt;The settings for encrypting data in transit.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EncryptionInTransit {
    /// <pre><code>        &lt;p&gt;Indicates the encryption setting for data in transit between clients and brokers. The following are the possible values.&lt;/p&gt;
    /// &lt;p&gt;
    /// TLS means that client-broker communication is enabled with TLS only.&lt;/p&gt;
    /// &lt;p&gt;
    /// TLS_PLAINTEXT means that client-broker communication is enabled for both TLS-encrypted, as well as plaintext data.&lt;/p&gt;
    /// &lt;p&gt;
    /// PLAINTEXT means that client-broker communication is enabled in plaintext only.&lt;/p&gt;
    /// &lt;p&gt;The default value is TLS_PLAINTEXT.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clientBroker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_broker: Option<String>,
    /// <pre><code>        &lt;p&gt;When set to true, it indicates that data communication among the broker nodes of the cluster is encrypted. When set to false, the communication happens in plaintext.&lt;/p&gt;
    /// &lt;p&gt;The default value is true.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "inCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_cluster: Option<bool>,
}

/// <pre><code>        &lt;p&gt;Includes encryption-related information, such as the AWS KMS key used for encrypting data at rest and whether you want MSK to encrypt your data in transit.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EncryptionInfo {
    /// <pre><code>        &lt;p&gt;The data-volume encryption details.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "encryptionAtRest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest: Option<EncryptionAtRest>,
    /// <pre><code>        &lt;p&gt;The details for encryption in transit.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "encryptionInTransit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_in_transit: Option<EncryptionInTransit>,
}

/// <pre><code>        &lt;p&gt;Returns information about an error state of the cluster.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorInfo {
    /// <pre><code>        &lt;p&gt;A number describing the error programmatically.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <pre><code>        &lt;p&gt;An optional field to provide more details about the error.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "errorString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Firehose {
    #[serde(rename = "deliveryStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream: Option<String>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBootstrapBrokersRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBootstrapBrokersResponse {
    /// <pre><code>        &lt;p&gt;A string containing one or more hostname:port pairs.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "bootstrapBrokerString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string: Option<String>,
    /// <pre><code>        &lt;p&gt;A string that contains one or more DNS names (or IP addresses) and SASL IAM port pairs.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "bootstrapBrokerStringSaslIam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_sasl_iam: Option<String>,
    /// <pre><code>        &lt;p&gt;A string containing one or more DNS names (or IP) and Sasl Scram port pairs.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "bootstrapBrokerStringSaslScram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_sasl_scram: Option<String>,
    /// <pre><code>        &lt;p&gt;A string containing one or more DNS names (or IP) and TLS port pairs.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "bootstrapBrokerStringTls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string_tls: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCompatibleKafkaVersionsRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster check.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCompatibleKafkaVersionsResponse {
    /// <pre><code>        &lt;p&gt;A list of CompatibleKafkaVersion objects.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "compatibleKafkaVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_kafka_versions: Option<Vec<CompatibleKafkaVersion>>,
}

/// <pre><code>        &lt;p&gt;Details for IAM access control.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Iam {
    /// <pre><code>        &lt;p&gt;Indicates whether IAM access control is enabled.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the JMX Exporter.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JmxExporter {
    /// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the JMX Exporter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "enabledInBroker")]
    pub enabled_in_broker: bool,
}

/// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the JMX Exporter.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct JmxExporterInfo {
    /// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the JMX Exporter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "enabledInBroker")]
    pub enabled_in_broker: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KafkaVersion {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListClusterOperationsRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;The maximum number of results to return in the response. If there are more results, the response includes a NextToken parameter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <pre><code>        &lt;p&gt;The paginated results marker. When the result of the operation is truncated, the call returns NextToken in the response.
    /// To get the next batch, provide this token in your next request.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListClusterOperationsResponse {
    /// <pre><code>        &lt;p&gt;An array of cluster operation information objects.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterOperationInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_info_list: Option<Vec<ClusterOperationInfo>>,
    /// <pre><code>        &lt;p&gt;If the response of ListClusterOperations is truncated, it returns a NextToken in the response. This Nexttoken should be sent in the subsequent request to ListClusterOperations.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListClustersRequest {
    /// <pre><code>        &lt;p&gt;Specify a prefix of the name of the clusters that you want to list. The service lists all the clusters whose names start with this prefix.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterNameFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name_filter: Option<String>,
    /// <pre><code>        &lt;p&gt;The maximum number of results to return in the response. If there are more results, the response includes a NextToken parameter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <pre><code>        &lt;p&gt;The paginated results marker. When the result of the operation is truncated, the call returns NextToken in the response.
    /// To get the next batch, provide this token in your next request.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListClustersResponse {
    /// <pre><code>        &lt;p&gt;Information on each of the MSK clusters in the response.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_info_list: Option<Vec<ClusterInfo>>,
    /// <pre><code>        &lt;p&gt;The paginated results marker. When the result of a ListClusters operation is truncated, the call returns NextToken in the response.
    /// To get another batch of clusters, provide this token in your next request.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConfigurationRevisionsRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies an MSK configuration and all of its revisions.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <pre><code>        &lt;p&gt;The maximum number of results to return in the response. If there are more results, the response includes a NextToken parameter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <pre><code>        &lt;p&gt;The paginated results marker. When the result of the operation is truncated, the call returns NextToken in the response.
    /// To get the next batch, provide this token in your next request.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConfigurationRevisionsResponse {
    /// <pre><code>        &lt;p&gt;Paginated results marker.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <pre><code>        &lt;p&gt;List of ConfigurationRevision objects.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "revisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<ConfigurationRevision>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConfigurationsRequest {
    /// <pre><code>        &lt;p&gt;The maximum number of results to return in the response. If there are more results, the response includes a NextToken parameter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <pre><code>        &lt;p&gt;The paginated results marker. When the result of the operation is truncated, the call returns NextToken in the response.
    /// To get the next batch, provide this token in your next request.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConfigurationsResponse {
    /// <pre><code>        &lt;p&gt;An array of MSK configurations.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    /// <pre><code>        &lt;p&gt;The paginated results marker. When the result of a ListConfigurations operation is truncated, the call returns NextToken in the response.
    /// To get another batch of configurations, provide this token in your next request.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListKafkaVersionsRequest {
    /// <pre><code>        &lt;p&gt;The maximum number of results to return in the response. If there are more results, the response includes a NextToken parameter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <pre><code>        &lt;p&gt;The paginated results marker. When the result of the operation is truncated, the call returns NextToken in the response. To get the next batch, provide this token in your next request.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListKafkaVersionsResponse {
    #[serde(rename = "kafkaVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_versions: Option<Vec<KafkaVersion>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListNodesRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;The maximum number of results to return in the response. If there are more results, the response includes a NextToken parameter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <pre><code>        &lt;p&gt;The paginated results marker. When the result of the operation is truncated, the call returns NextToken in the response.
    /// To get the next batch, provide this token in your next request.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListNodesResponse {
    /// <pre><code>        &lt;p&gt;The paginated results marker. When the result of a ListNodes operation is truncated, the call returns NextToken in the response.
    /// To get another batch of nodes, provide this token in your next request.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <pre><code>        &lt;p&gt;List containing a NodeInfo object.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nodeInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_info_list: Option<Vec<NodeInfo>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListScramSecretsRequest {
    /// <pre><code>        &lt;p&gt;The arn of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;The maxResults of the query.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <pre><code>        &lt;p&gt;The nextToken of the query.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListScramSecretsResponse {
    /// <pre><code>        &lt;p&gt;Paginated results marker.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <pre><code>        &lt;p&gt;The list of scram secrets associated with the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "secretArnList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn_list: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the resource that&#39;s associated with the tags.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <pre><code>        &lt;p&gt;The key-value pair for the resource tag.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LoggingInfo {
    #[serde(rename = "brokerLogs")]
    pub broker_logs: BrokerLogs,
}

/// <pre><code>        &lt;p&gt;Information about cluster attributes that can be updated via update APIs.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MutableClusterInfo {
    /// <pre><code>        &lt;p&gt;Specifies the size of the EBS volume and the ID of the associated broker.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "brokerEBSVolumeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_ebs_volume_info: Option<Vec<BrokerEBSVolumeInfo>>,
    /// <pre><code>        &lt;p&gt;Information about the changes in the configuration of the brokers.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "configurationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_info: Option<ConfigurationInfo>,
    /// <pre><code>        &lt;p&gt;Specifies which Apache Kafka metrics Amazon MSK gathers and sends to Amazon CloudWatch for this cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "enhancedMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    /// <pre><code>        &lt;p&gt;Information about the Amazon MSK broker type.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <pre><code>        &lt;p&gt;The Kafka version.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "kafkaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_version: Option<String>,
    #[serde(rename = "loggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <pre><code>        &lt;p&gt;The number of broker nodes in the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "numberOfBrokerNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_broker_nodes: Option<i64>,
    /// <pre><code>        &lt;p&gt;The settings for open monitoring.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "openMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_monitoring: Option<OpenMonitoring>,
}

/// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the Node Exporter.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodeExporter {
    /// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the Node Exporter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "enabledInBroker")]
    pub enabled_in_broker: bool,
}

/// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the Node Exporter.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NodeExporterInfo {
    /// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the Node Exporter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "enabledInBroker")]
    pub enabled_in_broker: bool,
}

/// <pre><code>        &lt;p&gt;The node information object.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodeInfo {
    /// <pre><code>        &lt;p&gt;The start time.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "addedToClusterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_cluster_time: Option<String>,
    /// <pre><code>        &lt;p&gt;The broker node info.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "brokerNodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_node_info: Option<BrokerNodeInfo>,
    /// <pre><code>        &lt;p&gt;The instance type.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the node.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nodeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The node type.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <pre><code>        &lt;p&gt;The ZookeeperNodeInfo.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "zookeeperNodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_node_info: Option<ZookeeperNodeInfo>,
}

/// <pre><code>        &lt;p&gt;JMX and Node monitoring for the MSK cluster.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OpenMonitoring {
    /// <pre><code>        &lt;p&gt;Prometheus settings.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "prometheus")]
    pub prometheus: Prometheus,
}

/// <pre><code>        &lt;p&gt;JMX and Node monitoring for the MSK cluster.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OpenMonitoringInfo {
    /// <pre><code>        &lt;p&gt;Prometheus settings.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "prometheus")]
    pub prometheus: PrometheusInfo,
}

/// <pre><code>        &lt;p&gt;Prometheus settings.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Prometheus {
    /// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the JMX Exporter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "jmxExporter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jmx_exporter: Option<JmxExporter>,
    /// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the Node Exporter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nodeExporter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_exporter: Option<NodeExporter>,
}

/// <pre><code>        &lt;p&gt;Prometheus settings.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PrometheusInfo {
    /// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the JMX Exporter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "jmxExporter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jmx_exporter: Option<JmxExporterInfo>,
    /// <pre><code>        &lt;p&gt;Indicates whether you want to enable or disable the Node Exporter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "nodeExporter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_exporter: Option<NodeExporterInfo>,
}

/// <p>Reboots a node.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebootBrokerRequest {
    /// <pre><code>        &lt;p&gt;The list of broker IDs to be rebooted. The reboot-broker operation supports rebooting one broker at a time.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "brokerIds")]
    pub broker_ids: Vec<String>,
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster to be updated.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RebootBrokerResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterOperationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3 {
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// <pre><code>        &lt;p&gt;Details for client authentication using SASL.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Sasl {
    /// <pre><code>        &lt;p&gt;Indicates whether IAM access control is enabled.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "iam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<Iam>,
    /// <pre><code>        &lt;p&gt;Details for SASL/SCRAM client authentication.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "scram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scram: Option<Scram>,
}

/// <pre><code>        &lt;p&gt;Details for SASL/SCRAM client authentication.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scram {
    /// <pre><code>        &lt;p&gt;SASL/SCRAM authentication is enabled or not.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StateInfo {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <pre><code>        &lt;p&gt;Contains information about storage volumes attached to MSK broker nodes.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StorageInfo {
    /// <pre><code>        &lt;p&gt;EBS volume information.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "ebsStorageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_storage_info: Option<EBSStorageInfo>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the resource that&#39;s associated with the tags.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <pre><code>        &lt;p&gt;The key-value pair for the resource tag.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <pre><code>        &lt;p&gt;Details for client authentication using TLS.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tls {
    /// <pre><code>        &lt;p&gt;List of ACM Certificate Authority ARNs.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "certificateAuthorityArnList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn_list: Option<Vec<String>>,
}

/// <pre><code>        &lt;p&gt;Error info for scram secret associate/disassociate failure.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnprocessedScramSecret {
    /// <pre><code>        &lt;p&gt;Error code for associate/disassociate failure.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <pre><code>        &lt;p&gt;Error message for associate/disassociate failure.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <pre><code>        &lt;p&gt;AWS Secrets Manager secret ARN.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "secretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the resource that&#39;s associated with the tags.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <pre><code>        &lt;p&gt;Tag keys must be unique for a given cluster. In addition, the following restrictions apply:&lt;/p&gt;
    /// &lt;ul&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;Each tag key must be unique. If you add a tag with a key that&#39;s already in
    /// use, your new tag overwrites the existing key-value pair. &lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;You can&#39;t start a tag key with aws: because this prefix is reserved for use
    /// by  AWS.  AWS creates tags that begin with this prefix on your behalf, but
    /// you can&#39;t edit or delete them.&lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;Tag keys must be between 1 and 128 Unicode characters in length.&lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;li&gt;
    /// &lt;p&gt;Tag keys must consist of the following characters: Unicode letters, digits,
    /// white space, and the following special characters: _ . / = + -
    /// @.&lt;/p&gt;
    /// &lt;/li&gt;
    /// &lt;/ul&gt;
    /// </code></pre>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateBrokerCountRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;The version of cluster to update from. A successful operation will then generate a new version.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "currentVersion")]
    pub current_version: String,
    /// <pre><code>        &lt;p&gt;The number of broker nodes that you want the cluster to have after this operation completes successfully.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "targetNumberOfBrokerNodes")]
    pub target_number_of_broker_nodes: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBrokerCountResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterOperationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateBrokerStorageRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;The version of cluster to update from. A successful operation will then generate a new version.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "currentVersion")]
    pub current_version: String,
    /// <pre><code>        &lt;p&gt;Describes the target volume size and the ID of the broker to apply the update to.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "targetBrokerEBSVolumeInfo")]
    pub target_broker_ebs_volume_info: Vec<BrokerEBSVolumeInfo>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBrokerStorageResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterOperationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateBrokerTypeRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;The cluster version that you want to change. After this operation completes successfully, the cluster will have a new version.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "currentVersion")]
    pub current_version: String,
    /// <pre><code>        &lt;p&gt;The Amazon MSK broker type that you want all of the brokers in this cluster to be.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "targetInstanceType")]
    pub target_instance_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBrokerTypeResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterOperationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateClusterConfigurationRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;Represents the configuration that you want MSK to use for the brokers in a cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "configurationInfo")]
    pub configuration_info: ConfigurationInfo,
    /// <pre><code>        &lt;p&gt;The version of the cluster that needs to be updated.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "currentVersion")]
    pub current_version: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateClusterConfigurationResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterOperationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateClusterKafkaVersionRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster to be updated.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;The custom configuration that should be applied on the new version of cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "configurationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_info: Option<ConfigurationInfo>,
    /// <pre><code>        &lt;p&gt;Current cluster version.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "currentVersion")]
    pub current_version: String,
    /// <pre><code>        &lt;p&gt;Target Kafka version.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "targetKafkaVersion")]
    pub target_kafka_version: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateClusterKafkaVersionResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterOperationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConfigurationRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <pre><code>        &lt;p&gt;The description of the configuration revision.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <pre><code>        &lt;p&gt;Contents of the &lt;filename&gt;server.properties&lt;/filename&gt; file. When using the API, you must ensure that the contents of the file are base64 encoded.
    /// When using the AWS Management Console, the SDK, or the AWS CLI, the contents of &lt;filename&gt;server.properties&lt;/filename&gt; can be in plaintext.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "serverProperties")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub server_properties: bytes::Bytes,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConfigurationResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <pre><code>        &lt;p&gt;Latest revision of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "latestRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
}

/// <p>Request body for UpdateMonitoring.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMonitoringRequest {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) that uniquely identifies the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    pub cluster_arn: String,
    /// <pre><code>        &lt;p&gt;The version of the MSK cluster to update. Cluster versions aren&#39;t simple numbers. You can describe an MSK cluster to find its version. When this update operation is successful, it generates a new cluster version.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "currentVersion")]
    pub current_version: String,
    /// <pre><code>        &lt;p&gt;Specifies which Apache Kafka metrics Amazon MSK gathers and sends to Amazon CloudWatch for this cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "enhancedMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    #[serde(rename = "loggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <pre><code>        &lt;p&gt;The settings for open monitoring.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "openMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_monitoring: Option<OpenMonitoringInfo>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMonitoringResponse {
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <pre><code>        &lt;p&gt;The Amazon Resource Name (ARN) of the cluster operation.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clusterOperationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_operation_arn: Option<String>,
}

/// <pre><code>        &lt;p&gt;Zookeeper node information.&lt;/p&gt;
/// </code></pre>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ZookeeperNodeInfo {
    /// <pre><code>        &lt;p&gt;The attached elastic network interface of the broker.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "attachedENIId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_eni_id: Option<String>,
    /// <pre><code>        &lt;p&gt;The virtual private cloud (VPC) IP address of the client.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "clientVpcIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_vpc_ip_address: Option<String>,
    /// <pre><code>        &lt;p&gt;Endpoints for accessing the ZooKeeper.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<String>>,
    /// <pre><code>        &lt;p&gt;The role-specific ID for Zookeeper.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "zookeeperId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_id: Option<f64>,
    /// <pre><code>        &lt;p&gt;The version of Zookeeper.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "zookeeperVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_version: Option<String>,
}

/// Errors returned by BatchAssociateScramSecret
#[derive(Debug, PartialEq)]
pub enum BatchAssociateScramSecretError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    TooManyRequests(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl BatchAssociateScramSecretError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchAssociateScramSecretError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(BatchAssociateScramSecretError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchAssociateScramSecretError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        BatchAssociateScramSecretError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchAssociateScramSecretError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        BatchAssociateScramSecretError::ServiceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(BatchAssociateScramSecretError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(BatchAssociateScramSecretError::Unauthorized(
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
impl fmt::Display for BatchAssociateScramSecretError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchAssociateScramSecretError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchAssociateScramSecretError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchAssociateScramSecretError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchAssociateScramSecretError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchAssociateScramSecretError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchAssociateScramSecretError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            BatchAssociateScramSecretError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchAssociateScramSecretError {}
/// Errors returned by BatchDisassociateScramSecret
#[derive(Debug, PartialEq)]
pub enum BatchDisassociateScramSecretError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    TooManyRequests(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl BatchDisassociateScramSecretError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchDisassociateScramSecretError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(BatchDisassociateScramSecretError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchDisassociateScramSecretError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        BatchDisassociateScramSecretError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchDisassociateScramSecretError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        BatchDisassociateScramSecretError::ServiceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        BatchDisassociateScramSecretError::TooManyRequests(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(BatchDisassociateScramSecretError::Unauthorized(
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
impl fmt::Display for BatchDisassociateScramSecretError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDisassociateScramSecretError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchDisassociateScramSecretError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchDisassociateScramSecretError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDisassociateScramSecretError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchDisassociateScramSecretError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDisassociateScramSecretError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            BatchDisassociateScramSecretError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDisassociateScramSecretError {}
/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Conflict(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    TooManyRequests(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl CreateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateClusterError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateClusterError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateClusterError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateClusterError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateClusterError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateClusterError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateClusterError::Unauthorized(err.msg))
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
            CreateClusterError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateClusterError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateClusterError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateClusterError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateClusterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateClusterError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateClusterError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateClusterError {}
/// Errors returned by CreateConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Conflict(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    TooManyRequests(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl CreateConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateConfigurationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateConfigurationError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateConfigurationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateConfigurationError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateConfigurationError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateConfigurationError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateConfigurationError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateConfigurationError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateConfigurationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateConfigurationError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateConfigurationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateConfigurationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConfigurationError {}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
}

impl DeleteClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteClusterError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteClusterError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteClusterError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteClusterError::NotFound(err.msg))
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
            DeleteClusterError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteClusterError {}
/// Errors returned by DeleteConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
}

impl DeleteConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteConfigurationError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteConfigurationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteConfigurationError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteConfigurationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteConfigurationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConfigurationError {}
/// Errors returned by DescribeCluster
#[derive(Debug, PartialEq)]
pub enum DescribeClusterError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl DescribeClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeClusterError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeClusterError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeClusterError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeClusterError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeClusterError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeClusterError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeClusterError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeClusterError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeClusterError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeClusterError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeClusterError {}
/// Errors returned by DescribeClusterOperation
#[derive(Debug, PartialEq)]
pub enum DescribeClusterOperationError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl DescribeClusterOperationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClusterOperationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeClusterOperationError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeClusterOperationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeClusterOperationError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeClusterOperationError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeClusterOperationError::Unauthorized(
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
impl fmt::Display for DescribeClusterOperationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeClusterOperationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeClusterOperationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeClusterOperationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeClusterOperationError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeClusterOperationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeClusterOperationError {}
/// Errors returned by DescribeConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl DescribeConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeConfigurationError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeConfigurationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeConfigurationError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeConfigurationError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeConfigurationError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeConfigurationError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeConfigurationError {}
/// Errors returned by DescribeConfigurationRevision
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationRevisionError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl DescribeConfigurationRevisionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigurationRevisionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeConfigurationRevisionError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeConfigurationRevisionError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeConfigurationRevisionError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeConfigurationRevisionError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DescribeConfigurationRevisionError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeConfigurationRevisionError::Unauthorized(
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
impl fmt::Display for DescribeConfigurationRevisionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConfigurationRevisionError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationRevisionError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationRevisionError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConfigurationRevisionError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationRevisionError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConfigurationRevisionError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeConfigurationRevisionError {}
/// Errors returned by GetBootstrapBrokers
#[derive(Debug, PartialEq)]
pub enum GetBootstrapBrokersError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Conflict(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl GetBootstrapBrokersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBootstrapBrokersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBootstrapBrokersError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetBootstrapBrokersError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetBootstrapBrokersError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetBootstrapBrokersError::InternalServerError(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetBootstrapBrokersError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBootstrapBrokersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBootstrapBrokersError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBootstrapBrokersError::Conflict(ref cause) => write!(f, "{}", cause),
            GetBootstrapBrokersError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetBootstrapBrokersError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetBootstrapBrokersError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBootstrapBrokersError {}
/// Errors returned by GetCompatibleKafkaVersions
#[derive(Debug, PartialEq)]
pub enum GetCompatibleKafkaVersionsError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    TooManyRequests(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl GetCompatibleKafkaVersionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCompatibleKafkaVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetCompatibleKafkaVersionsError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetCompatibleKafkaVersionsError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        GetCompatibleKafkaVersionsError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCompatibleKafkaVersionsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetCompatibleKafkaVersionsError::ServiceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCompatibleKafkaVersionsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetCompatibleKafkaVersionsError::Unauthorized(
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
impl fmt::Display for GetCompatibleKafkaVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCompatibleKafkaVersionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetCompatibleKafkaVersionsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetCompatibleKafkaVersionsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCompatibleKafkaVersionsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetCompatibleKafkaVersionsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCompatibleKafkaVersionsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            GetCompatibleKafkaVersionsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCompatibleKafkaVersionsError {}
/// Errors returned by ListClusterOperations
#[derive(Debug, PartialEq)]
pub enum ListClusterOperationsError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl ListClusterOperationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListClusterOperationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListClusterOperationsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListClusterOperationsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListClusterOperationsError::InternalServerError(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListClusterOperationsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListClusterOperationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListClusterOperationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListClusterOperationsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListClusterOperationsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListClusterOperationsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListClusterOperationsError {}
/// Errors returned by ListClusters
#[derive(Debug, PartialEq)]
pub enum ListClustersError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl ListClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListClustersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListClustersError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListClustersError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListClustersError::InternalServerError(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListClustersError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListClustersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListClustersError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListClustersError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListClustersError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListClustersError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListClustersError {}
/// Errors returned by ListConfigurationRevisions
#[derive(Debug, PartialEq)]
pub enum ListConfigurationRevisionsError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl ListConfigurationRevisionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListConfigurationRevisionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListConfigurationRevisionsError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListConfigurationRevisionsError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        ListConfigurationRevisionsError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListConfigurationRevisionsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListConfigurationRevisionsError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListConfigurationRevisionsError::Unauthorized(
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
impl fmt::Display for ListConfigurationRevisionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListConfigurationRevisionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListConfigurationRevisionsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListConfigurationRevisionsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListConfigurationRevisionsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListConfigurationRevisionsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            ListConfigurationRevisionsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListConfigurationRevisionsError {}
/// Errors returned by ListConfigurations
#[derive(Debug, PartialEq)]
pub enum ListConfigurationsError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl ListConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConfigurationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListConfigurationsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListConfigurationsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListConfigurationsError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListConfigurationsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListConfigurationsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListConfigurationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListConfigurationsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListConfigurationsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListConfigurationsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListConfigurationsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListConfigurationsError {}
/// Errors returned by ListKafkaVersions
#[derive(Debug, PartialEq)]
pub enum ListKafkaVersionsError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl ListKafkaVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListKafkaVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListKafkaVersionsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListKafkaVersionsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListKafkaVersionsError::InternalServerError(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListKafkaVersionsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListKafkaVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListKafkaVersionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListKafkaVersionsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListKafkaVersionsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListKafkaVersionsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListKafkaVersionsError {}
/// Errors returned by ListNodes
#[derive(Debug, PartialEq)]
pub enum ListNodesError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
}

impl ListNodesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListNodesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListNodesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListNodesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListNodesError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListNodesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListNodesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListNodesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListNodesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListNodesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListNodesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListNodesError {}
/// Errors returned by ListScramSecrets
#[derive(Debug, PartialEq)]
pub enum ListScramSecretsError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    TooManyRequests(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl ListScramSecretsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListScramSecretsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListScramSecretsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListScramSecretsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListScramSecretsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListScramSecretsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListScramSecretsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListScramSecretsError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListScramSecretsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListScramSecretsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListScramSecretsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListScramSecretsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListScramSecretsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListScramSecretsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListScramSecretsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListScramSecretsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListScramSecretsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListScramSecretsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
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
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
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
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by RebootBroker
#[derive(Debug, PartialEq)]
pub enum RebootBrokerError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    TooManyRequests(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl RebootBrokerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootBrokerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RebootBrokerError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RebootBrokerError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(RebootBrokerError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RebootBrokerError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RebootBrokerError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RebootBrokerError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(RebootBrokerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RebootBrokerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RebootBrokerError::BadRequest(ref cause) => write!(f, "{}", cause),
            RebootBrokerError::Forbidden(ref cause) => write!(f, "{}", cause),
            RebootBrokerError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RebootBrokerError::NotFound(ref cause) => write!(f, "{}", cause),
            RebootBrokerError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RebootBrokerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            RebootBrokerError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RebootBrokerError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
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
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
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
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
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
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
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
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateBrokerCount
#[derive(Debug, PartialEq)]
pub enum UpdateBrokerCountError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl UpdateBrokerCountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBrokerCountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateBrokerCountError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateBrokerCountError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateBrokerCountError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateBrokerCountError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateBrokerCountError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateBrokerCountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBrokerCountError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateBrokerCountError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateBrokerCountError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateBrokerCountError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateBrokerCountError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateBrokerCountError {}
/// Errors returned by UpdateBrokerStorage
#[derive(Debug, PartialEq)]
pub enum UpdateBrokerStorageError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl UpdateBrokerStorageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBrokerStorageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateBrokerStorageError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateBrokerStorageError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateBrokerStorageError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateBrokerStorageError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateBrokerStorageError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateBrokerStorageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBrokerStorageError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateBrokerStorageError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateBrokerStorageError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateBrokerStorageError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateBrokerStorageError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateBrokerStorageError {}
/// Errors returned by UpdateBrokerType
#[derive(Debug, PartialEq)]
pub enum UpdateBrokerTypeError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    TooManyRequests(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl UpdateBrokerTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBrokerTypeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateBrokerTypeError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateBrokerTypeError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateBrokerTypeError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateBrokerTypeError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateBrokerTypeError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateBrokerTypeError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateBrokerTypeError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateBrokerTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBrokerTypeError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateBrokerTypeError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateBrokerTypeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateBrokerTypeError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateBrokerTypeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateBrokerTypeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateBrokerTypeError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateBrokerTypeError {}
/// Errors returned by UpdateClusterConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateClusterConfigurationError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl UpdateClusterConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateClusterConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateClusterConfigurationError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateClusterConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateClusterConfigurationError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateClusterConfigurationError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        UpdateClusterConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateClusterConfigurationError::Unauthorized(
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
impl fmt::Display for UpdateClusterConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateClusterConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateClusterConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateClusterConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateClusterConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateClusterConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateClusterConfigurationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateClusterConfigurationError {}
/// Errors returned by UpdateClusterKafkaVersion
#[derive(Debug, PartialEq)]
pub enum UpdateClusterKafkaVersionError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    TooManyRequests(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl UpdateClusterKafkaVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateClusterKafkaVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateClusterKafkaVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateClusterKafkaVersionError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateClusterKafkaVersionError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateClusterKafkaVersionError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        UpdateClusterKafkaVersionError::ServiceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateClusterKafkaVersionError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateClusterKafkaVersionError::Unauthorized(
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
impl fmt::Display for UpdateClusterKafkaVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateClusterKafkaVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateClusterKafkaVersionError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateClusterKafkaVersionError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateClusterKafkaVersionError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateClusterKafkaVersionError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateClusterKafkaVersionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateClusterKafkaVersionError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateClusterKafkaVersionError {}
/// Errors returned by UpdateConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    NotFound(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl UpdateConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateConfigurationError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateConfigurationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateConfigurationError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateConfigurationError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateConfigurationError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateConfigurationError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateConfigurationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateConfigurationError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateConfigurationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateConfigurationError {}
/// Errors returned by UpdateMonitoring
#[derive(Debug, PartialEq)]
pub enum UpdateMonitoringError {
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    BadRequest(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Forbidden(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    InternalServerError(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    ServiceUnavailable(String),
    /// <pre><code>        &lt;p&gt;Returns information about an error.&lt;/p&gt;
    /// </code></pre>
    Unauthorized(String),
}

impl UpdateMonitoringError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMonitoringError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateMonitoringError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateMonitoringError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateMonitoringError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateMonitoringError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateMonitoringError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateMonitoringError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMonitoringError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateMonitoringError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateMonitoringError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateMonitoringError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateMonitoringError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateMonitoringError {}
/// Trait representing the capabilities of the Kafka API. Kafka clients implement this trait.
#[async_trait]
pub trait Kafka {
    /// <pre><code>        &lt;p&gt;Associates one or more Scram Secrets with an Amazon MSK cluster.&lt;/p&gt;
    /// </code></pre>
    async fn batch_associate_scram_secret(
        &self,
        input: BatchAssociateScramSecretRequest,
    ) -> Result<BatchAssociateScramSecretResponse, RusotoError<BatchAssociateScramSecretError>>;

    /// <pre><code>        &lt;p&gt;Disassociates one or more Scram Secrets from an Amazon MSK cluster.&lt;/p&gt;
    /// </code></pre>
    async fn batch_disassociate_scram_secret(
        &self,
        input: BatchDisassociateScramSecretRequest,
    ) -> Result<BatchDisassociateScramSecretResponse, RusotoError<BatchDisassociateScramSecretError>>;

    /// <pre><code>        &lt;p&gt;Creates a new MSK cluster.&lt;/p&gt;
    /// </code></pre>
    async fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> Result<CreateClusterResponse, RusotoError<CreateClusterError>>;

    /// <pre><code>        &lt;p&gt;Creates a new MSK configuration.&lt;/p&gt;
    /// </code></pre>
    async fn create_configuration(
        &self,
        input: CreateConfigurationRequest,
    ) -> Result<CreateConfigurationResponse, RusotoError<CreateConfigurationError>>;

    /// <pre><code>        &lt;p&gt;Deletes the MSK cluster specified by the Amazon Resource Name (ARN) in the request.&lt;/p&gt;
    /// </code></pre>
    async fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse, RusotoError<DeleteClusterError>>;

    /// <pre><code>        &lt;p&gt;Deletes an MSK Configuration.&lt;/p&gt;
    /// </code></pre>
    async fn delete_configuration(
        &self,
        input: DeleteConfigurationRequest,
    ) -> Result<DeleteConfigurationResponse, RusotoError<DeleteConfigurationError>>;

    /// <pre><code>        &lt;p&gt;Returns a description of the MSK cluster whose Amazon Resource Name (ARN) is specified in the request.&lt;/p&gt;
    /// </code></pre>
    async fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> Result<DescribeClusterResponse, RusotoError<DescribeClusterError>>;

    /// <pre><code>        &lt;p&gt;Returns a description of the cluster operation specified by the ARN.&lt;/p&gt;
    /// </code></pre>
    async fn describe_cluster_operation(
        &self,
        input: DescribeClusterOperationRequest,
    ) -> Result<DescribeClusterOperationResponse, RusotoError<DescribeClusterOperationError>>;

    /// <pre><code>        &lt;p&gt;Returns a description of this MSK configuration.&lt;/p&gt;
    /// </code></pre>
    async fn describe_configuration(
        &self,
        input: DescribeConfigurationRequest,
    ) -> Result<DescribeConfigurationResponse, RusotoError<DescribeConfigurationError>>;

    /// <pre><code>        &lt;p&gt;Returns a description of this revision of the configuration.&lt;/p&gt;
    /// </code></pre>
    async fn describe_configuration_revision(
        &self,
        input: DescribeConfigurationRevisionRequest,
    ) -> Result<
        DescribeConfigurationRevisionResponse,
        RusotoError<DescribeConfigurationRevisionError>,
    >;

    /// <pre><code>        &lt;p&gt;A list of brokers that a client application can use to bootstrap.&lt;/p&gt;
    /// </code></pre>
    async fn get_bootstrap_brokers(
        &self,
        input: GetBootstrapBrokersRequest,
    ) -> Result<GetBootstrapBrokersResponse, RusotoError<GetBootstrapBrokersError>>;

    /// <pre><code>        &lt;p&gt;Gets the Apache Kafka versions to which you can update the MSK cluster.&lt;/p&gt;
    /// </code></pre>
    async fn get_compatible_kafka_versions(
        &self,
        input: GetCompatibleKafkaVersionsRequest,
    ) -> Result<GetCompatibleKafkaVersionsResponse, RusotoError<GetCompatibleKafkaVersionsError>>;

    /// <pre><code>        &lt;p&gt;Returns a list of all the operations that have been performed on the specified MSK cluster.&lt;/p&gt;
    /// </code></pre>
    async fn list_cluster_operations(
        &self,
        input: ListClusterOperationsRequest,
    ) -> Result<ListClusterOperationsResponse, RusotoError<ListClusterOperationsError>>;

    /// <pre><code>        &lt;p&gt;Returns a list of all the MSK clusters in the current Region.&lt;/p&gt;
    /// </code></pre>
    async fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> Result<ListClustersResponse, RusotoError<ListClustersError>>;

    /// <pre><code>        &lt;p&gt;Returns a list of all the MSK configurations in this Region.&lt;/p&gt;
    /// </code></pre>
    async fn list_configuration_revisions(
        &self,
        input: ListConfigurationRevisionsRequest,
    ) -> Result<ListConfigurationRevisionsResponse, RusotoError<ListConfigurationRevisionsError>>;

    /// <pre><code>        &lt;p&gt;Returns a list of all the MSK configurations in this Region.&lt;/p&gt;
    /// </code></pre>
    async fn list_configurations(
        &self,
        input: ListConfigurationsRequest,
    ) -> Result<ListConfigurationsResponse, RusotoError<ListConfigurationsError>>;

    /// <pre><code>        &lt;p&gt;Returns a list of Kafka versions.&lt;/p&gt;
    /// </code></pre>
    async fn list_kafka_versions(
        &self,
        input: ListKafkaVersionsRequest,
    ) -> Result<ListKafkaVersionsResponse, RusotoError<ListKafkaVersionsError>>;

    /// <pre><code>        &lt;p&gt;Returns a list of the broker nodes in the cluster.&lt;/p&gt;
    /// </code></pre>
    async fn list_nodes(
        &self,
        input: ListNodesRequest,
    ) -> Result<ListNodesResponse, RusotoError<ListNodesError>>;

    /// <pre><code>        &lt;p&gt;Returns a list of the Scram Secrets associated with an Amazon MSK cluster.&lt;/p&gt;
    /// </code></pre>
    async fn list_scram_secrets(
        &self,
        input: ListScramSecretsRequest,
    ) -> Result<ListScramSecretsResponse, RusotoError<ListScramSecretsError>>;

    /// <pre><code>        &lt;p&gt;Returns a list of the tags associated with the specified resource.&lt;/p&gt;
    /// </code></pre>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Reboots brokers.</p>
    async fn reboot_broker(
        &self,
        input: RebootBrokerRequest,
    ) -> Result<RebootBrokerResponse, RusotoError<RebootBrokerError>>;

    /// <pre><code>        &lt;p&gt;Adds tags to the specified MSK resource.&lt;/p&gt;
    /// </code></pre>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <pre><code>        &lt;p&gt;Removes the tags associated with the keys that are provided in the query.&lt;/p&gt;
    /// </code></pre>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <pre><code>        &lt;p&gt;Updates the number of broker nodes in the cluster.&lt;/p&gt;
    /// </code></pre>
    async fn update_broker_count(
        &self,
        input: UpdateBrokerCountRequest,
    ) -> Result<UpdateBrokerCountResponse, RusotoError<UpdateBrokerCountError>>;

    /// <pre><code>        &lt;p&gt;Updates the EBS storage associated with MSK brokers.&lt;/p&gt;
    /// </code></pre>
    async fn update_broker_storage(
        &self,
        input: UpdateBrokerStorageRequest,
    ) -> Result<UpdateBrokerStorageResponse, RusotoError<UpdateBrokerStorageError>>;

    /// <pre><code>        &lt;p&gt;Updates EC2 instance type.&lt;/p&gt;
    /// </code></pre>
    async fn update_broker_type(
        &self,
        input: UpdateBrokerTypeRequest,
    ) -> Result<UpdateBrokerTypeResponse, RusotoError<UpdateBrokerTypeError>>;

    /// <pre><code>        &lt;p&gt;Updates the cluster with the configuration that is specified in the request body.&lt;/p&gt;
    /// </code></pre>
    async fn update_cluster_configuration(
        &self,
        input: UpdateClusterConfigurationRequest,
    ) -> Result<UpdateClusterConfigurationResponse, RusotoError<UpdateClusterConfigurationError>>;

    /// <pre><code>        &lt;p&gt;Updates the Apache Kafka version for the cluster.&lt;/p&gt;
    /// </code></pre>
    async fn update_cluster_kafka_version(
        &self,
        input: UpdateClusterKafkaVersionRequest,
    ) -> Result<UpdateClusterKafkaVersionResponse, RusotoError<UpdateClusterKafkaVersionError>>;

    /// <pre><code>        &lt;p&gt;Updates an MSK configuration.&lt;/p&gt;
    /// </code></pre>
    async fn update_configuration(
        &self,
        input: UpdateConfigurationRequest,
    ) -> Result<UpdateConfigurationResponse, RusotoError<UpdateConfigurationError>>;

    /// <pre><code>        &lt;p&gt;Updates the monitoring settings for the cluster. You can use this operation to specify which Apache Kafka metrics you want Amazon MSK to send to Amazon CloudWatch. You can also specify settings for open monitoring with Prometheus.&lt;/p&gt;
    /// </code></pre>
    async fn update_monitoring(
        &self,
        input: UpdateMonitoringRequest,
    ) -> Result<UpdateMonitoringResponse, RusotoError<UpdateMonitoringError>>;
}
/// A client for the Kafka API.
#[derive(Clone)]
pub struct KafkaClient {
    client: Client,
    region: region::Region,
}

impl KafkaClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KafkaClient {
        KafkaClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KafkaClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        KafkaClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> KafkaClient {
        KafkaClient { client, region }
    }
}

#[async_trait]
impl Kafka for KafkaClient {
    /// <pre><code>        &lt;p&gt;Associates one or more Scram Secrets with an Amazon MSK cluster.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn batch_associate_scram_secret(
        &self,
        input: BatchAssociateScramSecretRequest,
    ) -> Result<BatchAssociateScramSecretResponse, RusotoError<BatchAssociateScramSecretError>>
    {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/scram-secrets",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("POST", "kafka", &self.region, &request_uri);
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
                .deserialize::<BatchAssociateScramSecretResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchAssociateScramSecretError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Disassociates one or more Scram Secrets from an Amazon MSK cluster.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn batch_disassociate_scram_secret(
        &self,
        input: BatchDisassociateScramSecretRequest,
    ) -> Result<BatchDisassociateScramSecretResponse, RusotoError<BatchDisassociateScramSecretError>>
    {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/scram-secrets",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("PATCH", "kafka", &self.region, &request_uri);
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
                .deserialize::<BatchDisassociateScramSecretResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDisassociateScramSecretError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Creates a new MSK cluster.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> Result<CreateClusterResponse, RusotoError<CreateClusterError>> {
        let request_uri = "/v1/clusters";

        let mut request = SignedRequest::new("POST", "kafka", &self.region, &request_uri);
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
                .deserialize::<CreateClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateClusterError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Creates a new MSK configuration.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn create_configuration(
        &self,
        input: CreateConfigurationRequest,
    ) -> Result<CreateConfigurationResponse, RusotoError<CreateConfigurationError>> {
        let request_uri = "/v1/configurations";

        let mut request = SignedRequest::new("POST", "kafka", &self.region, &request_uri);
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
                .deserialize::<CreateConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateConfigurationError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Deletes the MSK cluster specified by the Amazon Resource Name (ARN) in the request.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse, RusotoError<DeleteClusterError>> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("DELETE", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.current_version {
            params.put("currentVersion", x);
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
                .deserialize::<DeleteClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteClusterError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Deletes an MSK Configuration.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn delete_configuration(
        &self,
        input: DeleteConfigurationRequest,
    ) -> Result<DeleteConfigurationResponse, RusotoError<DeleteConfigurationError>> {
        let request_uri = format!("/v1/configurations/{arn}", arn = input.arn);

        let mut request = SignedRequest::new("DELETE", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteConfigurationError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a description of the MSK cluster whose Amazon Resource Name (ARN) is specified in the request.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> Result<DescribeClusterResponse, RusotoError<DescribeClusterError>> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeClusterError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a description of the cluster operation specified by the ARN.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn describe_cluster_operation(
        &self,
        input: DescribeClusterOperationRequest,
    ) -> Result<DescribeClusterOperationResponse, RusotoError<DescribeClusterOperationError>> {
        let request_uri = format!(
            "/v1/operations/{cluster_operation_arn}",
            cluster_operation_arn = input.cluster_operation_arn
        );

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeClusterOperationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeClusterOperationError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a description of this MSK configuration.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn describe_configuration(
        &self,
        input: DescribeConfigurationRequest,
    ) -> Result<DescribeConfigurationResponse, RusotoError<DescribeConfigurationError>> {
        let request_uri = format!("/v1/configurations/{arn}", arn = input.arn);

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConfigurationError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a description of this revision of the configuration.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn describe_configuration_revision(
        &self,
        input: DescribeConfigurationRevisionRequest,
    ) -> Result<
        DescribeConfigurationRevisionResponse,
        RusotoError<DescribeConfigurationRevisionError>,
    > {
        let request_uri = format!(
            "/v1/configurations/{arn}/revisions/{revision}",
            arn = input.arn,
            revision = input.revision
        );

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeConfigurationRevisionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConfigurationRevisionError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;A list of brokers that a client application can use to bootstrap.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn get_bootstrap_brokers(
        &self,
        input: GetBootstrapBrokersRequest,
    ) -> Result<GetBootstrapBrokersResponse, RusotoError<GetBootstrapBrokersError>> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/bootstrap-brokers",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBootstrapBrokersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBootstrapBrokersError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Gets the Apache Kafka versions to which you can update the MSK cluster.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn get_compatible_kafka_versions(
        &self,
        input: GetCompatibleKafkaVersionsRequest,
    ) -> Result<GetCompatibleKafkaVersionsResponse, RusotoError<GetCompatibleKafkaVersionsError>>
    {
        let request_uri = "/v1/compatible-kafka-versions";

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.cluster_arn {
            params.put("clusterArn", x);
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
                .deserialize::<GetCompatibleKafkaVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCompatibleKafkaVersionsError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a list of all the operations that have been performed on the specified MSK cluster.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn list_cluster_operations(
        &self,
        input: ListClusterOperationsRequest,
    ) -> Result<ListClusterOperationsResponse, RusotoError<ListClusterOperationsError>> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/operations",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
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
                .deserialize::<ListClusterOperationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListClusterOperationsError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a list of all the MSK clusters in the current Region.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> Result<ListClustersResponse, RusotoError<ListClustersError>> {
        let request_uri = "/v1/clusters";

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.cluster_name_filter {
            params.put("clusterNameFilter", x);
        }
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
                .deserialize::<ListClustersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListClustersError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a list of all the MSK configurations in this Region.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn list_configuration_revisions(
        &self,
        input: ListConfigurationRevisionsRequest,
    ) -> Result<ListConfigurationRevisionsResponse, RusotoError<ListConfigurationRevisionsError>>
    {
        let request_uri = format!("/v1/configurations/{arn}/revisions", arn = input.arn);

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
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
                .deserialize::<ListConfigurationRevisionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListConfigurationRevisionsError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a list of all the MSK configurations in this Region.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn list_configurations(
        &self,
        input: ListConfigurationsRequest,
    ) -> Result<ListConfigurationsResponse, RusotoError<ListConfigurationsError>> {
        let request_uri = "/v1/configurations";

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
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
                .deserialize::<ListConfigurationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListConfigurationsError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a list of Kafka versions.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn list_kafka_versions(
        &self,
        input: ListKafkaVersionsRequest,
    ) -> Result<ListKafkaVersionsResponse, RusotoError<ListKafkaVersionsError>> {
        let request_uri = "/v1/kafka-versions";

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
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
                .deserialize::<ListKafkaVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListKafkaVersionsError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a list of the broker nodes in the cluster.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn list_nodes(
        &self,
        input: ListNodesRequest,
    ) -> Result<ListNodesResponse, RusotoError<ListNodesError>> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/nodes",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
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
                .deserialize::<ListNodesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListNodesError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a list of the Scram Secrets associated with an Amazon MSK cluster.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn list_scram_secrets(
        &self,
        input: ListScramSecretsRequest,
    ) -> Result<ListScramSecretsResponse, RusotoError<ListScramSecretsError>> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/scram-secrets",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
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
                .deserialize::<ListScramSecretsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListScramSecretsError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Returns a list of the tags associated with the specified resource.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
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

    /// <p>Reboots brokers.</p>
    #[allow(unused_mut)]
    async fn reboot_broker(
        &self,
        input: RebootBrokerRequest,
    ) -> Result<RebootBrokerResponse, RusotoError<RebootBrokerError>> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/reboot-broker",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("PUT", "kafka", &self.region, &request_uri);
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
                .deserialize::<RebootBrokerResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RebootBrokerError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Adds tags to the specified MSK resource.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "kafka", &self.region, &request_uri);
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
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Removes the tags associated with the keys that are provided in the query.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "kafka", &self.region, &request_uri);
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
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Updates the number of broker nodes in the cluster.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn update_broker_count(
        &self,
        input: UpdateBrokerCountRequest,
    ) -> Result<UpdateBrokerCountResponse, RusotoError<UpdateBrokerCountError>> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/nodes/count",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("PUT", "kafka", &self.region, &request_uri);
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
                .deserialize::<UpdateBrokerCountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateBrokerCountError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Updates the EBS storage associated with MSK brokers.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn update_broker_storage(
        &self,
        input: UpdateBrokerStorageRequest,
    ) -> Result<UpdateBrokerStorageResponse, RusotoError<UpdateBrokerStorageError>> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/nodes/storage",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("PUT", "kafka", &self.region, &request_uri);
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
                .deserialize::<UpdateBrokerStorageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateBrokerStorageError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Updates EC2 instance type.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn update_broker_type(
        &self,
        input: UpdateBrokerTypeRequest,
    ) -> Result<UpdateBrokerTypeResponse, RusotoError<UpdateBrokerTypeError>> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/nodes/type",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("PUT", "kafka", &self.region, &request_uri);
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
                .deserialize::<UpdateBrokerTypeResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateBrokerTypeError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Updates the cluster with the configuration that is specified in the request body.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn update_cluster_configuration(
        &self,
        input: UpdateClusterConfigurationRequest,
    ) -> Result<UpdateClusterConfigurationResponse, RusotoError<UpdateClusterConfigurationError>>
    {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/configuration",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("PUT", "kafka", &self.region, &request_uri);
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
                .deserialize::<UpdateClusterConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateClusterConfigurationError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Updates the Apache Kafka version for the cluster.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn update_cluster_kafka_version(
        &self,
        input: UpdateClusterKafkaVersionRequest,
    ) -> Result<UpdateClusterKafkaVersionResponse, RusotoError<UpdateClusterKafkaVersionError>>
    {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/version",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("PUT", "kafka", &self.region, &request_uri);
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
                .deserialize::<UpdateClusterKafkaVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateClusterKafkaVersionError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Updates an MSK configuration.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn update_configuration(
        &self,
        input: UpdateConfigurationRequest,
    ) -> Result<UpdateConfigurationResponse, RusotoError<UpdateConfigurationError>> {
        let request_uri = format!("/v1/configurations/{arn}", arn = input.arn);

        let mut request = SignedRequest::new("PUT", "kafka", &self.region, &request_uri);
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
                .deserialize::<UpdateConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateConfigurationError::from_response(response))
        }
    }

    /// <pre><code>        &lt;p&gt;Updates the monitoring settings for the cluster. You can use this operation to specify which Apache Kafka metrics you want Amazon MSK to send to Amazon CloudWatch. You can also specify settings for open monitoring with Prometheus.&lt;/p&gt;
    /// </code></pre>
    #[allow(unused_mut)]
    async fn update_monitoring(
        &self,
        input: UpdateMonitoringRequest,
    ) -> Result<UpdateMonitoringResponse, RusotoError<UpdateMonitoringError>> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/monitoring",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("PUT", "kafka", &self.region, &request_uri);
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
                .deserialize::<UpdateMonitoringResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateMonitoringError::from_response(response))
        }
    }
}
