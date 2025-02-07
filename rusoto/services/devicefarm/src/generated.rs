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

impl DeviceFarmClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "devicefarm", &self.region, request_uri);

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
/// <p>A container for account-level settings in AWS Device Farm.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccountSettings {
    /// <p>The AWS account number specified in the <code>AccountSettings</code> container.</p>
    #[serde(rename = "awsAccountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_number: Option<String>,
    /// <p>The default number of minutes (at the account level) a test run executes before it times out. The default value is 150 minutes.</p>
    #[serde(rename = "defaultJobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_job_timeout_minutes: Option<i64>,
    /// <p>The maximum number of minutes a test run executes before it times out.</p>
    #[serde(rename = "maxJobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_job_timeout_minutes: Option<i64>,
    /// <p>The maximum number of device slots that the AWS account can purchase. Each maximum is expressed as an <code>offering-id:number</code> pair, where the <code>offering-id</code> represents one of the IDs returned by the <code>ListOfferings</code> command.</p>
    #[serde(rename = "maxSlots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_slots: Option<::std::collections::HashMap<String, i64>>,
    /// <p>When set to <code>true</code>, for private devices, Device Farm does not sign your app again. For public devices, Device Farm always signs your apps again.</p> <p>For more information about how Device Farm re-signs your apps, see <a href="https://aws.amazon.com/device-farm/faq/">Do you modify my app?</a> in the <i>AWS Device Farm FAQs</i>.</p>
    #[serde(rename = "skipAppResign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_app_resign: Option<bool>,
    /// <p>Information about an AWS account's usage of free trial device minutes.</p>
    #[serde(rename = "trialMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_minutes: Option<TrialMinutes>,
    /// <p>Returns the unmetered devices you have purchased or want to purchase.</p>
    #[serde(rename = "unmeteredDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmetered_devices: Option<::std::collections::HashMap<String, i64>>,
    /// <p>Returns the unmetered remote access devices you have purchased or want to purchase.</p>
    #[serde(rename = "unmeteredRemoteAccessDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmetered_remote_access_devices: Option<::std::collections::HashMap<String, i64>>,
}

/// <p>Represents the output of a test. Examples of artifacts include logs and screenshots.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Artifact {
    /// <p>The artifact's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The artifact's file extension.</p>
    #[serde(rename = "extension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    /// <p>The artifact's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The artifact&#39;s type.</p> <p>Allowed values include the following:</p> <ul> <li> <p>UNKNOWN</p> </li> <li> <p>SCREENSHOT</p> </li> <li> <p>DEVICE<em>LOG</p> </li> <li> <p>MESSAGE</em>LOG</p> </li> <li> <p>VIDEO<em>LOG</p> </li> <li> <p>RESULT</em>LOG</p> </li> <li> <p>SERVICE<em>LOG</p> </li> <li> <p>WEBKIT</em>LOG</p> </li> <li> <p>INSTRUMENTATION<em>OUTPUT</p> </li> <li> <p>EXERCISER</em>MONKEY<em>OUTPUT: the artifact (log) generated by an Android fuzz test.</p> </li> <li> <p>CALABASH</em>JSON<em>OUTPUT</p> </li> <li> <p>CALABASH</em>PRETTY<em>OUTPUT</p> </li> <li> <p>CALABASH</em>STANDARD<em>OUTPUT</p> </li> <li> <p>CALABASH</em>JAVA<em>XML</em>OUTPUT</p> </li> <li> <p>AUTOMATION<em>OUTPUT</p> </li> <li> <p>APPIUM</em>SERVER<em>OUTPUT</p> </li> <li> <p>APPIUM</em>JAVA<em>OUTPUT</p> </li> <li> <p>APPIUM</em>JAVA<em>XML</em>OUTPUT</p> </li> <li> <p>APPIUM<em>PYTHON</em>OUTPUT</p> </li> <li> <p>APPIUM<em>PYTHON</em>XML<em>OUTPUT</p> </li> <li> <p>EXPLORER</em>EVENT<em>LOG</p> </li> <li> <p>EXPLORER</em>SUMMARY<em>LOG</p> </li> <li> <p>APPLICATION</em>CRASH<em>REPORT</p> </li> <li> <p>XCTEST</em>LOG</p> </li> <li> <p>VIDEO</p> </li> <li> <p>CUSTOMER<em>ARTIFACT</p> </li> <li> <p>CUSTOMER</em>ARTIFACT<em>LOG</p> </li> <li> <p>TESTSPEC</em>OUTPUT</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The presigned Amazon S3 URL that can be used with a GET request to download the artifact's file.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Represents the amount of CPU that an app is using on a physical device. Does not represent system-wide CPU usage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CPU {
    /// <p>The CPU's architecture (for example, x86 or ARM).</p>
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// <p>The clock speed of the device's CPU, expressed in hertz (Hz). For example, a 1.2 GHz CPU is expressed as 1200000000.</p>
    #[serde(rename = "clock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock: Option<f64>,
    /// <p>The CPU's frequency.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}

/// <p>Represents entity counters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Counters {
    /// <p>The number of errored entities.</p>
    #[serde(rename = "errored")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errored: Option<i64>,
    /// <p>The number of failed entities.</p>
    #[serde(rename = "failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    /// <p>The number of passed entities.</p>
    #[serde(rename = "passed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<i64>,
    /// <p>The number of skipped entities.</p>
    #[serde(rename = "skipped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i64>,
    /// <p>The number of stopped entities.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<i64>,
    /// <p>The total number of entities.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// <p>The number of warned entities.</p>
    #[serde(rename = "warned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warned: Option<i64>,
}

/// <p>Represents a request to the create device pool operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDevicePoolRequest {
    /// <p>The device pool's description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The number of devices that Device Farm can add to your device pool. Device Farm adds devices that are available and meet the criteria that you assign for the <code>rules</code> parameter. Depending on how many devices meet these constraints, your device pool might contain fewer devices than the value for this parameter.</p> <p>By specifying the maximum number of devices, you can control the costs that you incur by running tests.</p>
    #[serde(rename = "maxDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_devices: Option<i64>,
    /// <p>The device pool's name.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARN of the project for the device pool.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>The device pool's rules.</p>
    #[serde(rename = "rules")]
    pub rules: Vec<Rule>,
}

/// <p>Represents the result of a create device pool request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDevicePoolResult {
    /// <p>The newly created device pool.</p>
    #[serde(rename = "devicePool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pool: Option<DevicePool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInstanceProfileRequest {
    /// <p>The description of your instance profile.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An array of strings that specifies the list of app packages that should not be cleaned up from the device after a test run.</p> <p>The list of packages is considered only if you set <code>packageCleanup</code> to <code>true</code>.</p>
    #[serde(rename = "excludeAppPackagesFromCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_app_packages_from_cleanup: Option<Vec<String>>,
    /// <p>The name of your instance profile.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>When set to <code>true</code>, Device Farm removes app packages after a test run. The default value is <code>false</code> for private devices.</p>
    #[serde(rename = "packageCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_cleanup: Option<bool>,
    /// <p>When set to <code>true</code>, Device Farm reboots the instance after a test run. The default value is <code>true</code>.</p>
    #[serde(rename = "rebootAfterUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_after_use: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInstanceProfileResult {
    /// <p>An object that contains information about your instance profile.</p>
    #[serde(rename = "instanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateNetworkProfileRequest {
    /// <p>The description of the network profile.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "downlinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_jitter_ms: Option<i64>,
    /// <p>Proportion of received packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "downlinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_loss_percent: Option<i64>,
    /// <p>The name for the new network profile.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to create a network profile.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>The type of network profile to create. Valid values are listed here.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "uplinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_jitter_ms: Option<i64>,
    /// <p>Proportion of transmitted packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "uplinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_loss_percent: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNetworkProfileResult {
    /// <p>The network profile that is returned by the create network profile request.</p>
    #[serde(rename = "networkProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
}

/// <p>Represents a request to the create project operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProjectRequest {
    /// <p>Sets the execution timeout value (in minutes) for a project. All test runs in this project use the specified execution timeout value unless overridden when scheduling a run.</p>
    #[serde(rename = "defaultJobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_job_timeout_minutes: Option<i64>,
    /// <p>The project's name.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Represents the result of a create project request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProjectResult {
    /// <p>The newly created project.</p>
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

/// <p>Configuration settings for a remote access session, including billing method.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRemoteAccessSessionConfiguration {
    /// <p>The billing method for the remote access session.</p>
    #[serde(rename = "billingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_method: Option<String>,
    /// <p>An array of ARNs included in the VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfigurationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_arns: Option<Vec<String>>,
}

/// <p>Creates and submits a request to start a remote access session.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRemoteAccessSessionRequest {
    /// <p>Unique identifier for the client. If you want access to multiple devices on the same client, you should pass the same <code>clientId</code> value in each call to <code>CreateRemoteAccessSession</code>. This identifier is required only if <code>remoteDebugEnabled</code> is set to <code>true</code>.</p> <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>.</p>
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The configuration information for the remote access session request.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<CreateRemoteAccessSessionConfiguration>,
    /// <p>The ARN of the device for which you want to create a remote access session.</p>
    #[serde(rename = "deviceArn")]
    pub device_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the device instance for which you want to create a remote access session.</p>
    #[serde(rename = "instanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    /// <p><p>The interaction mode of the remote access session. Valid values are:</p> <ul> <li> <p>INTERACTIVE: You can interact with the iOS device by viewing, touching, and rotating the screen. You cannot run XCUITest framework-based tests in this mode.</p> </li> <li> <p>NO<em>VIDEO: You are connected to the device, but cannot interact with it or view the screen. This mode has the fastest test execution speed. You can run XCUITest framework-based tests in this mode.</p> </li> <li> <p>VIDEO</em>ONLY: You can view the screen, but cannot touch or rotate it. You can run XCUITest framework-based tests and watch the screen in this mode.</p> </li> </ul></p>
    #[serde(rename = "interactionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_mode: Option<String>,
    /// <p>The name of the remote access session to create.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to create a remote access session.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>Set to <code>true</code> if you want to access devices remotely for debugging in your remote access session.</p> <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>.</p>
    #[serde(rename = "remoteDebugEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_debug_enabled: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) for the app to be recorded in the remote access session.</p>
    #[serde(rename = "remoteRecordAppArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_record_app_arn: Option<String>,
    /// <p>Set to <code>true</code> to enable remote recording for the remote access session.</p>
    #[serde(rename = "remoteRecordEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_record_enabled: Option<bool>,
    /// <p>When set to <code>true</code>, for private devices, Device Farm does not sign your app again. For public devices, Device Farm always signs your apps again.</p> <p>For more information on how Device Farm modifies your uploads during tests, see <a href="https://aws.amazon.com/device-farm/faq/">Do you modify my app?</a> </p>
    #[serde(rename = "skipAppResign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_app_resign: Option<bool>,
    /// <p>Ignored. The public key of the <code>ssh</code> key pair you want to use for connecting to remote devices in your remote debugging session. This key is required only if <code>remoteDebugEnabled</code> is set to <code>true</code>.</p> <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>.</p>
    #[serde(rename = "sshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
}

/// <p>Represents the server response from a request to create a remote access session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRemoteAccessSessionResult {
    /// <p>A container that describes the remote access session when the request to create a remote access session is sent.</p>
    #[serde(rename = "remoteAccessSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_session: Option<RemoteAccessSession>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTestGridProjectRequest {
    /// <p>Human-readable description of the project.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Human-readable name of the Selenium testing project.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The VPC security groups and subnets that are attached to a project.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<TestGridVpcConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTestGridProjectResult {
    /// <p>ARN of the Selenium testing project that was created.</p>
    #[serde(rename = "testGridProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_grid_project: Option<TestGridProject>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTestGridUrlRequest {
    /// <p>Lifetime, in seconds, of the URL.</p>
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: i64,
    /// <p>ARN (from <a>CreateTestGridProject</a> or <a>ListTestGridProjects</a>) to associate with the short-term URL. </p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTestGridUrlResult {
    /// <p>The number of seconds the URL from <a>CreateTestGridUrlResult$url</a> stays active.</p>
    #[serde(rename = "expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<f64>,
    /// <p>A signed URL, expiring in <a>CreateTestGridUrlRequest$expiresInSeconds</a> seconds, to be passed to a <code>RemoteWebDriver</code>. </p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Represents a request to the create upload operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUploadRequest {
    /// <p>The upload's content type (for example, <code>application/octet-stream</code>).</p>
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The upload's file name. The name should not contain any forward slashes (<code>/</code>). If you are uploading an iOS app, the file name must end with the <code>.ipa</code> extension. If you are uploading an Android app, the file name must end with the <code>.apk</code> extension. For all others, the file name must end with the <code>.zip</code> file extension.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARN of the project for the upload.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>The upload's upload type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>ANDROID_APP</p> </li> <li> <p>IOS_APP</p> </li> <li> <p>WEB_APP</p> </li> <li> <p>EXTERNAL_DATA</p> </li> <li> <p>APPIUM_JAVA_JUNIT_TEST_PACKAGE</p> </li> <li> <p>APPIUM_JAVA_TESTNG_TEST_PACKAGE</p> </li> <li> <p>APPIUM_PYTHON_TEST_PACKAGE</p> </li> <li> <p>APPIUM_NODE_TEST_PACKAGE</p> </li> <li> <p>APPIUM_RUBY_TEST_PACKAGE</p> </li> <li> <p>APPIUM_WEB_JAVA_JUNIT_TEST_PACKAGE</p> </li> <li> <p>APPIUM_WEB_JAVA_TESTNG_TEST_PACKAGE</p> </li> <li> <p>APPIUM_WEB_PYTHON_TEST_PACKAGE</p> </li> <li> <p>APPIUM_WEB_NODE_TEST_PACKAGE</p> </li> <li> <p>APPIUM_WEB_RUBY_TEST_PACKAGE</p> </li> <li> <p>CALABASH_TEST_PACKAGE</p> </li> <li> <p>INSTRUMENTATION_TEST_PACKAGE</p> </li> <li> <p>UIAUTOMATION_TEST_PACKAGE</p> </li> <li> <p>UIAUTOMATOR_TEST_PACKAGE</p> </li> <li> <p>XCTEST_TEST_PACKAGE</p> </li> <li> <p>XCTEST_UI_TEST_PACKAGE</p> </li> <li> <p>APPIUM_JAVA_JUNIT_TEST_SPEC</p> </li> <li> <p>APPIUM_JAVA_TESTNG_TEST_SPEC</p> </li> <li> <p>APPIUM_PYTHON_TEST_SPEC</p> </li> <li> <p>APPIUM_NODE_TEST_SPEC</p> </li> <li> <p>APPIUM_RUBY_TEST_SPEC</p> </li> <li> <p>APPIUM_WEB_JAVA_JUNIT_TEST_SPEC</p> </li> <li> <p>APPIUM_WEB_JAVA_TESTNG_TEST_SPEC</p> </li> <li> <p>APPIUM_WEB_PYTHON_TEST_SPEC</p> </li> <li> <p>APPIUM_WEB_NODE_TEST_SPEC</p> </li> <li> <p>APPIUM_WEB_RUBY_TEST_SPEC</p> </li> <li> <p>INSTRUMENTATION_TEST_SPEC</p> </li> <li> <p>XCTEST_UI_TEST_SPEC</p> </li> </ul> <p> If you call <code>CreateUpload</code> with <code>WEB_APP</code> specified, AWS Device Farm throws an <code>ArgumentException</code> error.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Represents the result of a create upload request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUploadResult {
    /// <p>The newly created upload.</p>
    #[serde(rename = "upload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Upload>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVPCEConfigurationRequest {
    /// <p>The DNS name of the service running in your VPC that you want Device Farm to test.</p>
    #[serde(rename = "serviceDnsName")]
    pub service_dns_name: String,
    /// <p>An optional description that provides details about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_description: Option<String>,
    /// <p>The friendly name you give to your VPC endpoint configuration, to manage your configurations more easily.</p>
    #[serde(rename = "vpceConfigurationName")]
    pub vpce_configuration_name: String,
    /// <p>The name of the VPC endpoint service running in your AWS account that you want Device Farm to test.</p>
    #[serde(rename = "vpceServiceName")]
    pub vpce_service_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVPCEConfigurationResult {
    /// <p>An object that contains information about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration: Option<VPCEConfiguration>,
}

/// <p>A JSON object that specifies the paths where the artifacts generated by the customer's tests, on the device or in the test environment, are pulled from.</p> <p>Specify <code>deviceHostPaths</code> and optionally specify either <code>iosPaths</code> or <code>androidPaths</code>.</p> <p>For web app tests, you can specify both <code>iosPaths</code> and <code>androidPaths</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CustomerArtifactPaths {
    /// <p>Comma-separated list of paths on the Android device where the artifacts generated by the customer's tests are pulled from.</p>
    #[serde(rename = "androidPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_paths: Option<Vec<String>>,
    /// <p>Comma-separated list of paths in the test execution environment where the artifacts generated by the customer's tests are pulled from.</p>
    #[serde(rename = "deviceHostPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_host_paths: Option<Vec<String>>,
    /// <p>Comma-separated list of paths on the iOS device where the artifacts generated by the customer's tests are pulled from.</p>
    #[serde(rename = "iosPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_paths: Option<Vec<String>>,
}

/// <p>Represents a request to the delete device pool operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDevicePoolRequest {
    /// <p>Represents the Amazon Resource Name (ARN) of the Device Farm device pool to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a delete device pool request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDevicePoolResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInstanceProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the instance profile you are requesting to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInstanceProfileResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNetworkProfileRequest {
    /// <p>The ARN of the network profile to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteNetworkProfileResult {}

/// <p>Represents a request to the delete project operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProjectRequest {
    /// <p>Represents the Amazon Resource Name (ARN) of the Device Farm project to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a delete project request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProjectResult {}

/// <p>Represents the request to delete the specified remote access session.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRemoteAccessSessionRequest {
    /// <p>The Amazon Resource Name (ARN) of the session for which you want to delete remote access.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>The response from the server when a request is made to delete the remote access session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRemoteAccessSessionResult {}

/// <p>Represents a request to the delete run operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRunRequest {
    /// <p>The Amazon Resource Name (ARN) for the run to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a delete run request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRunResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTestGridProjectRequest {
    /// <p>The ARN of the project to delete, from <a>CreateTestGridProject</a> or <a>ListTestGridProjects</a>.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTestGridProjectResult {}

/// <p>Represents a request to the delete upload operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUploadRequest {
    /// <p>Represents the Amazon Resource Name (ARN) of the Device Farm upload to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a delete upload request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUploadResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVPCEConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration you want to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVPCEConfigurationResult {}

/// <p>Represents a device type that an app is tested against.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Device {
    /// <p>The device's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Indicates how likely a device is available for a test run. Currently available in the <a>ListDevices</a> and GetDevice API methods.</p>
    #[serde(rename = "availability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,
    /// <p>The device's carrier.</p>
    #[serde(rename = "carrier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// <p>Information about the device's CPU.</p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<CPU>,
    /// <p>The name of the fleet to which this device belongs.</p>
    #[serde(rename = "fleetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_name: Option<String>,
    /// <p>The type of fleet to which this device belongs. Possible values are PRIVATE and PUBLIC.</p>
    #[serde(rename = "fleetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_type: Option<String>,
    /// <p><p>The device&#39;s form factor.</p> <p>Allowed values include:</p> <ul> <li> <p>PHONE</p> </li> <li> <p>TABLET</p> </li> </ul></p>
    #[serde(rename = "formFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_factor: Option<String>,
    /// <p>The device's heap size, expressed in bytes.</p>
    #[serde(rename = "heapSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heap_size: Option<i64>,
    /// <p>The device's image name.</p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p>The instances that belong to this device.</p>
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<DeviceInstance>>,
    /// <p>The device's manufacturer name.</p>
    #[serde(rename = "manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// <p>The device's total memory size, expressed in bytes.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The device's model name.</p>
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// <p>The device's model ID.</p>
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The device's display name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The device's operating system type.</p>
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// <p><p>The device&#39;s platform.</p> <p>Allowed values include:</p> <ul> <li> <p>ANDROID</p> </li> <li> <p>IOS</p> </li> </ul></p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The device's radio.</p>
    #[serde(rename = "radio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radio: Option<String>,
    /// <p>Specifies whether remote access has been enabled for the specified device.</p>
    #[serde(rename = "remoteAccessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_enabled: Option<bool>,
    /// <p>This flag is set to <code>true</code> if remote debugging is enabled for the device.</p> <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>.</p>
    #[serde(rename = "remoteDebugEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_debug_enabled: Option<bool>,
    /// <p>The resolution of the device.</p>
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
}

/// <p>Represents a device filter used to select a set of devices to be included in a test run. This data structure is passed in as the <code>deviceSelectionConfiguration</code> parameter to <code>ScheduleRun</code>. For an example of the JSON request syntax, see <a>ScheduleRun</a>.</p> <p>It is also passed in as the <code>filters</code> parameter to <code>ListDevices</code>. For an example of the JSON request syntax, see <a>ListDevices</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeviceFilter {
    /// <p><p>The aspect of a device such as platform or model used as the selection criteria in a device filter.</p> <p>The supported operators for each attribute are provided in the following list.</p> <dl> <dt>ARN</dt> <dd> <p>The Amazon Resource Name (ARN) of the device (for example, <code>arn:aws:devicefarm:us-west-2::device:12345Example</code>).</p> <p>Supported operators: <code>EQUALS</code>, <code>IN</code>, <code>NOT<em>IN</code> </p> </dd> <dt>PLATFORM</dt> <dd> <p>The device platform. Valid values are ANDROID or IOS.</p> <p>Supported operators: <code>EQUALS</code> </p> </dd> <dt>OS</em>VERSION</dt> <dd> <p>The operating system version (for example, 10.3.2).</p> <p>Supported operators: <code>EQUALS</code>, <code>GREATER<em>THAN</code>, <code>GREATER</em>THAN<em>OR</em>EQUALS</code>, <code>IN</code>, <code>LESS<em>THAN</code>, <code>LESS</em>THAN<em>OR</em>EQUALS</code>, <code>NOT<em>IN</code> </p> </dd> <dt>MODEL</dt> <dd> <p>The device model (for example, iPad 5th Gen).</p> <p>Supported operators: <code>CONTAINS</code>, <code>EQUALS</code>, <code>IN</code>, <code>NOT</em>IN</code> </p> </dd> <dt>AVAILABILITY</dt> <dd> <p>The current availability of the device. Valid values are AVAILABLE, HIGHLY<em>AVAILABLE, BUSY, or TEMPORARY</em>NOT<em>AVAILABLE.</p> <p>Supported operators: <code>EQUALS</code> </p> </dd> <dt>FORM</em>FACTOR</dt> <dd> <p>The device form factor. Valid values are PHONE or TABLET.</p> <p>Supported operators: <code>EQUALS</code> </p> </dd> <dt>MANUFACTURER</dt> <dd> <p>The device manufacturer (for example, Apple).</p> <p>Supported operators: <code>EQUALS</code>, <code>IN</code>, <code>NOT<em>IN</code> </p> </dd> <dt>REMOTE</em>ACCESS<em>ENABLED</dt> <dd> <p>Whether the device is enabled for remote access. Valid values are TRUE or FALSE.</p> <p>Supported operators: <code>EQUALS</code> </p> </dd> <dt>REMOTE</em>DEBUG<em>ENABLED</dt> <dd> <p>Whether the device is enabled for remote debugging. Valid values are TRUE or FALSE.</p> <p>Supported operators: <code>EQUALS</code> </p> <p>Because remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>, this filter is ignored.</p> </dd> <dt>INSTANCE</em>ARN</dt> <dd> <p>The Amazon Resource Name (ARN) of the device instance.</p> <p>Supported operators: <code>EQUALS</code>, <code>IN</code>, <code>NOT<em>IN</code> </p> </dd> <dt>INSTANCE</em>LABELS</dt> <dd> <p>The label of the device instance.</p> <p>Supported operators: <code>CONTAINS</code> </p> </dd> <dt>FLEET_TYPE</dt> <dd> <p>The fleet type. Valid values are PUBLIC or PRIVATE.</p> <p>Supported operators: <code>EQUALS</code> </p> </dd> </dl></p>
    #[serde(rename = "attribute")]
    pub attribute: String,
    /// <p>Specifies how Device Farm compares the filter's attribute to the value. See the attribute descriptions.</p>
    #[serde(rename = "operator")]
    pub operator: String,
    /// <p><p>An array of one or more filter values used in a device filter.</p> <p class="title"> <b>Operator Values</b> </p> <ul> <li> <p>The IN and NOT<em>IN operators can take a values array that has more than one element.</p> </li> <li> <p>The other operators require an array with a single element.</p> </li> </ul> <p class="title"> <b>Attribute Values</b> </p> <ul> <li> <p>The PLATFORM attribute can be set to ANDROID or IOS.</p> </li> <li> <p>The AVAILABILITY attribute can be set to AVAILABLE, HIGHLY</em>AVAILABLE, BUSY, or TEMPORARY<em>NOT</em>AVAILABLE.</p> </li> <li> <p>The FORM<em>FACTOR attribute can be set to PHONE or TABLET.</p> </li> <li> <p>The FLEET</em>TYPE attribute can be set to PUBLIC or PRIVATE.</p> </li> </ul></p>
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

