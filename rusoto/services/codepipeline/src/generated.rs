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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Represents an AWS session credentials object. These credentials are temporary credentials that are issued by AWS Secure Token Service (STS). They can be used to access input and output artifacts in the Amazon S3 bucket used to store artifact for the pipeline in AWS CodePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AWSSessionCredentials {
    /// <p>The access key for the session.</p>
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,
    /// <p>The secret access key for the session.</p>
    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,
    /// <p>The token for the session.</p>
    #[serde(rename = "sessionToken")]
    pub session_token: String,
}

/// <p>Represents the input of an AcknowledgeJob action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AcknowledgeJobInput {
    /// <p>The unique system-generated ID of the job for which you want to confirm receipt.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Get this number from the response of the <a>PollForJobs</a> request that returned this job.</p>
    #[serde(rename = "nonce")]
    pub nonce: String,
}

/// <p>Represents the output of an AcknowledgeJob action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcknowledgeJobOutput {
    /// <p>Whether the job worker has received the specified job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents the input of an AcknowledgeThirdPartyJob action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AcknowledgeThirdPartyJobInput {
    /// <p>The clientToken portion of the clientId and clientToken pair used to verify that the calling entity is allowed access to the job and its details.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The unique system-generated ID of the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Get this number from the response to a <a>GetThirdPartyJobDetails</a> request.</p>
    #[serde(rename = "nonce")]
    pub nonce: String,
}

/// <p>Represents the output of an AcknowledgeThirdPartyJob action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcknowledgeThirdPartyJobOutput {
    /// <p>The status information for the third party job, if any.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents information about an action configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActionConfiguration {
    /// <p>The configuration data for the action.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents information about an action configuration property.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionConfigurationProperty {
    /// <p>The description of the action configuration property that will be displayed to users.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether the configuration property is a key.</p>
    #[serde(rename = "key")]
    pub key: bool,
    /// <p>The name of the action configuration property.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Indicates that the property will be used in conjunction with <code>PollForJobs</code>. When creating a custom action, an action can have up to one queryable property. If it has one, that property must be both required and not secret.</p> <p>If you create a pipeline with a custom action type, and that custom action contains a queryable property, the value for that configuration property is subject to additional restrictions. The value must be less than or equal to twenty (20) characters. The value can contain only alphanumeric characters, underscores, and hyphens.</p>
    #[serde(rename = "queryable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queryable: Option<bool>,
    /// <p>Whether the configuration property is a required value.</p>
    #[serde(rename = "required")]
    pub required: bool,
    /// <p>Whether the configuration property is secret. Secrets are hidden from all calls except for <code>GetJobDetails</code>, <code>GetThirdPartyJobDetails</code>, <code>PollForJobs</code>, and <code>PollForThirdPartyJobs</code>.</p> <p>When updating a pipeline, passing * * * * * without changing any other values of the action will preserve the prior value of the secret.</p>
    #[serde(rename = "secret")]
    pub secret: bool,
    /// <p>The type of the configuration property.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents the context of an action within the stage of a pipeline to a job worker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActionContext {
    /// <p>The system-generated unique ID that corresponds to an action's execution.</p>
    #[serde(rename = "actionExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_execution_id: Option<String>,
    /// <p>The name of the action within the context of a job.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents information about an action declaration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionDeclaration {
    /// <p>Specifies the action type and the provider of the action.</p>
    #[serde(rename = "actionTypeId")]
    pub action_type_id: ActionTypeId,
    /// <p>The action's configuration. These are key-value pairs that specify input values for an action. For more information, see <a href="https://docs.aws.amazon.com/codepipeline/latest/userguide/reference-pipeline-structure.html#action-requirements">Action Structure Requirements in CodePipeline</a>. For the list of configuration properties for the AWS CloudFormation action type in CodePipeline, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/continuous-delivery-codepipeline-action-reference.html">Configuration Properties Reference</a> in the <i>AWS CloudFormation User Guide</i>. For template snippets with examples, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/continuous-delivery-codepipeline-parameter-override-functions.html">Using Parameter Override Functions with CodePipeline Pipelines</a> in the <i>AWS CloudFormation User Guide</i>.</p> <p>The values can be represented in either JSON or YAML format. For example, the JSON configuration item format is as follows: </p> <p> <i>JSON:</i> </p> <p> <code>"Configuration" : { Key : Value },</code> </p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name or ID of the artifact consumed by the action, such as a test or build artifact.</p>
    #[serde(rename = "inputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<InputArtifact>>,
    /// <p>The action declaration's name.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The name or ID of the result of the action declaration, such as a test or build artifact.</p>
    #[serde(rename = "outputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<OutputArtifact>>,
    /// <p>The action declaration's AWS Region, such as us-east-1.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The ARN of the IAM service role that will perform the declared action. This is assumed through the roleArn for the pipeline.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The order in which actions are run.</p>
    #[serde(rename = "runOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_order: Option<i64>,
}

/// <p>Represents information about the run of an action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActionExecution {
    /// <p>The details of an error returned by a URL external to AWS.</p>
    #[serde(rename = "errorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    /// <p>The external ID of the run of the action.</p>
    #[serde(rename = "externalExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    /// <p>The URL of a resource external to AWS that will be used when running the action, for example an external repository URL.</p>
    #[serde(rename = "externalExecutionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_url: Option<String>,
    /// <p>The last status change of the action.</p>
    #[serde(rename = "lastStatusChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change: Option<f64>,
    /// <p>The ARN of the user who last changed the pipeline.</p>
    #[serde(rename = "lastUpdatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    /// <p>A percentage of completeness of the action as it runs.</p>
    #[serde(rename = "percentComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i64>,
    /// <p>The status of the action, or for a completed action, the last status of the action.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A summary of the run of the action.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// <p>The system-generated token used to identify a unique approval request. The token for each open approval request can be obtained using the <code>GetPipelineState</code> command and is used to validate that the approval request corresponding to this token is still valid.</p>
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// <p>Returns information about an execution of an action, including the action execution ID, and the name, version, and timing of the action. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActionExecutionDetail {
    /// <p>The action execution ID.</p>
    #[serde(rename = "actionExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_execution_id: Option<String>,
    /// <p>The name of the action.</p>
    #[serde(rename = "actionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    /// <p>Input details for the action execution, such as role ARN, Region, and input artifacts.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<ActionExecutionInput>,
    /// <p>The last update time of the action execution.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>Output details for the action execution, such as the action execution result.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<ActionExecutionOutput>,
    /// <p>The pipeline execution ID for the action execution.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    /// <p>The version of the pipeline where the action was run.</p>
    #[serde(rename = "pipelineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_version: Option<i64>,
    /// <p>The name of the stage that contains the action.</p>
    #[serde(rename = "stageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>The start time of the action execution.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p> The status of the action execution. Status categories are <code>InProgress</code>, <code>Succeeded</code>, and <code>Failed</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Filter values for the action execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ActionExecutionFilter {
    /// <p>The pipeline execution ID used to filter action execution history.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

/// <p>Input information used for an action execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActionExecutionInput {
    #[serde(rename = "actionTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type_id: Option<ActionTypeId>,
    /// <p>Configuration data for an action execution.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<::std::collections::HashMap<String, String>>,
    /// <p>Details of input artifacts of the action that correspond to the action execution.</p>
    #[serde(rename = "inputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<ArtifactDetail>>,
    /// <p>The AWS Region for the action, such as us-east-1.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The ARN of the IAM service role that performs the declared action. This is assumed through the roleArn for the pipeline. </p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Output details listed for an action execution, such as the action execution result.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActionExecutionOutput {
    /// <p>Execution result information listed in the output details for an action execution.</p>
    #[serde(rename = "executionResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_result: Option<ActionExecutionResult>,
    /// <p>Details of output artifacts of the action that correspond to the action execution.</p>
    #[serde(rename = "outputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<ArtifactDetail>>,
}

/// <p>Execution result information, such as the external execution ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActionExecutionResult {
    /// <p>The action provider's external ID for the action execution.</p>
    #[serde(rename = "externalExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    /// <p>The action provider's summary for the action execution.</p>
    #[serde(rename = "externalExecutionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_summary: Option<String>,
    /// <p>The deepest external link to the external resource (for example, a repository URL or deployment endpoint) that is used when running the action.</p>
    #[serde(rename = "externalExecutionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_url: Option<String>,
}

/// <p>Represents information about the version (or revision) of an action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionRevision {
    /// <p>The date and time when the most recent version of the action was created, in timestamp format.</p>
    #[serde(rename = "created")]
    pub created: Option<f64>,
    /// <p>The unique identifier of the change that set the state to this revision, for example a deployment ID or timestamp.</p>
    #[serde(rename = "revisionChangeId")]
    pub revision_change_id: Option<String>,
    /// <p>The system-generated unique ID that identifies the revision number of the action.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

/// <p>Represents information about the state of an action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActionState {
    /// <p>The name of the action.</p>
    #[serde(rename = "actionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    /// <p>Represents information about the version (or revision) of an action.</p>
    #[serde(rename = "currentRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<ActionRevision>,
    /// <p>A URL link for more information about the state of the action, such as a deployment group details page.</p>
    #[serde(rename = "entityUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_url: Option<String>,
    /// <p>Represents information about the run of an action.</p>
    #[serde(rename = "latestExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_execution: Option<ActionExecution>,
    /// <p>A URL link for more information about the revision, such as a commit details page.</p>
    #[serde(rename = "revisionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url: Option<String>,
}

/// <p>Returns information about the details of an action type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActionType {
    /// <p>The configuration properties for the action type.</p>
    #[serde(rename = "actionConfigurationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_configuration_properties: Option<Vec<ActionConfigurationProperty>>,
    /// <p>Represents information about an action type.</p>
    #[serde(rename = "id")]
    pub id: ActionTypeId,
    /// <p>The details of the input artifact for the action, such as its commit ID.</p>
    #[serde(rename = "inputArtifactDetails")]
    pub input_artifact_details: ArtifactDetails,
    /// <p>The details of the output artifact of the action, such as its commit ID.</p>
    #[serde(rename = "outputArtifactDetails")]
    pub output_artifact_details: ArtifactDetails,
    /// <p>The settings for the action type.</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ActionTypeSettings>,
}

/// <p>Represents information about an action type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionTypeId {
    /// <p>A category defines what kind of action can be taken in the stage, and constrains the provider type for the action. Valid categories are limited to one of the values below.</p>
    #[serde(rename = "category")]
    pub category: String,
    /// <p>The creator of the action being called.</p>
    #[serde(rename = "owner")]
    pub owner: String,
    /// <p>The provider of the service being called by the action. Valid providers are determined by the action category. For example, an action in the Deploy category type might have a provider of AWS CodeDeploy, which would be specified as CodeDeploy. To reference a list of action providers by action type, see <a href="https://docs.aws.amazon.com/codepipeline/latest/userguide/reference-pipeline-structure.html#actions-valid-providers">Valid Action Types and Providers in CodePipeline</a>.</p>
    #[serde(rename = "provider")]
    pub provider: String,
    /// <p>A string that describes the action version.</p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>Returns information about the settings for an action type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionTypeSettings {
    /// <p>The URL returned to the AWS CodePipeline console that provides a deep link to the resources of the external system, such as the configuration page for an AWS CodeDeploy deployment group. This link is provided as part of the action display within the pipeline.</p>
    #[serde(rename = "entityUrlTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_url_template: Option<String>,
    /// <p>The URL returned to the AWS CodePipeline console that contains a link to the top-level landing page for the external system, such as console page for AWS CodeDeploy. This link is shown on the pipeline view page in the AWS CodePipeline console and provides a link to the execution entity of the external action.</p>
    #[serde(rename = "executionUrlTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_url_template: Option<String>,
    /// <p>The URL returned to the AWS CodePipeline console that contains a link to the page where customers can update or change the configuration of the external action.</p>
    #[serde(rename = "revisionUrlTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url_template: Option<String>,
    /// <p>The URL of a sign-up page where users can sign up for an external service and perform initial configuration of the action provided by that service.</p>
    #[serde(rename = "thirdPartyConfigurationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_configuration_url: Option<String>,
}

/// <p>Represents information about the result of an approval request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ApprovalResult {
    /// <p>The response submitted by a reviewer assigned to an approval action request.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>The summary of the current status of the approval request.</p>
    #[serde(rename = "summary")]
    pub summary: String,
}

/// <p>Represents information about an artifact that will be worked upon by actions in the pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Artifact {
    /// <p>The location of an artifact.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ArtifactLocation>,
    /// <p>The artifact's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The artifact's revision ID. Depending on the type of object, this could be a commit ID (GitHub) or a revision ID (Amazon S3).</p>
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

/// <p>Artifact details for the action execution, such as the artifact location.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ArtifactDetail {
    /// <p>The artifact object name for the action execution.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon S3 artifact location for the action execution.</p>
    #[serde(rename = "s3location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3location: Option<S3Location>,
}

/// <p>Returns information about the details of an artifact.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactDetails {
    /// <p>The maximum number of artifacts allowed for the action type.</p>
    #[serde(rename = "maximumCount")]
    pub maximum_count: i64,
    /// <p>The minimum number of artifacts allowed for the action type.</p>
    #[serde(rename = "minimumCount")]
    pub minimum_count: i64,
}

