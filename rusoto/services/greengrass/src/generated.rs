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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateRoleToGroupRequest {
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The ARN of the role you wish to associate with this group. The existence of the role is not validated.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateRoleToGroupResponse {
    /// <p>The time, in milliseconds since the epoch, when the role ARN was associated with the group.</p>
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateServiceRoleToAccountRequest {
    /// <p>The ARN of the service role you wish to associate with your account.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateServiceRoleToAccountResponse {
    /// <p>The time when the service role was associated with the account.</p>
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
}

/// <p>Information about a bulk deployment. You cannot start a new bulk deployment while another one is still running or in a non-terminal state.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BulkDeployment {
    /// <p>The ARN of the bulk deployment.</p>
    #[serde(rename = "BulkDeploymentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_arn: Option<String>,
    /// <p>The ID of the bulk deployment.</p>
    #[serde(rename = "BulkDeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_id: Option<String>,
    /// <p>The time, in ISO format, when the deployment was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// <p>Relevant metrics on input records processed during bulk deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BulkDeploymentMetrics {
    /// <p>The total number of records that returned a non-retryable error. For example, this can occur if a group record from the input file uses an invalid format or specifies a nonexistent group version, or if the execution role doesn&#39;t grant permission to deploy a group or group version.</p>
    #[serde(rename = "InvalidInputRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_input_records: Option<i64>,
    /// <p>The total number of group records from the input file that have been processed so far, or attempted.</p>
    #[serde(rename = "RecordsProcessed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_processed: Option<i64>,
    /// <p>The total number of deployment attempts that returned a retryable error. For example, a retry is triggered if the attempt to deploy a group returns a throttling error. &#39;&#39;StartBulkDeployment&#39;&#39; retries a group deployment up to five times.</p>
    #[serde(rename = "RetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_attempts: Option<i64>,
}

/// <p>Information about an individual group deployment in a bulk deployment operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BulkDeploymentResult {
    /// <p>The time, in ISO format, when the deployment was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The ARN of the group deployment.</p>
    #[serde(rename = "DeploymentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    /// <p>The ID of the group deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The current status of the group deployment: &#39;&#39;InProgress&#39;&#39;, &#39;&#39;Building&#39;&#39;, &#39;&#39;Success&#39;&#39;, or &#39;&#39;Failure&#39;&#39;.</p>
    #[serde(rename = "DeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    /// <p>The type of the deployment.</p>
    #[serde(rename = "DeploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// <p>Details about the error.</p>
    #[serde(rename = "ErrorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<ErrorDetail>>,
    /// <p>The error message for a failed deployment</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ARN of the Greengrass group.</p>
    #[serde(rename = "GroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
}

/// <p>Information about a Greengrass core&#39;s connectivity.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectivityInfo {
    /// <p>The endpoint for the Greengrass core. Can be an IP address or DNS.</p>
    #[serde(rename = "HostAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    /// <p>The ID of the connectivity information.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Metadata for this endpoint.</p>
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// <p>The port of the Greengrass core. Usually 8883.</p>
    #[serde(rename = "PortNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i64>,
}

/// <p>Information about a connector. Connectors run on the Greengrass core and contain built-in integration with local infrastructure, device protocols, AWS, and other cloud services.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Connector {
    /// <p>The ARN of the connector.</p>
    #[serde(rename = "ConnectorArn")]
    pub connector_arn: String,
    /// <p>A descriptive or arbitrary ID for the connector. This value must be unique within the connector definition version. Max length is 128 characters with pattern [a-zA-Z0-9:_-]+.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The parameters or configuration that the connector uses.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about the connector definition version, which is a container for connectors.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectorDefinitionVersion {
    /// <p>A list of references to connectors in this version, with their corresponding configuration settings.</p>
    #[serde(rename = "Connectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<Connector>>,
}

/// <p>Information about a core.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Core {
    /// <p>The ARN of the certificate associated with the core.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// <p>A descriptive or arbitrary ID for the core. This value must be unique within the core definition version. Max length is 128 characters with pattern &#39;&#39;[a-zA-Z0-9:_-]+&#39;&#39;.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>If true, the core&#39;s local shadow is automatically synced with the cloud.</p>
    #[serde(rename = "SyncShadow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_shadow: Option<bool>,
    /// <p>The ARN of the thing which is the core.</p>
    #[serde(rename = "ThingArn")]
    pub thing_arn: String,
}