/// <p>Represents the device instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeviceInstance {
    /// <p>The Amazon Resource Name (ARN) of the device instance.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ARN of the device.</p>
    #[serde(rename = "deviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    /// <p>A object that contains information about the instance profile.</p>
    #[serde(rename = "instanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
    /// <p>An array of strings that describe the device instance.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>The status of the device instance. Valid values are listed here.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Unique device identifier for the device instance.</p>
    #[serde(rename = "udid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udid: Option<String>,
}

/// <p>Represents the total (metered or unmetered) minutes used by the resource to run tests. Contains the sum of minutes consumed by all children.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeviceMinutes {
    /// <p>When specified, represents only the sum of metered minutes used by the resource to run tests.</p>
    #[serde(rename = "metered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered: Option<f64>,
    /// <p>When specified, represents the total minutes used by the resource to run tests.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    /// <p>When specified, represents only the sum of unmetered minutes used by the resource to run tests.</p>
    #[serde(rename = "unmetered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmetered: Option<f64>,
}

/// <p>Represents a collection of device types.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DevicePool {
    /// <p>The device pool's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The device pool's description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The number of devices that Device Farm can add to your device pool. Device Farm adds devices that are available and meet the criteria that you assign for the <code>rules</code> parameter. Depending on how many devices meet these constraints, your device pool might contain fewer devices than the value for this parameter.</p> <p>By specifying the maximum number of devices, you can control the costs that you incur by running tests.</p>
    #[serde(rename = "maxDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_devices: Option<i64>,
    /// <p>The device pool's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Information about the device pool's rules.</p>
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    /// <p><p>The device pool&#39;s type.</p> <p>Allowed values include:</p> <ul> <li> <p>CURATED: A device pool that is created and managed by AWS Device Farm.</p> </li> <li> <p>PRIVATE: A device pool that is created and managed by the device pool developer.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents a device pool compatibility result.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DevicePoolCompatibilityResult {
    /// <p>Whether the result was compatible with the device pool.</p>
    #[serde(rename = "compatible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible: Option<bool>,
    /// <p>The device (phone or tablet) to return information about.</p>
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// <p>Information about the compatibility.</p>
    #[serde(rename = "incompatibilityMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incompatibility_messages: Option<Vec<IncompatibilityMessage>>,
}

/// <p>Represents the device filters used in a test run and the maximum number of devices to be included in the run. It is passed in as the <code>deviceSelectionConfiguration</code> request parameter in <a>ScheduleRun</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeviceSelectionConfiguration {
    /// <p><p>Used to dynamically select a set of devices for a test run. A filter is made up of an attribute, an operator, and one or more values.</p> <ul> <li> <p> <b>Attribute</b> </p> <p>The aspect of a device such as platform or model used as the selection criteria in a device filter.</p> <p>Allowed values include:</p> <ul> <li> <p>ARN: The Amazon Resource Name (ARN) of the device (for example, <code>arn:aws:devicefarm:us-west-2::device:12345Example</code>).</p> </li> <li> <p>PLATFORM: The device platform. Valid values are ANDROID or IOS.</p> </li> <li> <p>OS<em>VERSION: The operating system version (for example, 10.3.2).</p> </li> <li> <p>MODEL: The device model (for example, iPad 5th Gen).</p> </li> <li> <p>AVAILABILITY: The current availability of the device. Valid values are AVAILABLE, HIGHLY</em>AVAILABLE, BUSY, or TEMPORARY<em>NOT</em>AVAILABLE.</p> </li> <li> <p>FORM<em>FACTOR: The device form factor. Valid values are PHONE or TABLET.</p> </li> <li> <p>MANUFACTURER: The device manufacturer (for example, Apple).</p> </li> <li> <p>REMOTE</em>ACCESS<em>ENABLED: Whether the device is enabled for remote access. Valid values are TRUE or FALSE.</p> </li> <li> <p>REMOTE</em>DEBUG<em>ENABLED: Whether the device is enabled for remote debugging. Valid values are TRUE or FALSE. Because remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>, this filter is ignored.</p> </li> <li> <p>INSTANCE</em>ARN: The Amazon Resource Name (ARN) of the device instance.</p> </li> <li> <p>INSTANCE<em>LABELS: The label of the device instance.</p> </li> <li> <p>FLEET</em>TYPE: The fleet type. Valid values are PUBLIC or PRIVATE.</p> </li> </ul> </li> <li> <p> <b>Operator</b> </p> <p>The filter operator.</p> <ul> <li> <p>The EQUALS operator is available for every attribute except INSTANCE<em>LABELS.</p> </li> <li> <p>The CONTAINS operator is available for the INSTANCE</em>LABELS and MODEL attributes.</p> </li> <li> <p>The IN and NOT<em>IN operators are available for the ARN, OS</em>VERSION, MODEL, MANUFACTURER, and INSTANCE<em>ARN attributes.</p> </li> <li> <p>The LESS</em>THAN, GREATER<em>THAN, LESS</em>THAN<em>OR</em>EQUALS, and GREATER<em>THAN</em>OR<em>EQUALS operators are also available for the OS</em>VERSION attribute.</p> </li> </ul> </li> <li> <p> <b>Values</b> </p> <p>An array of one or more filter values.</p> <p class="title"> <b>Operator Values</b> </p> <ul> <li> <p>The IN and NOT<em>IN operators can take a values array that has more than one element.</p> </li> <li> <p>The other operators require an array with a single element.</p> </li> </ul> <p class="title"> <b>Attribute Values</b> </p> <ul> <li> <p>The PLATFORM attribute can be set to ANDROID or IOS.</p> </li> <li> <p>The AVAILABILITY attribute can be set to AVAILABLE, HIGHLY</em>AVAILABLE, BUSY, or TEMPORARY<em>NOT</em>AVAILABLE.</p> </li> <li> <p>The FORM<em>FACTOR attribute can be set to PHONE or TABLET.</p> </li> <li> <p>The FLEET</em>TYPE attribute can be set to PUBLIC or PRIVATE.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "filters")]
    pub filters: Vec<DeviceFilter>,
    /// <p>The maximum number of devices to be included in a test run.</p>
    #[serde(rename = "maxDevices")]
    pub max_devices: i64,
}

/// <p>Contains the run results requested by the device selection configuration and how many devices were returned. For an example of the JSON response syntax, see <a>ScheduleRun</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeviceSelectionResult {
    /// <p>The filters in a device selection result.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DeviceFilter>>,
    /// <p>The number of devices that matched the device filter selection criteria.</p>
    #[serde(rename = "matchedDevicesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_devices_count: Option<i64>,
    /// <p>The maximum number of devices to be selected by a device filter and included in a test run.</p>
    #[serde(rename = "maxDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_devices: Option<i64>,
}

/// <p>Represents configuration information about a test run, such as the execution timeout (in minutes).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExecutionConfiguration {
    /// <p>True if account cleanup is enabled at the beginning of the test. Otherwise, false.</p>
    #[serde(rename = "accountsCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_cleanup: Option<bool>,
    /// <p>True if app package cleanup is enabled at the beginning of the test. Otherwise, false.</p>
    #[serde(rename = "appPackagesCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_packages_cleanup: Option<bool>,
    /// <p>The number of minutes a test run executes before it times out.</p>
    #[serde(rename = "jobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_timeout_minutes: Option<i64>,
    /// <p>When set to <code>true</code>, for private devices, Device Farm does not sign your app again. For public devices, Device Farm always signs your apps again.</p> <p>For more information about how Device Farm re-signs your apps, see <a href="https://aws.amazon.com/device-farm/faq/">Do you modify my app?</a> in the <i>AWS Device Farm FAQs</i>.</p>
    #[serde(rename = "skipAppResign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_app_resign: Option<bool>,
    /// <p>Set to true to enable video capture. Otherwise, set to false. The default is true.</p>
    #[serde(rename = "videoCapture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_capture: Option<bool>,
}

/// <p>Represents the request sent to retrieve the account settings.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAccountSettingsRequest {}

/// <p>Represents the account settings return values from the <code>GetAccountSettings</code> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAccountSettingsResult {
    /// <p>The account settings.</p>
    #[serde(rename = "accountSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_settings: Option<AccountSettings>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeviceInstanceRequest {
    /// <p>The Amazon Resource Name (ARN) of the instance you're requesting information about.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeviceInstanceResult {
    /// <p>An object that contains information about your device instance.</p>
    #[serde(rename = "deviceInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_instance: Option<DeviceInstance>,
}

/// <p>Represents a request to the get device pool compatibility operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDevicePoolCompatibilityRequest {
    /// <p>The ARN of the app that is associated with the specified device pool.</p>
    #[serde(rename = "appArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    /// <p>An object that contains information about the settings for a run.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ScheduleRunConfiguration>,
    /// <p>The device pool's ARN.</p>
    #[serde(rename = "devicePoolArn")]
    pub device_pool_arn: String,
    /// <p>Information about the uploaded test to be run against the device pool.</p>
    #[serde(rename = "test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<ScheduleRunTest>,
    /// <p><p>The test type for the specified device pool.</p> <p>Allowed values include the following:</p> <ul> <li> <p>BUILTIN<em>FUZZ.</p> </li> <li> <p>BUILTIN</em>EXPLORER. For Android, an app explorer that traverses an Android app, interacting with it and capturing screenshots at the same time.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT.</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG.</p> </li> <li> <p>APPIUM<em>PYTHON.</p> </li> <li> <p>APPIUM</em>NODE.</p> </li> <li> <p>APPIUM<em>RUBY.</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT.</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG.</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON.</p> </li> <li> <p>APPIUM</em>WEB<em>NODE.</p> </li> <li> <p>APPIUM</em>WEB<em>RUBY.</p> </li> <li> <p>CALABASH.</p> </li> <li> <p>INSTRUMENTATION.</p> </li> <li> <p>UIAUTOMATION.</p> </li> <li> <p>UIAUTOMATOR.</p> </li> <li> <p>XCTEST.</p> </li> <li> <p>XCTEST</em>UI.</p> </li> </ul></p>
    #[serde(rename = "testType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_type: Option<String>,
}

/// <p>Represents the result of describe device pool compatibility request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDevicePoolCompatibilityResult {
    /// <p>Information about compatible devices.</p>
    #[serde(rename = "compatibleDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_devices: Option<Vec<DevicePoolCompatibilityResult>>,
    /// <p>Information about incompatible devices.</p>
    #[serde(rename = "incompatibleDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incompatible_devices: Option<Vec<DevicePoolCompatibilityResult>>,
}

/// <p>Represents a request to the get device pool operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDevicePoolRequest {
    /// <p>The device pool's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get device pool request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDevicePoolResult {
    /// <p>An object that contains information about the requested device pool.</p>
    #[serde(rename = "devicePool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pool: Option<DevicePool>,
}

/// <p>Represents a request to the get device request.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeviceRequest {
    /// <p>The device type's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get device request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeviceResult {
    /// <p>An object that contains information about the requested device.</p>
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstanceProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of an instance profile.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstanceProfileResult {
    /// <p>An object that contains information about an instance profile.</p>
    #[serde(rename = "instanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
}

/// <p>Represents a request to the get job operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJobRequest {
    /// <p>The job's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get job request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobResult {
    /// <p>An object that contains information about the requested job.</p>
    #[serde(rename = "job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetNetworkProfileRequest {
    /// <p>The ARN of the network profile to return information about.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetNetworkProfileResult {
    /// <p>The network profile.</p>
    #[serde(rename = "networkProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
}

/// <p>Represents the request to retrieve the offering status for the specified customer or account.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOfferingStatusRequest {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Returns the status result for a device offering.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOfferingStatusResult {
    /// <p>When specified, gets the offering status for the current period.</p>
    #[serde(rename = "current")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<::std::collections::HashMap<String, OfferingStatus>>,
    /// <p>When specified, gets the offering status for the next period.</p>
    #[serde(rename = "nextPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_period: Option<::std::collections::HashMap<String, OfferingStatus>>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents a request to the get project operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetProjectRequest {
    /// <p>The project's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get project request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetProjectResult {
    /// <p>The project to get information about.</p>
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

/// <p>Represents the request to get information about the specified remote access session.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRemoteAccessSessionRequest {
    /// <p>The Amazon Resource Name (ARN) of the remote access session about which you want to get session information.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the response from the server that lists detailed information about the remote access session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRemoteAccessSessionResult {
    /// <p>A container that lists detailed information about the remote access session.</p>
    #[serde(rename = "remoteAccessSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_session: Option<RemoteAccessSession>,
}

/// <p>Represents a request to the get run operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRunRequest {
    /// <p>The run's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get run request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRunResult {
    /// <p>The run to get results from.</p>
    #[serde(rename = "run")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<Run>,
}

/// <p>Represents a request to the get suite operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSuiteRequest {
    /// <p>The suite's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get suite request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSuiteResult {
    /// <p>A collection of one or more tests.</p>
    #[serde(rename = "suite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite: Option<Suite>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTestGridProjectRequest {
    /// <p>The ARN of the Selenium testing project, from either <a>CreateTestGridProject</a> or <a>ListTestGridProjects</a>.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTestGridProjectResult {
    /// <p>A <a>TestGridProject</a>.</p>
    #[serde(rename = "testGridProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_grid_project: Option<TestGridProject>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTestGridSessionRequest {
    /// <p>The ARN for the project that this session belongs to. See <a>CreateTestGridProject</a> and <a>ListTestGridProjects</a>.</p>
    #[serde(rename = "projectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_arn: Option<String>,
    /// <p>An ARN that uniquely identifies a <a>TestGridSession</a>.</p>
    #[serde(rename = "sessionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_arn: Option<String>,
    /// <p>An ID associated with this session.</p>
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTestGridSessionResult {
    /// <p>The <a>TestGridSession</a> that was requested.</p>
    #[serde(rename = "testGridSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_grid_session: Option<TestGridSession>,
}

/// <p>Represents a request to the get test operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTestRequest {
    /// <p>The test's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get test request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTestResult {
    /// <p>A test condition that is evaluated.</p>
    #[serde(rename = "test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<Test>,
}

/// <p>Represents a request to the get upload operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUploadRequest {
    /// <p>The upload's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get upload request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUploadResult {
    /// <p>An app or a set of one or more tests to upload or that have been uploaded.</p>
    #[serde(rename = "upload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Upload>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVPCEConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration you want to describe.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVPCEConfigurationResult {
    /// <p>An object that contains information about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration: Option<VPCEConfiguration>,
}

/// <p>Represents information about incompatibility.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IncompatibilityMessage {
    /// <p>A message about the incompatibility.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The type of incompatibility.</p> <p>Allowed values include:</p> <ul> <li> <p>ARN</p> </li> <li> <p>FORM<em>FACTOR (for example, phone or tablet)</p> </li> <li> <p>MANUFACTURER</p> </li> <li> <p>PLATFORM (for example, Android or iOS)</p> </li> <li> <p>REMOTE</em>ACCESS<em>ENABLED</p> </li> <li> <p>APPIUM</em>VERSION</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents the request to install an Android application (in .apk format) or an iOS application (in .ipa format) as part of a remote access session.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstallToRemoteAccessSessionRequest {
    /// <p>The ARN of the app about which you are requesting information.</p>
    #[serde(rename = "appArn")]
    pub app_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the remote access session about which you are requesting information.</p>
    #[serde(rename = "remoteAccessSessionArn")]
    pub remote_access_session_arn: String,
}

/// <p>Represents the response from the server after AWS Device Farm makes a request to install to a remote access session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstallToRemoteAccessSessionResult {
    /// <p>An app to upload or that has been uploaded.</p>
    #[serde(rename = "appUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_upload: Option<Upload>,
}

/// <p>Represents the instance profile.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceProfile {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the instance profile.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An array of strings containing the list of app packages that should not be cleaned up from the device after a test run completes.</p> <p>The list of packages is considered only if you set <code>packageCleanup</code> to <code>true</code>.</p>
    #[serde(rename = "excludeAppPackagesFromCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_app_packages_from_cleanup: Option<Vec<String>>,
    /// <p>The name of the instance profile.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>When set to <code>true</code>, Device Farm removes app packages after a test run. The default value is <code>false</code> for private devices.</p>
    #[serde(rename = "packageCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_cleanup: Option<bool>,
    /// <p>When set to <code>true</code>, Device Farm reboots the instance after a test run. The default value is <code>true</code>.</p>
    #[serde(rename = "rebootAfterUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_after_use: Option<bool>,
}

/// <p>Represents a device.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Job {
    /// <p>The job's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The job's result counters.</p>
    #[serde(rename = "counters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Counters>,
    /// <p>When the job was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The device (phone or tablet).</p>
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// <p>Represents the total (metered or unmetered) minutes used by the job.</p>
    #[serde(rename = "deviceMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_minutes: Option<DeviceMinutes>,
    /// <p>The ARN of the instance.</p>
    #[serde(rename = "instanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    /// <p>A message about the job's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The job's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The job&#39;s result.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>PASSED</p> </li> <li> <p>WARNED</p> </li> <li> <p>FAILED</p> </li> <li> <p>SKIPPED</p> </li> <li> <p>ERRORED</p> </li> <li> <p>STOPPED</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The job's start time.</p>
    #[serde(rename = "started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
    /// <p><p>The job&#39;s status.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>PENDING<em>CONCURRENCY</p> </li> <li> <p>PENDING</em>DEVICE</p> </li> <li> <p>PROCESSING</p> </li> <li> <p>SCHEDULING</p> </li> <li> <p>PREPARING</p> </li> <li> <p>RUNNING</p> </li> <li> <p>COMPLETED</p> </li> <li> <p>STOPPING</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The job's stop time.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<f64>,
    /// <p><p>The job&#39;s type.</p> <p>Allowed values include the following:</p> <ul> <li> <p>BUILTIN<em>FUZZ</p> </li> <li> <p>BUILTIN</em>EXPLORER. For Android, an app explorer that traverses an Android app, interacting with it and capturing screenshots at the same time.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG</p> </li> <li> <p>APPIUM<em>PYTHON</p> </li> <li> <p>APPIUM</em>NODE</p> </li> <li> <p>APPIUM<em>RUBY</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</p> </li> <li> <p>APPIUM</em>WEB<em>NODE</p> </li> <li> <p>APPIUM</em>WEB<em>RUBY</p> </li> <li> <p>CALABASH</p> </li> <li> <p>INSTRUMENTATION</p> </li> <li> <p>UIAUTOMATION</p> </li> <li> <p>UIAUTOMATOR</p> </li> <li> <p>XCTEST</p> </li> <li> <p>XCTEST</em>UI</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>This value is set to true if video capture is enabled. Otherwise, it is set to false.</p>
    #[serde(rename = "videoCapture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_capture: Option<bool>,
    /// <p>The endpoint for streaming device video.</p>
    #[serde(rename = "videoEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_endpoint: Option<String>,
}