/// <p>Represents information about the location of an artifact.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ArtifactLocation {
    /// <p>The Amazon S3 bucket that contains the artifact.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3ArtifactLocation>,
    /// <p>The type of artifact in the location.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents revision details of an artifact. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ArtifactRevision {
    /// <p>The date and time when the most recent revision of the artifact was created, in timestamp format.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The name of an artifact. This name might be system-generated, such as "MyApp", or might be defined by the user when an action is created.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An additional identifier for a revision, such as a commit date or, for artifacts stored in Amazon S3 buckets, the ETag value.</p>
    #[serde(rename = "revisionChangeIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_change_identifier: Option<String>,
    /// <p>The revision ID of the artifact.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Summary information about the most recent revision of the artifact. For GitHub and AWS CodeCommit repositories, the commit message. For Amazon S3 buckets or actions, the user-provided content of a <code>codepipeline-artifact-revision-summary</code> key specified in the object metadata.</p>
    #[serde(rename = "revisionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_summary: Option<String>,
    /// <p>The commit ID for the artifact revision. For artifacts stored in GitHub or AWS CodeCommit repositories, the commit ID is linked to a commit details page.</p>
    #[serde(rename = "revisionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url: Option<String>,
}

/// <p><p>The Amazon S3 bucket where artifacts are stored for the pipeline.</p> <note> <p>You must include either <code>artifactStore</code> or <code>artifactStores</code> in your pipeline, but you cannot use both. If you create a cross-region action in your pipeline, you must use <code>artifactStores</code>.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactStore {
    /// <p>The encryption key used to encrypt the data in the artifact store, such as an AWS Key Management Service (AWS KMS) key. If this is undefined, the default key for Amazon S3 is used.</p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>The Amazon S3 bucket used for storing the artifacts for a pipeline. You can specify the name of an S3 bucket but not a folder within the bucket. A folder to contain the pipeline artifacts is created for you based on the name of the pipeline. You can use any Amazon S3 bucket in the same AWS Region as the pipeline to store your pipeline artifacts.</p>
    #[serde(rename = "location")]
    pub location: String,
    /// <p>The type of the artifact store, such as S3.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Reserved for future use.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockerDeclaration {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Represents the input of a CreateCustomActionType operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCustomActionTypeInput {
    /// <p><p>The category of the custom action, such as a build action or a test action.</p> <note> <p>Although <code>Source</code> and <code>Approval</code> are listed as valid values, they are not currently functional. These values are reserved for future use.</p> </note></p>
    #[serde(rename = "category")]
    pub category: String,
    /// <p><p>The configuration properties for the custom action.</p> <note> <p>You can refer to a name in the configuration properties of the custom action within the URL templates by following the format of {Config:name}, as long as the configuration property is both required and not secret. For more information, see <a href="https://docs.aws.amazon.com/codepipeline/latest/userguide/how-to-create-custom-action.html">Create a Custom Action for a Pipeline</a>.</p> </note></p>
    #[serde(rename = "configurationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_properties: Option<Vec<ActionConfigurationProperty>>,
    /// <p>The details of the input artifact for the action, such as its commit ID.</p>
    #[serde(rename = "inputArtifactDetails")]
    pub input_artifact_details: ArtifactDetails,
    /// <p>The details of the output artifact of the action, such as its commit ID.</p>
    #[serde(rename = "outputArtifactDetails")]
    pub output_artifact_details: ArtifactDetails,
    /// <p>The provider of the service used in the custom action, such as AWS CodeDeploy.</p>
    #[serde(rename = "provider")]
    pub provider: String,
    /// <p>URLs that provide users information about this custom action.</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ActionTypeSettings>,
    /// <p>The tags for the custom action.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The version identifier of the custom action.</p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>Represents the output of a <code>CreateCustomActionType</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCustomActionTypeOutput {
    /// <p>Returns information about the details of an action type.</p>
    #[serde(rename = "actionType")]
    pub action_type: ActionType,
    /// <p>Specifies the tags applied to the custom action.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents the input of a <code>CreatePipeline</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePipelineInput {
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    #[serde(rename = "pipeline")]
    pub pipeline: PipelineDeclaration,
    /// <p>The tags for the pipeline.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents the output of a <code>CreatePipeline</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePipelineOutput {
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    #[serde(rename = "pipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<PipelineDeclaration>,
    /// <p>Specifies the tags applied to the pipeline.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents information about a current revision.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CurrentRevision {
    /// <p>The change identifier for the current revision.</p>
    #[serde(rename = "changeIdentifier")]
    pub change_identifier: String,
    /// <p>The date and time when the most recent revision of the artifact was created, in timestamp format.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The revision ID of the current version of an artifact.</p>
    #[serde(rename = "revision")]
    pub revision: String,
    /// <p>The summary of the most recent revision of the artifact.</p>
    #[serde(rename = "revisionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_summary: Option<String>,
}

/// <p>Represents the input of a <code>DeleteCustomActionType</code> operation. The custom action will be marked as deleted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCustomActionTypeInput {
    /// <p>The category of the custom action that you want to delete, such as source or deploy.</p>
    #[serde(rename = "category")]
    pub category: String,
    /// <p>The provider of the service used in the custom action, such as AWS CodeDeploy.</p>
    #[serde(rename = "provider")]
    pub provider: String,
    /// <p>The version of the custom action to delete.</p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>Represents the input of a <code>DeletePipeline</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePipelineInput {
    /// <p>The name of the pipeline to be deleted.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteWebhookInput {
    /// <p>The name of the webhook you want to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteWebhookOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterWebhookWithThirdPartyInput {
    /// <p>The name of the webhook you want to deregister.</p>
    #[serde(rename = "webhookName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterWebhookWithThirdPartyOutput {}

/// <p>Represents the input of a <code>DisableStageTransition</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableStageTransitionInput {
    /// <p>The name of the pipeline in which you want to disable the flow of artifacts from one stage to another.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>The reason given to the user why a stage is disabled, such as waiting for manual approval or manual tests. This message is displayed in the pipeline console UI.</p>
    #[serde(rename = "reason")]
    pub reason: String,
    /// <p>The name of the stage where you want to disable the inbound or outbound transition of artifacts.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
    /// <p>Specifies whether artifacts will be prevented from transitioning into the stage and being processed by the actions in that stage (inbound), or prevented from transitioning from the stage after they have been processed by the actions in that stage (outbound).</p>
    #[serde(rename = "transitionType")]
    pub transition_type: String,
}

/// <p>Represents the input of an <code>EnableStageTransition</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableStageTransitionInput {
    /// <p>The name of the pipeline in which you want to enable the flow of artifacts from one stage to another.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>The name of the stage where you want to enable the transition of artifacts, either into the stage (inbound) or from that stage to the next stage (outbound).</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
    /// <p>Specifies whether artifacts will be allowed to enter the stage and be processed by the actions in that stage (inbound) or whether already-processed artifacts will be allowed to transition to the next stage (outbound).</p>
    #[serde(rename = "transitionType")]
    pub transition_type: String,
}

/// <p>Represents information about the key used to encrypt data in the artifact store, such as an AWS Key Management Service (AWS KMS) key.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionKey {
    /// <p><p>The ID used to identify the key. For an AWS KMS key, you can use the key ID, the key ARN, or the alias ARN.</p> <note> <p>Aliases are recognized only in the account that created the customer master key (CMK). For cross-account actions, you can only use the key ID or key ARN to identify the key.</p> </note></p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The type of encryption key, such as an AWS Key Management Service (AWS KMS) key. When creating or updating a pipeline, the value must be set to 'KMS'.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Represents information about an error in AWS CodePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorDetails {
    /// <p>The system ID or error number code of the error.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The text of the error message.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The details of the actions taken and results produced on an artifact as it passes through stages in the pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExecutionDetails {
    /// <p>The system-generated unique ID of this action used to identify this job worker in any external systems, such as AWS CodeDeploy.</p>
    #[serde(rename = "externalExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    /// <p>The percentage of work completed on the action, represented on a scale of zero to one hundred percent.</p>
    #[serde(rename = "percentComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i64>,
    /// <p>The summary of the current status of the actions.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// <p>The interaction or event that started a pipeline execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecutionTrigger {
    /// <p>Detail related to the event that started a pipeline execution, such as the webhook ARN of the webhook that triggered the pipeline execution or the user ARN for a user-initiated <code>start-pipeline-execution</code> CLI command.</p>
    #[serde(rename = "triggerDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_detail: Option<String>,
    /// <p>The type of change-detection method, command, or user interaction that started a pipeline execution.</p>
    #[serde(rename = "triggerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
}

/// <p>Represents information about failure details.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FailureDetails {
    /// <p>The external ID of the run of the action that failed.</p>
    #[serde(rename = "externalExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    /// <p>The message about the failure.</p>
    #[serde(rename = "message")]
    pub message: String,
    /// <p>The type of the failure.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Represents the input of a <code>GetJobDetails</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobDetailsInput {
    /// <p>The unique system-generated ID for the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p>Represents the output of a <code>GetJobDetails</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobDetailsOutput {
    /// <p><p>The details of the job.</p> <note> <p>If AWSSessionCredentials is used, a long-running job can call <code>GetJobDetails</code> again to obtain new credentials.</p> </note></p>
    #[serde(rename = "jobDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<JobDetails>,
}

/// <p>Represents the input of a <code>GetPipelineExecution</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPipelineExecutionInput {
    /// <p>The ID of the pipeline execution about which you want to get execution details.</p>
    #[serde(rename = "pipelineExecutionId")]
    pub pipeline_execution_id: String,
    /// <p>The name of the pipeline about which you want to get execution details.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
}

/// <p>Represents the output of a <code>GetPipelineExecution</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPipelineExecutionOutput {
    /// <p>Represents information about the execution of a pipeline.</p>
    #[serde(rename = "pipelineExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution: Option<PipelineExecution>,
}

/// <p>Represents the input of a <code>GetPipeline</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPipelineInput {
    /// <p>The name of the pipeline for which you want to get information. Pipeline names must be unique under an Amazon Web Services (AWS) user account.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version number of the pipeline. If you do not specify a version, defaults to the most current version.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Represents the output of a <code>GetPipeline</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPipelineOutput {
    /// <p>Represents the pipeline metadata information returned as part of the output of a <code>GetPipeline</code> action.</p>
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PipelineMetadata>,
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    #[serde(rename = "pipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<PipelineDeclaration>,
}

/// <p>Represents the input of a <code>GetPipelineState</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPipelineStateInput {
    /// <p>The name of the pipeline about which you want to get information.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Represents the output of a <code>GetPipelineState</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPipelineStateOutput {
    /// <p>The date and time the pipeline was created, in timestamp format.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The name of the pipeline for which you want to get the state.</p>
    #[serde(rename = "pipelineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    /// <p><p>The version number of the pipeline.</p> <note> <p>A newly-created pipeline is always assigned a version number of <code>1</code>.</p> </note></p>
    #[serde(rename = "pipelineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_version: Option<i64>,
    /// <p>A list of the pipeline stage output information, including stage name, state, most recent run details, whether the stage is disabled, and other data.</p>
    #[serde(rename = "stageStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_states: Option<Vec<StageState>>,
    /// <p>The date and time the pipeline was last updated, in timestamp format.</p>
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<f64>,
}

/// <p>Represents the input of a <code>GetThirdPartyJobDetails</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetThirdPartyJobDetailsInput {
    /// <p>The clientToken portion of the clientId and clientToken pair used to verify that the calling entity is allowed access to the job and its details.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The unique system-generated ID used for identifying the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p>Represents the output of a <code>GetThirdPartyJobDetails</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetThirdPartyJobDetailsOutput {
    /// <p>The details of the job, including any protected values defined for the job.</p>
    #[serde(rename = "jobDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<ThirdPartyJobDetails>,
}

/// <p>Represents information about an artifact to be worked on, such as a test or build artifact.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputArtifact {
    /// <p>The name of the artifact to be worked on, for example, "My App".</p> <p>The input artifact of an action must exactly match the output artifact declared in a preceding action, but the input artifact does not have to be the next action in strict sequence from the action that provided the output artifact. Actions in parallel can declare different output artifacts, which are in turn consumed by different following actions.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Represents information about a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Job {
    /// <p>The ID of the AWS account to use when performing the job.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Additional data about a job.</p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JobData>,
    /// <p>The unique system-generated ID of the job.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Use this number in an <a>AcknowledgeJob</a> request.</p>
    #[serde(rename = "nonce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

/// <p>Represents additional information about a job required for a job worker to complete the job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobData {
    /// <p>Represents information about an action configuration.</p>
    #[serde(rename = "actionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_configuration: Option<ActionConfiguration>,
    /// <p>Represents information about an action type.</p>
    #[serde(rename = "actionTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type_id: Option<ActionTypeId>,
    /// <p>Represents an AWS session credentials object. These credentials are temporary credentials that are issued by AWS Secure Token Service (STS). They can be used to access input and output artifacts in the Amazon S3 bucket used to store artifacts for the pipeline in AWS CodePipeline.</p>
    #[serde(rename = "artifactCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_credentials: Option<AWSSessionCredentials>,
    /// <p>A system-generated token, such as a AWS CodeDeploy deployment ID, that a job requires in order to continue the job asynchronously.</p>
    #[serde(rename = "continuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// <p>Represents information about the key used to encrypt data in the artifact store, such as an AWS Key Management Service (AWS KMS) key. </p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>The artifact supplied to the job.</p>
    #[serde(rename = "inputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<Artifact>>,
    /// <p>The output of the job.</p>
    #[serde(rename = "outputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<Artifact>>,
    /// <p><p>Represents information about a pipeline to a job worker.</p> <note> <p>Includes <code>pipelineArn</code> and <code>pipelineExecutionId</code> for Custom jobs.</p> </note></p>
    #[serde(rename = "pipelineContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_context: Option<PipelineContext>,
}