/// <p>Information about a core definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreDefinitionVersion {
    /// <p>A list of cores in the core definition version.</p>
    #[serde(rename = "Cores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<Vec<Core>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConnectorDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the connector definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<ConnectorDefinitionVersion>,
    /// <p>The name of the connector definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) to add to the new resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConnectorDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConnectorDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the connector definition.</p>
    #[serde(rename = "ConnectorDefinitionId")]
    pub connector_definition_id: String,
    /// <p>A list of references to connectors in this version, with their corresponding configuration settings.</p>
    #[serde(rename = "Connectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<Connector>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConnectorDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information needed to create a core definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCoreDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the core definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<CoreDefinitionVersion>,
    /// <p>The name of the core definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) to add to the new resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCoreDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCoreDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// <p>A list of cores in the core definition version.</p>
    #[serde(rename = "Cores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<Vec<Core>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCoreDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeploymentRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the deployment if you wish to redeploy a previous deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The type of deployment. When used for &#39;&#39;CreateDeployment&#39;&#39;, only &#39;&#39;NewDeployment&#39;&#39; and &#39;&#39;Redeployment&#39;&#39; are valid.</p>
    #[serde(rename = "DeploymentType")]
    pub deployment_type: String,
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The ID of the group version to be deployed.</p>
    #[serde(rename = "GroupVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeploymentResponse {
    /// <p>The ARN of the deployment.</p>
    #[serde(rename = "DeploymentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    /// <p>The ID of the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeviceDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the device definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<DeviceDefinitionVersion>,
    /// <p>The name of the device definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) to add to the new resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeviceDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeviceDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// <p>A list of devices in the definition version.</p>
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeviceDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFunctionDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the function definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<FunctionDefinitionVersion>,
    /// <p>The name of the function definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) to add to the new resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFunctionDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information needed to create a function definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFunctionDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The default configuration that applies to all Lambda functions in this function definition version. Individual Lambda functions can override these settings.</p>
    #[serde(rename = "DefaultConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_config: Option<FunctionDefaultConfig>,
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// <p>A list of Lambda functions in this function definition version.</p>
    #[serde(rename = "Functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFunctionDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGroupCertificateAuthorityRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGroupCertificateAuthorityResponse {
    /// <p>The ARN of the group certificate authority.</p>
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGroupRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the group.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<GroupVersion>,
    /// <p>The name of the group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) to add to the new resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGroupResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGroupVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ARN of the connector definition version for this group.</p>
    #[serde(rename = "ConnectorDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_definition_version_arn: Option<String>,
    /// <p>The ARN of the core definition version for this group.</p>
    #[serde(rename = "CoreDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_definition_version_arn: Option<String>,
    /// <p>The ARN of the device definition version for this group.</p>
    #[serde(rename = "DeviceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_definition_version_arn: Option<String>,
    /// <p>The ARN of the function definition version for this group.</p>
    #[serde(rename = "FunctionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_definition_version_arn: Option<String>,
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The ARN of the logger definition version for this group.</p>
    #[serde(rename = "LoggerDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger_definition_version_arn: Option<String>,
    /// <p>The ARN of the resource definition version for this group.</p>
    #[serde(rename = "ResourceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_definition_version_arn: Option<String>,
    /// <p>The ARN of the subscription definition version for this group.</p>
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_definition_version_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGroupVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLoggerDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the logger definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<LoggerDefinitionVersion>,
    /// <p>The name of the logger definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) to add to the new resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLoggerDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLoggerDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// <p>A list of loggers.</p>
    #[serde(rename = "Loggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loggers: Option<Vec<Logger>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLoggerDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the resource definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<ResourceDefinitionVersion>,
    /// <p>The name of the resource definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) to add to the new resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateResourceDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
    /// <p>A list of resources.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateResourceDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSoftwareUpdateJobRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "S3UrlSignerRole")]
    pub s3_url_signer_role: String,
    #[serde(rename = "SoftwareToUpdate")]
    pub software_to_update: String,
    #[serde(rename = "UpdateAgentLogLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_agent_log_level: Option<String>,
    #[serde(rename = "UpdateTargets")]
    pub update_targets: Vec<String>,
    #[serde(rename = "UpdateTargetsArchitecture")]
    pub update_targets_architecture: String,
    #[serde(rename = "UpdateTargetsOperatingSystem")]
    pub update_targets_operating_system: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSoftwareUpdateJobResponse {
    /// <p>The IoT Job ARN corresponding to this update.</p>
    #[serde(rename = "IotJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_job_arn: Option<String>,
    /// <p>The IoT Job Id corresponding to this update.</p>
    #[serde(rename = "IotJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSubscriptionDefinitionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>Information about the initial version of the subscription definition.</p>
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<SubscriptionDefinitionVersion>,
    /// <p>The name of the subscription definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) to add to the new resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSubscriptionDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSubscriptionDefinitionVersionRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
    /// <p>A list of subscriptions.</p>
    #[serde(rename = "Subscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSubscriptionDefinitionVersionResponse {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about a definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DefinitionInformation {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) attached to the resource arn.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConnectorDefinitionRequest {
    /// <p>The ID of the connector definition.</p>
    #[serde(rename = "ConnectorDefinitionId")]
    pub connector_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConnectorDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCoreDefinitionRequest {
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCoreDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDeviceDefinitionRequest {
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDeviceDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFunctionDefinitionRequest {
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFunctionDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGroupRequest {
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLoggerDefinitionRequest {
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLoggerDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResourceDefinitionRequest {
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteResourceDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSubscriptionDefinitionRequest {
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSubscriptionDefinitionResponse {}

/// <p>Information about a deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Deployment {
    /// <p>The time, in milliseconds since the epoch, when the deployment was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The ARN of the deployment.</p>
    #[serde(rename = "DeploymentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    /// <p>The ID of the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The type of the deployment.</p>
    #[serde(rename = "DeploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// <p>The ARN of the group for this deployment.</p>
    #[serde(rename = "GroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
}

/// <p>Information about a device.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    /// <p>The ARN of the certificate associated with the device.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// <p>A descriptive or arbitrary ID for the device. This value must be unique within the device definition version. Max length is 128 characters with pattern &#39;&#39;[a-zA-Z0-9:_-]+&#39;&#39;.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>If true, the device&#39;s local shadow will be automatically synced with the cloud.</p>
    #[serde(rename = "SyncShadow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_shadow: Option<bool>,
    /// <p>The thing ARN of the device.</p>
    #[serde(rename = "ThingArn")]
    pub thing_arn: String,
}

/// <p>Information about a device definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceDefinitionVersion {
    /// <p>A list of devices in the definition version.</p>
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateRoleFromGroupRequest {
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateRoleFromGroupResponse {
    /// <p>The time, in milliseconds since the epoch, when the role was disassociated from the group.</p>
    #[serde(rename = "DisassociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociated_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateServiceRoleFromAccountRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateServiceRoleFromAccountResponse {
    /// <p>The time when the service role was disassociated from the account.</p>
    #[serde(rename = "DisassociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociated_at: Option<String>,
}

/// <p>Details about the error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorDetail {
    /// <p>A detailed error code.</p>
    #[serde(rename = "DetailedErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_error_code: Option<String>,
    /// <p>A detailed error message.</p>
    #[serde(rename = "DetailedErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_error_message: Option<String>,
}

/// <p>Information about a Lambda function.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    /// <p>The ARN of the Lambda function.</p>
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    /// <p>The configuration of the Lambda function.</p>
    #[serde(rename = "FunctionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_configuration: Option<FunctionConfiguration>,
    /// <p>A descriptive or arbitrary ID for the function. This value must be unique within the function definition version. Max length is 128 characters with pattern &#39;&#39;[a-zA-Z0-9:_-]+&#39;&#39;.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// <p>The configuration of the Lambda function.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionConfiguration {
    /// <p>The expected encoding type of the input payload for the function. The default is &#39;&#39;json&#39;&#39;.</p>
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// <p>The environment configuration of the function.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<FunctionConfigurationEnvironment>,
    /// <p>The execution arguments.</p>
    #[serde(rename = "ExecArgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_args: Option<String>,
    /// <p>The name of the function executable.</p>
    #[serde(rename = "Executable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable: Option<String>,
    /// <p>The memory size, in KB, which the function requires. This setting is not applicable and should be cleared when you run the Lambda function without containerization.</p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>True if the function is pinned. Pinned means the function is long-lived and starts when the core starts.</p>
    #[serde(rename = "Pinned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    /// <p>The allowed function execution time, after which Lambda should terminate the function. This timeout still applies to pinned Lambda functions for each request.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// <p>The environment configuration of the function.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionConfigurationEnvironment {
    /// <p>If true, the Lambda function is allowed to access the host&#39;s /sys folder. Use this when the Lambda function needs to read device information from /sys. This setting applies only when you run the Lambda function in a Greengrass container.</p>
    #[serde(rename = "AccessSysfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_sysfs: Option<bool>,
    /// <p>Configuration related to executing the Lambda function</p>
    #[serde(rename = "Execution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<FunctionExecutionConfig>,
    /// <p>A list of the resources, with their permissions, to which the Lambda function will be granted access. A Lambda function can have at most 10 resources. ResourceAccessPolicies apply only when you run the Lambda function in a Greengrass container.</p>
    #[serde(rename = "ResourceAccessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_access_policies: Option<Vec<ResourceAccessPolicy>>,
    /// <p>Environment variables for the Lambda function&#39;s configuration.</p>
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The default configuration that applies to all Lambda functions in the group. Individual Lambda functions can override these settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDefaultConfig {
    #[serde(rename = "Execution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<FunctionDefaultExecutionConfig>,
}

/// <p>Configuration information that specifies how a Lambda function runs. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDefaultExecutionConfig {
    #[serde(rename = "IsolationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation_mode: Option<String>,
    #[serde(rename = "RunAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as: Option<FunctionRunAsConfig>,
}

/// <p>Information about a function definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDefinitionVersion {
    /// <p>The default configuration that applies to all Lambda functions in this function definition version. Individual Lambda functions can override these settings.</p>
    #[serde(rename = "DefaultConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_config: Option<FunctionDefaultConfig>,
    /// <p>A list of Lambda functions in this function definition version.</p>
    #[serde(rename = "Functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
}

/// <p>Configuration information that specifies how a Lambda function runs. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionExecutionConfig {
    #[serde(rename = "IsolationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation_mode: Option<String>,
    #[serde(rename = "RunAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as: Option<FunctionRunAsConfig>,
}

/// <p>Specifies the user and group whose permissions are used when running the Lambda function. You can specify one or both values to override the default values. We recommend that you avoid running as root unless absolutely necessary to minimize the risk of unintended changes or malicious attacks. To run as root, you must set &#39;&#39;IsolationMode&#39;&#39; to &#39;&#39;NoContainer&#39;&#39; and update config.json in &#39;&#39;greengrass-root/config&#39;&#39; to set &#39;&#39;allowFunctionsToRunAsRoot&#39;&#39; to &#39;&#39;yes&#39;&#39;.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionRunAsConfig {
    /// <p>The group ID whose permissions are used to run a Lambda function.</p>
    #[serde(rename = "Gid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<i64>,
    /// <p>The user ID whose permissions are used to run a Lambda function.</p>
    #[serde(rename = "Uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAssociatedRoleRequest {
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAssociatedRoleResponse {
    /// <p>The time when the role was associated with the group.</p>
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
    /// <p>The ARN of the role that is associated with the group.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBulkDeploymentStatusRequest {
    /// <p>The ID of the bulk deployment.</p>
    #[serde(rename = "BulkDeploymentId")]
    pub bulk_deployment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBulkDeploymentStatusResponse {
    /// <p>Relevant metrics on input records processed during bulk deployment.</p>
    #[serde(rename = "BulkDeploymentMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_metrics: Option<BulkDeploymentMetrics>,
    /// <p>The status of the bulk deployment.</p>
    #[serde(rename = "BulkDeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_status: Option<String>,
    /// <p>The time, in ISO format, when the deployment was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>Error details</p>
    #[serde(rename = "ErrorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<ErrorDetail>>,
    /// <p>Error message</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Tag(s) attached to the resource arn.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectivityInfoRequest {
    /// <p>The thing name.</p>
    #[serde(rename = "ThingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConnectivityInfoResponse {
    /// <p>Connectivity info list.</p>
    #[serde(rename = "ConnectivityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<Vec<ConnectivityInfo>>,
    /// <p>A message about the connectivity info request.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectorDefinitionRequest {
    /// <p>The ID of the connector definition.</p>
    #[serde(rename = "ConnectorDefinitionId")]
    pub connector_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConnectorDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) attached to the resource arn.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectorDefinitionVersionRequest {
    /// <p>The ID of the connector definition.</p>
    #[serde(rename = "ConnectorDefinitionId")]
    pub connector_definition_id: String,
    /// <p>The ID of the connector definition version. This value maps to the &#39;&#39;Version&#39;&#39; property of the corresponding &#39;&#39;VersionInformation&#39;&#39; object, which is returned by &#39;&#39;ListConnectorDefinitionVersions&#39;&#39; requests. If the version is the last one that was associated with a connector definition, the value also maps to the &#39;&#39;LatestVersion&#39;&#39; property of the corresponding &#39;&#39;DefinitionInformation&#39;&#39; object.</p>
    #[serde(rename = "ConnectorDefinitionVersionId")]
    pub connector_definition_version_id: String,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConnectorDefinitionVersionResponse {
    /// <p>The ARN of the connector definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the connector definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the connector definition version.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ConnectorDefinitionVersion>,
    /// <p>The ID of the connector definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The version of the connector definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCoreDefinitionRequest {
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCoreDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) attached to the resource arn.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCoreDefinitionVersionRequest {
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// <p>The ID of the core definition version. This value maps to the &#39;&#39;Version&#39;&#39; property of the corresponding &#39;&#39;VersionInformation&#39;&#39; object, which is returned by &#39;&#39;ListCoreDefinitionVersions&#39;&#39; requests. If the version is the last one that was associated with a core definition, the value also maps to the &#39;&#39;LatestVersion&#39;&#39; property of the corresponding &#39;&#39;DefinitionInformation&#39;&#39; object.</p>
    #[serde(rename = "CoreDefinitionVersionId")]
    pub core_definition_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCoreDefinitionVersionResponse {
    /// <p>The ARN of the core definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the core definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the core definition version.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<CoreDefinitionVersion>,
    /// <p>The ID of the core definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The version of the core definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeploymentStatusRequest {
    /// <p>The ID of the deployment.</p>
    #[serde(rename = "DeploymentId")]
    pub deployment_id: String,
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeploymentStatusResponse {
    /// <p>The status of the deployment: &#39;&#39;InProgress&#39;&#39;, &#39;&#39;Building&#39;&#39;, &#39;&#39;Success&#39;&#39;, or &#39;&#39;Failure&#39;&#39;.</p>
    #[serde(rename = "DeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    /// <p>The type of the deployment.</p>
    #[serde(rename = "DeploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// <p>Error details</p>
    #[serde(rename = "ErrorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<ErrorDetail>>,
    /// <p>Error message</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the deployment status was updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeviceDefinitionRequest {
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeviceDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) attached to the resource arn.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeviceDefinitionVersionRequest {
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// <p>The ID of the device definition version. This value maps to the &#39;&#39;Version&#39;&#39; property of the corresponding &#39;&#39;VersionInformation&#39;&#39; object, which is returned by &#39;&#39;ListDeviceDefinitionVersions&#39;&#39; requests. If the version is the last one that was associated with a device definition, the value also maps to the &#39;&#39;LatestVersion&#39;&#39; property of the corresponding &#39;&#39;DefinitionInformation&#39;&#39; object.</p>
    #[serde(rename = "DeviceDefinitionVersionId")]
    pub device_definition_version_id: String,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeviceDefinitionVersionResponse {
    /// <p>The ARN of the device definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the device definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the device definition version.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DeviceDefinitionVersion>,
    /// <p>The ID of the device definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The version of the device definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFunctionDefinitionRequest {
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFunctionDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) attached to the resource arn.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFunctionDefinitionVersionRequest {
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// <p>The ID of the function definition version. This value maps to the &#39;&#39;Version&#39;&#39; property of the corresponding &#39;&#39;VersionInformation&#39;&#39; object, which is returned by &#39;&#39;ListFunctionDefinitionVersions&#39;&#39; requests. If the version is the last one that was associated with a function definition, the value also maps to the &#39;&#39;LatestVersion&#39;&#39; property of the corresponding &#39;&#39;DefinitionInformation&#39;&#39; object.</p>
    #[serde(rename = "FunctionDefinitionVersionId")]
    pub function_definition_version_id: String,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFunctionDefinitionVersionResponse {
    /// <p>The ARN of the function definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the function definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information on the definition.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<FunctionDefinitionVersion>,
    /// <p>The ID of the function definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The version of the function definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupCertificateAuthorityRequest {
    /// <p>The ID of the certificate authority.</p>
    #[serde(rename = "CertificateAuthorityId")]
    pub certificate_authority_id: String,
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupCertificateAuthorityResponse {
    /// <p>The ARN of the certificate authority for the group.</p>
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
    /// <p>The ID of the certificate authority for the group.</p>
    #[serde(rename = "GroupCertificateAuthorityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_id: Option<String>,
    /// <p>The PEM encoded certificate for the group.</p>
    #[serde(rename = "PemEncodedCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem_encoded_certificate: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupCertificateConfigurationRequest {
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupCertificateConfigurationResponse {
    /// <p>The amount of time remaining before the certificate authority expires, in milliseconds.</p>
    #[serde(rename = "CertificateAuthorityExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_expiry_in_milliseconds: Option<String>,
    /// <p>The amount of time remaining before the certificate expires, in milliseconds.</p>
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    /// <p>The ID of the group certificate configuration.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupRequest {
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) attached to the resource arn.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupVersionRequest {
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The ID of the group version. This value maps to the &#39;&#39;Version&#39;&#39; property of the corresponding &#39;&#39;VersionInformation&#39;&#39; object, which is returned by &#39;&#39;ListGroupVersions&#39;&#39; requests. If the version is the last one that was associated with a group, the value also maps to the &#39;&#39;LatestVersion&#39;&#39; property of the corresponding &#39;&#39;GroupInformation&#39;&#39; object.</p>
    #[serde(rename = "GroupVersionId")]
    pub group_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupVersionResponse {
    /// <p>The ARN of the group version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the group version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the group version definition.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<GroupVersion>,
    /// <p>The ID of the group version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID for the version of the group.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLoggerDefinitionRequest {
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLoggerDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) attached to the resource arn.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLoggerDefinitionVersionRequest {
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// <p>The ID of the logger definition version. This value maps to the &#39;&#39;Version&#39;&#39; property of the corresponding &#39;&#39;VersionInformation&#39;&#39; object, which is returned by &#39;&#39;ListLoggerDefinitionVersions&#39;&#39; requests. If the version is the last one that was associated with a logger definition, the value also maps to the &#39;&#39;LatestVersion&#39;&#39; property of the corresponding &#39;&#39;DefinitionInformation&#39;&#39; object.</p>
    #[serde(rename = "LoggerDefinitionVersionId")]
    pub logger_definition_version_id: String,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLoggerDefinitionVersionResponse {
    /// <p>The ARN of the logger definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the logger definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the logger definition version.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<LoggerDefinitionVersion>,
    /// <p>The ID of the logger definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The version of the logger definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourceDefinitionRequest {
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourceDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) attached to the resource arn.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourceDefinitionVersionRequest {
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
    /// <p>The ID of the resource definition version. This value maps to the &#39;&#39;Version&#39;&#39; property of the corresponding &#39;&#39;VersionInformation&#39;&#39; object, which is returned by &#39;&#39;ListResourceDefinitionVersions&#39;&#39; requests. If the version is the last one that was associated with a resource definition, the value also maps to the &#39;&#39;LatestVersion&#39;&#39; property of the corresponding &#39;&#39;DefinitionInformation&#39;&#39; object.</p>
    #[serde(rename = "ResourceDefinitionVersionId")]
    pub resource_definition_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourceDefinitionVersionResponse {
    /// <p>Arn of the resource definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the resource definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the definition.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ResourceDefinitionVersion>,
    /// <p>The ID of the resource definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The version of the resource definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetServiceRoleForAccountRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetServiceRoleForAccountResponse {
    /// <p>The time when the service role was associated with the account.</p>
    #[serde(rename = "AssociatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
    /// <p>The ARN of the role which is associated with the account.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSubscriptionDefinitionRequest {
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSubscriptionDefinitionResponse {
    /// <p>The ARN of the definition.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the definition.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the definition was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the definition.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Tag(s) attached to the resource arn.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSubscriptionDefinitionVersionRequest {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
    /// <p>The ID of the subscription definition version. This value maps to the &#39;&#39;Version&#39;&#39; property of the corresponding &#39;&#39;VersionInformation&#39;&#39; object, which is returned by &#39;&#39;ListSubscriptionDefinitionVersions&#39;&#39; requests. If the version is the last one that was associated with a subscription definition, the value also maps to the &#39;&#39;LatestVersion&#39;&#39; property of the corresponding &#39;&#39;DefinitionInformation&#39;&#39; object.</p>
    #[serde(rename = "SubscriptionDefinitionVersionId")]
    pub subscription_definition_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSubscriptionDefinitionVersionResponse {
    /// <p>The ARN of the subscription definition version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the subscription definition version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>Information about the subscription definition version.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<SubscriptionDefinitionVersion>,
    /// <p>The ID of the subscription definition version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The version of the subscription definition version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about a certificate authority for a group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GroupCertificateAuthorityProperties {
    /// <p>The ARN of the certificate authority for the group.</p>
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
    /// <p>The ID of the certificate authority for the group.</p>
    #[serde(rename = "GroupCertificateAuthorityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_id: Option<String>,
}

/// <p>Information about a group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GroupInformation {
    /// <p>The ARN of the group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the group was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the group was last updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    /// <p>The ID of the latest version associated with the group.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The ARN of the latest version associated with the group.</p>
    #[serde(rename = "LatestVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Group owner related settings for local resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupOwnerSetting {
    /// <p>If true, AWS IoT Greengrass automatically adds the specified Linux OS group owner of the resource to the Lambda process privileges. Thus the Lambda process will have the file access permissions of the added Linux group.</p>
    #[serde(rename = "AutoAddGroupOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_add_group_owner: Option<bool>,
    /// <p>The name of the Linux OS group whose privileges will be added to the Lambda process. This field is optional.</p>
    #[serde(rename = "GroupOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_owner: Option<String>,
}

/// <p>Information about a group version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupVersion {
    /// <p>The ARN of the connector definition version for this group.</p>
    #[serde(rename = "ConnectorDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_definition_version_arn: Option<String>,
    /// <p>The ARN of the core definition version for this group.</p>
    #[serde(rename = "CoreDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_definition_version_arn: Option<String>,
    /// <p>The ARN of the device definition version for this group.</p>
    #[serde(rename = "DeviceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_definition_version_arn: Option<String>,
    /// <p>The ARN of the function definition version for this group.</p>
    #[serde(rename = "FunctionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_definition_version_arn: Option<String>,
    /// <p>The ARN of the logger definition version for this group.</p>
    #[serde(rename = "LoggerDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger_definition_version_arn: Option<String>,
    /// <p>The ARN of the resource definition version for this group.</p>
    #[serde(rename = "ResourceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_definition_version_arn: Option<String>,
    /// <p>The ARN of the subscription definition version for this group.</p>
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_definition_version_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListBulkDeploymentDetailedReportsRequest {
    /// <p>The ID of the bulk deployment.</p>
    #[serde(rename = "BulkDeploymentId")]
    pub bulk_deployment_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBulkDeploymentDetailedReportsResponse {
    /// <p>A list of the individual group deployments in the bulk deployment operation.</p>
    #[serde(rename = "Deployments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<BulkDeploymentResult>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListBulkDeploymentsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBulkDeploymentsResponse {
    /// <p>A list of bulk deployments.</p>
    #[serde(rename = "BulkDeployments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployments: Option<Vec<BulkDeployment>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListConnectorDefinitionVersionsRequest {
    /// <p>The ID of the connector definition.</p>
    #[serde(rename = "ConnectorDefinitionId")]
    pub connector_definition_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConnectorDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListConnectorDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConnectorDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCoreDefinitionVersionsRequest {
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCoreDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCoreDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCoreDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeploymentsRequest {
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeploymentsResponse {
    /// <p>A list of deployments for the requested groups.</p>
    #[serde(rename = "Deployments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<Deployment>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeviceDefinitionVersionsRequest {
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeviceDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeviceDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeviceDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFunctionDefinitionVersionsRequest {
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFunctionDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFunctionDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFunctionDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGroupCertificateAuthoritiesRequest {
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGroupCertificateAuthoritiesResponse {
    /// <p>A list of certificate authorities associated with the group.</p>
    #[serde(rename = "GroupCertificateAuthorities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authorities: Option<Vec<GroupCertificateAuthorityProperties>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGroupVersionsRequest {
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGroupVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGroupsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGroupsResponse {
    /// <p>Information about a group.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLoggerDefinitionVersionsRequest {
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLoggerDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLoggerDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLoggerDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceDefinitionVersionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourceDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourceDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSubscriptionDefinitionVersionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSubscriptionDefinitionVersionsResponse {
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about a version.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSubscriptionDefinitionsRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSubscriptionDefinitionsResponse {
    /// <p>Information about a definition.</p>
    #[serde(rename = "Definitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    /// <p>The token for the next set of results, or &#39;&#39;null&#39;&#39; if there are no additional results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Attributes that define a local device resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalDeviceResourceData {
    /// <p>Group/owner related settings for local resources.</p>
    #[serde(rename = "GroupOwnerSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_owner_setting: Option<GroupOwnerSetting>,
    /// <p>The local absolute path of the device resource. The source path for a device resource can refer only to a character device or block device under &#39;&#39;/dev&#39;&#39;.</p>
    #[serde(rename = "SourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

/// <p>Attributes that define a local volume resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalVolumeResourceData {
    /// <p>The absolute local path of the resource inside the Lambda environment.</p>
    #[serde(rename = "DestinationPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_path: Option<String>,
    /// <p>Allows you to configure additional group privileges for the Lambda process. This field is optional.</p>
    #[serde(rename = "GroupOwnerSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_owner_setting: Option<GroupOwnerSetting>,
    /// <p>The local absolute path of the volume resource on the host. The source path for a volume resource type cannot start with &#39;&#39;/sys&#39;&#39;.</p>
    #[serde(rename = "SourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

/// <p>Information about a logger</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logger {
    /// <p>The component that will be subject to logging.</p>
    #[serde(rename = "Component")]
    pub component: String,
    /// <p>A descriptive or arbitrary ID for the logger. This value must be unique within the logger definition version. Max length is 128 characters with pattern &#39;&#39;[a-zA-Z0-9:_-]+&#39;&#39;.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The level of the logs.</p>
    #[serde(rename = "Level")]
    pub level: String,
    /// <p>The amount of file space, in KB, to use if the local file system is used for logging purposes.</p>
    #[serde(rename = "Space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<i64>,
    /// <p>The type of log output which will be used.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Information about a logger definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggerDefinitionVersion {
    /// <p>A list of loggers.</p>
    #[serde(rename = "Loggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loggers: Option<Vec<Logger>>,
}

/// <p>Information needed to reset deployments.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResetDeploymentsRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>If true, performs a best-effort only core reset.</p>
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResetDeploymentsResponse {
    /// <p>The ARN of the deployment.</p>
    #[serde(rename = "DeploymentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    /// <p>The ID of the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

/// <p>Information about a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    /// <p>The resource ID, used to refer to a resource in the Lambda function configuration. Max length is 128 characters with pattern &#39;&#39;[a-zA-Z0-9:_-]+&#39;&#39;. This must be unique within a Greengrass group.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The descriptive resource name, which is displayed on the AWS IoT Greengrass console. Max length 128 characters with pattern &#39;&#39;[a-zA-Z0-9:_-]+&#39;&#39;. This must be unique within a Greengrass group.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A container of data for all resource types.</p>
    #[serde(rename = "ResourceDataContainer")]
    pub resource_data_container: ResourceDataContainer,
}

/// <p>A policy used by the function to access a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceAccessPolicy {
    /// <p>The permissions that the Lambda function has to the resource. Can be one of &#39;&#39;rw&#39;&#39; (read/write) or &#39;&#39;ro&#39;&#39; (read-only).</p>
    #[serde(rename = "Permission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    /// <p>The ID of the resource. (This ID is assigned to the resource when you create the resource definiton.)</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

/// <p>A container for resource data. The container takes only one of the following supported resource data types: &#39;&#39;LocalDeviceResourceData&#39;&#39;, &#39;&#39;LocalVolumeResourceData&#39;&#39;, &#39;&#39;SageMakerMachineLearningModelResourceData&#39;&#39;, &#39;&#39;S3MachineLearningModelResourceData&#39;&#39;, &#39;&#39;SecretsManagerSecretResourceData&#39;&#39;.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDataContainer {
    /// <p>Attributes that define the local device resource.</p>
    #[serde(rename = "LocalDeviceResourceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_device_resource_data: Option<LocalDeviceResourceData>,
    /// <p>Attributes that define the local volume resource.</p>
    #[serde(rename = "LocalVolumeResourceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_volume_resource_data: Option<LocalVolumeResourceData>,
    /// <p>Attributes that define an Amazon S3 machine learning resource.</p>
    #[serde(rename = "S3MachineLearningModelResourceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_machine_learning_model_resource_data: Option<S3MachineLearningModelResourceData>,
    /// <p>Attributes that define an Amazon SageMaker machine learning resource.</p>
    #[serde(rename = "SageMakerMachineLearningModelResourceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_machine_learning_model_resource_data:
        Option<SageMakerMachineLearningModelResourceData>,
    /// <p>Attributes that define a secret resource, which references a secret from AWS Secrets Manager.</p>
    #[serde(rename = "SecretsManagerSecretResourceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_resource_data: Option<SecretsManagerSecretResourceData>,
}

/// <p>Information about a resource definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDefinitionVersion {
    /// <p>A list of resources.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

/// <p>Attributes that define an Amazon S3 machine learning resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3MachineLearningModelResourceData {
    /// <p>The absolute local path of the resource inside the Lambda environment.</p>
    #[serde(rename = "DestinationPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_path: Option<String>,
    /// <p>The URI of the source model in an S3 bucket. The model package must be in tar.gz or .zip format.</p>
    #[serde(rename = "S3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

/// <p>Attributes that define an Amazon SageMaker machine learning resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SageMakerMachineLearningModelResourceData {
    /// <p>The absolute local path of the resource inside the Lambda environment.</p>
    #[serde(rename = "DestinationPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_path: Option<String>,
    /// <p>The ARN of the Amazon SageMaker training job that represents the source model.</p>
    #[serde(rename = "SageMakerJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_job_arn: Option<String>,
}

/// <p>Attributes that define a secret resource, which references a secret from AWS Secrets Manager. AWS IoT Greengrass stores a local, encrypted copy of the secret on the Greengrass core, where it can be securely accessed by connectors and Lambda functions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretsManagerSecretResourceData {
    /// <p>The ARN of the Secrets Manager secret to make available on the core. The value of the secret&#39;s latest version (represented by the &#39;&#39;AWSCURRENT&#39;&#39; staging label) is included by default.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Optional. The staging labels whose values you want to make available on the core, in addition to &#39;&#39;AWSCURRENT&#39;&#39;.</p>
    #[serde(rename = "AdditionalStagingLabelsToDownload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_staging_labels_to_download: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartBulkDeploymentRequest {
    /// <p>A client token used to correlate requests and responses.</p>
    #[serde(rename = "AmznClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    /// <p>The ARN of the execution role to associate with the bulk deployment operation. This IAM role must allow the &#39;&#39;greengrass:CreateDeployment&#39;&#39; action for all group versions that are listed in the input file. This IAM role must have access to the S3 bucket containing the input file.</p>
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,
    /// <p>The URI of the input file contained in the S3 bucket. The execution role must have &#39;&#39;getObject&#39;&#39; permissions on this bucket to access the input file. The input file is a JSON-serialized, line delimited file with UTF-8 encoding that provides a list of group and version IDs and the deployment type. This file must be less than 100 MB. Currently, AWS IoT Greengrass supports only &#39;&#39;NewDeployment&#39;&#39; deployment types.</p>
    #[serde(rename = "InputFileUri")]
    pub input_file_uri: String,
    /// <p>Tag(s) to add to the new resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartBulkDeploymentResponse {
    /// <p>The ARN of the bulk deployment.</p>
    #[serde(rename = "BulkDeploymentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_arn: Option<String>,
    /// <p>The ID of the bulk deployment.</p>
    #[serde(rename = "BulkDeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopBulkDeploymentRequest {
    /// <p>The ID of the bulk deployment.</p>
    #[serde(rename = "BulkDeploymentId")]
    pub bulk_deployment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopBulkDeploymentResponse {}

/// <p>Information about a subscription.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    /// <p>A descriptive or arbitrary ID for the subscription. This value must be unique within the subscription definition version. Max length is 128 characters with pattern &#39;&#39;[a-zA-Z0-9:_-]+&#39;&#39;.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The source of the subscription. Can be a thing ARN, a Lambda function ARN, a connector ARN, &#39;cloud&#39; (which represents the AWS IoT cloud), or &#39;GGShadowService&#39;.</p>
    #[serde(rename = "Source")]
    pub source: String,
    /// <p>The MQTT topic used to route the message.</p>
    #[serde(rename = "Subject")]
    pub subject: String,
    /// <p>Where the message is sent to. Can be a thing ARN, a Lambda function ARN, a connector ARN, &#39;cloud&#39; (which represents the AWS IoT cloud), or &#39;GGShadowService&#39;.</p>
    #[serde(rename = "Target")]
    pub target: String,
}

/// <p>Information about a subscription definition version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionDefinitionVersion {
    /// <p>A list of subscriptions.</p>
    #[serde(rename = "Subscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}

/// <p>A map of the key-value pairs for the resource tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>An array of tag keys to delete</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Connectivity information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateConnectivityInfoRequest {
    /// <p>A list of connectivity info.</p>
    #[serde(rename = "ConnectivityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<Vec<ConnectivityInfo>>,
    /// <p>The thing name.</p>
    #[serde(rename = "ThingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConnectivityInfoResponse {
    /// <p>A message about the connectivity info update request.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The new version of the connectivity info.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateConnectorDefinitionRequest {
    /// <p>The ID of the connector definition.</p>
    #[serde(rename = "ConnectorDefinitionId")]
    pub connector_definition_id: String,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConnectorDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCoreDefinitionRequest {
    /// <p>The ID of the core definition.</p>
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCoreDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDeviceDefinitionRequest {
    /// <p>The ID of the device definition.</p>
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDeviceDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFunctionDefinitionRequest {
    /// <p>The ID of the Lambda function definition.</p>
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFunctionDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGroupCertificateConfigurationRequest {
    /// <p>The amount of time remaining before the certificate expires, in milliseconds.</p>
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGroupCertificateConfigurationResponse {
    /// <p>The amount of time remaining before the certificate authority expires, in milliseconds.</p>
    #[serde(rename = "CertificateAuthorityExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_expiry_in_milliseconds: Option<String>,
    /// <p>The amount of time remaining before the certificate expires, in milliseconds.</p>
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    /// <p>The ID of the group certificate configuration.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGroupRequest {
    /// <p>The ID of the Greengrass group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateLoggerDefinitionRequest {
    /// <p>The ID of the logger definition.</p>
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLoggerDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateResourceDefinitionRequest {
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the resource definition.</p>
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateResourceDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSubscriptionDefinitionRequest {
    /// <p>The name of the definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the subscription definition.</p>
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSubscriptionDefinitionResponse {}

/// <p>Information about a version.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VersionInformation {
    /// <p>The ARN of the version.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the version was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// <p>The ID of the version.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique ID of the version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Errors returned by AssociateRoleToGroup
#[derive(Debug, PartialEq)]
pub enum AssociateRoleToGroupError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl AssociateRoleToGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateRoleToGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(AssociateRoleToGroupError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(AssociateRoleToGroupError::InternalServerError(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateRoleToGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateRoleToGroupError {
    fn description(&self) -> &str {
        match *self {
            AssociateRoleToGroupError::BadRequest(ref cause) => cause,
            AssociateRoleToGroupError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateServiceRoleToAccount
#[derive(Debug, PartialEq)]
pub enum AssociateServiceRoleToAccountError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl AssociateServiceRoleToAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateServiceRoleToAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(AssociateServiceRoleToAccountError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        AssociateServiceRoleToAccountError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateServiceRoleToAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateServiceRoleToAccountError {
    fn description(&self) -> &str {
        match *self {
            AssociateServiceRoleToAccountError::BadRequest(ref cause) => cause,
            AssociateServiceRoleToAccountError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConnectorDefinition
#[derive(Debug, PartialEq)]
pub enum CreateConnectorDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateConnectorDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConnectorDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateConnectorDefinitionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateConnectorDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConnectorDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateConnectorDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConnectorDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateConnectorDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateConnectorDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateConnectorDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateConnectorDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateConnectorDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConnectorDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateConnectorDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCoreDefinition
#[derive(Debug, PartialEq)]
pub enum CreateCoreDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateCoreDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCoreDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateCoreDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCoreDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCoreDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateCoreDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCoreDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateCoreDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateCoreDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateCoreDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateCoreDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCoreDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCoreDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateCoreDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDeploymentError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeploymentError {
    fn description(&self) -> &str {
        match *self {
            CreateDeploymentError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDeviceDefinition
#[derive(Debug, PartialEq)]
pub enum CreateDeviceDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateDeviceDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeviceDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDeviceDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDeviceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeviceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateDeviceDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDeviceDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateDeviceDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateDeviceDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDeviceDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDeviceDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDeviceDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeviceDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateDeviceDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFunctionDefinition
#[derive(Debug, PartialEq)]
pub enum CreateFunctionDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateFunctionDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFunctionDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateFunctionDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateFunctionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFunctionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateFunctionDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFunctionDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateFunctionDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateFunctionDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateFunctionDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateFunctionDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateFunctionDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFunctionDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateFunctionDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGroup
#[derive(Debug, PartialEq)]
pub enum CreateGroupError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateGroupError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateGroupError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGroupCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum CreateGroupCertificateAuthorityError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl CreateGroupCertificateAuthorityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateGroupCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateGroupCertificateAuthorityError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        CreateGroupCertificateAuthorityError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateGroupCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGroupCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            CreateGroupCertificateAuthorityError::BadRequest(ref cause) => cause,
            CreateGroupCertificateAuthorityError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGroupVersion
#[derive(Debug, PartialEq)]
pub enum CreateGroupVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateGroupVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGroupVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateGroupVersionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateGroupVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGroupVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateGroupVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLoggerDefinition
#[derive(Debug, PartialEq)]
pub enum CreateLoggerDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateLoggerDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLoggerDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateLoggerDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateLoggerDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLoggerDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateLoggerDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLoggerDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateLoggerDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateLoggerDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateLoggerDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateLoggerDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateLoggerDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLoggerDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateLoggerDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateResourceDefinition
#[derive(Debug, PartialEq)]
pub enum CreateResourceDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateResourceDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateResourceDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateResourceDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateResourceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateResourceDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateResourceDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateResourceDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateResourceDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateResourceDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateResourceDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSoftwareUpdateJob
#[derive(Debug, PartialEq)]
pub enum CreateSoftwareUpdateJobError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl CreateSoftwareUpdateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSoftwareUpdateJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateSoftwareUpdateJobError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateSoftwareUpdateJobError::InternalServerError(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateSoftwareUpdateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSoftwareUpdateJobError {
    fn description(&self) -> &str {
        match *self {
            CreateSoftwareUpdateJobError::BadRequest(ref cause) => cause,
            CreateSoftwareUpdateJobError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSubscriptionDefinition
#[derive(Debug, PartialEq)]
pub enum CreateSubscriptionDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateSubscriptionDefinitionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateSubscriptionDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateSubscriptionDefinitionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateSubscriptionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSubscriptionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            CreateSubscriptionDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSubscriptionDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum CreateSubscriptionDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl CreateSubscriptionDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateSubscriptionDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        CreateSubscriptionDefinitionVersionError::BadRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateSubscriptionDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSubscriptionDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateSubscriptionDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConnectorDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteConnectorDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl DeleteConnectorDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConnectorDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteConnectorDefinitionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteConnectorDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConnectorDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteConnectorDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCoreDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteCoreDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl DeleteCoreDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCoreDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteCoreDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteCoreDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCoreDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteCoreDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDeviceDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteDeviceDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl DeleteDeviceDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeviceDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDeviceDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDeviceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDeviceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteDeviceDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFunctionDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteFunctionDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl DeleteFunctionDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFunctionDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteFunctionDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteFunctionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFunctionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteFunctionDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGroupError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl DeleteGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteGroupError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteGroupError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLoggerDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteLoggerDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl DeleteLoggerDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLoggerDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteLoggerDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteLoggerDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLoggerDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteLoggerDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteResourceDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteResourceDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl DeleteResourceDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourceDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteResourceDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteResourceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourceDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSubscriptionDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteSubscriptionDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl DeleteSubscriptionDefinitionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteSubscriptionDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteSubscriptionDefinitionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteSubscriptionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSubscriptionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteSubscriptionDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateRoleFromGroup
#[derive(Debug, PartialEq)]
pub enum DisassociateRoleFromGroupError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl DisassociateRoleFromGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateRoleFromGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DisassociateRoleFromGroupError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DisassociateRoleFromGroupError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateRoleFromGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateRoleFromGroupError {
    fn description(&self) -> &str {
        match *self {
            DisassociateRoleFromGroupError::BadRequest(ref cause) => cause,
            DisassociateRoleFromGroupError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateServiceRoleFromAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateServiceRoleFromAccountError {
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl DisassociateServiceRoleFromAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateServiceRoleFromAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DisassociateServiceRoleFromAccountError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateServiceRoleFromAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateServiceRoleFromAccountError {
    fn description(&self) -> &str {
        match *self {
            DisassociateServiceRoleFromAccountError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAssociatedRole
#[derive(Debug, PartialEq)]
pub enum GetAssociatedRoleError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl GetAssociatedRoleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAssociatedRoleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAssociatedRoleError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetAssociatedRoleError::InternalServerError(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAssociatedRoleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAssociatedRoleError {
    fn description(&self) -> &str {
        match *self {
            GetAssociatedRoleError::BadRequest(ref cause) => cause,
            GetAssociatedRoleError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBulkDeploymentStatus
#[derive(Debug, PartialEq)]
pub enum GetBulkDeploymentStatusError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetBulkDeploymentStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBulkDeploymentStatusError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBulkDeploymentStatusError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetBulkDeploymentStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBulkDeploymentStatusError {
    fn description(&self) -> &str {
        match *self {
            GetBulkDeploymentStatusError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnectivityInfo
#[derive(Debug, PartialEq)]
pub enum GetConnectivityInfoError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl GetConnectivityInfoError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectivityInfoError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetConnectivityInfoError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetConnectivityInfoError::InternalServerError(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConnectivityInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectivityInfoError {
    fn description(&self) -> &str {
        match *self {
            GetConnectivityInfoError::BadRequest(ref cause) => cause,
            GetConnectivityInfoError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnectorDefinition
#[derive(Debug, PartialEq)]
pub enum GetConnectorDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetConnectorDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectorDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetConnectorDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConnectorDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectorDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetConnectorDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnectorDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetConnectorDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetConnectorDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetConnectorDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetConnectorDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConnectorDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectorDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetConnectorDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCoreDefinition
#[derive(Debug, PartialEq)]
pub enum GetCoreDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetCoreDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCoreDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetCoreDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCoreDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCoreDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetCoreDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCoreDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetCoreDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetCoreDefinitionVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCoreDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetCoreDefinitionVersionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCoreDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCoreDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetCoreDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeploymentStatus
#[derive(Debug, PartialEq)]
pub enum GetDeploymentStatusError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetDeploymentStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentStatusError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDeploymentStatusError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDeploymentStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeploymentStatusError {
    fn description(&self) -> &str {
        match *self {
            GetDeploymentStatusError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeviceDefinition
#[derive(Debug, PartialEq)]
pub enum GetDeviceDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetDeviceDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeviceDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDeviceDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDeviceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeviceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetDeviceDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeviceDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetDeviceDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetDeviceDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDeviceDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDeviceDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDeviceDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeviceDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetDeviceDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFunctionDefinition
#[derive(Debug, PartialEq)]
pub enum GetFunctionDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetFunctionDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFunctionDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetFunctionDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFunctionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFunctionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetFunctionDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFunctionDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetFunctionDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetFunctionDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetFunctionDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetFunctionDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFunctionDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFunctionDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetFunctionDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGroup
#[derive(Debug, PartialEq)]
pub enum GetGroupError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetGroupError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupError {
    fn description(&self) -> &str {
        match *self {
            GetGroupError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGroupCertificateAuthority
#[derive(Debug, PartialEq)]
pub enum GetGroupCertificateAuthorityError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl GetGroupCertificateAuthorityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetGroupCertificateAuthorityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetGroupCertificateAuthorityError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        GetGroupCertificateAuthorityError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGroupCertificateAuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupCertificateAuthorityError {
    fn description(&self) -> &str {
        match *self {
            GetGroupCertificateAuthorityError::BadRequest(ref cause) => cause,
            GetGroupCertificateAuthorityError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGroupCertificateConfiguration
#[derive(Debug, PartialEq)]
pub enum GetGroupCertificateConfigurationError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl GetGroupCertificateConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetGroupCertificateConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetGroupCertificateConfigurationError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        GetGroupCertificateConfigurationError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGroupCertificateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupCertificateConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetGroupCertificateConfigurationError::BadRequest(ref cause) => cause,
            GetGroupCertificateConfigurationError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGroupVersion
#[derive(Debug, PartialEq)]
pub enum GetGroupVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetGroupVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGroupVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetGroupVersionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGroupVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupVersionError {
    fn description(&self) -> &str {
        match *self {
            GetGroupVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLoggerDefinition
#[derive(Debug, PartialEq)]
pub enum GetLoggerDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetLoggerDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLoggerDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetLoggerDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetLoggerDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoggerDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetLoggerDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLoggerDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetLoggerDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetLoggerDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetLoggerDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetLoggerDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetLoggerDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoggerDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetLoggerDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResourceDefinition
#[derive(Debug, PartialEq)]
pub enum GetResourceDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetResourceDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourceDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetResourceDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetResourceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetResourceDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResourceDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetResourceDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetResourceDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetResourceDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetResourceDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetResourceDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetResourceDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetServiceRoleForAccount
#[derive(Debug, PartialEq)]
pub enum GetServiceRoleForAccountError {
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl GetServiceRoleForAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetServiceRoleForAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        GetServiceRoleForAccountError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetServiceRoleForAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetServiceRoleForAccountError {
    fn description(&self) -> &str {
        match *self {
            GetServiceRoleForAccountError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSubscriptionDefinition
#[derive(Debug, PartialEq)]
pub enum GetSubscriptionDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetSubscriptionDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSubscriptionDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSubscriptionDefinitionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSubscriptionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSubscriptionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetSubscriptionDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSubscriptionDefinitionVersion
#[derive(Debug, PartialEq)]
pub enum GetSubscriptionDefinitionVersionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl GetSubscriptionDefinitionVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetSubscriptionDefinitionVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSubscriptionDefinitionVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSubscriptionDefinitionVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSubscriptionDefinitionVersionError {
    fn description(&self) -> &str {
        match *self {
            GetSubscriptionDefinitionVersionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBulkDeploymentDetailedReports
#[derive(Debug, PartialEq)]
pub enum ListBulkDeploymentDetailedReportsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListBulkDeploymentDetailedReportsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListBulkDeploymentDetailedReportsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        ListBulkDeploymentDetailedReportsError::BadRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListBulkDeploymentDetailedReportsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBulkDeploymentDetailedReportsError {
    fn description(&self) -> &str {
        match *self {
            ListBulkDeploymentDetailedReportsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBulkDeployments
#[derive(Debug, PartialEq)]
pub enum ListBulkDeploymentsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListBulkDeploymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBulkDeploymentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListBulkDeploymentsError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListBulkDeploymentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBulkDeploymentsError {
    fn description(&self) -> &str {
        match *self {
            ListBulkDeploymentsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListConnectorDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListConnectorDefinitionVersionsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListConnectorDefinitionVersionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListConnectorDefinitionVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListConnectorDefinitionVersionsError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListConnectorDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListConnectorDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListConnectorDefinitionVersionsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListConnectorDefinitions
#[derive(Debug, PartialEq)]
pub enum ListConnectorDefinitionsError {}

impl ListConnectorDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConnectorDefinitionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListConnectorDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListConnectorDefinitionsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListCoreDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListCoreDefinitionVersionsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListCoreDefinitionVersionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListCoreDefinitionVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListCoreDefinitionVersionsError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListCoreDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCoreDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListCoreDefinitionVersionsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCoreDefinitions
#[derive(Debug, PartialEq)]
pub enum ListCoreDefinitionsError {}

impl ListCoreDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCoreDefinitionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListCoreDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCoreDefinitionsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListDeployments
#[derive(Debug, PartialEq)]
pub enum ListDeploymentsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListDeploymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeploymentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDeploymentsError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDeploymentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeploymentsError {
    fn description(&self) -> &str {
        match *self {
            ListDeploymentsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDeviceDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListDeviceDefinitionVersionsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListDeviceDefinitionVersionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDeviceDefinitionVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDeviceDefinitionVersionsError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDeviceDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeviceDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListDeviceDefinitionVersionsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDeviceDefinitions
#[derive(Debug, PartialEq)]
pub enum ListDeviceDefinitionsError {}

impl ListDeviceDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeviceDefinitionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDeviceDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeviceDefinitionsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListFunctionDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListFunctionDefinitionVersionsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListFunctionDefinitionVersionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListFunctionDefinitionVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListFunctionDefinitionVersionsError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListFunctionDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFunctionDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListFunctionDefinitionVersionsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFunctionDefinitions
#[derive(Debug, PartialEq)]
pub enum ListFunctionDefinitionsError {}

impl ListFunctionDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFunctionDefinitionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListFunctionDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFunctionDefinitionsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListGroupCertificateAuthorities
#[derive(Debug, PartialEq)]
pub enum ListGroupCertificateAuthoritiesError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl ListGroupCertificateAuthoritiesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListGroupCertificateAuthoritiesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListGroupCertificateAuthoritiesError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        ListGroupCertificateAuthoritiesError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListGroupCertificateAuthoritiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGroupCertificateAuthoritiesError {
    fn description(&self) -> &str {
        match *self {
            ListGroupCertificateAuthoritiesError::BadRequest(ref cause) => cause,
            ListGroupCertificateAuthoritiesError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGroupVersions
#[derive(Debug, PartialEq)]
pub enum ListGroupVersionsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListGroupVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroupVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListGroupVersionsError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListGroupVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGroupVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListGroupVersionsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGroups
#[derive(Debug, PartialEq)]
pub enum ListGroupsError {}

impl ListGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGroupsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListLoggerDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListLoggerDefinitionVersionsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListLoggerDefinitionVersionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListLoggerDefinitionVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListLoggerDefinitionVersionsError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLoggerDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLoggerDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListLoggerDefinitionVersionsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLoggerDefinitions
#[derive(Debug, PartialEq)]
pub enum ListLoggerDefinitionsError {}

impl ListLoggerDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLoggerDefinitionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLoggerDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLoggerDefinitionsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListResourceDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListResourceDefinitionVersionsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListResourceDefinitionVersionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListResourceDefinitionVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListResourceDefinitionVersionsError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListResourceDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListResourceDefinitionVersionsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourceDefinitions
#[derive(Debug, PartialEq)]
pub enum ListResourceDefinitionsError {}

impl ListResourceDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourceDefinitionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListResourceDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceDefinitionsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListSubscriptionDefinitionVersions
#[derive(Debug, PartialEq)]
pub enum ListSubscriptionDefinitionVersionsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListSubscriptionDefinitionVersionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListSubscriptionDefinitionVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        ListSubscriptionDefinitionVersionsError::BadRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListSubscriptionDefinitionVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSubscriptionDefinitionVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListSubscriptionDefinitionVersionsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSubscriptionDefinitions
#[derive(Debug, PartialEq)]
pub enum ListSubscriptionDefinitionsError {}

impl ListSubscriptionDefinitionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListSubscriptionDefinitionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListSubscriptionDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSubscriptionDefinitionsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ResetDeployments
#[derive(Debug, PartialEq)]
pub enum ResetDeploymentsError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl ResetDeploymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetDeploymentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ResetDeploymentsError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ResetDeploymentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetDeploymentsError {
    fn description(&self) -> &str {
        match *self {
            ResetDeploymentsError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by StartBulkDeployment
#[derive(Debug, PartialEq)]
pub enum StartBulkDeploymentError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl StartBulkDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartBulkDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartBulkDeploymentError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartBulkDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartBulkDeploymentError {
    fn description(&self) -> &str {
        match *self {
            StartBulkDeploymentError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by StopBulkDeployment
#[derive(Debug, PartialEq)]
pub enum StopBulkDeploymentError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl StopBulkDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopBulkDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StopBulkDeploymentError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopBulkDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopBulkDeploymentError {
    fn description(&self) -> &str {
        match *self {
            StopBulkDeploymentError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConnectivityInfo
#[derive(Debug, PartialEq)]
pub enum UpdateConnectivityInfoError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl UpdateConnectivityInfoError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConnectivityInfoError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateConnectivityInfoError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateConnectivityInfoError::InternalServerError(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateConnectivityInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConnectivityInfoError {
    fn description(&self) -> &str {
        match *self {
            UpdateConnectivityInfoError::BadRequest(ref cause) => cause,
            UpdateConnectivityInfoError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConnectorDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateConnectorDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl UpdateConnectorDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConnectorDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateConnectorDefinitionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateConnectorDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConnectorDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateConnectorDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCoreDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateCoreDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl UpdateCoreDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCoreDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateCoreDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateCoreDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCoreDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateCoreDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDeviceDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl UpdateDeviceDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDeviceDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDeviceDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDeviceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDeviceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateDeviceDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFunctionDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateFunctionDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl UpdateFunctionDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFunctionDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFunctionDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateFunctionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFunctionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateFunctionDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGroup
#[derive(Debug, PartialEq)]
pub enum UpdateGroupError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl UpdateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateGroupError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateGroupError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGroupCertificateConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateGroupCertificateConfigurationError {
    /// <p>General error information.</p>
    BadRequest(String),
    /// <p>General error information.</p>
    InternalServerError(String),
}

impl UpdateGroupCertificateConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateGroupCertificateConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        UpdateGroupCertificateConfigurationError::BadRequest(err.msg),
                    )
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateGroupCertificateConfigurationError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateGroupCertificateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGroupCertificateConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateGroupCertificateConfigurationError::BadRequest(ref cause) => cause,
            UpdateGroupCertificateConfigurationError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateLoggerDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateLoggerDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl UpdateLoggerDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateLoggerDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateLoggerDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateLoggerDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLoggerDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateLoggerDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateResourceDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateResourceDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl UpdateResourceDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateResourceDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateResourceDefinitionError::BadRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateResourceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateResourceDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateResourceDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSubscriptionDefinition
#[derive(Debug, PartialEq)]
pub enum UpdateSubscriptionDefinitionError {
    /// <p>General error information.</p>
    BadRequest(String),
}

impl UpdateSubscriptionDefinitionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateSubscriptionDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateSubscriptionDefinitionError::BadRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateSubscriptionDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSubscriptionDefinitionError {
    fn description(&self) -> &str {
        match *self {
            UpdateSubscriptionDefinitionError::BadRequest(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Greengrass API. AWS Greengrass clients implement this trait.
pub trait GreenGrass {
    /// <p>Associates a role with a group. Your Greengrass core will use the role to access AWS cloud services. The role&#39;s permissions should allow Greengrass core Lambda functions to perform actions against the cloud.</p>
    fn associate_role_to_group(
        &self,
        input: AssociateRoleToGroupRequest,
    ) -> RusotoFuture<AssociateRoleToGroupResponse, AssociateRoleToGroupError>;

    /// <p>Associates a role with your account. AWS IoT Greengrass will use the role to access your Lambda functions and AWS IoT resources. This is necessary for deployments to succeed. The role must have at least minimum permissions in the policy &#39;&#39;AWSGreengrassResourceAccessRolePolicy&#39;&#39;.</p>
    fn associate_service_role_to_account(
        &self,
        input: AssociateServiceRoleToAccountRequest,
    ) -> RusotoFuture<AssociateServiceRoleToAccountResponse, AssociateServiceRoleToAccountError>;

    /// <p>Creates a connector definition. You may provide the initial version of the connector definition now or use &#39;&#39;CreateConnectorDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_connector_definition(
        &self,
        input: CreateConnectorDefinitionRequest,
    ) -> RusotoFuture<CreateConnectorDefinitionResponse, CreateConnectorDefinitionError>;

    /// <p>Creates a version of a connector definition which has already been defined.</p>
    fn create_connector_definition_version(
        &self,
        input: CreateConnectorDefinitionVersionRequest,
    ) -> RusotoFuture<CreateConnectorDefinitionVersionResponse, CreateConnectorDefinitionVersionError>;

    /// <p>Creates a core definition. You may provide the initial version of the core definition now or use &#39;&#39;CreateCoreDefinitionVersion&#39;&#39; at a later time. Greengrass groups must each contain exactly one Greengrass core.</p>
    fn create_core_definition(
        &self,
        input: CreateCoreDefinitionRequest,
    ) -> RusotoFuture<CreateCoreDefinitionResponse, CreateCoreDefinitionError>;

    /// <p>Creates a version of a core definition that has already been defined. Greengrass groups must each contain exactly one Greengrass core.</p>
    fn create_core_definition_version(
        &self,
        input: CreateCoreDefinitionVersionRequest,
    ) -> RusotoFuture<CreateCoreDefinitionVersionResponse, CreateCoreDefinitionVersionError>;

    /// <p>Creates a deployment. &#39;&#39;CreateDeployment&#39;&#39; requests are idempotent with respect to the &#39;&#39;X-Amzn-Client-Token&#39;&#39; token and the request parameters.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<CreateDeploymentResponse, CreateDeploymentError>;

    /// <p>Creates a device definition. You may provide the initial version of the device definition now or use &#39;&#39;CreateDeviceDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_device_definition(
        &self,
        input: CreateDeviceDefinitionRequest,
    ) -> RusotoFuture<CreateDeviceDefinitionResponse, CreateDeviceDefinitionError>;

    /// <p>Creates a version of a device definition that has already been defined.</p>
    fn create_device_definition_version(
        &self,
        input: CreateDeviceDefinitionVersionRequest,
    ) -> RusotoFuture<CreateDeviceDefinitionVersionResponse, CreateDeviceDefinitionVersionError>;

    /// <p>Creates a Lambda function definition which contains a list of Lambda functions and their configurations to be used in a group. You can create an initial version of the definition by providing a list of Lambda functions and their configurations now, or use &#39;&#39;CreateFunctionDefinitionVersion&#39;&#39; later.</p>
    fn create_function_definition(
        &self,
        input: CreateFunctionDefinitionRequest,
    ) -> RusotoFuture<CreateFunctionDefinitionResponse, CreateFunctionDefinitionError>;

    /// <p>Creates a version of a Lambda function definition that has already been defined.</p>
    fn create_function_definition_version(
        &self,
        input: CreateFunctionDefinitionVersionRequest,
    ) -> RusotoFuture<CreateFunctionDefinitionVersionResponse, CreateFunctionDefinitionVersionError>;

    /// <p>Creates a group. You may provide the initial version of the group or use &#39;&#39;CreateGroupVersion&#39;&#39; at a later time. Tip: You can use the &#39;&#39;gg<em>group</em>setup&#39;&#39; package (https://github.com/awslabs/aws-greengrass-group-setup) as a library or command-line application to create and deploy Greengrass groups.</p>
    fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> RusotoFuture<CreateGroupResponse, CreateGroupError>;

    /// <p>Creates a CA for the group. If a CA already exists, it will rotate the existing CA.</p>
    fn create_group_certificate_authority(
        &self,
        input: CreateGroupCertificateAuthorityRequest,
    ) -> RusotoFuture<CreateGroupCertificateAuthorityResponse, CreateGroupCertificateAuthorityError>;

    /// <p>Creates a version of a group which has already been defined.</p>
    fn create_group_version(
        &self,
        input: CreateGroupVersionRequest,
    ) -> RusotoFuture<CreateGroupVersionResponse, CreateGroupVersionError>;

    /// <p>Creates a logger definition. You may provide the initial version of the logger definition now or use &#39;&#39;CreateLoggerDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_logger_definition(
        &self,
        input: CreateLoggerDefinitionRequest,
    ) -> RusotoFuture<CreateLoggerDefinitionResponse, CreateLoggerDefinitionError>;

    /// <p>Creates a version of a logger definition that has already been defined.</p>
    fn create_logger_definition_version(
        &self,
        input: CreateLoggerDefinitionVersionRequest,
    ) -> RusotoFuture<CreateLoggerDefinitionVersionResponse, CreateLoggerDefinitionVersionError>;

    /// <p>Creates a resource definition which contains a list of resources to be used in a group. You can create an initial version of the definition by providing a list of resources now, or use &#39;&#39;CreateResourceDefinitionVersion&#39;&#39; later.</p>
    fn create_resource_definition(
        &self,
        input: CreateResourceDefinitionRequest,
    ) -> RusotoFuture<CreateResourceDefinitionResponse, CreateResourceDefinitionError>;

    /// <p>Creates a version of a resource definition that has already been defined.</p>
    fn create_resource_definition_version(
        &self,
        input: CreateResourceDefinitionVersionRequest,
    ) -> RusotoFuture<CreateResourceDefinitionVersionResponse, CreateResourceDefinitionVersionError>;

    /// <p>Creates a software update for a core or group of cores (specified as an IoT thing group.) Use this to update the OTA Agent as well as the Greengrass core software. It makes use of the IoT Jobs feature which provides additional commands to manage a Greengrass core software update job.</p>
    fn create_software_update_job(
        &self,
        input: CreateSoftwareUpdateJobRequest,
    ) -> RusotoFuture<CreateSoftwareUpdateJobResponse, CreateSoftwareUpdateJobError>;

    /// <p>Creates a subscription definition. You may provide the initial version of the subscription definition now or use &#39;&#39;CreateSubscriptionDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_subscription_definition(
        &self,
        input: CreateSubscriptionDefinitionRequest,
    ) -> RusotoFuture<CreateSubscriptionDefinitionResponse, CreateSubscriptionDefinitionError>;

    /// <p>Creates a version of a subscription definition which has already been defined.</p>
    fn create_subscription_definition_version(
        &self,
        input: CreateSubscriptionDefinitionVersionRequest,
    ) -> RusotoFuture<
        CreateSubscriptionDefinitionVersionResponse,
        CreateSubscriptionDefinitionVersionError,
    >;

    /// <p>Deletes a connector definition.</p>
    fn delete_connector_definition(
        &self,
        input: DeleteConnectorDefinitionRequest,
    ) -> RusotoFuture<DeleteConnectorDefinitionResponse, DeleteConnectorDefinitionError>;

    /// <p>Deletes a core definition.</p>
    fn delete_core_definition(
        &self,
        input: DeleteCoreDefinitionRequest,
    ) -> RusotoFuture<DeleteCoreDefinitionResponse, DeleteCoreDefinitionError>;

    /// <p>Deletes a device definition.</p>
    fn delete_device_definition(
        &self,
        input: DeleteDeviceDefinitionRequest,
    ) -> RusotoFuture<DeleteDeviceDefinitionResponse, DeleteDeviceDefinitionError>;

    /// <p>Deletes a Lambda function definition.</p>
    fn delete_function_definition(
        &self,
        input: DeleteFunctionDefinitionRequest,
    ) -> RusotoFuture<DeleteFunctionDefinitionResponse, DeleteFunctionDefinitionError>;

    /// <p>Deletes a group.</p>
    fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> RusotoFuture<DeleteGroupResponse, DeleteGroupError>;

    /// <p>Deletes a logger definition.</p>
    fn delete_logger_definition(
        &self,
        input: DeleteLoggerDefinitionRequest,
    ) -> RusotoFuture<DeleteLoggerDefinitionResponse, DeleteLoggerDefinitionError>;

    /// <p>Deletes a resource definition.</p>
    fn delete_resource_definition(
        &self,
        input: DeleteResourceDefinitionRequest,
    ) -> RusotoFuture<DeleteResourceDefinitionResponse, DeleteResourceDefinitionError>;

    /// <p>Deletes a subscription definition.</p>
    fn delete_subscription_definition(
        &self,
        input: DeleteSubscriptionDefinitionRequest,
    ) -> RusotoFuture<DeleteSubscriptionDefinitionResponse, DeleteSubscriptionDefinitionError>;

    /// <p>Disassociates the role from a group.</p>
    fn disassociate_role_from_group(
        &self,
        input: DisassociateRoleFromGroupRequest,
    ) -> RusotoFuture<DisassociateRoleFromGroupResponse, DisassociateRoleFromGroupError>;

    /// <p>Disassociates the service role from your account. Without a service role, deployments will not work.</p>
    fn disassociate_service_role_from_account(
        &self,
    ) -> RusotoFuture<
        DisassociateServiceRoleFromAccountResponse,
        DisassociateServiceRoleFromAccountError,
    >;

    /// <p>Retrieves the role associated with a particular group.</p>
    fn get_associated_role(
        &self,
        input: GetAssociatedRoleRequest,
    ) -> RusotoFuture<GetAssociatedRoleResponse, GetAssociatedRoleError>;

    /// <p>Returns the status of a bulk deployment.</p>
    fn get_bulk_deployment_status(
        &self,
        input: GetBulkDeploymentStatusRequest,
    ) -> RusotoFuture<GetBulkDeploymentStatusResponse, GetBulkDeploymentStatusError>;

    /// <p>Retrieves the connectivity information for a core.</p>
    fn get_connectivity_info(
        &self,
        input: GetConnectivityInfoRequest,
    ) -> RusotoFuture<GetConnectivityInfoResponse, GetConnectivityInfoError>;

    /// <p>Retrieves information about a connector definition.</p>
    fn get_connector_definition(
        &self,
        input: GetConnectorDefinitionRequest,
    ) -> RusotoFuture<GetConnectorDefinitionResponse, GetConnectorDefinitionError>;

    /// <p>Retrieves information about a connector definition version, including the connectors that the version contains. Connectors are prebuilt modules that interact with local infrastructure, device protocols, AWS, and other cloud services.</p>
    fn get_connector_definition_version(
        &self,
        input: GetConnectorDefinitionVersionRequest,
    ) -> RusotoFuture<GetConnectorDefinitionVersionResponse, GetConnectorDefinitionVersionError>;

    /// <p>Retrieves information about a core definition version.</p>
    fn get_core_definition(
        &self,
        input: GetCoreDefinitionRequest,
    ) -> RusotoFuture<GetCoreDefinitionResponse, GetCoreDefinitionError>;

    /// <p>Retrieves information about a core definition version.</p>
    fn get_core_definition_version(
        &self,
        input: GetCoreDefinitionVersionRequest,
    ) -> RusotoFuture<GetCoreDefinitionVersionResponse, GetCoreDefinitionVersionError>;

    /// <p>Returns the status of a deployment.</p>
    fn get_deployment_status(
        &self,
        input: GetDeploymentStatusRequest,
    ) -> RusotoFuture<GetDeploymentStatusResponse, GetDeploymentStatusError>;

    /// <p>Retrieves information about a device definition.</p>
    fn get_device_definition(
        &self,
        input: GetDeviceDefinitionRequest,
    ) -> RusotoFuture<GetDeviceDefinitionResponse, GetDeviceDefinitionError>;

    /// <p>Retrieves information about a device definition version.</p>
    fn get_device_definition_version(
        &self,
        input: GetDeviceDefinitionVersionRequest,
    ) -> RusotoFuture<GetDeviceDefinitionVersionResponse, GetDeviceDefinitionVersionError>;

    /// <p>Retrieves information about a Lambda function definition, including its creation time and latest version.</p>
    fn get_function_definition(
        &self,
        input: GetFunctionDefinitionRequest,
    ) -> RusotoFuture<GetFunctionDefinitionResponse, GetFunctionDefinitionError>;

    /// <p>Retrieves information about a Lambda function definition version, including which Lambda functions are included in the version and their configurations.</p>
    fn get_function_definition_version(
        &self,
        input: GetFunctionDefinitionVersionRequest,
    ) -> RusotoFuture<GetFunctionDefinitionVersionResponse, GetFunctionDefinitionVersionError>;

    /// <p>Retrieves information about a group.</p>
    fn get_group(&self, input: GetGroupRequest) -> RusotoFuture<GetGroupResponse, GetGroupError>;

    /// <p>Retreives the CA associated with a group. Returns the public key of the CA.</p>
    fn get_group_certificate_authority(
        &self,
        input: GetGroupCertificateAuthorityRequest,
    ) -> RusotoFuture<GetGroupCertificateAuthorityResponse, GetGroupCertificateAuthorityError>;

    /// <p>Retrieves the current configuration for the CA used by the group.</p>
    fn get_group_certificate_configuration(
        &self,
        input: GetGroupCertificateConfigurationRequest,
    ) -> RusotoFuture<GetGroupCertificateConfigurationResponse, GetGroupCertificateConfigurationError>;

    /// <p>Retrieves information about a group version.</p>
    fn get_group_version(
        &self,
        input: GetGroupVersionRequest,
    ) -> RusotoFuture<GetGroupVersionResponse, GetGroupVersionError>;

    /// <p>Retrieves information about a logger definition.</p>
    fn get_logger_definition(
        &self,
        input: GetLoggerDefinitionRequest,
    ) -> RusotoFuture<GetLoggerDefinitionResponse, GetLoggerDefinitionError>;

    /// <p>Retrieves information about a logger definition version.</p>
    fn get_logger_definition_version(
        &self,
        input: GetLoggerDefinitionVersionRequest,
    ) -> RusotoFuture<GetLoggerDefinitionVersionResponse, GetLoggerDefinitionVersionError>;

    /// <p>Retrieves information about a resource definition, including its creation time and latest version.</p>
    fn get_resource_definition(
        &self,
        input: GetResourceDefinitionRequest,
    ) -> RusotoFuture<GetResourceDefinitionResponse, GetResourceDefinitionError>;

    /// <p>Retrieves information about a resource definition version, including which resources are included in the version.</p>
    fn get_resource_definition_version(
        &self,
        input: GetResourceDefinitionVersionRequest,
    ) -> RusotoFuture<GetResourceDefinitionVersionResponse, GetResourceDefinitionVersionError>;

    /// <p>Retrieves the service role that is attached to your account.</p>
    fn get_service_role_for_account(
        &self,
    ) -> RusotoFuture<GetServiceRoleForAccountResponse, GetServiceRoleForAccountError>;

    /// <p>Retrieves information about a subscription definition.</p>
    fn get_subscription_definition(
        &self,
        input: GetSubscriptionDefinitionRequest,
    ) -> RusotoFuture<GetSubscriptionDefinitionResponse, GetSubscriptionDefinitionError>;

    /// <p>Retrieves information about a subscription definition version.</p>
    fn get_subscription_definition_version(
        &self,
        input: GetSubscriptionDefinitionVersionRequest,
    ) -> RusotoFuture<GetSubscriptionDefinitionVersionResponse, GetSubscriptionDefinitionVersionError>;

    /// <p>Gets a paginated list of the deployments that have been started in a bulk deployment operation, and their current deployment status.</p>
    fn list_bulk_deployment_detailed_reports(
        &self,
        input: ListBulkDeploymentDetailedReportsRequest,
    ) -> RusotoFuture<
        ListBulkDeploymentDetailedReportsResponse,
        ListBulkDeploymentDetailedReportsError,
    >;

    /// <p>Returns a list of bulk deployments.</p>
    fn list_bulk_deployments(
        &self,
        input: ListBulkDeploymentsRequest,
    ) -> RusotoFuture<ListBulkDeploymentsResponse, ListBulkDeploymentsError>;

    /// <p>Lists the versions of a connector definition, which are containers for connectors. Connectors run on the Greengrass core and contain built-in integration with local infrastructure, device protocols, AWS, and other cloud services.</p>
    fn list_connector_definition_versions(
        &self,
        input: ListConnectorDefinitionVersionsRequest,
    ) -> RusotoFuture<ListConnectorDefinitionVersionsResponse, ListConnectorDefinitionVersionsError>;

    /// <p>Retrieves a list of connector definitions.</p>
    fn list_connector_definitions(
        &self,
        input: ListConnectorDefinitionsRequest,
    ) -> RusotoFuture<ListConnectorDefinitionsResponse, ListConnectorDefinitionsError>;

    /// <p>Lists the versions of a core definition.</p>
    fn list_core_definition_versions(
        &self,
        input: ListCoreDefinitionVersionsRequest,
    ) -> RusotoFuture<ListCoreDefinitionVersionsResponse, ListCoreDefinitionVersionsError>;

    /// <p>Retrieves a list of core definitions.</p>
    fn list_core_definitions(
        &self,
        input: ListCoreDefinitionsRequest,
    ) -> RusotoFuture<ListCoreDefinitionsResponse, ListCoreDefinitionsError>;

    /// <p>Returns a history of deployments for the group.</p>
    fn list_deployments(
        &self,
        input: ListDeploymentsRequest,
    ) -> RusotoFuture<ListDeploymentsResponse, ListDeploymentsError>;

    /// <p>Lists the versions of a device definition.</p>
    fn list_device_definition_versions(
        &self,
        input: ListDeviceDefinitionVersionsRequest,
    ) -> RusotoFuture<ListDeviceDefinitionVersionsResponse, ListDeviceDefinitionVersionsError>;

    /// <p>Retrieves a list of device definitions.</p>
    fn list_device_definitions(
        &self,
        input: ListDeviceDefinitionsRequest,
    ) -> RusotoFuture<ListDeviceDefinitionsResponse, ListDeviceDefinitionsError>;

    /// <p>Lists the versions of a Lambda function definition.</p>
    fn list_function_definition_versions(
        &self,
        input: ListFunctionDefinitionVersionsRequest,
    ) -> RusotoFuture<ListFunctionDefinitionVersionsResponse, ListFunctionDefinitionVersionsError>;

    /// <p>Retrieves a list of Lambda function definitions.</p>
    fn list_function_definitions(
        &self,
        input: ListFunctionDefinitionsRequest,
    ) -> RusotoFuture<ListFunctionDefinitionsResponse, ListFunctionDefinitionsError>;

    /// <p>Retrieves the current CAs for a group.</p>
    fn list_group_certificate_authorities(
        &self,
        input: ListGroupCertificateAuthoritiesRequest,
    ) -> RusotoFuture<ListGroupCertificateAuthoritiesResponse, ListGroupCertificateAuthoritiesError>;

    /// <p>Lists the versions of a group.</p>
    fn list_group_versions(
        &self,
        input: ListGroupVersionsRequest,
    ) -> RusotoFuture<ListGroupVersionsResponse, ListGroupVersionsError>;

    /// <p>Retrieves a list of groups.</p>
    fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> RusotoFuture<ListGroupsResponse, ListGroupsError>;

    /// <p>Lists the versions of a logger definition.</p>
    fn list_logger_definition_versions(
        &self,
        input: ListLoggerDefinitionVersionsRequest,
    ) -> RusotoFuture<ListLoggerDefinitionVersionsResponse, ListLoggerDefinitionVersionsError>;

    /// <p>Retrieves a list of logger definitions.</p>
    fn list_logger_definitions(
        &self,
        input: ListLoggerDefinitionsRequest,
    ) -> RusotoFuture<ListLoggerDefinitionsResponse, ListLoggerDefinitionsError>;

    /// <p>Lists the versions of a resource definition.</p>
    fn list_resource_definition_versions(
        &self,
        input: ListResourceDefinitionVersionsRequest,
    ) -> RusotoFuture<ListResourceDefinitionVersionsResponse, ListResourceDefinitionVersionsError>;

    /// <p>Retrieves a list of resource definitions.</p>
    fn list_resource_definitions(
        &self,
        input: ListResourceDefinitionsRequest,
    ) -> RusotoFuture<ListResourceDefinitionsResponse, ListResourceDefinitionsError>;

    /// <p>Lists the versions of a subscription definition.</p>
    fn list_subscription_definition_versions(
        &self,
        input: ListSubscriptionDefinitionVersionsRequest,
    ) -> RusotoFuture<
        ListSubscriptionDefinitionVersionsResponse,
        ListSubscriptionDefinitionVersionsError,
    >;

    /// <p>Retrieves a list of subscription definitions.</p>
    fn list_subscription_definitions(
        &self,
        input: ListSubscriptionDefinitionsRequest,
    ) -> RusotoFuture<ListSubscriptionDefinitionsResponse, ListSubscriptionDefinitionsError>;

    /// <p>Retrieves a list of resource tags for a resource arn.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Resets a group&#39;s deployments.</p>
    fn reset_deployments(
        &self,
        input: ResetDeploymentsRequest,
    ) -> RusotoFuture<ResetDeploymentsResponse, ResetDeploymentsError>;

    /// <p>Deploys multiple groups in one operation. This action starts the bulk deployment of a specified set of group versions. Each group version deployment will be triggered with an adaptive rate that has a fixed upper limit. We recommend that you include an &#39;&#39;X-Amzn-Client-Token&#39;&#39; token in every &#39;&#39;StartBulkDeployment&#39;&#39; request. These requests are idempotent with respect to the token and the request parameters.</p>
    fn start_bulk_deployment(
        &self,
        input: StartBulkDeploymentRequest,
    ) -> RusotoFuture<StartBulkDeploymentResponse, StartBulkDeploymentError>;

    /// <p>Stops the execution of a bulk deployment. This action returns a status of &#39;&#39;Stopping&#39;&#39; until the deployment is stopped. You cannot start a new bulk deployment while a previous deployment is in the &#39;&#39;Stopping&#39;&#39; state. This action doesn&#39;t rollback completed deployments or cancel pending deployments.</p>
    fn stop_bulk_deployment(
        &self,
        input: StopBulkDeploymentRequest,
    ) -> RusotoFuture<StopBulkDeploymentResponse, StopBulkDeploymentError>;

    /// <p>Add resource tags to a Greengrass Resource. Valid resources are Group, Connector, Core, Device, Function, Logger, Subscription, and Resource Defintions, and also BulkDeploymentIds.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p>Remove resource tags from a Greengrass Resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

    /// <p>Updates the connectivity information for the core. Any devices that belong to the group which has this core will receive this information in order to find the location of the core and connect to it.</p>
    fn update_connectivity_info(
        &self,
        input: UpdateConnectivityInfoRequest,
    ) -> RusotoFuture<UpdateConnectivityInfoResponse, UpdateConnectivityInfoError>;

    /// <p>Updates a connector definition.</p>
    fn update_connector_definition(
        &self,
        input: UpdateConnectorDefinitionRequest,
    ) -> RusotoFuture<UpdateConnectorDefinitionResponse, UpdateConnectorDefinitionError>;

    /// <p>Updates a core definition.</p>
    fn update_core_definition(
        &self,
        input: UpdateCoreDefinitionRequest,
    ) -> RusotoFuture<UpdateCoreDefinitionResponse, UpdateCoreDefinitionError>;

    /// <p>Updates a device definition.</p>
    fn update_device_definition(
        &self,
        input: UpdateDeviceDefinitionRequest,
    ) -> RusotoFuture<UpdateDeviceDefinitionResponse, UpdateDeviceDefinitionError>;

    /// <p>Updates a Lambda function definition.</p>
    fn update_function_definition(
        &self,
        input: UpdateFunctionDefinitionRequest,
    ) -> RusotoFuture<UpdateFunctionDefinitionResponse, UpdateFunctionDefinitionError>;

    /// <p>Updates a group.</p>
    fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> RusotoFuture<UpdateGroupResponse, UpdateGroupError>;

    /// <p>Updates the Certificate expiry time for a group.</p>
    fn update_group_certificate_configuration(
        &self,
        input: UpdateGroupCertificateConfigurationRequest,
    ) -> RusotoFuture<
        UpdateGroupCertificateConfigurationResponse,
        UpdateGroupCertificateConfigurationError,
    >;

    /// <p>Updates a logger definition.</p>
    fn update_logger_definition(
        &self,
        input: UpdateLoggerDefinitionRequest,
    ) -> RusotoFuture<UpdateLoggerDefinitionResponse, UpdateLoggerDefinitionError>;

    /// <p>Updates a resource definition.</p>
    fn update_resource_definition(
        &self,
        input: UpdateResourceDefinitionRequest,
    ) -> RusotoFuture<UpdateResourceDefinitionResponse, UpdateResourceDefinitionError>;

    /// <p>Updates a subscription definition.</p>
    fn update_subscription_definition(
        &self,
        input: UpdateSubscriptionDefinitionRequest,
    ) -> RusotoFuture<UpdateSubscriptionDefinitionResponse, UpdateSubscriptionDefinitionError>;
}
/// A client for the AWS Greengrass API.
#[derive(Clone)]
pub struct GreenGrassClient {
    client: Client,
    region: region::Region,
}

impl GreenGrassClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> GreenGrassClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> GreenGrassClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> GreenGrassClient {
        GreenGrassClient { client, region }
    }
}

impl fmt::Debug for GreenGrassClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenGrassClient")
            .field("region", &self.region)
            .finish()
    }
}

impl GreenGrass for GreenGrassClient {
    /// <p>Associates a role with a group. Your Greengrass core will use the role to access AWS cloud services. The role&#39;s permissions should allow Greengrass core Lambda functions to perform actions against the cloud.</p>
    fn associate_role_to_group(
        &self,
        input: AssociateRoleToGroupRequest,
    ) -> RusotoFuture<AssociateRoleToGroupResponse, AssociateRoleToGroupError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/role",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateRoleToGroupResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AssociateRoleToGroupError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Associates a role with your account. AWS IoT Greengrass will use the role to access your Lambda functions and AWS IoT resources. This is necessary for deployments to succeed. The role must have at least minimum permissions in the policy &#39;&#39;AWSGreengrassResourceAccessRolePolicy&#39;&#39;.</p>
    fn associate_service_role_to_account(
        &self,
        input: AssociateServiceRoleToAccountRequest,
    ) -> RusotoFuture<AssociateServiceRoleToAccountResponse, AssociateServiceRoleToAccountError>
    {
        let request_uri = "/greengrass/servicerole";

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateServiceRoleToAccountResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateServiceRoleToAccountError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a connector definition. You may provide the initial version of the connector definition now or use &#39;&#39;CreateConnectorDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_connector_definition(
        &self,
        input: CreateConnectorDefinitionRequest,
    ) -> RusotoFuture<CreateConnectorDefinitionResponse, CreateConnectorDefinitionError> {
        let request_uri = "/greengrass/definition/connectors";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateConnectorDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateConnectorDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a version of a connector definition which has already been defined.</p>
    fn create_connector_definition_version(
        &self,
        input: CreateConnectorDefinitionVersionRequest,
    ) -> RusotoFuture<CreateConnectorDefinitionVersionResponse, CreateConnectorDefinitionVersionError>
    {
        let request_uri = format!(
            "/greengrass/definition/connectors/{connector_definition_id}/versions",
            connector_definition_id = input.connector_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateConnectorDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateConnectorDefinitionVersionError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Creates a core definition. You may provide the initial version of the core definition now or use &#39;&#39;CreateCoreDefinitionVersion&#39;&#39; at a later time. Greengrass groups must each contain exactly one Greengrass core.</p>
    fn create_core_definition(
        &self,
        input: CreateCoreDefinitionRequest,
    ) -> RusotoFuture<CreateCoreDefinitionResponse, CreateCoreDefinitionError> {
        let request_uri = "/greengrass/definition/cores";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCoreDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateCoreDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a version of a core definition that has already been defined. Greengrass groups must each contain exactly one Greengrass core.</p>
    fn create_core_definition_version(
        &self,
        input: CreateCoreDefinitionVersionRequest,
    ) -> RusotoFuture<CreateCoreDefinitionVersionResponse, CreateCoreDefinitionVersionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}/versions",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCoreDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateCoreDefinitionVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a deployment. &#39;&#39;CreateDeployment&#39;&#39; requests are idempotent with respect to the &#39;&#39;X-Amzn-Client-Token&#39;&#39; token and the request parameters.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<CreateDeploymentResponse, CreateDeploymentError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/deployments",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDeploymentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a device definition. You may provide the initial version of the device definition now or use &#39;&#39;CreateDeviceDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_device_definition(
        &self,
        input: CreateDeviceDefinitionRequest,
    ) -> RusotoFuture<CreateDeviceDefinitionResponse, CreateDeviceDefinitionError> {
        let request_uri = "/greengrass/definition/devices";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDeviceDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateDeviceDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a version of a device definition that has already been defined.</p>
    fn create_device_definition_version(
        &self,
        input: CreateDeviceDefinitionVersionRequest,
    ) -> RusotoFuture<CreateDeviceDefinitionVersionResponse, CreateDeviceDefinitionVersionError>
    {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}/versions",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDeviceDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDeviceDefinitionVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a Lambda function definition which contains a list of Lambda functions and their configurations to be used in a group. You can create an initial version of the definition by providing a list of Lambda functions and their configurations now, or use &#39;&#39;CreateFunctionDefinitionVersion&#39;&#39; later.</p>
    fn create_function_definition(
        &self,
        input: CreateFunctionDefinitionRequest,
    ) -> RusotoFuture<CreateFunctionDefinitionResponse, CreateFunctionDefinitionError> {
        let request_uri = "/greengrass/definition/functions";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateFunctionDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateFunctionDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a version of a Lambda function definition that has already been defined.</p>
    fn create_function_definition_version(
        &self,
        input: CreateFunctionDefinitionVersionRequest,
    ) -> RusotoFuture<CreateFunctionDefinitionVersionResponse, CreateFunctionDefinitionVersionError>
    {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}/versions",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateFunctionDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateFunctionDefinitionVersionError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Creates a group. You may provide the initial version of the group or use &#39;&#39;CreateGroupVersion&#39;&#39; at a later time. Tip: You can use the &#39;&#39;gg<em>group</em>setup&#39;&#39; package (https://github.com/awslabs/aws-greengrass-group-setup) as a library or command-line application to create and deploy Greengrass groups.</p>
    fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> RusotoFuture<CreateGroupResponse, CreateGroupError> {
        let request_uri = "/greengrass/groups";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateGroupResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a CA for the group. If a CA already exists, it will rotate the existing CA.</p>
    fn create_group_certificate_authority(
        &self,
        input: CreateGroupCertificateAuthorityRequest,
    ) -> RusotoFuture<CreateGroupCertificateAuthorityResponse, CreateGroupCertificateAuthorityError>
    {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateGroupCertificateAuthorityResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateGroupCertificateAuthorityError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Creates a version of a group which has already been defined.</p>
    fn create_group_version(
        &self,
        input: CreateGroupVersionRequest,
    ) -> RusotoFuture<CreateGroupVersionResponse, CreateGroupVersionError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/versions",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateGroupVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateGroupVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a logger definition. You may provide the initial version of the logger definition now or use &#39;&#39;CreateLoggerDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_logger_definition(
        &self,
        input: CreateLoggerDefinitionRequest,
    ) -> RusotoFuture<CreateLoggerDefinitionResponse, CreateLoggerDefinitionError> {
        let request_uri = "/greengrass/definition/loggers";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateLoggerDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateLoggerDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a version of a logger definition that has already been defined.</p>
    fn create_logger_definition_version(
        &self,
        input: CreateLoggerDefinitionVersionRequest,
    ) -> RusotoFuture<CreateLoggerDefinitionVersionResponse, CreateLoggerDefinitionVersionError>
    {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}/versions",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateLoggerDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateLoggerDefinitionVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a resource definition which contains a list of resources to be used in a group. You can create an initial version of the definition by providing a list of resources now, or use &#39;&#39;CreateResourceDefinitionVersion&#39;&#39; later.</p>
    fn create_resource_definition(
        &self,
        input: CreateResourceDefinitionRequest,
    ) -> RusotoFuture<CreateResourceDefinitionResponse, CreateResourceDefinitionError> {
        let request_uri = "/greengrass/definition/resources";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateResourceDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateResourceDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a version of a resource definition that has already been defined.</p>
    fn create_resource_definition_version(
        &self,
        input: CreateResourceDefinitionVersionRequest,
    ) -> RusotoFuture<CreateResourceDefinitionVersionResponse, CreateResourceDefinitionVersionError>
    {
        let request_uri = format!(
            "/greengrass/definition/resources/{resource_definition_id}/versions",
            resource_definition_id = input.resource_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateResourceDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateResourceDefinitionVersionError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Creates a software update for a core or group of cores (specified as an IoT thing group.) Use this to update the OTA Agent as well as the Greengrass core software. It makes use of the IoT Jobs feature which provides additional commands to manage a Greengrass core software update job.</p>
    fn create_software_update_job(
        &self,
        input: CreateSoftwareUpdateJobRequest,
    ) -> RusotoFuture<CreateSoftwareUpdateJobResponse, CreateSoftwareUpdateJobError> {
        let request_uri = "/greengrass/updates";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateSoftwareUpdateJobResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateSoftwareUpdateJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a subscription definition. You may provide the initial version of the subscription definition now or use &#39;&#39;CreateSubscriptionDefinitionVersion&#39;&#39; at a later time.</p>
    fn create_subscription_definition(
        &self,
        input: CreateSubscriptionDefinitionRequest,
    ) -> RusotoFuture<CreateSubscriptionDefinitionResponse, CreateSubscriptionDefinitionError> {
        let request_uri = "/greengrass/definition/subscriptions";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateSubscriptionDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateSubscriptionDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a version of a subscription definition which has already been defined.</p>
    fn create_subscription_definition_version(
        &self,
        input: CreateSubscriptionDefinitionVersionRequest,
    ) -> RusotoFuture<
        CreateSubscriptionDefinitionVersionResponse,
        CreateSubscriptionDefinitionVersionError,
    > {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}/versions",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateSubscriptionDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateSubscriptionDefinitionVersionError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a connector definition.</p>
    fn delete_connector_definition(
        &self,
        input: DeleteConnectorDefinitionRequest,
    ) -> RusotoFuture<DeleteConnectorDefinitionResponse, DeleteConnectorDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/connectors/{connector_definition_id}",
            connector_definition_id = input.connector_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteConnectorDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConnectorDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a core definition.</p>
    fn delete_core_definition(
        &self,
        input: DeleteCoreDefinitionRequest,
    ) -> RusotoFuture<DeleteCoreDefinitionResponse, DeleteCoreDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteCoreDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteCoreDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a device definition.</p>
    fn delete_device_definition(
        &self,
        input: DeleteDeviceDefinitionRequest,
    ) -> RusotoFuture<DeleteDeviceDefinitionResponse, DeleteDeviceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDeviceDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteDeviceDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a Lambda function definition.</p>
    fn delete_function_definition(
        &self,
        input: DeleteFunctionDefinitionRequest,
    ) -> RusotoFuture<DeleteFunctionDefinitionResponse, DeleteFunctionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteFunctionDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteFunctionDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a group.</p>
    fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> RusotoFuture<DeleteGroupResponse, DeleteGroupError> {
        let request_uri = format!("/greengrass/groups/{group_id}", group_id = input.group_id);

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteGroupResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a logger definition.</p>
    fn delete_logger_definition(
        &self,
        input: DeleteLoggerDefinitionRequest,
    ) -> RusotoFuture<DeleteLoggerDefinitionResponse, DeleteLoggerDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteLoggerDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteLoggerDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a resource definition.</p>
    fn delete_resource_definition(
        &self,
        input: DeleteResourceDefinitionRequest,
    ) -> RusotoFuture<DeleteResourceDefinitionResponse, DeleteResourceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/resources/{resource_definition_id}",
            resource_definition_id = input.resource_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteResourceDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteResourceDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a subscription definition.</p>
    fn delete_subscription_definition(
        &self,
        input: DeleteSubscriptionDefinitionRequest,
    ) -> RusotoFuture<DeleteSubscriptionDefinitionResponse, DeleteSubscriptionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteSubscriptionDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSubscriptionDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Disassociates the role from a group.</p>
    fn disassociate_role_from_group(
        &self,
        input: DisassociateRoleFromGroupRequest,
    ) -> RusotoFuture<DisassociateRoleFromGroupResponse, DisassociateRoleFromGroupError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/role",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateRoleFromGroupResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateRoleFromGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Disassociates the service role from your account. Without a service role, deployments will not work.</p>
    fn disassociate_service_role_from_account(
        &self,
    ) -> RusotoFuture<
        DisassociateServiceRoleFromAccountResponse,
        DisassociateServiceRoleFromAccountError,
    > {
        let request_uri = "/greengrass/servicerole";

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateServiceRoleFromAccountResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateServiceRoleFromAccountError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the role associated with a particular group.</p>
    fn get_associated_role(
        &self,
        input: GetAssociatedRoleRequest,
    ) -> RusotoFuture<GetAssociatedRoleResponse, GetAssociatedRoleError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/role",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAssociatedRoleResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAssociatedRoleError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the status of a bulk deployment.</p>
    fn get_bulk_deployment_status(
        &self,
        input: GetBulkDeploymentStatusRequest,
    ) -> RusotoFuture<GetBulkDeploymentStatusResponse, GetBulkDeploymentStatusError> {
        let request_uri = format!(
            "/greengrass/bulk/deployments/{bulk_deployment_id}/status",
            bulk_deployment_id = input.bulk_deployment_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetBulkDeploymentStatusResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBulkDeploymentStatusError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the connectivity information for a core.</p>
    fn get_connectivity_info(
        &self,
        input: GetConnectivityInfoRequest,
    ) -> RusotoFuture<GetConnectivityInfoResponse, GetConnectivityInfoError> {
        let request_uri = format!(
            "/greengrass/things/{thing_name}/connectivityInfo",
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConnectivityInfoResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetConnectivityInfoError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves information about a connector definition.</p>
    fn get_connector_definition(
        &self,
        input: GetConnectorDefinitionRequest,
    ) -> RusotoFuture<GetConnectorDefinitionResponse, GetConnectorDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/connectors/{connector_definition_id}",
            connector_definition_id = input.connector_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConnectorDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetConnectorDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves information about a connector definition version, including the connectors that the version contains. Connectors are prebuilt modules that interact with local infrastructure, device protocols, AWS, and other cloud services.</p>
    fn get_connector_definition_version(
        &self,
        input: GetConnectorDefinitionVersionRequest,
    ) -> RusotoFuture<GetConnectorDefinitionVersionResponse, GetConnectorDefinitionVersionError>
    {
        let request_uri = format!("/greengrass/definition/connectors/{connector_definition_id}/versions/{connector_definition_version_id}", connector_definition_id = input.connector_definition_id, connector_definition_version_id = input.connector_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConnectorDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetConnectorDefinitionVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves information about a core definition version.</p>
    fn get_core_definition(
        &self,
        input: GetCoreDefinitionRequest,
    ) -> RusotoFuture<GetCoreDefinitionResponse, GetCoreDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCoreDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCoreDefinitionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves information about a core definition version.</p>
    fn get_core_definition_version(
        &self,
        input: GetCoreDefinitionVersionRequest,
    ) -> RusotoFuture<GetCoreDefinitionVersionResponse, GetCoreDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/cores/{core_definition_id}/versions/{core_definition_version_id}", core_definition_id = input.core_definition_id, core_definition_version_id = input.core_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCoreDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCoreDefinitionVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the status of a deployment.</p>
    fn get_deployment_status(
        &self,
        input: GetDeploymentStatusRequest,
    ) -> RusotoFuture<GetDeploymentStatusResponse, GetDeploymentStatusError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/deployments/{deployment_id}/status",
            deployment_id = input.deployment_id,
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDeploymentStatusResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetDeploymentStatusError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves information about a device definition.</p>
    fn get_device_definition(
        &self,
        input: GetDeviceDefinitionRequest,
    ) -> RusotoFuture<GetDeviceDefinitionResponse, GetDeviceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDeviceDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetDeviceDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves information about a device definition version.</p>
    fn get_device_definition_version(
        &self,
        input: GetDeviceDefinitionVersionRequest,
    ) -> RusotoFuture<GetDeviceDefinitionVersionResponse, GetDeviceDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/devices/{device_definition_id}/versions/{device_definition_version_id}", device_definition_id = input.device_definition_id, device_definition_version_id = input.device_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDeviceDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDeviceDefinitionVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves information about a Lambda function definition, including its creation time and latest version.</p>
    fn get_function_definition(
        &self,
        input: GetFunctionDefinitionRequest,
    ) -> RusotoFuture<GetFunctionDefinitionResponse, GetFunctionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetFunctionDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetFunctionDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves information about a Lambda function definition version, including which Lambda functions are included in the version and their configurations.</p>
    fn get_function_definition_version(
        &self,
        input: GetFunctionDefinitionVersionRequest,
    ) -> RusotoFuture<GetFunctionDefinitionVersionResponse, GetFunctionDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/functions/{function_definition_id}/versions/{function_definition_version_id}", function_definition_id = input.function_definition_id, function_definition_version_id = input.function_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetFunctionDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetFunctionDefinitionVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves information about a group.</p>
    fn get_group(&self, input: GetGroupRequest) -> RusotoFuture<GetGroupResponse, GetGroupError> {
        let request_uri = format!("/greengrass/groups/{group_id}", group_id = input.group_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetGroupResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retreives the CA associated with a group. Returns the public key of the CA.</p>
    fn get_group_certificate_authority(
        &self,
        input: GetGroupCertificateAuthorityRequest,
    ) -> RusotoFuture<GetGroupCertificateAuthorityResponse, GetGroupCertificateAuthorityError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities/{certificate_authority_id}",
            certificate_authority_id = input.certificate_authority_id,
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetGroupCertificateAuthorityResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetGroupCertificateAuthorityError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the current configuration for the CA used by the group.</p>
    fn get_group_certificate_configuration(
        &self,
        input: GetGroupCertificateConfigurationRequest,
    ) -> RusotoFuture<GetGroupCertificateConfigurationResponse, GetGroupCertificateConfigurationError>
    {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities/configuration/expiry",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetGroupCertificateConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetGroupCertificateConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a group version.</p>
    fn get_group_version(
        &self,
        input: GetGroupVersionRequest,
    ) -> RusotoFuture<GetGroupVersionResponse, GetGroupVersionError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/versions/{group_version_id}",
            group_id = input.group_id,
            group_version_id = input.group_version_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetGroupVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGroupVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves information about a logger definition.</p>
    fn get_logger_definition(
        &self,
        input: GetLoggerDefinitionRequest,
    ) -> RusotoFuture<GetLoggerDefinitionResponse, GetLoggerDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetLoggerDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetLoggerDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves information about a logger definition version.</p>
    fn get_logger_definition_version(
        &self,
        input: GetLoggerDefinitionVersionRequest,
    ) -> RusotoFuture<GetLoggerDefinitionVersionResponse, GetLoggerDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/loggers/{logger_definition_id}/versions/{logger_definition_version_id}", logger_definition_id = input.logger_definition_id, logger_definition_version_id = input.logger_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetLoggerDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetLoggerDefinitionVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves information about a resource definition, including its creation time and latest version.</p>
    fn get_resource_definition(
        &self,
        input: GetResourceDefinitionRequest,
    ) -> RusotoFuture<GetResourceDefinitionResponse, GetResourceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/resources/{resource_definition_id}",
            resource_definition_id = input.resource_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetResourceDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetResourceDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves information about a resource definition version, including which resources are included in the version.</p>
    fn get_resource_definition_version(
        &self,
        input: GetResourceDefinitionVersionRequest,
    ) -> RusotoFuture<GetResourceDefinitionVersionResponse, GetResourceDefinitionVersionError> {
        let request_uri = format!("/greengrass/definition/resources/{resource_definition_id}/versions/{resource_definition_version_id}", resource_definition_id = input.resource_definition_id, resource_definition_version_id = input.resource_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetResourceDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetResourceDefinitionVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the service role that is attached to your account.</p>
    fn get_service_role_for_account(
        &self,
    ) -> RusotoFuture<GetServiceRoleForAccountResponse, GetServiceRoleForAccountError> {
        let request_uri = "/greengrass/servicerole";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetServiceRoleForAccountResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetServiceRoleForAccountError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves information about a subscription definition.</p>
    fn get_subscription_definition(
        &self,
        input: GetSubscriptionDefinitionRequest,
    ) -> RusotoFuture<GetSubscriptionDefinitionResponse, GetSubscriptionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSubscriptionDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSubscriptionDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves information about a subscription definition version.</p>
    fn get_subscription_definition_version(
        &self,
        input: GetSubscriptionDefinitionVersionRequest,
    ) -> RusotoFuture<GetSubscriptionDefinitionVersionResponse, GetSubscriptionDefinitionVersionError>
    {
        let request_uri = format!("/greengrass/definition/subscriptions/{subscription_definition_id}/versions/{subscription_definition_version_id}", subscription_definition_id = input.subscription_definition_id, subscription_definition_version_id = input.subscription_definition_version_id);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSubscriptionDefinitionVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSubscriptionDefinitionVersionError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Gets a paginated list of the deployments that have been started in a bulk deployment operation, and their current deployment status.</p>
    fn list_bulk_deployment_detailed_reports(
        &self,
        input: ListBulkDeploymentDetailedReportsRequest,
    ) -> RusotoFuture<
        ListBulkDeploymentDetailedReportsResponse,
        ListBulkDeploymentDetailedReportsError,
    > {
        let request_uri = format!(
            "/greengrass/bulk/deployments/{bulk_deployment_id}/detailed-reports",
            bulk_deployment_id = input.bulk_deployment_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListBulkDeploymentDetailedReportsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListBulkDeploymentDetailedReportsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns a list of bulk deployments.</p>
    fn list_bulk_deployments(
        &self,
        input: ListBulkDeploymentsRequest,
    ) -> RusotoFuture<ListBulkDeploymentsResponse, ListBulkDeploymentsError> {
        let request_uri = "/greengrass/bulk/deployments";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListBulkDeploymentsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListBulkDeploymentsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the versions of a connector definition, which are containers for connectors. Connectors run on the Greengrass core and contain built-in integration with local infrastructure, device protocols, AWS, and other cloud services.</p>
    fn list_connector_definition_versions(
        &self,
        input: ListConnectorDefinitionVersionsRequest,
    ) -> RusotoFuture<ListConnectorDefinitionVersionsResponse, ListConnectorDefinitionVersionsError>
    {
        let request_uri = format!(
            "/greengrass/definition/connectors/{connector_definition_id}/versions",
            connector_definition_id = input.connector_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListConnectorDefinitionVersionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListConnectorDefinitionVersionsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a list of connector definitions.</p>
    fn list_connector_definitions(
        &self,
        input: ListConnectorDefinitionsRequest,
    ) -> RusotoFuture<ListConnectorDefinitionsResponse, ListConnectorDefinitionsError> {
        let request_uri = "/greengrass/definition/connectors";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListConnectorDefinitionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListConnectorDefinitionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the versions of a core definition.</p>
    fn list_core_definition_versions(
        &self,
        input: ListCoreDefinitionVersionsRequest,
    ) -> RusotoFuture<ListCoreDefinitionVersionsResponse, ListCoreDefinitionVersionsError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}/versions",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListCoreDefinitionVersionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListCoreDefinitionVersionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list of core definitions.</p>
    fn list_core_definitions(
        &self,
        input: ListCoreDefinitionsRequest,
    ) -> RusotoFuture<ListCoreDefinitionsResponse, ListCoreDefinitionsError> {
        let request_uri = "/greengrass/definition/cores";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListCoreDefinitionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListCoreDefinitionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a history of deployments for the group.</p>
    fn list_deployments(
        &self,
        input: ListDeploymentsRequest,
    ) -> RusotoFuture<ListDeploymentsResponse, ListDeploymentsError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/deployments",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDeploymentsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDeploymentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the versions of a device definition.</p>
    fn list_device_definition_versions(
        &self,
        input: ListDeviceDefinitionVersionsRequest,
    ) -> RusotoFuture<ListDeviceDefinitionVersionsResponse, ListDeviceDefinitionVersionsError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}/versions",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDeviceDefinitionVersionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDeviceDefinitionVersionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list of device definitions.</p>
    fn list_device_definitions(
        &self,
        input: ListDeviceDefinitionsRequest,
    ) -> RusotoFuture<ListDeviceDefinitionsResponse, ListDeviceDefinitionsError> {
        let request_uri = "/greengrass/definition/devices";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDeviceDefinitionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListDeviceDefinitionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the versions of a Lambda function definition.</p>
    fn list_function_definition_versions(
        &self,
        input: ListFunctionDefinitionVersionsRequest,
    ) -> RusotoFuture<ListFunctionDefinitionVersionsResponse, ListFunctionDefinitionVersionsError>
    {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}/versions",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListFunctionDefinitionVersionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListFunctionDefinitionVersionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list of Lambda function definitions.</p>
    fn list_function_definitions(
        &self,
        input: ListFunctionDefinitionsRequest,
    ) -> RusotoFuture<ListFunctionDefinitionsResponse, ListFunctionDefinitionsError> {
        let request_uri = "/greengrass/definition/functions";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListFunctionDefinitionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListFunctionDefinitionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the current CAs for a group.</p>
    fn list_group_certificate_authorities(
        &self,
        input: ListGroupCertificateAuthoritiesRequest,
    ) -> RusotoFuture<ListGroupCertificateAuthoritiesResponse, ListGroupCertificateAuthoritiesError>
    {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListGroupCertificateAuthoritiesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListGroupCertificateAuthoritiesError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists the versions of a group.</p>
    fn list_group_versions(
        &self,
        input: ListGroupVersionsRequest,
    ) -> RusotoFuture<ListGroupVersionsResponse, ListGroupVersionsError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/versions",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListGroupVersionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGroupVersionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of groups.</p>
    fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> RusotoFuture<ListGroupsResponse, ListGroupsError> {
        let request_uri = "/greengrass/groups";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListGroupsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGroupsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the versions of a logger definition.</p>
    fn list_logger_definition_versions(
        &self,
        input: ListLoggerDefinitionVersionsRequest,
    ) -> RusotoFuture<ListLoggerDefinitionVersionsResponse, ListLoggerDefinitionVersionsError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}/versions",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListLoggerDefinitionVersionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListLoggerDefinitionVersionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list of logger definitions.</p>
    fn list_logger_definitions(
        &self,
        input: ListLoggerDefinitionsRequest,
    ) -> RusotoFuture<ListLoggerDefinitionsResponse, ListLoggerDefinitionsError> {
        let request_uri = "/greengrass/definition/loggers";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListLoggerDefinitionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListLoggerDefinitionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the versions of a resource definition.</p>
    fn list_resource_definition_versions(
        &self,
        input: ListResourceDefinitionVersionsRequest,
    ) -> RusotoFuture<ListResourceDefinitionVersionsResponse, ListResourceDefinitionVersionsError>
    {
        let request_uri = format!(
            "/greengrass/definition/resources/{resource_definition_id}/versions",
            resource_definition_id = input.resource_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListResourceDefinitionVersionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListResourceDefinitionVersionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list of resource definitions.</p>
    fn list_resource_definitions(
        &self,
        input: ListResourceDefinitionsRequest,
    ) -> RusotoFuture<ListResourceDefinitionsResponse, ListResourceDefinitionsError> {
        let request_uri = "/greengrass/definition/resources";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListResourceDefinitionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListResourceDefinitionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the versions of a subscription definition.</p>
    fn list_subscription_definition_versions(
        &self,
        input: ListSubscriptionDefinitionVersionsRequest,
    ) -> RusotoFuture<
        ListSubscriptionDefinitionVersionsResponse,
        ListSubscriptionDefinitionVersionsError,
    > {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}/versions",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListSubscriptionDefinitionVersionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSubscriptionDefinitionVersionsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves a list of subscription definitions.</p>
    fn list_subscription_definitions(
        &self,
        input: ListSubscriptionDefinitionsRequest,
    ) -> RusotoFuture<ListSubscriptionDefinitionsResponse, ListSubscriptionDefinitionsError> {
        let request_uri = "/greengrass/definition/subscriptions";

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListSubscriptionDefinitionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSubscriptionDefinitionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list of resource tags for a resource arn.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Resets a group&#39;s deployments.</p>
    fn reset_deployments(
        &self,
        input: ResetDeploymentsRequest,
    ) -> RusotoFuture<ResetDeploymentsResponse, ResetDeploymentsError> {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/deployments/$reset",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ResetDeploymentsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ResetDeploymentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deploys multiple groups in one operation. This action starts the bulk deployment of a specified set of group versions. Each group version deployment will be triggered with an adaptive rate that has a fixed upper limit. We recommend that you include an &#39;&#39;X-Amzn-Client-Token&#39;&#39; token in every &#39;&#39;StartBulkDeployment&#39;&#39; request. These requests are idempotent with respect to the token and the request parameters.</p>
    fn start_bulk_deployment(
        &self,
        input: StartBulkDeploymentRequest,
    ) -> RusotoFuture<StartBulkDeploymentResponse, StartBulkDeploymentError> {
        let request_uri = "/greengrass/bulk/deployments";

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref amzn_client_token) = input.amzn_client_token {
            request.add_header("X-Amzn-Client-Token", &amzn_client_token.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartBulkDeploymentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartBulkDeploymentError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Stops the execution of a bulk deployment. This action returns a status of &#39;&#39;Stopping&#39;&#39; until the deployment is stopped. You cannot start a new bulk deployment while a previous deployment is in the &#39;&#39;Stopping&#39;&#39; state. This action doesn&#39;t rollback completed deployments or cancel pending deployments.</p>
    fn stop_bulk_deployment(
        &self,
        input: StopBulkDeploymentRequest,
    ) -> RusotoFuture<StopBulkDeploymentResponse, StopBulkDeploymentError> {
        let request_uri = format!(
            "/greengrass/bulk/deployments/{bulk_deployment_id}/$stop",
            bulk_deployment_id = input.bulk_deployment_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopBulkDeploymentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopBulkDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Add resource tags to a Greengrass Resource. Valid resources are Group, Connector, Core, Device, Function, Logger, Subscription, and Resource Defintions, and also BulkDeploymentIds.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Remove resource tags from a Greengrass Resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the connectivity information for the core. Any devices that belong to the group which has this core will receive this information in order to find the location of the core and connect to it.</p>
    fn update_connectivity_info(
        &self,
        input: UpdateConnectivityInfoRequest,
    ) -> RusotoFuture<UpdateConnectivityInfoResponse, UpdateConnectivityInfoError> {
        let request_uri = format!(
            "/greengrass/things/{thing_name}/connectivityInfo",
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateConnectivityInfoResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateConnectivityInfoError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates a connector definition.</p>
    fn update_connector_definition(
        &self,
        input: UpdateConnectorDefinitionRequest,
    ) -> RusotoFuture<UpdateConnectorDefinitionResponse, UpdateConnectorDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/connectors/{connector_definition_id}",
            connector_definition_id = input.connector_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateConnectorDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConnectorDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates a core definition.</p>
    fn update_core_definition(
        &self,
        input: UpdateCoreDefinitionRequest,
    ) -> RusotoFuture<UpdateCoreDefinitionResponse, UpdateCoreDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/cores/{core_definition_id}",
            core_definition_id = input.core_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateCoreDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateCoreDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates a device definition.</p>
    fn update_device_definition(
        &self,
        input: UpdateDeviceDefinitionRequest,
    ) -> RusotoFuture<UpdateDeviceDefinitionResponse, UpdateDeviceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/devices/{device_definition_id}",
            device_definition_id = input.device_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDeviceDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateDeviceDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates a Lambda function definition.</p>
    fn update_function_definition(
        &self,
        input: UpdateFunctionDefinitionRequest,
    ) -> RusotoFuture<UpdateFunctionDefinitionResponse, UpdateFunctionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/functions/{function_definition_id}",
            function_definition_id = input.function_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateFunctionDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateFunctionDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates a group.</p>
    fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> RusotoFuture<UpdateGroupResponse, UpdateGroupError> {
        let request_uri = format!("/greengrass/groups/{group_id}", group_id = input.group_id);

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateGroupResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the Certificate expiry time for a group.</p>
    fn update_group_certificate_configuration(
        &self,
        input: UpdateGroupCertificateConfigurationRequest,
    ) -> RusotoFuture<
        UpdateGroupCertificateConfigurationResponse,
        UpdateGroupCertificateConfigurationError,
    > {
        let request_uri = format!(
            "/greengrass/groups/{group_id}/certificateauthorities/configuration/expiry",
            group_id = input.group_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateGroupCertificateConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateGroupCertificateConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Updates a logger definition.</p>
    fn update_logger_definition(
        &self,
        input: UpdateLoggerDefinitionRequest,
    ) -> RusotoFuture<UpdateLoggerDefinitionResponse, UpdateLoggerDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/loggers/{logger_definition_id}",
            logger_definition_id = input.logger_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateLoggerDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateLoggerDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates a resource definition.</p>
    fn update_resource_definition(
        &self,
        input: UpdateResourceDefinitionRequest,
    ) -> RusotoFuture<UpdateResourceDefinitionResponse, UpdateResourceDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/resources/{resource_definition_id}",
            resource_definition_id = input.resource_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateResourceDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateResourceDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates a subscription definition.</p>
    fn update_subscription_definition(
        &self,
        input: UpdateSubscriptionDefinitionRequest,
    ) -> RusotoFuture<UpdateSubscriptionDefinitionResponse, UpdateSubscriptionDefinitionError> {
        let request_uri = format!(
            "/greengrass/definition/subscriptions/{subscription_definition_id}",
            subscription_definition_id = input.subscription_definition_id
        );

        let mut request = SignedRequest::new("PUT", "greengrass", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateSubscriptionDefinitionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateSubscriptionDefinitionError::from_response(response))
                }))
            }
        })
    }
}