/// <p>Represents a request to the list artifacts operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListArtifactsRequest {
    /// <p>The run, job, suite, or test ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The artifacts&#39; type.</p> <p>Allowed values include:</p> <ul> <li> <p>FILE</p> </li> <li> <p>LOG</p> </li> <li> <p>SCREENSHOT</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Represents the result of a list artifacts operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListArtifactsResult {
    /// <p>Information about the artifacts.</p>
    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<Artifact>>,
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeviceInstancesRequest {
    /// <p>An integer that specifies the maximum number of items you want to return in the API response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeviceInstancesResult {
    /// <p>An object that contains information about your device instances.</p>
    #[serde(rename = "deviceInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_instances: Option<Vec<DeviceInstance>>,
    /// <p>An identifier that can be used in the next call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list device pools request.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDevicePoolsRequest {
    /// <p>The project ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The device pools&#39; type.</p> <p>Allowed values include:</p> <ul> <li> <p>CURATED: A device pool that is created and managed by AWS Device Farm.</p> </li> <li> <p>PRIVATE: A device pool that is created and managed by the device pool developer.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents the result of a list device pools request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDevicePoolsResult {
    /// <p>Information about the device pools.</p>
    #[serde(rename = "devicePools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pools: Option<Vec<DevicePool>>,
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list devices request.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDevicesRequest {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p><p>Used to select a set of devices. A filter is made up of an attribute, an operator, and one or more values.</p> <ul> <li> <p>Attribute: The aspect of a device such as platform or model used as the selection criteria in a device filter.</p> <p>Allowed values include:</p> <ul> <li> <p>ARN: The Amazon Resource Name (ARN) of the device (for example, <code>arn:aws:devicefarm:us-west-2::device:12345Example</code>).</p> </li> <li> <p>PLATFORM: The device platform. Valid values are ANDROID or IOS.</p> </li> <li> <p>OS<em>VERSION: The operating system version (for example, 10.3.2).</p> </li> <li> <p>MODEL: The device model (for example, iPad 5th Gen).</p> </li> <li> <p>AVAILABILITY: The current availability of the device. Valid values are AVAILABLE, HIGHLY</em>AVAILABLE, BUSY, or TEMPORARY<em>NOT</em>AVAILABLE.</p> </li> <li> <p>FORM<em>FACTOR: The device form factor. Valid values are PHONE or TABLET.</p> </li> <li> <p>MANUFACTURER: The device manufacturer (for example, Apple).</p> </li> <li> <p>REMOTE</em>ACCESS<em>ENABLED: Whether the device is enabled for remote access. Valid values are TRUE or FALSE.</p> </li> <li> <p>REMOTE</em>DEBUG<em>ENABLED: Whether the device is enabled for remote debugging. Valid values are TRUE or FALSE. Because remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>, this attribute is ignored.</p> </li> <li> <p>INSTANCE</em>ARN: The Amazon Resource Name (ARN) of the device instance.</p> </li> <li> <p>INSTANCE<em>LABELS: The label of the device instance.</p> </li> <li> <p>FLEET</em>TYPE: The fleet type. Valid values are PUBLIC or PRIVATE.</p> </li> </ul> </li> <li> <p>Operator: The filter operator.</p> <ul> <li> <p>The EQUALS operator is available for every attribute except INSTANCE<em>LABELS.</p> </li> <li> <p>The CONTAINS operator is available for the INSTANCE</em>LABELS and MODEL attributes.</p> </li> <li> <p>The IN and NOT<em>IN operators are available for the ARN, OS</em>VERSION, MODEL, MANUFACTURER, and INSTANCE<em>ARN attributes.</p> </li> <li> <p>The LESS</em>THAN, GREATER<em>THAN, LESS</em>THAN<em>OR</em>EQUALS, and GREATER<em>THAN</em>OR<em>EQUALS operators are also available for the OS</em>VERSION attribute.</p> </li> </ul> </li> <li> <p>Values: An array of one or more filter values.</p> <ul> <li> <p>The IN and NOT<em>IN operators take a values array that has one or more elements.</p> </li> <li> <p>The other operators require an array with a single element.</p> </li> <li> <p>In a request, the AVAILABILITY attribute takes the following values: AVAILABLE, HIGHLY</em>AVAILABLE, BUSY, or TEMPORARY<em>NOT</em>AVAILABLE.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DeviceFilter>>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list devices operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDevicesResult {
    /// <p>Information about the devices.</p>
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInstanceProfilesRequest {
    /// <p>An integer that specifies the maximum number of items you want to return in the API response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInstanceProfilesResult {
    /// <p>An object that contains information about your instance profiles.</p>
    #[serde(rename = "instanceProfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profiles: Option<Vec<InstanceProfile>>,
    /// <p>An identifier that can be used in the next call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents a request to the list jobs operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobsRequest {
    /// <p>The run's Amazon Resource Name (ARN).</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list jobs request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJobsResult {
    /// <p>Information about the jobs.</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListNetworkProfilesRequest {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to list network profiles.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of network profile to return information about. Valid values are listed here.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListNetworkProfilesResult {
    /// <p>A list of the available network profiles.</p>
    #[serde(rename = "networkProfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profiles: Option<Vec<NetworkProfile>>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOfferingPromotionsRequest {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOfferingPromotionsResult {
    /// <p>An identifier to be used in the next call to this operation, to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the offering promotions.</p>
    #[serde(rename = "offeringPromotions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_promotions: Option<Vec<OfferingPromotion>>,
}

/// <p>Represents the request to list the offering transaction history.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOfferingTransactionsRequest {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Returns the transaction log of the specified offerings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOfferingTransactionsResult {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The audit log of subscriptions you have purchased and modified through AWS Device Farm.</p>
    #[serde(rename = "offeringTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_transactions: Option<Vec<OfferingTransaction>>,
}

/// <p>Represents the request to list all offerings.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOfferingsRequest {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the return values of the list of offerings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOfferingsResult {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A value that represents the list offering results.</p>
    #[serde(rename = "offerings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offerings: Option<Vec<Offering>>,
}

/// <p>Represents a request to the list projects operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProjectsRequest {
    /// <p>Optional. If no Amazon Resource Name (ARN) is specified, then AWS Device Farm returns a list of all projects for the AWS account. You can also specify a project ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list projects request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProjectsResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the projects.</p>
    #[serde(rename = "projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<Project>>,
}

/// <p>Represents the request to return information about the remote access session.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRemoteAccessSessionsRequest {
    /// <p>The Amazon Resource Name (ARN) of the project about which you are requesting information.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the response from the server after AWS Device Farm makes a request to return information about the remote access session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRemoteAccessSessionsResult {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A container that represents the metadata from the service about each remote access session you are requesting.</p>
    #[serde(rename = "remoteAccessSessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_sessions: Option<Vec<RemoteAccessSession>>,
}

/// <p>Represents a request to the list runs operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRunsRequest {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to list runs.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list runs request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRunsResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the runs.</p>
    #[serde(rename = "runs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs: Option<Vec<Run>>,
}

/// <p>Represents a request to the list samples operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSamplesRequest {
    /// <p>The Amazon Resource Name (ARN) of the job used to list samples.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list samples request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSamplesResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the samples.</p>
    #[serde(rename = "samples")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samples: Option<Vec<Sample>>,
}

/// <p>Represents a request to the list suites operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSuitesRequest {
    /// <p>The job's Amazon Resource Name (ARN).</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list suites request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSuitesResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the suites.</p>
    #[serde(rename = "suites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suites: Option<Vec<Suite>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource or resources for which to list tags. You can associate tags with the following Device Farm resources: <code>PROJECT</code>, <code>RUN</code>, <code>NETWORK_PROFILE</code>, <code>INSTANCE_PROFILE</code>, <code>DEVICE_INSTANCE</code>, <code>SESSION</code>, <code>DEVICE_POOL</code>, <code>DEVICE</code>, and <code>VPCE_CONFIGURATION</code>.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags to add to the resource. A tag is an array of key-value pairs. Tag keys can have a maximum character length of 128 characters. Tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTestGridProjectsRequest {
    /// <p>Return no more than this number of results.</p>
    #[serde(rename = "maxResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i64>,
    /// <p>From a response, used to continue a paginated listing. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTestGridProjectsResult {
    /// <p>Used for pagination. Pass into <a>ListTestGridProjects</a> to get more results in a paginated request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of TestGridProjects, based on a <a>ListTestGridProjectsRequest</a>.</p>
    #[serde(rename = "testGridProjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_grid_projects: Option<Vec<TestGridProject>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTestGridSessionActionsRequest {
    /// <p>The maximum number of sessions to return per response.</p>
    #[serde(rename = "maxResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i64>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the session to retrieve.</p>
    #[serde(rename = "sessionArn")]
    pub session_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTestGridSessionActionsResult {
    /// <p>The action taken by the session.</p>
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<TestGridSessionAction>>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTestGridSessionArtifactsRequest {
    /// <p>The maximum number of results to be returned by a request.</p>
    #[serde(rename = "maxResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i64>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of a <a>TestGridSession</a>. </p>
    #[serde(rename = "sessionArn")]
    pub session_arn: String,
    /// <p>Limit results to a specified type of artifact.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTestGridSessionArtifactsResult {
    /// <p>A list of test grid session artifacts for a <a>TestGridSession</a>.</p>
    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<TestGridSessionArtifact>>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTestGridSessionsRequest {
    /// <p>Return only sessions created after this time.</p>
    #[serde(rename = "creationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>Return only sessions created before this time.</p>
    #[serde(rename = "creationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>Return only sessions that ended after this time.</p>
    #[serde(rename = "endTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_after: Option<f64>,
    /// <p>Return only sessions that ended before this time.</p>
    #[serde(rename = "endTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_before: Option<f64>,
    /// <p>Return only this many results at a time.</p>
    #[serde(rename = "maxResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i64>,
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>ARN of a <a>TestGridProject</a>.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>Return only sessions in this state.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTestGridSessionsResult {
    /// <p>Pagination token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sessions that match the criteria in a <a>ListTestGridSessionsRequest</a>. </p>
    #[serde(rename = "testGridSessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_grid_sessions: Option<Vec<TestGridSession>>,
}

/// <p>Represents a request to the list tests operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTestsRequest {
    /// <p>The test suite's Amazon Resource Name (ARN).</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list tests request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTestsResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the tests.</p>
    #[serde(rename = "tests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests: Option<Vec<Test>>,
}

/// <p>Represents a request to the list unique problems operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUniqueProblemsRequest {
    /// <p>The unique problems' ARNs.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list unique problems request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUniqueProblemsResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>Information about the unique problems.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>PASSED</p> </li> <li> <p>WARNED</p> </li> <li> <p>FAILED</p> </li> <li> <p>SKIPPED</p> </li> <li> <p>ERRORED</p> </li> <li> <p>STOPPED</p> </li> </ul></p>
    #[serde(rename = "uniqueProblems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_problems: Option<::std::collections::HashMap<String, Vec<UniqueProblem>>>,
}

/// <p>Represents a request to the list uploads operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUploadsRequest {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to list uploads.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The type of upload.</p> <p>Must be one of the following values:</p> <ul> <li> <p>ANDROID<em>APP</p> </li> <li> <p>IOS</em>APP</p> </li> <li> <p>WEB<em>APP</p> </li> <li> <p>EXTERNAL</em>DATA</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT<em>TEST</em>PACKAGE</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG<em>TEST</em>PACKAGE</p> </li> <li> <p>APPIUM<em>PYTHON</em>TEST<em>PACKAGE</p> </li> <li> <p>APPIUM</em>NODE<em>TEST</em>PACKAGE</p> </li> <li> <p>APPIUM<em>RUBY</em>TEST<em>PACKAGE</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT<em>TEST</em>PACKAGE</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</em>TEST<em>PACKAGE</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</em>TEST<em>PACKAGE</p> </li> <li> <p>APPIUM</em>WEB<em>NODE</em>TEST<em>PACKAGE</p> </li> <li> <p>APPIUM</em>WEB<em>RUBY</em>TEST<em>PACKAGE</p> </li> <li> <p>CALABASH</em>TEST<em>PACKAGE</p> </li> <li> <p>INSTRUMENTATION</em>TEST<em>PACKAGE</p> </li> <li> <p>UIAUTOMATION</em>TEST<em>PACKAGE</p> </li> <li> <p>UIAUTOMATOR</em>TEST<em>PACKAGE</p> </li> <li> <p>XCTEST</em>TEST<em>PACKAGE</p> </li> <li> <p>XCTEST</em>UI<em>TEST</em>PACKAGE</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT<em>TEST</em>SPEC</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG<em>TEST</em>SPEC</p> </li> <li> <p>APPIUM<em>PYTHON</em>TEST<em>SPEC</p> </li> <li> <p>APPIUM</em>NODE<em>TEST</em>SPEC</p> </li> <li> <p> APPIUM<em>RUBY</em>TEST<em>SPEC</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT<em>TEST</em>SPEC</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</em>TEST<em>SPEC</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</em>TEST<em>SPEC</p> </li> <li> <p>APPIUM</em>WEB<em>NODE</em>TEST<em>SPEC</p> </li> <li> <p>APPIUM</em>WEB<em>RUBY</em>TEST<em>SPEC</p> </li> <li> <p>INSTRUMENTATION</em>TEST<em>SPEC</p> </li> <li> <p>XCTEST</em>UI<em>TEST</em>SPEC</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents the result of a list uploads request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUploadsResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the uploads.</p>
    #[serde(rename = "uploads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploads: Option<Vec<Upload>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVPCEConfigurationsRequest {
    /// <p>An integer that specifies the maximum number of items you want to return in the API response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVPCEConfigurationsResult {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>VPCEConfiguration</code> objects that contain information about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configurations: Option<Vec<VPCEConfiguration>>,
}

/// <p>Represents a latitude and longitude pair, expressed in geographic coordinate system degrees (for example, 47.6204, -122.3491).</p> <p>Elevation is currently not supported.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Location {
    /// <p>The latitude.</p>
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// <p>The longitude.</p>
    #[serde(rename = "longitude")]
    pub longitude: f64,
}

/// <p>A number that represents the monetary amount for an offering or transaction.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MonetaryAmount {
    /// <p>The numerical amount of an offering or transaction.</p>
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// <p>The currency code of a monetary amount. For example, <code>USD</code> means U.S. dollars.</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

/// <p>An array of settings that describes characteristics of a network profile.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkProfile {
    /// <p>The Amazon Resource Name (ARN) of the network profile.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the network profile.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "downlinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_jitter_ms: Option<i64>,
    /// <p>Proportion of received packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "downlinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_loss_percent: Option<i64>,
    /// <p>The name of the network profile.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of network profile. Valid values are listed here.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "uplinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_jitter_ms: Option<i64>,
    /// <p>Proportion of transmitted packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "uplinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_loss_percent: Option<i64>,
}

/// <p>Represents the metadata of a device offering.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Offering {
    /// <p>A string that describes the offering.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID that corresponds to a device offering.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The platform of the device (for example, <code>ANDROID</code> or <code>IOS</code>).</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Specifies whether there are recurring charges for the offering.</p>
    #[serde(rename = "recurringCharges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    /// <p>The type of offering (for example, <code>RECURRING</code>) for a device.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents information about an offering promotion.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OfferingPromotion {
    /// <p>A string that describes the offering promotion.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the offering promotion.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The status of the offering.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OfferingStatus {
    /// <p>The date on which the offering is effective.</p>
    #[serde(rename = "effectiveOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_on: Option<f64>,
    /// <p>Represents the metadata of an offering status.</p>
    #[serde(rename = "offering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering: Option<Offering>,
    /// <p>The number of available devices in the offering.</p>
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// <p>The type specified for the offering status.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents the metadata of an offering transaction.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OfferingTransaction {
    /// <p>The cost of an offering transaction.</p>
    #[serde(rename = "cost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<MonetaryAmount>,
    /// <p>The date on which an offering transaction was created.</p>
    #[serde(rename = "createdOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    /// <p>The ID that corresponds to a device offering promotion.</p>
    #[serde(rename = "offeringPromotionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_promotion_id: Option<String>,
    /// <p>The status of an offering transaction.</p>
    #[serde(rename = "offeringStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_status: Option<OfferingStatus>,
    /// <p>The transaction ID of the offering transaction.</p>
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>Represents a specific warning or failure.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Problem {
    /// <p>Information about the associated device.</p>
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// <p>Information about the associated job.</p>
    #[serde(rename = "job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<ProblemDetail>,
    /// <p>A message about the problem's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The problem&#39;s result.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>PASSED</p> </li> <li> <p>WARNED</p> </li> <li> <p>FAILED</p> </li> <li> <p>SKIPPED</p> </li> <li> <p>ERRORED</p> </li> <li> <p>STOPPED</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>Information about the associated run.</p>
    #[serde(rename = "run")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<ProblemDetail>,
    /// <p>Information about the associated suite.</p>
    #[serde(rename = "suite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite: Option<ProblemDetail>,
    /// <p>Information about the associated test.</p>
    #[serde(rename = "test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<ProblemDetail>,
}

/// <p>Information about a problem detail.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProblemDetail {
    /// <p>The problem detail's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The problem detail's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents an operating-system neutral workspace for running and managing tests.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Project {
    /// <p>The project's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>When the project was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The default number of minutes (at the project level) a test run executes before it times out. The default value is 150 minutes.</p>
    #[serde(rename = "defaultJobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_job_timeout_minutes: Option<i64>,
    /// <p>The project's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents a request for a purchase offering.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PurchaseOfferingRequest {
    /// <p>The ID of the offering.</p>
    #[serde(rename = "offeringId")]
    pub offering_id: String,
    /// <p>The ID of the offering promotion to be applied to the purchase.</p>
    #[serde(rename = "offeringPromotionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_promotion_id: Option<String>,
    /// <p>The number of device slots to purchase in an offering request.</p>
    #[serde(rename = "quantity")]
    pub quantity: i64,
}

/// <p>The result of the purchase offering (for example, success or failure).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PurchaseOfferingResult {
    /// <p>Represents the offering transaction for the purchase result.</p>
    #[serde(rename = "offeringTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_transaction: Option<OfferingTransaction>,
}

/// <p>Represents the set of radios and their states on a device. Examples of radios include Wi-Fi, GPS, Bluetooth, and NFC.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Radios {
    /// <p>True if Bluetooth is enabled at the beginning of the test. Otherwise, false.</p>
    #[serde(rename = "bluetooth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth: Option<bool>,
    /// <p>True if GPS is enabled at the beginning of the test. Otherwise, false.</p>
    #[serde(rename = "gps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps: Option<bool>,
    /// <p>True if NFC is enabled at the beginning of the test. Otherwise, false.</p>
    #[serde(rename = "nfc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfc: Option<bool>,
    /// <p>True if Wi-Fi is enabled at the beginning of the test. Otherwise, false.</p>
    #[serde(rename = "wifi")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi: Option<bool>,
}

/// <p>Specifies whether charges for devices are recurring.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecurringCharge {
    /// <p>The cost of the recurring charge.</p>
    #[serde(rename = "cost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<MonetaryAmount>,
    /// <p>The frequency in which charges recur.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}

/// <p>Represents information about the remote access session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoteAccessSession {
    /// <p>The Amazon Resource Name (ARN) of the remote access session.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The billing method of the remote access session. Possible values include <code>METERED</code> or <code>UNMETERED</code>. For more information about metered devices, see <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/welcome.html#welcome-terminology">AWS Device Farm terminology</a>.</p>
    #[serde(rename = "billingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_method: Option<String>,
    /// <p>Unique identifier of your client for the remote access session. Only returned if remote debugging is enabled for the remote access session.</p> <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>.</p>
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The date and time the remote access session was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The device (phone or tablet) used in the remote access session.</p>
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// <p>The number of minutes a device is used in a remote access session (including setup and teardown minutes).</p>
    #[serde(rename = "deviceMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_minutes: Option<DeviceMinutes>,
    /// <p>Unique device identifier for the remote device. Only returned if remote debugging is enabled for the remote access session.</p> <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>.</p>
    #[serde(rename = "deviceUdid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_udid: Option<String>,
    /// <p>The endpoint for the remote access sesssion.</p>
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>IP address of the EC2 host where you need to connect to remotely debug devices. Only returned if remote debugging is enabled for the remote access session.</p> <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>.</p>
    #[serde(rename = "hostAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    /// <p>The ARN of the instance.</p>
    #[serde(rename = "instanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    /// <p><p>The interaction mode of the remote access session. Valid values are:</p> <ul> <li> <p>INTERACTIVE: You can interact with the iOS device by viewing, touching, and rotating the screen. You cannot run XCUITest framework-based tests in this mode.</p> </li> <li> <p>NO<em>VIDEO: You are connected to the device, but cannot interact with it or view the screen. This mode has the fastest test execution speed. You can run XCUITest framework-based tests in this mode.</p> </li> <li> <p>VIDEO</em>ONLY: You can view the screen, but cannot touch or rotate it. You can run XCUITest framework-based tests and watch the screen in this mode.</p> </li> </ul></p>
    #[serde(rename = "interactionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_mode: Option<String>,
    /// <p>A message about the remote access session.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The name of the remote access session.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>This flag is set to <code>true</code> if remote debugging is enabled for the remote access session.</p> <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>.</p>
    #[serde(rename = "remoteDebugEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_debug_enabled: Option<bool>,
    /// <p>The ARN for the app to be recorded in the remote access session.</p>
    #[serde(rename = "remoteRecordAppArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_record_app_arn: Option<String>,
    /// <p>This flag is set to <code>true</code> if remote recording is enabled for the remote access session.</p>
    #[serde(rename = "remoteRecordEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_record_enabled: Option<bool>,
    /// <p><p>The result of the remote access session. Can be any of the following:</p> <ul> <li> <p>PENDING.</p> </li> <li> <p>PASSED.</p> </li> <li> <p>WARNED.</p> </li> <li> <p>FAILED.</p> </li> <li> <p>SKIPPED.</p> </li> <li> <p>ERRORED.</p> </li> <li> <p>STOPPED.</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>When set to <code>true</code>, for private devices, Device Farm does not sign your app again. For public devices, Device Farm always signs your apps again.</p> <p>For more information about how Device Farm re-signs your apps, see <a href="https://aws.amazon.com/device-farm/faq/">Do you modify my app?</a> in the <i>AWS Device Farm FAQs</i>.</p>
    #[serde(rename = "skipAppResign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_app_resign: Option<bool>,
    /// <p>The date and time the remote access session was started.</p>
    #[serde(rename = "started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
    /// <p><p>The status of the remote access session. Can be any of the following:</p> <ul> <li> <p>PENDING.</p> </li> <li> <p>PENDING<em>CONCURRENCY.</p> </li> <li> <p>PENDING</em>DEVICE.</p> </li> <li> <p>PROCESSING.</p> </li> <li> <p>SCHEDULING.</p> </li> <li> <p>PREPARING.</p> </li> <li> <p>RUNNING.</p> </li> <li> <p>COMPLETED.</p> </li> <li> <p>STOPPING.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The date and time the remote access session was stopped.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<f64>,
}

/// <p>A request that represents an offering renewal.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RenewOfferingRequest {
    /// <p>The ID of a request to renew an offering.</p>
    #[serde(rename = "offeringId")]
    pub offering_id: String,
    /// <p>The quantity requested in an offering renewal.</p>
    #[serde(rename = "quantity")]
    pub quantity: i64,
}

/// <p>The result of a renewal offering.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RenewOfferingResult {
    /// <p>Represents the status of the offering transaction for the renewal.</p>
    #[serde(rename = "offeringTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_transaction: Option<OfferingTransaction>,
}

/// <p>Represents the screen resolution of a device in height and width, expressed in pixels.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Resolution {
    /// <p>The screen resolution's height, expressed in pixels.</p>
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>The screen resolution's width, expressed in pixels.</p>
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