/// <p>Represents information about the details of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobDetails {
    /// <p>The AWS account ID associated with the job.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Represents additional information about a job required for a job worker to complete the job. </p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JobData>,
    /// <p>The unique system-generated ID of the job.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListActionExecutionsInput {
    /// <p>Input information used to filter action execution history.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ActionExecutionFilter>,
    /// <p><p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value. Action execution history is retained for up to 12 months, based on action execution start times. Default value is 100. </p> <note> <p>Detailed execution history is available for executions run on or after February 21, 2019.</p> </note></p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that was returned from the previous <code>ListActionExecutions</code> call, which can be used to return the next set of action executions in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The name of the pipeline for which you want to list action execution history.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListActionExecutionsOutput {
    /// <p>The details for a list of recent executions, such as action execution ID.</p>
    #[serde(rename = "actionExecutionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_execution_details: Option<Vec<ActionExecutionDetail>>,
    /// <p>If the amount of returned information is significantly large, an identifier is also returned and can be used in a subsequent <code>ListActionExecutions</code> call to return the next set of action executions in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input of a <code>ListActionTypes</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListActionTypesInput {
    /// <p>Filters the list of action types to those created by a specified entity.</p>
    #[serde(rename = "actionOwnerFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_owner_filter: Option<String>,
    /// <p>An identifier that was returned from the previous list action types call, which can be used to return the next set of action types in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a <code>ListActionTypes</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListActionTypesOutput {
    /// <p>Provides details of the action types.</p>
    #[serde(rename = "actionTypes")]
    pub action_types: Vec<ActionType>,
    /// <p>If the amount of returned information is significantly large, an identifier is also returned which can be used in a subsequent list action types call to return the next set of action types in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input of a <code>ListPipelineExecutions</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPipelineExecutionsInput {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value. Pipeline history is limited to the most recent 12 months, based on pipeline execution start times. Default value is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that was returned from the previous <code>ListPipelineExecutions</code> call, which can be used to return the next set of pipeline executions in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the pipeline for which you want to get execution summary information.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
}

/// <p>Represents the output of a <code>ListPipelineExecutions</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPipelineExecutionsOutput {
    /// <p>A token that can be used in the next <code>ListPipelineExecutions</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more nextToken values are returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of executions in the history of a pipeline.</p>
    #[serde(rename = "pipelineExecutionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_summaries: Option<Vec<PipelineExecutionSummary>>,
}

/// <p>Represents the input of a <code>ListPipelines</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPipelinesInput {
    /// <p>An identifier that was returned from the previous list pipelines call, which can be used to return the next set of pipelines in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a <code>ListPipelines</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPipelinesOutput {
    /// <p>If the amount of returned information is significantly large, an identifier is also returned which can be used in a subsequent list pipelines call to return the next set of pipelines in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of pipelines.</p>
    #[serde(rename = "pipelines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines: Option<Vec<PipelineSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceInput {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that was returned from the previous API call, which would be used to return the next page of the list. However, the ListTagsforResource call lists all available tags in one call and does not use pagination.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource to get tags for.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>If the amount of returned information is significantly large, an identifier is also returned and can be used in a subsequent API call to return the next page of the list. However, the ListTagsforResource call lists all available tags in one call and does not use pagination.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>The detail returned for each webhook after listing webhooks, such as the webhook URL, the webhook name, and the webhook ARN.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWebhookItem {
    /// <p>The Amazon Resource Name (ARN) of the webhook.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The detail returned for each webhook, such as the webhook authentication type and filter rules.</p>
    #[serde(rename = "definition")]
    pub definition: WebhookDefinition,
    /// <p>The number code of the error.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message about the webhook.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The date and time a webhook was last successfully triggered, in timestamp format.</p>
    #[serde(rename = "lastTriggered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_triggered: Option<f64>,
    /// <p>Specifies the tags applied to the webhook.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A unique URL generated by CodePipeline. When a POST request is made to this URL, the defined pipeline is started as long as the body of the post request satisfies the defined authentication and filtering conditions. Deleting and re-creating a webhook will make the old URL invalid and generate a new URL.</p>
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListWebhooksInput {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that was returned from the previous ListWebhooks call, which can be used to return the next set of webhooks in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWebhooksOutput {
    /// <p>If the amount of returned information is significantly large, an identifier is also returned and can be used in a subsequent ListWebhooks call to return the next set of webhooks in the list. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The JSON detail returned for each webhook in the list output for the ListWebhooks call.</p>
    #[serde(rename = "webhooks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<ListWebhookItem>>,
}

/// <p>Represents information about the output of an action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputArtifact {
    /// <p>The name of the output of an artifact, such as "My App".</p> <p>The input artifact of an action must exactly match the output artifact declared in a preceding action, but the input artifact does not have to be the next action in strict sequence from the action that provided the output artifact. Actions in parallel can declare different output artifacts, which are in turn consumed by different following actions.</p> <p>Output artifact names must be unique within a pipeline.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p><p>Represents information about a pipeline to a job worker.</p> <note> <p>PipelineContext contains <code>pipelineArn</code> and <code>pipelineExecutionId</code> for custom action jobs. The <code>pipelineArn</code> and <code>pipelineExecutionId</code> fields are not populated for ThirdParty action jobs.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PipelineContext {
    /// <p>The context of an action to a job worker within the stage of a pipeline.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionContext>,
    /// <p>The Amazon Resource Name (ARN) of the pipeline.</p>
    #[serde(rename = "pipelineArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    /// <p>The execution ID of the pipeline.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    /// <p>The name of the pipeline. This is a user-specified value. Pipeline names must be unique across all pipeline names under an Amazon Web Services account.</p>
    #[serde(rename = "pipelineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    /// <p>The stage of the pipeline.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<StageContext>,
}

/// <p>Represents the structure of actions and stages to be performed in the pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipelineDeclaration {
    /// <p><p>Represents information about the Amazon S3 bucket where artifacts are stored for the pipeline.</p> <note> <p>You must include either <code>artifactStore</code> or <code>artifactStores</code> in your pipeline, but you cannot use both. If you create a cross-region action in your pipeline, you must use <code>artifactStores</code>.</p> </note></p>
    #[serde(rename = "artifactStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_store: Option<ArtifactStore>,
    /// <p><p>A mapping of <code>artifactStore</code> objects and their corresponding regions. There must be an artifact store for the pipeline region and for each cross-region action within the pipeline.</p> <note> <p>You must include either <code>artifactStore</code> or <code>artifactStores</code> in your pipeline, but you cannot use both. If you create a cross-region action in your pipeline, you must use <code>artifactStores</code>.</p> </note></p>
    #[serde(rename = "artifactStores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_stores: Option<::std::collections::HashMap<String, ArtifactStore>>,
    /// <p>The name of the action to be performed.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) for AWS CodePipeline to use to either perform actions with no <code>actionRoleArn</code>, or to use to assume roles for actions with an <code>actionRoleArn</code>.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The stage in which to perform the action.</p>
    #[serde(rename = "stages")]
    pub stages: Vec<StageDeclaration>,
    /// <p>The version number of the pipeline. A new pipeline always has a version number of 1. This number is automatically incremented when a pipeline is updated.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Represents information about an execution of a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PipelineExecution {
    /// <p>A list of <code>ArtifactRevision</code> objects included in a pipeline execution.</p>
    #[serde(rename = "artifactRevisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_revisions: Option<Vec<ArtifactRevision>>,
    /// <p>The ID of the pipeline execution.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    /// <p>The name of the pipeline that was executed.</p>
    #[serde(rename = "pipelineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    /// <p>The version number of the pipeline that was executed.</p>
    #[serde(rename = "pipelineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_version: Option<i64>,
    /// <p><p>The status of the pipeline execution.</p> <ul> <li> <p>InProgress: The pipeline execution is currently running.</p> </li> <li> <p>Succeeded: The pipeline execution was completed successfully. </p> </li> <li> <p>Superseded: While this pipeline execution was waiting for the next stage to be completed, a newer pipeline execution advanced and continued through the pipeline instead. </p> </li> <li> <p>Failed: The pipeline execution was not completed successfully.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Summary information about a pipeline execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PipelineExecutionSummary {
    /// <p>The date and time of the last change to the pipeline execution, in timestamp format.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The ID of the pipeline execution.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    /// <p>A list of the source artifact revisions that initiated a pipeline execution.</p>
    #[serde(rename = "sourceRevisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_revisions: Option<Vec<SourceRevision>>,
    /// <p>The date and time when the pipeline execution began, in timestamp format.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p><p>The status of the pipeline execution.</p> <ul> <li> <p>InProgress: The pipeline execution is currently running.</p> </li> <li> <p>Succeeded: The pipeline execution was completed successfully. </p> </li> <li> <p>Superseded: While this pipeline execution was waiting for the next stage to be completed, a newer pipeline execution advanced and continued through the pipeline instead. </p> </li> <li> <p>Failed: The pipeline execution was not completed successfully.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The interaction or event that started a pipeline execution, such as automated change detection or a <code>StartPipelineExecution</code> API call.</p>
    #[serde(rename = "trigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<ExecutionTrigger>,
}

/// <p>Information about a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PipelineMetadata {
    /// <p>The date and time the pipeline was created, in timestamp format.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the pipeline.</p>
    #[serde(rename = "pipelineArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    /// <p>The date and time the pipeline was last updated, in timestamp format.</p>
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<f64>,
}

/// <p>Returns a summary of a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PipelineSummary {
    /// <p>The date and time the pipeline was created, in timestamp format.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The name of the pipeline.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The date and time of the last update to the pipeline, in timestamp format.</p>
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<f64>,
    /// <p>The version number of the pipeline.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Represents the input of a <code>PollForJobs</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PollForJobsInput {
    /// <p>Represents information about an action type.</p>
    #[serde(rename = "actionTypeId")]
    pub action_type_id: ActionTypeId,
    /// <p>The maximum number of jobs to return in a poll for jobs call.</p>
    #[serde(rename = "maxBatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i64>,
    /// <p>A map of property names and values. For an action type with no queryable properties, this value must be null or an empty map. For an action type with a queryable property, you must supply that property as a key in the map. Only jobs whose action configuration matches the mapped value will be returned.</p>
    #[serde(rename = "queryParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_param: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents the output of a <code>PollForJobs</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PollForJobsOutput {
    /// <p>Information about the jobs to take action on.</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
}

/// <p>Represents the input of a <code>PollForThirdPartyJobs</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PollForThirdPartyJobsInput {
    /// <p>Represents information about an action type.</p>
    #[serde(rename = "actionTypeId")]
    pub action_type_id: ActionTypeId,
    /// <p>The maximum number of jobs to return in a poll for jobs call.</p>
    #[serde(rename = "maxBatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i64>,
}

/// <p>Represents the output of a <code>PollForThirdPartyJobs</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PollForThirdPartyJobsOutput {
    /// <p>Information about the jobs to take action on.</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<ThirdPartyJob>>,
}

/// <p>Represents the input of a <code>PutActionRevision</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutActionRevisionInput {
    /// <p>The name of the action that will process the revision.</p>
    #[serde(rename = "actionName")]
    pub action_name: String,
    /// <p>Represents information about the version (or revision) of an action.</p>
    #[serde(rename = "actionRevision")]
    pub action_revision: ActionRevision,
    /// <p>The name of the pipeline that will start processing the revision to the source.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>The name of the stage that contains the action that will act upon the revision.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Represents the output of a <code>PutActionRevision</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutActionRevisionOutput {
    /// <p>Indicates whether the artifact revision was previously used in an execution of the specified pipeline.</p>
    #[serde(rename = "newRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_revision: Option<bool>,
    /// <p>The ID of the current workflow state of the pipeline.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

/// <p>Represents the input of a <code>PutApprovalResult</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutApprovalResultInput {
    /// <p>The name of the action for which approval is requested.</p>
    #[serde(rename = "actionName")]
    pub action_name: String,
    /// <p>The name of the pipeline that contains the action. </p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>Represents information about the result of the approval request.</p>
    #[serde(rename = "result")]
    pub result: ApprovalResult,
    /// <p>The name of the stage that contains the action.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
    /// <p>The system-generated token used to identify a unique approval request. The token for each open approval request can be obtained using the <a>GetPipelineState</a> action and is used to validate that the approval request corresponding to this token is still valid.</p>
    #[serde(rename = "token")]
    pub token: String,
}

/// <p>Represents the output of a <code>PutApprovalResult</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutApprovalResultOutput {
    /// <p>The timestamp showing when the approval or rejection was submitted.</p>
    #[serde(rename = "approvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<f64>,
}

/// <p>Represents the input of a <code>PutJobFailureResult</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutJobFailureResultInput {
    /// <p>The details about the failure of a job.</p>
    #[serde(rename = "failureDetails")]
    pub failure_details: FailureDetails,
    /// <p>The unique system-generated ID of the job that failed. This is the same ID returned from <code>PollForJobs</code>.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p>Represents the input of a <code>PutJobSuccessResult</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutJobSuccessResultInput {
    /// <p>A token generated by a job worker, such as an AWS CodeDeploy deployment ID, that a successful job provides to identify a custom action in progress. Future jobs will use this token in order to identify the running instance of the action. It can be reused to return additional information about the progress of the custom action. When the action is complete, no continuation token should be supplied.</p>
    #[serde(rename = "continuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// <p>The ID of the current revision of the artifact successfully worked upon by the job.</p>
    #[serde(rename = "currentRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<CurrentRevision>,
    /// <p>The execution details of the successful job, such as the actions taken by the job worker.</p>
    #[serde(rename = "executionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_details: Option<ExecutionDetails>,
    /// <p>The unique system-generated ID of the job that succeeded. This is the same ID returned from <code>PollForJobs</code>.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p>Represents the input of a <code>PutThirdPartyJobFailureResult</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutThirdPartyJobFailureResultInput {
    /// <p>The clientToken portion of the clientId and clientToken pair used to verify that the calling entity is allowed access to the job and its details.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>Represents information about failure details.</p>
    #[serde(rename = "failureDetails")]
    pub failure_details: FailureDetails,
    /// <p>The ID of the job that failed. This is the same ID returned from <code>PollForThirdPartyJobs</code>.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