/// <p>Represents a condition for a device pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Rule {
    /// <p><p>The rule&#39;s stringified attribute. For example, specify the value as <code>&quot;&quot;abc&quot;&quot;</code>.</p> <p>The supported operators for each attribute are provided in the following list.</p> <dl> <dt>APPIUM<em>VERSION</dt> <dd> <p>The Appium version for the test.</p> <p>Supported operators: <code>CONTAINS</code> </p> </dd> <dt>ARN</dt> <dd> <p>The Amazon Resource Name (ARN) of the device (for example, <code>arn:aws:devicefarm:us-west-2::device:12345Example</code>.</p> <p>Supported operators: <code>EQUALS</code>, <code>IN</code>, <code>NOT</em>IN</code> </p> </dd> <dt>AVAILABILITY</dt> <dd> <p>The current availability of the device. Valid values are AVAILABLE, HIGHLY<em>AVAILABLE, BUSY, or TEMPORARY</em>NOT<em>AVAILABLE.</p> <p>Supported operators: <code>EQUALS</code> </p> </dd> <dt>FLEET</em>TYPE</dt> <dd> <p>The fleet type. Valid values are PUBLIC or PRIVATE.</p> <p>Supported operators: <code>EQUALS</code> </p> </dd> <dt>FORM<em>FACTOR</dt> <dd> <p>The device form factor. Valid values are PHONE or TABLET.</p> <p>Supported operators: <code>EQUALS</code>, <code>IN</code>, <code>NOT</em>IN</code> </p> </dd> <dt>INSTANCE<em>ARN</dt> <dd> <p>The Amazon Resource Name (ARN) of the device instance.</p> <p>Supported operators: <code>IN</code>, <code>NOT</em>IN</code> </p> </dd> <dt>INSTANCE<em>LABELS</dt> <dd> <p>The label of the device instance.</p> <p>Supported operators: <code>CONTAINS</code> </p> </dd> <dt>MANUFACTURER</dt> <dd> <p>The device manufacturer (for example, Apple).</p> <p>Supported operators: <code>EQUALS</code>, <code>IN</code>, <code>NOT</em>IN</code> </p> </dd> <dt>MODEL</dt> <dd> <p>The device model, such as Apple iPad Air 2 or Google Pixel.</p> <p>Supported operators: <code>CONTAINS</code>, <code>EQUALS</code>, <code>IN</code>, <code>NOT<em>IN</code> </p> </dd> <dt>OS</em>VERSION</dt> <dd> <p>The operating system version (for example, 10.3.2).</p> <p>Supported operators: <code>EQUALS</code>, <code>GREATER<em>THAN</code>, <code>GREATER</em>THAN<em>OR</em>EQUALS</code>, <code>IN</code>, <code>LESS<em>THAN</code>, <code>LESS</em>THAN<em>OR</em>EQUALS</code>, <code>NOT<em>IN</code> </p> </dd> <dt>PLATFORM</dt> <dd> <p>The device platform. Valid values are ANDROID or IOS.</p> <p>Supported operators: <code>EQUALS</code>, <code>IN</code>, <code>NOT</em>IN</code> </p> </dd> <dt>REMOTE<em>ACCESS</em>ENABLED</dt> <dd> <p>Whether the device is enabled for remote access. Valid values are TRUE or FALSE.</p> <p>Supported operators: <code>EQUALS</code> </p> </dd> <dt>REMOTE<em>DEBUG</em>ENABLED</dt> <dd> <p>Whether the device is enabled for remote debugging. Valid values are TRUE or FALSE.</p> <p>Supported operators: <code>EQUALS</code> </p> <p>Because remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>, this filter is ignored.</p> </dd> </dl></p>
    #[serde(rename = "attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    /// <p>Specifies how Device Farm compares the rule's attribute to the value. For the operators that are supported by each attribute, see the attribute descriptions.</p>
    #[serde(rename = "operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// <p>The rule's value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Represents a test run on a set of devices with a given app package, test parameters, and so on.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Run {
    /// <p>An app to upload or that has been uploaded.</p>
    #[serde(rename = "appUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_upload: Option<String>,
    /// <p>The run's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p><p>Specifies the billing method for a test run: <code>metered</code> or <code>unmetered</code>. If the parameter is not specified, the default value is <code>metered</code>.</p> <note> <p>If you have unmetered device slots, you must set this to <code>unmetered</code> to use them. Otherwise, the run is counted toward metered device minutes.</p> </note></p>
    #[serde(rename = "billingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_method: Option<String>,
    /// <p>The total number of completed jobs.</p>
    #[serde(rename = "completedJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_jobs: Option<i64>,
    /// <p>The run's result counters.</p>
    #[serde(rename = "counters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Counters>,
    /// <p>When the run was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Output <code>CustomerArtifactPaths</code> object for the test run.</p>
    #[serde(rename = "customerArtifactPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_artifact_paths: Option<CustomerArtifactPaths>,
    /// <p>Represents the total (metered or unmetered) minutes used by the test run.</p>
    #[serde(rename = "deviceMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_minutes: Option<DeviceMinutes>,
    /// <p>The ARN of the device pool for the run.</p>
    #[serde(rename = "devicePoolArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pool_arn: Option<String>,
    /// <p>The results of a device filter used to select the devices for a test run.</p>
    #[serde(rename = "deviceSelectionResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_selection_result: Option<DeviceSelectionResult>,
    /// <p>For fuzz tests, this is the number of events, between 1 and 10000, that the UI fuzz test should perform.</p>
    #[serde(rename = "eventCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i64>,
    /// <p>The number of minutes the job executes before it times out.</p>
    #[serde(rename = "jobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_timeout_minutes: Option<i64>,
    /// <p>Information about the locale that is used for the run.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>Information about the location that is used for the run.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>A message about the run's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The run's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The network profile being used for a test run.</p>
    #[serde(rename = "networkProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    /// <p>Read-only URL for an object in an S3 bucket where you can get the parsing results of the test package. If the test package doesn't parse, the reason why it doesn't parse appears in the file that this URL points to.</p>
    #[serde(rename = "parsingResultUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_result_url: Option<String>,
    /// <p><p>The run&#39;s platform.</p> <p>Allowed values include:</p> <ul> <li> <p>ANDROID</p> </li> <li> <p>IOS</p> </li> </ul></p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Information about the radio states for the run.</p>
    #[serde(rename = "radios")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radios: Option<Radios>,
    /// <p><p>The run&#39;s result.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>PASSED</p> </li> <li> <p>WARNED</p> </li> <li> <p>FAILED</p> </li> <li> <p>SKIPPED</p> </li> <li> <p>ERRORED</p> </li> <li> <p>STOPPED</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>Supporting field for the result field. Set only if <code>result</code> is <code>SKIPPED</code>. <code>PARSING_FAILED</code> if the result is skipped because of test package parsing failure.</p>
    #[serde(rename = "resultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
    /// <p>For fuzz tests, this is a seed to use for randomizing the UI fuzz test. Using the same seed value between tests ensures identical event sequences.</p>
    #[serde(rename = "seed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// <p>When set to <code>true</code>, for private devices, Device Farm does not sign your app again. For public devices, Device Farm always signs your apps again.</p> <p>For more information about how Device Farm re-signs your apps, see <a href="https://aws.amazon.com/device-farm/faq/">Do you modify my app?</a> in the <i>AWS Device Farm FAQs</i>.</p>
    #[serde(rename = "skipAppResign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_app_resign: Option<bool>,
    /// <p>The run's start time.</p>
    #[serde(rename = "started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
    /// <p><p>The run&#39;s status.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>PENDING<em>CONCURRENCY</p> </li> <li> <p>PENDING</em>DEVICE</p> </li> <li> <p>PROCESSING</p> </li> <li> <p>SCHEDULING</p> </li> <li> <p>PREPARING</p> </li> <li> <p>RUNNING</p> </li> <li> <p>COMPLETED</p> </li> <li> <p>STOPPING</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The run's stop time.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<f64>,
    /// <p>The ARN of the YAML-formatted test specification for the run.</p>
    #[serde(rename = "testSpecArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_spec_arn: Option<String>,
    /// <p>The total number of jobs for the run.</p>
    #[serde(rename = "totalJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_jobs: Option<i64>,
    /// <p><p>The run&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>BUILTIN<em>FUZZ</p> </li> <li> <p>BUILTIN</em>EXPLORER</p> <note> <p>For Android, an app explorer that traverses an Android app, interacting with it and capturing screenshots at the same time.</p> </note> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG</p> </li> <li> <p>APPIUM<em>PYTHON</p> </li> <li> <p>APPIUM</em>NODE</p> </li> <li> <p>APPIUM<em>RUBY</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</p> </li> <li> <p>APPIUM</em>WEB<em>NODE</p> </li> <li> <p>APPIUM</em>WEB<em>RUBY</p> </li> <li> <p>CALABASH</p> </li> <li> <p>INSTRUMENTATION</p> </li> <li> <p>UIAUTOMATION</p> </li> <li> <p>UIAUTOMATOR</p> </li> <li> <p>XCTEST</p> </li> <li> <p>XCTEST</em>UI</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The Device Farm console URL for the recording of the run.</p>
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

/// <p>Represents a sample of performance data.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Sample {
    /// <p>The sample's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p><p>The sample&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>CPU: A CPU sample type. This is expressed as the app processing CPU time (including child processes) as reported by process, as a percentage.</p> </li> <li> <p>MEMORY: A memory usage sample type. This is expressed as the total proportional set size of an app process, in kilobytes.</p> </li> <li> <p>NATIVE<em>AVG</em>DRAWTIME</p> </li> <li> <p>NATIVE<em>FPS</p> </li> <li> <p>NATIVE</em>FRAMES</p> </li> <li> <p>NATIVE<em>MAX</em>DRAWTIME</p> </li> <li> <p>NATIVE<em>MIN</em>DRAWTIME</p> </li> <li> <p>OPENGL<em>AVG</em>DRAWTIME</p> </li> <li> <p>OPENGL<em>FPS</p> </li> <li> <p>OPENGL</em>FRAMES</p> </li> <li> <p>OPENGL<em>MAX</em>DRAWTIME</p> </li> <li> <p>OPENGL<em>MIN</em>DRAWTIME</p> </li> <li> <p>RX</p> </li> <li> <p>RX<em>RATE: The total number of bytes per second (TCP and UDP) that are sent, by app process.</p> </li> <li> <p>THREADS: A threads sample type. This is expressed as the total number of threads per app process.</p> </li> <li> <p>TX</p> </li> <li> <p>TX</em>RATE: The total number of bytes per second (TCP and UDP) that are received, by app process.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The presigned Amazon S3 URL that can be used with a GET request to download the sample's file.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Represents the settings for a run. Includes things like location, radio states, auxiliary apps, and network profiles.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ScheduleRunConfiguration {
    /// <p>A list of upload ARNs for app packages to be installed with your app.</p>
    #[serde(rename = "auxiliaryApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_apps: Option<Vec<String>>,
    /// <p><p>Specifies the billing method for a test run: <code>metered</code> or <code>unmetered</code>. If the parameter is not specified, the default value is <code>metered</code>.</p> <note> <p>If you have purchased unmetered device slots, you must set this parameter to <code>unmetered</code> to make use of them. Otherwise, your run counts against your metered time.</p> </note></p>
    #[serde(rename = "billingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_method: Option<String>,
    /// <p>Input <code>CustomerArtifactPaths</code> object for the scheduled run configuration.</p>
    #[serde(rename = "customerArtifactPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_artifact_paths: Option<CustomerArtifactPaths>,
    /// <p>The ARN of the extra data for the run. The extra data is a .zip file that AWS Device Farm extracts to external data for Android or the app's sandbox for iOS.</p>
    #[serde(rename = "extraDataPackageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data_package_arn: Option<String>,
    /// <p>Information about the locale that is used for the run.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>Information about the location that is used for the run.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>Reserved for internal use.</p>
    #[serde(rename = "networkProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_arn: Option<String>,
    /// <p>Information about the radio states for the run.</p>
    #[serde(rename = "radios")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radios: Option<Radios>,
    /// <p>An array of ARNs for your VPC endpoint configurations.</p>
    #[serde(rename = "vpceConfigurationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_arns: Option<Vec<String>>,
}

/// <p>Represents a request to the schedule run operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ScheduleRunRequest {
    /// <p>The ARN of an application package to run tests against, created with <a>CreateUpload</a>. See <a>ListUploads</a>.</p>
    #[serde(rename = "appArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    /// <p>Information about the settings for the run to be scheduled.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ScheduleRunConfiguration>,
    /// <p>The ARN of the device pool for the run to be scheduled.</p>
    #[serde(rename = "devicePoolArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pool_arn: Option<String>,
    /// <p>The filter criteria used to dynamically select a set of devices for a test run and the maximum number of devices to be included in the run.</p> <p>Either <b> <code>devicePoolArn</code> </b> or <b> <code>deviceSelectionConfiguration</code> </b> is required in a request.</p>
    #[serde(rename = "deviceSelectionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_selection_configuration: Option<DeviceSelectionConfiguration>,
    /// <p>Specifies configuration information about a test run, such as the execution timeout (in minutes).</p>
    #[serde(rename = "executionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_configuration: Option<ExecutionConfiguration>,
    /// <p>The name for the run to be scheduled.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of the project for the run to be scheduled.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>Information about the test for the run to be scheduled.</p>
    #[serde(rename = "test")]
    pub test: ScheduleRunTest,
}

/// <p>Represents the result of a schedule run request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScheduleRunResult {
    /// <p>Information about the scheduled run.</p>
    #[serde(rename = "run")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<Run>,
}

/// <p>Represents test settings. This data structure is passed in as the test parameter to ScheduleRun. For an example of the JSON request syntax, see <a>ScheduleRun</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ScheduleRunTest {
    /// <p>The test's filter.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// <p><p>The test&#39;s parameters, such as test framework parameters and fixture settings. Parameters are represented by name-value pairs of strings.</p> <p>For all tests:</p> <ul> <li> <p> <code>app<em>performance</em>monitoring</code>: Performance monitoring is enabled by default. Set this parameter to false to disable it.</p> </li> </ul> <p>For Calabash tests:</p> <ul> <li> <p>profile: A cucumber profile (for example, <code>my<em>profile</em>name</code>).</p> </li> <li> <p>tags: You can limit execution to features or scenarios that have (or don&#39;t have) certain tags (for example, @smoke or @smoke,~@wip).</p> </li> </ul> <p>For Appium tests (all types):</p> <ul> <li> <p>appium<em>version: The Appium version. Currently supported values are 1.6.5 (and later), latest, and default.</p> <ul> <li> <p>latest runs the latest Appium version supported by Device Farm (1.9.1).</p> </li> <li> <p>For default, Device Farm selects a compatible version of Appium for the device. The current behavior is to run 1.7.2 on Android devices and iOS 9 and earlier and 1.7.2 for iOS 10 and later.</p> </li> <li> <p>This behavior is subject to change.</p> </li> </ul> </li> </ul> <p>For fuzz tests (Android only):</p> <ul> <li> <p>event</em>count: The number of events, between 1 and 10000, that the UI fuzz test should perform.</p> </li> <li> <p>throttle: The time, in ms, between 0 and 1000, that the UI fuzz test should wait between events.</p> </li> <li> <p>seed: A seed to use for randomizing the UI fuzz test. Using the same seed value between tests ensures identical event sequences.</p> </li> </ul> <p>For Explorer tests:</p> <ul> <li> <p>username: A user name to use if the Explorer encounters a login form. If not supplied, no user name is inserted.</p> </li> <li> <p>password: A password to use if the Explorer encounters a login form. If not supplied, no password is inserted.</p> </li> </ul> <p>For Instrumentation:</p> <ul> <li> <p>filter: A test filter string. Examples:</p> <ul> <li> <p>Running a single test case: <code>com.android.abc.Test1</code> </p> </li> <li> <p>Running a single test: <code>com.android.abc.Test1#smoke</code> </p> </li> <li> <p>Running multiple tests: <code>com.android.abc.Test1,com.android.abc.Test2</code> </p> </li> </ul> </li> </ul> <p>For XCTest and XCTestUI:</p> <ul> <li> <p>filter: A test filter string. Examples:</p> <ul> <li> <p>Running a single test class: <code>LoginTests</code> </p> </li> <li> <p>Running a multiple test classes: <code>LoginTests,SmokeTests</code> </p> </li> <li> <p>Running a single test: <code>LoginTests/testValid</code> </p> </li> <li> <p>Running multiple tests: <code>LoginTests/testValid,LoginTests/testInvalid</code> </p> </li> </ul> </li> </ul> <p>For UIAutomator:</p> <ul> <li> <p>filter: A test filter string. Examples:</p> <ul> <li> <p>Running a single test case: <code>com.android.abc.Test1</code> </p> </li> <li> <p>Running a single test: <code>com.android.abc.Test1#smoke</code> </p> </li> <li> <p>Running multiple tests: <code>com.android.abc.Test1,com.android.abc.Test2</code> </p> </li> </ul> </li> </ul></p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ARN of the uploaded test to be run.</p>
    #[serde(rename = "testPackageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_package_arn: Option<String>,
    /// <p>The ARN of the YAML-formatted test specification.</p>
    #[serde(rename = "testSpecArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_spec_arn: Option<String>,
    /// <p><p>The test&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>BUILTIN<em>FUZZ</p> </li> <li> <p>BUILTIN</em>EXPLORER. For Android, an app explorer that traverses an Android app, interacting with it and capturing screenshots at the same time.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG</p> </li> <li> <p>APPIUM<em>PYTHON</p> </li> <li> <p>APPIUM</em>NODE</p> </li> <li> <p>APPIUM<em>RUBY</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</p> </li> <li> <p>APPIUM</em>WEB<em>NODE</p> </li> <li> <p>APPIUM</em>WEB<em>RUBY</p> </li> <li> <p>CALABASH</p> </li> <li> <p>INSTRUMENTATION</p> </li> <li> <p>UIAUTOMATION</p> </li> <li> <p>UIAUTOMATOR</p> </li> <li> <p>XCTEST</p> </li> <li> <p>XCTEST</em>UI</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopJobRequest {
    /// <p>Represents the Amazon Resource Name (ARN) of the Device Farm job to stop.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopJobResult {
    /// <p>The job that was stopped.</p>
    #[serde(rename = "job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

/// <p>Represents the request to stop the remote access session.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopRemoteAccessSessionRequest {
    /// <p>The Amazon Resource Name (ARN) of the remote access session to stop.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the response from the server that describes the remote access session when AWS Device Farm stops the session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopRemoteAccessSessionResult {
    /// <p>A container that represents the metadata from the service about the remote access session you are stopping.</p>
    #[serde(rename = "remoteAccessSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_session: Option<RemoteAccessSession>,
}

/// <p>Represents the request to stop a specific run.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopRunRequest {
    /// <p>Represents the Amazon Resource Name (ARN) of the Device Farm run to stop.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the results of your stop run attempt.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopRunResult {
    /// <p>The run that was stopped.</p>
    #[serde(rename = "run")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<Run>,
}

/// <p>Represents a collection of one or more tests.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Suite {
    /// <p>The suite's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The suite's result counters.</p>
    #[serde(rename = "counters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Counters>,
    /// <p>When the suite was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Represents the total (metered or unmetered) minutes used by the test suite.</p>
    #[serde(rename = "deviceMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_minutes: Option<DeviceMinutes>,
    /// <p>A message about the suite's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The suite's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The suite&#39;s result.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>PASSED</p> </li> <li> <p>WARNED</p> </li> <li> <p>FAILED</p> </li> <li> <p>SKIPPED</p> </li> <li> <p>ERRORED</p> </li> <li> <p>STOPPED</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The suite's start time.</p>
    #[serde(rename = "started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
    /// <p><p>The suite&#39;s status.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>PENDING<em>CONCURRENCY</p> </li> <li> <p>PENDING</em>DEVICE</p> </li> <li> <p>PROCESSING</p> </li> <li> <p>SCHEDULING</p> </li> <li> <p>PREPARING</p> </li> <li> <p>RUNNING</p> </li> <li> <p>COMPLETED</p> </li> <li> <p>STOPPING</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The suite's stop time.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<f64>,
    /// <p><p>The suite&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>BUILTIN<em>FUZZ</p> </li> <li> <p>BUILTIN</em>EXPLORER </p> <note> <p>Only available for Android; an app explorer that traverses an Android app, interacting with it and capturing screenshots at the same time.</p> </note> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG</p> </li> <li> <p>APPIUM<em>PYTHON</p> </li> <li> <p>APPIUM</em>NODE</p> </li> <li> <p>APPIUM<em>RUBY</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</p> </li> <li> <p>APPIUM</em>WEB<em>NODE</p> </li> <li> <p>APPIUM</em>WEB<em>RUBY</p> </li> <li> <p>CALABASH</p> </li> <li> <p>INSTRUMENTATION</p> </li> <li> <p>UIAUTOMATION</p> </li> <li> <p>UIAUTOMATOR</p> </li> <li> <p>XCTEST</p> </li> <li> <p>XCTEST</em>UI</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The metadata that you apply to a resource to help you categorize and organize it. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters. Tag values can have a maximum length of 256 characters. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that makes up a tag. A <code>key</code> is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The optional part of a key-value pair that makes up a tag. A <code>value</code> acts as a descriptor in a tag category (key).</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource or resources to which to add tags. You can associate tags with the following Device Farm resources: <code>PROJECT</code>, <code>RUN</code>, <code>NETWORK_PROFILE</code>, <code>INSTANCE_PROFILE</code>, <code>DEVICE_INSTANCE</code>, <code>SESSION</code>, <code>DEVICE_POOL</code>, <code>DEVICE</code>, and <code>VPCE_CONFIGURATION</code>.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource. A tag is an array of key-value pairs. Tag keys can have a maximum character length of 128 characters. Tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Represents a condition that is evaluated.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Test {
    /// <p>The test's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The test's result counters.</p>
    #[serde(rename = "counters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Counters>,
    /// <p>When the test was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Represents the total (metered or unmetered) minutes used by the test.</p>
    #[serde(rename = "deviceMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_minutes: Option<DeviceMinutes>,
    /// <p>A message about the test's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The test's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The test&#39;s result.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>PASSED</p> </li> <li> <p>WARNED</p> </li> <li> <p>FAILED</p> </li> <li> <p>SKIPPED</p> </li> <li> <p>ERRORED</p> </li> <li> <p>STOPPED</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The test's start time.</p>
    #[serde(rename = "started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
    /// <p><p>The test&#39;s status.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>PENDING<em>CONCURRENCY</p> </li> <li> <p>PENDING</em>DEVICE</p> </li> <li> <p>PROCESSING</p> </li> <li> <p>SCHEDULING</p> </li> <li> <p>PREPARING</p> </li> <li> <p>RUNNING</p> </li> <li> <p>COMPLETED</p> </li> <li> <p>STOPPING</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The test's stop time.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<f64>,
    /// <p><p>The test&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>BUILTIN<em>FUZZ</p> </li> <li> <p>BUILTIN</em>EXPLORER</p> <note> <p>For Android, an app explorer that traverses an Android app, interacting with it and capturing screenshots at the same time.</p> </note> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG</p> </li> <li> <p>APPIUM<em>PYTHON</p> </li> <li> <p>APPIUM</em>NODE</p> </li> <li> <p>APPIUM<em>RUBY</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</p> </li> <li> <p>APPIUM</em>WEB<em>NODE</p> </li> <li> <p>APPIUM</em>WEB<em>RUBY</p> </li> <li> <p>CALABASH</p> </li> <li> <p>INSTRUMENTATION</p> </li> <li> <p>UIAUTOMATION</p> </li> <li> <p>UIAUTOMATOR</p> </li> <li> <p>XCTEST</p> </li> <li> <p>XCTEST</em>UI</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A Selenium testing project. Projects are used to collect and collate sessions.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestGridProject {
    /// <p>The ARN for the project.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>When the project was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>A human-readable description for the project.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A human-readable name for the project.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The VPC security groups and subnets that are attached to a project.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<TestGridVpcConfig>,
}

/// <p>A <a>TestGridSession</a> is a single instance of a browser launched from the URL provided by a call to <a>CreateTestGridUrl</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestGridSession {
    /// <p>The ARN of the session.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The number of billed minutes that were used for this session. </p>
    #[serde(rename = "billingMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_minutes: Option<f64>,
    /// <p>The time that the session was started.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The time the session ended.</p>
    #[serde(rename = "ended")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended: Option<f64>,
    /// <p>A JSON object of options and parameters passed to the Selenium WebDriver.</p>
    #[serde(rename = "seleniumProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selenium_properties: Option<String>,
    /// <p>The state of the session.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>An action taken by a <a>TestGridSession</a> browser instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestGridSessionAction {
    /// <p>The action taken by the session.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The time, in milliseconds, that the action took to complete in the browser.</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>HTTP method that the browser used to make the request.</p>
    #[serde(rename = "requestMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_method: Option<String>,
    /// <p>The time that the session invoked the action.</p>
    #[serde(rename = "started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
    /// <p>HTTP status code returned to the browser when the action was taken.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p><p>Artifacts are video and other files that are produced in the process of running a browser in an automated context. </p> <note> <p>Video elements might be broken up into multiple artifacts as they grow in size during creation. </p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestGridSessionArtifact {
    /// <p>The file name of the artifact.</p>
    #[serde(rename = "filename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// <p>The kind of artifact.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>A semi-stable URL to the content of the object.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>The VPC security groups and subnets that are attached to a project.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TestGridVpcConfig {
    /// <p>A list of VPC security group IDs in your Amazon VPC.</p>
    #[serde(rename = "securityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>A list of VPC subnet IDs in your Amazon VPC.</p>
    #[serde(rename = "subnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The ID of the Amazon VPC.</p>
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
}

/// <p>Represents information about free trial device minutes for an AWS account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrialMinutes {
    /// <p>The number of free trial minutes remaining in the account.</p>
    #[serde(rename = "remaining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<f64>,
    /// <p>The total number of free trial minutes that the account started with.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
}

/// <p>A collection of one or more problems, grouped by their result.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UniqueProblem {
    /// <p>A message about the unique problems' result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>Information about the problems.</p>
    #[serde(rename = "problems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problems: Option<Vec<Problem>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource or resources from which to delete tags. You can associate tags with the following Device Farm resources: <code>PROJECT</code>, <code>RUN</code>, <code>NETWORK_PROFILE</code>, <code>INSTANCE_PROFILE</code>, <code>DEVICE_INSTANCE</code>, <code>SESSION</code>, <code>DEVICE_POOL</code>, <code>DEVICE</code>, and <code>VPCE_CONFIGURATION</code>.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDeviceInstanceRequest {
    /// <p>The Amazon Resource Name (ARN) of the device instance.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An array of strings that you want to associate with the device instance.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>The ARN of the profile that you want to associate with the device instance.</p>
    #[serde(rename = "profileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDeviceInstanceResult {
    /// <p>An object that contains information about your device instance.</p>
    #[serde(rename = "deviceInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_instance: Option<DeviceInstance>,
}

/// <p>Represents a request to the update device pool operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDevicePoolRequest {
    /// <p>The Amazon Resource Name (ARN) of the Device Farm device pool to update.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>Sets whether the <code>maxDevices</code> parameter applies to your device pool. If you set this parameter to <code>true</code>, the <code>maxDevices</code> parameter does not apply, and Device Farm does not limit the number of devices that it adds to your device pool. In this case, Device Farm adds all available devices that meet the criteria specified in the <code>rules</code> parameter.</p> <p>If you use this parameter in your request, you cannot use the <code>maxDevices</code> parameter in the same request.</p>
    #[serde(rename = "clearMaxDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_max_devices: Option<bool>,
    /// <p>A description of the device pool to update.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The number of devices that Device Farm can add to your device pool. Device Farm adds devices that are available and that meet the criteria that you assign for the <code>rules</code> parameter. Depending on how many devices meet these constraints, your device pool might contain fewer devices than the value for this parameter.</p> <p>By specifying the maximum number of devices, you can control the costs that you incur by running tests.</p> <p>If you use this parameter in your request, you cannot use the <code>clearMaxDevices</code> parameter in the same request.</p>
    #[serde(rename = "maxDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_devices: Option<i64>,
    /// <p>A string that represents the name of the device pool to update.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Represents the rules to modify for the device pool. Updating rules is optional. If you update rules for your request, the update replaces the existing rules.</p>
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
}

/// <p>Represents the result of an update device pool request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDevicePoolResult {
    /// <p>The device pool you just updated.</p>
    #[serde(rename = "devicePool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pool: Option<DevicePool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInstanceProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The updated description for your instance profile.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An array of strings that specifies the list of app packages that should not be cleaned up from the device after a test run is over.</p> <p>The list of packages is only considered if you set <code>packageCleanup</code> to <code>true</code>.</p>
    #[serde(rename = "excludeAppPackagesFromCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_app_packages_from_cleanup: Option<Vec<String>>,
    /// <p>The updated name for your instance profile.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The updated choice for whether you want to specify package cleanup. The default value is <code>false</code> for private devices.</p>
    #[serde(rename = "packageCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_cleanup: Option<bool>,
    /// <p>The updated choice for whether you want to reboot the device after use. The default value is <code>true</code>.</p>
    #[serde(rename = "rebootAfterUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_after_use: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateInstanceProfileResult {
    /// <p>An object that contains information about your instance profile.</p>
    #[serde(rename = "instanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateNetworkProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to update network profile settings.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The description of the network profile about which you are returning information.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "downlinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_jitter_ms: Option<i64>,
    /// <p>Proportion of received packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "downlinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_loss_percent: Option<i64>,
    /// <p>The name of the network profile about which you are returning information.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of network profile to return information about. Valid values are listed here.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "uplinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_jitter_ms: Option<i64>,
    /// <p>Proportion of transmitted packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "uplinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_loss_percent: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNetworkProfileResult {
    /// <p>A list of the available network profiles.</p>
    #[serde(rename = "networkProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
}

/// <p>Represents a request to the update project operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProjectRequest {
    /// <p>The Amazon Resource Name (ARN) of the project whose name to update.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The number of minutes a test run in the project executes before it times out.</p>
    #[serde(rename = "defaultJobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_job_timeout_minutes: Option<i64>,
    /// <p>A string that represents the new name of the project that you are updating.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents the result of an update project request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProjectResult {
    /// <p>The project to update.</p>
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTestGridProjectRequest {
    /// <p>Human-readable description for the project.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Human-readable name for the project.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>ARN of the project to update.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>The VPC security groups and subnets that are attached to a project.</p>
    #[serde(rename = "vpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<TestGridVpcConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTestGridProjectResult {
    /// <p>The project, including updated information.</p>
    #[serde(rename = "testGridProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_grid_project: Option<TestGridProject>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUploadRequest {
    /// <p>The Amazon Resource Name (ARN) of the uploaded test spec.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The upload's content type (for example, <code>application/x-yaml</code>).</p>
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>Set to true if the YAML file has changed and must be updated. Otherwise, set to false.</p>
    #[serde(rename = "editContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_content: Option<bool>,
    /// <p>The upload's test spec file name. The name must not contain any forward slashes (/). The test spec file name must end with the <code>.yaml</code> or <code>.yml</code> file extension.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUploadResult {
    /// <p>A test spec uploaded to Device Farm.</p>
    #[serde(rename = "upload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Upload>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVPCEConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration you want to update.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The DNS (domain) name used to connect to your private service in your VPC. The DNS name must not already be in use on the internet.</p>
    #[serde(rename = "serviceDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_dns_name: Option<String>,
    /// <p>An optional description that provides details about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_description: Option<String>,
    /// <p>The friendly name you give to your VPC endpoint configuration to manage your configurations more easily.</p>
    #[serde(rename = "vpceConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_name: Option<String>,
    /// <p>The name of the VPC endpoint service running in your AWS account that you want Device Farm to test.</p>
    #[serde(rename = "vpceServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_service_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVPCEConfigurationResult {
    /// <p>An object that contains information about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration: Option<VPCEConfiguration>,
}

/// <p>An app or a set of one or more tests to upload or that have been uploaded.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Upload {
    /// <p>The upload's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p><p>The upload&#39;s category. Allowed values include:</p> <ul> <li> <p>CURATED: An upload managed by AWS Device Farm.</p> </li> <li> <p>PRIVATE: An upload managed by the AWS Device Farm customer.</p> </li> </ul></p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The upload's content type (for example, <code>application/octet-stream</code>).</p>
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>When the upload was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>A message about the upload's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The upload's metadata. For example, for Android, this contains information that is parsed from the manifest and is displayed in the AWS Device Farm console after the associated app is uploaded.</p>
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// <p>The upload's file name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The upload&#39;s status.</p> <p>Must be one of the following values:</p> <ul> <li> <p>FAILED</p> </li> <li> <p>INITIALIZED</p> </li> <li> <p>PROCESSING</p> </li> <li> <p>SUCCEEDED</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The upload&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>ANDROID<em>APP</p> </li> <li> <p>IOS</em>APP</p> </li> <li> <p>WEB<em>APP</p> </li> <li> <p>EXTERNAL</em>DATA</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT<em>TEST</em>PACKAGE</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG<em>TEST</em>PACKAGE</p> </li> <li> <p>APPIUM<em>PYTHON</em>TEST<em>PACKAGE</p> </li> <li> <p>APPIUM</em>NODE<em>TEST</em>PACKAGE</p> </li> <li> <p>APPIUM<em>RUBY</em>TEST<em>PACKAGE</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT<em>TEST</em>PACKAGE</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</em>TEST<em>PACKAGE</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</em>TEST<em>PACKAGE</p> </li> <li> <p>APPIUM</em>WEB<em>NODE</em>TEST<em>PACKAGE</p> </li> <li> <p>APPIUM</em>WEB<em>RUBY</em>TEST<em>PACKAGE</p> </li> <li> <p>CALABASH</em>TEST<em>PACKAGE</p> </li> <li> <p>INSTRUMENTATION</em>TEST<em>PACKAGE</p> </li> <li> <p>UIAUTOMATION</em>TEST<em>PACKAGE</p> </li> <li> <p>UIAUTOMATOR</em>TEST<em>PACKAGE</p> </li> <li> <p>XCTEST</em>TEST<em>PACKAGE</p> </li> <li> <p>XCTEST</em>UI<em>TEST</em>PACKAGE</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT<em>TEST</em>SPEC</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG<em>TEST</em>SPEC</p> </li> <li> <p>APPIUM<em>PYTHON</em>TEST<em>SPEC</p> </li> <li> <p>APPIUM</em>NODE<em>TEST</em>SPEC</p> </li> <li> <p>APPIUM<em>RUBY</em>TEST<em>SPEC</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT<em>TEST</em>SPEC</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</em>TEST<em>SPEC</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</em>TEST<em>SPEC</p> </li> <li> <p>APPIUM</em>WEB<em>NODE</em>TEST<em>SPEC</p> </li> <li> <p>APPIUM</em>WEB<em>RUBY</em>TEST<em>SPEC</p> </li> <li> <p>INSTRUMENTATION</em>TEST<em>SPEC</p> </li> <li> <p>XCTEST</em>UI<em>TEST</em>SPEC</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The presigned Amazon S3 URL that was used to store a file using a PUT request.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Represents an Amazon Virtual Private Cloud (VPC) endpoint configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VPCEConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The DNS name that maps to the private IP address of the service you want to access.</p>
    #[serde(rename = "serviceDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_dns_name: Option<String>,
    /// <p>An optional description that provides details about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_description: Option<String>,
    /// <p>The friendly name you give to your VPC endpoint configuration to manage your configurations more easily.</p>
    #[serde(rename = "vpceConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_name: Option<String>,
    /// <p>The name of the VPC endpoint service running in your AWS account that you want Device Farm to test.</p>
    #[serde(rename = "vpceServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_service_name: Option<String>,
}