/// <p>Represents the input of a <code>PutThirdPartyJobSuccessResult</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutThirdPartyJobSuccessResultInput {
    /// <p>The clientToken portion of the clientId and clientToken pair used to verify that the calling entity is allowed access to the job and its details.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>A token generated by a job worker, such as an AWS CodeDeploy deployment ID, that a successful job provides to identify a partner action in progress. Future jobs will use this token in order to identify the running instance of the action. It can be reused to return additional information about the progress of the partner action. When the action is complete, no continuation token should be supplied.</p>
    #[serde(rename = "continuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// <p>Represents information about a current revision.</p>
    #[serde(rename = "currentRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<CurrentRevision>,
    /// <p>The details of the actions taken and results produced on an artifact as it passes through stages in the pipeline. </p>
    #[serde(rename = "executionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_details: Option<ExecutionDetails>,
    /// <p>The ID of the job that successfully completed. This is the same ID returned from <code>PollForThirdPartyJobs</code>.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutWebhookInput {
    /// <p>The tags for the webhook.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The detail provided in an input file to create the webhook, such as the webhook name, the pipeline name, and the action name. Give the webhook a unique name which identifies the webhook being defined. You may choose to name the webhook after the pipeline and action it targets so that you can easily recognize what it's used for later.</p>
    #[serde(rename = "webhook")]
    pub webhook: WebhookDefinition,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutWebhookOutput {
    /// <p>The detail returned from creating the webhook, such as the webhook name, webhook URL, and webhook ARN.</p>
    #[serde(rename = "webhook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<ListWebhookItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterWebhookWithThirdPartyInput {
    /// <p>The name of an existing webhook created with PutWebhook to register with a supported third party. </p>
    #[serde(rename = "webhookName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterWebhookWithThirdPartyOutput {}

/// <p>Represents the input of a <code>RetryStageExecution</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RetryStageExecutionInput {
    /// <p>The ID of the pipeline execution in the failed stage to be retried. Use the <a>GetPipelineState</a> action to retrieve the current pipelineExecutionId of the failed stage</p>
    #[serde(rename = "pipelineExecutionId")]
    pub pipeline_execution_id: String,
    /// <p>The name of the pipeline that contains the failed stage.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>The scope of the retry attempt. Currently, the only supported value is FAILED_ACTIONS.</p>
    #[serde(rename = "retryMode")]
    pub retry_mode: String,
    /// <p>The name of the failed stage to be retried.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Represents the output of a <code>RetryStageExecution</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RetryStageExecutionOutput {
    /// <p>The ID of the current workflow execution in the failed stage.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

/// <p>The location of the Amazon S3 bucket that contains a revision.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3ArtifactLocation {
    /// <p>The name of the Amazon S3 bucket.</p>
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// <p>The key of the object in the Amazon S3 bucket, which uniquely identifies the object in the bucket.</p>
    #[serde(rename = "objectKey")]
    pub object_key: String,
}

/// <p>The Amazon S3 artifact location for an action's artifacts.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3Location {
    /// <p>The Amazon S3 artifact bucket for an action's artifacts.</p>
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>The artifact name.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// <p>Information about the version (or revision) of a source artifact that initiated a pipeline execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SourceRevision {
    /// <p>The name of the action that processed the revision to the source artifact.</p>
    #[serde(rename = "actionName")]
    pub action_name: String,
    /// <p>The system-generated unique ID that identifies the revision number of the artifact.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Summary information about the most recent revision of the artifact. For GitHub and AWS CodeCommit repositories, the commit message. For Amazon S3 buckets or actions, the user-provided content of a <code>codepipeline-artifact-revision-summary</code> key specified in the object metadata.</p>
    #[serde(rename = "revisionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_summary: Option<String>,
    /// <p>The commit ID for the artifact revision. For artifacts stored in GitHub or AWS CodeCommit repositories, the commit ID is linked to a commit details page.</p>
    #[serde(rename = "revisionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url: Option<String>,
}

/// <p>Represents information about a stage to a job worker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StageContext {
    /// <p>The name of the stage.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents information about a stage and its definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageDeclaration {
    /// <p>The actions included in a stage.</p>
    #[serde(rename = "actions")]
    pub actions: Vec<ActionDeclaration>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "blockers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockers: Option<Vec<BlockerDeclaration>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Represents information about the run of a stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StageExecution {
    /// <p>The ID of the pipeline execution associated with the stage.</p>
    #[serde(rename = "pipelineExecutionId")]
    pub pipeline_execution_id: String,
    /// <p>The status of the stage, or for a completed stage, the last status of the stage.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Represents information about the state of the stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StageState {
    /// <p>The state of the stage.</p>
    #[serde(rename = "actionStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_states: Option<Vec<ActionState>>,
    /// <p>The state of the inbound transition, which is either enabled or disabled.</p>
    #[serde(rename = "inboundTransitionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transition_state: Option<TransitionState>,
    /// <p>Information about the latest execution in the stage, including its ID and status.</p>
    #[serde(rename = "latestExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_execution: Option<StageExecution>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "stageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

/// <p>Represents the input of a <code>StartPipelineExecution</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartPipelineExecutionInput {
    /// <p>The system-generated unique ID used to identify a unique execution request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the pipeline to start.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Represents the output of a <code>StartPipelineExecution</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartPipelineExecutionOutput {
    /// <p>The unique system-generated ID of the pipeline execution that was started.</p>
    #[serde(rename = "pipelineExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

/// <p>A tag is a key/value pair that is used to manage the resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The tag's key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The tag's value.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource you want to add tags to.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags you want to modify or add to the resource.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceOutput {}

/// <p>A response to a <code>PollForThirdPartyJobs </code>request returned by AWS CodePipeline when there is a job to be worked upon by a partner action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ThirdPartyJob {
    /// <p>The <code>clientToken</code> portion of the <code>clientId</code> and <code>clientToken</code> pair used to verify that the calling entity is allowed access to the job and its details.</p>
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The identifier used to identify the job in AWS CodePipeline.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p>Represents information about the job data for a partner action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ThirdPartyJobData {
    /// <p>Represents information about an action configuration.</p>
    #[serde(rename = "actionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_configuration: Option<ActionConfiguration>,
    /// <p>Represents information about an action type.</p>
    #[serde(rename = "actionTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type_id: Option<ActionTypeId>,
    /// <p>Represents an AWS session credentials object. These credentials are temporary credentials that are issued by AWS Secure Token Service (STS). They can be used to access input and output artifacts in the Amazon S3 bucket used to store artifact for the pipeline in AWS CodePipeline. </p>
    #[serde(rename = "artifactCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_credentials: Option<AWSSessionCredentials>,
    /// <p>A system-generated token, such as a AWS CodeDeploy deployment ID, that a job requires in order to continue the job asynchronously.</p>
    #[serde(rename = "continuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// <p>The encryption key used to encrypt and decrypt data in the artifact store for the pipeline, such as an AWS Key Management Service (AWS KMS) key. This is optional and might not be present.</p>
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>The name of the artifact that will be worked upon by the action, if any. This name might be system-generated, such as "MyApp", or might be defined by the user when the action is created. The input artifact name must match the name of an output artifact generated by an action in an earlier action or stage of the pipeline.</p>
    #[serde(rename = "inputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<Artifact>>,
    /// <p>The name of the artifact that will be the result of the action, if any. This name might be system-generated, such as "MyBuiltApp", or might be defined by the user when the action is created.</p>
    #[serde(rename = "outputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<Artifact>>,
    /// <p><p>Represents information about a pipeline to a job worker.</p> <note> <p>Does not include <code>pipelineArn</code> and <code>pipelineExecutionId</code> for ThirdParty jobs.</p> </note></p>
    #[serde(rename = "pipelineContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_context: Option<PipelineContext>,
}

/// <p>The details of a job sent in response to a <code>GetThirdPartyJobDetails</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ThirdPartyJobDetails {
    /// <p>The data to be returned by the third party job worker.</p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ThirdPartyJobData>,
    /// <p>The identifier used to identify the job details in AWS CodePipeline.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Use this number in an <a>AcknowledgeThirdPartyJob</a> request.</p>
    #[serde(rename = "nonce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

/// <p>Represents information about the state of transitions between one stage and another stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TransitionState {
    /// <p>The user-specified reason why the transition between two stages of a pipeline was disabled.</p>
    #[serde(rename = "disabledReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// <p>Whether the transition between stages is enabled (true) or disabled (false).</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The timestamp when the transition state was last changed.</p>
    #[serde(rename = "lastChangedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_at: Option<f64>,
    /// <p>The ID of the user who last changed the transition state.</p>
    #[serde(rename = "lastChangedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceInput {
    /// <p> The Amazon Resource Name (ARN) of the resource to remove tags from.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The list of keys for the tags to be removed from the resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceOutput {}

/// <p>Represents the input of an <code>UpdatePipeline</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePipelineInput {
    /// <p>The name of the pipeline to be updated.</p>
    #[serde(rename = "pipeline")]
    pub pipeline: PipelineDeclaration,
}

/// <p>Represents the output of an <code>UpdatePipeline</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePipelineOutput {
    /// <p>The structure of the updated pipeline.</p>
    #[serde(rename = "pipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<PipelineDeclaration>,
}

/// <p>The authentication applied to incoming webhook trigger requests.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookAuthConfiguration {
    /// <p>The property used to configure acceptance of webhooks within a specific IP range. For IP, only the <code>AllowedIPRange</code> property must be set, and this property must be set to a valid CIDR range.</p>
    #[serde(rename = "AllowedIPRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ip_range: Option<String>,
    /// <p>The property used to configure GitHub authentication. For GITHUB_HMAC, only the <code>SecretToken</code> property must be set.</p>
    #[serde(rename = "SecretToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

/// <p>Represents information about a webhook and its definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookDefinition {
    /// <p><p>Supported options are GITHUB<em>HMAC, IP and UNAUTHENTICATED.</p> <ul> <li> <p>For information about the authentication scheme implemented by GITHUB</em>HMAC, see <a href="https://developer.github.com/webhooks/securing/">Securing your webhooks</a> on the GitHub Developer website.</p> </li> <li> <p> IP will reject webhooks trigger requests unless they originate from an IP within the IP range whitelisted in the authentication configuration.</p> </li> <li> <p> UNAUTHENTICATED will accept all webhook trigger requests regardless of origin.</p> </li> </ul></p>
    #[serde(rename = "authentication")]
    pub authentication: String,
    /// <p>Properties that configure the authentication applied to incoming webhook trigger requests. The required properties depend on the authentication type. For GITHUB_HMAC, only the <code>SecretToken </code>property must be set. For IP, only the <code>AllowedIPRange </code>property must be set to a valid CIDR range. For UNAUTHENTICATED, no properties can be set.</p>
    #[serde(rename = "authenticationConfiguration")]
    pub authentication_configuration: WebhookAuthConfiguration,
    /// <p>A list of rules applied to the body/payload sent in the POST request to a webhook URL. All defined rules must pass for the request to be accepted and the pipeline started.</p>
    #[serde(rename = "filters")]
    pub filters: Vec<WebhookFilterRule>,
    /// <p>The name of the webhook.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The name of the action in a pipeline you want to connect to the webhook. The action must be from the source (first) stage of the pipeline.</p>
    #[serde(rename = "targetAction")]
    pub target_action: String,
    /// <p>The name of the pipeline you want to connect to the webhook.</p>
    #[serde(rename = "targetPipeline")]
    pub target_pipeline: String,
}

/// <p>The event criteria that specify when a webhook notification is sent to your URL.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookFilterRule {
    /// <p>A JsonPath expression that will be applied to the body/payload of the webhook. The value selected by the JsonPath expression must match the value specified in the <code>MatchEquals</code> field, otherwise the request will be ignored. For more information about JsonPath expressions, see <a href="https://github.com/json-path/JsonPath">Java JsonPath implementation</a> in GitHub.</p>
    #[serde(rename = "jsonPath")]
    pub json_path: String,
    /// <p>The value selected by the <code>JsonPath</code> expression must match what is supplied in the <code>MatchEquals</code> field, otherwise the request will be ignored. Properties from the target action configuration can be included as placeholders in this value by surrounding the action configuration key with curly braces. For example, if the value supplied here is "refs/heads/{Branch}" and the target action has an action configuration property called "Branch" with a value of "master", the <code>MatchEquals</code> value will be evaluated as "refs/heads/master". For a list of action configuration properties for built-in action types, see <a href="https://docs.aws.amazon.com/codepipeline/latest/userguide/reference-pipeline-structure.html#action-requirements">Pipeline Structure Reference Action Requirements</a>.</p>
    #[serde(rename = "matchEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_equals: Option<String>,
}