/// Errors returned by CreateDevicePool
#[derive(Debug, PartialEq)]
pub enum CreateDevicePoolError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl CreateDevicePoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDevicePoolError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(CreateDevicePoolError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDevicePoolError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDevicePoolError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(CreateDevicePoolError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDevicePoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDevicePoolError::Argument(ref cause) => write!(f, "{}", cause),
            CreateDevicePoolError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDevicePoolError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateDevicePoolError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDevicePoolError {}
/// Errors returned by CreateInstanceProfile
#[derive(Debug, PartialEq)]
pub enum CreateInstanceProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl CreateInstanceProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInstanceProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(CreateInstanceProfileError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateInstanceProfileError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateInstanceProfileError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(CreateInstanceProfileError::ServiceAccount(
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
impl fmt::Display for CreateInstanceProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInstanceProfileError::Argument(ref cause) => write!(f, "{}", cause),
            CreateInstanceProfileError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateInstanceProfileError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateInstanceProfileError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInstanceProfileError {}
/// Errors returned by CreateNetworkProfile
#[derive(Debug, PartialEq)]
pub enum CreateNetworkProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl CreateNetworkProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateNetworkProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(CreateNetworkProfileError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateNetworkProfileError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateNetworkProfileError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(CreateNetworkProfileError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateNetworkProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNetworkProfileError::Argument(ref cause) => write!(f, "{}", cause),
            CreateNetworkProfileError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateNetworkProfileError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateNetworkProfileError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateNetworkProfileError {}
/// Errors returned by CreateProject
#[derive(Debug, PartialEq)]
pub enum CreateProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// <p>The operation was not successful. Try again.</p>
    TagOperation(String),
}

impl CreateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(CreateProjectError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateProjectError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateProjectError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(CreateProjectError::ServiceAccount(err.msg))
                }
                "TagOperationException" => {
                    return RusotoError::Service(CreateProjectError::TagOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProjectError::Argument(ref cause) => write!(f, "{}", cause),
            CreateProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateProjectError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateProjectError::ServiceAccount(ref cause) => write!(f, "{}", cause),
            CreateProjectError::TagOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProjectError {}
/// Errors returned by CreateRemoteAccessSession
#[derive(Debug, PartialEq)]
pub enum CreateRemoteAccessSessionError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl CreateRemoteAccessSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRemoteAccessSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(CreateRemoteAccessSessionError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateRemoteAccessSessionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateRemoteAccessSessionError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(CreateRemoteAccessSessionError::ServiceAccount(
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
impl fmt::Display for CreateRemoteAccessSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRemoteAccessSessionError::Argument(ref cause) => write!(f, "{}", cause),
            CreateRemoteAccessSessionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateRemoteAccessSessionError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateRemoteAccessSessionError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRemoteAccessSessionError {}
/// Errors returned by CreateTestGridProject
#[derive(Debug, PartialEq)]
pub enum CreateTestGridProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>An internal exception was raised in the service. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you see this error. </p>
    InternalService(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
}

impl CreateTestGridProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTestGridProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(CreateTestGridProjectError::Argument(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateTestGridProjectError::InternalService(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateTestGridProjectError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTestGridProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTestGridProjectError::Argument(ref cause) => write!(f, "{}", cause),
            CreateTestGridProjectError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateTestGridProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTestGridProjectError {}
/// Errors returned by CreateTestGridUrl
#[derive(Debug, PartialEq)]
pub enum CreateTestGridUrlError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>An internal exception was raised in the service. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you see this error. </p>
    InternalService(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
}

impl CreateTestGridUrlError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTestGridUrlError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(CreateTestGridUrlError::Argument(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateTestGridUrlError::InternalService(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateTestGridUrlError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTestGridUrlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTestGridUrlError::Argument(ref cause) => write!(f, "{}", cause),
            CreateTestGridUrlError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateTestGridUrlError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTestGridUrlError {}
/// Errors returned by CreateUpload
#[derive(Debug, PartialEq)]
pub enum CreateUploadError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl CreateUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUploadError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(CreateUploadError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUploadError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateUploadError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(CreateUploadError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUploadError::Argument(ref cause) => write!(f, "{}", cause),
            CreateUploadError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateUploadError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateUploadError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUploadError {}
/// Errors returned by CreateVPCEConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateVPCEConfigurationError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl CreateVPCEConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVPCEConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(CreateVPCEConfigurationError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateVPCEConfigurationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(CreateVPCEConfigurationError::ServiceAccount(
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
impl fmt::Display for CreateVPCEConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVPCEConfigurationError::Argument(ref cause) => write!(f, "{}", cause),
            CreateVPCEConfigurationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateVPCEConfigurationError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVPCEConfigurationError {}
/// Errors returned by DeleteDevicePool
#[derive(Debug, PartialEq)]
pub enum DeleteDevicePoolError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl DeleteDevicePoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDevicePoolError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(DeleteDevicePoolError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteDevicePoolError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDevicePoolError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(DeleteDevicePoolError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDevicePoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDevicePoolError::Argument(ref cause) => write!(f, "{}", cause),
            DeleteDevicePoolError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteDevicePoolError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteDevicePoolError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDevicePoolError {}
/// Errors returned by DeleteInstanceProfile
#[derive(Debug, PartialEq)]
pub enum DeleteInstanceProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl DeleteInstanceProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInstanceProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(DeleteInstanceProfileError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteInstanceProfileError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteInstanceProfileError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(DeleteInstanceProfileError::ServiceAccount(
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
impl fmt::Display for DeleteInstanceProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInstanceProfileError::Argument(ref cause) => write!(f, "{}", cause),
            DeleteInstanceProfileError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteInstanceProfileError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteInstanceProfileError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInstanceProfileError {}
/// Errors returned by DeleteNetworkProfile
#[derive(Debug, PartialEq)]
pub enum DeleteNetworkProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl DeleteNetworkProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNetworkProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(DeleteNetworkProfileError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteNetworkProfileError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteNetworkProfileError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(DeleteNetworkProfileError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteNetworkProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNetworkProfileError::Argument(ref cause) => write!(f, "{}", cause),
            DeleteNetworkProfileError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteNetworkProfileError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteNetworkProfileError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteNetworkProfileError {}
/// Errors returned by DeleteProject
#[derive(Debug, PartialEq)]
pub enum DeleteProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl DeleteProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(DeleteProjectError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteProjectError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteProjectError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(DeleteProjectError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProjectError::Argument(ref cause) => write!(f, "{}", cause),
            DeleteProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteProjectError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteProjectError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProjectError {}
/// Errors returned by DeleteRemoteAccessSession
#[derive(Debug, PartialEq)]
pub enum DeleteRemoteAccessSessionError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl DeleteRemoteAccessSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRemoteAccessSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(DeleteRemoteAccessSessionError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteRemoteAccessSessionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRemoteAccessSessionError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(DeleteRemoteAccessSessionError::ServiceAccount(
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
impl fmt::Display for DeleteRemoteAccessSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRemoteAccessSessionError::Argument(ref cause) => write!(f, "{}", cause),
            DeleteRemoteAccessSessionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteRemoteAccessSessionError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteRemoteAccessSessionError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRemoteAccessSessionError {}
/// Errors returned by DeleteRun
#[derive(Debug, PartialEq)]
pub enum DeleteRunError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl DeleteRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(DeleteRunError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteRunError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRunError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(DeleteRunError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRunError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRunError::Argument(ref cause) => write!(f, "{}", cause),
            DeleteRunError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteRunError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteRunError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRunError {}
/// Errors returned by DeleteTestGridProject
#[derive(Debug, PartialEq)]
pub enum DeleteTestGridProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>The requested object could not be deleted.</p>
    CannotDelete(String),
    /// <p>An internal exception was raised in the service. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you see this error. </p>
    InternalService(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
}

impl DeleteTestGridProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTestGridProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(DeleteTestGridProjectError::Argument(err.msg))
                }
                "CannotDeleteException" => {
                    return RusotoError::Service(DeleteTestGridProjectError::CannotDelete(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteTestGridProjectError::InternalService(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteTestGridProjectError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTestGridProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTestGridProjectError::Argument(ref cause) => write!(f, "{}", cause),
            DeleteTestGridProjectError::CannotDelete(ref cause) => write!(f, "{}", cause),
            DeleteTestGridProjectError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteTestGridProjectError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTestGridProjectError {}
/// Errors returned by DeleteUpload
#[derive(Debug, PartialEq)]
pub enum DeleteUploadError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl DeleteUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUploadError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(DeleteUploadError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteUploadError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteUploadError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(DeleteUploadError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUploadError::Argument(ref cause) => write!(f, "{}", cause),
            DeleteUploadError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteUploadError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteUploadError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUploadError {}
/// Errors returned by DeleteVPCEConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteVPCEConfigurationError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>There was an error with the update request, or you do not have sufficient permissions to update this VPC endpoint configuration.</p>
    InvalidOperation(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl DeleteVPCEConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVPCEConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(DeleteVPCEConfigurationError::Argument(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(DeleteVPCEConfigurationError::InvalidOperation(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVPCEConfigurationError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(DeleteVPCEConfigurationError::ServiceAccount(
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
impl fmt::Display for DeleteVPCEConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVPCEConfigurationError::Argument(ref cause) => write!(f, "{}", cause),
            DeleteVPCEConfigurationError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            DeleteVPCEConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVPCEConfigurationError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVPCEConfigurationError {}
/// Errors returned by GetAccountSettings
#[derive(Debug, PartialEq)]
pub enum GetAccountSettingsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetAccountSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccountSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetAccountSettingsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetAccountSettingsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetAccountSettingsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetAccountSettingsError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAccountSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAccountSettingsError::Argument(ref cause) => write!(f, "{}", cause),
            GetAccountSettingsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetAccountSettingsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetAccountSettingsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAccountSettingsError {}
/// Errors returned by GetDevice
#[derive(Debug, PartialEq)]
pub enum GetDeviceError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeviceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetDeviceError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetDeviceError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDeviceError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetDeviceError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeviceError::Argument(ref cause) => write!(f, "{}", cause),
            GetDeviceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetDeviceError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDeviceError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeviceError {}
/// Errors returned by GetDeviceInstance
#[derive(Debug, PartialEq)]
pub enum GetDeviceInstanceError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetDeviceInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeviceInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetDeviceInstanceError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetDeviceInstanceError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDeviceInstanceError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetDeviceInstanceError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDeviceInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeviceInstanceError::Argument(ref cause) => write!(f, "{}", cause),
            GetDeviceInstanceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetDeviceInstanceError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDeviceInstanceError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeviceInstanceError {}
/// Errors returned by GetDevicePool
#[derive(Debug, PartialEq)]
pub enum GetDevicePoolError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetDevicePoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDevicePoolError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetDevicePoolError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetDevicePoolError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDevicePoolError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetDevicePoolError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDevicePoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDevicePoolError::Argument(ref cause) => write!(f, "{}", cause),
            GetDevicePoolError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetDevicePoolError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDevicePoolError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDevicePoolError {}
/// Errors returned by GetDevicePoolCompatibility
#[derive(Debug, PartialEq)]
pub enum GetDevicePoolCompatibilityError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetDevicePoolCompatibilityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDevicePoolCompatibilityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetDevicePoolCompatibilityError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetDevicePoolCompatibilityError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDevicePoolCompatibilityError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetDevicePoolCompatibilityError::ServiceAccount(
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
impl fmt::Display for GetDevicePoolCompatibilityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDevicePoolCompatibilityError::Argument(ref cause) => write!(f, "{}", cause),
            GetDevicePoolCompatibilityError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetDevicePoolCompatibilityError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDevicePoolCompatibilityError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDevicePoolCompatibilityError {}
/// Errors returned by GetInstanceProfile
#[derive(Debug, PartialEq)]
pub enum GetInstanceProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetInstanceProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetInstanceProfileError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetInstanceProfileError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceProfileError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetInstanceProfileError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInstanceProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstanceProfileError::Argument(ref cause) => write!(f, "{}", cause),
            GetInstanceProfileError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetInstanceProfileError::NotFound(ref cause) => write!(f, "{}", cause),
            GetInstanceProfileError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstanceProfileError {}
/// Errors returned by GetJob
#[derive(Debug, PartialEq)]
pub enum GetJobError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => return RusotoError::Service(GetJobError::Argument(err.msg)),
                "LimitExceededException" => {
                    return RusotoError::Service(GetJobError::LimitExceeded(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(GetJobError::NotFound(err.msg)),
                "ServiceAccountException" => {
                    return RusotoError::Service(GetJobError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJobError::Argument(ref cause) => write!(f, "{}", cause),
            GetJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetJobError::NotFound(ref cause) => write!(f, "{}", cause),
            GetJobError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetJobError {}
/// Errors returned by GetNetworkProfile
#[derive(Debug, PartialEq)]
pub enum GetNetworkProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetNetworkProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetNetworkProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetNetworkProfileError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetNetworkProfileError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetNetworkProfileError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetNetworkProfileError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetNetworkProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetNetworkProfileError::Argument(ref cause) => write!(f, "{}", cause),
            GetNetworkProfileError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetNetworkProfileError::NotFound(ref cause) => write!(f, "{}", cause),
            GetNetworkProfileError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetNetworkProfileError {}
/// Errors returned by GetOfferingStatus
#[derive(Debug, PartialEq)]
pub enum GetOfferingStatusError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetOfferingStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOfferingStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetOfferingStatusError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetOfferingStatusError::LimitExceeded(err.msg))
                }
                "NotEligibleException" => {
                    return RusotoError::Service(GetOfferingStatusError::NotEligible(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetOfferingStatusError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetOfferingStatusError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOfferingStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOfferingStatusError::Argument(ref cause) => write!(f, "{}", cause),
            GetOfferingStatusError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetOfferingStatusError::NotEligible(ref cause) => write!(f, "{}", cause),
            GetOfferingStatusError::NotFound(ref cause) => write!(f, "{}", cause),
            GetOfferingStatusError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOfferingStatusError {}
/// Errors returned by GetProject
#[derive(Debug, PartialEq)]
pub enum GetProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetProjectError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetProjectError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetProjectError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetProjectError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetProjectError::Argument(ref cause) => write!(f, "{}", cause),
            GetProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetProjectError::NotFound(ref cause) => write!(f, "{}", cause),
            GetProjectError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetProjectError {}
/// Errors returned by GetRemoteAccessSession
#[derive(Debug, PartialEq)]
pub enum GetRemoteAccessSessionError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetRemoteAccessSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRemoteAccessSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetRemoteAccessSessionError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetRemoteAccessSessionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRemoteAccessSessionError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetRemoteAccessSessionError::ServiceAccount(
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
impl fmt::Display for GetRemoteAccessSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRemoteAccessSessionError::Argument(ref cause) => write!(f, "{}", cause),
            GetRemoteAccessSessionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetRemoteAccessSessionError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRemoteAccessSessionError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRemoteAccessSessionError {}
/// Errors returned by GetRun
#[derive(Debug, PartialEq)]
pub enum GetRunError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => return RusotoError::Service(GetRunError::Argument(err.msg)),
                "LimitExceededException" => {
                    return RusotoError::Service(GetRunError::LimitExceeded(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(GetRunError::NotFound(err.msg)),
                "ServiceAccountException" => {
                    return RusotoError::Service(GetRunError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRunError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRunError::Argument(ref cause) => write!(f, "{}", cause),
            GetRunError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetRunError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRunError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRunError {}
/// Errors returned by GetSuite
#[derive(Debug, PartialEq)]
pub enum GetSuiteError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetSuiteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSuiteError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetSuiteError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetSuiteError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSuiteError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetSuiteError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSuiteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSuiteError::Argument(ref cause) => write!(f, "{}", cause),
            GetSuiteError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetSuiteError::NotFound(ref cause) => write!(f, "{}", cause),
            GetSuiteError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSuiteError {}
/// Errors returned by GetTest
#[derive(Debug, PartialEq)]
pub enum GetTestError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetTestError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetTestError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetTestError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetTestError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetTestError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTestError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTestError::Argument(ref cause) => write!(f, "{}", cause),
            GetTestError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetTestError::NotFound(ref cause) => write!(f, "{}", cause),
            GetTestError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTestError {}
/// Errors returned by GetTestGridProject
#[derive(Debug, PartialEq)]
pub enum GetTestGridProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>An internal exception was raised in the service. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you see this error. </p>
    InternalService(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
}

impl GetTestGridProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTestGridProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetTestGridProjectError::Argument(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTestGridProjectError::InternalService(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetTestGridProjectError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTestGridProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTestGridProjectError::Argument(ref cause) => write!(f, "{}", cause),
            GetTestGridProjectError::InternalService(ref cause) => write!(f, "{}", cause),
            GetTestGridProjectError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTestGridProjectError {}
/// Errors returned by GetTestGridSession
#[derive(Debug, PartialEq)]
pub enum GetTestGridSessionError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>An internal exception was raised in the service. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you see this error. </p>
    InternalService(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
}

impl GetTestGridSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTestGridSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetTestGridSessionError::Argument(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTestGridSessionError::InternalService(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetTestGridSessionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTestGridSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTestGridSessionError::Argument(ref cause) => write!(f, "{}", cause),
            GetTestGridSessionError::InternalService(ref cause) => write!(f, "{}", cause),
            GetTestGridSessionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTestGridSessionError {}
/// Errors returned by GetUpload
#[derive(Debug, PartialEq)]
pub enum GetUploadError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUploadError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetUploadError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetUploadError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetUploadError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetUploadError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUploadError::Argument(ref cause) => write!(f, "{}", cause),
            GetUploadError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetUploadError::NotFound(ref cause) => write!(f, "{}", cause),
            GetUploadError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUploadError {}
/// Errors returned by GetVPCEConfiguration
#[derive(Debug, PartialEq)]
pub enum GetVPCEConfigurationError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl GetVPCEConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVPCEConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(GetVPCEConfigurationError::Argument(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVPCEConfigurationError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(GetVPCEConfigurationError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVPCEConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVPCEConfigurationError::Argument(ref cause) => write!(f, "{}", cause),
            GetVPCEConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            GetVPCEConfigurationError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVPCEConfigurationError {}
/// Errors returned by InstallToRemoteAccessSession
#[derive(Debug, PartialEq)]
pub enum InstallToRemoteAccessSessionError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl InstallToRemoteAccessSessionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<InstallToRemoteAccessSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(InstallToRemoteAccessSessionError::Argument(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(InstallToRemoteAccessSessionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(InstallToRemoteAccessSessionError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(InstallToRemoteAccessSessionError::ServiceAccount(
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
impl fmt::Display for InstallToRemoteAccessSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InstallToRemoteAccessSessionError::Argument(ref cause) => write!(f, "{}", cause),
            InstallToRemoteAccessSessionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            InstallToRemoteAccessSessionError::NotFound(ref cause) => write!(f, "{}", cause),
            InstallToRemoteAccessSessionError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InstallToRemoteAccessSessionError {}
/// Errors returned by ListArtifacts
#[derive(Debug, PartialEq)]
pub enum ListArtifactsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListArtifactsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListArtifactsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListArtifactsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListArtifactsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListArtifactsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListArtifactsError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListArtifactsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListArtifactsError::Argument(ref cause) => write!(f, "{}", cause),
            ListArtifactsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListArtifactsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListArtifactsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListArtifactsError {}
/// Errors returned by ListDeviceInstances
#[derive(Debug, PartialEq)]
pub enum ListDeviceInstancesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListDeviceInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeviceInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListDeviceInstancesError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListDeviceInstancesError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListDeviceInstancesError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListDeviceInstancesError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDeviceInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeviceInstancesError::Argument(ref cause) => write!(f, "{}", cause),
            ListDeviceInstancesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListDeviceInstancesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListDeviceInstancesError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDeviceInstancesError {}
/// Errors returned by ListDevicePools
#[derive(Debug, PartialEq)]
pub enum ListDevicePoolsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListDevicePoolsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDevicePoolsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListDevicePoolsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListDevicePoolsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListDevicePoolsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListDevicePoolsError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDevicePoolsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDevicePoolsError::Argument(ref cause) => write!(f, "{}", cause),
            ListDevicePoolsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListDevicePoolsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListDevicePoolsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDevicePoolsError {}
/// Errors returned by ListDevices
#[derive(Debug, PartialEq)]
pub enum ListDevicesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListDevicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDevicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListDevicesError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListDevicesError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListDevicesError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListDevicesError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDevicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDevicesError::Argument(ref cause) => write!(f, "{}", cause),
            ListDevicesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListDevicesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListDevicesError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDevicesError {}
/// Errors returned by ListInstanceProfiles
#[derive(Debug, PartialEq)]
pub enum ListInstanceProfilesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListInstanceProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInstanceProfilesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListInstanceProfilesError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListInstanceProfilesError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListInstanceProfilesError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListInstanceProfilesError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInstanceProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInstanceProfilesError::Argument(ref cause) => write!(f, "{}", cause),
            ListInstanceProfilesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListInstanceProfilesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListInstanceProfilesError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInstanceProfilesError {}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListJobsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListJobsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListJobsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListJobsError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListJobsError::Argument(ref cause) => write!(f, "{}", cause),
            ListJobsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListJobsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListJobsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobsError {}
/// Errors returned by ListNetworkProfiles
#[derive(Debug, PartialEq)]
pub enum ListNetworkProfilesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListNetworkProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListNetworkProfilesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListNetworkProfilesError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListNetworkProfilesError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListNetworkProfilesError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListNetworkProfilesError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListNetworkProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListNetworkProfilesError::Argument(ref cause) => write!(f, "{}", cause),
            ListNetworkProfilesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListNetworkProfilesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListNetworkProfilesError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListNetworkProfilesError {}
/// Errors returned by ListOfferingPromotions
#[derive(Debug, PartialEq)]
pub enum ListOfferingPromotionsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListOfferingPromotionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOfferingPromotionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListOfferingPromotionsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListOfferingPromotionsError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotEligibleException" => {
                    return RusotoError::Service(ListOfferingPromotionsError::NotEligible(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListOfferingPromotionsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListOfferingPromotionsError::ServiceAccount(
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
impl fmt::Display for ListOfferingPromotionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOfferingPromotionsError::Argument(ref cause) => write!(f, "{}", cause),
            ListOfferingPromotionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListOfferingPromotionsError::NotEligible(ref cause) => write!(f, "{}", cause),
            ListOfferingPromotionsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListOfferingPromotionsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOfferingPromotionsError {}
/// Errors returned by ListOfferingTransactions
#[derive(Debug, PartialEq)]
pub enum ListOfferingTransactionsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListOfferingTransactionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOfferingTransactionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListOfferingTransactionsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListOfferingTransactionsError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotEligibleException" => {
                    return RusotoError::Service(ListOfferingTransactionsError::NotEligible(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListOfferingTransactionsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListOfferingTransactionsError::ServiceAccount(
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
impl fmt::Display for ListOfferingTransactionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOfferingTransactionsError::Argument(ref cause) => write!(f, "{}", cause),
            ListOfferingTransactionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListOfferingTransactionsError::NotEligible(ref cause) => write!(f, "{}", cause),
            ListOfferingTransactionsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListOfferingTransactionsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOfferingTransactionsError {}
/// Errors returned by ListOfferings
#[derive(Debug, PartialEq)]
pub enum ListOfferingsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListOfferingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOfferingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListOfferingsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListOfferingsError::LimitExceeded(err.msg))
                }
                "NotEligibleException" => {
                    return RusotoError::Service(ListOfferingsError::NotEligible(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListOfferingsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListOfferingsError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOfferingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOfferingsError::Argument(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::NotEligible(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOfferingsError {}
/// Errors returned by ListProjects
#[derive(Debug, PartialEq)]
pub enum ListProjectsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListProjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProjectsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListProjectsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListProjectsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListProjectsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListProjectsError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProjectsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProjectsError::Argument(ref cause) => write!(f, "{}", cause),
            ListProjectsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListProjectsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListProjectsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProjectsError {}
/// Errors returned by ListRemoteAccessSessions
#[derive(Debug, PartialEq)]
pub enum ListRemoteAccessSessionsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListRemoteAccessSessionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRemoteAccessSessionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListRemoteAccessSessionsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListRemoteAccessSessionsError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListRemoteAccessSessionsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListRemoteAccessSessionsError::ServiceAccount(
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
impl fmt::Display for ListRemoteAccessSessionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRemoteAccessSessionsError::Argument(ref cause) => write!(f, "{}", cause),
            ListRemoteAccessSessionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListRemoteAccessSessionsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListRemoteAccessSessionsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRemoteAccessSessionsError {}
/// Errors returned by ListRuns
#[derive(Debug, PartialEq)]
pub enum ListRunsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRunsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListRunsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListRunsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListRunsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListRunsError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRunsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRunsError::Argument(ref cause) => write!(f, "{}", cause),
            ListRunsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListRunsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListRunsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRunsError {}
/// Errors returned by ListSamples
#[derive(Debug, PartialEq)]
pub enum ListSamplesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListSamplesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSamplesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListSamplesError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListSamplesError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListSamplesError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListSamplesError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSamplesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSamplesError::Argument(ref cause) => write!(f, "{}", cause),
            ListSamplesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListSamplesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListSamplesError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSamplesError {}
/// Errors returned by ListSuites
#[derive(Debug, PartialEq)]
pub enum ListSuitesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListSuitesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSuitesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListSuitesError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListSuitesError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListSuitesError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListSuitesError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSuitesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSuitesError::Argument(ref cause) => write!(f, "{}", cause),
            ListSuitesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListSuitesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListSuitesError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSuitesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>The operation was not successful. Try again.</p>
    TagOperation(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListTagsForResourceError::Argument(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
                }
                "TagOperationException" => {
                    return RusotoError::Service(ListTagsForResourceError::TagOperation(err.msg))
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
            ListTagsForResourceError::Argument(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TagOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTestGridProjects
#[derive(Debug, PartialEq)]
pub enum ListTestGridProjectsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>An internal exception was raised in the service. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you see this error. </p>
    InternalService(String),
}

impl ListTestGridProjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTestGridProjectsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListTestGridProjectsError::Argument(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListTestGridProjectsError::InternalService(
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
impl fmt::Display for ListTestGridProjectsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTestGridProjectsError::Argument(ref cause) => write!(f, "{}", cause),
            ListTestGridProjectsError::InternalService(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTestGridProjectsError {}
/// Errors returned by ListTestGridSessionActions
#[derive(Debug, PartialEq)]
pub enum ListTestGridSessionActionsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>An internal exception was raised in the service. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you see this error. </p>
    InternalService(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
}

impl ListTestGridSessionActionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListTestGridSessionActionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListTestGridSessionActionsError::Argument(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListTestGridSessionActionsError::InternalService(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTestGridSessionActionsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTestGridSessionActionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTestGridSessionActionsError::Argument(ref cause) => write!(f, "{}", cause),
            ListTestGridSessionActionsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListTestGridSessionActionsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTestGridSessionActionsError {}
/// Errors returned by ListTestGridSessionArtifacts
#[derive(Debug, PartialEq)]
pub enum ListTestGridSessionArtifactsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>An internal exception was raised in the service. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you see this error. </p>
    InternalService(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
}

impl ListTestGridSessionArtifactsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListTestGridSessionArtifactsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListTestGridSessionArtifactsError::Argument(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(
                        ListTestGridSessionArtifactsError::InternalService(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTestGridSessionArtifactsError::NotFound(
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
impl fmt::Display for ListTestGridSessionArtifactsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTestGridSessionArtifactsError::Argument(ref cause) => write!(f, "{}", cause),
            ListTestGridSessionArtifactsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListTestGridSessionArtifactsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTestGridSessionArtifactsError {}
/// Errors returned by ListTestGridSessions
#[derive(Debug, PartialEq)]
pub enum ListTestGridSessionsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>An internal exception was raised in the service. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you see this error. </p>
    InternalService(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
}

impl ListTestGridSessionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTestGridSessionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListTestGridSessionsError::Argument(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListTestGridSessionsError::InternalService(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTestGridSessionsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTestGridSessionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTestGridSessionsError::Argument(ref cause) => write!(f, "{}", cause),
            ListTestGridSessionsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListTestGridSessionsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTestGridSessionsError {}
/// Errors returned by ListTests
#[derive(Debug, PartialEq)]
pub enum ListTestsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListTestsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTestsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListTestsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListTestsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTestsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListTestsError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTestsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTestsError::Argument(ref cause) => write!(f, "{}", cause),
            ListTestsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListTestsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListTestsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTestsError {}
/// Errors returned by ListUniqueProblems
#[derive(Debug, PartialEq)]
pub enum ListUniqueProblemsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListUniqueProblemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUniqueProblemsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListUniqueProblemsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListUniqueProblemsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListUniqueProblemsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListUniqueProblemsError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUniqueProblemsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUniqueProblemsError::Argument(ref cause) => write!(f, "{}", cause),
            ListUniqueProblemsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListUniqueProblemsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListUniqueProblemsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUniqueProblemsError {}
/// Errors returned by ListUploads
#[derive(Debug, PartialEq)]
pub enum ListUploadsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListUploadsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUploadsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListUploadsError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListUploadsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListUploadsError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListUploadsError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUploadsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUploadsError::Argument(ref cause) => write!(f, "{}", cause),
            ListUploadsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListUploadsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListUploadsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUploadsError {}
/// Errors returned by ListVPCEConfigurations
#[derive(Debug, PartialEq)]
pub enum ListVPCEConfigurationsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ListVPCEConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVPCEConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ListVPCEConfigurationsError::Argument(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ListVPCEConfigurationsError::ServiceAccount(
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
impl fmt::Display for ListVPCEConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVPCEConfigurationsError::Argument(ref cause) => write!(f, "{}", cause),
            ListVPCEConfigurationsError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVPCEConfigurationsError {}
/// Errors returned by PurchaseOffering
#[derive(Debug, PartialEq)]
pub enum PurchaseOfferingError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl PurchaseOfferingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PurchaseOfferingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(PurchaseOfferingError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PurchaseOfferingError::LimitExceeded(err.msg))
                }
                "NotEligibleException" => {
                    return RusotoError::Service(PurchaseOfferingError::NotEligible(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PurchaseOfferingError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(PurchaseOfferingError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PurchaseOfferingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PurchaseOfferingError::Argument(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::NotEligible(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::NotFound(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PurchaseOfferingError {}
/// Errors returned by RenewOffering
#[derive(Debug, PartialEq)]
pub enum RenewOfferingError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl RenewOfferingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RenewOfferingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(RenewOfferingError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RenewOfferingError::LimitExceeded(err.msg))
                }
                "NotEligibleException" => {
                    return RusotoError::Service(RenewOfferingError::NotEligible(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RenewOfferingError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(RenewOfferingError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RenewOfferingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenewOfferingError::Argument(ref cause) => write!(f, "{}", cause),
            RenewOfferingError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            RenewOfferingError::NotEligible(ref cause) => write!(f, "{}", cause),
            RenewOfferingError::NotFound(ref cause) => write!(f, "{}", cause),
            RenewOfferingError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RenewOfferingError {}
/// Errors returned by ScheduleRun
#[derive(Debug, PartialEq)]
pub enum ScheduleRunError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>An entity with the same name already exists.</p>
    Idempotency(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl ScheduleRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ScheduleRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(ScheduleRunError::Argument(err.msg))
                }
                "IdempotencyException" => {
                    return RusotoError::Service(ScheduleRunError::Idempotency(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ScheduleRunError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ScheduleRunError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(ScheduleRunError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ScheduleRunError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScheduleRunError::Argument(ref cause) => write!(f, "{}", cause),
            ScheduleRunError::Idempotency(ref cause) => write!(f, "{}", cause),
            ScheduleRunError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ScheduleRunError::NotFound(ref cause) => write!(f, "{}", cause),
            ScheduleRunError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ScheduleRunError {}
/// Errors returned by StopJob
#[derive(Debug, PartialEq)]
pub enum StopJobError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl StopJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(StopJobError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StopJobError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopJobError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(StopJobError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopJobError::Argument(ref cause) => write!(f, "{}", cause),
            StopJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StopJobError::NotFound(ref cause) => write!(f, "{}", cause),
            StopJobError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopJobError {}
/// Errors returned by StopRemoteAccessSession
#[derive(Debug, PartialEq)]
pub enum StopRemoteAccessSessionError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl StopRemoteAccessSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopRemoteAccessSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(StopRemoteAccessSessionError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StopRemoteAccessSessionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopRemoteAccessSessionError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(StopRemoteAccessSessionError::ServiceAccount(
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
impl fmt::Display for StopRemoteAccessSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopRemoteAccessSessionError::Argument(ref cause) => write!(f, "{}", cause),
            StopRemoteAccessSessionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StopRemoteAccessSessionError::NotFound(ref cause) => write!(f, "{}", cause),
            StopRemoteAccessSessionError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopRemoteAccessSessionError {}
/// Errors returned by StopRun
#[derive(Debug, PartialEq)]
pub enum StopRunError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl StopRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(StopRunError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StopRunError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopRunError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(StopRunError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopRunError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopRunError::Argument(ref cause) => write!(f, "{}", cause),
            StopRunError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StopRunError::NotFound(ref cause) => write!(f, "{}", cause),
            StopRunError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopRunError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>The operation was not successful. Try again.</p>
    TagOperation(String),
    /// <p>The request doesn't comply with the AWS Identity and Access Management (IAM) tag policy. Correct your request and then retry it.</p>
    TagPolicy(String),
    /// <p>The list of tags on the repository is over the limit. The maximum number of tags that can be applied to a repository is 50. </p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(TagResourceError::Argument(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "TagOperationException" => {
                    return RusotoError::Service(TagResourceError::TagOperation(err.msg))
                }
                "TagPolicyException" => {
                    return RusotoError::Service(TagResourceError::TagPolicy(err.msg))
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
            TagResourceError::Argument(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TagOperation(ref cause) => write!(f, "{}", cause),
            TagResourceError::TagPolicy(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>The operation was not successful. Try again.</p>
    TagOperation(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(UntagResourceError::Argument(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "TagOperationException" => {
                    return RusotoError::Service(UntagResourceError::TagOperation(err.msg))
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
            UntagResourceError::Argument(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TagOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDeviceInstance
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceInstanceError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl UpdateDeviceInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDeviceInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(UpdateDeviceInstanceError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateDeviceInstanceError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDeviceInstanceError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(UpdateDeviceInstanceError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDeviceInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDeviceInstanceError::Argument(ref cause) => write!(f, "{}", cause),
            UpdateDeviceInstanceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateDeviceInstanceError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateDeviceInstanceError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDeviceInstanceError {}
/// Errors returned by UpdateDevicePool
#[derive(Debug, PartialEq)]
pub enum UpdateDevicePoolError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl UpdateDevicePoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDevicePoolError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(UpdateDevicePoolError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateDevicePoolError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDevicePoolError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(UpdateDevicePoolError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDevicePoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDevicePoolError::Argument(ref cause) => write!(f, "{}", cause),
            UpdateDevicePoolError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateDevicePoolError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateDevicePoolError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDevicePoolError {}
/// Errors returned by UpdateInstanceProfile
#[derive(Debug, PartialEq)]
pub enum UpdateInstanceProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl UpdateInstanceProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateInstanceProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(UpdateInstanceProfileError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateInstanceProfileError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateInstanceProfileError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(UpdateInstanceProfileError::ServiceAccount(
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
impl fmt::Display for UpdateInstanceProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInstanceProfileError::Argument(ref cause) => write!(f, "{}", cause),
            UpdateInstanceProfileError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateInstanceProfileError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateInstanceProfileError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateInstanceProfileError {}
/// Errors returned by UpdateNetworkProfile
#[derive(Debug, PartialEq)]
pub enum UpdateNetworkProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl UpdateNetworkProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateNetworkProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(UpdateNetworkProfileError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateNetworkProfileError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateNetworkProfileError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(UpdateNetworkProfileError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateNetworkProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNetworkProfileError::Argument(ref cause) => write!(f, "{}", cause),
            UpdateNetworkProfileError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateNetworkProfileError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateNetworkProfileError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateNetworkProfileError {}
/// Errors returned by UpdateProject
#[derive(Debug, PartialEq)]
pub enum UpdateProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl UpdateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(UpdateProjectError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateProjectError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateProjectError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(UpdateProjectError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateProjectError::Argument(ref cause) => write!(f, "{}", cause),
            UpdateProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateProjectError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateProjectError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateProjectError {}
/// Errors returned by UpdateTestGridProject
#[derive(Debug, PartialEq)]
pub enum UpdateTestGridProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>An internal exception was raised in the service. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you see this error. </p>
    InternalService(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
}

impl UpdateTestGridProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTestGridProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(UpdateTestGridProjectError::Argument(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateTestGridProjectError::InternalService(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateTestGridProjectError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateTestGridProjectError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateTestGridProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTestGridProjectError::Argument(ref cause) => write!(f, "{}", cause),
            UpdateTestGridProjectError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateTestGridProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateTestGridProjectError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTestGridProjectError {}
/// Errors returned by UpdateUpload
#[derive(Debug, PartialEq)]
pub enum UpdateUploadError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl UpdateUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUploadError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(UpdateUploadError::Argument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateUploadError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateUploadError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(UpdateUploadError::ServiceAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUploadError::Argument(ref cause) => write!(f, "{}", cause),
            UpdateUploadError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateUploadError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateUploadError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUploadError {}
/// Errors returned by UpdateVPCEConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateVPCEConfigurationError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>There was an error with the update request, or you do not have sufficient permissions to update this VPC endpoint configuration.</p>
    InvalidOperation(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
}

impl UpdateVPCEConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVPCEConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArgumentException" => {
                    return RusotoError::Service(UpdateVPCEConfigurationError::Argument(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(UpdateVPCEConfigurationError::InvalidOperation(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVPCEConfigurationError::NotFound(err.msg))
                }
                "ServiceAccountException" => {
                    return RusotoError::Service(UpdateVPCEConfigurationError::ServiceAccount(
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
impl fmt::Display for UpdateVPCEConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVPCEConfigurationError::Argument(ref cause) => write!(f, "{}", cause),
            UpdateVPCEConfigurationError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateVPCEConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateVPCEConfigurationError::ServiceAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVPCEConfigurationError {}
/// Trait representing the capabilities of the AWS Device Farm API. AWS Device Farm clients implement this trait.
#[async_trait]
pub trait DeviceFarm {
    /// <p>Creates a device pool.</p>
    async fn create_device_pool(
        &self,
        input: CreateDevicePoolRequest,
    ) -> Result<CreateDevicePoolResult, RusotoError<CreateDevicePoolError>>;

    /// <p>Creates a profile that can be applied to one or more private fleet device instances.</p>
    async fn create_instance_profile(
        &self,
        input: CreateInstanceProfileRequest,
    ) -> Result<CreateInstanceProfileResult, RusotoError<CreateInstanceProfileError>>;

    /// <p>Creates a network profile.</p>
    async fn create_network_profile(
        &self,
        input: CreateNetworkProfileRequest,
    ) -> Result<CreateNetworkProfileResult, RusotoError<CreateNetworkProfileError>>;

    /// <p>Creates a project.</p>
    async fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> Result<CreateProjectResult, RusotoError<CreateProjectError>>;

    /// <p>Specifies and starts a remote access session.</p>
    async fn create_remote_access_session(
        &self,
        input: CreateRemoteAccessSessionRequest,
    ) -> Result<CreateRemoteAccessSessionResult, RusotoError<CreateRemoteAccessSessionError>>;

    /// <p>Creates a Selenium testing project. Projects are used to track <a>TestGridSession</a> instances.</p>
    async fn create_test_grid_project(
        &self,
        input: CreateTestGridProjectRequest,
    ) -> Result<CreateTestGridProjectResult, RusotoError<CreateTestGridProjectError>>;

    /// <p>Creates a signed, short-term URL that can be passed to a Selenium <code>RemoteWebDriver</code> constructor.</p>
    async fn create_test_grid_url(
        &self,
        input: CreateTestGridUrlRequest,
    ) -> Result<CreateTestGridUrlResult, RusotoError<CreateTestGridUrlError>>;

    /// <p>Uploads an app or test scripts.</p>
    async fn create_upload(
        &self,
        input: CreateUploadRequest,
    ) -> Result<CreateUploadResult, RusotoError<CreateUploadError>>;

    /// <p>Creates a configuration record in Device Farm for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    async fn create_vpce_configuration(
        &self,
        input: CreateVPCEConfigurationRequest,
    ) -> Result<CreateVPCEConfigurationResult, RusotoError<CreateVPCEConfigurationError>>;

    /// <p>Deletes a device pool given the pool ARN. Does not allow deletion of curated pools owned by the system.</p>
    async fn delete_device_pool(
        &self,
        input: DeleteDevicePoolRequest,
    ) -> Result<DeleteDevicePoolResult, RusotoError<DeleteDevicePoolError>>;

    /// <p>Deletes a profile that can be applied to one or more private device instances.</p>
    async fn delete_instance_profile(
        &self,
        input: DeleteInstanceProfileRequest,
    ) -> Result<DeleteInstanceProfileResult, RusotoError<DeleteInstanceProfileError>>;

    /// <p>Deletes a network profile.</p>
    async fn delete_network_profile(
        &self,
        input: DeleteNetworkProfileRequest,
    ) -> Result<DeleteNetworkProfileResult, RusotoError<DeleteNetworkProfileError>>;

    /// <p>Deletes an AWS Device Farm project, given the project ARN.</p> <p> Deleting this resource does not stop an in-progress run.</p>
    async fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> Result<DeleteProjectResult, RusotoError<DeleteProjectError>>;

    /// <p>Deletes a completed remote access session and its results.</p>
    async fn delete_remote_access_session(
        &self,
        input: DeleteRemoteAccessSessionRequest,
    ) -> Result<DeleteRemoteAccessSessionResult, RusotoError<DeleteRemoteAccessSessionError>>;

    /// <p>Deletes the run, given the run ARN.</p> <p> Deleting this resource does not stop an in-progress run.</p>
    async fn delete_run(
        &self,
        input: DeleteRunRequest,
    ) -> Result<DeleteRunResult, RusotoError<DeleteRunError>>;

    /// <p><p> Deletes a Selenium testing project and all content generated under it. </p> <important> <p>You cannot undo this operation.</p> </important> <note> <p>You cannot delete a project if it has active sessions.</p> </note></p>
    async fn delete_test_grid_project(
        &self,
        input: DeleteTestGridProjectRequest,
    ) -> Result<DeleteTestGridProjectResult, RusotoError<DeleteTestGridProjectError>>;

    /// <p>Deletes an upload given the upload ARN.</p>
    async fn delete_upload(
        &self,
        input: DeleteUploadRequest,
    ) -> Result<DeleteUploadResult, RusotoError<DeleteUploadError>>;

    /// <p>Deletes a configuration for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    async fn delete_vpce_configuration(
        &self,
        input: DeleteVPCEConfigurationRequest,
    ) -> Result<DeleteVPCEConfigurationResult, RusotoError<DeleteVPCEConfigurationError>>;

    /// <p>Returns the number of unmetered iOS or unmetered Android devices that have been purchased by the account.</p>
    async fn get_account_settings(
        &self,
    ) -> Result<GetAccountSettingsResult, RusotoError<GetAccountSettingsError>>;

    /// <p>Gets information about a unique device type.</p>
    async fn get_device(
        &self,
        input: GetDeviceRequest,
    ) -> Result<GetDeviceResult, RusotoError<GetDeviceError>>;

    /// <p>Returns information about a device instance that belongs to a private device fleet.</p>
    async fn get_device_instance(
        &self,
        input: GetDeviceInstanceRequest,
    ) -> Result<GetDeviceInstanceResult, RusotoError<GetDeviceInstanceError>>;

    /// <p>Gets information about a device pool.</p>
    async fn get_device_pool(
        &self,
        input: GetDevicePoolRequest,
    ) -> Result<GetDevicePoolResult, RusotoError<GetDevicePoolError>>;

    /// <p>Gets information about compatibility with a device pool.</p>
    async fn get_device_pool_compatibility(
        &self,
        input: GetDevicePoolCompatibilityRequest,
    ) -> Result<GetDevicePoolCompatibilityResult, RusotoError<GetDevicePoolCompatibilityError>>;

    /// <p>Returns information about the specified instance profile.</p>
    async fn get_instance_profile(
        &self,
        input: GetInstanceProfileRequest,
    ) -> Result<GetInstanceProfileResult, RusotoError<GetInstanceProfileError>>;

    /// <p>Gets information about a job.</p>
    async fn get_job(&self, input: GetJobRequest)
        -> Result<GetJobResult, RusotoError<GetJobError>>;

    /// <p>Returns information about a network profile.</p>
    async fn get_network_profile(
        &self,
        input: GetNetworkProfileRequest,
    ) -> Result<GetNetworkProfileResult, RusotoError<GetNetworkProfileError>>;

    /// <p>Gets the current status and future status of all offerings purchased by an AWS account. The response indicates how many offerings are currently available and the offerings that will be available in the next period. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. If you must be able to invoke this operation, contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a>.</p>
    async fn get_offering_status(
        &self,
        input: GetOfferingStatusRequest,
    ) -> Result<GetOfferingStatusResult, RusotoError<GetOfferingStatusError>>;

    /// <p>Gets information about a project.</p>
    async fn get_project(
        &self,
        input: GetProjectRequest,
    ) -> Result<GetProjectResult, RusotoError<GetProjectError>>;

    /// <p>Returns a link to a currently running remote access session.</p>
    async fn get_remote_access_session(
        &self,
        input: GetRemoteAccessSessionRequest,
    ) -> Result<GetRemoteAccessSessionResult, RusotoError<GetRemoteAccessSessionError>>;

    /// <p>Gets information about a run.</p>
    async fn get_run(&self, input: GetRunRequest)
        -> Result<GetRunResult, RusotoError<GetRunError>>;

    /// <p>Gets information about a suite.</p>
    async fn get_suite(
        &self,
        input: GetSuiteRequest,
    ) -> Result<GetSuiteResult, RusotoError<GetSuiteError>>;

    /// <p>Gets information about a test.</p>
    async fn get_test(
        &self,
        input: GetTestRequest,
    ) -> Result<GetTestResult, RusotoError<GetTestError>>;

    /// <p>Retrieves information about a Selenium testing project.</p>
    async fn get_test_grid_project(
        &self,
        input: GetTestGridProjectRequest,
    ) -> Result<GetTestGridProjectResult, RusotoError<GetTestGridProjectError>>;

    /// <p><p>A session is an instance of a browser created through a <code>RemoteWebDriver</code> with the URL from <a>CreateTestGridUrlResult$url</a>. You can use the following to look up sessions:</p> <ul> <li> <p>The session ARN (<a>GetTestGridSessionRequest$sessionArn</a>).</p> </li> <li> <p>The project ARN and a session ID (<a>GetTestGridSessionRequest$projectArn</a> and <a>GetTestGridSessionRequest$sessionId</a>).</p> </li> </ul> <p/></p>
    async fn get_test_grid_session(
        &self,
        input: GetTestGridSessionRequest,
    ) -> Result<GetTestGridSessionResult, RusotoError<GetTestGridSessionError>>;

    /// <p>Gets information about an upload.</p>
    async fn get_upload(
        &self,
        input: GetUploadRequest,
    ) -> Result<GetUploadResult, RusotoError<GetUploadError>>;

    /// <p>Returns information about the configuration settings for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    async fn get_vpce_configuration(
        &self,
        input: GetVPCEConfigurationRequest,
    ) -> Result<GetVPCEConfigurationResult, RusotoError<GetVPCEConfigurationError>>;

    /// <p>Installs an application to the device in a remote access session. For Android applications, the file must be in .apk format. For iOS applications, the file must be in .ipa format.</p>
    async fn install_to_remote_access_session(
        &self,
        input: InstallToRemoteAccessSessionRequest,
    ) -> Result<InstallToRemoteAccessSessionResult, RusotoError<InstallToRemoteAccessSessionError>>;

    /// <p>Gets information about artifacts.</p>
    async fn list_artifacts(
        &self,
        input: ListArtifactsRequest,
    ) -> Result<ListArtifactsResult, RusotoError<ListArtifactsError>>;

    /// <p>Returns information about the private device instances associated with one or more AWS accounts.</p>
    async fn list_device_instances(
        &self,
        input: ListDeviceInstancesRequest,
    ) -> Result<ListDeviceInstancesResult, RusotoError<ListDeviceInstancesError>>;

    /// <p>Gets information about device pools.</p>
    async fn list_device_pools(
        &self,
        input: ListDevicePoolsRequest,
    ) -> Result<ListDevicePoolsResult, RusotoError<ListDevicePoolsError>>;

    /// <p>Gets information about unique device types.</p>
    async fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> Result<ListDevicesResult, RusotoError<ListDevicesError>>;

    /// <p>Returns information about all the instance profiles in an AWS account.</p>
    async fn list_instance_profiles(
        &self,
        input: ListInstanceProfilesRequest,
    ) -> Result<ListInstanceProfilesResult, RusotoError<ListInstanceProfilesError>>;

    /// <p>Gets information about jobs for a given test run.</p>
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResult, RusotoError<ListJobsError>>;

    /// <p>Returns the list of available network profiles.</p>
    async fn list_network_profiles(
        &self,
        input: ListNetworkProfilesRequest,
    ) -> Result<ListNetworkProfilesResult, RusotoError<ListNetworkProfilesError>>;

    /// <p>Returns a list of offering promotions. Each offering promotion record contains the ID and description of the promotion. The API returns a <code>NotEligible</code> error if the caller is not permitted to invoke the operation. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you must be able to invoke this operation.</p>
    async fn list_offering_promotions(
        &self,
        input: ListOfferingPromotionsRequest,
    ) -> Result<ListOfferingPromotionsResult, RusotoError<ListOfferingPromotionsError>>;

    /// <p>Returns a list of all historical purchases, renewals, and system renewal transactions for an AWS account. The list is paginated and ordered by a descending timestamp (most recent transactions are first). The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. If you must be able to invoke this operation, contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a>.</p>
    async fn list_offering_transactions(
        &self,
        input: ListOfferingTransactionsRequest,
    ) -> Result<ListOfferingTransactionsResult, RusotoError<ListOfferingTransactionsError>>;

    /// <p>Returns a list of products or offerings that the user can manage through the API. Each offering record indicates the recurring price per unit and the frequency for that offering. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. If you must be able to invoke this operation, contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a>.</p>
    async fn list_offerings(
        &self,
        input: ListOfferingsRequest,
    ) -> Result<ListOfferingsResult, RusotoError<ListOfferingsError>>;

    /// <p>Gets information about projects.</p>
    async fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> Result<ListProjectsResult, RusotoError<ListProjectsError>>;

    /// <p>Returns a list of all currently running remote access sessions.</p>
    async fn list_remote_access_sessions(
        &self,
        input: ListRemoteAccessSessionsRequest,
    ) -> Result<ListRemoteAccessSessionsResult, RusotoError<ListRemoteAccessSessionsError>>;

    /// <p>Gets information about runs, given an AWS Device Farm project ARN.</p>
    async fn list_runs(
        &self,
        input: ListRunsRequest,
    ) -> Result<ListRunsResult, RusotoError<ListRunsError>>;

    /// <p>Gets information about samples, given an AWS Device Farm job ARN.</p>
    async fn list_samples(
        &self,
        input: ListSamplesRequest,
    ) -> Result<ListSamplesResult, RusotoError<ListSamplesError>>;

    /// <p>Gets information about test suites for a given job.</p>
    async fn list_suites(
        &self,
        input: ListSuitesRequest,
    ) -> Result<ListSuitesResult, RusotoError<ListSuitesError>>;

    /// <p>List the tags for an AWS Device Farm resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Gets a list of all Selenium testing projects in your account.</p>
    async fn list_test_grid_projects(
        &self,
        input: ListTestGridProjectsRequest,
    ) -> Result<ListTestGridProjectsResult, RusotoError<ListTestGridProjectsError>>;

    /// <p>Returns a list of the actions taken in a <a>TestGridSession</a>.</p>
    async fn list_test_grid_session_actions(
        &self,
        input: ListTestGridSessionActionsRequest,
    ) -> Result<ListTestGridSessionActionsResult, RusotoError<ListTestGridSessionActionsError>>;

    /// <p>Retrieves a list of artifacts created during the session.</p>
    async fn list_test_grid_session_artifacts(
        &self,
        input: ListTestGridSessionArtifactsRequest,
    ) -> Result<ListTestGridSessionArtifactsResult, RusotoError<ListTestGridSessionArtifactsError>>;

    /// <p>Retrieves a list of sessions for a <a>TestGridProject</a>.</p>
    async fn list_test_grid_sessions(
        &self,
        input: ListTestGridSessionsRequest,
    ) -> Result<ListTestGridSessionsResult, RusotoError<ListTestGridSessionsError>>;

    /// <p>Gets information about tests in a given test suite.</p>
    async fn list_tests(
        &self,
        input: ListTestsRequest,
    ) -> Result<ListTestsResult, RusotoError<ListTestsError>>;

    /// <p>Gets information about unique problems, such as exceptions or crashes.</p> <p>Unique problems are defined as a single instance of an error across a run, job, or suite. For example, if a call in your application consistently raises an exception (<code>OutOfBoundsException in MyActivity.java:386</code>), <code>ListUniqueProblems</code> returns a single entry instead of many individual entries for that exception.</p>
    async fn list_unique_problems(
        &self,
        input: ListUniqueProblemsRequest,
    ) -> Result<ListUniqueProblemsResult, RusotoError<ListUniqueProblemsError>>;

    /// <p>Gets information about uploads, given an AWS Device Farm project ARN.</p>
    async fn list_uploads(
        &self,
        input: ListUploadsRequest,
    ) -> Result<ListUploadsResult, RusotoError<ListUploadsError>>;

    /// <p>Returns information about all Amazon Virtual Private Cloud (VPC) endpoint configurations in the AWS account.</p>
    async fn list_vpce_configurations(
        &self,
        input: ListVPCEConfigurationsRequest,
    ) -> Result<ListVPCEConfigurationsResult, RusotoError<ListVPCEConfigurationsError>>;

    /// <p>Immediately purchases offerings for an AWS account. Offerings renew with the latest total purchased quantity for an offering, unless the renewal was overridden. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. If you must be able to invoke this operation, contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a>.</p>
    async fn purchase_offering(
        &self,
        input: PurchaseOfferingRequest,
    ) -> Result<PurchaseOfferingResult, RusotoError<PurchaseOfferingError>>;

    /// <p>Explicitly sets the quantity of devices to renew for an offering, starting from the <code>effectiveDate</code> of the next period. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. If you must be able to invoke this operation, contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a>.</p>
    async fn renew_offering(
        &self,
        input: RenewOfferingRequest,
    ) -> Result<RenewOfferingResult, RusotoError<RenewOfferingError>>;

    /// <p>Schedules a run.</p>
    async fn schedule_run(
        &self,
        input: ScheduleRunRequest,
    ) -> Result<ScheduleRunResult, RusotoError<ScheduleRunError>>;

    /// <p>Initiates a stop request for the current job. AWS Device Farm immediately stops the job on the device where tests have not started. You are not billed for this device. On the device where tests have started, setup suite and teardown suite tests run to completion on the device. You are billed for setup, teardown, and any tests that were in progress or already completed.</p>
    async fn stop_job(
        &self,
        input: StopJobRequest,
    ) -> Result<StopJobResult, RusotoError<StopJobError>>;

    /// <p>Ends a specified remote access session.</p>
    async fn stop_remote_access_session(
        &self,
        input: StopRemoteAccessSessionRequest,
    ) -> Result<StopRemoteAccessSessionResult, RusotoError<StopRemoteAccessSessionError>>;

    /// <p>Initiates a stop request for the current test run. AWS Device Farm immediately stops the run on devices where tests have not started. You are not billed for these devices. On devices where tests have started executing, setup suite and teardown suite tests run to completion on those devices. You are billed for setup, teardown, and any tests that were in progress or already completed.</p>
    async fn stop_run(
        &self,
        input: StopRunRequest,
    ) -> Result<StopRunResult, RusotoError<StopRunError>>;

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are also deleted.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Deletes the specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates information about a private device instance.</p>
    async fn update_device_instance(
        &self,
        input: UpdateDeviceInstanceRequest,
    ) -> Result<UpdateDeviceInstanceResult, RusotoError<UpdateDeviceInstanceError>>;

    /// <p>Modifies the name, description, and rules in a device pool given the attributes and the pool ARN. Rule updates are all-or-nothing, meaning they can only be updated as a whole (or not at all).</p>
    async fn update_device_pool(
        &self,
        input: UpdateDevicePoolRequest,
    ) -> Result<UpdateDevicePoolResult, RusotoError<UpdateDevicePoolError>>;

    /// <p>Updates information about an existing private device instance profile.</p>
    async fn update_instance_profile(
        &self,
        input: UpdateInstanceProfileRequest,
    ) -> Result<UpdateInstanceProfileResult, RusotoError<UpdateInstanceProfileError>>;

    /// <p>Updates the network profile.</p>
    async fn update_network_profile(
        &self,
        input: UpdateNetworkProfileRequest,
    ) -> Result<UpdateNetworkProfileResult, RusotoError<UpdateNetworkProfileError>>;

    /// <p>Modifies the specified project name, given the project ARN and a new name.</p>
    async fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> Result<UpdateProjectResult, RusotoError<UpdateProjectError>>;

    /// <p>Change details of a project.</p>
    async fn update_test_grid_project(
        &self,
        input: UpdateTestGridProjectRequest,
    ) -> Result<UpdateTestGridProjectResult, RusotoError<UpdateTestGridProjectError>>;

    /// <p>Updates an uploaded test spec.</p>
    async fn update_upload(
        &self,
        input: UpdateUploadRequest,
    ) -> Result<UpdateUploadResult, RusotoError<UpdateUploadError>>;

    /// <p>Updates information about an Amazon Virtual Private Cloud (VPC) endpoint configuration.</p>
    async fn update_vpce_configuration(
        &self,
        input: UpdateVPCEConfigurationRequest,
    ) -> Result<UpdateVPCEConfigurationResult, RusotoError<UpdateVPCEConfigurationError>>;
}
/// A client for the AWS Device Farm API.
#[derive(Clone)]
pub struct DeviceFarmClient {
    client: Client,
    region: region::Region,
}

impl DeviceFarmClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DeviceFarmClient {
        DeviceFarmClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DeviceFarmClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DeviceFarmClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DeviceFarmClient {
        DeviceFarmClient { client, region }
    }
}

#[async_trait]
impl DeviceFarm for DeviceFarmClient {
    /// <p>Creates a device pool.</p>
    async fn create_device_pool(
        &self,
        input: CreateDevicePoolRequest,
    ) -> Result<CreateDevicePoolResult, RusotoError<CreateDevicePoolError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateDevicePool");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateDevicePoolError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateDevicePoolResult, _>()
    }

    /// <p>Creates a profile that can be applied to one or more private fleet device instances.</p>
    async fn create_instance_profile(
        &self,
        input: CreateInstanceProfileRequest,
    ) -> Result<CreateInstanceProfileResult, RusotoError<CreateInstanceProfileError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateInstanceProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateInstanceProfileError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateInstanceProfileResult, _>()
    }

    /// <p>Creates a network profile.</p>
    async fn create_network_profile(
        &self,
        input: CreateNetworkProfileRequest,
    ) -> Result<CreateNetworkProfileResult, RusotoError<CreateNetworkProfileError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateNetworkProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateNetworkProfileError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateNetworkProfileResult, _>()
    }

    /// <p>Creates a project.</p>
    async fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> Result<CreateProjectResult, RusotoError<CreateProjectError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateProjectError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateProjectResult, _>()
    }

    /// <p>Specifies and starts a remote access session.</p>
    async fn create_remote_access_session(
        &self,
        input: CreateRemoteAccessSessionRequest,
    ) -> Result<CreateRemoteAccessSessionResult, RusotoError<CreateRemoteAccessSessionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.CreateRemoteAccessSession",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateRemoteAccessSessionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateRemoteAccessSessionResult, _>()
    }

    /// <p>Creates a Selenium testing project. Projects are used to track <a>TestGridSession</a> instances.</p>
    async fn create_test_grid_project(
        &self,
        input: CreateTestGridProjectRequest,
    ) -> Result<CreateTestGridProjectResult, RusotoError<CreateTestGridProjectError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateTestGridProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateTestGridProjectError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateTestGridProjectResult, _>()
    }

    /// <p>Creates a signed, short-term URL that can be passed to a Selenium <code>RemoteWebDriver</code> constructor.</p>
    async fn create_test_grid_url(
        &self,
        input: CreateTestGridUrlRequest,
    ) -> Result<CreateTestGridUrlResult, RusotoError<CreateTestGridUrlError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateTestGridUrl");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateTestGridUrlError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateTestGridUrlResult, _>()
    }

    /// <p>Uploads an app or test scripts.</p>
    async fn create_upload(
        &self,
        input: CreateUploadRequest,
    ) -> Result<CreateUploadResult, RusotoError<CreateUploadError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateUpload");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateUploadError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateUploadResult, _>()
    }

    /// <p>Creates a configuration record in Device Farm for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    async fn create_vpce_configuration(
        &self,
        input: CreateVPCEConfigurationRequest,
    ) -> Result<CreateVPCEConfigurationResult, RusotoError<CreateVPCEConfigurationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.CreateVPCEConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateVPCEConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateVPCEConfigurationResult, _>()
    }

    /// <p>Deletes a device pool given the pool ARN. Does not allow deletion of curated pools owned by the system.</p>
    async fn delete_device_pool(
        &self,
        input: DeleteDevicePoolRequest,
    ) -> Result<DeleteDevicePoolResult, RusotoError<DeleteDevicePoolError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteDevicePool");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteDevicePoolError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteDevicePoolResult, _>()
    }

    /// <p>Deletes a profile that can be applied to one or more private device instances.</p>
    async fn delete_instance_profile(
        &self,
        input: DeleteInstanceProfileRequest,
    ) -> Result<DeleteInstanceProfileResult, RusotoError<DeleteInstanceProfileError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteInstanceProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteInstanceProfileError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteInstanceProfileResult, _>()
    }

    /// <p>Deletes a network profile.</p>
    async fn delete_network_profile(
        &self,
        input: DeleteNetworkProfileRequest,
    ) -> Result<DeleteNetworkProfileResult, RusotoError<DeleteNetworkProfileError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteNetworkProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteNetworkProfileError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteNetworkProfileResult, _>()
    }

    /// <p>Deletes an AWS Device Farm project, given the project ARN.</p> <p> Deleting this resource does not stop an in-progress run.</p>
    async fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> Result<DeleteProjectResult, RusotoError<DeleteProjectError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteProjectError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteProjectResult, _>()
    }

    /// <p>Deletes a completed remote access session and its results.</p>
    async fn delete_remote_access_session(
        &self,
        input: DeleteRemoteAccessSessionRequest,
    ) -> Result<DeleteRemoteAccessSessionResult, RusotoError<DeleteRemoteAccessSessionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.DeleteRemoteAccessSession",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteRemoteAccessSessionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteRemoteAccessSessionResult, _>()
    }

    /// <p>Deletes the run, given the run ARN.</p> <p> Deleting this resource does not stop an in-progress run.</p>
    async fn delete_run(
        &self,
        input: DeleteRunRequest,
    ) -> Result<DeleteRunResult, RusotoError<DeleteRunError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteRunError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteRunResult, _>()
    }

    /// <p><p> Deletes a Selenium testing project and all content generated under it. </p> <important> <p>You cannot undo this operation.</p> </important> <note> <p>You cannot delete a project if it has active sessions.</p> </note></p>
    async fn delete_test_grid_project(
        &self,
        input: DeleteTestGridProjectRequest,
    ) -> Result<DeleteTestGridProjectResult, RusotoError<DeleteTestGridProjectError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteTestGridProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteTestGridProjectError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteTestGridProjectResult, _>()
    }

    /// <p>Deletes an upload given the upload ARN.</p>
    async fn delete_upload(
        &self,
        input: DeleteUploadRequest,
    ) -> Result<DeleteUploadResult, RusotoError<DeleteUploadError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteUpload");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteUploadError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteUploadResult, _>()
    }

    /// <p>Deletes a configuration for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    async fn delete_vpce_configuration(
        &self,
        input: DeleteVPCEConfigurationRequest,
    ) -> Result<DeleteVPCEConfigurationResult, RusotoError<DeleteVPCEConfigurationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.DeleteVPCEConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteVPCEConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteVPCEConfigurationResult, _>()
    }

    /// <p>Returns the number of unmetered iOS or unmetered Android devices that have been purchased by the account.</p>
    async fn get_account_settings(
        &self,
    ) -> Result<GetAccountSettingsResult, RusotoError<GetAccountSettingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetAccountSettings");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, GetAccountSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetAccountSettingsResult, _>()
    }

    /// <p>Gets information about a unique device type.</p>
    async fn get_device(
        &self,
        input: GetDeviceRequest,
    ) -> Result<GetDeviceResult, RusotoError<GetDeviceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetDevice");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetDeviceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetDeviceResult, _>()
    }

    /// <p>Returns information about a device instance that belongs to a private device fleet.</p>
    async fn get_device_instance(
        &self,
        input: GetDeviceInstanceRequest,
    ) -> Result<GetDeviceInstanceResult, RusotoError<GetDeviceInstanceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetDeviceInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetDeviceInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetDeviceInstanceResult, _>()
    }

    /// <p>Gets information about a device pool.</p>
    async fn get_device_pool(
        &self,
        input: GetDevicePoolRequest,
    ) -> Result<GetDevicePoolResult, RusotoError<GetDevicePoolError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetDevicePool");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetDevicePoolError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetDevicePoolResult, _>()
    }

    /// <p>Gets information about compatibility with a device pool.</p>
    async fn get_device_pool_compatibility(
        &self,
        input: GetDevicePoolCompatibilityRequest,
    ) -> Result<GetDevicePoolCompatibilityResult, RusotoError<GetDevicePoolCompatibilityError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.GetDevicePoolCompatibility",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetDevicePoolCompatibilityError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetDevicePoolCompatibilityResult, _>()
    }

    /// <p>Returns information about the specified instance profile.</p>
    async fn get_instance_profile(
        &self,
        input: GetInstanceProfileRequest,
    ) -> Result<GetInstanceProfileResult, RusotoError<GetInstanceProfileError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetInstanceProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetInstanceProfileError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetInstanceProfileResult, _>()
    }

    /// <p>Gets information about a job.</p>
    async fn get_job(
        &self,
        input: GetJobRequest,
    ) -> Result<GetJobResult, RusotoError<GetJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetJobResult, _>()
    }

    /// <p>Returns information about a network profile.</p>
    async fn get_network_profile(
        &self,
        input: GetNetworkProfileRequest,
    ) -> Result<GetNetworkProfileResult, RusotoError<GetNetworkProfileError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetNetworkProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetNetworkProfileError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetNetworkProfileResult, _>()
    }

    /// <p>Gets the current status and future status of all offerings purchased by an AWS account. The response indicates how many offerings are currently available and the offerings that will be available in the next period. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. If you must be able to invoke this operation, contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a>.</p>
    async fn get_offering_status(
        &self,
        input: GetOfferingStatusRequest,
    ) -> Result<GetOfferingStatusResult, RusotoError<GetOfferingStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetOfferingStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetOfferingStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetOfferingStatusResult, _>()
    }

    /// <p>Gets information about a project.</p>
    async fn get_project(
        &self,
        input: GetProjectRequest,
    ) -> Result<GetProjectResult, RusotoError<GetProjectError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetProjectError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetProjectResult, _>()
    }

    /// <p>Returns a link to a currently running remote access session.</p>
    async fn get_remote_access_session(
        &self,
        input: GetRemoteAccessSessionRequest,
    ) -> Result<GetRemoteAccessSessionResult, RusotoError<GetRemoteAccessSessionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetRemoteAccessSession");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRemoteAccessSessionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetRemoteAccessSessionResult, _>()
    }

    /// <p>Gets information about a run.</p>
    async fn get_run(
        &self,
        input: GetRunRequest,
    ) -> Result<GetRunResult, RusotoError<GetRunError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRunError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetRunResult, _>()
    }

    /// <p>Gets information about a suite.</p>
    async fn get_suite(
        &self,
        input: GetSuiteRequest,
    ) -> Result<GetSuiteResult, RusotoError<GetSuiteError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetSuite");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetSuiteError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetSuiteResult, _>()
    }

    /// <p>Gets information about a test.</p>
    async fn get_test(
        &self,
        input: GetTestRequest,
    ) -> Result<GetTestResult, RusotoError<GetTestError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetTest");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetTestError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetTestResult, _>()
    }

    /// <p>Retrieves information about a Selenium testing project.</p>
    async fn get_test_grid_project(
        &self,
        input: GetTestGridProjectRequest,
    ) -> Result<GetTestGridProjectResult, RusotoError<GetTestGridProjectError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetTestGridProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetTestGridProjectError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetTestGridProjectResult, _>()
    }

    /// <p><p>A session is an instance of a browser created through a <code>RemoteWebDriver</code> with the URL from <a>CreateTestGridUrlResult$url</a>. You can use the following to look up sessions:</p> <ul> <li> <p>The session ARN (<a>GetTestGridSessionRequest$sessionArn</a>).</p> </li> <li> <p>The project ARN and a session ID (<a>GetTestGridSessionRequest$projectArn</a> and <a>GetTestGridSessionRequest$sessionId</a>).</p> </li> </ul> <p/></p>
    async fn get_test_grid_session(
        &self,
        input: GetTestGridSessionRequest,
    ) -> Result<GetTestGridSessionResult, RusotoError<GetTestGridSessionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetTestGridSession");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetTestGridSessionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetTestGridSessionResult, _>()
    }

    /// <p>Gets information about an upload.</p>
    async fn get_upload(
        &self,
        input: GetUploadRequest,
    ) -> Result<GetUploadResult, RusotoError<GetUploadError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetUpload");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetUploadError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetUploadResult, _>()
    }

    /// <p>Returns information about the configuration settings for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    async fn get_vpce_configuration(
        &self,
        input: GetVPCEConfigurationRequest,
    ) -> Result<GetVPCEConfigurationResult, RusotoError<GetVPCEConfigurationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetVPCEConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetVPCEConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetVPCEConfigurationResult, _>()
    }

    /// <p>Installs an application to the device in a remote access session. For Android applications, the file must be in .apk format. For iOS applications, the file must be in .ipa format.</p>
    async fn install_to_remote_access_session(
        &self,
        input: InstallToRemoteAccessSessionRequest,
    ) -> Result<InstallToRemoteAccessSessionResult, RusotoError<InstallToRemoteAccessSessionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.InstallToRemoteAccessSession",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, InstallToRemoteAccessSessionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<InstallToRemoteAccessSessionResult, _>()
    }

    /// <p>Gets information about artifacts.</p>
    async fn list_artifacts(
        &self,
        input: ListArtifactsRequest,
    ) -> Result<ListArtifactsResult, RusotoError<ListArtifactsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListArtifacts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListArtifactsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListArtifactsResult, _>()
    }

    /// <p>Returns information about the private device instances associated with one or more AWS accounts.</p>
    async fn list_device_instances(
        &self,
        input: ListDeviceInstancesRequest,
    ) -> Result<ListDeviceInstancesResult, RusotoError<ListDeviceInstancesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListDeviceInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDeviceInstancesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListDeviceInstancesResult, _>()
    }

    /// <p>Gets information about device pools.</p>
    async fn list_device_pools(
        &self,
        input: ListDevicePoolsRequest,
    ) -> Result<ListDevicePoolsResult, RusotoError<ListDevicePoolsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListDevicePools");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDevicePoolsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListDevicePoolsResult, _>()
    }

    /// <p>Gets information about unique device types.</p>
    async fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> Result<ListDevicesResult, RusotoError<ListDevicesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListDevices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDevicesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListDevicesResult, _>()
    }

    /// <p>Returns information about all the instance profiles in an AWS account.</p>
    async fn list_instance_profiles(
        &self,
        input: ListInstanceProfilesRequest,
    ) -> Result<ListInstanceProfilesResult, RusotoError<ListInstanceProfilesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListInstanceProfiles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListInstanceProfilesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListInstanceProfilesResult, _>()
    }

    /// <p>Gets information about jobs for a given test run.</p>
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResult, RusotoError<ListJobsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListJobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListJobsResult, _>()
    }

    /// <p>Returns the list of available network profiles.</p>
    async fn list_network_profiles(
        &self,
        input: ListNetworkProfilesRequest,
    ) -> Result<ListNetworkProfilesResult, RusotoError<ListNetworkProfilesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListNetworkProfiles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListNetworkProfilesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListNetworkProfilesResult, _>()
    }

    /// <p>Returns a list of offering promotions. Each offering promotion record contains the ID and description of the promotion. The API returns a <code>NotEligible</code> error if the caller is not permitted to invoke the operation. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you must be able to invoke this operation.</p>
    async fn list_offering_promotions(
        &self,
        input: ListOfferingPromotionsRequest,
    ) -> Result<ListOfferingPromotionsResult, RusotoError<ListOfferingPromotionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListOfferingPromotions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListOfferingPromotionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListOfferingPromotionsResult, _>()
    }

    /// <p>Returns a list of all historical purchases, renewals, and system renewal transactions for an AWS account. The list is paginated and ordered by a descending timestamp (most recent transactions are first). The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. If you must be able to invoke this operation, contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a>.</p>
    async fn list_offering_transactions(
        &self,
        input: ListOfferingTransactionsRequest,
    ) -> Result<ListOfferingTransactionsResult, RusotoError<ListOfferingTransactionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.ListOfferingTransactions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListOfferingTransactionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListOfferingTransactionsResult, _>()
    }

    /// <p>Returns a list of products or offerings that the user can manage through the API. Each offering record indicates the recurring price per unit and the frequency for that offering. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. If you must be able to invoke this operation, contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a>.</p>
    async fn list_offerings(
        &self,
        input: ListOfferingsRequest,
    ) -> Result<ListOfferingsResult, RusotoError<ListOfferingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListOfferings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListOfferingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListOfferingsResult, _>()
    }

    /// <p>Gets information about projects.</p>
    async fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> Result<ListProjectsResult, RusotoError<ListProjectsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListProjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListProjectsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListProjectsResult, _>()
    }

    /// <p>Returns a list of all currently running remote access sessions.</p>
    async fn list_remote_access_sessions(
        &self,
        input: ListRemoteAccessSessionsRequest,
    ) -> Result<ListRemoteAccessSessionsResult, RusotoError<ListRemoteAccessSessionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.ListRemoteAccessSessions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRemoteAccessSessionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListRemoteAccessSessionsResult, _>()
    }

    /// <p>Gets information about runs, given an AWS Device Farm project ARN.</p>
    async fn list_runs(
        &self,
        input: ListRunsRequest,
    ) -> Result<ListRunsResult, RusotoError<ListRunsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListRuns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRunsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListRunsResult, _>()
    }

    /// <p>Gets information about samples, given an AWS Device Farm job ARN.</p>
    async fn list_samples(
        &self,
        input: ListSamplesRequest,
    ) -> Result<ListSamplesResult, RusotoError<ListSamplesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListSamples");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListSamplesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListSamplesResult, _>()
    }

    /// <p>Gets information about test suites for a given job.</p>
    async fn list_suites(
        &self,
        input: ListSuitesRequest,
    ) -> Result<ListSuitesResult, RusotoError<ListSuitesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListSuites");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListSuitesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListSuitesResult, _>()
    }

    /// <p>List the tags for an AWS Device Farm resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Gets a list of all Selenium testing projects in your account.</p>
    async fn list_test_grid_projects(
        &self,
        input: ListTestGridProjectsRequest,
    ) -> Result<ListTestGridProjectsResult, RusotoError<ListTestGridProjectsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListTestGridProjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTestGridProjectsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTestGridProjectsResult, _>()
    }

    /// <p>Returns a list of the actions taken in a <a>TestGridSession</a>.</p>
    async fn list_test_grid_session_actions(
        &self,
        input: ListTestGridSessionActionsRequest,
    ) -> Result<ListTestGridSessionActionsResult, RusotoError<ListTestGridSessionActionsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.ListTestGridSessionActions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTestGridSessionActionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListTestGridSessionActionsResult, _>()
    }

    /// <p>Retrieves a list of artifacts created during the session.</p>
    async fn list_test_grid_session_artifacts(
        &self,
        input: ListTestGridSessionArtifactsRequest,
    ) -> Result<ListTestGridSessionArtifactsResult, RusotoError<ListTestGridSessionArtifactsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.ListTestGridSessionArtifacts",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTestGridSessionArtifactsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListTestGridSessionArtifactsResult, _>()
    }

    /// <p>Retrieves a list of sessions for a <a>TestGridProject</a>.</p>
    async fn list_test_grid_sessions(
        &self,
        input: ListTestGridSessionsRequest,
    ) -> Result<ListTestGridSessionsResult, RusotoError<ListTestGridSessionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListTestGridSessions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTestGridSessionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTestGridSessionsResult, _>()
    }

    /// <p>Gets information about tests in a given test suite.</p>
    async fn list_tests(
        &self,
        input: ListTestsRequest,
    ) -> Result<ListTestsResult, RusotoError<ListTestsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListTests");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTestsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTestsResult, _>()
    }

    /// <p>Gets information about unique problems, such as exceptions or crashes.</p> <p>Unique problems are defined as a single instance of an error across a run, job, or suite. For example, if a call in your application consistently raises an exception (<code>OutOfBoundsException in MyActivity.java:386</code>), <code>ListUniqueProblems</code> returns a single entry instead of many individual entries for that exception.</p>
    async fn list_unique_problems(
        &self,
        input: ListUniqueProblemsRequest,
    ) -> Result<ListUniqueProblemsResult, RusotoError<ListUniqueProblemsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListUniqueProblems");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListUniqueProblemsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListUniqueProblemsResult, _>()
    }

    /// <p>Gets information about uploads, given an AWS Device Farm project ARN.</p>
    async fn list_uploads(
        &self,
        input: ListUploadsRequest,
    ) -> Result<ListUploadsResult, RusotoError<ListUploadsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListUploads");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListUploadsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListUploadsResult, _>()
    }

    /// <p>Returns information about all Amazon Virtual Private Cloud (VPC) endpoint configurations in the AWS account.</p>
    async fn list_vpce_configurations(
        &self,
        input: ListVPCEConfigurationsRequest,
    ) -> Result<ListVPCEConfigurationsResult, RusotoError<ListVPCEConfigurationsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListVPCEConfigurations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListVPCEConfigurationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListVPCEConfigurationsResult, _>()
    }

    /// <p>Immediately purchases offerings for an AWS account. Offerings renew with the latest total purchased quantity for an offering, unless the renewal was overridden. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. If you must be able to invoke this operation, contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a>.</p>
    async fn purchase_offering(
        &self,
        input: PurchaseOfferingRequest,
    ) -> Result<PurchaseOfferingResult, RusotoError<PurchaseOfferingError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.PurchaseOffering");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PurchaseOfferingError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PurchaseOfferingResult, _>()
    }

    /// <p>Explicitly sets the quantity of devices to renew for an offering, starting from the <code>effectiveDate</code> of the next period. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. If you must be able to invoke this operation, contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a>.</p>
    async fn renew_offering(
        &self,
        input: RenewOfferingRequest,
    ) -> Result<RenewOfferingResult, RusotoError<RenewOfferingError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.RenewOffering");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RenewOfferingError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RenewOfferingResult, _>()
    }

    /// <p>Schedules a run.</p>
    async fn schedule_run(
        &self,
        input: ScheduleRunRequest,
    ) -> Result<ScheduleRunResult, RusotoError<ScheduleRunError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.ScheduleRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ScheduleRunError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ScheduleRunResult, _>()
    }

    /// <p>Initiates a stop request for the current job. AWS Device Farm immediately stops the job on the device where tests have not started. You are not billed for this device. On the device where tests have started, setup suite and teardown suite tests run to completion on the device. You are billed for setup, teardown, and any tests that were in progress or already completed.</p>
    async fn stop_job(
        &self,
        input: StopJobRequest,
    ) -> Result<StopJobResult, RusotoError<StopJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.StopJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StopJobResult, _>()
    }

    /// <p>Ends a specified remote access session.</p>
    async fn stop_remote_access_session(
        &self,
        input: StopRemoteAccessSessionRequest,
    ) -> Result<StopRemoteAccessSessionResult, RusotoError<StopRemoteAccessSessionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.StopRemoteAccessSession",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopRemoteAccessSessionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StopRemoteAccessSessionResult, _>()
    }

    /// <p>Initiates a stop request for the current test run. AWS Device Farm immediately stops the run on devices where tests have not started. You are not billed for these devices. On devices where tests have started executing, setup suite and teardown suite tests run to completion on those devices. You are billed for setup, teardown, and any tests that were in progress or already completed.</p>
    async fn stop_run(
        &self,
        input: StopRunRequest,
    ) -> Result<StopRunResult, RusotoError<StopRunError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.StopRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopRunError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StopRunResult, _>()
    }

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are also deleted.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Deletes the specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Updates information about a private device instance.</p>
    async fn update_device_instance(
        &self,
        input: UpdateDeviceInstanceRequest,
    ) -> Result<UpdateDeviceInstanceResult, RusotoError<UpdateDeviceInstanceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateDeviceInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateDeviceInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateDeviceInstanceResult, _>()
    }

    /// <p>Modifies the name, description, and rules in a device pool given the attributes and the pool ARN. Rule updates are all-or-nothing, meaning they can only be updated as a whole (or not at all).</p>
    async fn update_device_pool(
        &self,
        input: UpdateDevicePoolRequest,
    ) -> Result<UpdateDevicePoolResult, RusotoError<UpdateDevicePoolError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateDevicePool");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateDevicePoolError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateDevicePoolResult, _>()
    }

    /// <p>Updates information about an existing private device instance profile.</p>
    async fn update_instance_profile(
        &self,
        input: UpdateInstanceProfileRequest,
    ) -> Result<UpdateInstanceProfileResult, RusotoError<UpdateInstanceProfileError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateInstanceProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateInstanceProfileError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateInstanceProfileResult, _>()
    }

    /// <p>Updates the network profile.</p>
    async fn update_network_profile(
        &self,
        input: UpdateNetworkProfileRequest,
    ) -> Result<UpdateNetworkProfileResult, RusotoError<UpdateNetworkProfileError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateNetworkProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateNetworkProfileError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateNetworkProfileResult, _>()
    }

    /// <p>Modifies the specified project name, given the project ARN and a new name.</p>
    async fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> Result<UpdateProjectResult, RusotoError<UpdateProjectError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateProjectError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateProjectResult, _>()
    }

    /// <p>Change details of a project.</p>
    async fn update_test_grid_project(
        &self,
        input: UpdateTestGridProjectRequest,
    ) -> Result<UpdateTestGridProjectResult, RusotoError<UpdateTestGridProjectError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateTestGridProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateTestGridProjectError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateTestGridProjectResult, _>()
    }

    /// <p>Updates an uploaded test spec.</p>
    async fn update_upload(
        &self,
        input: UpdateUploadRequest,
    ) -> Result<UpdateUploadResult, RusotoError<UpdateUploadError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateUpload");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateUploadError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateUploadResult, _>()
    }

    /// <p>Updates information about an Amazon Virtual Private Cloud (VPC) endpoint configuration.</p>
    async fn update_vpce_configuration(
        &self,
        input: UpdateVPCEConfigurationRequest,
    ) -> Result<UpdateVPCEConfigurationResult, RusotoError<UpdateVPCEConfigurationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.UpdateVPCEConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateVPCEConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateVPCEConfigurationResult, _>()
    }
}