/// Errors returned by AcknowledgeJob
#[derive(Debug, PartialEq)]
pub enum AcknowledgeJobError {
    /// <p>The specified nonce was specified in an invalid format.</p>
    InvalidNonce(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
}

impl AcknowledgeJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcknowledgeJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNonceException" => {
                    return RusotoError::Service(AcknowledgeJobError::InvalidNonce(err.msg))
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(AcknowledgeJobError::JobNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AcknowledgeJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcknowledgeJobError {
    fn description(&self) -> &str {
        match *self {
            AcknowledgeJobError::InvalidNonce(ref cause) => cause,
            AcknowledgeJobError::JobNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by AcknowledgeThirdPartyJob
#[derive(Debug, PartialEq)]
pub enum AcknowledgeThirdPartyJobError {
    /// <p>The client token was specified in an invalid format</p>
    InvalidClientToken(String),
    /// <p>The specified nonce was specified in an invalid format.</p>
    InvalidNonce(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
}

impl AcknowledgeThirdPartyJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcknowledgeThirdPartyJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidClientTokenException" => {
                    return RusotoError::Service(AcknowledgeThirdPartyJobError::InvalidClientToken(
                        err.msg,
                    ))
                }
                "InvalidNonceException" => {
                    return RusotoError::Service(AcknowledgeThirdPartyJobError::InvalidNonce(
                        err.msg,
                    ))
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(AcknowledgeThirdPartyJobError::JobNotFound(
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
impl fmt::Display for AcknowledgeThirdPartyJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcknowledgeThirdPartyJobError {
    fn description(&self) -> &str {
        match *self {
            AcknowledgeThirdPartyJobError::InvalidClientToken(ref cause) => cause,
            AcknowledgeThirdPartyJobError::InvalidNonce(ref cause) => cause,
            AcknowledgeThirdPartyJobError::JobNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCustomActionType
#[derive(Debug, PartialEq)]
pub enum CreateCustomActionTypeError {
    /// <p>Unable to modify the tag due to a simultaneous update request.</p>
    ConcurrentModification(String),
    /// <p>The specified resource tags are invalid.</p>
    InvalidTags(String),
    /// <p>The number of pipelines associated with the AWS account has exceeded the limit allowed for the account.</p>
    LimitExceeded(String),
    /// <p>The tags limit for a resource has been exceeded.</p>
    TooManyTags(String),
}

impl CreateCustomActionTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCustomActionTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreateCustomActionTypeError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidTagsException" => {
                    return RusotoError::Service(CreateCustomActionTypeError::InvalidTags(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateCustomActionTypeError::LimitExceeded(
                        err.msg,
                    ))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateCustomActionTypeError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCustomActionTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCustomActionTypeError {
    fn description(&self) -> &str {
        match *self {
            CreateCustomActionTypeError::ConcurrentModification(ref cause) => cause,
            CreateCustomActionTypeError::InvalidTags(ref cause) => cause,
            CreateCustomActionTypeError::LimitExceeded(ref cause) => cause,
            CreateCustomActionTypeError::TooManyTags(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePipeline
#[derive(Debug, PartialEq)]
pub enum CreatePipelineError {
    /// <p>Unable to modify the tag due to a simultaneous update request.</p>
    ConcurrentModification(String),
    /// <p>The specified action declaration was specified in an invalid format.</p>
    InvalidActionDeclaration(String),
    /// <p>Reserved for future use.</p>
    InvalidBlockerDeclaration(String),
    /// <p>The specified stage declaration was specified in an invalid format.</p>
    InvalidStageDeclaration(String),
    /// <p>The specified structure was specified in an invalid format.</p>
    InvalidStructure(String),
    /// <p>The specified resource tags are invalid.</p>
    InvalidTags(String),
    /// <p>The number of pipelines associated with the AWS account has exceeded the limit allowed for the account.</p>
    LimitExceeded(String),
    /// <p>The specified pipeline name is already in use.</p>
    PipelineNameInUse(String),
    /// <p>The tags limit for a resource has been exceeded.</p>
    TooManyTags(String),
}

impl CreatePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePipelineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreatePipelineError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidActionDeclarationException" => {
                    return RusotoError::Service(CreatePipelineError::InvalidActionDeclaration(
                        err.msg,
                    ))
                }
                "InvalidBlockerDeclarationException" => {
                    return RusotoError::Service(CreatePipelineError::InvalidBlockerDeclaration(
                        err.msg,
                    ))
                }
                "InvalidStageDeclarationException" => {
                    return RusotoError::Service(CreatePipelineError::InvalidStageDeclaration(
                        err.msg,
                    ))
                }
                "InvalidStructureException" => {
                    return RusotoError::Service(CreatePipelineError::InvalidStructure(err.msg))
                }
                "InvalidTagsException" => {
                    return RusotoError::Service(CreatePipelineError::InvalidTags(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreatePipelineError::LimitExceeded(err.msg))
                }
                "PipelineNameInUseException" => {
                    return RusotoError::Service(CreatePipelineError::PipelineNameInUse(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreatePipelineError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreatePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePipelineError {
    fn description(&self) -> &str {
        match *self {
            CreatePipelineError::ConcurrentModification(ref cause) => cause,
            CreatePipelineError::InvalidActionDeclaration(ref cause) => cause,
            CreatePipelineError::InvalidBlockerDeclaration(ref cause) => cause,
            CreatePipelineError::InvalidStageDeclaration(ref cause) => cause,
            CreatePipelineError::InvalidStructure(ref cause) => cause,
            CreatePipelineError::InvalidTags(ref cause) => cause,
            CreatePipelineError::LimitExceeded(ref cause) => cause,
            CreatePipelineError::PipelineNameInUse(ref cause) => cause,
            CreatePipelineError::TooManyTags(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCustomActionType
#[derive(Debug, PartialEq)]
pub enum DeleteCustomActionTypeError {
    /// <p>Unable to modify the tag due to a simultaneous update request.</p>
    ConcurrentModification(String),
}

impl DeleteCustomActionTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCustomActionTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteCustomActionTypeError::ConcurrentModification(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteCustomActionTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCustomActionTypeError {
    fn description(&self) -> &str {
        match *self {
            DeleteCustomActionTypeError::ConcurrentModification(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePipeline
#[derive(Debug, PartialEq)]
pub enum DeletePipelineError {
    /// <p>Unable to modify the tag due to a simultaneous update request.</p>
    ConcurrentModification(String),
}

impl DeletePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePipelineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeletePipelineError::ConcurrentModification(
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
impl fmt::Display for DeletePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePipelineError {
    fn description(&self) -> &str {
        match *self {
            DeletePipelineError::ConcurrentModification(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteWebhook
#[derive(Debug, PartialEq)]
pub enum DeleteWebhookError {
    /// <p>Unable to modify the tag due to a simultaneous update request.</p>
    ConcurrentModification(String),
}

impl DeleteWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWebhookError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteWebhookError::ConcurrentModification(
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
impl fmt::Display for DeleteWebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteWebhookError {
    fn description(&self) -> &str {
        match *self {
            DeleteWebhookError::ConcurrentModification(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterWebhookWithThirdParty
#[derive(Debug, PartialEq)]
pub enum DeregisterWebhookWithThirdPartyError {
    /// <p>The specified webhook was entered in an invalid format or cannot be found.</p>
    WebhookNotFound(String),
}

impl DeregisterWebhookWithThirdPartyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeregisterWebhookWithThirdPartyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WebhookNotFoundException" => {
                    return RusotoError::Service(
                        DeregisterWebhookWithThirdPartyError::WebhookNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeregisterWebhookWithThirdPartyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterWebhookWithThirdPartyError {
    fn description(&self) -> &str {
        match *self {
            DeregisterWebhookWithThirdPartyError::WebhookNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableStageTransition
#[derive(Debug, PartialEq)]
pub enum DisableStageTransitionError {
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified stage was specified in an invalid format or cannot be found.</p>
    StageNotFound(String),
}

impl DisableStageTransitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableStageTransitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "PipelineNotFoundException" => {
                    return RusotoError::Service(DisableStageTransitionError::PipelineNotFound(
                        err.msg,
                    ))
                }
                "StageNotFoundException" => {
                    return RusotoError::Service(DisableStageTransitionError::StageNotFound(
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
impl fmt::Display for DisableStageTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableStageTransitionError {
    fn description(&self) -> &str {
        match *self {
            DisableStageTransitionError::PipelineNotFound(ref cause) => cause,
            DisableStageTransitionError::StageNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableStageTransition
#[derive(Debug, PartialEq)]
pub enum EnableStageTransitionError {
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified stage was specified in an invalid format or cannot be found.</p>
    StageNotFound(String),
}

impl EnableStageTransitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableStageTransitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "PipelineNotFoundException" => {
                    return RusotoError::Service(EnableStageTransitionError::PipelineNotFound(
                        err.msg,
                    ))
                }
                "StageNotFoundException" => {
                    return RusotoError::Service(EnableStageTransitionError::StageNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EnableStageTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableStageTransitionError {
    fn description(&self) -> &str {
        match *self {
            EnableStageTransitionError::PipelineNotFound(ref cause) => cause,
            EnableStageTransitionError::StageNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobDetails
#[derive(Debug, PartialEq)]
pub enum GetJobDetailsError {
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
}

impl GetJobDetailsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobDetailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "JobNotFoundException" => {
                    return RusotoError::Service(GetJobDetailsError::JobNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobDetailsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobDetailsError {
    fn description(&self) -> &str {
        match *self {
            GetJobDetailsError::JobNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPipeline
#[derive(Debug, PartialEq)]
pub enum GetPipelineError {
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified pipeline version was specified in an invalid format or cannot be found.</p>
    PipelineVersionNotFound(String),
}

impl GetPipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPipelineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "PipelineNotFoundException" => {
                    return RusotoError::Service(GetPipelineError::PipelineNotFound(err.msg))
                }
                "PipelineVersionNotFoundException" => {
                    return RusotoError::Service(GetPipelineError::PipelineVersionNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPipelineError {
    fn description(&self) -> &str {
        match *self {
            GetPipelineError::PipelineNotFound(ref cause) => cause,
            GetPipelineError::PipelineVersionNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPipelineExecution
#[derive(Debug, PartialEq)]
pub enum GetPipelineExecutionError {
    /// <p>The pipeline execution was specified in an invalid format or cannot be found, or an execution ID does not belong to the specified pipeline. </p>
    PipelineExecutionNotFound(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
}

impl GetPipelineExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPipelineExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "PipelineExecutionNotFoundException" => {
                    return RusotoError::Service(
                        GetPipelineExecutionError::PipelineExecutionNotFound(err.msg),
                    )
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(GetPipelineExecutionError::PipelineNotFound(
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
impl fmt::Display for GetPipelineExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPipelineExecutionError {
    fn description(&self) -> &str {
        match *self {
            GetPipelineExecutionError::PipelineExecutionNotFound(ref cause) => cause,
            GetPipelineExecutionError::PipelineNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPipelineState
#[derive(Debug, PartialEq)]
pub enum GetPipelineStateError {
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
}

impl GetPipelineStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPipelineStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "PipelineNotFoundException" => {
                    return RusotoError::Service(GetPipelineStateError::PipelineNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPipelineStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPipelineStateError {
    fn description(&self) -> &str {
        match *self {
            GetPipelineStateError::PipelineNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetThirdPartyJobDetails
#[derive(Debug, PartialEq)]
pub enum GetThirdPartyJobDetailsError {
    /// <p>The client token was specified in an invalid format</p>
    InvalidClientToken(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    InvalidJob(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
}

impl GetThirdPartyJobDetailsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetThirdPartyJobDetailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidClientTokenException" => {
                    return RusotoError::Service(GetThirdPartyJobDetailsError::InvalidClientToken(
                        err.msg,
                    ))
                }
                "InvalidJobException" => {
                    return RusotoError::Service(GetThirdPartyJobDetailsError::InvalidJob(err.msg))
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(GetThirdPartyJobDetailsError::JobNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetThirdPartyJobDetailsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetThirdPartyJobDetailsError {
    fn description(&self) -> &str {
        match *self {
            GetThirdPartyJobDetailsError::InvalidClientToken(ref cause) => cause,
            GetThirdPartyJobDetailsError::InvalidJob(ref cause) => cause,
            GetThirdPartyJobDetailsError::JobNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListActionExecutions
#[derive(Debug, PartialEq)]
pub enum ListActionExecutionsError {
    /// <p>The next token was specified in an invalid format. Make sure that the next token you provided is the token returned by a previous call.</p>
    InvalidNextToken(String),
    /// <p>The pipeline execution was specified in an invalid format or cannot be found, or an execution ID does not belong to the specified pipeline. </p>
    PipelineExecutionNotFound(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
}

impl ListActionExecutionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListActionExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListActionExecutionsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "PipelineExecutionNotFoundException" => {
                    return RusotoError::Service(
                        ListActionExecutionsError::PipelineExecutionNotFound(err.msg),
                    )
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(ListActionExecutionsError::PipelineNotFound(
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
impl fmt::Display for ListActionExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListActionExecutionsError {
    fn description(&self) -> &str {
        match *self {
            ListActionExecutionsError::InvalidNextToken(ref cause) => cause,
            ListActionExecutionsError::PipelineExecutionNotFound(ref cause) => cause,
            ListActionExecutionsError::PipelineNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListActionTypes
#[derive(Debug, PartialEq)]
pub enum ListActionTypesError {
    /// <p>The next token was specified in an invalid format. Make sure that the next token you provided is the token returned by a previous call.</p>
    InvalidNextToken(String),
}

impl ListActionTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListActionTypesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListActionTypesError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListActionTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListActionTypesError {
    fn description(&self) -> &str {
        match *self {
            ListActionTypesError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPipelineExecutions
#[derive(Debug, PartialEq)]
pub enum ListPipelineExecutionsError {
    /// <p>The next token was specified in an invalid format. Make sure that the next token you provided is the token returned by a previous call.</p>
    InvalidNextToken(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
}

impl ListPipelineExecutionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPipelineExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListPipelineExecutionsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(ListPipelineExecutionsError::PipelineNotFound(
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
impl fmt::Display for ListPipelineExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPipelineExecutionsError {
    fn description(&self) -> &str {
        match *self {
            ListPipelineExecutionsError::InvalidNextToken(ref cause) => cause,
            ListPipelineExecutionsError::PipelineNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPipelines
#[derive(Debug, PartialEq)]
pub enum ListPipelinesError {
    /// <p>The next token was specified in an invalid format. Make sure that the next token you provided is the token returned by a previous call.</p>
    InvalidNextToken(String),
}

impl ListPipelinesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPipelinesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListPipelinesError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPipelinesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPipelinesError {
    fn description(&self) -> &str {
        match *self {
            ListPipelinesError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The specified resource ARN is invalid.</p>
    InvalidArn(String),
    /// <p>The next token was specified in an invalid format. Make sure that the next token you provided is the token returned by a previous call.</p>
    InvalidNextToken(String),
    /// <p>The specified resource was specified in an invalid format.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidArn(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidNextToken(
                        err.msg,
                    ))
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
            ListTagsForResourceError::InvalidArn(ref cause) => cause,
            ListTagsForResourceError::InvalidNextToken(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListWebhooks
#[derive(Debug, PartialEq)]
pub enum ListWebhooksError {
    /// <p>The next token was specified in an invalid format. Make sure that the next token you provided is the token returned by a previous call.</p>
    InvalidNextToken(String),
}

impl ListWebhooksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWebhooksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListWebhooksError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListWebhooksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWebhooksError {
    fn description(&self) -> &str {
        match *self {
            ListWebhooksError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by PollForJobs
#[derive(Debug, PartialEq)]
pub enum PollForJobsError {
    /// <p>The specified action type cannot be found.</p>
    ActionTypeNotFound(String),
}

impl PollForJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PollForJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActionTypeNotFoundException" => {
                    return RusotoError::Service(PollForJobsError::ActionTypeNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PollForJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PollForJobsError {
    fn description(&self) -> &str {
        match *self {
            PollForJobsError::ActionTypeNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PollForThirdPartyJobs
#[derive(Debug, PartialEq)]
pub enum PollForThirdPartyJobsError {
    /// <p>The specified action type cannot be found.</p>
    ActionTypeNotFound(String),
}

impl PollForThirdPartyJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PollForThirdPartyJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActionTypeNotFoundException" => {
                    return RusotoError::Service(PollForThirdPartyJobsError::ActionTypeNotFound(
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
impl fmt::Display for PollForThirdPartyJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PollForThirdPartyJobsError {
    fn description(&self) -> &str {
        match *self {
            PollForThirdPartyJobsError::ActionTypeNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutActionRevision
#[derive(Debug, PartialEq)]
pub enum PutActionRevisionError {
    /// <p>The specified action cannot be found.</p>
    ActionNotFound(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified stage was specified in an invalid format or cannot be found.</p>
    StageNotFound(String),
}

impl PutActionRevisionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutActionRevisionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActionNotFoundException" => {
                    return RusotoError::Service(PutActionRevisionError::ActionNotFound(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(PutActionRevisionError::PipelineNotFound(err.msg))
                }
                "StageNotFoundException" => {
                    return RusotoError::Service(PutActionRevisionError::StageNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutActionRevisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutActionRevisionError {
    fn description(&self) -> &str {
        match *self {
            PutActionRevisionError::ActionNotFound(ref cause) => cause,
            PutActionRevisionError::PipelineNotFound(ref cause) => cause,
            PutActionRevisionError::StageNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutApprovalResult
#[derive(Debug, PartialEq)]
pub enum PutApprovalResultError {
    /// <p>The specified action cannot be found.</p>
    ActionNotFound(String),
    /// <p>The approval action has already been approved or rejected.</p>
    ApprovalAlreadyCompleted(String),
    /// <p>The approval request already received a response or has expired.</p>
    InvalidApprovalToken(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified stage was specified in an invalid format or cannot be found.</p>
    StageNotFound(String),
}

impl PutApprovalResultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutApprovalResultError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActionNotFoundException" => {
                    return RusotoError::Service(PutApprovalResultError::ActionNotFound(err.msg))
                }
                "ApprovalAlreadyCompletedException" => {
                    return RusotoError::Service(PutApprovalResultError::ApprovalAlreadyCompleted(
                        err.msg,
                    ))
                }
                "InvalidApprovalTokenException" => {
                    return RusotoError::Service(PutApprovalResultError::InvalidApprovalToken(
                        err.msg,
                    ))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(PutApprovalResultError::PipelineNotFound(err.msg))
                }
                "StageNotFoundException" => {
                    return RusotoError::Service(PutApprovalResultError::StageNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutApprovalResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutApprovalResultError {
    fn description(&self) -> &str {
        match *self {
            PutApprovalResultError::ActionNotFound(ref cause) => cause,
            PutApprovalResultError::ApprovalAlreadyCompleted(ref cause) => cause,
            PutApprovalResultError::InvalidApprovalToken(ref cause) => cause,
            PutApprovalResultError::PipelineNotFound(ref cause) => cause,
            PutApprovalResultError::StageNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutJobFailureResult
#[derive(Debug, PartialEq)]
pub enum PutJobFailureResultError {
    /// <p>The specified job state was specified in an invalid format.</p>
    InvalidJobState(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
}

impl PutJobFailureResultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutJobFailureResultError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidJobStateException" => {
                    return RusotoError::Service(PutJobFailureResultError::InvalidJobState(err.msg))
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(PutJobFailureResultError::JobNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutJobFailureResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutJobFailureResultError {
    fn description(&self) -> &str {
        match *self {
            PutJobFailureResultError::InvalidJobState(ref cause) => cause,
            PutJobFailureResultError::JobNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutJobSuccessResult
#[derive(Debug, PartialEq)]
pub enum PutJobSuccessResultError {
    /// <p>The specified job state was specified in an invalid format.</p>
    InvalidJobState(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
}

impl PutJobSuccessResultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutJobSuccessResultError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidJobStateException" => {
                    return RusotoError::Service(PutJobSuccessResultError::InvalidJobState(err.msg))
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(PutJobSuccessResultError::JobNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutJobSuccessResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutJobSuccessResultError {
    fn description(&self) -> &str {
        match *self {
            PutJobSuccessResultError::InvalidJobState(ref cause) => cause,
            PutJobSuccessResultError::JobNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutThirdPartyJobFailureResult
#[derive(Debug, PartialEq)]
pub enum PutThirdPartyJobFailureResultError {
    /// <p>The client token was specified in an invalid format</p>
    InvalidClientToken(String),
    /// <p>The specified job state was specified in an invalid format.</p>
    InvalidJobState(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
}

impl PutThirdPartyJobFailureResultError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutThirdPartyJobFailureResultError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidClientTokenException" => {
                    return RusotoError::Service(
                        PutThirdPartyJobFailureResultError::InvalidClientToken(err.msg),
                    )
                }
                "InvalidJobStateException" => {
                    return RusotoError::Service(
                        PutThirdPartyJobFailureResultError::InvalidJobState(err.msg),
                    )
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(PutThirdPartyJobFailureResultError::JobNotFound(
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
impl fmt::Display for PutThirdPartyJobFailureResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutThirdPartyJobFailureResultError {
    fn description(&self) -> &str {
        match *self {
            PutThirdPartyJobFailureResultError::InvalidClientToken(ref cause) => cause,
            PutThirdPartyJobFailureResultError::InvalidJobState(ref cause) => cause,
            PutThirdPartyJobFailureResultError::JobNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutThirdPartyJobSuccessResult
#[derive(Debug, PartialEq)]
pub enum PutThirdPartyJobSuccessResultError {
    /// <p>The client token was specified in an invalid format</p>
    InvalidClientToken(String),
    /// <p>The specified job state was specified in an invalid format.</p>
    InvalidJobState(String),
    /// <p>The specified job was specified in an invalid format or cannot be found.</p>
    JobNotFound(String),
}

impl PutThirdPartyJobSuccessResultError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutThirdPartyJobSuccessResultError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidClientTokenException" => {
                    return RusotoError::Service(
                        PutThirdPartyJobSuccessResultError::InvalidClientToken(err.msg),
                    )
                }
                "InvalidJobStateException" => {
                    return RusotoError::Service(
                        PutThirdPartyJobSuccessResultError::InvalidJobState(err.msg),
                    )
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(PutThirdPartyJobSuccessResultError::JobNotFound(
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
impl fmt::Display for PutThirdPartyJobSuccessResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutThirdPartyJobSuccessResultError {
    fn description(&self) -> &str {
        match *self {
            PutThirdPartyJobSuccessResultError::InvalidClientToken(ref cause) => cause,
            PutThirdPartyJobSuccessResultError::InvalidJobState(ref cause) => cause,
            PutThirdPartyJobSuccessResultError::JobNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutWebhook
#[derive(Debug, PartialEq)]
pub enum PutWebhookError {
    /// <p>Unable to modify the tag due to a simultaneous update request.</p>
    ConcurrentModification(String),
    /// <p>The specified resource tags are invalid.</p>
    InvalidTags(String),
    /// <p>The specified authentication type is in an invalid format.</p>
    InvalidWebhookAuthenticationParameters(String),
    /// <p>The specified event filter rule is in an invalid format.</p>
    InvalidWebhookFilterPattern(String),
    /// <p>The number of pipelines associated with the AWS account has exceeded the limit allowed for the account.</p>
    LimitExceeded(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The tags limit for a resource has been exceeded.</p>
    TooManyTags(String),
}

impl PutWebhookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutWebhookError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(PutWebhookError::ConcurrentModification(err.msg))
                }
                "InvalidTagsException" => {
                    return RusotoError::Service(PutWebhookError::InvalidTags(err.msg))
                }
                "InvalidWebhookAuthenticationParametersException" => {
                    return RusotoError::Service(
                        PutWebhookError::InvalidWebhookAuthenticationParameters(err.msg),
                    )
                }
                "InvalidWebhookFilterPatternException" => {
                    return RusotoError::Service(PutWebhookError::InvalidWebhookFilterPattern(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutWebhookError::LimitExceeded(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(PutWebhookError::PipelineNotFound(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(PutWebhookError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutWebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutWebhookError {
    fn description(&self) -> &str {
        match *self {
            PutWebhookError::ConcurrentModification(ref cause) => cause,
            PutWebhookError::InvalidTags(ref cause) => cause,
            PutWebhookError::InvalidWebhookAuthenticationParameters(ref cause) => cause,
            PutWebhookError::InvalidWebhookFilterPattern(ref cause) => cause,
            PutWebhookError::LimitExceeded(ref cause) => cause,
            PutWebhookError::PipelineNotFound(ref cause) => cause,
            PutWebhookError::TooManyTags(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterWebhookWithThirdParty
#[derive(Debug, PartialEq)]
pub enum RegisterWebhookWithThirdPartyError {
    /// <p>The specified webhook was entered in an invalid format or cannot be found.</p>
    WebhookNotFound(String),
}

impl RegisterWebhookWithThirdPartyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RegisterWebhookWithThirdPartyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WebhookNotFoundException" => {
                    return RusotoError::Service(
                        RegisterWebhookWithThirdPartyError::WebhookNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterWebhookWithThirdPartyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterWebhookWithThirdPartyError {
    fn description(&self) -> &str {
        match *self {
            RegisterWebhookWithThirdPartyError::WebhookNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by RetryStageExecution
#[derive(Debug, PartialEq)]
pub enum RetryStageExecutionError {
    /// <p>The stage has failed in a later run of the pipeline and the pipelineExecutionId associated with the request is out of date.</p>
    NotLatestPipelineExecution(String),
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
    /// <p>The specified stage was specified in an invalid format or cannot be found.</p>
    StageNotFound(String),
    /// <p>The specified stage can't be retried because the pipeline structure or stage state changed after the stage was not completed; the stage contains no failed actions; one or more actions are still in progress; or another retry attempt is already in progress. </p>
    StageNotRetryable(String),
}

impl RetryStageExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RetryStageExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotLatestPipelineExecutionException" => {
                    return RusotoError::Service(
                        RetryStageExecutionError::NotLatestPipelineExecution(err.msg),
                    )
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(RetryStageExecutionError::PipelineNotFound(
                        err.msg,
                    ))
                }
                "StageNotFoundException" => {
                    return RusotoError::Service(RetryStageExecutionError::StageNotFound(err.msg))
                }
                "StageNotRetryableException" => {
                    return RusotoError::Service(RetryStageExecutionError::StageNotRetryable(
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
impl fmt::Display for RetryStageExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RetryStageExecutionError {
    fn description(&self) -> &str {
        match *self {
            RetryStageExecutionError::NotLatestPipelineExecution(ref cause) => cause,
            RetryStageExecutionError::PipelineNotFound(ref cause) => cause,
            RetryStageExecutionError::StageNotFound(ref cause) => cause,
            RetryStageExecutionError::StageNotRetryable(ref cause) => cause,
        }
    }
}
/// Errors returned by StartPipelineExecution
#[derive(Debug, PartialEq)]
pub enum StartPipelineExecutionError {
    /// <p>The specified pipeline was specified in an invalid format or cannot be found.</p>
    PipelineNotFound(String),
}

impl StartPipelineExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartPipelineExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "PipelineNotFoundException" => {
                    return RusotoError::Service(StartPipelineExecutionError::PipelineNotFound(
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
impl fmt::Display for StartPipelineExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartPipelineExecutionError {
    fn description(&self) -> &str {
        match *self {
            StartPipelineExecutionError::PipelineNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Unable to modify the tag due to a simultaneous update request.</p>
    ConcurrentModification(String),
    /// <p>The specified resource ARN is invalid.</p>
    InvalidArn(String),
    /// <p>The specified resource tags are invalid.</p>
    InvalidTags(String),
    /// <p>The specified resource was specified in an invalid format.</p>
    ResourceNotFound(String),
    /// <p>The tags limit for a resource has been exceeded.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(TagResourceError::ConcurrentModification(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(TagResourceError::InvalidArn(err.msg))
                }
                "InvalidTagsException" => {
                    return RusotoError::Service(TagResourceError::InvalidTags(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(TagResourceError::TooManyTags(err.msg))
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
            TagResourceError::ConcurrentModification(ref cause) => cause,
            TagResourceError::InvalidArn(ref cause) => cause,
            TagResourceError::InvalidTags(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::TooManyTags(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Unable to modify the tag due to a simultaneous update request.</p>
    ConcurrentModification(String),
    /// <p>The specified resource ARN is invalid.</p>
    InvalidArn(String),
    /// <p>The specified resource tags are invalid.</p>
    InvalidTags(String),
    /// <p>The specified resource was specified in an invalid format.</p>
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
                "InvalidArnException" => {
                    return RusotoError::Service(UntagResourceError::InvalidArn(err.msg))
                }
                "InvalidTagsException" => {
                    return RusotoError::Service(UntagResourceError::InvalidTags(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::ConcurrentModification(ref cause) => cause,
            UntagResourceError::InvalidArn(ref cause) => cause,
            UntagResourceError::InvalidTags(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePipeline
#[derive(Debug, PartialEq)]
pub enum UpdatePipelineError {
    /// <p>The specified action declaration was specified in an invalid format.</p>
    InvalidActionDeclaration(String),
    /// <p>Reserved for future use.</p>
    InvalidBlockerDeclaration(String),
    /// <p>The specified stage declaration was specified in an invalid format.</p>
    InvalidStageDeclaration(String),
    /// <p>The specified structure was specified in an invalid format.</p>
    InvalidStructure(String),
    /// <p>The number of pipelines associated with the AWS account has exceeded the limit allowed for the account.</p>
    LimitExceeded(String),
}

impl UpdatePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePipelineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidActionDeclarationException" => {
                    return RusotoError::Service(UpdatePipelineError::InvalidActionDeclaration(
                        err.msg,
                    ))
                }
                "InvalidBlockerDeclarationException" => {
                    return RusotoError::Service(UpdatePipelineError::InvalidBlockerDeclaration(
                        err.msg,
                    ))
                }
                "InvalidStageDeclarationException" => {
                    return RusotoError::Service(UpdatePipelineError::InvalidStageDeclaration(
                        err.msg,
                    ))
                }
                "InvalidStructureException" => {
                    return RusotoError::Service(UpdatePipelineError::InvalidStructure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdatePipelineError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdatePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePipelineError {
    fn description(&self) -> &str {
        match *self {
            UpdatePipelineError::InvalidActionDeclaration(ref cause) => cause,
            UpdatePipelineError::InvalidBlockerDeclaration(ref cause) => cause,
            UpdatePipelineError::InvalidStageDeclaration(ref cause) => cause,
            UpdatePipelineError::InvalidStructure(ref cause) => cause,
            UpdatePipelineError::LimitExceeded(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the CodePipeline API. CodePipeline clients implement this trait.
pub trait CodePipeline {
    /// <p>Returns information about a specified job and whether that job has been received by the job worker. Only used for custom actions.</p>
    fn acknowledge_job(
        &self,
        input: AcknowledgeJobInput,
    ) -> RusotoFuture<AcknowledgeJobOutput, AcknowledgeJobError>;

    /// <p>Confirms a job worker has received the specified job. Only used for partner actions.</p>
    fn acknowledge_third_party_job(
        &self,
        input: AcknowledgeThirdPartyJobInput,
    ) -> RusotoFuture<AcknowledgeThirdPartyJobOutput, AcknowledgeThirdPartyJobError>;

    /// <p>Creates a new custom action that can be used in all pipelines associated with the AWS account. Only used for custom actions.</p>
    fn create_custom_action_type(
        &self,
        input: CreateCustomActionTypeInput,
    ) -> RusotoFuture<CreateCustomActionTypeOutput, CreateCustomActionTypeError>;

    /// <p><p>Creates a pipeline.</p> <note> <p>In the pipeline structure, you must include either <code>artifactStore</code> or <code>artifactStores</code> in your pipeline, but you cannot use both. If you create a cross-region action in your pipeline, you must use <code>artifactStores</code>.</p> </note></p>
    fn create_pipeline(
        &self,
        input: CreatePipelineInput,
    ) -> RusotoFuture<CreatePipelineOutput, CreatePipelineError>;

    /// <p><p>Marks a custom action as deleted. <code>PollForJobs</code> for the custom action will fail after the action is marked for deletion. Only used for custom actions.</p> <important> <p>To re-create a custom action after it has been deleted you must use a string in the version field that has never been used before. This string can be an incremented version number, for example. To restore a deleted custom action, use a JSON file that is identical to the deleted action, including the original string in the version field.</p> </important></p>
    fn delete_custom_action_type(
        &self,
        input: DeleteCustomActionTypeInput,
    ) -> RusotoFuture<(), DeleteCustomActionTypeError>;

    /// <p>Deletes the specified pipeline.</p>
    fn delete_pipeline(&self, input: DeletePipelineInput) -> RusotoFuture<(), DeletePipelineError>;

    /// <p>Deletes a previously created webhook by name. Deleting the webhook stops AWS CodePipeline from starting a pipeline every time an external event occurs. The API will return successfully when trying to delete a webhook that is already deleted. If a deleted webhook is re-created by calling PutWebhook with the same name, it will have a different URL.</p>
    fn delete_webhook(
        &self,
        input: DeleteWebhookInput,
    ) -> RusotoFuture<DeleteWebhookOutput, DeleteWebhookError>;

    /// <p>Removes the connection between the webhook that was created by CodePipeline and the external tool with events to be detected. Currently only supported for webhooks that target an action type of GitHub.</p>
    fn deregister_webhook_with_third_party(
        &self,
        input: DeregisterWebhookWithThirdPartyInput,
    ) -> RusotoFuture<DeregisterWebhookWithThirdPartyOutput, DeregisterWebhookWithThirdPartyError>;

    /// <p>Prevents artifacts in a pipeline from transitioning to the next stage in the pipeline.</p>
    fn disable_stage_transition(
        &self,
        input: DisableStageTransitionInput,
    ) -> RusotoFuture<(), DisableStageTransitionError>;

    /// <p>Enables artifacts in a pipeline to transition to a stage in a pipeline.</p>
    fn enable_stage_transition(
        &self,
        input: EnableStageTransitionInput,
    ) -> RusotoFuture<(), EnableStageTransitionError>;

    /// <p><p>Returns information about a job. Only used for custom actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn get_job_details(
        &self,
        input: GetJobDetailsInput,
    ) -> RusotoFuture<GetJobDetailsOutput, GetJobDetailsError>;

    /// <p>Returns the metadata, structure, stages, and actions of a pipeline. Can be used to return the entire structure of a pipeline in JSON format, which can then be modified and used to update the pipeline structure with <a>UpdatePipeline</a>.</p>
    fn get_pipeline(
        &self,
        input: GetPipelineInput,
    ) -> RusotoFuture<GetPipelineOutput, GetPipelineError>;

    /// <p>Returns information about an execution of a pipeline, including details about artifacts, the pipeline execution ID, and the name, version, and status of the pipeline.</p>
    fn get_pipeline_execution(
        &self,
        input: GetPipelineExecutionInput,
    ) -> RusotoFuture<GetPipelineExecutionOutput, GetPipelineExecutionError>;

    /// <p><p>Returns information about the state of a pipeline, including the stages and actions.</p> <note> <p>Values returned in the <code>revisionId</code> and <code>revisionUrl</code> fields indicate the source revision information, such as the commit ID, for the current state.</p> </note></p>
    fn get_pipeline_state(
        &self,
        input: GetPipelineStateInput,
    ) -> RusotoFuture<GetPipelineStateOutput, GetPipelineStateError>;

    /// <p><p>Requests the details of a job for a third party action. Only used for partner actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn get_third_party_job_details(
        &self,
        input: GetThirdPartyJobDetailsInput,
    ) -> RusotoFuture<GetThirdPartyJobDetailsOutput, GetThirdPartyJobDetailsError>;

    /// <p>Lists the action executions that have occurred in a pipeline.</p>
    fn list_action_executions(
        &self,
        input: ListActionExecutionsInput,
    ) -> RusotoFuture<ListActionExecutionsOutput, ListActionExecutionsError>;

    /// <p>Gets a summary of all AWS CodePipeline action types associated with your account.</p>
    fn list_action_types(
        &self,
        input: ListActionTypesInput,
    ) -> RusotoFuture<ListActionTypesOutput, ListActionTypesError>;

    /// <p>Gets a summary of the most recent executions for a pipeline.</p>
    fn list_pipeline_executions(
        &self,
        input: ListPipelineExecutionsInput,
    ) -> RusotoFuture<ListPipelineExecutionsOutput, ListPipelineExecutionsError>;

    /// <p>Gets a summary of all of the pipelines associated with your account.</p>
    fn list_pipelines(
        &self,
        input: ListPipelinesInput,
    ) -> RusotoFuture<ListPipelinesOutput, ListPipelinesError>;

    /// <p>Gets the set of key/value pairs (metadata) that are used to manage the resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> RusotoFuture<ListTagsForResourceOutput, ListTagsForResourceError>;

    /// <p>Gets a listing of all the webhooks in this region for this account. The output lists all webhooks and includes the webhook URL and ARN, as well the configuration for each webhook.</p>
    fn list_webhooks(
        &self,
        input: ListWebhooksInput,
    ) -> RusotoFuture<ListWebhooksOutput, ListWebhooksError>;

    /// <p><p>Returns information about any jobs for AWS CodePipeline to act upon. <code>PollForJobs</code> is only valid for action types with &quot;Custom&quot; in the owner field. If the action type contains &quot;AWS&quot; or &quot;ThirdParty&quot; in the owner field, the <code>PollForJobs</code> action returns an error.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn poll_for_jobs(
        &self,
        input: PollForJobsInput,
    ) -> RusotoFuture<PollForJobsOutput, PollForJobsError>;

    /// <p><p>Determines whether there are any third party jobs for a job worker to act on. Only used for partner actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts.</p> </important></p>
    fn poll_for_third_party_jobs(
        &self,
        input: PollForThirdPartyJobsInput,
    ) -> RusotoFuture<PollForThirdPartyJobsOutput, PollForThirdPartyJobsError>;

    /// <p>Provides information to AWS CodePipeline about new revisions to a source.</p>
    fn put_action_revision(
        &self,
        input: PutActionRevisionInput,
    ) -> RusotoFuture<PutActionRevisionOutput, PutActionRevisionError>;

    /// <p>Provides the response to a manual approval request to AWS CodePipeline. Valid responses include Approved and Rejected.</p>
    fn put_approval_result(
        &self,
        input: PutApprovalResultInput,
    ) -> RusotoFuture<PutApprovalResultOutput, PutApprovalResultError>;

    /// <p>Represents the failure of a job as returned to the pipeline by a job worker. Only used for custom actions.</p>
    fn put_job_failure_result(
        &self,
        input: PutJobFailureResultInput,
    ) -> RusotoFuture<(), PutJobFailureResultError>;

    /// <p>Represents the success of a job as returned to the pipeline by a job worker. Only used for custom actions.</p>
    fn put_job_success_result(
        &self,
        input: PutJobSuccessResultInput,
    ) -> RusotoFuture<(), PutJobSuccessResultError>;

    /// <p>Represents the failure of a third party job as returned to the pipeline by a job worker. Only used for partner actions.</p>
    fn put_third_party_job_failure_result(
        &self,
        input: PutThirdPartyJobFailureResultInput,
    ) -> RusotoFuture<(), PutThirdPartyJobFailureResultError>;

    /// <p>Represents the success of a third party job as returned to the pipeline by a job worker. Only used for partner actions.</p>
    fn put_third_party_job_success_result(
        &self,
        input: PutThirdPartyJobSuccessResultInput,
    ) -> RusotoFuture<(), PutThirdPartyJobSuccessResultError>;

    /// <p>Defines a webhook and returns a unique webhook URL generated by CodePipeline. This URL can be supplied to third party source hosting providers to call every time there's a code change. When CodePipeline receives a POST request on this URL, the pipeline defined in the webhook is started as long as the POST request satisfied the authentication and filtering requirements supplied when defining the webhook. RegisterWebhookWithThirdParty and DeregisterWebhookWithThirdParty APIs can be used to automatically configure supported third parties to call the generated webhook URL.</p>
    fn put_webhook(
        &self,
        input: PutWebhookInput,
    ) -> RusotoFuture<PutWebhookOutput, PutWebhookError>;

    /// <p>Configures a connection between the webhook that was created and the external tool with events to be detected.</p>
    fn register_webhook_with_third_party(
        &self,
        input: RegisterWebhookWithThirdPartyInput,
    ) -> RusotoFuture<RegisterWebhookWithThirdPartyOutput, RegisterWebhookWithThirdPartyError>;

    /// <p>Resumes the pipeline execution by retrying the last failed actions in a stage.</p>
    fn retry_stage_execution(
        &self,
        input: RetryStageExecutionInput,
    ) -> RusotoFuture<RetryStageExecutionOutput, RetryStageExecutionError>;

    /// <p>Starts the specified pipeline. Specifically, it begins processing the latest commit to the source location specified as part of the pipeline.</p>
    fn start_pipeline_execution(
        &self,
        input: StartPipelineExecutionInput,
    ) -> RusotoFuture<StartPipelineExecutionOutput, StartPipelineExecutionError>;

    /// <p>Adds to or modifies the tags of the given resource. Tags are metadata that can be used to manage a resource. </p>
    fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> RusotoFuture<TagResourceOutput, TagResourceError>;

    /// <p>Removes tags from an AWS resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> RusotoFuture<UntagResourceOutput, UntagResourceError>;

    /// <p>Updates a specified pipeline with edits or changes to its structure. Use a JSON file with the pipeline structure in conjunction with <code>UpdatePipeline</code> to provide the full structure of the pipeline. Updating the pipeline increases the version number of the pipeline by 1.</p>
    fn update_pipeline(
        &self,
        input: UpdatePipelineInput,
    ) -> RusotoFuture<UpdatePipelineOutput, UpdatePipelineError>;
}
/// A client for the CodePipeline API.
#[derive(Clone)]
pub struct CodePipelineClient {
    client: Client,
    region: region::Region,
}

impl CodePipelineClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodePipelineClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodePipelineClient
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

    pub fn new_with_client(client: Client, region: region::Region) -> CodePipelineClient {
        CodePipelineClient { client, region }
    }
}

impl fmt::Debug for CodePipelineClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CodePipelineClient")
            .field("region", &self.region)
            .finish()
    }
}

impl CodePipeline for CodePipelineClient {
    /// <p>Returns information about a specified job and whether that job has been received by the job worker. Only used for custom actions.</p>
    fn acknowledge_job(
        &self,
        input: AcknowledgeJobInput,
    ) -> RusotoFuture<AcknowledgeJobOutput, AcknowledgeJobError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.AcknowledgeJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AcknowledgeJobOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AcknowledgeJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Confirms a job worker has received the specified job. Only used for partner actions.</p>
    fn acknowledge_third_party_job(
        &self,
        input: AcknowledgeThirdPartyJobInput,
    ) -> RusotoFuture<AcknowledgeThirdPartyJobOutput, AcknowledgeThirdPartyJobError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.AcknowledgeThirdPartyJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AcknowledgeThirdPartyJobOutput, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AcknowledgeThirdPartyJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a new custom action that can be used in all pipelines associated with the AWS account. Only used for custom actions.</p>
    fn create_custom_action_type(
        &self,
        input: CreateCustomActionTypeInput,
    ) -> RusotoFuture<CreateCustomActionTypeOutput, CreateCustomActionTypeError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.CreateCustomActionType",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCustomActionTypeOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateCustomActionTypeError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Creates a pipeline.</p> <note> <p>In the pipeline structure, you must include either <code>artifactStore</code> or <code>artifactStores</code> in your pipeline, but you cannot use both. If you create a cross-region action in your pipeline, you must use <code>artifactStores</code>.</p> </note></p>
    fn create_pipeline(
        &self,
        input: CreatePipelineInput,
    ) -> RusotoFuture<CreatePipelineOutput, CreatePipelineError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.CreatePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreatePipelineOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePipelineError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Marks a custom action as deleted. <code>PollForJobs</code> for the custom action will fail after the action is marked for deletion. Only used for custom actions.</p> <important> <p>To re-create a custom action after it has been deleted you must use a string in the version field that has never been used before. This string can be an incremented version number, for example. To restore a deleted custom action, use a JSON file that is identical to the deleted action, including the original string in the version field.</p> </important></p>
    fn delete_custom_action_type(
        &self,
        input: DeleteCustomActionTypeInput,
    ) -> RusotoFuture<(), DeleteCustomActionTypeError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.DeleteCustomActionType",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteCustomActionTypeError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the specified pipeline.</p>
    fn delete_pipeline(&self, input: DeletePipelineInput) -> RusotoFuture<(), DeletePipelineError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.DeletePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePipelineError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a previously created webhook by name. Deleting the webhook stops AWS CodePipeline from starting a pipeline every time an external event occurs. The API will return successfully when trying to delete a webhook that is already deleted. If a deleted webhook is re-created by calling PutWebhook with the same name, it will have a different URL.</p>
    fn delete_webhook(
        &self,
        input: DeleteWebhookInput,
    ) -> RusotoFuture<DeleteWebhookOutput, DeleteWebhookError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.DeleteWebhook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteWebhookOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteWebhookError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes the connection between the webhook that was created by CodePipeline and the external tool with events to be detected. Currently only supported for webhooks that target an action type of GitHub.</p>
    fn deregister_webhook_with_third_party(
        &self,
        input: DeregisterWebhookWithThirdPartyInput,
    ) -> RusotoFuture<DeregisterWebhookWithThirdPartyOutput, DeregisterWebhookWithThirdPartyError>
    {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.DeregisterWebhookWithThirdParty",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeregisterWebhookWithThirdPartyOutput, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterWebhookWithThirdPartyError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Prevents artifacts in a pipeline from transitioning to the next stage in the pipeline.</p>
    fn disable_stage_transition(
        &self,
        input: DisableStageTransitionInput,
    ) -> RusotoFuture<(), DisableStageTransitionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.DisableStageTransition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DisableStageTransitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Enables artifacts in a pipeline to transition to a stage in a pipeline.</p>
    fn enable_stage_transition(
        &self,
        input: EnableStageTransitionInput,
    ) -> RusotoFuture<(), EnableStageTransitionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.EnableStageTransition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(EnableStageTransitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Returns information about a job. Only used for custom actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn get_job_details(
        &self,
        input: GetJobDetailsInput,
    ) -> RusotoFuture<GetJobDetailsOutput, GetJobDetailsError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.GetJobDetails");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetJobDetailsOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobDetailsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the metadata, structure, stages, and actions of a pipeline. Can be used to return the entire structure of a pipeline in JSON format, which can then be modified and used to update the pipeline structure with <a>UpdatePipeline</a>.</p>
    fn get_pipeline(
        &self,
        input: GetPipelineInput,
    ) -> RusotoFuture<GetPipelineOutput, GetPipelineError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.GetPipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPipelineOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPipelineError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about an execution of a pipeline, including details about artifacts, the pipeline execution ID, and the name, version, and status of the pipeline.</p>
    fn get_pipeline_execution(
        &self,
        input: GetPipelineExecutionInput,
    ) -> RusotoFuture<GetPipelineExecutionOutput, GetPipelineExecutionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.GetPipelineExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPipelineExecutionOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetPipelineExecutionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Returns information about the state of a pipeline, including the stages and actions.</p> <note> <p>Values returned in the <code>revisionId</code> and <code>revisionUrl</code> fields indicate the source revision information, such as the commit ID, for the current state.</p> </note></p>
    fn get_pipeline_state(
        &self,
        input: GetPipelineStateInput,
    ) -> RusotoFuture<GetPipelineStateOutput, GetPipelineStateError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.GetPipelineState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPipelineStateOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPipelineStateError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Requests the details of a job for a third party action. Only used for partner actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn get_third_party_job_details(
        &self,
        input: GetThirdPartyJobDetailsInput,
    ) -> RusotoFuture<GetThirdPartyJobDetailsOutput, GetThirdPartyJobDetailsError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.GetThirdPartyJobDetails",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetThirdPartyJobDetailsOutput, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetThirdPartyJobDetailsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the action executions that have occurred in a pipeline.</p>
    fn list_action_executions(
        &self,
        input: ListActionExecutionsInput,
    ) -> RusotoFuture<ListActionExecutionsOutput, ListActionExecutionsError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.ListActionExecutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListActionExecutionsOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListActionExecutionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets a summary of all AWS CodePipeline action types associated with your account.</p>
    fn list_action_types(
        &self,
        input: ListActionTypesInput,
    ) -> RusotoFuture<ListActionTypesOutput, ListActionTypesError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.ListActionTypes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListActionTypesOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListActionTypesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a summary of the most recent executions for a pipeline.</p>
    fn list_pipeline_executions(
        &self,
        input: ListPipelineExecutionsInput,
    ) -> RusotoFuture<ListPipelineExecutionsOutput, ListPipelineExecutionsError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.ListPipelineExecutions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPipelineExecutionsOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListPipelineExecutionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets a summary of all of the pipelines associated with your account.</p>
    fn list_pipelines(
        &self,
        input: ListPipelinesInput,
    ) -> RusotoFuture<ListPipelinesOutput, ListPipelinesError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.ListPipelines");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPipelinesOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPipelinesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the set of key/value pairs (metadata) that are used to manage the resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> RusotoFuture<ListTagsForResourceOutput, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceOutput, _>()
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

    /// <p>Gets a listing of all the webhooks in this region for this account. The output lists all webhooks and includes the webhook URL and ARN, as well the configuration for each webhook.</p>
    fn list_webhooks(
        &self,
        input: ListWebhooksInput,
    ) -> RusotoFuture<ListWebhooksOutput, ListWebhooksError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.ListWebhooks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListWebhooksOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListWebhooksError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Returns information about any jobs for AWS CodePipeline to act upon. <code>PollForJobs</code> is only valid for action types with &quot;Custom&quot; in the owner field. If the action type contains &quot;AWS&quot; or &quot;ThirdParty&quot; in the owner field, the <code>PollForJobs</code> action returns an error.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts. Additionally, this API returns any secret values defined for the action.</p> </important></p>
    fn poll_for_jobs(
        &self,
        input: PollForJobsInput,
    ) -> RusotoFuture<PollForJobsOutput, PollForJobsError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PollForJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PollForJobsOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PollForJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Determines whether there are any third party jobs for a job worker to act on. Only used for partner actions.</p> <important> <p>When this API is called, AWS CodePipeline returns temporary credentials for the Amazon S3 bucket used to store artifacts for the pipeline, if the action requires access to that Amazon S3 bucket for input or output artifacts.</p> </important></p>
    fn poll_for_third_party_jobs(
        &self,
        input: PollForThirdPartyJobsInput,
    ) -> RusotoFuture<PollForThirdPartyJobsOutput, PollForThirdPartyJobsError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.PollForThirdPartyJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PollForThirdPartyJobsOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PollForThirdPartyJobsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Provides information to AWS CodePipeline about new revisions to a source.</p>
    fn put_action_revision(
        &self,
        input: PutActionRevisionInput,
    ) -> RusotoFuture<PutActionRevisionOutput, PutActionRevisionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PutActionRevision");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutActionRevisionOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutActionRevisionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Provides the response to a manual approval request to AWS CodePipeline. Valid responses include Approved and Rejected.</p>
    fn put_approval_result(
        &self,
        input: PutApprovalResultInput,
    ) -> RusotoFuture<PutApprovalResultOutput, PutApprovalResultError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PutApprovalResult");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutApprovalResultOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutApprovalResultError::from_response(response))),
                )
            }
        })
    }

    /// <p>Represents the failure of a job as returned to the pipeline by a job worker. Only used for custom actions.</p>
    fn put_job_failure_result(
        &self,
        input: PutJobFailureResultInput,
    ) -> RusotoFuture<(), PutJobFailureResultError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PutJobFailureResult");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutJobFailureResultError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Represents the success of a job as returned to the pipeline by a job worker. Only used for custom actions.</p>
    fn put_job_success_result(
        &self,
        input: PutJobSuccessResultInput,
    ) -> RusotoFuture<(), PutJobSuccessResultError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PutJobSuccessResult");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutJobSuccessResultError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Represents the failure of a third party job as returned to the pipeline by a job worker. Only used for partner actions.</p>
    fn put_third_party_job_failure_result(
        &self,
        input: PutThirdPartyJobFailureResultInput,
    ) -> RusotoFuture<(), PutThirdPartyJobFailureResultError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.PutThirdPartyJobFailureResult",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutThirdPartyJobFailureResultError::from_response(response))
                }))
            }
        })
    }

    /// <p>Represents the success of a third party job as returned to the pipeline by a job worker. Only used for partner actions.</p>
    fn put_third_party_job_success_result(
        &self,
        input: PutThirdPartyJobSuccessResultInput,
    ) -> RusotoFuture<(), PutThirdPartyJobSuccessResultError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.PutThirdPartyJobSuccessResult",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutThirdPartyJobSuccessResultError::from_response(response))
                }))
            }
        })
    }

    /// <p>Defines a webhook and returns a unique webhook URL generated by CodePipeline. This URL can be supplied to third party source hosting providers to call every time there's a code change. When CodePipeline receives a POST request on this URL, the pipeline defined in the webhook is started as long as the POST request satisfied the authentication and filtering requirements supplied when defining the webhook. RegisterWebhookWithThirdParty and DeregisterWebhookWithThirdParty APIs can be used to automatically configure supported third parties to call the generated webhook URL.</p>
    fn put_webhook(
        &self,
        input: PutWebhookInput,
    ) -> RusotoFuture<PutWebhookOutput, PutWebhookError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.PutWebhook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutWebhookOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutWebhookError::from_response(response))),
                )
            }
        })
    }

    /// <p>Configures a connection between the webhook that was created and the external tool with events to be detected.</p>
    fn register_webhook_with_third_party(
        &self,
        input: RegisterWebhookWithThirdPartyInput,
    ) -> RusotoFuture<RegisterWebhookWithThirdPartyOutput, RegisterWebhookWithThirdPartyError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.RegisterWebhookWithThirdParty",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RegisterWebhookWithThirdPartyOutput, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RegisterWebhookWithThirdPartyError::from_response(response))
                }))
            }
        })
    }

    /// <p>Resumes the pipeline execution by retrying the last failed actions in a stage.</p>
    fn retry_stage_execution(
        &self,
        input: RetryStageExecutionInput,
    ) -> RusotoFuture<RetryStageExecutionOutput, RetryStageExecutionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.RetryStageExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RetryStageExecutionOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RetryStageExecutionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts the specified pipeline. Specifically, it begins processing the latest commit to the source location specified as part of the pipeline.</p>
    fn start_pipeline_execution(
        &self,
        input: StartPipelineExecutionInput,
    ) -> RusotoFuture<StartPipelineExecutionOutput, StartPipelineExecutionError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodePipeline_20150709.StartPipelineExecution",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartPipelineExecutionOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartPipelineExecutionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Adds to or modifies the tags of the given resource. Tags are metadata that can be used to manage a resource. </p>
    fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> RusotoFuture<TagResourceOutput, TagResourceError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceOutput, _>()
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

    /// <p>Removes tags from an AWS resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> RusotoFuture<UntagResourceOutput, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceOutput, _>()
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

    /// <p>Updates a specified pipeline with edits or changes to its structure. Use a JSON file with the pipeline structure in conjunction with <code>UpdatePipeline</code> to provide the full structure of the pipeline. Updating the pipeline increases the version number of the pipeline by 1.</p>
    fn update_pipeline(
        &self,
        input: UpdatePipelineInput,
    ) -> RusotoFuture<UpdatePipelineOutput, UpdatePipelineError> {
        let mut request = SignedRequest::new("POST", "codepipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodePipeline_20150709.UpdatePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdatePipelineOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdatePipelineError::from_response(response))),
                )
            }
        })
    }
}
