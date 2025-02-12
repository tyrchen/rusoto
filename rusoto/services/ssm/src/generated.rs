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
/// <p>An activation registers one or more on-premises servers or virtual machines (VMs) with AWS so that you can configure those servers or VMs using Run Command. A server or VM that has been registered with AWS is called a managed instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Activation {
    /// <p>The ID created by Systems Manager when you submitted the activation.</p>
    #[serde(rename = "ActivationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
    /// <p>The date the activation was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A name for the managed instance when it is created.</p>
    #[serde(rename = "DefaultInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_name: Option<String>,
    /// <p>A user defined description of the activation.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date when this activation can no longer be used to register managed instances.</p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p>Whether or not the activation is expired.</p>
    #[serde(rename = "Expired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    /// <p>The Amazon Identity and Access Management (IAM) role to assign to the managed instance.</p>
    #[serde(rename = "IamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    /// <p>The maximum number of managed instances that can be registered using this activation.</p>
    #[serde(rename = "RegistrationLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_limit: Option<i64>,
    /// <p>The number of managed instances already registered with this activation.</p>
    #[serde(rename = "RegistrationsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrations_count: Option<i64>,
    /// <p>Tags assigned to the activation.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddTagsToResourceRequest {
    /// <p><p>The resource ID you want to tag.</p> <p>Use the ID of the resource. Here are some examples:</p> <p>ManagedInstance: mi-012345abcde</p> <p>MaintenanceWindow: mw-012345abcde</p> <p>PatchBaseline: pb-012345abcde</p> <p>For the Document and Parameter values, use the name of the resource.</p> <note> <p>The ManagedInstance type for this API action is only for on-premises managed instances. You must specify the name of the managed instance in the following format: mi-ID_number. For example, mi-1a2b3c4d5e6f.</p> </note></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>Specifies the type of resource you are tagging.</p> <note> <p>The ManagedInstance type for this API action is for on-premises managed instances. You must specify the name of the managed instance in the following format: mi-ID_number. For example, mi-1a2b3c4d5e6f.</p> </note></p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p><p> One or more tags. The value parameter is required, but if you don&#39;t want the tag to have a value, specify the parameter with no value, and we set the value to an empty string. </p> <important> <p>Do not enter personally identifiable information in this field.</p> </important></p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddTagsToResourceResult {}

/// <p>Describes an association of a Systems Manager document and an instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Association {
    /// <p>The ID created by the system when you create an association. An association is a binding between a document and a set of targets with a schedule.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The association name.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>The association version.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>The version of the document used in the association.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The date on which the association was last run.</p>
    #[serde(rename = "LastExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Information about the association.</p>
    #[serde(rename = "Overview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<AssociationOverview>,
    /// <p>A cron expression that specifies a schedule when the association runs.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The instances targeted by the request to create an association. </p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>Describes the parameters for a document.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociationDescription {
    /// <p>The association ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The association name.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>The association version.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>Specify the target for the association. This target is required for associations that use an Automation document and target resources by using rate controls.</p>
    #[serde(rename = "AutomationTargetParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_target_parameter_name: Option<String>,
    /// <p>The severity level that is assigned to the association.</p>
    #[serde(rename = "ComplianceSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_severity: Option<String>,
    /// <p>The date when the association was made.</p>
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The date on which the association was last run.</p>
    #[serde(rename = "LastExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    /// <p>The last date on which the association was successfully run.</p>
    #[serde(rename = "LastSuccessfulExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_execution_date: Option<f64>,
    /// <p>The date when the association was last updated.</p>
    #[serde(rename = "LastUpdateAssociationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_association_date: Option<f64>,
    /// <p>The maximum number of targets allowed to run the association at the same time. You can specify a number, for example 10, or a percentage of the target set, for example 10%. The default value is 100%, which means all targets run the association at the same time.</p> <p>If a new instance starts and attempts to run an association while Systems Manager is running MaxConcurrency associations, the association is allowed to run. During the next association interval, the new instance will process its association within the limit specified for MaxConcurrency.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The number of errors that are allowed before the system stops sending requests to run the association on additional targets. You can specify either an absolute number of errors, for example 10, or a percentage of the target set, for example 10%. If you specify 3, for example, the system stops sending requests when the fourth error is received. If you specify 0, then the system stops sending requests after the first error is returned. If you run an association on 50 instances and set MaxError to 10%, then the system stops sending the request when the sixth error is received.</p> <p>Executions that are already running an association when MaxErrors is reached are allowed to complete, but some of these executions may fail as well. If you need to ensure that there won't be more than max-errors failed executions, set MaxConcurrency to 1 so that executions proceed one at a time.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An Amazon S3 bucket where you want to store the output details of the request.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    /// <p>Information about the association.</p>
    #[serde(rename = "Overview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<AssociationOverview>,
    /// <p>A description of the parameters for a document. </p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>A cron expression that specifies a schedule when the association runs.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The association status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AssociationStatus>,
    /// <p>The instances targeted by the request. </p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>Includes information about the specified association.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociationExecution {
    /// <p>The association ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The association version.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>The time the execution started.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>Detailed status information about the execution.</p>
    #[serde(rename = "DetailedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    /// <p>The execution ID for the association.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// <p>The date of the last execution.</p>
    #[serde(rename = "LastExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    /// <p>An aggregate status of the resources in the execution based on the status type.</p>
    #[serde(rename = "ResourceCountByStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_count_by_status: Option<String>,
    /// <p>The status of the association execution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Filters used in the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociationExecutionFilter {
    /// <p>The key value used in the request.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The filter type specified in the request.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The value specified for the key.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Includes information about the specified association execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociationExecutionTarget {
    /// <p>The association ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The association version.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>Detailed information about the execution status.</p>
    #[serde(rename = "DetailedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    /// <p>The execution ID.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// <p>The date of the last execution.</p>
    #[serde(rename = "LastExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    /// <p>The location where the association details are saved.</p>
    #[serde(rename = "OutputSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_source: Option<OutputSource>,
    /// <p>The resource ID, for example, the instance ID where the association ran.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The resource type, for example, instance.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The association execution status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Filters for the association execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociationExecutionTargetsFilter {
    /// <p>The key value used in the request.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value specified for the key.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Describes a filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociationFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The filter value.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>Information about the association.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociationOverview {
    /// <p>Returns the number of targets for the association status. For example, if you created an association with two instances, and one of them was successful, this would return the count of instances by status.</p>
    #[serde(rename = "AssociationStatusAggregatedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status_aggregated_count: Option<::std::collections::HashMap<String, i64>>,
    /// <p>A detailed status of the association.</p>
    #[serde(rename = "DetailedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    /// <p>The status of the association. Status can be: Pending, Success, or Failed.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes an association status.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssociationStatus {
    /// <p>A user-defined string.</p>
    #[serde(rename = "AdditionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    /// <p>The date when the status changed.</p>
    #[serde(rename = "Date")]
    pub date: f64,
    /// <p>The reason for the status.</p>
    #[serde(rename = "Message")]
    pub message: String,
    /// <p>The status.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Information about the association version.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociationVersionInfo {
    /// <p>The ID created by the system when the association was created.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The name specified for the association version when the association version was created.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>The association version.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>The severity level that is assigned to the association.</p>
    #[serde(rename = "ComplianceSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_severity: Option<String>,
    /// <p>The date the association version was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The version of a Systems Manager document used when the association version was created.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The maximum number of targets allowed to run the association at the same time. You can specify a number, for example 10, or a percentage of the target set, for example 10%. The default value is 100%, which means all targets run the association at the same time.</p> <p>If a new instance starts and attempts to run an association while Systems Manager is running MaxConcurrency associations, the association is allowed to run. During the next association interval, the new instance will process its association within the limit specified for MaxConcurrency.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The number of errors that are allowed before the system stops sending requests to run the association on additional targets. You can specify either an absolute number of errors, for example 10, or a percentage of the target set, for example 10%. If you specify 3, for example, the system stops sending requests when the fourth error is received. If you specify 0, then the system stops sending requests after the first error is returned. If you run an association on 50 instances and set MaxError to 10%, then the system stops sending the request when the sixth error is received.</p> <p>Executions that are already running an association when MaxErrors is reached are allowed to complete, but some of these executions may fail as well. If you need to ensure that there won't be more than max-errors failed executions, set MaxConcurrency to 1 so that executions proceed one at a time.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The name specified when the association was created.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The location in Amazon S3 specified for the association when the association version was created.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    /// <p>Parameters specified when the association version was created.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The cron or rate schedule specified for the association when the association version was created.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The targets specified for the association when the association version was created. </p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>A structure that includes attributes that describe a document attachment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachmentContent {
    /// <p>The cryptographic hash value of the document content.</p>
    #[serde(rename = "Hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// <p>The hash algorithm used to calculate the hash value.</p>
    #[serde(rename = "HashType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_type: Option<String>,
    /// <p>The name of an attachment.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The size of an attachment in bytes.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// <p>The URL location of the attachment content.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>An attribute of an attachment, such as the attachment name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachmentInformation {
    /// <p>The name of the attachment.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A key and value pair that identifies the location of an attachment to a document.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachmentsSource {
    /// <p>The key of a key and value pair that identifies the location of an attachment to a document.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The URL of the location of a document attachment, such as the URL of an Amazon S3 bucket.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Detailed information about the current state of an individual Automation execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutomationExecution {
    /// <p>The execution ID.</p>
    #[serde(rename = "AutomationExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_id: Option<String>,
    /// <p>The execution status of the Automation.</p>
    #[serde(rename = "AutomationExecutionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_status: Option<String>,
    /// <p>The action of the step that is currently running.</p>
    #[serde(rename = "CurrentAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_action: Option<String>,
    /// <p>The name of the step that is currently running.</p>
    #[serde(rename = "CurrentStepName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_step_name: Option<String>,
    /// <p>The name of the Automation document used during the execution.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>The version of the document to use during execution.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the user who ran the automation.</p>
    #[serde(rename = "ExecutedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_by: Option<String>,
    /// <p>The time the execution finished.</p>
    #[serde(rename = "ExecutionEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_time: Option<f64>,
    /// <p>The time the execution started.</p>
    #[serde(rename = "ExecutionStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_time: Option<f64>,
    /// <p>A message describing why an execution has failed, if the status is set to Failed.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// <p>The MaxConcurrency value specified by the user when the execution started.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The MaxErrors value specified by the user when the execution started.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The automation execution mode.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>The list of execution outputs as defined in the automation document.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The key-value map of execution parameters, which were supplied when calling StartAutomationExecution.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The AutomationExecutionId of the parent automation.</p>
    #[serde(rename = "ParentAutomationExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_automation_execution_id: Option<String>,
    /// <p>An aggregate of step execution statuses displayed in the AWS Console for a multi-Region and multi-account Automation execution.</p>
    #[serde(rename = "ProgressCounters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_counters: Option<ProgressCounters>,
    /// <p>A list of resolved targets in the rate control execution.</p>
    #[serde(rename = "ResolvedTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_targets: Option<ResolvedTargets>,
    /// <p>A list of details about the current state of all steps that comprise an execution. An Automation document contains a list of steps that are run in order.</p>
    #[serde(rename = "StepExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_executions: Option<Vec<StepExecution>>,
    /// <p>A boolean value that indicates if the response contains the full list of the Automation step executions. If true, use the DescribeAutomationStepExecutions API action to get the full list of step executions.</p>
    #[serde(rename = "StepExecutionsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_executions_truncated: Option<bool>,
    /// <p>The target of the execution.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// <p>The combination of AWS Regions and/or AWS accounts where you want to run the Automation.</p>
    #[serde(rename = "TargetLocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations: Option<Vec<TargetLocation>>,
    /// <p>The specified key-value mapping of document parameters to target resources.</p>
    #[serde(rename = "TargetMaps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<::std::collections::HashMap<String, Vec<String>>>>,
    /// <p>The parameter name.</p>
    #[serde(rename = "TargetParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter_name: Option<String>,
    /// <p>The specified targets.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>A filter used to match specific automation executions. This is used to limit the scope of Automation execution information returned.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AutomationExecutionFilter {
    /// <p>One or more keys to limit the results. Valid filter keys include the following: DocumentNamePrefix, ExecutionStatus, ExecutionId, ParentExecutionId, CurrentAction, StartTimeBefore, StartTimeAfter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The values used to limit the execution information associated with the filter's key.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Details about a specific Automation execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutomationExecutionMetadata {
    /// <p>The execution ID.</p>
    #[serde(rename = "AutomationExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_id: Option<String>,
    /// <p>The status of the execution. Valid values include: Running, Succeeded, Failed, Timed out, or Cancelled.</p>
    #[serde(rename = "AutomationExecutionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_status: Option<String>,
    /// <p>Use this filter with <a>DescribeAutomationExecutions</a>. Specify either Local or CrossAccount. CrossAccount is an Automation that runs in multiple AWS Regions and accounts. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-automation-multiple-accounts-and-regions.html">Executing Automations in Multiple AWS Regions and Accounts</a> in the <i>AWS Systems Manager User Guide</i>. </p>
    #[serde(rename = "AutomationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_type: Option<String>,
    /// <p>The action of the step that is currently running.</p>
    #[serde(rename = "CurrentAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_action: Option<String>,
    /// <p>The name of the step that is currently running.</p>
    #[serde(rename = "CurrentStepName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_step_name: Option<String>,
    /// <p>The name of the Automation document used during execution.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>The document version used during the execution.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The IAM role ARN of the user who ran the Automation.</p>
    #[serde(rename = "ExecutedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_by: Option<String>,
    /// <p>The time the execution finished. This is not populated if the execution is still in progress.</p>
    #[serde(rename = "ExecutionEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_time: Option<f64>,
    /// <p>The time the execution started.&gt;</p>
    #[serde(rename = "ExecutionStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_time: Option<f64>,
    /// <p>The list of execution outputs as defined in the Automation document.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// <p>An Amazon S3 bucket where execution information is stored.</p>
    #[serde(rename = "LogFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file: Option<String>,
    /// <p>The MaxConcurrency value specified by the user when starting the Automation.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The MaxErrors value specified by the user when starting the Automation.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The Automation execution mode.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>The list of execution outputs as defined in the Automation document.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The ExecutionId of the parent Automation.</p>
    #[serde(rename = "ParentAutomationExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_automation_execution_id: Option<String>,
    /// <p>A list of targets that resolved during the execution.</p>
    #[serde(rename = "ResolvedTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_targets: Option<ResolvedTargets>,
    /// <p>The list of execution outputs as defined in the Automation document.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// <p>The specified key-value mapping of document parameters to target resources.</p>
    #[serde(rename = "TargetMaps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<::std::collections::HashMap<String, Vec<String>>>>,
    /// <p>The list of execution outputs as defined in the Automation document.</p>
    #[serde(rename = "TargetParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter_name: Option<String>,
    /// <p>The targets defined by the user when starting the Automation.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelCommandRequest {
    /// <p>The ID of the command you want to cancel.</p>
    #[serde(rename = "CommandId")]
    pub command_id: String,
    /// <p>(Optional) A list of instance IDs on which you want to cancel the command. If not provided, the command is canceled on every instance on which it was requested.</p>
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

/// <p>Whether or not the command was successfully canceled. There is no guarantee that a request can be canceled.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelCommandResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelMaintenanceWindowExecutionRequest {
    /// <p>The ID of the maintenance window execution to stop.</p>
    #[serde(rename = "WindowExecutionId")]
    pub window_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelMaintenanceWindowExecutionResult {
    /// <p>The ID of the maintenance window execution that has been stopped.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
}

/// <p>Configuration options for sending command output to CloudWatch Logs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudWatchOutputConfig {
    /// <p>The name of the CloudWatch log group where you want to send command output. If you don't specify a group name, Systems Manager automatically creates a log group for you. The log group uses the following naming format: aws/ssm/<i>SystemsManagerDocumentName</i>.</p>
    #[serde(rename = "CloudWatchLogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_name: Option<String>,
    /// <p>Enables Systems Manager to send command output to CloudWatch Logs.</p>
    #[serde(rename = "CloudWatchOutputEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_enabled: Option<bool>,
}

/// <p>Describes a command request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Command {
    /// <p>CloudWatch Logs information where you want Systems Manager to send the command output.</p>
    #[serde(rename = "CloudWatchOutputConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_config: Option<CloudWatchOutputConfig>,
    /// <p>A unique identifier for this command.</p>
    #[serde(rename = "CommandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// <p>User-specified information about the command, such as a brief description of what the command should do.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The number of targets for which the command invocation reached a terminal state. Terminal states include the following: Success, Failed, Execution Timed Out, Delivery Timed Out, Canceled, Terminated, or Undeliverable.</p>
    #[serde(rename = "CompletedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_count: Option<i64>,
    /// <p>The number of targets for which the status is Delivery Timed Out.</p>
    #[serde(rename = "DeliveryTimedOutCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_timed_out_count: Option<i64>,
    /// <p>The name of the document requested for execution.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>The SSM document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The number of targets for which the status is Failed or Execution Timed Out.</p>
    #[serde(rename = "ErrorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i64>,
    /// <p>If this time is reached and the command has not already started running, it will not run. Calculated based on the ExpiresAfter user input provided as part of the SendCommand API.</p>
    #[serde(rename = "ExpiresAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<f64>,
    /// <p>The instance IDs against which this command was requested.</p>
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// <p>The maximum number of instances that are allowed to run the command at the same time. You can specify a number of instances, such as 10, or a percentage of instances, such as 10%. The default value is 50. For more information about how to use MaxConcurrency, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/run-command.html">Running Commands Using Systems Manager Run Command</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The maximum number of errors allowed before the system stops sending the command to additional targets. You can specify a number of errors, such as 10, or a percentage or errors, such as 10%. The default value is 0. For more information about how to use MaxErrors, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/run-command.html">Running Commands Using Systems Manager Run Command</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>Configurations for sending notifications about command status changes. </p>
    #[serde(rename = "NotificationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    /// <p>The S3 bucket where the responses to the command executions should be stored. This was requested when issuing the command.</p>
    #[serde(rename = "OutputS3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    /// <p>The S3 directory path inside the bucket where the responses to the command executions should be stored. This was requested when issuing the command.</p>
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    /// <p>(Deprecated) You can no longer specify this parameter. The system ignores it. Instead, Systems Manager automatically determines the Amazon S3 bucket region.</p>
    #[serde(rename = "OutputS3Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
    /// <p>The parameter values to be inserted in the document when running the command.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The date and time the command was requested.</p>
    #[serde(rename = "RequestedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_date_time: Option<f64>,
    /// <p>The IAM service role that Run Command uses to act on your behalf when sending notifications about command status changes. </p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The status of the command.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>A detailed status of the command execution. StatusDetails includes more information than Status because it includes states resulting from error and concurrency control parameters. StatusDetails can show different results than Status. For more information about these statuses, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-commands.html">Understanding Command Statuses</a> in the <i>AWS Systems Manager User Guide</i>. StatusDetails can be one of the following values:</p> <ul> <li> <p>Pending: The command has not been sent to any instances.</p> </li> <li> <p>In Progress: The command has been sent to at least one instance but has not reached a final state on all instances.</p> </li> <li> <p>Success: The command successfully ran on all invocations. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out: The value of MaxErrors or more command invocations shows a status of Delivery Timed Out. This is a terminal state.</p> </li> <li> <p>Execution Timed Out: The value of MaxErrors or more command invocations shows a status of Execution Timed Out. This is a terminal state.</p> </li> <li> <p>Failed: The value of MaxErrors or more command invocations shows a status of Failed. This is a terminal state.</p> </li> <li> <p>Incomplete: The command was attempted on all instances and one or more invocations does not have a value of Success but not enough invocations failed for the status to be Failed. This is a terminal state.</p> </li> <li> <p>Canceled: The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Rate Exceeded: The number of instances targeted by the command exceeded the account limit for pending invocations. The system has canceled the command before running it on any instance. This is a terminal state.</p> </li> </ul></p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The number of targets for the command.</p>
    #[serde(rename = "TargetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_count: Option<i64>,
    /// <p>An array of search criteria that targets instances using a Key,Value combination that you specify. Targets is required if you don't provide one or more instance IDs in the call.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>Describes a command filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CommandFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p><p>The filter value. Valid values for each filter key are as follows:</p> <ul> <li> <p> <b>InvokedAfter</b>: Specify a timestamp to limit your results. For example, specify <code>2018-07-07T00:00:00Z</code> to see a list of command executions occurring July 7, 2018, and later.</p> </li> <li> <p> <b>InvokedBefore</b>: Specify a timestamp to limit your results. For example, specify <code>2018-07-07T00:00:00Z</code> to see a list of command executions from before July 7, 2018.</p> </li> <li> <p> <b>Status</b>: Specify a valid command status to see a list of all command executions with that status. Status values you can specify include:</p> <ul> <li> <p> <code>Pending</code> </p> </li> <li> <p> <code>InProgress</code> </p> </li> <li> <p> <code>Success</code> </p> </li> <li> <p> <code>Cancelled</code> </p> </li> <li> <p> <code>Failed</code> </p> </li> <li> <p> <code>TimedOut</code> </p> </li> <li> <p> <code>Cancelling</code> </p> </li> </ul> </li> <li> <p> <b>DocumentName</b>: Specify name of the SSM document for which you want to see command execution results. For example, specify <code>AWS-RunPatchBaseline</code> to see command executions that used this SSM document to perform security patching operations on instances. </p> </li> <li> <p> <b>ExecutionStage</b>: Specify one of the following values:</p> <ul> <li> <p> <code>Executing</code>: Returns a list of command executions that are currently still running.</p> </li> <li> <p> <code>Complete</code>: Returns a list of command executions that have already completed. </p> </li> </ul> </li> </ul></p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>An invocation is copy of a command sent to a specific instance. A command can apply to one or more instances. A command invocation applies to one instance. For example, if a user runs SendCommand against three instances, then a command invocation is created for each requested instance ID. A command invocation returns status and detail information about a command you ran. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CommandInvocation {
    /// <p>CloudWatch Logs information where you want Systems Manager to send the command output.</p>
    #[serde(rename = "CloudWatchOutputConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_config: Option<CloudWatchOutputConfig>,
    /// <p>The command against which this invocation was requested.</p>
    #[serde(rename = "CommandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(rename = "CommandPlugins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_plugins: Option<Vec<CommandPlugin>>,
    /// <p>User-specified information about the command, such as a brief description of what the command should do.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The document name that was requested for execution.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>The SSM document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The instance ID in which this invocation was requested.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the invocation target. For Amazon EC2 instances this is the value for the aws:Name tag. For on-premises instances, this is the name of the instance.</p>
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// <p>Configurations for sending notifications about command status changes on a per instance basis.</p>
    #[serde(rename = "NotificationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    /// <p>The time and date the request was sent to this instance.</p>
    #[serde(rename = "RequestedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_date_time: Option<f64>,
    /// <p>The IAM service role that Run Command uses to act on your behalf when sending notifications about command status changes on a per instance basis.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The URL to the plugin's StdErr file in Amazon S3, if the Amazon S3 bucket was defined for the parent command. For an invocation, StandardErrorUrl is populated if there is just one plugin defined for the command, and the Amazon S3 bucket was defined for the command.</p>
    #[serde(rename = "StandardErrorUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_url: Option<String>,
    /// <p>The URL to the plugin's StdOut file in Amazon S3, if the Amazon S3 bucket was defined for the parent command. For an invocation, StandardOutputUrl is populated if there is just one plugin defined for the command, and the Amazon S3 bucket was defined for the command.</p>
    #[serde(rename = "StandardOutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_url: Option<String>,
    /// <p>Whether or not the invocation succeeded, failed, or is pending.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>A detailed status of the command execution for each invocation (each instance targeted by the command). StatusDetails includes more information than Status because it includes states resulting from error and concurrency control parameters. StatusDetails can show different results than Status. For more information about these statuses, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-commands.html">Understanding Command Statuses</a> in the <i>AWS Systems Manager User Guide</i>. StatusDetails can be one of the following values:</p> <ul> <li> <p>Pending: The command has not been sent to the instance.</p> </li> <li> <p>In Progress: The command has been sent to the instance but has not reached a terminal state.</p> </li> <li> <p>Success: The execution of the command or plugin was successfully completed. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out: The command was not delivered to the instance before the delivery timeout expired. Delivery timeouts do not count against the parent command&#39;s MaxErrors limit, but they do contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Execution Timed Out: Command execution started on the instance, but the execution was not complete before the execution timeout expired. Execution timeouts count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Failed: The command was not successful on the instance. For a plugin, this indicates that the result code was not zero. For a command invocation, this indicates that the result code for one or more plugins was not zero. Invocation failures count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Canceled: The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Undeliverable: The command can&#39;t be delivered to the instance. The instance might not exist or might not be responding. Undeliverable invocations don&#39;t count against the parent command&#39;s MaxErrors limit and don&#39;t contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Terminated: The parent command exceeded its MaxErrors limit and subsequent command invocations were canceled by the system. This is a terminal state.</p> </li> </ul></p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p> Gets the trace output sent by the agent. </p>
    #[serde(rename = "TraceOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_output: Option<String>,
}

/// <p>Describes plugin details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CommandPlugin {
    /// <p>The name of the plugin. Must be one of the following: aws:updateAgent, aws:domainjoin, aws:applications, aws:runPowerShellScript, aws:psmodule, aws:cloudWatch, aws:runShellScript, or aws:updateSSMAgent. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Output of the plugin execution.</p>
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// <p>The S3 bucket where the responses to the command executions should be stored. This was requested when issuing the command. For example, in the following response:</p> <p> test_folder/ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix/i-1234567876543/awsrunShellScript </p> <p>test_folder is the name of the Amazon S3 bucket;</p> <p> ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix is the name of the S3 prefix;</p> <p>i-1234567876543 is the instance ID;</p> <p>awsrunShellScript is the name of the plugin.</p>
    #[serde(rename = "OutputS3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    /// <p>The S3 directory path inside the bucket where the responses to the command executions should be stored. This was requested when issuing the command. For example, in the following response:</p> <p> test_folder/ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix/i-1234567876543/awsrunShellScript </p> <p>test_folder is the name of the Amazon S3 bucket;</p> <p> ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix is the name of the S3 prefix;</p> <p>i-1234567876543 is the instance ID;</p> <p>awsrunShellScript is the name of the plugin.</p>
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    /// <p>(Deprecated) You can no longer specify this parameter. The system ignores it. Instead, Systems Manager automatically determines the Amazon S3 bucket region.</p>
    #[serde(rename = "OutputS3Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
    /// <p>A numeric response code generated after running the plugin. </p>
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i64>,
    /// <p>The time the plugin stopped running. Could stop prematurely if, for example, a cancel command was sent. </p>
    #[serde(rename = "ResponseFinishDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_finish_date_time: Option<f64>,
    /// <p>The time the plugin started running. </p>
    #[serde(rename = "ResponseStartDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_start_date_time: Option<f64>,
    /// <p>The URL for the complete text written by the plugin to stderr. If execution is not yet complete, then this string is empty.</p>
    #[serde(rename = "StandardErrorUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_url: Option<String>,
    /// <p>The URL for the complete text written by the plugin to stdout in Amazon S3. If the Amazon S3 bucket for the command was not specified, then this string is empty.</p>
    #[serde(rename = "StandardOutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_url: Option<String>,
    /// <p>The status of this plugin. You can run a document with multiple plugins.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>A detailed status of the plugin execution. StatusDetails includes more information than Status because it includes states resulting from error and concurrency control parameters. StatusDetails can show different results than Status. For more information about these statuses, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-commands.html">Understanding Command Statuses</a> in the <i>AWS Systems Manager User Guide</i>. StatusDetails can be one of the following values:</p> <ul> <li> <p>Pending: The command has not been sent to the instance.</p> </li> <li> <p>In Progress: The command has been sent to the instance but has not reached a terminal state.</p> </li> <li> <p>Success: The execution of the command or plugin was successfully completed. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out: The command was not delivered to the instance before the delivery timeout expired. Delivery timeouts do not count against the parent command&#39;s MaxErrors limit, but they do contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Execution Timed Out: Command execution started on the instance, but the execution was not complete before the execution timeout expired. Execution timeouts count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Failed: The command was not successful on the instance. For a plugin, this indicates that the result code was not zero. For a command invocation, this indicates that the result code for one or more plugins was not zero. Invocation failures count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Canceled: The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Undeliverable: The command can&#39;t be delivered to the instance. The instance might not exist, or it might not be responding. Undeliverable invocations don&#39;t count against the parent command&#39;s MaxErrors limit, and they don&#39;t contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Terminated: The parent command exceeded its MaxErrors limit and subsequent command invocations were canceled by the system. This is a terminal state.</p> </li> </ul></p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

/// <p>A summary of the call execution that includes an execution ID, the type of execution (for example, <code>Command</code>), and the date/time of the execution using a datetime object that is saved in the following format: yyyy-MM-dd'T'HH:mm:ss'Z'.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceExecutionSummary {
    /// <p>An ID created by the system when <code>PutComplianceItems</code> was called. For example, <code>CommandID</code> is a valid execution ID. You can use this ID in subsequent calls.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// <p>The time the execution ran as a datetime object that is saved in the following format: yyyy-MM-dd'T'HH:mm:ss'Z'.</p>
    #[serde(rename = "ExecutionTime")]
    pub execution_time: f64,
    /// <p>The type of execution. For example, <code>Command</code> is a valid execution type.</p>
    #[serde(rename = "ExecutionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<String>,
}

/// <p>Information about the compliance as defined by the resource type. For example, for a patch resource type, <code>Items</code> includes information about the PatchSeverity, Classification, etc.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComplianceItem {
    /// <p>The compliance type. For example, Association (for a State Manager association), Patch, or Custom:<code>string</code> are all valid compliance types.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>A "Key": "Value" tag combination for the compliance item.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<::std::collections::HashMap<String, String>>,
    /// <p>A summary for the compliance item. The summary includes an execution ID, the execution type (for example, command), and the execution time.</p>
    #[serde(rename = "ExecutionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary: Option<ComplianceExecutionSummary>,
    /// <p>An ID for the compliance item. For example, if the compliance item is a Windows patch, the ID could be the number of the KB article; for example: KB4010320.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>An ID for the resource. For a managed instance, this is the instance ID.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The type of resource. <code>ManagedInstance</code> is currently the only supported resource type.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The severity of the compliance status. Severity can be one of the following: Critical, High, Medium, Low, Informational, Unspecified.</p>
    #[serde(rename = "Severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// <p>The status of the compliance item. An item is either COMPLIANT or NON_COMPLIANT.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A title for the compliance item. For example, if the compliance item is a Windows patch, the title could be the title of the KB article for the patch; for example: Security Update for Active Directory Federation Services.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>Information about a compliance item.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ComplianceItemEntry {
    /// <p>A "Key": "Value" tag combination for the compliance item.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<::std::collections::HashMap<String, String>>,
    /// <p>The compliance item ID. For example, if the compliance item is a Windows patch, the ID could be the number of the KB article.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The severity of the compliance status. Severity can be one of the following: Critical, High, Medium, Low, Informational, Unspecified.</p>
    #[serde(rename = "Severity")]
    pub severity: String,
    /// <p>The status of the compliance item. An item is either COMPLIANT or NON_COMPLIANT.</p>
    #[serde(rename = "Status")]
    pub status: String,
    /// <p>The title of the compliance item. For example, if the compliance item is a Windows patch, the title could be the title of the KB article for the patch; for example: Security Update for Active Directory Federation Services. </p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>One or more filters. Use a filter to return a more specific list of results.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ComplianceStringFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The type of comparison that should be performed for the value: Equal, NotEqual, BeginWith, LessThan, or GreaterThan.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value for which to search.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>A summary of compliance information by compliance type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComplianceSummaryItem {
    /// <p>The type of compliance item. For example, the compliance type can be Association, Patch, or Custom:string.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>A list of COMPLIANT items for the specified compliance type.</p>
    #[serde(rename = "CompliantSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_summary: Option<CompliantSummary>,
    /// <p>A list of NON_COMPLIANT items for the specified compliance type.</p>
    #[serde(rename = "NonCompliantSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_summary: Option<NonCompliantSummary>,
}

/// <p>A summary of resources that are compliant. The summary is organized according to the resource count for each compliance type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CompliantSummary {
    /// <p>The total number of resources that are compliant.</p>
    #[serde(rename = "CompliantCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_count: Option<i64>,
    /// <p>A summary of the compliance severity by compliance type.</p>
    #[serde(rename = "SeveritySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_summary: Option<SeveritySummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateActivationRequest {
    /// <p><p>The name of the registered, managed instance as it will appear in the Amazon EC2 console or when you use the AWS command line tools to list EC2 resources.</p> <important> <p>Do not enter personally identifiable information in this field.</p> </important></p>
    #[serde(rename = "DefaultInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_name: Option<String>,
    /// <p><p>A user-defined description of the resource that you want to register with Amazon EC2. </p> <important> <p>Do not enter personally identifiable information in this field.</p> </important></p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date by which this activation request should expire. The default value is 24 hours.</p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p>The Amazon Identity and Access Management (IAM) role that you want to assign to the managed instance. </p>
    #[serde(rename = "IamRole")]
    pub iam_role: String,
    /// <p>Specify the maximum number of managed instances you want to register. The default value is 1 instance.</p>
    #[serde(rename = "RegistrationLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_limit: Option<i64>,
    /// <p>Optional metadata that you assign to a resource. Tags enable you to categorize a resource in different ways, such as by purpose, owner, or environment. For example, you might want to tag an activation to identify which servers or virtual machines (VMs) in your on-premises environment you intend to activate. In this case, you could specify the following key name/value pairs:</p> <ul> <li> <p> <code>Key=OS,Value=Windows</code> </p> </li> <li> <p> <code>Key=Environment,Value=Production</code> </p> </li> </ul> <important> <p>When you install SSM Agent on your on-premises servers and VMs, you specify an activation ID and code. When you specify the activation ID and code, tags assigned to the activation are automatically applied to the on-premises servers or VMs.</p> </important> <p>You can't add tags to or delete tags from an existing activation. You can tag your on-premises servers and VMs after they connect to Systems Manager for the first time and are assigned a managed instance ID. This means they are listed in the AWS Systems Manager console with an ID that is prefixed with "mi-". For information about how to add tags to your managed instances, see <a>AddTagsToResource</a>. For information about how to remove tags from your managed instances, see <a>RemoveTagsFromResource</a>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateActivationResult {
    /// <p>The code the system generates when it processes the activation. The activation code functions like a password to validate the activation ID. </p>
    #[serde(rename = "ActivationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_code: Option<String>,
    /// <p>The ID number generated by the system when it processed the activation. The activation ID functions like a user name.</p>
    #[serde(rename = "ActivationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAssociationBatchRequest {
    /// <p>One or more associations.</p>
    #[serde(rename = "Entries")]
    pub entries: Vec<CreateAssociationBatchRequestEntry>,
}

/// <p>Describes the association of a Systems Manager SSM document and an instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateAssociationBatchRequestEntry {
    /// <p>Specify a descriptive name for the association.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>Specify the target for the association. This target is required for associations that use an Automation document and target resources by using rate controls.</p>
    #[serde(rename = "AutomationTargetParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_target_parameter_name: Option<String>,
    /// <p>The severity level to assign to the association.</p>
    #[serde(rename = "ComplianceSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_severity: Option<String>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The ID of the instance. </p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The maximum number of targets allowed to run the association at the same time. You can specify a number, for example 10, or a percentage of the target set, for example 10%. The default value is 100%, which means all targets run the association at the same time.</p> <p>If a new instance starts and attempts to run an association while Systems Manager is running MaxConcurrency associations, the association is allowed to run. During the next association interval, the new instance will process its association within the limit specified for MaxConcurrency.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The number of errors that are allowed before the system stops sending requests to run the association on additional targets. You can specify either an absolute number of errors, for example 10, or a percentage of the target set, for example 10%. If you specify 3, for example, the system stops sending requests when the fourth error is received. If you specify 0, then the system stops sending requests after the first error is returned. If you run an association on 50 instances and set MaxError to 10%, then the system stops sending the request when the sixth error is received.</p> <p>Executions that are already running an association when MaxErrors is reached are allowed to complete, but some of these executions may fail as well. If you need to ensure that there won't be more than max-errors failed executions, set MaxConcurrency to 1 so that executions proceed one at a time.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The name of the SSM document that contains the configuration information for the instance. You can specify Command or Automation documents.</p> <p>You can specify AWS-predefined documents, documents you created, or a document that is shared with you from another account.</p> <p>For SSM documents that are shared with you from other AWS accounts, you must specify the complete SSM document ARN, in the following format:</p> <p> <code>arn:aws:ssm:<i>region</i>:<i>account-id</i>:document/<i>document-name</i> </code> </p> <p>For example:</p> <p> <code>arn:aws:ssm:us-east-2:12345678912:document/My-Shared-Document</code> </p> <p>For AWS-predefined documents and SSM documents you created in your account, you only need to specify the document name. For example, <code>AWS-ApplyPatchBaseline</code> or <code>My-Document</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    /// <p>A description of the parameters for a document. </p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>A cron expression that specifies a schedule when the association runs.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The instances targeted by the request.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAssociationBatchResult {
    /// <p>Information about the associations that failed.</p>
    #[serde(rename = "Failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<FailedCreateAssociation>>,
    /// <p>Information about the associations that succeeded.</p>
    #[serde(rename = "Successful")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<AssociationDescription>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAssociationRequest {
    /// <p>Specify a descriptive name for the association.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>Specify the target for the association. This target is required for associations that use an Automation document and target resources by using rate controls.</p>
    #[serde(rename = "AutomationTargetParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_target_parameter_name: Option<String>,
    /// <p>The severity level to assign to the association.</p>
    #[serde(rename = "ComplianceSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_severity: Option<String>,
    /// <p>The document version you want to associate with the target(s). Can be a specific version or the default version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p><p>The instance ID.</p> <note> <p> <code>InstanceId</code> has been deprecated. To specify an instance ID for an association, use the <code>Targets</code> parameter. If you use the parameter <code>InstanceId</code>, you cannot use the parameters <code>AssociationName</code>, <code>DocumentVersion</code>, <code>MaxErrors</code>, <code>MaxConcurrency</code>, <code>OutputLocation</code>, or <code>ScheduleExpression</code>. To use these parameters, you must use the <code>Targets</code> parameter.</p> </note></p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The maximum number of targets allowed to run the association at the same time. You can specify a number, for example 10, or a percentage of the target set, for example 10%. The default value is 100%, which means all targets run the association at the same time.</p> <p>If a new instance starts and attempts to run an association while Systems Manager is running MaxConcurrency associations, the association is allowed to run. During the next association interval, the new instance will process its association within the limit specified for MaxConcurrency.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The number of errors that are allowed before the system stops sending requests to run the association on additional targets. You can specify either an absolute number of errors, for example 10, or a percentage of the target set, for example 10%. If you specify 3, for example, the system stops sending requests when the fourth error is received. If you specify 0, then the system stops sending requests after the first error is returned. If you run an association on 50 instances and set MaxError to 10%, then the system stops sending the request when the sixth error is received.</p> <p>Executions that are already running an association when MaxErrors is reached are allowed to complete, but some of these executions may fail as well. If you need to ensure that there won't be more than max-errors failed executions, set MaxConcurrency to 1 so that executions proceed one at a time.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The name of the SSM document that contains the configuration information for the instance. You can specify Command or Automation documents.</p> <p>You can specify AWS-predefined documents, documents you created, or a document that is shared with you from another account.</p> <p>For SSM documents that are shared with you from other AWS accounts, you must specify the complete SSM document ARN, in the following format:</p> <p> <code>arn:<i>partition</i>:ssm:<i>region</i>:<i>account-id</i>:document/<i>document-name</i> </code> </p> <p>For example:</p> <p> <code>arn:aws:ssm:us-east-2:12345678912:document/My-Shared-Document</code> </p> <p>For AWS-predefined documents and SSM documents you created in your account, you only need to specify the document name. For example, <code>AWS-ApplyPatchBaseline</code> or <code>My-Document</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An Amazon S3 bucket where you want to store the output details of the request.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    /// <p>The parameters for the runtime configuration of the document.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>A cron expression when the association will be applied to the target(s).</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The targets (either instances or tags) for the association. You must specify a value for <code>Targets</code> if you don't specify a value for <code>InstanceId</code>.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAssociationResult {
    /// <p>Information about the association.</p>
    #[serde(rename = "AssociationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDocumentRequest {
    /// <p>A list of key and value pairs that describe attachments to a version of a document.</p>
    #[serde(rename = "Attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentsSource>>,
    /// <p>A valid JSON or YAML string.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>Specify the document format for the request. The document format can be either JSON or YAML. JSON is the default format.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The type of document to create. Valid document types include: <code>Command</code>, <code>Policy</code>, <code>Automation</code>, <code>Session</code>, and <code>Package</code>.</p>
    #[serde(rename = "DocumentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    /// <p><p>A name for the Systems Manager document.</p> <important> <p>Do not use the following to begin the names of documents you create. They are reserved by AWS for use as document prefixes:</p> <ul> <li> <p> <code>aws</code> </p> </li> <li> <p> <code>amazon</code> </p> </li> <li> <p> <code>amzn</code> </p> </li> </ul> </important></p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Optional metadata that you assign to a resource. Tags enable you to categorize a resource in different ways, such as by purpose, owner, or environment. For example, you might want to tag an SSM document to identify the types of targets or the environment where it will run. In this case, you could specify the following key name/value pairs:</p> <ul> <li> <p> <code>Key=OS,Value=Windows</code> </p> </li> <li> <p> <code>Key=Environment,Value=Production</code> </p> </li> </ul> <note> <p>To add tags to an existing SSM document, use the <a>AddTagsToResource</a> action.</p> </note></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Specify a target type to define the kinds of resources the document can run on. For example, to run a document on EC2 instances, specify the following value: /AWS::EC2::Instance. If you specify a value of '/' the document can run on all types of resources. If you don't specify a value, the document can't run on any resources. For a list of valid resource types, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a> in the <i>AWS CloudFormation User Guide</i>. </p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// <p>An optional field specifying the version of the artifact you are creating with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document, and cannot be changed.</p>
    #[serde(rename = "VersionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDocumentResult {
    /// <p>Information about the Systems Manager document.</p>
    #[serde(rename = "DocumentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateMaintenanceWindowRequest {
    /// <p>Enables a maintenance window task to run on managed instances, even if you have not registered those instances as targets. If enabled, then you must specify the unregistered instances (by instance ID) when you register a task with the maintenance window.</p> <p>If you don't enable this option, then you must specify previously-registered targets when you register a task with the maintenance window.</p>
    #[serde(rename = "AllowUnassociatedTargets")]
    pub allow_unassociated_targets: bool,
    /// <p>User-provided idempotency token.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The number of hours before the end of the maintenance window that Systems Manager stops scheduling new tasks for execution.</p>
    #[serde(rename = "Cutoff")]
    pub cutoff: i64,
    /// <p>An optional description for the maintenance window. We recommend specifying a description to help you organize your maintenance windows. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The duration of the maintenance window in hours.</p>
    #[serde(rename = "Duration")]
    pub duration: i64,
    /// <p>The date and time, in ISO-8601 Extended format, for when you want the maintenance window to become inactive. EndDate allows you to set a date and time in the future when the maintenance window will no longer run.</p>
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// <p>The name of the maintenance window.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The schedule of the maintenance window in the form of a cron or rate expression.</p>
    #[serde(rename = "Schedule")]
    pub schedule: String,
    /// <p>The time zone that the scheduled maintenance window executions are based on, in Internet Assigned Numbers Authority (IANA) format. For example: "America/Los_Angeles", "etc/UTC", or "Asia/Seoul". For more information, see the <a href="https://www.iana.org/time-zones">Time Zone Database</a> on the IANA website.</p>
    #[serde(rename = "ScheduleTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_timezone: Option<String>,
    /// <p>The date and time, in ISO-8601 Extended format, for when you want the maintenance window to become active. StartDate allows you to delay activation of the maintenance window until the specified future date.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// <p><p>Optional metadata that you assign to a resource. Tags enable you to categorize a resource in different ways, such as by purpose, owner, or environment. For example, you might want to tag a maintenance window to identify the type of tasks it will run, the types of targets, and the environment it will run in. In this case, you could specify the following key name/value pairs:</p> <ul> <li> <p> <code>Key=TaskType,Value=AgentUpdate</code> </p> </li> <li> <p> <code>Key=OS,Value=Windows</code> </p> </li> <li> <p> <code>Key=Environment,Value=Production</code> </p> </li> </ul> <note> <p>To add tags to an existing maintenance window, use the <a>AddTagsToResource</a> action.</p> </note></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMaintenanceWindowResult {
    /// <p>The ID of the created maintenance window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateOpsItemRequest {
    /// <p>Information about the OpsItem. </p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The Amazon Resource Name (ARN) of an SNS topic where notifications are sent when this OpsItem is edited or changed.</p>
    #[serde(rename = "Notifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<OpsItemNotification>>,
    /// <p>Operational data is custom data that provides useful reference details about the OpsItem. For example, you can specify log files, error strings, license keys, troubleshooting tips, or other relevant data. You enter operational data as key-value pairs. The key has a maximum length of 128 characters. The value has a maximum size of 20 KB.</p> <important> <p>Operational data keys <i>can't</i> begin with the following: amazon, aws, amzn, ssm, /amazon, /aws, /amzn, /ssm.</p> </important> <p>You can choose to make the data searchable by other users in the account or you can restrict search access. Searchable data means that all users with access to the OpsItem Overview page (as provided by the <a>DescribeOpsItems</a> API action) can view and search on the specified data. Operational data that is not searchable is only viewable by users who have access to the OpsItem (as provided by the <a>GetOpsItem</a> API action).</p> <p>Use the <code>/aws/resources</code> key in OperationalData to specify a related resource in the request. Use the <code>/aws/automations</code> key in OperationalData to associate an Automation runbook with the OpsItem. To view AWS CLI example commands that use these keys, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-creating-OpsItems.html#OpsCenter-manually-create-OpsItems">Creating OpsItems Manually</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "OperationalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_data: Option<::std::collections::HashMap<String, OpsItemDataValue>>,
    /// <p>The importance of this OpsItem in relation to other OpsItems in the system.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>One or more OpsItems that share something in common with the current OpsItems. For example, related OpsItems can include OpsItems with similar error messages, impacted resources, or statuses for the impacted resource.</p>
    #[serde(rename = "RelatedOpsItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_ops_items: Option<Vec<RelatedOpsItem>>,
    /// <p>The origin of the OpsItem, such as Amazon EC2 or AWS Systems Manager.</p>
    #[serde(rename = "Source")]
    pub source: String,
    /// <p><p>Optional metadata that you assign to a resource. You can restrict access to OpsItems by using an inline IAM policy that specifies tags. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-getting-started.html#OpsCenter-getting-started-user-permissions">Getting Started with OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>.</p> <p>Tags use a key-value pair. For example:</p> <p> <code>Key=Department,Value=Finance</code> </p> <note> <p>To add tags to an existing OpsItem, use the <a>AddTagsToResource</a> action.</p> </note></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A short heading that describes the nature of the OpsItem and the impacted resource.</p>
    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateOpsItemResponse {
    /// <p>The ID of the OpsItem.</p>
    #[serde(rename = "OpsItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePatchBaselineRequest {
    /// <p>A set of rules used to include patches in the baseline.</p>
    #[serde(rename = "ApprovalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    /// <p>A list of explicitly approved patches for the baseline.</p> <p>For information about accepted formats for lists of approved patches and rejected patches, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/patch-manager-approved-rejected-package-name-formats.html">Package Name Formats for Approved and Rejected Patch Lists</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "ApprovedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    /// <p>Defines the compliance level for approved patches. This means that if an approved patch is reported as missing, this is the severity of the compliance violation. The default value is UNSPECIFIED.</p>
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    /// <p>Indicates whether the list of approved patches includes non-security updates that should be applied to the instances. The default value is 'false'. Applies to Linux instances only.</p>
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    /// <p>User-provided idempotency token.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>A description of the patch baseline.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A set of global filters used to include patches in the baseline.</p>
    #[serde(rename = "GlobalFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    /// <p>The name of the patch baseline.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Defines the operating system the patch baseline applies to. The Default value is WINDOWS.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>A list of explicitly rejected patches for the baseline.</p> <p>For information about accepted formats for lists of approved patches and rejected patches, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/patch-manager-approved-rejected-package-name-formats.html">Package Name Formats for Approved and Rejected Patch Lists</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "RejectedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    /// <p><p>The action for Patch Manager to take on patches included in the RejectedPackages list.</p> <ul> <li> <p> <b>ALLOW<em>AS</em>DEPENDENCY</b>: A package in the Rejected patches list is installed only if it is a dependency of another package. It is considered compliant with the patch baseline, and its status is reported as <i>InstalledOther</i>. This is the default action if no option is specified.</p> </li> <li> <p> <b>BLOCK</b>: Packages in the RejectedPatches list, and packages that include them as dependencies, are not installed under any circumstances. If a package was installed before it was added to the Rejected patches list, it is considered non-compliant with the patch baseline, and its status is reported as <i>InstalledRejected</i>.</p> </li> </ul></p>
    #[serde(rename = "RejectedPatchesAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches_action: Option<String>,
    /// <p>Information about the patches to use to update the instances, including target operating systems and source repositories. Applies to Linux instances only.</p>
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
    /// <p><p>Optional metadata that you assign to a resource. Tags enable you to categorize a resource in different ways, such as by purpose, owner, or environment. For example, you might want to tag a patch baseline to identify the severity level of patches it specifies and the operating system family it applies to. In this case, you could specify the following key name/value pairs:</p> <ul> <li> <p> <code>Key=PatchSeverity,Value=Critical</code> </p> </li> <li> <p> <code>Key=OS,Value=Windows</code> </p> </li> </ul> <note> <p>To add tags to an existing patch baseline, use the <a>AddTagsToResource</a> action.</p> </note></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePatchBaselineResult {
    /// <p>The ID of the created patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceDataSyncRequest {
    /// <p>Amazon S3 configuration details for the sync.</p>
    #[serde(rename = "S3Destination")]
    pub s3_destination: ResourceDataSyncS3Destination,
    /// <p>A name for the configuration.</p>
    #[serde(rename = "SyncName")]
    pub sync_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateResourceDataSyncResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteActivationRequest {
    /// <p>The ID of the activation that you want to delete.</p>
    #[serde(rename = "ActivationId")]
    pub activation_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteActivationResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAssociationRequest {
    /// <p>The association ID that you want to delete.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAssociationResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDocumentRequest {
    /// <p>The version of the document that you want to delete. If not provided, all versions of the document are deleted.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the document.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The version name of the document that you want to delete. If not provided, all versions of the document are deleted.</p>
    #[serde(rename = "VersionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDocumentResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteInventoryRequest {
    /// <p>User-provided idempotency token.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Use this option to view a summary of the deletion request without deleting any data or the data type. This option is useful when you only want to understand what will be deleted. Once you validate that the data to be deleted is what you intend to delete, you can run the same command without specifying the <code>DryRun</code> option.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>Use the <code>SchemaDeleteOption</code> to delete a custom inventory type (schema). If you don't choose this option, the system only deletes existing inventory data associated with the custom inventory type. Choose one of the following options:</p> <p>DisableSchema: If you choose this option, the system ignores all inventory data for the specified version, and any earlier versions. To enable this schema again, you must call the <code>PutInventory</code> action for a version greater than the disabled version.</p> <p>DeleteSchema: This option deletes the specified custom type from the Inventory service. You can recreate the schema later, if you want.</p>
    #[serde(rename = "SchemaDeleteOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_delete_option: Option<String>,
    /// <p>The name of the custom inventory type for which you want to delete either all previously collected data, or the inventory type itself. </p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInventoryResult {
    /// <p>Every <code>DeleteInventory</code> action is assigned a unique ID. This option returns a unique ID. You can use this ID to query the status of a delete operation. This option is useful for ensuring that a delete operation has completed before you begin other actions. </p>
    #[serde(rename = "DeletionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_id: Option<String>,
    /// <p>A summary of the delete operation. For more information about this summary, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-inventory-delete.html#sysman-inventory-delete-summary">Understanding the Delete Inventory Summary</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "DeletionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_summary: Option<InventoryDeletionSummary>,
    /// <p>The name of the inventory data type specified in the request.</p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMaintenanceWindowRequest {
    /// <p>The ID of the maintenance window to delete.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMaintenanceWindowResult {
    /// <p>The ID of the deleted maintenance window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteParameterRequest {
    /// <p>The name of the parameter to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteParameterResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteParametersRequest {
    /// <p>The names of the parameters to delete.</p>
    #[serde(rename = "Names")]
    pub names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteParametersResult {
    /// <p>The names of the deleted parameters.</p>
    #[serde(rename = "DeletedParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_parameters: Option<Vec<String>>,
    /// <p>The names of parameters that weren't deleted because the parameters are not valid.</p>
    #[serde(rename = "InvalidParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_parameters: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePatchBaselineRequest {
    /// <p>The ID of the patch baseline to delete.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePatchBaselineResult {
    /// <p>The ID of the deleted patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResourceDataSyncRequest {
    /// <p>The name of the configuration to delete.</p>
    #[serde(rename = "SyncName")]
    pub sync_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteResourceDataSyncResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterManagedInstanceRequest {
    /// <p>The ID assigned to the managed instance when you registered it using the activation process. </p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterManagedInstanceResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterPatchBaselineForPatchGroupRequest {
    /// <p>The ID of the patch baseline to deregister the patch group from.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
    /// <p>The name of the patch group that should be deregistered from the patch baseline.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterPatchBaselineForPatchGroupResult {
    /// <p>The ID of the patch baseline the patch group was deregistered from.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The name of the patch group deregistered from the patch baseline.</p>
    #[serde(rename = "PatchGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterTargetFromMaintenanceWindowRequest {
    /// <p>The system checks if the target is being referenced by a task. If the target is being referenced, the system returns an error and does not deregister the target from the maintenance window.</p>
    #[serde(rename = "Safe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<bool>,
    /// <p>The ID of the maintenance window the target should be removed from.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// <p>The ID of the target definition to remove.</p>
    #[serde(rename = "WindowTargetId")]
    pub window_target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterTargetFromMaintenanceWindowResult {
    /// <p>The ID of the maintenance window the target was removed from.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The ID of the removed target definition.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterTaskFromMaintenanceWindowRequest {
    /// <p>The ID of the maintenance window the task should be removed from.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// <p>The ID of the task to remove from the maintenance window.</p>
    #[serde(rename = "WindowTaskId")]
    pub window_task_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterTaskFromMaintenanceWindowResult {
    /// <p>The ID of the maintenance window the task was removed from.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The ID of the task removed from the maintenance window.</p>
    #[serde(rename = "WindowTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

/// <p>Filter for the DescribeActivation API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeActivationsFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "FilterKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_key: Option<String>,
    /// <p>The filter values.</p>
    #[serde(rename = "FilterValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeActivationsRequest {
    /// <p>A filter to view information about your activations.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribeActivationsFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeActivationsResult {
    /// <p>A list of activations for your AWS account.</p>
    #[serde(rename = "ActivationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_list: Option<Vec<Activation>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAssociationExecutionTargetsRequest {
    /// <p>The association ID that includes the execution for which you want to view details.</p>
    #[serde(rename = "AssociationId")]
    pub association_id: String,
    /// <p>The execution ID for which you want to view details.</p>
    #[serde(rename = "ExecutionId")]
    pub execution_id: String,
    /// <p>Filters for the request. You can specify the following filters and values.</p> <p>Status (EQUAL)</p> <p>ResourceId (EQUAL)</p> <p>ResourceType (EQUAL)</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AssociationExecutionTargetsFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAssociationExecutionTargetsResult {
    /// <p>Information about the execution.</p>
    #[serde(rename = "AssociationExecutionTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_execution_targets: Option<Vec<AssociationExecutionTarget>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAssociationExecutionsRequest {
    /// <p>The association ID for which you want to view execution history details.</p>
    #[serde(rename = "AssociationId")]
    pub association_id: String,
    /// <p>Filters for the request. You can specify the following filters and values.</p> <p>ExecutionId (EQUAL)</p> <p>Status (EQUAL)</p> <p>CreatedTime (EQUAL, GREATER_THAN, LESS_THAN)</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AssociationExecutionFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAssociationExecutionsResult {
    /// <p>A list of the executions for the specified association ID.</p>
    #[serde(rename = "AssociationExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_executions: Option<Vec<AssociationExecution>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAssociationRequest {
    /// <p>The association ID for which you want information.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>Specify the association version to retrieve. To view the latest version, either specify <code>$LATEST</code> for this parameter, or omit this parameter. To view a list of all associations for an instance, use <a>ListAssociations</a>. To get a list of versions for a specific association, use <a>ListAssociationVersions</a>. </p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAssociationResult {
    /// <p>Information about the association.</p>
    #[serde(rename = "AssociationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAutomationExecutionsRequest {
    /// <p>Filters used to limit the scope of executions that are requested.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AutomationExecutionFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAutomationExecutionsResult {
    /// <p>The list of details about each automation execution which has occurred which matches the filter specification, if any.</p>
    #[serde(rename = "AutomationExecutionMetadataList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_metadata_list: Option<Vec<AutomationExecutionMetadata>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAutomationStepExecutionsRequest {
    /// <p>The Automation execution ID for which you want step execution descriptions.</p>
    #[serde(rename = "AutomationExecutionId")]
    pub automation_execution_id: String,
    /// <p>One or more filters to limit the number of step executions returned by the request.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<StepExecutionFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A boolean that indicates whether to list step executions in reverse order by start time. The default value is false.</p>
    #[serde(rename = "ReverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAutomationStepExecutionsResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of details about the current state of all steps that make up an execution.</p>
    #[serde(rename = "StepExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_executions: Option<Vec<StepExecution>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAvailablePatchesRequest {
    /// <p>Filters used to scope down the returned patches.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
    /// <p>The maximum number of patches to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAvailablePatchesResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of patches. Each entry in the array is a patch structure.</p>
    #[serde(rename = "Patches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<Patch>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDocumentPermissionRequest {
    /// <p>The name of the document for which you are the owner.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The permission type for the document. The permission type can be <i>Share</i>.</p>
    #[serde(rename = "PermissionType")]
    pub permission_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDocumentPermissionResponse {
    /// <p>The account IDs that have permission to use this document. The ID can be either an AWS account or <i>All</i>.</p>
    #[serde(rename = "AccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDocumentRequest {
    /// <p>The document version for which you want information. Can be a specific version or the default version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An optional field specifying the version of the artifact associated with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document, and cannot be changed.</p>
    #[serde(rename = "VersionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDocumentResult {
    /// <p>Information about the Systems Manager document.</p>
    #[serde(rename = "Document")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEffectiveInstanceAssociationsRequest {
    /// <p>The instance ID for which you want to view all associations.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEffectiveInstanceAssociationsResult {
    /// <p>The associations for the requested instance.</p>
    #[serde(rename = "Associations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<InstanceAssociation>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEffectivePatchesForPatchBaselineRequest {
    /// <p>The ID of the patch baseline to retrieve the effective patches for.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
    /// <p>The maximum number of patches to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEffectivePatchesForPatchBaselineResult {
    /// <p>An array of patches and patch status.</p>
    #[serde(rename = "EffectivePatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_patches: Option<Vec<EffectivePatch>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInstanceAssociationsStatusRequest {
    /// <p>The instance IDs for which you want association status information.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInstanceAssociationsStatusResult {
    /// <p>Status information about the association.</p>
    #[serde(rename = "InstanceAssociationStatusInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_association_status_infos: Option<Vec<InstanceAssociationStatusInfo>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInstanceInformationRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of instances. You can filter on Amazon EC2 tag. Specify tags by using a key-value mapping.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InstanceInformationStringFilter>>,
    /// <p>This is a legacy method. We recommend that you don't use this method. Instead, use the <a>InstanceInformationFilter</a> action. The <code>InstanceInformationFilter</code> action enables you to return instance information by using tags that are specified as a key-value mapping. </p> <p>If you do use this method, then you can't use the <code>InstanceInformationFilter</code> action. Using this method and the <code>InstanceInformationFilter</code> action causes an exception error. </p>
    #[serde(rename = "InstanceInformationFilterList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_information_filter_list: Option<Vec<InstanceInformationFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInstanceInformationResult {
    /// <p>The instance information list.</p>
    #[serde(rename = "InstanceInformationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_information_list: Option<Vec<InstanceInformation>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInstancePatchStatesForPatchGroupRequest {
    /// <p>Each entry in the array is a structure containing:</p> <p>Key (string between 1 and 200 characters)</p> <p> Values (array containing a single string)</p> <p> Type (string "Equal", "NotEqual", "LessThan", "GreaterThan")</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InstancePatchStateFilter>>,
    /// <p>The maximum number of patches to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the patch group for which the patch state information should be retrieved.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInstancePatchStatesForPatchGroupResult {
    /// <p>The high-level patch state for the requested instances. </p>
    #[serde(rename = "InstancePatchStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_patch_states: Option<Vec<InstancePatchState>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInstancePatchStatesRequest {
    /// <p>The ID of the instance whose patch state information should be retrieved.</p>
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
    /// <p>The maximum number of instances to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInstancePatchStatesResult {
    /// <p>The high-level patch state for the requested instances.</p>
    #[serde(rename = "InstancePatchStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_patch_states: Option<Vec<InstancePatchState>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInstancePatchesRequest {
    /// <p>An array of structures. Each entry in the array is a structure containing a Key, Value combination. Valid values for Key are <code>Classification</code> | <code>KBId</code> | <code>Severity</code> | <code>State</code>.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
    /// <p>The ID of the instance whose patch state information should be retrieved.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of patches to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInstancePatchesResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Each entry in the array is a structure containing:</p> <p>Title (string)</p> <p>KBId (string)</p> <p>Classification (string)</p> <p>Severity (string)</p> <p>State (string, such as "INSTALLED" or "FAILED")</p> <p>InstalledTime (DateTime)</p> <p>InstalledBy (string)</p>
    #[serde(rename = "Patches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<PatchComplianceData>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInventoryDeletionsRequest {
    /// <p>Specify the delete inventory ID for which you want information. This ID was returned by the <code>DeleteInventory</code> action.</p>
    #[serde(rename = "DeletionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_id: Option<String>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInventoryDeletionsResult {
    /// <p>A list of status items for deleted inventory.</p>
    #[serde(rename = "InventoryDeletions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_deletions: Option<Vec<InventoryDeletionStatusItem>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowExecutionTaskInvocationsRequest {
    /// <p>Optional filters used to scope down the returned task invocations. The supported filter key is STATUS with the corresponding values PENDING, IN_PROGRESS, SUCCESS, FAILED, TIMED_OUT, CANCELLING, and CANCELLED.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the specific task in the maintenance window task that should be retrieved.</p>
    #[serde(rename = "TaskId")]
    pub task_id: String,
    /// <p>The ID of the maintenance window execution the task is part of.</p>
    #[serde(rename = "WindowExecutionId")]
    pub window_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMaintenanceWindowExecutionTaskInvocationsResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the task invocation results per invocation.</p>
    #[serde(rename = "WindowExecutionTaskInvocationIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_task_invocation_identities:
        Option<Vec<MaintenanceWindowExecutionTaskInvocationIdentity>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowExecutionTasksRequest {
    /// <p>Optional filters used to scope down the returned tasks. The supported filter key is STATUS with the corresponding values PENDING, IN_PROGRESS, SUCCESS, FAILED, TIMED_OUT, CANCELLING, and CANCELLED. </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the maintenance window execution whose task executions should be retrieved.</p>
    #[serde(rename = "WindowExecutionId")]
    pub window_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMaintenanceWindowExecutionTasksResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the task executions.</p>
    #[serde(rename = "WindowExecutionTaskIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_task_identities: Option<Vec<MaintenanceWindowExecutionTaskIdentity>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowExecutionsRequest {
    /// <p>Each entry in the array is a structure containing:</p> <p>Key (string, between 1 and 128 characters)</p> <p>Values (array of strings, each string is between 1 and 256 characters)</p> <p>The supported Keys are ExecutedBefore and ExecutedAfter with the value being a date/time string such as 2016-11-04T05:00:00Z.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the maintenance window whose executions should be retrieved.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMaintenanceWindowExecutionsResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the maintenance window executions.</p>
    #[serde(rename = "WindowExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_executions: Option<Vec<MaintenanceWindowExecution>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowScheduleRequest {
    /// <p>Filters used to limit the range of results. For example, you can limit maintenance window executions to only those scheduled before or after a certain date and time.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of resource you want to retrieve information about. For example, "INSTANCE".</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The instance ID or key/value pair to retrieve information about.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The ID of the maintenance window to retrieve information about.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMaintenanceWindowScheduleResult {
    /// <p>The token for the next set of items to return. (You use this token in the next call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about maintenance window executions scheduled for the specified time range.</p>
    #[serde(rename = "ScheduledWindowExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_window_executions: Option<Vec<ScheduledWindowExecution>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowTargetsRequest {
    /// <p>Optional filters that can be used to narrow down the scope of the returned window targets. The supported filter keys are Type, WindowTargetId and OwnerInformation.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the maintenance window whose targets should be retrieved.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMaintenanceWindowTargetsResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the targets in the maintenance window.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<MaintenanceWindowTarget>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowTasksRequest {
    /// <p>Optional filters used to narrow down the scope of the returned tasks. The supported filter keys are WindowTaskId, TaskArn, Priority, and TaskType.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the maintenance window whose tasks should be retrieved.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMaintenanceWindowTasksResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the tasks in the maintenance window.</p>
    #[serde(rename = "Tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<MaintenanceWindowTask>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowsForTargetRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of resource you want to retrieve information about. For example, "INSTANCE".</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>The instance ID or key/value pair to retrieve information about.</p>
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMaintenanceWindowsForTargetResult {
    /// <p>The token for the next set of items to return. (You use this token in the next call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the maintenance window targets and tasks an instance is associated with.</p>
    #[serde(rename = "WindowIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_identities: Option<Vec<MaintenanceWindowIdentityForTarget>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowsRequest {
    /// <p>Optional filters used to narrow down the scope of the returned maintenance windows. Supported filter keys are <b>Name</b> and <b>Enabled</b>.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMaintenanceWindowsResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the maintenance windows.</p>
    #[serde(rename = "WindowIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_identities: Option<Vec<MaintenanceWindowIdentity>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeOpsItemsRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>One or more filters to limit the reponse.</p> <ul> <li> <p>Key: CreatedTime</p> <p>Operations: GreaterThan, LessThan</p> </li> <li> <p>Key: LastModifiedBy</p> <p>Operations: Contains, Equals</p> </li> <li> <p>Key: LastModifiedTime</p> <p>Operations: GreaterThan, LessThan</p> </li> <li> <p>Key: Priority</p> <p>Operations: Equals</p> </li> <li> <p>Key: Source</p> <p>Operations: Contains, Equals</p> </li> <li> <p>Key: Status</p> <p>Operations: Equals</p> </li> <li> <p>Key: Title</p> <p>Operations: Contains</p> </li> <li> <p>Key: OperationalData*</p> <p>Operations: Equals</p> </li> <li> <p>Key: OperationalDataKey</p> <p>Operations: Equals</p> </li> <li> <p>Key: OperationalDataValue</p> <p>Operations: Equals, Contains</p> </li> <li> <p>Key: OpsItemId</p> <p>Operations: Equals</p> </li> <li> <p>Key: ResourceId</p> <p>Operations: Contains</p> </li> <li> <p>Key: AutomationId</p> <p>Operations: Equals</p> </li> </ul> <p>*If you filter the response by using the OperationalData operator, specify a key-value pair by using the following JSON format: {"key":"key_name","value":"a_value"}</p>
    #[serde(rename = "OpsItemFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_filters: Option<Vec<OpsItemFilter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOpsItemsResponse {
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of OpsItems.</p>
    #[serde(rename = "OpsItemSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_summaries: Option<Vec<OpsItemSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeParametersRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ParametersFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Filters to limit the request results.</p>
    #[serde(rename = "ParameterFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_filters: Option<Vec<ParameterStringFilter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeParametersResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Parameters returned by the request.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterMetadata>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePatchBaselinesRequest {
    /// <p>Each element in the array is a structure containing: </p> <p>Key: (string, "NAME_PREFIX" or "OWNER")</p> <p>Value: (array of strings, exactly 1 entry, between 1 and 255 characters)</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
    /// <p>The maximum number of patch baselines to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePatchBaselinesResult {
    /// <p>An array of PatchBaselineIdentity elements.</p>
    #[serde(rename = "BaselineIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_identities: Option<Vec<PatchBaselineIdentity>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePatchGroupStateRequest {
    /// <p>The name of the patch group whose patch snapshot should be retrieved.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePatchGroupStateResult {
    /// <p>The number of instances in the patch group.</p>
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<i64>,
    /// <p>The number of instances with patches from the patch baseline that failed to install.</p>
    #[serde(rename = "InstancesWithFailedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_failed_patches: Option<i64>,
    /// <p>The number of instances with patches installed that aren't defined in the patch baseline.</p>
    #[serde(rename = "InstancesWithInstalledOtherPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_installed_other_patches: Option<i64>,
    /// <p>The number of instances with installed patches.</p>
    #[serde(rename = "InstancesWithInstalledPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_installed_patches: Option<i64>,
    /// <p><p>The number of instances with patches installed that are specified in a RejectedPatches list. Patches with a status of <i>INSTALLED<em>REJECTED</i> were typically installed before they were added to a RejectedPatches list.</p> <note> <p>If ALLOW</em>AS_DEPENDENCY is the specified option for RejectedPatchesAction, the value of InstancesWithInstalledRejectedPatches will always be 0 (zero).</p> </note></p>
    #[serde(rename = "InstancesWithInstalledRejectedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_installed_rejected_patches: Option<i64>,
    /// <p>The number of instances with missing patches from the patch baseline.</p>
    #[serde(rename = "InstancesWithMissingPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_missing_patches: Option<i64>,
    /// <p>The number of instances with patches that aren't applicable.</p>
    #[serde(rename = "InstancesWithNotApplicablePatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_not_applicable_patches: Option<i64>,
    /// <p>The number of instances with <code>NotApplicable</code> patches beyond the supported limit, which are not reported by name to Systems Manager Inventory.</p>
    #[serde(rename = "InstancesWithUnreportedNotApplicablePatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_unreported_not_applicable_patches: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePatchGroupsRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
    /// <p>The maximum number of patch groups to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePatchGroupsResult {
    /// <p>Each entry in the array contains:</p> <p>PatchGroup: string (between 1 and 256 characters, Regex: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$)</p> <p>PatchBaselineIdentity: A PatchBaselineIdentity element. </p>
    #[serde(rename = "Mappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<PatchGroupPatchBaselineMapping>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePatchPropertiesRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The operating system type for which to list patches.</p>
    #[serde(rename = "OperatingSystem")]
    pub operating_system: String,
    /// <p>Indicates whether to list patches for the Windows operating system or for Microsoft applications. Not applicable for Linux operating systems.</p>
    #[serde(rename = "PatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_set: Option<String>,
    /// <p>The patch property for which you want to view patch details. </p>
    #[serde(rename = "Property")]
    pub property: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePatchPropertiesResult {
    /// <p>The token for the next set of items to return. (You use this token in the next call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the properties for patches matching the filter request parameters.</p>
    #[serde(rename = "Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<::std::collections::HashMap<String, String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSessionsRequest {
    /// <p>One or more filters to limit the type of sessions returned by the request.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SessionFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The session status to retrieve a list of sessions for. For example, "Active".</p>
    #[serde(rename = "State")]
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSessionsResponse {
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of sessions meeting the request parameters.</p>
    #[serde(rename = "Sessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<Session>>,
}

/// <p>A default version of a document.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DocumentDefaultVersionDescription {
    /// <p>The default version of the document.</p>
    #[serde(rename = "DefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    /// <p>The default version of the artifact associated with the document.</p>
    #[serde(rename = "DefaultVersionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_name: Option<String>,
    /// <p>The name of the document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Describes a Systems Manager document. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DocumentDescription {
    /// <p>Details about the document attachments, including names, locations, sizes, etc.</p>
    #[serde(rename = "AttachmentsInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments_information: Option<Vec<AttachmentInformation>>,
    /// <p>The date when the document was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The default version.</p>
    #[serde(rename = "DefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    /// <p>A description of the document. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The document format, either JSON or YAML.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The type of document.</p>
    #[serde(rename = "DocumentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p><p>The Sha256 or Sha1 hash created by the system when the document was created. </p> <note> <p>Sha1 hashes have been deprecated.</p> </note></p>
    #[serde(rename = "Hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// <p><p>The hash type of the document. Valid values include <code>Sha256</code> or <code>Sha1</code>.</p> <note> <p>Sha1 hashes have been deprecated.</p> </note></p>
    #[serde(rename = "HashType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_type: Option<String>,
    /// <p>The latest version of the document.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AWS user account that created the document.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>A description of the parameters for a document.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<DocumentParameter>>,
    /// <p>The list of OS platforms compatible with this Systems Manager document. </p>
    #[serde(rename = "PlatformTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_types: Option<Vec<String>>,
    /// <p>The schema version.</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    /// <p>The SHA1 hash of the document, which you can use for verification.</p>
    #[serde(rename = "Sha1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_1: Option<String>,
    /// <p>The status of the Systems Manager document.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A message returned by AWS Systems Manager that explains the <code>Status</code> value. For example, a <code>Failed</code> status might be explained by the <code>StatusInformation</code> message, "The specified S3 bucket does not exist. Verify that the URL of the S3 bucket is correct."</p>
    #[serde(rename = "StatusInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_information: Option<String>,
    /// <p>The tags, or metadata, that have been applied to the document.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The target type which defines the kinds of resources the document can run on. For example, /AWS::EC2::Instance. For a list of valid resource types, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a> in the <i>AWS CloudFormation User Guide</i>. </p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// <p>The version of the artifact associated with the document.</p>
    #[serde(rename = "VersionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

/// <p>Describes a filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DocumentFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value of the filter.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>Describes the name of a Systems Manager document.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DocumentIdentifier {
    /// <p>The document format, either JSON or YAML.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The document type.</p>
    #[serde(rename = "DocumentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AWS user account that created the document.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The operating system platform. </p>
    #[serde(rename = "PlatformTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_types: Option<Vec<String>>,
    /// <p>The schema version.</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    /// <p>The tags, or metadata, that have been applied to the document.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The target type which defines the kinds of resources the document can run on. For example, /AWS::EC2::Instance. For a list of valid resource types, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a> in the <i>AWS CloudFormation User Guide</i>. </p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// <p>An optional field specifying the version of the artifact associated with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document, and cannot be changed.</p>
    #[serde(rename = "VersionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

/// <p>One or more filters. Use a filter to return a more specific list of documents.</p> <p>For keys, you can specify one or more tags that have been applied to a document. </p> <p>Other valid values include Owner, Name, PlatformTypes, and DocumentType.</p> <p>Note that only one Owner can be specified in a request. For example: <code>Key=Owner,Values=Self</code>.</p> <p>If you use Name as a key, you can use a name prefix to return a list of documents. For example, in the AWS CLI, to return a list of all documents that begin with <code>Te</code>, run the following command:</p> <p> <code>aws ssm list-documents --filters Key=Name,Values=Te</code> </p> <p>If you specify more than two keys, only documents that are identified by all the tags are returned in the results. If you specify more than two values for a key, documents that are identified by any of the values are returned in the results.</p> <p>To specify a custom key and value pair, use the format <code>Key=tag:[tagName],Values=[valueName]</code>.</p> <p>For example, if you created a Key called region and are using the AWS CLI to call the <code>list-documents</code> command: </p> <p> <code>aws ssm list-documents --filters Key=tag:region,Values=east,west Key=Owner,Values=Self</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DocumentKeyValuesFilter {
    /// <p>The name of the filter key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value for the filter key.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Parameters specified in a System Manager document that run on the server when the command is run. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DocumentParameter {
    /// <p>If specified, the default values for the parameters. Parameters without a default value are required. Parameters with a default value are optional.</p>
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>A description of what the parameter does, how to use it, the default value, and whether or not the parameter is optional.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the parameter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of parameter. The type can be either String or StringList.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Version information about the document.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DocumentVersionInfo {
    /// <p>The date the document was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The document format, either JSON or YAML.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>An identifier for the default version of the document.</p>
    #[serde(rename = "IsDefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    /// <p>The document name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the Systems Manager document, such as <code>Creating</code>, <code>Active</code>, <code>Failed</code>, and <code>Deleting</code>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A message returned by AWS Systems Manager that explains the <code>Status</code> value. For example, a <code>Failed</code> status might be explained by the <code>StatusInformation</code> message, "The specified S3 bucket does not exist. Verify that the URL of the S3 bucket is correct."</p>
    #[serde(rename = "StatusInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_information: Option<String>,
    /// <p>The version of the artifact associated with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document, and cannot be changed.</p>
    #[serde(rename = "VersionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

/// <p>The EffectivePatch structure defines metadata about a patch along with the approval state of the patch in a particular patch baseline. The approval state includes information about whether the patch is currently approved, due to be approved by a rule, explicitly approved, or explicitly rejected and the date the patch was or will be approved.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EffectivePatch {
    /// <p>Provides metadata for a patch, including information such as the KB ID, severity, classification and a URL for where more information can be obtained about the patch.</p>
    #[serde(rename = "Patch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Patch>,
    /// <p>The status of the patch in a patch baseline. This includes information about whether the patch is currently approved, due to be approved by a rule, explicitly approved, or explicitly rejected and the date the patch was or will be approved.</p>
    #[serde(rename = "PatchStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_status: Option<PatchStatus>,
}

/// <p>Describes a failed association.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailedCreateAssociation {
    /// <p>The association.</p>
    #[serde(rename = "Entry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<CreateAssociationBatchRequestEntry>,
    /// <p>The source of the failure.</p>
    #[serde(rename = "Fault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault: Option<String>,
    /// <p>A description of the failure.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>Information about an Automation failure.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailureDetails {
    /// <p>Detailed information about the Automation step failure.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The stage of the Automation execution when the failure occurred. The stages include the following: InputValidation, PreVerification, Invocation, PostVerification.</p>
    #[serde(rename = "FailureStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_stage: Option<String>,
    /// <p>The type of Automation failure. Failure types include the following: Action, Permission, Throttling, Verification, Internal.</p>
    #[serde(rename = "FailureType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAutomationExecutionRequest {
    /// <p>The unique identifier for an existing automation execution to examine. The execution ID is returned by StartAutomationExecution when the execution of an Automation document is initiated.</p>
    #[serde(rename = "AutomationExecutionId")]
    pub automation_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAutomationExecutionResult {
    /// <p>Detailed information about the current state of an automation execution.</p>
    #[serde(rename = "AutomationExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution: Option<AutomationExecution>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCommandInvocationRequest {
    /// <p>(Required) The parent command ID of the invocation plugin.</p>
    #[serde(rename = "CommandId")]
    pub command_id: String,
    /// <p>(Required) The ID of the managed instance targeted by the command. A managed instance can be an Amazon EC2 instance or an instance in your hybrid environment that is configured for Systems Manager.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>(Optional) The name of the plugin for which you want detailed results. If the document contains only one plugin, the name can be omitted and the details will be returned.</p>
    #[serde(rename = "PluginName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCommandInvocationResult {
    /// <p>CloudWatch Logs information where Systems Manager sent the command output.</p>
    #[serde(rename = "CloudWatchOutputConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_config: Option<CloudWatchOutputConfig>,
    /// <p>The parent command ID of the invocation plugin.</p>
    #[serde(rename = "CommandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// <p>The comment text for the command.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The name of the document that was run. For example, AWS-RunShellScript.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>The SSM document version used in the request.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>Duration since ExecutionStartDateTime.</p>
    #[serde(rename = "ExecutionElapsedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_elapsed_time: Option<String>,
    /// <p>The date and time the plugin was finished running. Date and time are written in ISO 8601 format. For example, June 7, 2017 is represented as 2017-06-7. The following sample AWS CLI command uses the <code>InvokedAfter</code> filter.</p> <p> <code>aws ssm list-commands --filters key=InvokedAfter,value=2017-06-07T00:00:00Z</code> </p> <p>If the plugin has not started to run, the string is empty.</p>
    #[serde(rename = "ExecutionEndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_date_time: Option<String>,
    /// <p>The date and time the plugin started running. Date and time are written in ISO 8601 format. For example, June 7, 2017 is represented as 2017-06-7. The following sample AWS CLI command uses the <code>InvokedBefore</code> filter.</p> <p> <code>aws ssm list-commands --filters key=InvokedBefore,value=2017-06-07T00:00:00Z</code> </p> <p>If the plugin has not started to run, the string is empty.</p>
    #[serde(rename = "ExecutionStartDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_date_time: Option<String>,
    /// <p>The ID of the managed instance targeted by the command. A managed instance can be an Amazon EC2 instance or an instance in your hybrid environment that is configured for Systems Manager.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the plugin for which you want detailed results. For example, aws:RunShellScript is a plugin.</p>
    #[serde(rename = "PluginName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,
    /// <p>The error level response code for the plugin script. If the response code is -1, then the command has not started running on the instance, or it was not received by the instance.</p>
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i64>,
    /// <p>The first 8,000 characters written by the plugin to stderr. If the command has not finished running, then this string is empty.</p>
    #[serde(rename = "StandardErrorContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_content: Option<String>,
    /// <p>The URL for the complete text written by the plugin to stderr. If the command has not finished running, then this string is empty.</p>
    #[serde(rename = "StandardErrorUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_url: Option<String>,
    /// <p>The first 24,000 characters written by the plugin to stdout. If the command has not finished running, if ExecutionStatus is neither Succeeded nor Failed, then this string is empty.</p>
    #[serde(rename = "StandardOutputContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_content: Option<String>,
    /// <p>The URL for the complete text written by the plugin to stdout in Amazon S3. If an Amazon S3 bucket was not specified, then this string is empty.</p>
    #[serde(rename = "StandardOutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_url: Option<String>,
    /// <p>The status of this invocation plugin. This status can be different than StatusDetails.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>A detailed status of the command execution for an invocation. StatusDetails includes more information than Status because it includes states resulting from error and concurrency control parameters. StatusDetails can show different results than Status. For more information about these statuses, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-commands.html">Understanding Command Statuses</a> in the <i>AWS Systems Manager User Guide</i>. StatusDetails can be one of the following values:</p> <ul> <li> <p>Pending: The command has not been sent to the instance.</p> </li> <li> <p>In Progress: The command has been sent to the instance but has not reached a terminal state.</p> </li> <li> <p>Delayed: The system attempted to send the command to the target, but the target was not available. The instance might not be available because of network issues, the instance was stopped, etc. The system will try to deliver the command again.</p> </li> <li> <p>Success: The command or plugin was run successfully. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out: The command was not delivered to the instance before the delivery timeout expired. Delivery timeouts do not count against the parent command&#39;s MaxErrors limit, but they do contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Execution Timed Out: The command started to run on the instance, but the execution was not complete before the timeout expired. Execution timeouts count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Failed: The command wasn&#39;t run successfully on the instance. For a plugin, this indicates that the result code was not zero. For a command invocation, this indicates that the result code for one or more plugins was not zero. Invocation failures count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Canceled: The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Undeliverable: The command can&#39;t be delivered to the instance. The instance might not exist or might not be responding. Undeliverable invocations don&#39;t count against the parent command&#39;s MaxErrors limit and don&#39;t contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Terminated: The parent command exceeded its MaxErrors limit and subsequent command invocations were canceled by the system. This is a terminal state.</p> </li> </ul></p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectionStatusRequest {
    /// <p>The ID of the instance.</p>
    #[serde(rename = "Target")]
    pub target: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConnectionStatusResponse {
    /// <p>The status of the connection to the instance. For example, 'Connected' or 'Not Connected'.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The ID of the instance to check connection status. </p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDefaultPatchBaselineRequest {
    /// <p>Returns the default patch baseline for the specified operating system.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDefaultPatchBaselineResult {
    /// <p>The ID of the default patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The operating system for the returned patch baseline. </p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeployablePatchSnapshotForInstanceRequest {
    /// <p>The ID of the instance for which the appropriate patch snapshot should be retrieved.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The user-defined snapshot ID.</p>
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeployablePatchSnapshotForInstanceResult {
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>Returns the specific operating system (for example Windows Server 2012 or Amazon Linux 2015.09) on the instance for the specified patch snapshot.</p>
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// <p>A pre-signed Amazon S3 URL that can be used to download the patch snapshot.</p>
    #[serde(rename = "SnapshotDownloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_download_url: Option<String>,
    /// <p>The user-defined snapshot ID.</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDocumentRequest {
    /// <p>Returns the document in the specified format. The document format can be either JSON or YAML. JSON is the default format.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The document version for which you want information.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An optional field specifying the version of the artifact associated with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document, and cannot be changed.</p>
    #[serde(rename = "VersionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDocumentResult {
    /// <p>A description of the document attachments, including names, locations, sizes, etc.</p>
    #[serde(rename = "AttachmentsContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments_content: Option<Vec<AttachmentContent>>,
    /// <p>The contents of the Systems Manager document.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The document format, either JSON or YAML.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The document type.</p>
    #[serde(rename = "DocumentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the Systems Manager document, such as <code>Creating</code>, <code>Active</code>, <code>Updating</code>, <code>Failed</code>, and <code>Deleting</code>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A message returned by AWS Systems Manager that explains the <code>Status</code> value. For example, a <code>Failed</code> status might be explained by the <code>StatusInformation</code> message, "The specified S3 bucket does not exist. Verify that the URL of the S3 bucket is correct."</p>
    #[serde(rename = "StatusInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_information: Option<String>,
    /// <p>The version of the artifact associated with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document, and cannot be changed.</p>
    #[serde(rename = "VersionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInventoryRequest {
    /// <p>Returns counts of inventory types based on one or more expressions. For example, if you aggregate by using an expression that uses the <code>AWS:InstanceInformation.PlatformType</code> type, you can see a count of how many Windows and Linux instances exist in your inventoried fleet.</p>
    #[serde(rename = "Aggregators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<InventoryAggregator>>,
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InventoryFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of inventory item types to return.</p>
    #[serde(rename = "ResultAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_attributes: Option<Vec<ResultAttribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInventoryResult {
    /// <p>Collection of inventory entities such as a collection of instance inventory. </p>
    #[serde(rename = "Entities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<InventoryResultEntity>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInventorySchemaRequest {
    /// <p>Returns inventory schemas that support aggregation. For example, this call returns the <code>AWS:InstanceInformation</code> type, because it supports aggregation based on the <code>PlatformName</code>, <code>PlatformType</code>, and <code>PlatformVersion</code> attributes.</p>
    #[serde(rename = "Aggregator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator: Option<bool>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns the sub-type schema for a specified inventory type.</p>
    #[serde(rename = "SubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<bool>,
    /// <p>The type of inventory item to return.</p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInventorySchemaResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Inventory schemas returned by the request.</p>
    #[serde(rename = "Schemas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<InventoryItemSchema>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMaintenanceWindowExecutionRequest {
    /// <p>The ID of the maintenance window execution that includes the task.</p>
    #[serde(rename = "WindowExecutionId")]
    pub window_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMaintenanceWindowExecutionResult {
    /// <p>The time the maintenance window finished running.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The time the maintenance window started running.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the maintenance window execution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the Status. Only available for certain status values.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The ID of the task executions from the maintenance window execution.</p>
    #[serde(rename = "TaskIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_ids: Option<Vec<String>>,
    /// <p>The ID of the maintenance window execution.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMaintenanceWindowExecutionTaskInvocationRequest {
    /// <p>The invocation ID to retrieve.</p>
    #[serde(rename = "InvocationId")]
    pub invocation_id: String,
    /// <p>The ID of the specific task in the maintenance window task that should be retrieved. </p>
    #[serde(rename = "TaskId")]
    pub task_id: String,
    /// <p>The ID of the maintenance window execution for which the task is a part.</p>
    #[serde(rename = "WindowExecutionId")]
    pub window_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMaintenanceWindowExecutionTaskInvocationResult {
    /// <p>The time that the task finished running on the target.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The execution ID.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// <p>The invocation ID.</p>
    #[serde(rename = "InvocationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    /// <p>User-provided value to be included in any CloudWatch events raised while running tasks for these targets in this maintenance window. </p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The parameters used at the time that the task ran.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    /// <p>The time that the task started running on the target.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The task status for an invocation.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the status. Details are only available for certain status values.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The task execution ID.</p>
    #[serde(rename = "TaskExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    /// <p>Retrieves the task type for a maintenance window. Task types include the following: LAMBDA, STEP_FUNCTIONS, AUTOMATION, RUN_COMMAND.</p>
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// <p>The maintenance window execution ID.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
    /// <p>The maintenance window target ID.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMaintenanceWindowExecutionTaskRequest {
    /// <p>The ID of the specific task execution in the maintenance window task that should be retrieved.</p>
    #[serde(rename = "TaskId")]
    pub task_id: String,
    /// <p>The ID of the maintenance window execution that includes the task.</p>
    #[serde(rename = "WindowExecutionId")]
    pub window_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMaintenanceWindowExecutionTaskResult {
    /// <p>The time the task execution completed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The defined maximum number of task executions that could be run in parallel.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The defined maximum number of task execution errors allowed before scheduling of the task execution would have been stopped.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The priority of the task.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The role that was assumed when running the task.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The time the task execution started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the task.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the Status. Only available for certain status values.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The ARN of the task that ran.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The ID of the specific task execution in the maintenance window task that was retrieved.</p>
    #[serde(rename = "TaskExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    /// <p>The parameters passed to the task when it was run.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note> <p>The map has the following format:</p> <p>Key: string, between 1 and 255 characters</p> <p>Value: an array of strings, each string is between 1 and 255 characters</p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters: Option<
        Vec<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    >,
    /// <p>The type of task that was run.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The ID of the maintenance window execution that includes the task.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMaintenanceWindowRequest {
    /// <p>The ID of the maintenance window for which you want to retrieve information.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMaintenanceWindowResult {
    /// <p>Whether targets must be registered with the maintenance window before tasks can be defined for those targets.</p>
    #[serde(rename = "AllowUnassociatedTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unassociated_targets: Option<bool>,
    /// <p>The date the maintenance window was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The number of hours before the end of the maintenance window that Systems Manager stops scheduling new tasks for execution.</p>
    #[serde(rename = "Cutoff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i64>,
    /// <p>The description of the maintenance window.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The duration of the maintenance window in hours.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Indicates whether the maintenance window is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The date and time, in ISO-8601 Extended format, for when the maintenance window is scheduled to become inactive. The maintenance window will not run after this specified time.</p>
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// <p>The date the maintenance window was last modified.</p>
    #[serde(rename = "ModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<f64>,
    /// <p>The name of the maintenance window.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The next time the maintenance window will actually run, taking into account any specified times for the maintenance window to become active or inactive.</p>
    #[serde(rename = "NextExecutionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_execution_time: Option<String>,
    /// <p>The schedule of the maintenance window in the form of a cron or rate expression.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The time zone that the scheduled maintenance window executions are based on, in Internet Assigned Numbers Authority (IANA) format. For example: "America/Los_Angeles", "etc/UTC", or "Asia/Seoul". For more information, see the <a href="https://www.iana.org/time-zones">Time Zone Database</a> on the IANA website.</p>
    #[serde(rename = "ScheduleTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_timezone: Option<String>,
    /// <p>The date and time, in ISO-8601 Extended format, for when the maintenance window is scheduled to become active. The maintenance window will not run before this specified time.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// <p>The ID of the created maintenance window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMaintenanceWindowTaskRequest {
    /// <p>The maintenance window ID that includes the task to retrieve.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// <p>The maintenance window task ID to retrieve.</p>
    #[serde(rename = "WindowTaskId")]
    pub window_task_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMaintenanceWindowTaskResult {
    /// <p>The retrieved task description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The location in Amazon S3 where the task results are logged.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "LoggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <p>The maximum number of targets allowed to run this task in parallel.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The maximum number of errors allowed before the task stops being scheduled.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The retrieved task name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The priority of the task when it runs. The lower the number, the higher the priority. Tasks that have the same priority are scheduled in parallel.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The ARN of the IAM service role to use to publish Amazon Simple Notification Service (Amazon SNS) notifications for maintenance window Run Command tasks.</p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The targets where the task should run.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The resource that the task used during execution. For RUN_COMMAND and AUTOMATION task types, the TaskArn is the Systems Manager Document name/ARN. For LAMBDA tasks, the value is the function name/ARN. For STEP_FUNCTIONS tasks, the value is the state machine ARN.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The parameters to pass to the task when it runs.</p>
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    /// <p><p>The parameters to pass to the task when it runs.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    /// <p>The type of task to run.</p>
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// <p>The retrieved maintenance window ID.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The retrieved maintenance window task ID.</p>
    #[serde(rename = "WindowTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetOpsItemRequest {
    /// <p>The ID of the OpsItem that you want to get.</p>
    #[serde(rename = "OpsItemId")]
    pub ops_item_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOpsItemResponse {
    /// <p>The OpsItem.</p>
    #[serde(rename = "OpsItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item: Option<OpsItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetOpsSummaryRequest {
    /// <p>Optional aggregators that return counts of OpsItems based on one or more expressions.</p>
    #[serde(rename = "Aggregators")]
    pub aggregators: Vec<OpsAggregator>,
    /// <p>Optional filters used to scope down the returned OpsItems. </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<OpsFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOpsSummaryResult {
    /// <p>The list of aggregated and filtered OpsItems.</p>
    #[serde(rename = "Entities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<OpsEntity>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetParameterHistoryRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of a parameter you want to query.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Return decrypted values for secure string parameters. This flag is ignored for String and StringList parameter types.</p>
    #[serde(rename = "WithDecryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetParameterHistoryResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of parameters returned by the request.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterHistory>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetParameterRequest {
    /// <p>The name of the parameter you want to query.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Return decrypted values for secure string parameters. This flag is ignored for String and StringList parameter types.</p>
    #[serde(rename = "WithDecryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetParameterResult {
    /// <p>Information about a parameter.</p>
    #[serde(rename = "Parameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<Parameter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetParametersByPathRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>Filters to limit the request results.</p> <note> <p>You can&#39;t filter using the parameter name.</p> </note></p>
    #[serde(rename = "ParameterFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_filters: Option<Vec<ParameterStringFilter>>,
    /// <p>The hierarchy for the parameter. Hierarchies start with a forward slash (/) and end with the parameter name. A parameter name hierarchy can have a maximum of 15 levels. Here is an example of a hierarchy: <code>/Finance/Prod/IAD/WinServ2016/license33</code> </p>
    #[serde(rename = "Path")]
    pub path: String,
    /// <p><p>Retrieve all parameters within a hierarchy.</p> <important> <p>If a user has access to a path, then the user can access all levels of that path. For example, if a user has permission to access path <code>/a</code>, then the user can also access <code>/a/b</code>. Even if a user has explicitly been denied access in IAM for parameter <code>/a/b</code>, they can still call the GetParametersByPath API action recursively for <code>/a</code> and view <code>/a/b</code>.</p> </important></p>
    #[serde(rename = "Recursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    /// <p>Retrieve all parameters in a hierarchy with their value decrypted.</p>
    #[serde(rename = "WithDecryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetParametersByPathResult {
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of parameters found in the specified hierarchy.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetParametersRequest {
    /// <p>Names of the parameters for which you want to query information.</p>
    #[serde(rename = "Names")]
    pub names: Vec<String>,
    /// <p>Return decrypted secure string value. Return decrypted values for secure string parameters. This flag is ignored for String and StringList parameter types.</p>
    #[serde(rename = "WithDecryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetParametersResult {
    /// <p>A list of parameters that are not formatted correctly or do not run during an execution.</p>
    #[serde(rename = "InvalidParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_parameters: Option<Vec<String>>,
    /// <p>A list of details for a parameter.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPatchBaselineForPatchGroupRequest {
    /// <p>Returns he operating system rule specified for patch groups using the patch baseline.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>The name of the patch group whose patch baseline should be retrieved.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPatchBaselineForPatchGroupResult {
    /// <p>The ID of the patch baseline that should be used for the patch group.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The operating system rule specified for patch groups using the patch baseline.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>The name of the patch group.</p>
    #[serde(rename = "PatchGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPatchBaselineRequest {
    /// <p>The ID of the patch baseline to retrieve.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPatchBaselineResult {
    /// <p>A set of rules used to include patches in the baseline.</p>
    #[serde(rename = "ApprovalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    /// <p>A list of explicitly approved patches for the baseline.</p>
    #[serde(rename = "ApprovedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    /// <p>Returns the specified compliance severity level for approved patches in the patch baseline.</p>
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    /// <p>Indicates whether the list of approved patches includes non-security updates that should be applied to the instances. The default value is 'false'. Applies to Linux instances only.</p>
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    /// <p>The ID of the retrieved patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The date the patch baseline was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the patch baseline.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A set of global filters used to exclude patches from the baseline.</p>
    #[serde(rename = "GlobalFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    /// <p>The date the patch baseline was last modified.</p>
    #[serde(rename = "ModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<f64>,
    /// <p>The name of the patch baseline.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Returns the operating system specified for the patch baseline.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>Patch groups included in the patch baseline.</p>
    #[serde(rename = "PatchGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_groups: Option<Vec<String>>,
    /// <p>A list of explicitly rejected patches for the baseline.</p>
    #[serde(rename = "RejectedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    /// <p>The action specified to take on patches included in the RejectedPatches list. A patch can be allowed only if it is a dependency of another package, or blocked entirely along with packages that include it as a dependency.</p>
    #[serde(rename = "RejectedPatchesAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches_action: Option<String>,
    /// <p>Information about the patches to use to update the instances, including target operating systems and source repositories. Applies to Linux instances only.</p>
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
}

/// <p>The request body of the GetServiceSetting API action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetServiceSettingRequest {
    /// <p>The ID of the service setting to get.</p>
    #[serde(rename = "SettingId")]
    pub setting_id: String,
}

/// <p>The query result body of the GetServiceSetting API action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetServiceSettingResult {
    /// <p>The query result of the current service setting.</p>
    #[serde(rename = "ServiceSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_setting: Option<ServiceSetting>,
}

/// <p>Status information about the aggregated associations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceAggregatedAssociationOverview {
    /// <p>Detailed status information about the aggregated associations.</p>
    #[serde(rename = "DetailedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    /// <p>The number of associations for the instance(s).</p>
    #[serde(rename = "InstanceAssociationStatusAggregatedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_association_status_aggregated_count:
        Option<::std::collections::HashMap<String, i64>>,
}

/// <p>One or more association documents on the instance. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceAssociation {
    /// <p>The association ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>Version information for the association on the instance.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>The content of the association document for the instance(s).</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

/// <p>An Amazon S3 bucket where you want to store the results of this request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceAssociationOutputLocation {
    /// <p>An Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "S3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<S3OutputLocation>,
}

/// <p>The URL of Amazon S3 bucket where you want to store the results of this request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceAssociationOutputUrl {
    /// <p>The URL of Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "S3OutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_output_url: Option<S3OutputUrl>,
}

/// <p>Status information about the instance association.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceAssociationStatusInfo {
    /// <p>The association ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The name of the association applied to the instance.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>The version of the association applied to the instance.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>Detailed status information about the instance association.</p>
    #[serde(rename = "DetailedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    /// <p>The association document versions.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>An error code returned by the request to create the association.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The date the instance association ran. </p>
    #[serde(rename = "ExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_date: Option<f64>,
    /// <p>Summary information about association execution.</p>
    #[serde(rename = "ExecutionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary: Option<String>,
    /// <p>The instance ID where the association was created.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the association.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A URL for an Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "OutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_url: Option<InstanceAssociationOutputUrl>,
    /// <p>Status information about the instance association.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes a filter for a specific list of instances. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceInformation {
    /// <p>The activation ID created by Systems Manager when the server or VM was registered.</p>
    #[serde(rename = "ActivationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
    /// <p>The version of SSM Agent running on your Linux instance. </p>
    #[serde(rename = "AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>Information about the association.</p>
    #[serde(rename = "AssociationOverview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_overview: Option<InstanceAggregatedAssociationOverview>,
    /// <p>The status of the association.</p>
    #[serde(rename = "AssociationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    /// <p>The fully qualified host name of the managed instance.</p>
    #[serde(rename = "ComputerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// <p>The IP address of the managed instance.</p>
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>The Amazon Identity and Access Management (IAM) role assigned to the on-premises Systems Manager managed instances. This call does not return the IAM role for Amazon EC2 instances. </p>
    #[serde(rename = "IamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    /// <p>The instance ID. </p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>Indicates whether latest version of SSM Agent is running on your instance. Some older versions of Windows Server use the EC2Config service to process SSM requests. For this reason, this field does not indicate whether or not the latest version is installed on Windows managed instances.</p>
    #[serde(rename = "IsLatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_latest_version: Option<bool>,
    /// <p>The date the association was last run.</p>
    #[serde(rename = "LastAssociationExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_association_execution_date: Option<f64>,
    /// <p>The date and time when agent last pinged Systems Manager service. </p>
    #[serde(rename = "LastPingDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ping_date_time: Option<f64>,
    /// <p>The last date the association was successfully run.</p>
    #[serde(rename = "LastSuccessfulAssociationExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_association_execution_date: Option<f64>,
    /// <p>The name of the managed instance.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Connection status of SSM Agent. </p>
    #[serde(rename = "PingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_status: Option<String>,
    /// <p>The name of the operating system platform running on your instance. </p>
    #[serde(rename = "PlatformName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    /// <p>The operating system platform type. </p>
    #[serde(rename = "PlatformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<String>,
    /// <p>The version of the OS platform running on your instance. </p>
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The date the server or VM was registered with AWS as a managed instance.</p>
    #[serde(rename = "RegistrationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<f64>,
    /// <p>The type of instance. Instances are either EC2 instances or managed instances. </p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Describes a filter for a specific list of instances. You can filter instances information by using tags. You specify tags by using a key-value mapping.</p> <p>Use this action instead of the <a>DescribeInstanceInformationRequest$InstanceInformationFilterList</a> method. The <code>InstanceInformationFilterList</code> method is a legacy method and does not support tags. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InstanceInformationFilter {
    /// <p>The name of the filter. </p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The filter values.</p>
    #[serde(rename = "valueSet")]
    pub value_set: Vec<String>,
}

/// <p>The filters to describe or get information about your managed instances.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InstanceInformationStringFilter {
    /// <p>The filter key name to describe your instances. For example:</p> <p>"InstanceIds"|"AgentVersion"|"PingStatus"|"PlatformTypes"|"ActivationIds"|"IamRole"|"ResourceType"|"AssociationStatus"|"Tag Key"</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The filter values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Defines the high-level patch compliance state for a managed instance, providing information about the number of installed, missing, not applicable, and failed patches along with metadata about the operation when this information was gathered for the instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstancePatchState {
    /// <p>The ID of the patch baseline used to patch the instance.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
    /// <p>The number of patches from the patch baseline that were attempted to be installed during the last patching operation, but failed to install.</p>
    #[serde(rename = "FailedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i64>,
    /// <p>An https URL or an Amazon S3 path-style URL to a list of patches to be installed. This patch installation list, which you maintain in an Amazon S3 bucket in YAML format and specify in the SSM document <code>AWS-RunPatchBaseline</code>, overrides the patches specified by the default patch baseline.</p> <p>For more information about the <code>InstallOverrideList</code> parameter, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/patch-manager-about-aws-runpatchbaseline.html">About the SSM Document AWS-RunPatchBaseline</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "InstallOverrideList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_override_list: Option<String>,
    /// <p>The number of patches from the patch baseline that are installed on the instance.</p>
    #[serde(rename = "InstalledCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_count: Option<i64>,
    /// <p>The number of patches not specified in the patch baseline that are installed on the instance.</p>
    #[serde(rename = "InstalledOtherCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_other_count: Option<i64>,
    /// <p><p>The number of instances with patches installed that are specified in a RejectedPatches list. Patches with a status of <i>InstalledRejected</i> were typically installed before they were added to a RejectedPatches list.</p> <note> <p>If ALLOW<em>AS</em>DEPENDENCY is the specified option for RejectedPatchesAction, the value of InstalledRejectedCount will always be 0 (zero).</p> </note></p>
    #[serde(rename = "InstalledRejectedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_rejected_count: Option<i64>,
    /// <p>The ID of the managed instance the high-level patch compliance information was collected for.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The number of patches from the patch baseline that are applicable for the instance but aren't currently installed.</p>
    #[serde(rename = "MissingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_count: Option<i64>,
    /// <p>The number of patches from the patch baseline that aren't applicable for the instance and therefore aren't installed on the instance. This number may be truncated if the list of patch names is very large. The number of patches beyond this limit are reported in <code>UnreportedNotApplicableCount</code>.</p>
    #[serde(rename = "NotApplicableCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_count: Option<i64>,
    /// <p>The type of patching operation that was performed: SCAN (assess patch compliance state) or INSTALL (install missing patches).</p>
    #[serde(rename = "Operation")]
    pub operation: String,
    /// <p>The time the most recent patching operation completed on the instance.</p>
    #[serde(rename = "OperationEndTime")]
    pub operation_end_time: f64,
    /// <p>The time the most recent patching operation was started on the instance.</p>
    #[serde(rename = "OperationStartTime")]
    pub operation_start_time: f64,
    /// <p>Placeholder information. This field will always be empty in the current release of the service.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The name of the patch group the managed instance belongs to.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
    /// <p>The ID of the patch baseline snapshot used during the patching operation when this compliance data was collected.</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The number of patches beyond the supported limit of <code>NotApplicableCount</code> that are not reported by name to Systems Manager Inventory.</p>
    #[serde(rename = "UnreportedNotApplicableCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreported_not_applicable_count: Option<i64>,
}

/// <p>Defines a filter used in DescribeInstancePatchStatesForPatchGroup used to scope down the information returned by the API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InstancePatchStateFilter {
    /// <p>The key for the filter. Supported values are FailedCount, InstalledCount, InstalledOtherCount, MissingCount and NotApplicableCount.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The type of comparison that should be performed for the value: Equal, NotEqual, LessThan or GreaterThan.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The value for the filter, must be an integer greater than or equal to 0.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Specifies the inventory type and attribute for the aggregation execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryAggregator {
    /// <p>Nested aggregators to further refine aggregation for an inventory type.</p>
    #[serde(rename = "Aggregators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<InventoryAggregator>>,
    /// <p>The inventory type and attribute name for aggregation.</p>
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>A user-defined set of one or more filters on which to aggregate inventory data. Groups return a count of resources that match and don't match the specified criteria.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<InventoryGroup>>,
}

/// <p>Status information returned by the <code>DeleteInventory</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InventoryDeletionStatusItem {
    /// <p>The deletion ID returned by the <code>DeleteInventory</code> action.</p>
    #[serde(rename = "DeletionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_id: Option<String>,
    /// <p>The UTC timestamp when the delete operation started.</p>
    #[serde(rename = "DeletionStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_start_time: Option<f64>,
    /// <p>Information about the delete operation. For more information about this summary, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-inventory-custom.html#sysman-inventory-delete">Understanding the Delete Inventory Summary</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "DeletionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_summary: Option<InventoryDeletionSummary>,
    /// <p>The status of the operation. Possible values are InProgress and Complete.</p>
    #[serde(rename = "LastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>Information about the status.</p>
    #[serde(rename = "LastStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_message: Option<String>,
    /// <p>The UTC timestamp of when the last status report.</p>
    #[serde(rename = "LastStatusUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_update_time: Option<f64>,
    /// <p>The name of the inventory data type.</p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

/// <p>Information about the delete operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InventoryDeletionSummary {
    /// <p>Remaining number of items to delete.</p>
    #[serde(rename = "RemainingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<i64>,
    /// <p>A list of counts and versions for deleted items.</p>
    #[serde(rename = "SummaryItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_items: Option<Vec<InventoryDeletionSummaryItem>>,
    /// <p>The total number of items to delete. This count does not change during the delete operation.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// <p>Either a count, remaining count, or a version number in a delete inventory summary.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InventoryDeletionSummaryItem {
    /// <p>A count of the number of deleted items.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The remaining number of items to delete.</p>
    #[serde(rename = "RemainingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<i64>,
    /// <p>The inventory type version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>One or more filters. Use a filter to return a more specific list of results.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryFilter {
    /// <p>The name of the filter key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The type of filter. Valid values include the following: "Equal"|"NotEqual"|"BeginWith"|"LessThan"|"GreaterThan"</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>Inventory filter values. Example: inventory filter where instance IDs are specified as values Key=AWS:InstanceInformation.InstanceId,Values= i-a12b3c4d5e6g, i-1a2b3c4d5e6,Type=Equal </p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>A user-defined set of one or more filters on which to aggregate inventory data. Groups return a count of resources that match and don't match the specified criteria.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryGroup {
    /// <p>Filters define the criteria for the group. The <code>matchingCount</code> field displays the number of resources that match the criteria. The <code>notMatchingCount</code> field displays the number of resources that don't match the criteria. </p>
    #[serde(rename = "Filters")]
    pub filters: Vec<InventoryFilter>,
    /// <p>The name of the group.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Information collected from managed instances based on your inventory policy document</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryItem {
    /// <p>The time the inventory information was collected.</p>
    #[serde(rename = "CaptureTime")]
    pub capture_time: String,
    /// <p>The inventory data of the inventory type.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<::std::collections::HashMap<String, String>>>,
    /// <p>MD5 hash of the inventory item type contents. The content hash is used to determine whether to update inventory information. The PutInventory API does not update the inventory item type contents if the MD5 hash has not changed since last update. </p>
    #[serde(rename = "ContentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    /// <p>A map of associated properties for a specified inventory type. For example, with this attribute, you can specify the <code>ExecutionId</code>, <code>ExecutionType</code>, <code>ComplianceType</code> properties of the <code>AWS:ComplianceItem</code> type.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>The schema version for the inventory item.</p>
    #[serde(rename = "SchemaVersion")]
    pub schema_version: String,
    /// <p>The name of the inventory type. Default inventory item type names start with AWS. Custom inventory type names will start with Custom. Default inventory item types include the following: AWS:AWSComponent, AWS:Application, AWS:InstanceInformation, AWS:Network, and AWS:WindowsUpdate.</p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
}

/// <p>Attributes are the entries within the inventory item content. It contains name and value.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InventoryItemAttribute {
    /// <p>The data type of the inventory item attribute. </p>
    #[serde(rename = "DataType")]
    pub data_type: String,
    /// <p>Name of the inventory item attribute.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>The inventory item schema definition. Users can use this to compose inventory query filters.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InventoryItemSchema {
    /// <p>The schema attributes for inventory. This contains data type and attribute name.</p>
    #[serde(rename = "Attributes")]
    pub attributes: Vec<InventoryItemAttribute>,
    /// <p>The alias name of the inventory type. The alias name is used for display purposes.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the inventory type. Default inventory item type names start with AWS. Custom inventory type names will start with Custom. Default inventory item types include the following: AWS:AWSComponent, AWS:Application, AWS:InstanceInformation, AWS:Network, and AWS:WindowsUpdate.</p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
    /// <p>The schema version for the inventory item.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Inventory query results.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InventoryResultEntity {
    /// <p>The data section in the inventory result entity JSON.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, InventoryResultItem>>,
    /// <p>ID of the inventory result entity. For example, for managed instance inventory the result will be the managed instance ID. For EC2 instance inventory, the result will be the instance ID. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The inventory result item.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InventoryResultItem {
    /// <p>The time inventory item data was captured.</p>
    #[serde(rename = "CaptureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_time: Option<String>,
    /// <p>Contains all the inventory data of the item type. Results include attribute names and values. </p>
    #[serde(rename = "Content")]
    pub content: Vec<::std::collections::HashMap<String, String>>,
    /// <p>MD5 hash of the inventory item type contents. The content hash is used to determine whether to update inventory information. The PutInventory API does not update the inventory item type contents if the MD5 hash has not changed since last update. </p>
    #[serde(rename = "ContentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    /// <p>The schema version for the inventory result item/</p>
    #[serde(rename = "SchemaVersion")]
    pub schema_version: String,
    /// <p>The name of the inventory result item type.</p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LabelParameterVersionRequest {
    /// <p>One or more labels to attach to the specified parameter version.</p>
    #[serde(rename = "Labels")]
    pub labels: Vec<String>,
    /// <p>The parameter name on which you want to attach one or more labels.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The specific version of the parameter on which you want to attach one or more labels. If no version is specified, the system attaches the label to the latest version.)</p>
    #[serde(rename = "ParameterVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LabelParameterVersionResult {
    /// <p>The label does not meet the requirements. For information about parameter label requirements, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-labels.html">Labeling Parameters</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "InvalidLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_labels: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssociationVersionsRequest {
    /// <p>The association ID for which you want to view all versions.</p>
    #[serde(rename = "AssociationId")]
    pub association_id: String,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAssociationVersionsResult {
    /// <p>Information about all versions of the association for the specified association ID.</p>
    #[serde(rename = "AssociationVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_versions: Option<Vec<AssociationVersionInfo>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssociationsRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "AssociationFilterList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_filter_list: Option<Vec<AssociationFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAssociationsResult {
    /// <p>The associations.</p>
    #[serde(rename = "Associations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<Association>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCommandInvocationsRequest {
    /// <p>(Optional) The invocations for a specific command ID.</p>
    #[serde(rename = "CommandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// <p>(Optional) If set this returns the response of the command executions and any command output. By default this is set to False. </p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<bool>,
    /// <p>(Optional) One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<CommandFilter>>,
    /// <p>(Optional) The command execution details for a specific instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>(Optional) The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCommandInvocationsResult {
    /// <p>(Optional) A list of all invocations. </p>
    #[serde(rename = "CommandInvocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_invocations: Option<Vec<CommandInvocation>>,
    /// <p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCommandsRequest {
    /// <p>(Optional) If provided, lists only the specified command.</p>
    #[serde(rename = "CommandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// <p>(Optional) One or more filters. Use a filter to return a more specific list of results. </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<CommandFilter>>,
    /// <p>(Optional) Lists commands issued against this instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>(Optional) The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCommandsResult {
    /// <p>(Optional) The list of commands requested by the user. </p>
    #[serde(rename = "Commands")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<Command>>,
    /// <p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListComplianceItemsRequest {
    /// <p>One or more compliance filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ComplianceStringFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID for the resources from which to get compliance information. Currently, you can only specify one resource ID.</p>
    #[serde(rename = "ResourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    /// <p>The type of resource from which to get compliance information. Currently, the only supported resource type is <code>ManagedInstance</code>.</p>
    #[serde(rename = "ResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListComplianceItemsResult {
    /// <p>A list of compliance information for the specified resource ID. </p>
    #[serde(rename = "ComplianceItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_items: Option<Vec<ComplianceItem>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListComplianceSummariesRequest {
    /// <p>One or more compliance or inventory filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ComplianceStringFilter>>,
    /// <p>The maximum number of items to return for this call. Currently, you can specify null or 50. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListComplianceSummariesResult {
    /// <p>A list of compliant and non-compliant summary counts based on compliance types. For example, this call returns State Manager associations, patches, or custom compliance types according to the filter criteria that you specified.</p>
    #[serde(rename = "ComplianceSummaryItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary_items: Option<Vec<ComplianceSummaryItem>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDocumentVersionsRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the document about which you want version information.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDocumentVersionsResult {
    /// <p>The document versions.</p>
    #[serde(rename = "DocumentVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_versions: Option<Vec<DocumentVersionInfo>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDocumentsRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "DocumentFilterList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_filter_list: Option<Vec<DocumentFilter>>,
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DocumentKeyValuesFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDocumentsResult {
    /// <p>The names of the Systems Manager documents.</p>
    #[serde(rename = "DocumentIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_identifiers: Option<Vec<DocumentIdentifier>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListInventoryEntriesRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InventoryFilter>>,
    /// <p>The instance ID for which you want inventory information.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of inventory item for which you want information.</p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInventoryEntriesResult {
    /// <p>The time that inventory information was collected for the instance(s).</p>
    #[serde(rename = "CaptureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_time: Option<String>,
    /// <p>A list of inventory items on the instance(s).</p>
    #[serde(rename = "Entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<::std::collections::HashMap<String, String>>>,
    /// <p>The instance ID targeted by the request to query inventory information.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The inventory schema version used by the instance(s).</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    /// <p>The type of inventory item returned by the request.</p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceComplianceSummariesRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ComplianceStringFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourceComplianceSummariesResult {
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A summary count for specified or targeted managed instances. Summary count includes information about compliant and non-compliant State Manager associations, patch status, or custom items according to the filter criteria that you specify. </p>
    #[serde(rename = "ResourceComplianceSummaryItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_compliance_summary_items: Option<Vec<ResourceComplianceSummaryItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceDataSyncRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourceDataSyncResult {
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of your current Resource Data Sync configurations and their statuses.</p>
    #[serde(rename = "ResourceDataSyncItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_data_sync_items: Option<Vec<ResourceDataSyncItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The resource ID for which you want to see a list of tags.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>Returns a list of tags for a specific resource type.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResult {
    /// <p>A list of tags.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

/// <p><p>Information about an Amazon S3 bucket to write instance-level logs to.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingInfo {
    /// <p>The name of an Amazon S3 bucket where execution logs are stored .</p>
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,
    /// <p>(Optional) The Amazon S3 bucket subfolder. </p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// <p>The region where the Amazon S3 bucket is located.</p>
    #[serde(rename = "S3Region")]
    pub s3_region: String,
}

/// <p>The parameters for an AUTOMATION task type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowAutomationParameters {
    /// <p>The version of an Automation document to use during task execution.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p><p>The parameters for the AUTOMATION task.</p> <p>For information about specifying and updating task parameters, see <a>RegisterTaskWithMaintenanceWindow</a> and <a>UpdateMaintenanceWindowTask</a>.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p>For AUTOMATION task types, Systems Manager ignores any values specified for these parameters.</p> </note></p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
}

/// <p>Describes the information about an execution of a maintenance window. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MaintenanceWindowExecution {
    /// <p>The time the execution finished.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The time the execution started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the execution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the Status. Only available for certain status values.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The ID of the maintenance window execution.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
    /// <p>The ID of the maintenance window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

/// <p>Information about a task execution performed as part of a maintenance window execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MaintenanceWindowExecutionTaskIdentity {
    /// <p>The time the task execution finished.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The time the task execution started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the task execution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the status of the task execution. Only available for certain status values.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The ARN of the task that ran.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The ID of the specific task execution in the maintenance window execution.</p>
    #[serde(rename = "TaskExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    /// <p>The type of task that ran.</p>
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// <p>The ID of the maintenance window execution that ran the task.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
}

/// <p>Describes the information about a task invocation for a particular target as part of a task execution performed as part of a maintenance window execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MaintenanceWindowExecutionTaskInvocationIdentity {
    /// <p>The time the invocation finished.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The ID of the action performed in the service that actually handled the task invocation. If the task type is RUN_COMMAND, this value is the command ID.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// <p>The ID of the task invocation.</p>
    #[serde(rename = "InvocationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    /// <p>User-provided value that was specified when the target was registered with the maintenance window. This was also included in any CloudWatch events raised during the task invocation.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The parameters that were provided for the invocation when it was run.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    /// <p>The time the invocation started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the task invocation.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the status of the task invocation. Only available for certain Status values. </p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The ID of the specific task execution in the maintenance window execution.</p>
    #[serde(rename = "TaskExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    /// <p>The task type.</p>
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// <p>The ID of the maintenance window execution that ran the task.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
    /// <p>The ID of the target definition in this maintenance window the invocation was performed for.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

/// <p>Filter used in the request. Supported filter keys are Name and Enabled.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MaintenanceWindowFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The filter values.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Information about the maintenance window.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MaintenanceWindowIdentity {
    /// <p>The number of hours before the end of the maintenance window that Systems Manager stops scheduling new tasks for execution.</p>
    #[serde(rename = "Cutoff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i64>,
    /// <p>A description of the maintenance window.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The duration of the maintenance window in hours.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Indicates whether the maintenance window is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The date and time, in ISO-8601 Extended format, for when the maintenance window is scheduled to become inactive.</p>
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// <p>The name of the maintenance window.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The next time the maintenance window will actually run, taking into account any specified times for the maintenance window to become active or inactive.</p>
    #[serde(rename = "NextExecutionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_execution_time: Option<String>,
    /// <p>The schedule of the maintenance window in the form of a cron or rate expression.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The time zone that the scheduled maintenance window executions are based on, in Internet Assigned Numbers Authority (IANA) format.</p>
    #[serde(rename = "ScheduleTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_timezone: Option<String>,
    /// <p>The date and time, in ISO-8601 Extended format, for when the maintenance window is scheduled to become active.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// <p>The ID of the maintenance window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

/// <p>The maintenance window to which the specified target belongs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MaintenanceWindowIdentityForTarget {
    /// <p>The name of the maintenance window.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the maintenance window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

/// <p><p>The parameters for a LAMBDA task type.</p> <p>For information about specifying and updating task parameters, see <a>RegisterTaskWithMaintenanceWindow</a> and <a>UpdateMaintenanceWindowTask</a>.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p>For Lambda tasks, Systems Manager ignores any values specified for TaskParameters and LoggingInfo.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowLambdaParameters {
    /// <p>Pass client-specific information to the Lambda function that you are invoking. You can then process the client information in your Lambda function as you choose through the context variable.</p>
    #[serde(rename = "ClientContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_context: Option<String>,
    /// <p>JSON to provide to your Lambda function as input.</p>
    #[serde(rename = "Payload")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<bytes::Bytes>,
    /// <p>(Optional) Specify a Lambda function version or alias name. If you specify a function version, the action uses the qualified function ARN to invoke a specific Lambda function. If you specify an alias name, the action uses the alias ARN to invoke the Lambda function version to which the alias points.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// <p><p>The parameters for a RUN_COMMAND task type.</p> <p>For information about specifying and updating task parameters, see <a>RegisterTaskWithMaintenanceWindow</a> and <a>UpdateMaintenanceWindowTask</a>.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p>For Run Command tasks, Systems Manager uses specified values for <code>TaskParameters</code> and <code>LoggingInfo</code> only if no values are specified for <code>TaskInvocationParameters</code>. </p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowRunCommandParameters {
    /// <p>Information about the commands to run.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The SHA-256 or SHA-1 hash created by the system when the document was created. SHA-1 hashes have been deprecated.</p>
    #[serde(rename = "DocumentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash: Option<String>,
    /// <p>SHA-256 or SHA-1. SHA-1 hashes have been deprecated.</p>
    #[serde(rename = "DocumentHashType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash_type: Option<String>,
    /// <p>Configurations for sending notifications about command status changes on a per-instance basis.</p>
    #[serde(rename = "NotificationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    /// <p>The name of the Amazon S3 bucket.</p>
    #[serde(rename = "OutputS3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    /// <p>The Amazon S3 bucket subfolder.</p>
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    /// <p>The parameters for the RUN_COMMAND task execution.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The ARN of the IAM service role to use to publish Amazon Simple Notification Service (Amazon SNS) notifications for maintenance window Run Command tasks.</p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>If this time is reached and the command has not already started running, it doesn't run.</p>
    #[serde(rename = "TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
}

/// <p><p>The parameters for a STEP_FUNCTIONS task.</p> <p>For information about specifying and updating task parameters, see <a>RegisterTaskWithMaintenanceWindow</a> and <a>UpdateMaintenanceWindowTask</a>.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p>For Step Functions tasks, Systems Manager ignores any values specified for <code>TaskParameters</code> and <code>LoggingInfo</code>.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowStepFunctionsParameters {
    /// <p>The inputs for the STEP_FUNCTIONS task.</p>
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The name of the STEP_FUNCTIONS task.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The target registered with the maintenance window.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MaintenanceWindowTarget {
    /// <p>A description for the target.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name for the maintenance window target.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A user-provided value that will be included in any CloudWatch events that are raised while running tasks for these targets in this maintenance window.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The type of target that is being registered with the maintenance window.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The targets, either instances or tags.</p> <p>Specify instances using the following format:</p> <p> <code>Key=instanceids,Values=&lt;instanceid1&gt;,&lt;instanceid2&gt;</code> </p> <p>Tags are specified using the following format:</p> <p> <code>Key=&lt;tag name&gt;,Values=&lt;tag value&gt;</code>.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The ID of the maintenance window to register the target with.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The ID of the target.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

/// <p>Information about a task defined for a maintenance window.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MaintenanceWindowTask {
    /// <p>A description of the task.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>Information about an Amazon S3 bucket to write task-level logs to.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "LoggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <p>The maximum number of targets this task can be run for, in parallel.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The maximum number of errors allowed before this task stops being scheduled.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The task name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The priority of the task in the maintenance window. The lower the number, the higher the priority. Tasks that have the same priority are scheduled in parallel.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The ARN of the IAM service role to use to publish Amazon Simple Notification Service (Amazon SNS) notifications for maintenance window Run Command tasks.</p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The targets (either instances or tags). Instances are specified using Key=instanceids,Values=&lt;instanceid1&gt;,&lt;instanceid2&gt;. Tags are specified using Key=&lt;tag name&gt;,Values=&lt;tag value&gt;.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The resource that the task uses during execution. For RUN_COMMAND and AUTOMATION task types, <code>TaskArn</code> is the Systems Manager document name or ARN. For LAMBDA tasks, it's the function name or ARN. For STEP_FUNCTIONS tasks, it's the state machine ARN.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p><p>The parameters that should be passed to the task when it is run.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    /// <p>The type of task. The type can be one of the following: RUN_COMMAND, AUTOMATION, LAMBDA, or STEP_FUNCTIONS.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The ID of the maintenance window where the task is registered.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The task ID.</p>
    #[serde(rename = "WindowTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

/// <p>The parameters for task execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowTaskInvocationParameters {
    /// <p>The parameters for an AUTOMATION task type.</p>
    #[serde(rename = "Automation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<MaintenanceWindowAutomationParameters>,
    /// <p>The parameters for a LAMBDA task type.</p>
    #[serde(rename = "Lambda")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<MaintenanceWindowLambdaParameters>,
    /// <p>The parameters for a RUN_COMMAND task type.</p>
    #[serde(rename = "RunCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_command: Option<MaintenanceWindowRunCommandParameters>,
    /// <p>The parameters for a STEP_FUNCTIONS task type.</p>
    #[serde(rename = "StepFunctions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_functions: Option<MaintenanceWindowStepFunctionsParameters>,
}

/// <p>Defines the values for a task parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowTaskParameterValueExpression {
    /// <p>This field contains an array of 0 or more strings, each 1 to 255 characters in length.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyDocumentPermissionRequest {
    /// <p>The AWS user accounts that should have access to the document. The account IDs can either be a group of account IDs or <i>All</i>.</p>
    #[serde(rename = "AccountIdsToAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids_to_add: Option<Vec<String>>,
    /// <p>The AWS user accounts that should no longer have access to the document. The AWS user account can either be a group of account IDs or <i>All</i>. This action has a higher priority than <i>AccountIdsToAdd</i>. If you specify an account ID to add and the same ID to remove, the system removes access to the document.</p>
    #[serde(rename = "AccountIdsToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids_to_remove: Option<Vec<String>>,
    /// <p>The name of the document that you want to share.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The permission type for the document. The permission type can be <i>Share</i>.</p>
    #[serde(rename = "PermissionType")]
    pub permission_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyDocumentPermissionResponse {}

/// <p>A summary of resources that are not compliant. The summary is organized according to resource type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NonCompliantSummary {
    /// <p>The total number of compliance items that are not compliant.</p>
    #[serde(rename = "NonCompliantCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_count: Option<i64>,
    /// <p>A summary of the non-compliance severity by compliance type</p>
    #[serde(rename = "SeveritySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_summary: Option<SeveritySummary>,
}

/// <p>Configurations for sending notifications.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// <p>An Amazon Resource Name (ARN) for an Amazon Simple Notification Service (Amazon SNS) topic. Run Command pushes notifications about command status changes to this topic.</p>
    #[serde(rename = "NotificationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arn: Option<String>,
    /// <p>The different events for which you can receive notifications. These events include the following: All (events), InProgress, Success, TimedOut, Cancelled, Failed. To learn more about these events, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/monitoring-sns-notifications.html">Configuring Amazon SNS Notifications for AWS Systems Manager</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "NotificationEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_events: Option<Vec<String>>,
    /// <p>Command: Receive notification when the status of a command changes. Invocation: For commands sent to multiple instances, receive notification on a per-instance basis when the status of a command changes. </p>
    #[serde(rename = "NotificationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<String>,
}

/// <p>One or more aggregators for viewing counts of OpsItems using different dimensions such as <code>Source</code>, <code>CreatedTime</code>, or <code>Source and CreatedTime</code>, to name a few.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct OpsAggregator {
    /// <p>Either a Range or Count aggregator for limiting an OpsItem summary.</p>
    #[serde(rename = "AggregatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator_type: Option<String>,
    /// <p>A nested aggregator for viewing counts of OpsItems.</p>
    #[serde(rename = "Aggregators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<OpsAggregator>>,
    /// <p>The name of an OpsItem attribute on which to limit the count of OpsItems.</p>
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The aggregator filters.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<OpsFilter>>,
    /// <p>The data type name to use for viewing counts of OpsItems.</p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    /// <p>The aggregator value.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The result of the query.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OpsEntity {
    /// <p>The data returned by the query.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, OpsEntityItem>>,
    /// <p>The query ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The OpsItem summaries result item.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OpsEntityItem {
    /// <p>The detailed data content for an OpsItem summaries result item.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<::std::collections::HashMap<String, String>>>,
}

/// <p>A filter for viewing OpsItem summaries.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct OpsFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The type of filter.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The filter value.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Operations engineers and IT professionals use OpsCenter to view, investigate, and remediate operational issues impacting the performance and health of their AWS resources. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter.html">AWS Systems Manager OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OpsItem {
    /// <p>The ARN of the AWS account that created the OpsItem.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The date and time the OpsItem was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The OpsItem description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the AWS account that last updated the OpsItem.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time the OpsItem was last updated.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of an SNS topic where notifications are sent when this OpsItem is edited or changed.</p>
    #[serde(rename = "Notifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<OpsItemNotification>>,
    /// <p>Operational data is custom data that provides useful reference details about the OpsItem. For example, you can specify log files, error strings, license keys, troubleshooting tips, or other relevant data. You enter operational data as key-value pairs. The key has a maximum length of 128 characters. The value has a maximum size of 20 KB.</p> <important> <p>Operational data keys <i>can't</i> begin with the following: amazon, aws, amzn, ssm, /amazon, /aws, /amzn, /ssm.</p> </important> <p>You can choose to make the data searchable by other users in the account or you can restrict search access. Searchable data means that all users with access to the OpsItem Overview page (as provided by the <a>DescribeOpsItems</a> API action) can view and search on the specified data. Operational data that is not searchable is only viewable by users who have access to the OpsItem (as provided by the <a>GetOpsItem</a> API action).</p> <p>Use the <code>/aws/resources</code> key in OperationalData to specify a related resource in the request. Use the <code>/aws/automations</code> key in OperationalData to associate an Automation runbook with the OpsItem. To view AWS CLI example commands that use these keys, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-creating-OpsItems.html#OpsCenter-manually-create-OpsItems">Creating OpsItems Manually</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "OperationalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_data: Option<::std::collections::HashMap<String, OpsItemDataValue>>,
    /// <p>The ID of the OpsItem.</p>
    #[serde(rename = "OpsItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_id: Option<String>,
    /// <p>The importance of this OpsItem in relation to other OpsItems in the system.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>One or more OpsItems that share something in common with the current OpsItem. For example, related OpsItems can include OpsItems with similar error messages, impacted resources, or statuses for the impacted resource.</p>
    #[serde(rename = "RelatedOpsItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_ops_items: Option<Vec<RelatedOpsItem>>,
    /// <p>The origin of the OpsItem, such as Amazon EC2 or AWS Systems Manager. The impacted resource is a subset of source.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The OpsItem status. Status can be <code>Open</code>, <code>In Progress</code>, or <code>Resolved</code>. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-working-with-OpsItems-editing-details.html">Editing OpsItem Details</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A short heading that describes the nature of the OpsItem and the impacted resource.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The version of this OpsItem. Each time the OpsItem is edited the version number increments by one.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>An object that defines the value of the key and its type in the OperationalData map.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpsItemDataValue {
    /// <p>The type of key-value pair. Valid types include <code>SearchableString</code> and <code>String</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value of the OperationalData key.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Describes an OpsItem filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct OpsItemFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The operator used by the filter call.</p>
    #[serde(rename = "Operator")]
    pub operator: String,
    /// <p>The filter value.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>A notification about the OpsItem.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpsItemNotification {
    /// <p>The Amazon Resource Name (ARN) of an SNS topic where notifications are sent when this OpsItem is edited or changed.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// <p>A count of OpsItems.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OpsItemSummary {
    /// <p>The Amazon Resource Name (ARN) of the IAM entity that created the OpsItem.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The date and time the OpsItem was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the IAM entity that created the OpsItem.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time the OpsItem was last updated.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>Operational data is custom data that provides useful reference details about the OpsItem. </p>
    #[serde(rename = "OperationalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_data: Option<::std::collections::HashMap<String, OpsItemDataValue>>,
    /// <p>The ID of the OpsItem.</p>
    #[serde(rename = "OpsItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_id: Option<String>,
    /// <p>The importance of this OpsItem in relation to other OpsItems in the system.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The impacted AWS resource.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The OpsItem status. Status can be <code>Open</code>, <code>In Progress</code>, or <code>Resolved</code>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A short heading that describes the nature of the OpsItem and the impacted resource.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>Information about the source where the association execution details are stored.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OutputSource {
    /// <p>The ID of the output source, for example the URL of an Amazon S3 bucket.</p>
    #[serde(rename = "OutputSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_source_id: Option<String>,
    /// <p>The type of source where the association execution details are stored, for example, Amazon S3.</p>
    #[serde(rename = "OutputSourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_source_type: Option<String>,
}

/// <p>An Amazon EC2 Systems Manager parameter in Parameter Store.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Parameter {
    /// <p>The Amazon Resource Name (ARN) of the parameter.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Date the parameter was last changed or updated and the parameter version was created.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The name of the parameter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Either the version number or the label used to retrieve the parameter value. Specify selectors by using one of the following formats:</p> <p>parameter_name:version</p> <p>parameter_name:label</p>
    #[serde(rename = "Selector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// <p>Applies to parameters that reference information in other AWS services. SourceResult is the raw result or response from the source.</p>
    #[serde(rename = "SourceResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_result: Option<String>,
    /// <p>The type of parameter. Valid values include the following: String, String list, Secure string.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The parameter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// <p>The parameter version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Information about parameter usage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ParameterHistory {
    /// <p>Parameter names can include the following letters and symbols.</p> <p>a-zA-Z0-9_.-</p>
    #[serde(rename = "AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    /// <p>Information about the parameter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the query key used for this parameter.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>Labels assigned to the parameter version.</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>Date the parameter was last changed or updated.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>Amazon Resource Name (ARN) of the AWS user who last changed the parameter.</p>
    #[serde(rename = "LastModifiedUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    /// <p>The name of the parameter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Information about the policies assigned to a parameter.</p> <p> <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-policies.html">Working with Parameter Policies</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<ParameterInlinePolicy>>,
    /// <p>The parameter tier.</p>
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// <p>The type of parameter used.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The parameter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// <p>The parameter version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>One or more policies assigned to a parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ParameterInlinePolicy {
    /// <p>The status of the policy. Policies report the following statuses: Pending (the policy has not been enforced or applied yet), Finished (the policy was applied), Failed (the policy was not applied), or InProgress (the policy is being applied now). </p>
    #[serde(rename = "PolicyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_status: Option<String>,
    /// <p>The JSON text of the policy.</p>
    #[serde(rename = "PolicyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
    /// <p>The type of policy. Parameter Store supports the following policy types: Expiration, ExpirationNotification, and NoChangeNotification. </p>
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

/// <p>Metadata includes information like the ARN of the last user and the date/time the parameter was last used.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ParameterMetadata {
    /// <p>A parameter name can include only the following letters and symbols.</p> <p>a-zA-Z0-9_.-</p>
    #[serde(rename = "AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    /// <p>Description of the parameter actions.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the query key used for this parameter.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>Date the parameter was last changed or updated.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>Amazon Resource Name (ARN) of the AWS user who last changed the parameter.</p>
    #[serde(rename = "LastModifiedUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    /// <p>The parameter name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of policies associated with a parameter.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<ParameterInlinePolicy>>,
    /// <p>The parameter tier.</p>
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// <p>The type of parameter. Valid parameter types include the following: String, String list, Secure string.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The parameter version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p><p>One or more filters. Use a filter to return a more specific list of results.</p> <note> <p>The <code>Name</code> and <code>Tier</code> filter keys can&#39;t be used with the <a>GetParametersByPath</a> API action. Also, the <code>Label</code> filter key can&#39;t be used with the <a>DescribeParameters</a> API action.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ParameterStringFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>Valid options are Equals and BeginsWith. For Path filter, valid options are Recursive and OneLevel.</p>
    #[serde(rename = "Option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
    /// <p>The value you want to search for.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>This data type is deprecated. Instead, use <a>ParameterStringFilter</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ParametersFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The filter values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Represents metadata about a patch.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Patch {
    /// <p>The classification of the patch (for example, SecurityUpdates, Updates, CriticalUpdates).</p>
    #[serde(rename = "Classification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// <p>The URL where more information can be obtained about the patch.</p>
    #[serde(rename = "ContentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    /// <p>The description of the patch.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the patch (this is different than the Microsoft Knowledge Base ID).</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The Microsoft Knowledge Base ID of the patch.</p>
    #[serde(rename = "KbNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kb_number: Option<String>,
    /// <p>The language of the patch if it's language-specific.</p>
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// <p>The ID of the MSRC bulletin the patch is related to.</p>
    #[serde(rename = "MsrcNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msrc_number: Option<String>,
    /// <p>The severity of the patch (for example Critical, Important, Moderate).</p>
    #[serde(rename = "MsrcSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msrc_severity: Option<String>,
    /// <p>The specific product the patch is applicable for (for example, WindowsServer2016).</p>
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// <p>The product family the patch is applicable for (for example, Windows).</p>
    #[serde(rename = "ProductFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_family: Option<String>,
    /// <p>The date the patch was released.</p>
    #[serde(rename = "ReleaseDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<f64>,
    /// <p>The title of the patch.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The name of the vendor providing the patch.</p>
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

/// <p>Defines the basic information about a patch baseline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PatchBaselineIdentity {
    /// <p>The description of the patch baseline.</p>
    #[serde(rename = "BaselineDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_description: Option<String>,
    /// <p>The ID of the patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The name of the patch baseline.</p>
    #[serde(rename = "BaselineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_name: Option<String>,
    /// <p>Whether this is the default baseline. Note that Systems Manager supports creating multiple default patch baselines. For example, you can create a default patch baseline for each operating system.</p>
    #[serde(rename = "DefaultBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_baseline: Option<bool>,
    /// <p>Defines the operating system the patch baseline applies to. The Default value is WINDOWS. </p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

/// <p>Information about the state of a patch on a particular instance as it relates to the patch baseline used to patch the instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PatchComplianceData {
    /// <p>The classification of the patch (for example, SecurityUpdates, Updates, CriticalUpdates).</p>
    #[serde(rename = "Classification")]
    pub classification: String,
    /// <p>The date/time the patch was installed on the instance. Note that not all operating systems provide this level of information.</p>
    #[serde(rename = "InstalledTime")]
    pub installed_time: f64,
    /// <p>The operating system-specific ID of the patch.</p>
    #[serde(rename = "KBId")]
    pub kb_id: String,
    /// <p>The severity of the patch (for example, Critical, Important, Moderate).</p>
    #[serde(rename = "Severity")]
    pub severity: String,
    /// <p>The state of the patch on the instance, such as INSTALLED or FAILED.</p> <p>For descriptions of each patch state, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-compliance-about.html#sysman-compliance-monitor-patch">About Patch Compliance</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "State")]
    pub state: String,
    /// <p>The title of the patch.</p>
    #[serde(rename = "Title")]
    pub title: String,
}

/// <p> Defines which patches should be included in a patch baseline.</p> <p>A patch filter consists of a key and a set of values. The filter key is a patch property. For example, the available filter keys for WINDOWS are PATCH_SET, PRODUCT, PRODUCT_FAMILY, CLASSIFICATION, and MSRC_SEVERITY. The filter values define a matching criterion for the patch property indicated by the key. For example, if the filter key is PRODUCT and the filter values are ["Office 2013", "Office 2016"], then the filter accepts all patches where product name is either "Office 2013" or "Office 2016". The filter values can be exact values for the patch property given as a key, or a wildcard (*), which matches all values.</p> <p>You can view lists of valid values for the patch properties by running the <code>DescribePatchProperties</code> command. For information about which patch properties can be used with each major operating system, see <a>DescribePatchProperties</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchFilter {
    /// <p>The key for the filter.</p> <p>Run the <a>DescribePatchProperties</a> command to view lists of valid keys for each operating system type.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value for the filter key.</p> <p>Run the <a>DescribePatchProperties</a> command to view lists of valid values for each key based on operating system type.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>A set of patch filters, typically used for approval rules.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchFilterGroup {
    /// <p>The set of patch filters that make up the group.</p>
    #[serde(rename = "PatchFilters")]
    pub patch_filters: Vec<PatchFilter>,
}

/// <p>The mapping between a patch group and the patch baseline the patch group is registered with.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PatchGroupPatchBaselineMapping {
    /// <p>The patch baseline the patch group is registered with.</p>
    #[serde(rename = "BaselineIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_identity: Option<PatchBaselineIdentity>,
    /// <p>The name of the patch group registered with the patch baseline.</p>
    #[serde(rename = "PatchGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

/// <p>Defines a filter used in Patch Manager APIs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PatchOrchestratorFilter {
    /// <p>The key for the filter.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value for the filter.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Defines an approval rule for a patch baseline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchRule {
    /// <p>The number of days after the release date of each patch matched by the rule that the patch is marked as approved in the patch baseline. For example, a value of <code>7</code> means that patches are approved seven days after they are released. </p>
    #[serde(rename = "ApproveAfterDays")]
    pub approve_after_days: i64,
    /// <p>A compliance severity level for all approved patches in a patch baseline. Valid compliance severity levels include the following: Unspecified, Critical, High, Medium, Low, and Informational.</p>
    #[serde(rename = "ComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_level: Option<String>,
    /// <p>For instances identified by the approval rule filters, enables a patch baseline to apply non-security updates available in the specified repository. The default value is 'false'. Applies to Linux instances only.</p>
    #[serde(rename = "EnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_non_security: Option<bool>,
    /// <p>The patch filter group that defines the criteria for the rule.</p>
    #[serde(rename = "PatchFilterGroup")]
    pub patch_filter_group: PatchFilterGroup,
}

/// <p>A set of rules defining the approval rules for a patch baseline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchRuleGroup {
    /// <p>The rules that make up the rule group.</p>
    #[serde(rename = "PatchRules")]
    pub patch_rules: Vec<PatchRule>,
}

/// <p>Information about the patches to use to update the instances, including target operating systems and source repository. Applies to Linux instances only.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchSource {
    /// <p>The value of the yum repo configuration. For example:</p> <p> <code>[main]</code> </p> <p> <code>cachedir=/var/cache/yum/$basesearch$releasever</code> </p> <p> <code>keepcache=0</code> </p> <p> <code>debuglevel=2</code> </p>
    #[serde(rename = "Configuration")]
    pub configuration: String,
    /// <p>The name specified to identify the patch source.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The specific operating system versions a patch repository applies to, such as "Ubuntu16.04", "AmazonLinux2016.09", "RedhatEnterpriseLinux7.2" or "Suse12.7". For lists of supported product values, see <a>PatchFilter</a>.</p>
    #[serde(rename = "Products")]
    pub products: Vec<String>,
}

/// <p>Information about the approval status of a patch.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PatchStatus {
    /// <p>The date the patch was approved (or will be approved if the status is PENDING_APPROVAL).</p>
    #[serde(rename = "ApprovalDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_date: Option<f64>,
    /// <p>The compliance severity level for a patch.</p>
    #[serde(rename = "ComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_level: Option<String>,
    /// <p>The approval status of a patch (APPROVED, PENDING_APPROVAL, EXPLICIT_APPROVED, EXPLICIT_REJECTED).</p>
    #[serde(rename = "DeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
}

/// <p>An aggregate of step execution statuses displayed in the AWS Console for a multi-Region and multi-account Automation execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProgressCounters {
    /// <p>The total number of steps that the system cancelled in all specified AWS Regions and accounts for the current Automation execution.</p>
    #[serde(rename = "CancelledSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_steps: Option<i64>,
    /// <p>The total number of steps that failed to run in all specified AWS Regions and accounts for the current Automation execution.</p>
    #[serde(rename = "FailedSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_steps: Option<i64>,
    /// <p>The total number of steps that successfully completed in all specified AWS Regions and accounts for the current Automation execution.</p>
    #[serde(rename = "SuccessSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_steps: Option<i64>,
    /// <p>The total number of steps that timed out in all specified AWS Regions and accounts for the current Automation execution.</p>
    #[serde(rename = "TimedOutSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_out_steps: Option<i64>,
    /// <p>The total number of steps run in all specified AWS Regions and accounts for the current Automation execution.</p>
    #[serde(rename = "TotalSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_steps: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutComplianceItemsRequest {
    /// <p>Specify the compliance type. For example, specify Association (for a State Manager association), Patch, or Custom:<code>string</code>.</p>
    #[serde(rename = "ComplianceType")]
    pub compliance_type: String,
    /// <p>A summary of the call execution that includes an execution ID, the type of execution (for example, <code>Command</code>), and the date/time of the execution using a datetime object that is saved in the following format: yyyy-MM-dd'T'HH:mm:ss'Z'.</p>
    #[serde(rename = "ExecutionSummary")]
    pub execution_summary: ComplianceExecutionSummary,
    /// <p>MD5 or SHA-256 content hash. The content hash is used to determine if existing information should be overwritten or ignored. If the content hashes match, the request to put compliance information is ignored.</p>
    #[serde(rename = "ItemContentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_content_hash: Option<String>,
    /// <p>Information about the compliance as defined by the resource type. For example, for a patch compliance type, <code>Items</code> includes information about the PatchSeverity, Classification, etc.</p>
    #[serde(rename = "Items")]
    pub items: Vec<ComplianceItemEntry>,
    /// <p>Specify an ID for this resource. For a managed instance, this is the instance ID.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>Specify the type of resource. <code>ManagedInstance</code> is currently the only supported resource type.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutComplianceItemsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutInventoryRequest {
    /// <p>One or more instance IDs where you want to add or update inventory items.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The inventory items that you want to add or update on instances.</p>
    #[serde(rename = "Items")]
    pub items: Vec<InventoryItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutInventoryResult {
    /// <p>Information about the request.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutParameterRequest {
    /// <p>A regular expression used to validate the parameter value. For example, for String types with values restricted to numbers, you can specify the following: AllowedPattern=^\d+$ </p>
    #[serde(rename = "AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    /// <p><p>Information about the parameter that you want to add to the system. Optional but recommended.</p> <important> <p>Do not enter personally identifiable information in this field.</p> </important></p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The KMS Key ID that you want to use to encrypt a parameter. Either the default AWS Key Management Service (AWS KMS) key automatically assigned to your AWS account or a custom key. Required for parameters that use the <code>SecureString</code> data type.</p> <p>If you don&#39;t specify a key ID, the system uses the default key associated with your AWS account.</p> <ul> <li> <p>To use your default AWS KMS key, choose the <code>SecureString</code> data type, and do <i>not</i> specify the <code>Key ID</code> when you create the parameter. The system automatically populates <code>Key ID</code> with your default KMS key.</p> </li> <li> <p>To use a custom KMS key, choose the <code>SecureString</code> data type with the <code>Key ID</code> parameter.</p> </li> </ul></p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p><p>The fully qualified name of the parameter that you want to add to the system. The fully qualified name includes the complete hierarchy of the parameter path and name. For example: <code>/Dev/DBServer/MySQL/db-string13</code> </p> <p>Naming Constraints:</p> <ul> <li> <p>Parameter names are case sensitive.</p> </li> <li> <p>A parameter name must be unique within an AWS Region</p> </li> <li> <p>A parameter name can&#39;t be prefixed with &quot;aws&quot; or &quot;ssm&quot; (case-insensitive).</p> </li> <li> <p>Parameter names can include only the following symbols and letters: <code>a-zA-Z0-9_.-/</code> </p> </li> <li> <p>A parameter name can&#39;t include spaces.</p> </li> <li> <p>Parameter hierarchies are limited to a maximum depth of fifteen levels.</p> </li> </ul> <p>For additional information about valid values for parameter names, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-parameter-name-constraints.html">Requirements and Constraints for Parameter Names</a> in the <i>AWS Systems Manager User Guide</i>.</p> <note> <p>The maximum length constraint listed below includes capacity for additional system attributes that are not part of the name. The maximum length for the fully qualified parameter name is 1011 characters. </p> </note></p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Overwrite an existing parameter. If not specified, will default to "false".</p>
    #[serde(rename = "Overwrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    /// <p>One or more policies to apply to a parameter. This action takes a JSON array. Parameter Store supports the following policy types:</p> <p>Expiration: This policy deletes the parameter after it expires. When you create the policy, you specify the expiration date. You can update the expiration date and time by updating the policy. Updating the <i>parameter</i> does not affect the expiration date and time. When the expiration time is reached, Parameter Store deletes the parameter.</p> <p>ExpirationNotification: This policy triggers an event in Amazon CloudWatch Events that notifies you about the expiration. By using this policy, you can receive notification before or after the expiration time is reached, in units of days or hours.</p> <p>NoChangeNotification: This policy triggers a CloudWatch event if a parameter has not been modified for a specified period of time. This policy type is useful when, for example, a secret needs to be changed within a period of time, but it has not been changed.</p> <p>All existing policies are preserved until you send new policies or an empty policy. For more information about parameter policies, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-su-policies.html">Working with Parameter Policies</a>. </p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<String>,
    /// <p><p>Optional metadata that you assign to a resource. Tags enable you to categorize a resource in different ways, such as by purpose, owner, or environment. For example, you might want to tag a Systems Manager parameter to identify the type of resource to which it applies, the environment, or the type of configuration data referenced by the parameter. In this case, you could specify the following key name/value pairs:</p> <ul> <li> <p> <code>Key=Resource,Value=S3bucket</code> </p> </li> <li> <p> <code>Key=OS,Value=Windows</code> </p> </li> <li> <p> <code>Key=ParameterType,Value=LicenseKey</code> </p> </li> </ul> <note> <p>To add tags to an existing Systems Manager parameter, use the <a>AddTagsToResource</a> action.</p> </note></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The parameter tier to assign to a parameter.</p> <p>Parameter Store offers a standard tier and an advanced tier for parameters. Standard parameters have a content size limit of 4 KB and can't be configured to use parameter policies. You can create a maximum of 10,000 standard parameters for each Region in an AWS account. Standard parameters are offered at no additional cost. </p> <p>Advanced parameters have a content size limit of 8 KB and can be configured to use parameter policies. You can create a maximum of 100,000 advanced parameters for each Region in an AWS account. Advanced parameters incur a charge. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-advanced-parameters.html">About Advanced Parameters</a> in the <i>AWS Systems Manager User Guide</i>.</p> <p>You can change a standard parameter to an advanced parameter any time. But you can't revert an advanced parameter to a standard parameter. Reverting an advanced parameter to a standard parameter would result in data loss because the system would truncate the size of the parameter from 8 KB to 4 KB. Reverting would also remove any policies attached to the parameter. Lastly, advanced parameters use a different form of encryption than standard parameters. </p> <p>If you no longer need an advanced parameter, or if you no longer want to incur charges for an advanced parameter, you must delete it and recreate it as a new standard parameter. </p> <p> <b>Using the Default Tier Configuration</b> </p> <p>In <code>PutParameter</code> requests, you can specify the tier to create the parameter in. Whenever you specify a tier in the request, Parameter Store creates or updates the parameter according to that request. However, if you do not specify a tier in a request, Parameter Store assigns the tier based on the current Parameter Store default tier configuration.</p> <p>The default tier when you begin using Parameter Store is the standard-parameter tier. If you use the advanced-parameter tier, you can specify one of the following as the default:</p> <ul> <li> <p> <b>Advanced</b>: With this option, Parameter Store evaluates all requests as advanced parameters. </p> </li> <li> <p> <b>Intelligent-Tiering</b>: With this option, Parameter Store evaluates each request to determine if the parameter is standard or advanced. </p> <p>If the request doesn't include any options that require an advanced parameter, the parameter is created in the standard-parameter tier. If one or more options requiring an advanced parameter are included in the request, Parameter Store create a parameter in the advanced-parameter tier.</p> <p>This approach helps control your parameter-related costs by always creating standard parameters unless an advanced parameter is necessary. </p> </li> </ul> <p>Options that require an advanced parameter include the following:</p> <ul> <li> <p>The content size of the parameter is more than 4 KB.</p> </li> <li> <p>The parameter uses a parameter policy.</p> </li> <li> <p>More than 10,000 parameters already exist in your AWS account in the current Region.</p> </li> </ul> <p>For more information about configuring the default tier option, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/ps-default-tier.html">Specifying a Default Parameter Tier</a> in the AWS Systems Manager User Guide.</p>
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// <p><p>The type of parameter that you want to add to the system.</p> <p>Items in a <code>StringList</code> must be separated by a comma (,). You can&#39;t use other punctuation or special character to escape items in the list. If you have a parameter value that requires a comma, then use the <code>String</code> data type.</p> <note> <p> <code>SecureString</code> is not currently supported for AWS CloudFormation templates or in the China Regions.</p> </note></p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The parameter value that you want to add to the system. Standard parameters have a value limit of 4 KB. Advanced parameters have a value limit of 8 KB.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutParameterResult {
    /// <p>The new version number of a parameter. If you edit a parameter value, Parameter Store automatically creates a new version and assigns this new version a unique ID. You can reference a parameter version ID in API actions or in Systems Manager documents (SSM documents). By default, if you don't specify a specific version, the system returns the latest parameter value when a parameter is called.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterDefaultPatchBaselineRequest {
    /// <p>The ID of the patch baseline that should be the default patch baseline.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterDefaultPatchBaselineResult {
    /// <p>The ID of the default patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterPatchBaselineForPatchGroupRequest {
    /// <p>The ID of the patch baseline to register the patch group with.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
    /// <p>The name of the patch group that should be registered with the patch baseline.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterPatchBaselineForPatchGroupResult {
    /// <p>The ID of the patch baseline the patch group was registered with.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The name of the patch group registered with the patch baseline.</p>
    #[serde(rename = "PatchGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterTargetWithMaintenanceWindowRequest {
    /// <p>User-provided idempotency token.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>An optional description for the target.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An optional name for the target.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this maintenance window.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The type of target being registered with the maintenance window.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>The targets to register with the maintenance window. In other words, the instances to run commands on when the maintenance window runs.</p> <p>You can specify targets using instance IDs, resource group names, or tags that have been applied to instances.</p> <p> <b>Example 1</b>: Specify instance IDs</p> <p> <code>Key=InstanceIds,Values=<i>instance-id-1</i>,<i>instance-id-2</i>,<i>instance-id-3</i> </code> </p> <p> <b>Example 2</b>: Use tag key-pairs applied to instances</p> <p> <code>Key=tag:<i>my-tag-key</i>,Values=<i>my-tag-value-1</i>,<i>my-tag-value-2</i> </code> </p> <p> <b>Example 3</b>: Use tag-keys applied to instances</p> <p> <code>Key=tag-key,Values=<i>my-tag-key-1</i>,<i>my-tag-key-2</i> </code> </p> <p> <b>Example 4</b>: Use resource group names</p> <p> <code>Key=resource-groups:Name,Values=<i>resource-group-name</i> </code> </p> <p> <b>Example 5</b>: Use filters for resource group types</p> <p> <code>Key=resource-groups:ResourceTypeFilters,Values=<i>resource-type-1</i>,<i>resource-type-2</i> </code> </p> <note> <p>For <code>Key=resource-groups:ResourceTypeFilters</code>, specify resource types in the following format</p> <p> <code>Key=resource-groups:ResourceTypeFilters,Values=<i>AWS::EC2::INSTANCE</i>,<i>AWS::EC2::VPC</i> </code> </p> </note> <p>For more information about these examples formats, including the best use case for each one, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/mw-cli-tutorial-targets-examples.html">Examples: Register Targets with a Maintenance Window</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,
    /// <p>The ID of the maintenance window the target should be registered with.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterTargetWithMaintenanceWindowResult {
    /// <p>The ID of the target definition in this maintenance window.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterTaskWithMaintenanceWindowRequest {
    /// <p>User-provided idempotency token.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>An optional description for the task.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>A structure containing information about an Amazon S3 bucket to write instance-level logs to. </p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "LoggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <p>The maximum number of targets this task can be run for in parallel.</p>
    #[serde(rename = "MaxConcurrency")]
    pub max_concurrency: String,
    /// <p>The maximum number of errors allowed before this task stops being scheduled.</p>
    #[serde(rename = "MaxErrors")]
    pub max_errors: String,
    /// <p>An optional name for the task.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The priority of the task in the maintenance window, the lower the number the higher the priority. Tasks in a maintenance window are scheduled in priority order with tasks that have the same priority scheduled in parallel.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p><p>The ARN of the IAM service role for Systems Manager to assume when running a maintenance window task. If you do not specify a service role ARN, Systems Manager uses your account&#39;s service-linked role. If no service-linked role for Systems Manager exists in your account, it is created when you run <code>RegisterTaskWithMaintenanceWindow</code>.</p> <p>For more information, see the following topics in the in the <i>AWS Systems Manager User Guide</i>:</p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/using-service-linked-roles.html#slr-permissions">Service-Linked Role Permissions for Systems Manager</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-maintenance-permissions.html#maintenance-window-tasks-service-role">Should I Use a Service-Linked Role or a Custom Service Role to Run Maintenance Window Tasks? </a> </p> </li> </ul></p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The targets (either instances or maintenance window targets).</p> <p>Specify instances using the following format: </p> <p> <code>Key=InstanceIds,Values=&lt;instance-id-1&gt;,&lt;instance-id-2&gt;</code> </p> <p>Specify maintenance window targets using the following format:</p> <p> <code>Key=WindowTargetIds;,Values=&lt;window-target-id-1&gt;,&lt;window-target-id-2&gt;</code> </p>
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,
    /// <p>The ARN of the task to run.</p>
    #[serde(rename = "TaskArn")]
    pub task_arn: String,
    /// <p>The parameters that the task should use during execution. Populate only the fields that match the task type. All other fields should be empty. </p>
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    /// <p><p>The parameters that should be passed to the task when it is run.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    /// <p>The type of task being registered.</p>
    #[serde(rename = "TaskType")]
    pub task_type: String,
    /// <p>The ID of the maintenance window the task should be added to.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterTaskWithMaintenanceWindowResult {
    /// <p>The ID of the task in the maintenance window.</p>
    #[serde(rename = "WindowTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

/// <p>An OpsItems that shares something in common with the current OpsItem. For example, related OpsItems can include OpsItems with similar error messages, impacted resources, or statuses for the impacted resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedOpsItem {
    /// <p>The ID of an OpsItem related to the current OpsItem.</p>
    #[serde(rename = "OpsItemId")]
    pub ops_item_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveTagsFromResourceRequest {
    /// <p><p>The ID of the resource from which you want to remove tags. For example:</p> <p>ManagedInstance: mi-012345abcde</p> <p>MaintenanceWindow: mw-012345abcde</p> <p>PatchBaseline: pb-012345abcde</p> <p>For the Document and Parameter values, use the name of the resource.</p> <note> <p>The ManagedInstance type for this API action is only for on-premises managed instances. Specify the name of the managed instance in the following format: mi-ID_number. For example, mi-1a2b3c4d5e6f.</p> </note></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The type of resource from which you want to remove a tag.</p> <note> <p>The ManagedInstance type for this API action is only for on-premises managed instances. Specify the name of the managed instance in the following format: mi-ID_number. For example, mi-1a2b3c4d5e6f.</p> </note></p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>Tag keys that you want to remove from the specified resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveTagsFromResourceResult {}

/// <p>The request body of the ResetServiceSetting API action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResetServiceSettingRequest {
    /// <p>The ID of the service setting to reset.</p>
    #[serde(rename = "SettingId")]
    pub setting_id: String,
}

/// <p>The result body of the ResetServiceSetting API action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResetServiceSettingResult {
    /// <p>The current, effective service setting after calling the ResetServiceSetting API action.</p>
    #[serde(rename = "ServiceSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_setting: Option<ServiceSetting>,
}

/// <p>Information about targets that resolved during the Automation execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolvedTargets {
    /// <p>A list of parameter values sent to targets that resolved during the Automation execution.</p>
    #[serde(rename = "ParameterValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<Vec<String>>,
    /// <p>A boolean value indicating whether the resolved target list is truncated.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

/// <p>Compliance summary information for a specific resource. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceComplianceSummaryItem {
    /// <p>The compliance type.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>A list of items that are compliant for the resource.</p>
    #[serde(rename = "CompliantSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_summary: Option<CompliantSummary>,
    /// <p>Information about the execution.</p>
    #[serde(rename = "ExecutionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary: Option<ComplianceExecutionSummary>,
    /// <p>A list of items that aren't compliant for the resource.</p>
    #[serde(rename = "NonCompliantSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_summary: Option<NonCompliantSummary>,
    /// <p>The highest severity item found for the resource. The resource is compliant for this item.</p>
    #[serde(rename = "OverallSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_severity: Option<String>,
    /// <p>The resource ID.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The resource type.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The compliance status for the resource.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about a Resource Data Sync configuration, including its current status and last successful sync.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceDataSyncItem {
    /// <p>The status reported by the last sync.</p>
    #[serde(rename = "LastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The last time the sync operations returned a status of <code>SUCCESSFUL</code> (UTC).</p>
    #[serde(rename = "LastSuccessfulSyncTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_sync_time: Option<f64>,
    /// <p>The status message details reported by the last sync.</p>
    #[serde(rename = "LastSyncStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_status_message: Option<String>,
    /// <p>The last time the configuration attempted to sync (UTC).</p>
    #[serde(rename = "LastSyncTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_time: Option<f64>,
    /// <p>Configuration information for the target Amazon S3 bucket.</p>
    #[serde(rename = "S3Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<ResourceDataSyncS3Destination>,
    /// <p>The date and time the configuration was created (UTC).</p>
    #[serde(rename = "SyncCreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_created_time: Option<f64>,
    /// <p>The name of the Resource Data Sync.</p>
    #[serde(rename = "SyncName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_name: Option<String>,
}

/// <p>Information about the target Amazon S3 bucket for the Resource Data Sync.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDataSyncS3Destination {
    /// <p>The ARN of an encryption key for a destination in Amazon S3. Must belong to the same Region as the destination Amazon S3 bucket.</p>
    #[serde(rename = "AWSKMSKeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awskms_key_arn: Option<String>,
    /// <p>The name of the Amazon S3 bucket where the aggregated data is stored.</p>
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    /// <p>An Amazon S3 prefix for the bucket.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The AWS Region with the Amazon S3 bucket targeted by the Resource Data Sync.</p>
    #[serde(rename = "Region")]
    pub region: String,
    /// <p>A supported sync format. The following format is currently supported: JsonSerDe</p>
    #[serde(rename = "SyncFormat")]
    pub sync_format: String,
}

/// <p>The inventory item result attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResultAttribute {
    /// <p>Name of the inventory item type. Valid value: AWS:InstanceInformation. Default Value: AWS:InstanceInformation.</p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResumeSessionRequest {
    /// <p>The ID of the disconnected session to resume.</p>
    #[serde(rename = "SessionId")]
    pub session_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResumeSessionResponse {
    /// <p>The ID of the session.</p>
    #[serde(rename = "SessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// <p>A URL back to SSM Agent on the instance that the Session Manager client uses to send commands and receive output from the instance. Format: <code>wss://ssmmessages.<b>region</b>.amazonaws.com/v1/data-channel/<b>session-id</b>?stream=(input|output)</code>.</p> <p> <b>region</b> represents the Region identifier for an AWS Region supported by AWS Systems Manager, such as <code>us-east-2</code> for the US East (Ohio) Region. For a list of supported <b>region</b> values, see the <b>Region</b> column in the <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#ssm_region">AWS Systems Manager table of regions and endpoints</a> in the <i>AWS General Reference</i>.</p> <p> <b>session-id</b> represents the ID of a Session Manager session, such as <code>1a2b3c4dEXAMPLE</code>.</p>
    #[serde(rename = "StreamUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_url: Option<String>,
    /// <p>An encrypted token value containing session and caller information. Used to authenticate the connection to the instance.</p>
    #[serde(rename = "TokenValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_value: Option<String>,
}

/// <p>An Amazon S3 bucket where you want to store the results of this request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3OutputLocation {
    /// <p>The name of the Amazon S3 bucket.</p>
    #[serde(rename = "OutputS3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    /// <p>The Amazon S3 bucket subfolder.</p>
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    /// <p>(Deprecated) You can no longer specify this parameter. The system ignores it. Instead, Systems Manager automatically determines the Amazon S3 bucket region.</p>
    #[serde(rename = "OutputS3Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
}

/// <p>A URL for the Amazon S3 bucket where you want to store the results of this request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3OutputUrl {
    /// <p>A URL for an Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "OutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_url: Option<String>,
}

/// <p>Information about a scheduled execution for a maintenance window.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScheduledWindowExecution {
    /// <p>The time, in ISO-8601 Extended format, that the maintenance window is scheduled to be run.</p>
    #[serde(rename = "ExecutionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<String>,
    /// <p>The name of the maintenance window to be run.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the maintenance window to be run.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendAutomationSignalRequest {
    /// <p>The unique identifier for an existing Automation execution that you want to send the signal to.</p>
    #[serde(rename = "AutomationExecutionId")]
    pub automation_execution_id: String,
    /// <p>The data sent with the signal. The data schema depends on the type of signal used in the request.</p> <p>For <code>Approve</code> and <code>Reject</code> signal types, the payload is an optional comment that you can send with the signal type. For example:</p> <p> <code>Comment="Looks good"</code> </p> <p>For <code>StartStep</code> and <code>Resume</code> signal types, you must send the name of the Automation step to start or resume as the payload. For example:</p> <p> <code>StepName="step1"</code> </p> <p>For the <code>StopStep</code> signal type, you must send the step execution ID as the payload. For example:</p> <p> <code>StepExecutionId="97fff367-fc5a-4299-aed8-0123456789ab"</code> </p>
    #[serde(rename = "Payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The type of signal to send to an Automation execution. </p>
    #[serde(rename = "SignalType")]
    pub signal_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendAutomationSignalResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendCommandRequest {
    /// <p>Enables Systems Manager to send Run Command output to Amazon CloudWatch Logs. </p>
    #[serde(rename = "CloudWatchOutputConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_config: Option<CloudWatchOutputConfig>,
    /// <p>User-specified information about the command, such as a brief description of what the command should do.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p><p>The Sha256 or Sha1 hash created by the system when the document was created. </p> <note> <p>Sha1 hashes have been deprecated.</p> </note></p>
    #[serde(rename = "DocumentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash: Option<String>,
    /// <p><p>Sha256 or Sha1.</p> <note> <p>Sha1 hashes have been deprecated.</p> </note></p>
    #[serde(rename = "DocumentHashType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash_type: Option<String>,
    /// <p>Required. The name of the Systems Manager document to run. This can be a public document or a custom document.</p>
    #[serde(rename = "DocumentName")]
    pub document_name: String,
    /// <p>The SSM document version to use in the request. You can specify $DEFAULT, $LATEST, or a specific version number. If you run commands by using the AWS CLI, then you must escape the first two options by using a backslash. If you specify a version number, then you don't need to use the backslash. For example:</p> <p>--document-version "\$DEFAULT"</p> <p>--document-version "\$LATEST"</p> <p>--document-version "3"</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The instance IDs where the command should run. You can specify a maximum of 50 IDs. If you prefer not to list individual instance IDs, you can instead send commands to a fleet of instances using the Targets parameter, which accepts EC2 tags. For more information about how to use targets, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/send-commands-multiple.html">Sending Commands to a Fleet</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// <p>(Optional) The maximum number of instances that are allowed to run the command at the same time. You can specify a number such as 10 or a percentage such as 10%. The default value is 50. For more information about how to use MaxConcurrency, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/send-commands-multiple.html#send-commands-velocity">Using Concurrency Controls</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The maximum number of errors allowed without the command failing. When the command fails one more time beyond the value of MaxErrors, the systems stops sending the command to additional targets. You can specify a number like 10 or a percentage like 10%. The default value is 0. For more information about how to use MaxErrors, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/send-commands-multiple.html#send-commands-maxerrors">Using Error Controls</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>Configurations for sending notifications.</p>
    #[serde(rename = "NotificationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    /// <p>The name of the S3 bucket where command execution responses should be stored.</p>
    #[serde(rename = "OutputS3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    /// <p>The directory structure within the S3 bucket where the responses should be stored.</p>
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    /// <p>(Deprecated) You can no longer specify this parameter. The system ignores it. Instead, Systems Manager automatically determines the Amazon S3 bucket region.</p>
    #[serde(rename = "OutputS3Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
    /// <p>The required and optional parameters specified in the document being run.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The ARN of the IAM service role to use to publish Amazon Simple Notification Service (Amazon SNS) notifications for Run Command commands.</p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>(Optional) An array of search criteria that targets instances using a Key,Value combination that you specify. Targets is required if you don't provide one or more instance IDs in the call. For more information about how to use targets, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/send-commands-multiple.html">Sending Commands to a Fleet</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>If this time is reached and the command has not already started running, it will not run.</p>
    #[serde(rename = "TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendCommandResult {
    /// <p>The request as it was received by Systems Manager. Also provides the command ID which can be used future references to this request.</p>
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
}

/// <p>The service setting data structure.</p> <p> <code>ServiceSetting</code> is an account-level setting for an AWS service. This setting defines how a user interacts with or uses a service or a feature of a service. For example, if an AWS service charges money to the account based on feature or service usage, then the AWS service team might create a default setting of "false". This means the user can't use this feature unless they change the setting to "true" and intentionally opt in for a paid feature.</p> <p>Services map a <code>SettingId</code> object to a setting value. AWS services teams define the default value for a <code>SettingId</code>. You can't create a new <code>SettingId</code>, but you can overwrite the default value if you have the <code>ssm:UpdateServiceSetting</code> permission for the setting. Use the <a>UpdateServiceSetting</a> API action to change the default setting. Or, use the <a>ResetServiceSetting</a> to change the value back to the original value defined by the AWS service team.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceSetting {
    /// <p>The ARN of the service setting.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The last time the service setting was modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The ARN of the last modified user. This field is populated only if the setting value was overwritten.</p>
    #[serde(rename = "LastModifiedUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    /// <p>The ID of the service setting.</p>
    #[serde(rename = "SettingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_id: Option<String>,
    /// <p>The value of the service setting.</p>
    #[serde(rename = "SettingValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_value: Option<String>,
    /// <p><p>The status of the service setting. The value can be Default, Customized or PendingUpdate.</p> <ul> <li> <p>Default: The current setting uses a default value provisioned by the AWS service team.</p> </li> <li> <p>Customized: The current setting use a custom value specified by the customer.</p> </li> <li> <p>PendingUpdate: The current setting uses a default or custom value, but a setting change request is pending approval.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about a Session Manager connection to an instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Session {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The name of the Session Manager SSM document used to define the parameters and plugin settings for the session. For example, <code>SSM-SessionManagerRunShell</code>.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>The date and time, in ISO-8601 Extended format, when the session was terminated.</p>
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<f64>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "OutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_url: Option<SessionManagerOutputUrl>,
    /// <p>The ID of the AWS user account that started the session.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The ID of the session.</p>
    #[serde(rename = "SessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// <p>The date and time, in ISO-8601 Extended format, when the session began.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    /// <p>The status of the session. For example, "Connected" or "Terminated".</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The instance that the Session Manager session connected to.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// <p>Describes a filter for Session Manager information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SessionFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p><p>The filter value. Valid values for each filter key are as follows:</p> <ul> <li> <p>InvokedAfter: Specify a timestamp to limit your results. For example, specify 2018-08-29T00:00:00Z to see sessions that started August 29, 2018, and later.</p> </li> <li> <p>InvokedBefore: Specify a timestamp to limit your results. For example, specify 2018-08-29T00:00:00Z to see sessions that started before August 29, 2018.</p> </li> <li> <p>Target: Specify an instance to which session connections have been made.</p> </li> <li> <p>Owner: Specify an AWS user account to see a list of sessions started by that user.</p> </li> <li> <p>Status: Specify a valid session status to see a list of all sessions with that status. Status values you can specify include:</p> <ul> <li> <p>Connected</p> </li> <li> <p>Connecting</p> </li> <li> <p>Disconnected</p> </li> <li> <p>Terminated</p> </li> <li> <p>Terminating</p> </li> <li> <p>Failed</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>Reserved for future use.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SessionManagerOutputUrl {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "CloudWatchOutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_url: Option<String>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "S3OutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_output_url: Option<String>,
}

/// <p>The number of managed instances found for each patch severity level defined in the request filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SeveritySummary {
    /// <p>The total number of resources or compliance items that have a severity level of critical. Critical severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "CriticalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_count: Option<i64>,
    /// <p>The total number of resources or compliance items that have a severity level of high. High severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "HighCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_count: Option<i64>,
    /// <p>The total number of resources or compliance items that have a severity level of informational. Informational severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "InformationalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub informational_count: Option<i64>,
    /// <p>The total number of resources or compliance items that have a severity level of low. Low severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "LowCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_count: Option<i64>,
    /// <p>The total number of resources or compliance items that have a severity level of medium. Medium severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "MediumCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_count: Option<i64>,
    /// <p>The total number of resources or compliance items that have a severity level of unspecified. Unspecified severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "UnspecifiedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unspecified_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartAssociationsOnceRequest {
    /// <p>The association IDs that you want to run immediately and only one time.</p>
    #[serde(rename = "AssociationIds")]
    pub association_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartAssociationsOnceResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartAutomationExecutionRequest {
    /// <p>User-provided idempotency token. The token must be unique, is case insensitive, enforces the UUID format, and can't be reused.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the Automation document to use for this execution.</p>
    #[serde(rename = "DocumentName")]
    pub document_name: String,
    /// <p>The version of the Automation document to use for this execution.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The maximum number of targets allowed to run this task in parallel. You can specify a number, such as 10, or a percentage, such as 10%. The default value is 10.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The number of errors that are allowed before the system stops running the automation on additional targets. You can specify either an absolute number of errors, for example 10, or a percentage of the target set, for example 10%. If you specify 3, for example, the system stops running the automation when the fourth error is received. If you specify 0, then the system stops running the automation on additional targets after the first error result is returned. If you run an automation on 50 resources and set max-errors to 10%, then the system stops running the automation on additional targets when the sixth error is received.</p> <p>Executions that are already running an automation when max-errors is reached are allowed to complete, but some of these executions may fail as well. If you need to ensure that there won't be more than max-errors failed executions, set max-concurrency to 1 so the executions proceed one at a time.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The execution mode of the automation. Valid modes include the following: Auto and Interactive. The default mode is Auto.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>A key-value map of execution parameters, which match the declared parameters in the Automation document.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>A location is a combination of AWS Regions and/or AWS accounts where you want to run the Automation. Use this action to start an Automation in multiple Regions and multiple accounts. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-automation-multiple-accounts-and-regions.html">Executing Automations in Multiple AWS Regions and Accounts</a> in the <i>AWS Systems Manager User Guide</i>. </p>
    #[serde(rename = "TargetLocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations: Option<Vec<TargetLocation>>,
    /// <p>A key-value mapping of document parameters to target resources. Both Targets and TargetMaps cannot be specified together.</p>
    #[serde(rename = "TargetMaps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<::std::collections::HashMap<String, Vec<String>>>>,
    /// <p>The name of the parameter used as the target resource for the rate-controlled execution. Required if you specify targets.</p>
    #[serde(rename = "TargetParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter_name: Option<String>,
    /// <p>A key-value mapping to target resources. Required if you specify TargetParameterName.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartAutomationExecutionResult {
    /// <p>The unique ID of a newly scheduled automation execution.</p>
    #[serde(rename = "AutomationExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartSessionRequest {
    /// <p>The name of the SSM document to define the parameters and plugin settings for the session. For example, <code>SSM-SessionManagerRunShell</code>. If no document name is provided, a shell to the instance is launched by default.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The instance to connect to for the session.</p>
    #[serde(rename = "Target")]
    pub target: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartSessionResponse {
    /// <p>The ID of the session.</p>
    #[serde(rename = "SessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// <p>A URL back to SSM Agent on the instance that the Session Manager client uses to send commands and receive output from the instance. Format: <code>wss://ssmmessages.<b>region</b>.amazonaws.com/v1/data-channel/<b>session-id</b>?stream=(input|output)</code> </p> <p> <b>region</b> represents the Region identifier for an AWS Region supported by AWS Systems Manager, such as <code>us-east-2</code> for the US East (Ohio) Region. For a list of supported <b>region</b> values, see the <b>Region</b> column in the <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#ssm_region">AWS Systems Manager table of regions and endpoints</a> in the <i>AWS General Reference</i>.</p> <p> <b>session-id</b> represents the ID of a Session Manager session, such as <code>1a2b3c4dEXAMPLE</code>.</p>
    #[serde(rename = "StreamUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_url: Option<String>,
    /// <p>An encrypted token value containing session and caller information. Used to authenticate the connection to the instance.</p>
    #[serde(rename = "TokenValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_value: Option<String>,
}

/// <p>Detailed information about an the execution state of an Automation step.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StepExecution {
    /// <p>The action this step performs. The action determines the behavior of the step.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>If a step has finished execution, this contains the time the execution ended. If the step has not yet concluded, this field is not populated.</p>
    #[serde(rename = "ExecutionEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_time: Option<f64>,
    /// <p>If a step has begun execution, this contains the time the step started. If the step is in Pending status, this field is not populated.</p>
    #[serde(rename = "ExecutionStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_time: Option<f64>,
    /// <p>Information about the Automation failure.</p>
    #[serde(rename = "FailureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FailureDetails>,
    /// <p>If a step failed, this message explains why the execution failed.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// <p>Fully-resolved values passed into the step before execution.</p>
    #[serde(rename = "Inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<::std::collections::HashMap<String, String>>,
    /// <p>The flag which can be used to help decide whether the failure of current step leads to the Automation failure.</p>
    #[serde(rename = "IsCritical")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_critical: Option<bool>,
    /// <p>The flag which can be used to end automation no matter whether the step succeeds or fails.</p>
    #[serde(rename = "IsEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_end: Option<bool>,
    /// <p>The maximum number of tries to run the action of the step. The default value is 1.</p>
    #[serde(rename = "MaxAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
    /// <p>The next step after the step succeeds.</p>
    #[serde(rename = "NextStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step: Option<String>,
    /// <p>The action to take if the step fails. The default value is Abort.</p>
    #[serde(rename = "OnFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<String>,
    /// <p>Returned values from the execution of the step.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>A user-specified list of parameters to override when running a step.</p>
    #[serde(rename = "OverriddenParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridden_parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>A message associated with the response code for an execution.</p>
    #[serde(rename = "Response")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// <p>The response code returned by the execution of the step.</p>
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    /// <p>The unique ID of a step execution.</p>
    #[serde(rename = "StepExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_execution_id: Option<String>,
    /// <p>The name of this execution step.</p>
    #[serde(rename = "StepName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_name: Option<String>,
    /// <p>The execution status for this step. Valid values include: Pending, InProgress, Success, Cancelled, Failed, and TimedOut.</p>
    #[serde(rename = "StepStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_status: Option<String>,
    /// <p>The combination of AWS Regions and accounts targeted by the current Automation execution.</p>
    #[serde(rename = "TargetLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_location: Option<TargetLocation>,
    /// <p>The targets for the step execution.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The timeout seconds of the step.</p>
    #[serde(rename = "TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
    /// <p>Strategies used when step fails, we support Continue and Abort. Abort will fail the automation when the step fails. Continue will ignore the failure of current step and allow automation to run the next step. With conditional branching, we add step:stepName to support the automation to go to another specific step.</p>
    #[serde(rename = "ValidNextSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_next_steps: Option<Vec<String>>,
}

/// <p>A filter to limit the amount of step execution information returned by the call.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StepExecutionFilter {
    /// <p>One or more keys to limit the results. Valid filter keys include the following: StepName, Action, StepExecutionId, StepExecutionStatus, StartTimeBefore, StartTimeAfter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The values of the filter key.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopAutomationExecutionRequest {
    /// <p>The execution ID of the Automation to stop.</p>
    #[serde(rename = "AutomationExecutionId")]
    pub automation_execution_id: String,
    /// <p>The stop request type. Valid types include the following: Cancel and Complete. The default type is Cancel.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopAutomationExecutionResult {}

/// <p>Metadata that you assign to your AWS resources. Tags enable you to categorize your resources in different ways, for example, by purpose, owner, or environment. In Systems Manager, you can apply tags to documents, managed instances, maintenance windows, Parameter Store parameters, and patch baselines.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The name of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>An array of search criteria that targets instances using a Key,Value combination that you specify. </p> <p>Supported formats include the following.</p> <ul> <li> <p> <code>Key=InstanceIds,Values=<i>instance-id-1</i>,<i>instance-id-2</i>,<i>instance-id-3</i> </code> </p> </li> <li> <p> <code>Key=tag:<i>my-tag-key</i>,Values=<i>my-tag-value-1</i>,<i>my-tag-value-2</i> </code> </p> </li> <li> <p> <code>Key=tag-key,Values=<i>my-tag-key-1</i>,<i>my-tag-key-2</i> </code> </p> </li> <li> <p>(Maintenance window targets only) <code>Key=resource-groups:Name,Values=<i>resource-group-name</i> </code> </p> </li> <li> <p>(Maintenance window targets only) <code>Key=resource-groups:ResourceTypeFilters,Values=<i>resource-type-1</i>,<i>resource-type-2</i> </code> </p> </li> </ul> <p>For example:</p> <ul> <li> <p> <code>Key=InstanceIds,Values=i-02573cafcfEXAMPLE,i-0471e04240EXAMPLE,i-07782c72faEXAMPLE</code> </p> </li> <li> <p> <code>Key=tag:CostCenter,Values=CostCenter1,CostCenter2,CostCenter3</code> </p> </li> <li> <p> <code>Key=tag-key,Values=Name,Instance-Type,CostCenter</code> </p> </li> <li> <p>(Maintenance window targets only) <code>Key=resource-groups:Name,Values=ProductionResourceGroup</code> </p> </li> <li> <p>(Maintenance window targets only) <code>Key=resource-groups:ResourceTypeFilters,Values=<i>AWS::EC2::INSTANCE</i>,<i>AWS::EC2::VPC</i> </code> </p> </li> </ul> <p>For information about how to send commands that target instances using <code>Key,Value</code> parameters, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/send-commands-multiple.html#send-commands-targeting">Using Targets and Rate Controls to Send Commands to a Fleet</a> in the <i>AWS Systems Manager User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    /// <p>User-defined criteria for sending commands that target instances that meet the criteria.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>User-defined criteria that maps to <code>Key</code>. For example, if you specified <code>tag:ServerRole</code>, you could specify <code>value:WebServer</code> to run a command on instances that include Amazon EC2 tags of <code>ServerRole,WebServer</code>. </p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>The combination of AWS Regions and accounts targeted by the current Automation execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetLocation {
    /// <p>The AWS accounts targeted by the current Automation execution.</p>
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    /// <p>The Automation execution role used by the currently running Automation.</p>
    #[serde(rename = "ExecutionRoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_name: Option<String>,
    /// <p>The AWS Regions targeted by the current Automation execution.</p>
    #[serde(rename = "Regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    /// <p>The maximum number of AWS accounts and AWS regions allowed to run the Automation concurrently </p>
    #[serde(rename = "TargetLocationMaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_location_max_concurrency: Option<String>,
    /// <p>The maximum number of errors allowed before the system stops queueing additional Automation executions for the currently running Automation. </p>
    #[serde(rename = "TargetLocationMaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_location_max_errors: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TerminateSessionRequest {
    /// <p>The ID of the session to terminate.</p>
    #[serde(rename = "SessionId")]
    pub session_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TerminateSessionResponse {
    /// <p>The ID of the session that has been terminated.</p>
    #[serde(rename = "SessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAssociationRequest {
    /// <p>The ID of the association you want to update. </p>
    #[serde(rename = "AssociationId")]
    pub association_id: String,
    /// <p>The name of the association that you want to update.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>This parameter is provided for concurrency control purposes. You must specify the latest association version in the service. If you want to ensure that this request succeeds, either specify <code>$LATEST</code>, or omit this parameter.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>Specify the target for the association. This target is required for associations that use an Automation document and target resources by using rate controls.</p>
    #[serde(rename = "AutomationTargetParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_target_parameter_name: Option<String>,
    /// <p>The severity level to assign to the association.</p>
    #[serde(rename = "ComplianceSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_severity: Option<String>,
    /// <p>The document version you want update for the association. </p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The maximum number of targets allowed to run the association at the same time. You can specify a number, for example 10, or a percentage of the target set, for example 10%. The default value is 100%, which means all targets run the association at the same time.</p> <p>If a new instance starts and attempts to run an association while Systems Manager is running MaxConcurrency associations, the association is allowed to run. During the next association interval, the new instance will process its association within the limit specified for MaxConcurrency.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The number of errors that are allowed before the system stops sending requests to run the association on additional targets. You can specify either an absolute number of errors, for example 10, or a percentage of the target set, for example 10%. If you specify 3, for example, the system stops sending requests when the fourth error is received. If you specify 0, then the system stops sending requests after the first error is returned. If you run an association on 50 instances and set MaxError to 10%, then the system stops sending the request when the sixth error is received.</p> <p>Executions that are already running an association when MaxErrors is reached are allowed to complete, but some of these executions may fail as well. If you need to ensure that there won't be more than max-errors failed executions, set MaxConcurrency to 1 so that executions proceed one at a time.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The name of the SSM document that contains the configuration information for the instance. You can specify Command or Automation documents.</p> <p>You can specify AWS-predefined documents, documents you created, or a document that is shared with you from another account.</p> <p>For SSM documents that are shared with you from other AWS accounts, you must specify the complete SSM document ARN, in the following format:</p> <p> <code>arn:aws:ssm:<i>region</i>:<i>account-id</i>:document/<i>document-name</i> </code> </p> <p>For example:</p> <p> <code>arn:aws:ssm:us-east-2:12345678912:document/My-Shared-Document</code> </p> <p>For AWS-predefined documents and SSM documents you created in your account, you only need to specify the document name. For example, <code>AWS-ApplyPatchBaseline</code> or <code>My-Document</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    /// <p>The parameters you want to update for the association. If you create a parameter using Parameter Store, you can reference the parameter using {{ssm:parameter-name}}</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The cron expression used to schedule the association that you want to update.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The targets of the association.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAssociationResult {
    /// <p>The description of the association that was updated.</p>
    #[serde(rename = "AssociationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAssociationStatusRequest {
    /// <p>The association status.</p>
    #[serde(rename = "AssociationStatus")]
    pub association_status: AssociationStatus,
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAssociationStatusResult {
    /// <p>Information about the association.</p>
    #[serde(rename = "AssociationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDocumentDefaultVersionRequest {
    /// <p>The version of a custom document that you want to set as the default version.</p>
    #[serde(rename = "DocumentVersion")]
    pub document_version: String,
    /// <p>The name of a custom document that you want to set as the default version.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDocumentDefaultVersionResult {
    /// <p>The description of a custom document that you want to set as the default version.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<DocumentDefaultVersionDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDocumentRequest {
    /// <p>A list of key and value pairs that describe attachments to a version of a document.</p>
    #[serde(rename = "Attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentsSource>>,
    /// <p>A valid JSON or YAML string.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>Specify the document format for the new document version. Systems Manager supports JSON and YAML documents. JSON is the default format.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>(Required) The version of the document that you want to update. </p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the document that you want to update.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specify a new target type for the document.</p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// <p>An optional field specifying the version of the artifact you are updating with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document, and cannot be changed.</p>
    #[serde(rename = "VersionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDocumentResult {
    /// <p>A description of the document that was updated.</p>
    #[serde(rename = "DocumentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMaintenanceWindowRequest {
    /// <p>Whether targets must be registered with the maintenance window before tasks can be defined for those targets.</p>
    #[serde(rename = "AllowUnassociatedTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unassociated_targets: Option<bool>,
    /// <p>The number of hours before the end of the maintenance window that Systems Manager stops scheduling new tasks for execution.</p>
    #[serde(rename = "Cutoff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i64>,
    /// <p>An optional description for the update request.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The duration of the maintenance window in hours.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Whether the maintenance window is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The date and time, in ISO-8601 Extended format, for when you want the maintenance window to become inactive. EndDate allows you to set a date and time in the future when the maintenance window will no longer run.</p>
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// <p>The name of the maintenance window.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If True, then all fields that are required by the CreateMaintenanceWindow action are also required for this API request. Optional fields that are not specified are set to null. </p>
    #[serde(rename = "Replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    /// <p>The schedule of the maintenance window in the form of a cron or rate expression.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The time zone that the scheduled maintenance window executions are based on, in Internet Assigned Numbers Authority (IANA) format. For example: "America/Los_Angeles", "etc/UTC", or "Asia/Seoul". For more information, see the <a href="https://www.iana.org/time-zones">Time Zone Database</a> on the IANA website.</p>
    #[serde(rename = "ScheduleTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_timezone: Option<String>,
    /// <p>The time zone that the scheduled maintenance window executions are based on, in Internet Assigned Numbers Authority (IANA) format. For example: "America/Los_Angeles", "etc/UTC", or "Asia/Seoul". For more information, see the <a href="https://www.iana.org/time-zones">Time Zone Database</a> on the IANA website.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// <p>The ID of the maintenance window to update.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMaintenanceWindowResult {
    /// <p>Whether targets must be registered with the maintenance window before tasks can be defined for those targets.</p>
    #[serde(rename = "AllowUnassociatedTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unassociated_targets: Option<bool>,
    /// <p>The number of hours before the end of the maintenance window that Systems Manager stops scheduling new tasks for execution.</p>
    #[serde(rename = "Cutoff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i64>,
    /// <p>An optional description of the update.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The duration of the maintenance window in hours.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Whether the maintenance window is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The date and time, in ISO-8601 Extended format, for when the maintenance window is scheduled to become inactive. The maintenance window will not run after this specified time.</p>
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// <p>The name of the maintenance window.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schedule of the maintenance window in the form of a cron or rate expression.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The time zone that the scheduled maintenance window executions are based on, in Internet Assigned Numbers Authority (IANA) format. For example: "America/Los_Angeles", "etc/UTC", or "Asia/Seoul". For more information, see the <a href="https://www.iana.org/time-zones">Time Zone Database</a> on the IANA website.</p>
    #[serde(rename = "ScheduleTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_timezone: Option<String>,
    /// <p>The date and time, in ISO-8601 Extended format, for when the maintenance window is scheduled to become active. The maintenance window will not run before this specified time.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// <p>The ID of the created maintenance window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMaintenanceWindowTargetRequest {
    /// <p>An optional description for the update.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A name for the update.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this maintenance window.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>If True, then all fields that are required by the RegisterTargetWithMaintenanceWindow action are also required for this API request. Optional fields that are not specified are set to null.</p>
    #[serde(rename = "Replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    /// <p>The targets to add or replace.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The maintenance window ID with which to modify the target.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// <p>The target ID to modify.</p>
    #[serde(rename = "WindowTargetId")]
    pub window_target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMaintenanceWindowTargetResult {
    /// <p>The updated description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The updated owner.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The updated targets.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The maintenance window ID specified in the update request.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The target ID specified in the update request.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMaintenanceWindowTaskRequest {
    /// <p>The new task description to specify.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The new logging location in Amazon S3 to specify.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "LoggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <p>The new <code>MaxConcurrency</code> value you want to specify. <code>MaxConcurrency</code> is the number of targets that are allowed to run this task in parallel.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The new <code>MaxErrors</code> value to specify. <code>MaxErrors</code> is the maximum number of errors that are allowed before the task stops being scheduled.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The new task name to specify.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The new task priority to specify. The lower the number, the higher the priority. Tasks that have the same priority are scheduled in parallel.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>If True, then all fields that are required by the RegisterTaskWithMaintenanceWndow action are also required for this API request. Optional fields that are not specified are set to null.</p>
    #[serde(rename = "Replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    /// <p><p>The ARN of the IAM service role for Systems Manager to assume when running a maintenance window task. If you do not specify a service role ARN, Systems Manager uses your account&#39;s service-linked role. If no service-linked role for Systems Manager exists in your account, it is created when you run <code>RegisterTaskWithMaintenanceWindow</code>.</p> <p>For more information, see the following topics in the in the <i>AWS Systems Manager User Guide</i>:</p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/using-service-linked-roles.html#slr-permissions">Service-Linked Role Permissions for Systems Manager</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-maintenance-permissions.html#maintenance-window-tasks-service-role">Should I Use a Service-Linked Role or a Custom Service Role to Run Maintenance Window Tasks? </a> </p> </li> </ul></p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The targets (either instances or tags) to modify. Instances are specified using Key=instanceids,Values=instanceID_1,instanceID_2. Tags are specified using Key=tag_name,Values=tag_value. </p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The task ARN to modify.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The parameters that the task should use during execution. Populate only the fields that match the task type. All other fields should be empty.</p>
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    /// <p>The parameters to modify.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note> <p>The map has the following format:</p> <p>Key: string, between 1 and 255 characters</p> <p>Value: an array of strings, each string is between 1 and 255 characters</p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    /// <p>The maintenance window ID that contains the task to modify.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// <p>The task ID to modify.</p>
    #[serde(rename = "WindowTaskId")]
    pub window_task_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMaintenanceWindowTaskResult {
    /// <p>The updated task description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The updated logging information in Amazon S3.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "LoggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <p>The updated MaxConcurrency value.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The updated MaxErrors value.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The updated task name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The updated priority value.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The ARN of the IAM service role to use to publish Amazon Simple Notification Service (Amazon SNS) notifications for maintenance window Run Command tasks.</p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The updated target values.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The updated task ARN value.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The updated parameter values.</p>
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    /// <p><p>The updated parameter values.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported maintenance window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    /// <p>The ID of the maintenance window that was updated.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The task ID of the maintenance window that was updated.</p>
    #[serde(rename = "WindowTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateManagedInstanceRoleRequest {
    /// <p>The IAM role you want to assign or change.</p>
    #[serde(rename = "IamRole")]
    pub iam_role: String,
    /// <p>The ID of the managed instance where you want to update the role.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateManagedInstanceRoleResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateOpsItemRequest {
    /// <p>Update the information about the OpsItem. Provide enough information so that users reading this OpsItem for the first time understand the issue. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of an SNS topic where notifications are sent when this OpsItem is edited or changed.</p>
    #[serde(rename = "Notifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<OpsItemNotification>>,
    /// <p>Add new keys or edit existing key-value pairs of the OperationalData map in the OpsItem object.</p> <p>Operational data is custom data that provides useful reference details about the OpsItem. For example, you can specify log files, error strings, license keys, troubleshooting tips, or other relevant data. You enter operational data as key-value pairs. The key has a maximum length of 128 characters. The value has a maximum size of 20 KB.</p> <important> <p>Operational data keys <i>can't</i> begin with the following: amazon, aws, amzn, ssm, /amazon, /aws, /amzn, /ssm.</p> </important> <p>You can choose to make the data searchable by other users in the account or you can restrict search access. Searchable data means that all users with access to the OpsItem Overview page (as provided by the <a>DescribeOpsItems</a> API action) can view and search on the specified data. Operational data that is not searchable is only viewable by users who have access to the OpsItem (as provided by the <a>GetOpsItem</a> API action).</p> <p>Use the <code>/aws/resources</code> key in OperationalData to specify a related resource in the request. Use the <code>/aws/automations</code> key in OperationalData to associate an Automation runbook with the OpsItem. To view AWS CLI example commands that use these keys, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-creating-OpsItems.html#OpsCenter-manually-create-OpsItems">Creating OpsItems Manually</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "OperationalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_data: Option<::std::collections::HashMap<String, OpsItemDataValue>>,
    /// <p>Keys that you want to remove from the OperationalData map.</p>
    #[serde(rename = "OperationalDataToDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_data_to_delete: Option<Vec<String>>,
    /// <p>The ID of the OpsItem.</p>
    #[serde(rename = "OpsItemId")]
    pub ops_item_id: String,
    /// <p>The importance of this OpsItem in relation to other OpsItems in the system.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>One or more OpsItems that share something in common with the current OpsItems. For example, related OpsItems can include OpsItems with similar error messages, impacted resources, or statuses for the impacted resource.</p>
    #[serde(rename = "RelatedOpsItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_ops_items: Option<Vec<RelatedOpsItem>>,
    /// <p>The OpsItem status. Status can be <code>Open</code>, <code>In Progress</code>, or <code>Resolved</code>. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-working-with-OpsItems-editing-details.html">Editing OpsItem Details</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A short heading that describes the nature of the OpsItem and the impacted resource.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateOpsItemResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePatchBaselineRequest {
    /// <p>A set of rules used to include patches in the baseline.</p>
    #[serde(rename = "ApprovalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    /// <p>A list of explicitly approved patches for the baseline.</p> <p>For information about accepted formats for lists of approved patches and rejected patches, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/patch-manager-approved-rejected-package-name-formats.html">Package Name Formats for Approved and Rejected Patch Lists</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "ApprovedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    /// <p>Assigns a new compliance severity level to an existing patch baseline.</p>
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    /// <p>Indicates whether the list of approved patches includes non-security updates that should be applied to the instances. The default value is 'false'. Applies to Linux instances only.</p>
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    /// <p>The ID of the patch baseline to update.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
    /// <p>A description of the patch baseline.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A set of global filters used to include patches in the baseline.</p>
    #[serde(rename = "GlobalFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    /// <p>The name of the patch baseline.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of explicitly rejected patches for the baseline.</p> <p>For information about accepted formats for lists of approved patches and rejected patches, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/patch-manager-approved-rejected-package-name-formats.html">Package Name Formats for Approved and Rejected Patch Lists</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "RejectedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    /// <p><p>The action for Patch Manager to take on patches included in the RejectedPackages list.</p> <ul> <li> <p> <b>ALLOW<em>AS</em>DEPENDENCY</b>: A package in the Rejected patches list is installed only if it is a dependency of another package. It is considered compliant with the patch baseline, and its status is reported as <i>InstalledOther</i>. This is the default action if no option is specified.</p> </li> <li> <p> <b>BLOCK</b>: Packages in the RejectedPatches list, and packages that include them as dependencies, are not installed under any circumstances. If a package was installed before it was added to the Rejected patches list, it is considered non-compliant with the patch baseline, and its status is reported as <i>InstalledRejected</i>.</p> </li> </ul></p>
    #[serde(rename = "RejectedPatchesAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches_action: Option<String>,
    /// <p>If True, then all fields that are required by the CreatePatchBaseline action are also required for this API request. Optional fields that are not specified are set to null.</p>
    #[serde(rename = "Replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    /// <p>Information about the patches to use to update the instances, including target operating systems and source repositories. Applies to Linux instances only.</p>
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePatchBaselineResult {
    /// <p>A set of rules used to include patches in the baseline.</p>
    #[serde(rename = "ApprovalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    /// <p>A list of explicitly approved patches for the baseline.</p>
    #[serde(rename = "ApprovedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    /// <p>The compliance severity level assigned to the patch baseline after the update completed.</p>
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    /// <p>Indicates whether the list of approved patches includes non-security updates that should be applied to the instances. The default value is 'false'. Applies to Linux instances only.</p>
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    /// <p>The ID of the deleted patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The date when the patch baseline was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the Patch Baseline.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A set of global filters used to exclude patches from the baseline.</p>
    #[serde(rename = "GlobalFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    /// <p>The date when the patch baseline was last modified.</p>
    #[serde(rename = "ModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<f64>,
    /// <p>The name of the patch baseline.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The operating system rule used by the updated patch baseline.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>A list of explicitly rejected patches for the baseline.</p>
    #[serde(rename = "RejectedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    /// <p>The action specified to take on patches included in the RejectedPatches list. A patch can be allowed only if it is a dependency of another package, or blocked entirely along with packages that include it as a dependency.</p>
    #[serde(rename = "RejectedPatchesAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches_action: Option<String>,
    /// <p>Information about the patches to use to update the instances, including target operating systems and source repositories. Applies to Linux instances only.</p>
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
}

/// <p>The request body of the UpdateServiceSetting API action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateServiceSettingRequest {
    /// <p>The ID of the service setting to update.</p>
    #[serde(rename = "SettingId")]
    pub setting_id: String,
    /// <p>The new value to specify for the service setting.</p>
    #[serde(rename = "SettingValue")]
    pub setting_value: String,
}

/// <p>The result body of the UpdateServiceSetting API action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateServiceSettingResult {}

/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The resource type is not valid. For example, if you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
    InvalidResourceType(String),
    /// <p>The <code>Targets</code> parameter includes too many tags. Remove one or more tags and try the command again.</p>
    TooManyTagsError(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
}

impl AddTagsToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(AddTagsToResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidResourceId" => {
                    return RusotoError::Service(AddTagsToResourceError::InvalidResourceId(err.msg))
                }
                "InvalidResourceType" => {
                    return RusotoError::Service(AddTagsToResourceError::InvalidResourceType(
                        err.msg,
                    ))
                }
                "TooManyTagsError" => {
                    return RusotoError::Service(AddTagsToResourceError::TooManyTagsError(err.msg))
                }
                "TooManyUpdates" => {
                    return RusotoError::Service(AddTagsToResourceError::TooManyUpdates(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AddTagsToResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsToResourceError {
    fn description(&self) -> &str {
        match *self {
            AddTagsToResourceError::InternalServerError(ref cause) => cause,
            AddTagsToResourceError::InvalidResourceId(ref cause) => cause,
            AddTagsToResourceError::InvalidResourceType(ref cause) => cause,
            AddTagsToResourceError::TooManyTagsError(ref cause) => cause,
            AddTagsToResourceError::TooManyUpdates(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelCommand
#[derive(Debug, PartialEq)]
pub enum CancelCommandError {
    /// <p>You cannot specify an instance ID in more than one association.</p>
    DuplicateInstanceId(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),

    InvalidCommandId(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
}

impl CancelCommandError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelCommandError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateInstanceId" => {
                    return RusotoError::Service(CancelCommandError::DuplicateInstanceId(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CancelCommandError::InternalServerError(err.msg))
                }
                "InvalidCommandId" => {
                    return RusotoError::Service(CancelCommandError::InvalidCommandId(err.msg))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(CancelCommandError::InvalidInstanceId(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CancelCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelCommandError {
    fn description(&self) -> &str {
        match *self {
            CancelCommandError::DuplicateInstanceId(ref cause) => cause,
            CancelCommandError::InternalServerError(ref cause) => cause,
            CancelCommandError::InvalidCommandId(ref cause) => cause,
            CancelCommandError::InvalidInstanceId(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelMaintenanceWindowExecution
#[derive(Debug, PartialEq)]
pub enum CancelMaintenanceWindowExecutionError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl CancelMaintenanceWindowExecutionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CancelMaintenanceWindowExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        CancelMaintenanceWindowExecutionError::DoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        CancelMaintenanceWindowExecutionError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CancelMaintenanceWindowExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelMaintenanceWindowExecutionError {
    fn description(&self) -> &str {
        match *self {
            CancelMaintenanceWindowExecutionError::DoesNotExist(ref cause) => cause,
            CancelMaintenanceWindowExecutionError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateActivation
#[derive(Debug, PartialEq)]
pub enum CreateActivationError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl CreateActivationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateActivationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CreateActivationError::InternalServerError(
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
impl fmt::Display for CreateActivationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateActivationError {
    fn description(&self) -> &str {
        match *self {
            CreateActivationError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAssociation
#[derive(Debug, PartialEq)]
pub enum CreateAssociationError {
    /// <p>The specified association already exists.</p>
    AssociationAlreadyExists(String),
    /// <p>You can have at most 2,000 active associations.</p>
    AssociationLimitExceeded(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The output location is not valid or does not exist.</p>
    InvalidOutputLocation(String),
    /// <p>You must specify values for all required parameters in the Systems Manager document. You can only supply values to parameters defined in the Systems Manager document.</p>
    InvalidParameters(String),
    /// <p>The schedule is invalid. Verify your cron or rate expression and try again.</p>
    InvalidSchedule(String),
    /// <p>The target is not valid or does not exist. It might not be configured for EC2 Systems Manager or you might not have permission to perform the operation.</p>
    InvalidTarget(String),
    /// <p>The document does not support the platform type of the given instance ID(s). For example, you sent an document for a Windows instance to a Linux instance.</p>
    UnsupportedPlatformType(String),
}

impl CreateAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAssociationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociationAlreadyExists" => {
                    return RusotoError::Service(CreateAssociationError::AssociationAlreadyExists(
                        err.msg,
                    ))
                }
                "AssociationLimitExceeded" => {
                    return RusotoError::Service(CreateAssociationError::AssociationLimitExceeded(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateAssociationError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(CreateAssociationError::InvalidDocument(err.msg))
                }
                "InvalidDocumentVersion" => {
                    return RusotoError::Service(CreateAssociationError::InvalidDocumentVersion(
                        err.msg,
                    ))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(CreateAssociationError::InvalidInstanceId(err.msg))
                }
                "InvalidOutputLocation" => {
                    return RusotoError::Service(CreateAssociationError::InvalidOutputLocation(
                        err.msg,
                    ))
                }
                "InvalidParameters" => {
                    return RusotoError::Service(CreateAssociationError::InvalidParameters(err.msg))
                }
                "InvalidSchedule" => {
                    return RusotoError::Service(CreateAssociationError::InvalidSchedule(err.msg))
                }
                "InvalidTarget" => {
                    return RusotoError::Service(CreateAssociationError::InvalidTarget(err.msg))
                }
                "UnsupportedPlatformType" => {
                    return RusotoError::Service(CreateAssociationError::UnsupportedPlatformType(
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
impl fmt::Display for CreateAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAssociationError {
    fn description(&self) -> &str {
        match *self {
            CreateAssociationError::AssociationAlreadyExists(ref cause) => cause,
            CreateAssociationError::AssociationLimitExceeded(ref cause) => cause,
            CreateAssociationError::InternalServerError(ref cause) => cause,
            CreateAssociationError::InvalidDocument(ref cause) => cause,
            CreateAssociationError::InvalidDocumentVersion(ref cause) => cause,
            CreateAssociationError::InvalidInstanceId(ref cause) => cause,
            CreateAssociationError::InvalidOutputLocation(ref cause) => cause,
            CreateAssociationError::InvalidParameters(ref cause) => cause,
            CreateAssociationError::InvalidSchedule(ref cause) => cause,
            CreateAssociationError::InvalidTarget(ref cause) => cause,
            CreateAssociationError::UnsupportedPlatformType(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAssociationBatch
#[derive(Debug, PartialEq)]
pub enum CreateAssociationBatchError {
    /// <p>You can have at most 2,000 active associations.</p>
    AssociationLimitExceeded(String),
    /// <p>You cannot specify an instance ID in more than one association.</p>
    DuplicateInstanceId(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The output location is not valid or does not exist.</p>
    InvalidOutputLocation(String),
    /// <p>You must specify values for all required parameters in the Systems Manager document. You can only supply values to parameters defined in the Systems Manager document.</p>
    InvalidParameters(String),
    /// <p>The schedule is invalid. Verify your cron or rate expression and try again.</p>
    InvalidSchedule(String),
    /// <p>The target is not valid or does not exist. It might not be configured for EC2 Systems Manager or you might not have permission to perform the operation.</p>
    InvalidTarget(String),
    /// <p>The document does not support the platform type of the given instance ID(s). For example, you sent an document for a Windows instance to a Linux instance.</p>
    UnsupportedPlatformType(String),
}

impl CreateAssociationBatchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAssociationBatchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociationLimitExceeded" => {
                    return RusotoError::Service(
                        CreateAssociationBatchError::AssociationLimitExceeded(err.msg),
                    )
                }
                "DuplicateInstanceId" => {
                    return RusotoError::Service(CreateAssociationBatchError::DuplicateInstanceId(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateAssociationBatchError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(CreateAssociationBatchError::InvalidDocument(
                        err.msg,
                    ))
                }
                "InvalidDocumentVersion" => {
                    return RusotoError::Service(
                        CreateAssociationBatchError::InvalidDocumentVersion(err.msg),
                    )
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(CreateAssociationBatchError::InvalidInstanceId(
                        err.msg,
                    ))
                }
                "InvalidOutputLocation" => {
                    return RusotoError::Service(
                        CreateAssociationBatchError::InvalidOutputLocation(err.msg),
                    )
                }
                "InvalidParameters" => {
                    return RusotoError::Service(CreateAssociationBatchError::InvalidParameters(
                        err.msg,
                    ))
                }
                "InvalidSchedule" => {
                    return RusotoError::Service(CreateAssociationBatchError::InvalidSchedule(
                        err.msg,
                    ))
                }
                "InvalidTarget" => {
                    return RusotoError::Service(CreateAssociationBatchError::InvalidTarget(
                        err.msg,
                    ))
                }
                "UnsupportedPlatformType" => {
                    return RusotoError::Service(
                        CreateAssociationBatchError::UnsupportedPlatformType(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAssociationBatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAssociationBatchError {
    fn description(&self) -> &str {
        match *self {
            CreateAssociationBatchError::AssociationLimitExceeded(ref cause) => cause,
            CreateAssociationBatchError::DuplicateInstanceId(ref cause) => cause,
            CreateAssociationBatchError::InternalServerError(ref cause) => cause,
            CreateAssociationBatchError::InvalidDocument(ref cause) => cause,
            CreateAssociationBatchError::InvalidDocumentVersion(ref cause) => cause,
            CreateAssociationBatchError::InvalidInstanceId(ref cause) => cause,
            CreateAssociationBatchError::InvalidOutputLocation(ref cause) => cause,
            CreateAssociationBatchError::InvalidParameters(ref cause) => cause,
            CreateAssociationBatchError::InvalidSchedule(ref cause) => cause,
            CreateAssociationBatchError::InvalidTarget(ref cause) => cause,
            CreateAssociationBatchError::UnsupportedPlatformType(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDocument
#[derive(Debug, PartialEq)]
pub enum CreateDocumentError {
    /// <p>The specified document already exists.</p>
    DocumentAlreadyExists(String),
    /// <p>You can have at most 500 active Systems Manager documents.</p>
    DocumentLimitExceeded(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The content for the document is not valid.</p>
    InvalidDocumentContent(String),
    /// <p>The version of the document schema is not supported.</p>
    InvalidDocumentSchemaVersion(String),
    /// <p>The size limit of a document is 64 KB.</p>
    MaxDocumentSizeExceeded(String),
}

impl CreateDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDocumentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DocumentAlreadyExists" => {
                    return RusotoError::Service(CreateDocumentError::DocumentAlreadyExists(
                        err.msg,
                    ))
                }
                "DocumentLimitExceeded" => {
                    return RusotoError::Service(CreateDocumentError::DocumentLimitExceeded(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateDocumentError::InternalServerError(err.msg))
                }
                "InvalidDocumentContent" => {
                    return RusotoError::Service(CreateDocumentError::InvalidDocumentContent(
                        err.msg,
                    ))
                }
                "InvalidDocumentSchemaVersion" => {
                    return RusotoError::Service(CreateDocumentError::InvalidDocumentSchemaVersion(
                        err.msg,
                    ))
                }
                "MaxDocumentSizeExceeded" => {
                    return RusotoError::Service(CreateDocumentError::MaxDocumentSizeExceeded(
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
impl fmt::Display for CreateDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDocumentError {
    fn description(&self) -> &str {
        match *self {
            CreateDocumentError::DocumentAlreadyExists(ref cause) => cause,
            CreateDocumentError::DocumentLimitExceeded(ref cause) => cause,
            CreateDocumentError::InternalServerError(ref cause) => cause,
            CreateDocumentError::InvalidDocumentContent(ref cause) => cause,
            CreateDocumentError::InvalidDocumentSchemaVersion(ref cause) => cause,
            CreateDocumentError::MaxDocumentSizeExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum CreateMaintenanceWindowError {
    /// <p>Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. </p>
    IdempotentParameterMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Error returned when the caller has exceeded the default resource limits. For example, too many maintenance windows or patch baselines have been created.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    ResourceLimitExceeded(String),
}

impl CreateMaintenanceWindowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMaintenanceWindowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatch" => {
                    return RusotoError::Service(
                        CreateMaintenanceWindowError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateMaintenanceWindowError::InternalServerError(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        CreateMaintenanceWindowError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            CreateMaintenanceWindowError::IdempotentParameterMismatch(ref cause) => cause,
            CreateMaintenanceWindowError::InternalServerError(ref cause) => cause,
            CreateMaintenanceWindowError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateOpsItem
#[derive(Debug, PartialEq)]
pub enum CreateOpsItemError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The OpsItem already exists.</p>
    OpsItemAlreadyExists(String),
    /// <p>A specified parameter argument isn't valid. Verify the available arguments and try again.</p>
    OpsItemInvalidParameter(String),
    /// <p>The request caused OpsItems to exceed one or more limits. For information about OpsItem limits, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-learn-more.html#OpsCenter-learn-more-limits">What are the resource limits for OpsCenter?</a>.</p>
    OpsItemLimitExceeded(String),
}

impl CreateOpsItemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOpsItemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CreateOpsItemError::InternalServerError(err.msg))
                }
                "OpsItemAlreadyExistsException" => {
                    return RusotoError::Service(CreateOpsItemError::OpsItemAlreadyExists(err.msg))
                }
                "OpsItemInvalidParameterException" => {
                    return RusotoError::Service(CreateOpsItemError::OpsItemInvalidParameter(
                        err.msg,
                    ))
                }
                "OpsItemLimitExceededException" => {
                    return RusotoError::Service(CreateOpsItemError::OpsItemLimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateOpsItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateOpsItemError {
    fn description(&self) -> &str {
        match *self {
            CreateOpsItemError::InternalServerError(ref cause) => cause,
            CreateOpsItemError::OpsItemAlreadyExists(ref cause) => cause,
            CreateOpsItemError::OpsItemInvalidParameter(ref cause) => cause,
            CreateOpsItemError::OpsItemLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePatchBaseline
#[derive(Debug, PartialEq)]
pub enum CreatePatchBaselineError {
    /// <p>Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. </p>
    IdempotentParameterMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Error returned when the caller has exceeded the default resource limits. For example, too many maintenance windows or patch baselines have been created.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    ResourceLimitExceeded(String),
}

impl CreatePatchBaselineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePatchBaselineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatch" => {
                    return RusotoError::Service(
                        CreatePatchBaselineError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreatePatchBaselineError::InternalServerError(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreatePatchBaselineError::ResourceLimitExceeded(
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
impl fmt::Display for CreatePatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            CreatePatchBaselineError::IdempotentParameterMismatch(ref cause) => cause,
            CreatePatchBaselineError::InternalServerError(ref cause) => cause,
            CreatePatchBaselineError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateResourceDataSync
#[derive(Debug, PartialEq)]
pub enum CreateResourceDataSyncError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>A sync configuration with the same name already exists.</p>
    ResourceDataSyncAlreadyExists(String),
    /// <p>You have exceeded the allowed maximum sync configurations.</p>
    ResourceDataSyncCountExceeded(String),
    /// <p>The specified sync configuration is invalid.</p>
    ResourceDataSyncInvalidConfiguration(String),
}

impl CreateResourceDataSyncError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateResourceDataSyncError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CreateResourceDataSyncError::InternalServerError(
                        err.msg,
                    ))
                }
                "ResourceDataSyncAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateResourceDataSyncError::ResourceDataSyncAlreadyExists(err.msg),
                    )
                }
                "ResourceDataSyncCountExceededException" => {
                    return RusotoError::Service(
                        CreateResourceDataSyncError::ResourceDataSyncCountExceeded(err.msg),
                    )
                }
                "ResourceDataSyncInvalidConfigurationException" => {
                    return RusotoError::Service(
                        CreateResourceDataSyncError::ResourceDataSyncInvalidConfiguration(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateResourceDataSyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceDataSyncError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceDataSyncError::InternalServerError(ref cause) => cause,
            CreateResourceDataSyncError::ResourceDataSyncAlreadyExists(ref cause) => cause,
            CreateResourceDataSyncError::ResourceDataSyncCountExceeded(ref cause) => cause,
            CreateResourceDataSyncError::ResourceDataSyncInvalidConfiguration(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteActivation
#[derive(Debug, PartialEq)]
pub enum DeleteActivationError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The activation is not valid. The activation might have been deleted, or the ActivationId and the ActivationCode do not match.</p>
    InvalidActivation(String),
    /// <p>The activation ID is not valid. Verify the you entered the correct ActivationId or ActivationCode and try again.</p>
    InvalidActivationId(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
}

impl DeleteActivationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteActivationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteActivationError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidActivation" => {
                    return RusotoError::Service(DeleteActivationError::InvalidActivation(err.msg))
                }
                "InvalidActivationId" => {
                    return RusotoError::Service(DeleteActivationError::InvalidActivationId(
                        err.msg,
                    ))
                }
                "TooManyUpdates" => {
                    return RusotoError::Service(DeleteActivationError::TooManyUpdates(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteActivationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteActivationError {
    fn description(&self) -> &str {
        match *self {
            DeleteActivationError::InternalServerError(ref cause) => cause,
            DeleteActivationError::InvalidActivation(ref cause) => cause,
            DeleteActivationError::InvalidActivationId(ref cause) => cause,
            DeleteActivationError::TooManyUpdates(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAssociation
#[derive(Debug, PartialEq)]
pub enum DeleteAssociationError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
}

impl DeleteAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAssociationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociationDoesNotExist" => {
                    return RusotoError::Service(DeleteAssociationError::AssociationDoesNotExist(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteAssociationError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(DeleteAssociationError::InvalidDocument(err.msg))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(DeleteAssociationError::InvalidInstanceId(err.msg))
                }
                "TooManyUpdates" => {
                    return RusotoError::Service(DeleteAssociationError::TooManyUpdates(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAssociationError {
    fn description(&self) -> &str {
        match *self {
            DeleteAssociationError::AssociationDoesNotExist(ref cause) => cause,
            DeleteAssociationError::InternalServerError(ref cause) => cause,
            DeleteAssociationError::InvalidDocument(ref cause) => cause,
            DeleteAssociationError::InvalidInstanceId(ref cause) => cause,
            DeleteAssociationError::TooManyUpdates(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDocument
#[derive(Debug, PartialEq)]
pub enum DeleteDocumentError {
    /// <p>You must disassociate a document from all instances before you can delete it.</p>
    AssociatedInstances(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>You attempted to delete a document while it is still shared. You must stop sharing the document before you can delete it.</p>
    InvalidDocumentOperation(String),
}

impl DeleteDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDocumentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociatedInstances" => {
                    return RusotoError::Service(DeleteDocumentError::AssociatedInstances(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteDocumentError::InternalServerError(err.msg))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(DeleteDocumentError::InvalidDocument(err.msg))
                }
                "InvalidDocumentOperation" => {
                    return RusotoError::Service(DeleteDocumentError::InvalidDocumentOperation(
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
impl fmt::Display for DeleteDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDocumentError {
    fn description(&self) -> &str {
        match *self {
            DeleteDocumentError::AssociatedInstances(ref cause) => cause,
            DeleteDocumentError::InternalServerError(ref cause) => cause,
            DeleteDocumentError::InvalidDocument(ref cause) => cause,
            DeleteDocumentError::InvalidDocumentOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteInventory
#[derive(Debug, PartialEq)]
pub enum DeleteInventoryError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>One or more of the parameters specified for the delete operation is not valid. Verify all parameters and try again.</p>
    InvalidDeleteInventoryParameters(String),
    /// <p>The request is not valid.</p>
    InvalidInventoryRequest(String),
    /// <p>The delete inventory option specified is not valid. Verify the option and try again.</p>
    InvalidOption(String),
    /// <p>The parameter type name is not valid.</p>
    InvalidTypeName(String),
}

impl DeleteInventoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInventoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteInventoryError::InternalServerError(err.msg))
                }
                "InvalidDeleteInventoryParametersException" => {
                    return RusotoError::Service(
                        DeleteInventoryError::InvalidDeleteInventoryParameters(err.msg),
                    )
                }
                "InvalidInventoryRequestException" => {
                    return RusotoError::Service(DeleteInventoryError::InvalidInventoryRequest(
                        err.msg,
                    ))
                }
                "InvalidOptionException" => {
                    return RusotoError::Service(DeleteInventoryError::InvalidOption(err.msg))
                }
                "InvalidTypeNameException" => {
                    return RusotoError::Service(DeleteInventoryError::InvalidTypeName(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteInventoryError {
    fn description(&self) -> &str {
        match *self {
            DeleteInventoryError::InternalServerError(ref cause) => cause,
            DeleteInventoryError::InvalidDeleteInventoryParameters(ref cause) => cause,
            DeleteInventoryError::InvalidInventoryRequest(ref cause) => cause,
            DeleteInventoryError::InvalidOption(ref cause) => cause,
            DeleteInventoryError::InvalidTypeName(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum DeleteMaintenanceWindowError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DeleteMaintenanceWindowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMaintenanceWindowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteMaintenanceWindowError::InternalServerError(
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
impl fmt::Display for DeleteMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            DeleteMaintenanceWindowError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteParameter
#[derive(Debug, PartialEq)]
pub enum DeleteParameterError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The parameter could not be found. Verify the name and try again.</p>
    ParameterNotFound(String),
}

impl DeleteParameterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteParameterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteParameterError::InternalServerError(err.msg))
                }
                "ParameterNotFound" => {
                    return RusotoError::Service(DeleteParameterError::ParameterNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteParameterError {
    fn description(&self) -> &str {
        match *self {
            DeleteParameterError::InternalServerError(ref cause) => cause,
            DeleteParameterError::ParameterNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteParameters
#[derive(Debug, PartialEq)]
pub enum DeleteParametersError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DeleteParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteParametersError::InternalServerError(
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
impl fmt::Display for DeleteParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteParametersError {
    fn description(&self) -> &str {
        match *self {
            DeleteParametersError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePatchBaseline
#[derive(Debug, PartialEq)]
pub enum DeletePatchBaselineError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Error returned if an attempt is made to delete a patch baseline that is registered for a patch group.</p>
    ResourceInUse(String),
}

impl DeletePatchBaselineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePatchBaselineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeletePatchBaselineError::InternalServerError(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeletePatchBaselineError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            DeletePatchBaselineError::InternalServerError(ref cause) => cause,
            DeletePatchBaselineError::ResourceInUse(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteResourceDataSync
#[derive(Debug, PartialEq)]
pub enum DeleteResourceDataSyncError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified sync name was not found.</p>
    ResourceDataSyncNotFound(String),
}

impl DeleteResourceDataSyncError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourceDataSyncError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteResourceDataSyncError::InternalServerError(
                        err.msg,
                    ))
                }
                "ResourceDataSyncNotFoundException" => {
                    return RusotoError::Service(
                        DeleteResourceDataSyncError::ResourceDataSyncNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteResourceDataSyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourceDataSyncError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourceDataSyncError::InternalServerError(ref cause) => cause,
            DeleteResourceDataSyncError::ResourceDataSyncNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterManagedInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterManagedInstanceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
}

impl DeregisterManagedInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterManagedInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DeregisterManagedInstanceError::InternalServerError(err.msg),
                    )
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(DeregisterManagedInstanceError::InvalidInstanceId(
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
impl fmt::Display for DeregisterManagedInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterManagedInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeregisterManagedInstanceError::InternalServerError(ref cause) => cause,
            DeregisterManagedInstanceError::InvalidInstanceId(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterPatchBaselineForPatchGroup
#[derive(Debug, PartialEq)]
pub enum DeregisterPatchBaselineForPatchGroupError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
}

impl DeregisterPatchBaselineForPatchGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeregisterPatchBaselineForPatchGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DeregisterPatchBaselineForPatchGroupError::InternalServerError(err.msg),
                    )
                }
                "InvalidResourceId" => {
                    return RusotoError::Service(
                        DeregisterPatchBaselineForPatchGroupError::InvalidResourceId(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeregisterPatchBaselineForPatchGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterPatchBaselineForPatchGroupError {
    fn description(&self) -> &str {
        match *self {
            DeregisterPatchBaselineForPatchGroupError::InternalServerError(ref cause) => cause,
            DeregisterPatchBaselineForPatchGroupError::InvalidResourceId(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterTargetFromMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum DeregisterTargetFromMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>You specified the <code>Safe</code> option for the DeregisterTargetFromMaintenanceWindow operation, but the target is still referenced in a task.</p>
    TargetInUse(String),
}

impl DeregisterTargetFromMaintenanceWindowError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeregisterTargetFromMaintenanceWindowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        DeregisterTargetFromMaintenanceWindowError::DoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DeregisterTargetFromMaintenanceWindowError::InternalServerError(err.msg),
                    )
                }
                "TargetInUseException" => {
                    return RusotoError::Service(
                        DeregisterTargetFromMaintenanceWindowError::TargetInUse(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeregisterTargetFromMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterTargetFromMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            DeregisterTargetFromMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            DeregisterTargetFromMaintenanceWindowError::InternalServerError(ref cause) => cause,
            DeregisterTargetFromMaintenanceWindowError::TargetInUse(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterTaskFromMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum DeregisterTaskFromMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DeregisterTaskFromMaintenanceWindowError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeregisterTaskFromMaintenanceWindowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        DeregisterTaskFromMaintenanceWindowError::DoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DeregisterTaskFromMaintenanceWindowError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeregisterTaskFromMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterTaskFromMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            DeregisterTaskFromMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            DeregisterTaskFromMaintenanceWindowError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeActivations
#[derive(Debug, PartialEq)]
pub enum DescribeActivationsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeActivationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeActivationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeActivationsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidFilter" => {
                    return RusotoError::Service(DescribeActivationsError::InvalidFilter(err.msg))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(DescribeActivationsError::InvalidNextToken(
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
impl fmt::Display for DescribeActivationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeActivationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeActivationsError::InternalServerError(ref cause) => cause,
            DescribeActivationsError::InvalidFilter(ref cause) => cause,
            DescribeActivationsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAssociation
#[derive(Debug, PartialEq)]
pub enum DescribeAssociationError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The version you specified is not valid. Use ListAssociationVersions to view all versions of an association according to the association ID. Or, use the <code>$LATEST</code> parameter to view the latest version of the association.</p>
    InvalidAssociationVersion(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
}

impl DescribeAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAssociationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociationDoesNotExist" => {
                    return RusotoError::Service(DescribeAssociationError::AssociationDoesNotExist(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeAssociationError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidAssociationVersion" => {
                    return RusotoError::Service(
                        DescribeAssociationError::InvalidAssociationVersion(err.msg),
                    )
                }
                "InvalidDocument" => {
                    return RusotoError::Service(DescribeAssociationError::InvalidDocument(err.msg))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(DescribeAssociationError::InvalidInstanceId(
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
impl fmt::Display for DescribeAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAssociationError {
    fn description(&self) -> &str {
        match *self {
            DescribeAssociationError::AssociationDoesNotExist(ref cause) => cause,
            DescribeAssociationError::InternalServerError(ref cause) => cause,
            DescribeAssociationError::InvalidAssociationVersion(ref cause) => cause,
            DescribeAssociationError::InvalidDocument(ref cause) => cause,
            DescribeAssociationError::InvalidInstanceId(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAssociationExecutionTargets
#[derive(Debug, PartialEq)]
pub enum DescribeAssociationExecutionTargetsError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>The specified execution ID does not exist. Verify the ID number and try again.</p>
    AssociationExecutionDoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeAssociationExecutionTargetsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAssociationExecutionTargetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociationDoesNotExist" => {
                    return RusotoError::Service(
                        DescribeAssociationExecutionTargetsError::AssociationDoesNotExist(err.msg),
                    )
                }
                "AssociationExecutionDoesNotExist" => {
                    return RusotoError::Service(
                        DescribeAssociationExecutionTargetsError::AssociationExecutionDoesNotExist(
                            err.msg,
                        ),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeAssociationExecutionTargetsError::InternalServerError(err.msg),
                    )
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(
                        DescribeAssociationExecutionTargetsError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAssociationExecutionTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAssociationExecutionTargetsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAssociationExecutionTargetsError::AssociationDoesNotExist(ref cause) => cause,
            DescribeAssociationExecutionTargetsError::AssociationExecutionDoesNotExist(
                ref cause,
            ) => cause,
            DescribeAssociationExecutionTargetsError::InternalServerError(ref cause) => cause,
            DescribeAssociationExecutionTargetsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAssociationExecutions
#[derive(Debug, PartialEq)]
pub enum DescribeAssociationExecutionsError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeAssociationExecutionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAssociationExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociationDoesNotExist" => {
                    return RusotoError::Service(
                        DescribeAssociationExecutionsError::AssociationDoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeAssociationExecutionsError::InternalServerError(err.msg),
                    )
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(
                        DescribeAssociationExecutionsError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAssociationExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAssociationExecutionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAssociationExecutionsError::AssociationDoesNotExist(ref cause) => cause,
            DescribeAssociationExecutionsError::InternalServerError(ref cause) => cause,
            DescribeAssociationExecutionsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAutomationExecutions
#[derive(Debug, PartialEq)]
pub enum DescribeAutomationExecutionsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The filter value is not valid. Verify the value and try again.</p>
    InvalidFilterValue(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeAutomationExecutionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAutomationExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeAutomationExecutionsError::InternalServerError(err.msg),
                    )
                }
                "InvalidFilterKey" => {
                    return RusotoError::Service(
                        DescribeAutomationExecutionsError::InvalidFilterKey(err.msg),
                    )
                }
                "InvalidFilterValue" => {
                    return RusotoError::Service(
                        DescribeAutomationExecutionsError::InvalidFilterValue(err.msg),
                    )
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(
                        DescribeAutomationExecutionsError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAutomationExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAutomationExecutionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAutomationExecutionsError::InternalServerError(ref cause) => cause,
            DescribeAutomationExecutionsError::InvalidFilterKey(ref cause) => cause,
            DescribeAutomationExecutionsError::InvalidFilterValue(ref cause) => cause,
            DescribeAutomationExecutionsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAutomationStepExecutions
#[derive(Debug, PartialEq)]
pub enum DescribeAutomationStepExecutionsError {
    /// <p>There is no automation execution information for the requested automation execution ID.</p>
    AutomationExecutionNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The filter value is not valid. Verify the value and try again.</p>
    InvalidFilterValue(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeAutomationStepExecutionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAutomationStepExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AutomationExecutionNotFoundException" => {
                    return RusotoError::Service(
                        DescribeAutomationStepExecutionsError::AutomationExecutionNotFound(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeAutomationStepExecutionsError::InternalServerError(err.msg),
                    )
                }
                "InvalidFilterKey" => {
                    return RusotoError::Service(
                        DescribeAutomationStepExecutionsError::InvalidFilterKey(err.msg),
                    )
                }
                "InvalidFilterValue" => {
                    return RusotoError::Service(
                        DescribeAutomationStepExecutionsError::InvalidFilterValue(err.msg),
                    )
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(
                        DescribeAutomationStepExecutionsError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAutomationStepExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAutomationStepExecutionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAutomationStepExecutionsError::AutomationExecutionNotFound(ref cause) => cause,
            DescribeAutomationStepExecutionsError::InternalServerError(ref cause) => cause,
            DescribeAutomationStepExecutionsError::InvalidFilterKey(ref cause) => cause,
            DescribeAutomationStepExecutionsError::InvalidFilterValue(ref cause) => cause,
            DescribeAutomationStepExecutionsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAvailablePatches
#[derive(Debug, PartialEq)]
pub enum DescribeAvailablePatchesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeAvailablePatchesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAvailablePatchesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeAvailablePatchesError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAvailablePatchesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAvailablePatchesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAvailablePatchesError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDocument
#[derive(Debug, PartialEq)]
pub enum DescribeDocumentError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
}

impl DescribeDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDocumentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeDocumentError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(DescribeDocumentError::InvalidDocument(err.msg))
                }
                "InvalidDocumentVersion" => {
                    return RusotoError::Service(DescribeDocumentError::InvalidDocumentVersion(
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
impl fmt::Display for DescribeDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDocumentError {
    fn description(&self) -> &str {
        match *self {
            DescribeDocumentError::InternalServerError(ref cause) => cause,
            DescribeDocumentError::InvalidDocument(ref cause) => cause,
            DescribeDocumentError::InvalidDocumentVersion(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDocumentPermission
#[derive(Debug, PartialEq)]
pub enum DescribeDocumentPermissionError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The permission type is not supported. <i>Share</i> is the only supported permission type.</p>
    InvalidPermissionType(String),
}

impl DescribeDocumentPermissionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDocumentPermissionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeDocumentPermissionError::InternalServerError(err.msg),
                    )
                }
                "InvalidDocument" => {
                    return RusotoError::Service(DescribeDocumentPermissionError::InvalidDocument(
                        err.msg,
                    ))
                }
                "InvalidPermissionType" => {
                    return RusotoError::Service(
                        DescribeDocumentPermissionError::InvalidPermissionType(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDocumentPermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDocumentPermissionError {
    fn description(&self) -> &str {
        match *self {
            DescribeDocumentPermissionError::InternalServerError(ref cause) => cause,
            DescribeDocumentPermissionError::InvalidDocument(ref cause) => cause,
            DescribeDocumentPermissionError::InvalidPermissionType(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEffectiveInstanceAssociations
#[derive(Debug, PartialEq)]
pub enum DescribeEffectiveInstanceAssociationsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeEffectiveInstanceAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEffectiveInstanceAssociationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeEffectiveInstanceAssociationsError::InternalServerError(err.msg),
                    )
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(
                        DescribeEffectiveInstanceAssociationsError::InvalidInstanceId(err.msg),
                    )
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(
                        DescribeEffectiveInstanceAssociationsError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEffectiveInstanceAssociationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEffectiveInstanceAssociationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEffectiveInstanceAssociationsError::InternalServerError(ref cause) => cause,
            DescribeEffectiveInstanceAssociationsError::InvalidInstanceId(ref cause) => cause,
            DescribeEffectiveInstanceAssociationsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEffectivePatchesForPatchBaseline
#[derive(Debug, PartialEq)]
pub enum DescribeEffectivePatchesForPatchBaselineError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The operating systems you specified is not supported, or the operation is not supported for the operating system. Valid operating systems include: Windows, AmazonLinux, RedhatEnterpriseLinux, and Ubuntu.</p>
    UnsupportedOperatingSystem(String),
}

impl DescribeEffectivePatchesForPatchBaselineError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEffectivePatchesForPatchBaselineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeEffectivePatchesForPatchBaselineError::DoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeEffectivePatchesForPatchBaselineError::InternalServerError(err.msg),
                    )
                }
                "InvalidResourceId" => {
                    return RusotoError::Service(
                        DescribeEffectivePatchesForPatchBaselineError::InvalidResourceId(err.msg),
                    )
                }
                "UnsupportedOperatingSystem" => {
                    return RusotoError::Service(
                        DescribeEffectivePatchesForPatchBaselineError::UnsupportedOperatingSystem(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEffectivePatchesForPatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEffectivePatchesForPatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            DescribeEffectivePatchesForPatchBaselineError::DoesNotExist(ref cause) => cause,
            DescribeEffectivePatchesForPatchBaselineError::InternalServerError(ref cause) => cause,
            DescribeEffectivePatchesForPatchBaselineError::InvalidResourceId(ref cause) => cause,
            DescribeEffectivePatchesForPatchBaselineError::UnsupportedOperatingSystem(
                ref cause,
            ) => cause,
        }
    }
}
/// Errors returned by DescribeInstanceAssociationsStatus
#[derive(Debug, PartialEq)]
pub enum DescribeInstanceAssociationsStatusError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeInstanceAssociationsStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeInstanceAssociationsStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeInstanceAssociationsStatusError::InternalServerError(err.msg),
                    )
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(
                        DescribeInstanceAssociationsStatusError::InvalidInstanceId(err.msg),
                    )
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(
                        DescribeInstanceAssociationsStatusError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeInstanceAssociationsStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstanceAssociationsStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstanceAssociationsStatusError::InternalServerError(ref cause) => cause,
            DescribeInstanceAssociationsStatusError::InvalidInstanceId(ref cause) => cause,
            DescribeInstanceAssociationsStatusError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstanceInformation
#[derive(Debug, PartialEq)]
pub enum DescribeInstanceInformationError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified filter value is not valid.</p>
    InvalidInstanceInformationFilterValue(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeInstanceInformationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeInstanceInformationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeInstanceInformationError::InternalServerError(err.msg),
                    )
                }
                "InvalidFilterKey" => {
                    return RusotoError::Service(
                        DescribeInstanceInformationError::InvalidFilterKey(err.msg),
                    )
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(
                        DescribeInstanceInformationError::InvalidInstanceId(err.msg),
                    )
                }
                "InvalidInstanceInformationFilterValue" => {
                    return RusotoError::Service(
                        DescribeInstanceInformationError::InvalidInstanceInformationFilterValue(
                            err.msg,
                        ),
                    )
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(
                        DescribeInstanceInformationError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeInstanceInformationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstanceInformationError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstanceInformationError::InternalServerError(ref cause) => cause,
            DescribeInstanceInformationError::InvalidFilterKey(ref cause) => cause,
            DescribeInstanceInformationError::InvalidInstanceId(ref cause) => cause,
            DescribeInstanceInformationError::InvalidInstanceInformationFilterValue(ref cause) => {
                cause
            }
            DescribeInstanceInformationError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstancePatchStates
#[derive(Debug, PartialEq)]
pub enum DescribeInstancePatchStatesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeInstancePatchStatesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeInstancePatchStatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeInstancePatchStatesError::InternalServerError(err.msg),
                    )
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(
                        DescribeInstancePatchStatesError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeInstancePatchStatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstancePatchStatesError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstancePatchStatesError::InternalServerError(ref cause) => cause,
            DescribeInstancePatchStatesError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstancePatchStatesForPatchGroup
#[derive(Debug, PartialEq)]
pub enum DescribeInstancePatchStatesForPatchGroupError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeInstancePatchStatesForPatchGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeInstancePatchStatesForPatchGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeInstancePatchStatesForPatchGroupError::InternalServerError(err.msg),
                    )
                }
                "InvalidFilter" => {
                    return RusotoError::Service(
                        DescribeInstancePatchStatesForPatchGroupError::InvalidFilter(err.msg),
                    )
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(
                        DescribeInstancePatchStatesForPatchGroupError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeInstancePatchStatesForPatchGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstancePatchStatesForPatchGroupError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstancePatchStatesForPatchGroupError::InternalServerError(ref cause) => cause,
            DescribeInstancePatchStatesForPatchGroupError::InvalidFilter(ref cause) => cause,
            DescribeInstancePatchStatesForPatchGroupError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstancePatches
#[derive(Debug, PartialEq)]
pub enum DescribeInstancePatchesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeInstancePatchesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInstancePatchesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeInstancePatchesError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidFilter" => {
                    return RusotoError::Service(DescribeInstancePatchesError::InvalidFilter(
                        err.msg,
                    ))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(DescribeInstancePatchesError::InvalidInstanceId(
                        err.msg,
                    ))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(DescribeInstancePatchesError::InvalidNextToken(
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
impl fmt::Display for DescribeInstancePatchesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstancePatchesError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstancePatchesError::InternalServerError(ref cause) => cause,
            DescribeInstancePatchesError::InvalidFilter(ref cause) => cause,
            DescribeInstancePatchesError::InvalidInstanceId(ref cause) => cause,
            DescribeInstancePatchesError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInventoryDeletions
#[derive(Debug, PartialEq)]
pub enum DescribeInventoryDeletionsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The ID specified for the delete operation does not exist or is not valid. Verify the ID and try again.</p>
    InvalidDeletionId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeInventoryDeletionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeInventoryDeletionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeInventoryDeletionsError::InternalServerError(err.msg),
                    )
                }
                "InvalidDeletionIdException" => {
                    return RusotoError::Service(
                        DescribeInventoryDeletionsError::InvalidDeletionId(err.msg),
                    )
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(DescribeInventoryDeletionsError::InvalidNextToken(
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
impl fmt::Display for DescribeInventoryDeletionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInventoryDeletionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeInventoryDeletionsError::InternalServerError(ref cause) => cause,
            DescribeInventoryDeletionsError::InvalidDeletionId(ref cause) => cause,
            DescribeInventoryDeletionsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowExecutionTaskInvocations
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowExecutionTaskInvocationsError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeMaintenanceWindowExecutionTaskInvocationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMaintenanceWindowExecutionTaskInvocationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowExecutionTaskInvocationsError::DoesNotExist(
                            err.msg,
                        ),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowExecutionTaskInvocationsError::InternalServerError(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeMaintenanceWindowExecutionTaskInvocationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowExecutionTaskInvocationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowExecutionTaskInvocationsError::DoesNotExist(ref cause) => {
                cause
            }
            DescribeMaintenanceWindowExecutionTaskInvocationsError::InternalServerError(
                ref cause,
            ) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowExecutionTasks
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowExecutionTasksError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeMaintenanceWindowExecutionTasksError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMaintenanceWindowExecutionTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowExecutionTasksError::DoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowExecutionTasksError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeMaintenanceWindowExecutionTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowExecutionTasksError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowExecutionTasksError::DoesNotExist(ref cause) => cause,
            DescribeMaintenanceWindowExecutionTasksError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowExecutions
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowExecutionsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeMaintenanceWindowExecutionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMaintenanceWindowExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowExecutionsError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeMaintenanceWindowExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowExecutionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowExecutionsError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowSchedule
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowScheduleError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeMaintenanceWindowScheduleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMaintenanceWindowScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowScheduleError::DoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowScheduleError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeMaintenanceWindowScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowScheduleError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowScheduleError::DoesNotExist(ref cause) => cause,
            DescribeMaintenanceWindowScheduleError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowTargets
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowTargetsError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeMaintenanceWindowTargetsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMaintenanceWindowTargetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowTargetsError::DoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowTargetsError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeMaintenanceWindowTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowTargetsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowTargetsError::DoesNotExist(ref cause) => cause,
            DescribeMaintenanceWindowTargetsError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowTasks
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowTasksError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeMaintenanceWindowTasksError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMaintenanceWindowTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(DescribeMaintenanceWindowTasksError::DoesNotExist(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowTasksError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeMaintenanceWindowTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowTasksError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowTasksError::DoesNotExist(ref cause) => cause,
            DescribeMaintenanceWindowTasksError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindows
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeMaintenanceWindowsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMaintenanceWindowsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowsError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeMaintenanceWindowsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowsError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowsForTarget
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowsForTargetError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeMaintenanceWindowsForTargetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMaintenanceWindowsForTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeMaintenanceWindowsForTargetError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeMaintenanceWindowsForTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowsForTargetError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowsForTargetError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeOpsItems
#[derive(Debug, PartialEq)]
pub enum DescribeOpsItemsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeOpsItemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeOpsItemsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeOpsItemsError::InternalServerError(
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
impl fmt::Display for DescribeOpsItemsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOpsItemsError {
    fn description(&self) -> &str {
        match *self {
            DescribeOpsItemsError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeParameters
#[derive(Debug, PartialEq)]
pub enum DescribeParametersError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The specified filter option is not valid. Valid options are Equals and BeginsWith. For Path filter, valid options are Recursive and OneLevel.</p>
    InvalidFilterOption(String),
    /// <p>The filter value is not valid. Verify the value and try again.</p>
    InvalidFilterValue(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeParametersError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidFilterKey" => {
                    return RusotoError::Service(DescribeParametersError::InvalidFilterKey(err.msg))
                }
                "InvalidFilterOption" => {
                    return RusotoError::Service(DescribeParametersError::InvalidFilterOption(
                        err.msg,
                    ))
                }
                "InvalidFilterValue" => {
                    return RusotoError::Service(DescribeParametersError::InvalidFilterValue(
                        err.msg,
                    ))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(DescribeParametersError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeParametersError::InternalServerError(ref cause) => cause,
            DescribeParametersError::InvalidFilterKey(ref cause) => cause,
            DescribeParametersError::InvalidFilterOption(ref cause) => cause,
            DescribeParametersError::InvalidFilterValue(ref cause) => cause,
            DescribeParametersError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePatchBaselines
#[derive(Debug, PartialEq)]
pub enum DescribePatchBaselinesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribePatchBaselinesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePatchBaselinesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribePatchBaselinesError::InternalServerError(
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
impl fmt::Display for DescribePatchBaselinesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePatchBaselinesError {
    fn description(&self) -> &str {
        match *self {
            DescribePatchBaselinesError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePatchGroupState
#[derive(Debug, PartialEq)]
pub enum DescribePatchGroupStateError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribePatchGroupStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePatchGroupStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribePatchGroupStateError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(DescribePatchGroupStateError::InvalidNextToken(
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
impl fmt::Display for DescribePatchGroupStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePatchGroupStateError {
    fn description(&self) -> &str {
        match *self {
            DescribePatchGroupStateError::InternalServerError(ref cause) => cause,
            DescribePatchGroupStateError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePatchGroups
#[derive(Debug, PartialEq)]
pub enum DescribePatchGroupsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribePatchGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePatchGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribePatchGroupsError::InternalServerError(
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
impl fmt::Display for DescribePatchGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePatchGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribePatchGroupsError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePatchProperties
#[derive(Debug, PartialEq)]
pub enum DescribePatchPropertiesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribePatchPropertiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePatchPropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribePatchPropertiesError::InternalServerError(
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
impl fmt::Display for DescribePatchPropertiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePatchPropertiesError {
    fn description(&self) -> &str {
        match *self {
            DescribePatchPropertiesError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSessions
#[derive(Debug, PartialEq)]
pub enum DescribeSessionsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeSessionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSessionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeSessionsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidFilterKey" => {
                    return RusotoError::Service(DescribeSessionsError::InvalidFilterKey(err.msg))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(DescribeSessionsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeSessionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSessionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeSessionsError::InternalServerError(ref cause) => cause,
            DescribeSessionsError::InvalidFilterKey(ref cause) => cause,
            DescribeSessionsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAutomationExecution
#[derive(Debug, PartialEq)]
pub enum GetAutomationExecutionError {
    /// <p>There is no automation execution information for the requested automation execution ID.</p>
    AutomationExecutionNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl GetAutomationExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAutomationExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AutomationExecutionNotFoundException" => {
                    return RusotoError::Service(
                        GetAutomationExecutionError::AutomationExecutionNotFound(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetAutomationExecutionError::InternalServerError(
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
impl fmt::Display for GetAutomationExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAutomationExecutionError {
    fn description(&self) -> &str {
        match *self {
            GetAutomationExecutionError::AutomationExecutionNotFound(ref cause) => cause,
            GetAutomationExecutionError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCommandInvocation
#[derive(Debug, PartialEq)]
pub enum GetCommandInvocationError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),

    InvalidCommandId(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The plugin name is not valid.</p>
    InvalidPluginName(String),
    /// <p>The command ID and instance ID you specified did not match any invocations. Verify the command ID and the instance ID and try again. </p>
    InvocationDoesNotExist(String),
}

impl GetCommandInvocationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCommandInvocationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetCommandInvocationError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidCommandId" => {
                    return RusotoError::Service(GetCommandInvocationError::InvalidCommandId(
                        err.msg,
                    ))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(GetCommandInvocationError::InvalidInstanceId(
                        err.msg,
                    ))
                }
                "InvalidPluginName" => {
                    return RusotoError::Service(GetCommandInvocationError::InvalidPluginName(
                        err.msg,
                    ))
                }
                "InvocationDoesNotExist" => {
                    return RusotoError::Service(GetCommandInvocationError::InvocationDoesNotExist(
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
impl fmt::Display for GetCommandInvocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCommandInvocationError {
    fn description(&self) -> &str {
        match *self {
            GetCommandInvocationError::InternalServerError(ref cause) => cause,
            GetCommandInvocationError::InvalidCommandId(ref cause) => cause,
            GetCommandInvocationError::InvalidInstanceId(ref cause) => cause,
            GetCommandInvocationError::InvalidPluginName(ref cause) => cause,
            GetCommandInvocationError::InvocationDoesNotExist(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnectionStatus
#[derive(Debug, PartialEq)]
pub enum GetConnectionStatusError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl GetConnectionStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectionStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetConnectionStatusError::InternalServerError(
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
impl fmt::Display for GetConnectionStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectionStatusError {
    fn description(&self) -> &str {
        match *self {
            GetConnectionStatusError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDefaultPatchBaseline
#[derive(Debug, PartialEq)]
pub enum GetDefaultPatchBaselineError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl GetDefaultPatchBaselineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDefaultPatchBaselineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetDefaultPatchBaselineError::InternalServerError(
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
impl fmt::Display for GetDefaultPatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDefaultPatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            GetDefaultPatchBaselineError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeployablePatchSnapshotForInstance
#[derive(Debug, PartialEq)]
pub enum GetDeployablePatchSnapshotForInstanceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Microsoft application patching is only available on EC2 instances and Advanced Instances. To patch Microsoft applications on on-premises servers and VMs, you must enable Advanced Instances. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-managedinstances-advanced.html">Using the Advanced-Instances Tier</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    UnsupportedFeatureRequired(String),
    /// <p>The operating systems you specified is not supported, or the operation is not supported for the operating system. Valid operating systems include: Windows, AmazonLinux, RedhatEnterpriseLinux, and Ubuntu.</p>
    UnsupportedOperatingSystem(String),
}

impl GetDeployablePatchSnapshotForInstanceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDeployablePatchSnapshotForInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        GetDeployablePatchSnapshotForInstanceError::InternalServerError(err.msg),
                    )
                }
                "UnsupportedFeatureRequiredException" => {
                    return RusotoError::Service(
                        GetDeployablePatchSnapshotForInstanceError::UnsupportedFeatureRequired(
                            err.msg,
                        ),
                    )
                }
                "UnsupportedOperatingSystem" => {
                    return RusotoError::Service(
                        GetDeployablePatchSnapshotForInstanceError::UnsupportedOperatingSystem(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDeployablePatchSnapshotForInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeployablePatchSnapshotForInstanceError {
    fn description(&self) -> &str {
        match *self {
            GetDeployablePatchSnapshotForInstanceError::InternalServerError(ref cause) => cause,
            GetDeployablePatchSnapshotForInstanceError::UnsupportedFeatureRequired(ref cause) => {
                cause
            }
            GetDeployablePatchSnapshotForInstanceError::UnsupportedOperatingSystem(ref cause) => {
                cause
            }
        }
    }
}
/// Errors returned by GetDocument
#[derive(Debug, PartialEq)]
pub enum GetDocumentError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
}

impl GetDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDocumentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetDocumentError::InternalServerError(err.msg))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(GetDocumentError::InvalidDocument(err.msg))
                }
                "InvalidDocumentVersion" => {
                    return RusotoError::Service(GetDocumentError::InvalidDocumentVersion(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentError::InternalServerError(ref cause) => cause,
            GetDocumentError::InvalidDocument(ref cause) => cause,
            GetDocumentError::InvalidDocumentVersion(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInventory
#[derive(Debug, PartialEq)]
pub enum GetInventoryError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified aggregator is not valid for inventory groups. Verify that the aggregator uses a valid inventory type such as <code>AWS:Application</code> or <code>AWS:InstanceInformation</code>.</p>
    InvalidAggregator(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified inventory group is not valid.</p>
    InvalidInventoryGroup(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The specified inventory item result attribute is not valid.</p>
    InvalidResultAttribute(String),
    /// <p>The parameter type name is not valid.</p>
    InvalidTypeName(String),
}

impl GetInventoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInventoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetInventoryError::InternalServerError(err.msg))
                }
                "InvalidAggregatorException" => {
                    return RusotoError::Service(GetInventoryError::InvalidAggregator(err.msg))
                }
                "InvalidFilter" => {
                    return RusotoError::Service(GetInventoryError::InvalidFilter(err.msg))
                }
                "InvalidInventoryGroupException" => {
                    return RusotoError::Service(GetInventoryError::InvalidInventoryGroup(err.msg))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(GetInventoryError::InvalidNextToken(err.msg))
                }
                "InvalidResultAttributeException" => {
                    return RusotoError::Service(GetInventoryError::InvalidResultAttribute(err.msg))
                }
                "InvalidTypeNameException" => {
                    return RusotoError::Service(GetInventoryError::InvalidTypeName(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInventoryError {
    fn description(&self) -> &str {
        match *self {
            GetInventoryError::InternalServerError(ref cause) => cause,
            GetInventoryError::InvalidAggregator(ref cause) => cause,
            GetInventoryError::InvalidFilter(ref cause) => cause,
            GetInventoryError::InvalidInventoryGroup(ref cause) => cause,
            GetInventoryError::InvalidNextToken(ref cause) => cause,
            GetInventoryError::InvalidResultAttribute(ref cause) => cause,
            GetInventoryError::InvalidTypeName(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInventorySchema
#[derive(Debug, PartialEq)]
pub enum GetInventorySchemaError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The parameter type name is not valid.</p>
    InvalidTypeName(String),
}

impl GetInventorySchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInventorySchemaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetInventorySchemaError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(GetInventorySchemaError::InvalidNextToken(err.msg))
                }
                "InvalidTypeNameException" => {
                    return RusotoError::Service(GetInventorySchemaError::InvalidTypeName(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetInventorySchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInventorySchemaError {
    fn description(&self) -> &str {
        match *self {
            GetInventorySchemaError::InternalServerError(ref cause) => cause,
            GetInventorySchemaError::InvalidNextToken(ref cause) => cause,
            GetInventorySchemaError::InvalidTypeName(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum GetMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl GetMaintenanceWindowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMaintenanceWindowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(GetMaintenanceWindowError::DoesNotExist(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetMaintenanceWindowError::InternalServerError(
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
impl fmt::Display for GetMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            GetMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            GetMaintenanceWindowError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMaintenanceWindowExecution
#[derive(Debug, PartialEq)]
pub enum GetMaintenanceWindowExecutionError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl GetMaintenanceWindowExecutionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetMaintenanceWindowExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(GetMaintenanceWindowExecutionError::DoesNotExist(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        GetMaintenanceWindowExecutionError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMaintenanceWindowExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMaintenanceWindowExecutionError {
    fn description(&self) -> &str {
        match *self {
            GetMaintenanceWindowExecutionError::DoesNotExist(ref cause) => cause,
            GetMaintenanceWindowExecutionError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMaintenanceWindowExecutionTask
#[derive(Debug, PartialEq)]
pub enum GetMaintenanceWindowExecutionTaskError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl GetMaintenanceWindowExecutionTaskError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetMaintenanceWindowExecutionTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        GetMaintenanceWindowExecutionTaskError::DoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        GetMaintenanceWindowExecutionTaskError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMaintenanceWindowExecutionTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMaintenanceWindowExecutionTaskError {
    fn description(&self) -> &str {
        match *self {
            GetMaintenanceWindowExecutionTaskError::DoesNotExist(ref cause) => cause,
            GetMaintenanceWindowExecutionTaskError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMaintenanceWindowExecutionTaskInvocation
#[derive(Debug, PartialEq)]
pub enum GetMaintenanceWindowExecutionTaskInvocationError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl GetMaintenanceWindowExecutionTaskInvocationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetMaintenanceWindowExecutionTaskInvocationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        GetMaintenanceWindowExecutionTaskInvocationError::DoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        GetMaintenanceWindowExecutionTaskInvocationError::InternalServerError(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMaintenanceWindowExecutionTaskInvocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMaintenanceWindowExecutionTaskInvocationError {
    fn description(&self) -> &str {
        match *self {
            GetMaintenanceWindowExecutionTaskInvocationError::DoesNotExist(ref cause) => cause,
            GetMaintenanceWindowExecutionTaskInvocationError::InternalServerError(ref cause) => {
                cause
            }
        }
    }
}
/// Errors returned by GetMaintenanceWindowTask
#[derive(Debug, PartialEq)]
pub enum GetMaintenanceWindowTaskError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl GetMaintenanceWindowTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMaintenanceWindowTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(GetMaintenanceWindowTaskError::DoesNotExist(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        GetMaintenanceWindowTaskError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMaintenanceWindowTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMaintenanceWindowTaskError {
    fn description(&self) -> &str {
        match *self {
            GetMaintenanceWindowTaskError::DoesNotExist(ref cause) => cause,
            GetMaintenanceWindowTaskError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOpsItem
#[derive(Debug, PartialEq)]
pub enum GetOpsItemError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified OpsItem ID doesn't exist. Verify the ID and try again.</p>
    OpsItemNotFound(String),
}

impl GetOpsItemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOpsItemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetOpsItemError::InternalServerError(err.msg))
                }
                "OpsItemNotFoundException" => {
                    return RusotoError::Service(GetOpsItemError::OpsItemNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetOpsItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOpsItemError {
    fn description(&self) -> &str {
        match *self {
            GetOpsItemError::InternalServerError(ref cause) => cause,
            GetOpsItemError::OpsItemNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOpsSummary
#[derive(Debug, PartialEq)]
pub enum GetOpsSummaryError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified aggregator is not valid for inventory groups. Verify that the aggregator uses a valid inventory type such as <code>AWS:Application</code> or <code>AWS:InstanceInformation</code>.</p>
    InvalidAggregator(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The parameter type name is not valid.</p>
    InvalidTypeName(String),
}

impl GetOpsSummaryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOpsSummaryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetOpsSummaryError::InternalServerError(err.msg))
                }
                "InvalidAggregatorException" => {
                    return RusotoError::Service(GetOpsSummaryError::InvalidAggregator(err.msg))
                }
                "InvalidFilter" => {
                    return RusotoError::Service(GetOpsSummaryError::InvalidFilter(err.msg))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(GetOpsSummaryError::InvalidNextToken(err.msg))
                }
                "InvalidTypeNameException" => {
                    return RusotoError::Service(GetOpsSummaryError::InvalidTypeName(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetOpsSummaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOpsSummaryError {
    fn description(&self) -> &str {
        match *self {
            GetOpsSummaryError::InternalServerError(ref cause) => cause,
            GetOpsSummaryError::InvalidAggregator(ref cause) => cause,
            GetOpsSummaryError::InvalidFilter(ref cause) => cause,
            GetOpsSummaryError::InvalidNextToken(ref cause) => cause,
            GetOpsSummaryError::InvalidTypeName(ref cause) => cause,
        }
    }
}
/// Errors returned by GetParameter
#[derive(Debug, PartialEq)]
pub enum GetParameterError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The query key ID is not valid.</p>
    InvalidKeyId(String),
    /// <p>The parameter could not be found. Verify the name and try again.</p>
    ParameterNotFound(String),
    /// <p>The specified parameter version was not found. Verify the parameter name and version, and try again.</p>
    ParameterVersionNotFound(String),
}

impl GetParameterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetParameterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetParameterError::InternalServerError(err.msg))
                }
                "InvalidKeyId" => {
                    return RusotoError::Service(GetParameterError::InvalidKeyId(err.msg))
                }
                "ParameterNotFound" => {
                    return RusotoError::Service(GetParameterError::ParameterNotFound(err.msg))
                }
                "ParameterVersionNotFound" => {
                    return RusotoError::Service(GetParameterError::ParameterVersionNotFound(
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
impl fmt::Display for GetParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetParameterError {
    fn description(&self) -> &str {
        match *self {
            GetParameterError::InternalServerError(ref cause) => cause,
            GetParameterError::InvalidKeyId(ref cause) => cause,
            GetParameterError::ParameterNotFound(ref cause) => cause,
            GetParameterError::ParameterVersionNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetParameterHistory
#[derive(Debug, PartialEq)]
pub enum GetParameterHistoryError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The query key ID is not valid.</p>
    InvalidKeyId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The parameter could not be found. Verify the name and try again.</p>
    ParameterNotFound(String),
}

impl GetParameterHistoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetParameterHistoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetParameterHistoryError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidKeyId" => {
                    return RusotoError::Service(GetParameterHistoryError::InvalidKeyId(err.msg))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(GetParameterHistoryError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "ParameterNotFound" => {
                    return RusotoError::Service(GetParameterHistoryError::ParameterNotFound(
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
impl fmt::Display for GetParameterHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetParameterHistoryError {
    fn description(&self) -> &str {
        match *self {
            GetParameterHistoryError::InternalServerError(ref cause) => cause,
            GetParameterHistoryError::InvalidKeyId(ref cause) => cause,
            GetParameterHistoryError::InvalidNextToken(ref cause) => cause,
            GetParameterHistoryError::ParameterNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetParameters
#[derive(Debug, PartialEq)]
pub enum GetParametersError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The query key ID is not valid.</p>
    InvalidKeyId(String),
}

impl GetParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetParametersError::InternalServerError(err.msg))
                }
                "InvalidKeyId" => {
                    return RusotoError::Service(GetParametersError::InvalidKeyId(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetParametersError {
    fn description(&self) -> &str {
        match *self {
            GetParametersError::InternalServerError(ref cause) => cause,
            GetParametersError::InvalidKeyId(ref cause) => cause,
        }
    }
}
/// Errors returned by GetParametersByPath
#[derive(Debug, PartialEq)]
pub enum GetParametersByPathError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The specified filter option is not valid. Valid options are Equals and BeginsWith. For Path filter, valid options are Recursive and OneLevel.</p>
    InvalidFilterOption(String),
    /// <p>The filter value is not valid. Verify the value and try again.</p>
    InvalidFilterValue(String),
    /// <p>The query key ID is not valid.</p>
    InvalidKeyId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl GetParametersByPathError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetParametersByPathError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetParametersByPathError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidFilterKey" => {
                    return RusotoError::Service(GetParametersByPathError::InvalidFilterKey(
                        err.msg,
                    ))
                }
                "InvalidFilterOption" => {
                    return RusotoError::Service(GetParametersByPathError::InvalidFilterOption(
                        err.msg,
                    ))
                }
                "InvalidFilterValue" => {
                    return RusotoError::Service(GetParametersByPathError::InvalidFilterValue(
                        err.msg,
                    ))
                }
                "InvalidKeyId" => {
                    return RusotoError::Service(GetParametersByPathError::InvalidKeyId(err.msg))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(GetParametersByPathError::InvalidNextToken(
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
impl fmt::Display for GetParametersByPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetParametersByPathError {
    fn description(&self) -> &str {
        match *self {
            GetParametersByPathError::InternalServerError(ref cause) => cause,
            GetParametersByPathError::InvalidFilterKey(ref cause) => cause,
            GetParametersByPathError::InvalidFilterOption(ref cause) => cause,
            GetParametersByPathError::InvalidFilterValue(ref cause) => cause,
            GetParametersByPathError::InvalidKeyId(ref cause) => cause,
            GetParametersByPathError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPatchBaseline
#[derive(Debug, PartialEq)]
pub enum GetPatchBaselineError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
}

impl GetPatchBaselineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPatchBaselineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(GetPatchBaselineError::DoesNotExist(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetPatchBaselineError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidResourceId" => {
                    return RusotoError::Service(GetPatchBaselineError::InvalidResourceId(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            GetPatchBaselineError::DoesNotExist(ref cause) => cause,
            GetPatchBaselineError::InternalServerError(ref cause) => cause,
            GetPatchBaselineError::InvalidResourceId(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPatchBaselineForPatchGroup
#[derive(Debug, PartialEq)]
pub enum GetPatchBaselineForPatchGroupError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl GetPatchBaselineForPatchGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetPatchBaselineForPatchGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        GetPatchBaselineForPatchGroupError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPatchBaselineForPatchGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPatchBaselineForPatchGroupError {
    fn description(&self) -> &str {
        match *self {
            GetPatchBaselineForPatchGroupError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by GetServiceSetting
#[derive(Debug, PartialEq)]
pub enum GetServiceSettingError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified service setting was not found. Either the service name or the setting has not been provisioned by the AWS service team.</p>
    ServiceSettingNotFound(String),
}

impl GetServiceSettingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetServiceSettingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetServiceSettingError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceSettingNotFound" => {
                    return RusotoError::Service(GetServiceSettingError::ServiceSettingNotFound(
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
impl fmt::Display for GetServiceSettingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetServiceSettingError {
    fn description(&self) -> &str {
        match *self {
            GetServiceSettingError::InternalServerError(ref cause) => cause,
            GetServiceSettingError::ServiceSettingNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by LabelParameterVersion
#[derive(Debug, PartialEq)]
pub enum LabelParameterVersionError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The parameter could not be found. Verify the name and try again.</p>
    ParameterNotFound(String),
    /// <p>A parameter version can have a maximum of ten labels.</p>
    ParameterVersionLabelLimitExceeded(String),
    /// <p>The specified parameter version was not found. Verify the parameter name and version, and try again.</p>
    ParameterVersionNotFound(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
}

impl LabelParameterVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<LabelParameterVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(LabelParameterVersionError::InternalServerError(
                        err.msg,
                    ))
                }
                "ParameterNotFound" => {
                    return RusotoError::Service(LabelParameterVersionError::ParameterNotFound(
                        err.msg,
                    ))
                }
                "ParameterVersionLabelLimitExceeded" => {
                    return RusotoError::Service(
                        LabelParameterVersionError::ParameterVersionLabelLimitExceeded(err.msg),
                    )
                }
                "ParameterVersionNotFound" => {
                    return RusotoError::Service(
                        LabelParameterVersionError::ParameterVersionNotFound(err.msg),
                    )
                }
                "TooManyUpdates" => {
                    return RusotoError::Service(LabelParameterVersionError::TooManyUpdates(
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
impl fmt::Display for LabelParameterVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for LabelParameterVersionError {
    fn description(&self) -> &str {
        match *self {
            LabelParameterVersionError::InternalServerError(ref cause) => cause,
            LabelParameterVersionError::ParameterNotFound(ref cause) => cause,
            LabelParameterVersionError::ParameterVersionLabelLimitExceeded(ref cause) => cause,
            LabelParameterVersionError::ParameterVersionNotFound(ref cause) => cause,
            LabelParameterVersionError::TooManyUpdates(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssociationVersions
#[derive(Debug, PartialEq)]
pub enum ListAssociationVersionsError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl ListAssociationVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAssociationVersionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociationDoesNotExist" => {
                    return RusotoError::Service(
                        ListAssociationVersionsError::AssociationDoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListAssociationVersionsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(ListAssociationVersionsError::InvalidNextToken(
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
impl fmt::Display for ListAssociationVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssociationVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListAssociationVersionsError::AssociationDoesNotExist(ref cause) => cause,
            ListAssociationVersionsError::InternalServerError(ref cause) => cause,
            ListAssociationVersionsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssociations
#[derive(Debug, PartialEq)]
pub enum ListAssociationsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl ListAssociationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAssociationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListAssociationsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(ListAssociationsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAssociationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssociationsError {
    fn description(&self) -> &str {
        match *self {
            ListAssociationsError::InternalServerError(ref cause) => cause,
            ListAssociationsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCommandInvocations
#[derive(Debug, PartialEq)]
pub enum ListCommandInvocationsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),

    InvalidCommandId(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl ListCommandInvocationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCommandInvocationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListCommandInvocationsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidCommandId" => {
                    return RusotoError::Service(ListCommandInvocationsError::InvalidCommandId(
                        err.msg,
                    ))
                }
                "InvalidFilterKey" => {
                    return RusotoError::Service(ListCommandInvocationsError::InvalidFilterKey(
                        err.msg,
                    ))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(ListCommandInvocationsError::InvalidInstanceId(
                        err.msg,
                    ))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(ListCommandInvocationsError::InvalidNextToken(
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
impl fmt::Display for ListCommandInvocationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCommandInvocationsError {
    fn description(&self) -> &str {
        match *self {
            ListCommandInvocationsError::InternalServerError(ref cause) => cause,
            ListCommandInvocationsError::InvalidCommandId(ref cause) => cause,
            ListCommandInvocationsError::InvalidFilterKey(ref cause) => cause,
            ListCommandInvocationsError::InvalidInstanceId(ref cause) => cause,
            ListCommandInvocationsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCommands
#[derive(Debug, PartialEq)]
pub enum ListCommandsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),

    InvalidCommandId(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl ListCommandsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCommandsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListCommandsError::InternalServerError(err.msg))
                }
                "InvalidCommandId" => {
                    return RusotoError::Service(ListCommandsError::InvalidCommandId(err.msg))
                }
                "InvalidFilterKey" => {
                    return RusotoError::Service(ListCommandsError::InvalidFilterKey(err.msg))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(ListCommandsError::InvalidInstanceId(err.msg))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(ListCommandsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListCommandsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCommandsError {
    fn description(&self) -> &str {
        match *self {
            ListCommandsError::InternalServerError(ref cause) => cause,
            ListCommandsError::InvalidCommandId(ref cause) => cause,
            ListCommandsError::InvalidFilterKey(ref cause) => cause,
            ListCommandsError::InvalidInstanceId(ref cause) => cause,
            ListCommandsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListComplianceItems
#[derive(Debug, PartialEq)]
pub enum ListComplianceItemsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The resource type is not valid. For example, if you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
    InvalidResourceType(String),
}

impl ListComplianceItemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListComplianceItemsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListComplianceItemsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidFilter" => {
                    return RusotoError::Service(ListComplianceItemsError::InvalidFilter(err.msg))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(ListComplianceItemsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidResourceId" => {
                    return RusotoError::Service(ListComplianceItemsError::InvalidResourceId(
                        err.msg,
                    ))
                }
                "InvalidResourceType" => {
                    return RusotoError::Service(ListComplianceItemsError::InvalidResourceType(
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
impl fmt::Display for ListComplianceItemsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListComplianceItemsError {
    fn description(&self) -> &str {
        match *self {
            ListComplianceItemsError::InternalServerError(ref cause) => cause,
            ListComplianceItemsError::InvalidFilter(ref cause) => cause,
            ListComplianceItemsError::InvalidNextToken(ref cause) => cause,
            ListComplianceItemsError::InvalidResourceId(ref cause) => cause,
            ListComplianceItemsError::InvalidResourceType(ref cause) => cause,
        }
    }
}
/// Errors returned by ListComplianceSummaries
#[derive(Debug, PartialEq)]
pub enum ListComplianceSummariesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl ListComplianceSummariesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListComplianceSummariesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListComplianceSummariesError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidFilter" => {
                    return RusotoError::Service(ListComplianceSummariesError::InvalidFilter(
                        err.msg,
                    ))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(ListComplianceSummariesError::InvalidNextToken(
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
impl fmt::Display for ListComplianceSummariesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListComplianceSummariesError {
    fn description(&self) -> &str {
        match *self {
            ListComplianceSummariesError::InternalServerError(ref cause) => cause,
            ListComplianceSummariesError::InvalidFilter(ref cause) => cause,
            ListComplianceSummariesError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDocumentVersions
#[derive(Debug, PartialEq)]
pub enum ListDocumentVersionsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl ListDocumentVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDocumentVersionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListDocumentVersionsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(ListDocumentVersionsError::InvalidDocument(
                        err.msg,
                    ))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(ListDocumentVersionsError::InvalidNextToken(
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
impl fmt::Display for ListDocumentVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDocumentVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListDocumentVersionsError::InternalServerError(ref cause) => cause,
            ListDocumentVersionsError::InvalidDocument(ref cause) => cause,
            ListDocumentVersionsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDocuments
#[derive(Debug, PartialEq)]
pub enum ListDocumentsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl ListDocumentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDocumentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListDocumentsError::InternalServerError(err.msg))
                }
                "InvalidFilterKey" => {
                    return RusotoError::Service(ListDocumentsError::InvalidFilterKey(err.msg))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(ListDocumentsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDocumentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDocumentsError {
    fn description(&self) -> &str {
        match *self {
            ListDocumentsError::InternalServerError(ref cause) => cause,
            ListDocumentsError::InvalidFilterKey(ref cause) => cause,
            ListDocumentsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListInventoryEntries
#[derive(Debug, PartialEq)]
pub enum ListInventoryEntriesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The parameter type name is not valid.</p>
    InvalidTypeName(String),
}

impl ListInventoryEntriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInventoryEntriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListInventoryEntriesError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidFilter" => {
                    return RusotoError::Service(ListInventoryEntriesError::InvalidFilter(err.msg))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(ListInventoryEntriesError::InvalidInstanceId(
                        err.msg,
                    ))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(ListInventoryEntriesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidTypeNameException" => {
                    return RusotoError::Service(ListInventoryEntriesError::InvalidTypeName(
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
impl fmt::Display for ListInventoryEntriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListInventoryEntriesError {
    fn description(&self) -> &str {
        match *self {
            ListInventoryEntriesError::InternalServerError(ref cause) => cause,
            ListInventoryEntriesError::InvalidFilter(ref cause) => cause,
            ListInventoryEntriesError::InvalidInstanceId(ref cause) => cause,
            ListInventoryEntriesError::InvalidNextToken(ref cause) => cause,
            ListInventoryEntriesError::InvalidTypeName(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourceComplianceSummaries
#[derive(Debug, PartialEq)]
pub enum ListResourceComplianceSummariesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl ListResourceComplianceSummariesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListResourceComplianceSummariesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        ListResourceComplianceSummariesError::InternalServerError(err.msg),
                    )
                }
                "InvalidFilter" => {
                    return RusotoError::Service(
                        ListResourceComplianceSummariesError::InvalidFilter(err.msg),
                    )
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(
                        ListResourceComplianceSummariesError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListResourceComplianceSummariesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceComplianceSummariesError {
    fn description(&self) -> &str {
        match *self {
            ListResourceComplianceSummariesError::InternalServerError(ref cause) => cause,
            ListResourceComplianceSummariesError::InvalidFilter(ref cause) => cause,
            ListResourceComplianceSummariesError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourceDataSync
#[derive(Debug, PartialEq)]
pub enum ListResourceDataSyncError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
}

impl ListResourceDataSyncError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourceDataSyncError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListResourceDataSyncError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidNextToken" => {
                    return RusotoError::Service(ListResourceDataSyncError::InvalidNextToken(
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
impl fmt::Display for ListResourceDataSyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceDataSyncError {
    fn description(&self) -> &str {
        match *self {
            ListResourceDataSyncError::InternalServerError(ref cause) => cause,
            ListResourceDataSyncError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The resource type is not valid. For example, if you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
    InvalidResourceType(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidResourceId" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidResourceId(
                        err.msg,
                    ))
                }
                "InvalidResourceType" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidResourceType(
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
            ListTagsForResourceError::InternalServerError(ref cause) => cause,
            ListTagsForResourceError::InvalidResourceId(ref cause) => cause,
            ListTagsForResourceError::InvalidResourceType(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyDocumentPermission
#[derive(Debug, PartialEq)]
pub enum ModifyDocumentPermissionError {
    /// <p>You can have at most 500 active Systems Manager documents.</p>
    DocumentLimitExceeded(String),
    /// <p>The document cannot be shared with more AWS user accounts. You can share a document with a maximum of 20 accounts. You can publicly share up to five documents. If you need to increase this limit, contact AWS Support.</p>
    DocumentPermissionLimit(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The permission type is not supported. <i>Share</i> is the only supported permission type.</p>
    InvalidPermissionType(String),
}

impl ModifyDocumentPermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyDocumentPermissionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DocumentLimitExceeded" => {
                    return RusotoError::Service(
                        ModifyDocumentPermissionError::DocumentLimitExceeded(err.msg),
                    )
                }
                "DocumentPermissionLimit" => {
                    return RusotoError::Service(
                        ModifyDocumentPermissionError::DocumentPermissionLimit(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        ModifyDocumentPermissionError::InternalServerError(err.msg),
                    )
                }
                "InvalidDocument" => {
                    return RusotoError::Service(ModifyDocumentPermissionError::InvalidDocument(
                        err.msg,
                    ))
                }
                "InvalidPermissionType" => {
                    return RusotoError::Service(
                        ModifyDocumentPermissionError::InvalidPermissionType(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ModifyDocumentPermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyDocumentPermissionError {
    fn description(&self) -> &str {
        match *self {
            ModifyDocumentPermissionError::DocumentLimitExceeded(ref cause) => cause,
            ModifyDocumentPermissionError::DocumentPermissionLimit(ref cause) => cause,
            ModifyDocumentPermissionError::InternalServerError(ref cause) => cause,
            ModifyDocumentPermissionError::InvalidDocument(ref cause) => cause,
            ModifyDocumentPermissionError::InvalidPermissionType(ref cause) => cause,
        }
    }
}
/// Errors returned by PutComplianceItems
#[derive(Debug, PartialEq)]
pub enum PutComplianceItemsError {
    /// <p>You specified too many custom compliance types. You can specify a maximum of 10 different types. </p>
    ComplianceTypeCountLimitExceeded(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>One or more content items is not valid.</p>
    InvalidItemContent(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The resource type is not valid. For example, if you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
    InvalidResourceType(String),
    /// <p>The inventory item size has exceeded the size limit.</p>
    ItemSizeLimitExceeded(String),
    /// <p>The size of inventory data has exceeded the total size limit for the resource.</p>
    TotalSizeLimitExceeded(String),
}

impl PutComplianceItemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutComplianceItemsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ComplianceTypeCountLimitExceededException" => {
                    return RusotoError::Service(
                        PutComplianceItemsError::ComplianceTypeCountLimitExceeded(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(PutComplianceItemsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidItemContentException" => {
                    return RusotoError::Service(PutComplianceItemsError::InvalidItemContent(
                        err.msg,
                    ))
                }
                "InvalidResourceId" => {
                    return RusotoError::Service(PutComplianceItemsError::InvalidResourceId(
                        err.msg,
                    ))
                }
                "InvalidResourceType" => {
                    return RusotoError::Service(PutComplianceItemsError::InvalidResourceType(
                        err.msg,
                    ))
                }
                "ItemSizeLimitExceededException" => {
                    return RusotoError::Service(PutComplianceItemsError::ItemSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "TotalSizeLimitExceededException" => {
                    return RusotoError::Service(PutComplianceItemsError::TotalSizeLimitExceeded(
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
impl fmt::Display for PutComplianceItemsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutComplianceItemsError {
    fn description(&self) -> &str {
        match *self {
            PutComplianceItemsError::ComplianceTypeCountLimitExceeded(ref cause) => cause,
            PutComplianceItemsError::InternalServerError(ref cause) => cause,
            PutComplianceItemsError::InvalidItemContent(ref cause) => cause,
            PutComplianceItemsError::InvalidResourceId(ref cause) => cause,
            PutComplianceItemsError::InvalidResourceType(ref cause) => cause,
            PutComplianceItemsError::ItemSizeLimitExceeded(ref cause) => cause,
            PutComplianceItemsError::TotalSizeLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by PutInventory
#[derive(Debug, PartialEq)]
pub enum PutInventoryError {
    /// <p>You have exceeded the limit for custom schemas. Delete one or more custom schemas and try again.</p>
    CustomSchemaCountLimitExceeded(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>You specified invalid keys or values in the <code>Context</code> attribute for <code>InventoryItem</code>. Verify the keys and values, and try again.</p>
    InvalidInventoryItemContext(String),
    /// <p>One or more content items is not valid.</p>
    InvalidItemContent(String),
    /// <p>The parameter type name is not valid.</p>
    InvalidTypeName(String),
    /// <p>The inventory item has invalid content. </p>
    ItemContentMismatch(String),
    /// <p>The inventory item size has exceeded the size limit.</p>
    ItemSizeLimitExceeded(String),
    /// <p>The sub-type count exceeded the limit for the inventory type.</p>
    SubTypeCountLimitExceeded(String),
    /// <p>The size of inventory data has exceeded the total size limit for the resource.</p>
    TotalSizeLimitExceeded(String),
    /// <p>The <code>Context</code> attribute that you specified for the <code>InventoryItem</code> is not allowed for this inventory type. You can only use the <code>Context</code> attribute with inventory types like <code>AWS:ComplianceItem</code>.</p>
    UnsupportedInventoryItemContext(String),
    /// <p>Inventory item type schema version has to match supported versions in the service. Check output of GetInventorySchema to see the available schema version for each type.</p>
    UnsupportedInventorySchemaVersion(String),
}

impl PutInventoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutInventoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomSchemaCountLimitExceededException" => {
                    return RusotoError::Service(PutInventoryError::CustomSchemaCountLimitExceeded(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(PutInventoryError::InternalServerError(err.msg))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(PutInventoryError::InvalidInstanceId(err.msg))
                }
                "InvalidInventoryItemContextException" => {
                    return RusotoError::Service(PutInventoryError::InvalidInventoryItemContext(
                        err.msg,
                    ))
                }
                "InvalidItemContentException" => {
                    return RusotoError::Service(PutInventoryError::InvalidItemContent(err.msg))
                }
                "InvalidTypeNameException" => {
                    return RusotoError::Service(PutInventoryError::InvalidTypeName(err.msg))
                }
                "ItemContentMismatchException" => {
                    return RusotoError::Service(PutInventoryError::ItemContentMismatch(err.msg))
                }
                "ItemSizeLimitExceededException" => {
                    return RusotoError::Service(PutInventoryError::ItemSizeLimitExceeded(err.msg))
                }
                "SubTypeCountLimitExceededException" => {
                    return RusotoError::Service(PutInventoryError::SubTypeCountLimitExceeded(
                        err.msg,
                    ))
                }
                "TotalSizeLimitExceededException" => {
                    return RusotoError::Service(PutInventoryError::TotalSizeLimitExceeded(err.msg))
                }
                "UnsupportedInventoryItemContextException" => {
                    return RusotoError::Service(
                        PutInventoryError::UnsupportedInventoryItemContext(err.msg),
                    )
                }
                "UnsupportedInventorySchemaVersionException" => {
                    return RusotoError::Service(
                        PutInventoryError::UnsupportedInventorySchemaVersion(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutInventoryError {
    fn description(&self) -> &str {
        match *self {
            PutInventoryError::CustomSchemaCountLimitExceeded(ref cause) => cause,
            PutInventoryError::InternalServerError(ref cause) => cause,
            PutInventoryError::InvalidInstanceId(ref cause) => cause,
            PutInventoryError::InvalidInventoryItemContext(ref cause) => cause,
            PutInventoryError::InvalidItemContent(ref cause) => cause,
            PutInventoryError::InvalidTypeName(ref cause) => cause,
            PutInventoryError::ItemContentMismatch(ref cause) => cause,
            PutInventoryError::ItemSizeLimitExceeded(ref cause) => cause,
            PutInventoryError::SubTypeCountLimitExceeded(ref cause) => cause,
            PutInventoryError::TotalSizeLimitExceeded(ref cause) => cause,
            PutInventoryError::UnsupportedInventoryItemContext(ref cause) => cause,
            PutInventoryError::UnsupportedInventorySchemaVersion(ref cause) => cause,
        }
    }
}
/// Errors returned by PutParameter
#[derive(Debug, PartialEq)]
pub enum PutParameterError {
    /// <p>A hierarchy can have a maximum of 15 levels. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-parameter-name-constraints.html">Requirements and Constraints for Parameter Names</a> in the <i>AWS Systems Manager User Guide</i>. </p>
    HierarchyLevelLimitExceeded(String),
    /// <p>Parameter Store does not support changing a parameter type in a hierarchy. For example, you can't change a parameter from a String type to a SecureString type. You must create a new, unique parameter.</p>
    HierarchyTypeMismatch(String),
    /// <p>There is a conflict in the policies specified for this parameter. You can't, for example, specify two Expiration policies for a parameter. Review your policies, and try again.</p>
    IncompatiblePolicy(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The request does not meet the regular expression requirement.</p>
    InvalidAllowedPattern(String),
    /// <p>The query key ID is not valid.</p>
    InvalidKeyId(String),
    /// <p>A policy attribute or its value is invalid. </p>
    InvalidPolicyAttribute(String),
    /// <p>The policy type is not supported. Parameter Store supports the following policy types: Expiration, ExpirationNotification, and NoChangeNotification.</p>
    InvalidPolicyType(String),
    /// <p>The parameter already exists. You can't create duplicate parameters.</p>
    ParameterAlreadyExists(String),
    /// <p>You have exceeded the number of parameters for this AWS account. Delete one or more parameters and try again.</p>
    ParameterLimitExceeded(String),
    /// <p>The parameter exceeded the maximum number of allowed versions.</p>
    ParameterMaxVersionLimitExceeded(String),
    /// <p>The parameter name is not valid.</p>
    ParameterPatternMismatch(String),
    /// <p>You specified more than the maximum number of allowed policies for the parameter. The maximum is 10.</p>
    PoliciesLimitExceeded(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
    /// <p>The parameter type is not supported.</p>
    UnsupportedParameterType(String),
}

impl PutParameterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutParameterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "HierarchyLevelLimitExceededException" => {
                    return RusotoError::Service(PutParameterError::HierarchyLevelLimitExceeded(
                        err.msg,
                    ))
                }
                "HierarchyTypeMismatchException" => {
                    return RusotoError::Service(PutParameterError::HierarchyTypeMismatch(err.msg))
                }
                "IncompatiblePolicyException" => {
                    return RusotoError::Service(PutParameterError::IncompatiblePolicy(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(PutParameterError::InternalServerError(err.msg))
                }
                "InvalidAllowedPatternException" => {
                    return RusotoError::Service(PutParameterError::InvalidAllowedPattern(err.msg))
                }
                "InvalidKeyId" => {
                    return RusotoError::Service(PutParameterError::InvalidKeyId(err.msg))
                }
                "InvalidPolicyAttributeException" => {
                    return RusotoError::Service(PutParameterError::InvalidPolicyAttribute(err.msg))
                }
                "InvalidPolicyTypeException" => {
                    return RusotoError::Service(PutParameterError::InvalidPolicyType(err.msg))
                }
                "ParameterAlreadyExists" => {
                    return RusotoError::Service(PutParameterError::ParameterAlreadyExists(err.msg))
                }
                "ParameterLimitExceeded" => {
                    return RusotoError::Service(PutParameterError::ParameterLimitExceeded(err.msg))
                }
                "ParameterMaxVersionLimitExceeded" => {
                    return RusotoError::Service(
                        PutParameterError::ParameterMaxVersionLimitExceeded(err.msg),
                    )
                }
                "ParameterPatternMismatchException" => {
                    return RusotoError::Service(PutParameterError::ParameterPatternMismatch(
                        err.msg,
                    ))
                }
                "PoliciesLimitExceededException" => {
                    return RusotoError::Service(PutParameterError::PoliciesLimitExceeded(err.msg))
                }
                "TooManyUpdates" => {
                    return RusotoError::Service(PutParameterError::TooManyUpdates(err.msg))
                }
                "UnsupportedParameterType" => {
                    return RusotoError::Service(PutParameterError::UnsupportedParameterType(
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
impl fmt::Display for PutParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutParameterError {
    fn description(&self) -> &str {
        match *self {
            PutParameterError::HierarchyLevelLimitExceeded(ref cause) => cause,
            PutParameterError::HierarchyTypeMismatch(ref cause) => cause,
            PutParameterError::IncompatiblePolicy(ref cause) => cause,
            PutParameterError::InternalServerError(ref cause) => cause,
            PutParameterError::InvalidAllowedPattern(ref cause) => cause,
            PutParameterError::InvalidKeyId(ref cause) => cause,
            PutParameterError::InvalidPolicyAttribute(ref cause) => cause,
            PutParameterError::InvalidPolicyType(ref cause) => cause,
            PutParameterError::ParameterAlreadyExists(ref cause) => cause,
            PutParameterError::ParameterLimitExceeded(ref cause) => cause,
            PutParameterError::ParameterMaxVersionLimitExceeded(ref cause) => cause,
            PutParameterError::ParameterPatternMismatch(ref cause) => cause,
            PutParameterError::PoliciesLimitExceeded(ref cause) => cause,
            PutParameterError::TooManyUpdates(ref cause) => cause,
            PutParameterError::UnsupportedParameterType(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterDefaultPatchBaseline
#[derive(Debug, PartialEq)]
pub enum RegisterDefaultPatchBaselineError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
}

impl RegisterDefaultPatchBaselineError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RegisterDefaultPatchBaselineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(RegisterDefaultPatchBaselineError::DoesNotExist(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        RegisterDefaultPatchBaselineError::InternalServerError(err.msg),
                    )
                }
                "InvalidResourceId" => {
                    return RusotoError::Service(
                        RegisterDefaultPatchBaselineError::InvalidResourceId(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterDefaultPatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterDefaultPatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            RegisterDefaultPatchBaselineError::DoesNotExist(ref cause) => cause,
            RegisterDefaultPatchBaselineError::InternalServerError(ref cause) => cause,
            RegisterDefaultPatchBaselineError::InvalidResourceId(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterPatchBaselineForPatchGroup
#[derive(Debug, PartialEq)]
pub enum RegisterPatchBaselineForPatchGroupError {
    /// <p>Error returned if an attempt is made to register a patch group with a patch baseline that is already registered with a different patch baseline.</p>
    AlreadyExists(String),
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>Error returned when the caller has exceeded the default resource limits. For example, too many maintenance windows or patch baselines have been created.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    ResourceLimitExceeded(String),
}

impl RegisterPatchBaselineForPatchGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RegisterPatchBaselineForPatchGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(
                        RegisterPatchBaselineForPatchGroupError::AlreadyExists(err.msg),
                    )
                }
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        RegisterPatchBaselineForPatchGroupError::DoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        RegisterPatchBaselineForPatchGroupError::InternalServerError(err.msg),
                    )
                }
                "InvalidResourceId" => {
                    return RusotoError::Service(
                        RegisterPatchBaselineForPatchGroupError::InvalidResourceId(err.msg),
                    )
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        RegisterPatchBaselineForPatchGroupError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterPatchBaselineForPatchGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterPatchBaselineForPatchGroupError {
    fn description(&self) -> &str {
        match *self {
            RegisterPatchBaselineForPatchGroupError::AlreadyExists(ref cause) => cause,
            RegisterPatchBaselineForPatchGroupError::DoesNotExist(ref cause) => cause,
            RegisterPatchBaselineForPatchGroupError::InternalServerError(ref cause) => cause,
            RegisterPatchBaselineForPatchGroupError::InvalidResourceId(ref cause) => cause,
            RegisterPatchBaselineForPatchGroupError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterTargetWithMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum RegisterTargetWithMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. </p>
    IdempotentParameterMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Error returned when the caller has exceeded the default resource limits. For example, too many maintenance windows or patch baselines have been created.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    ResourceLimitExceeded(String),
}

impl RegisterTargetWithMaintenanceWindowError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RegisterTargetWithMaintenanceWindowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        RegisterTargetWithMaintenanceWindowError::DoesNotExist(err.msg),
                    )
                }
                "IdempotentParameterMismatch" => {
                    return RusotoError::Service(
                        RegisterTargetWithMaintenanceWindowError::IdempotentParameterMismatch(
                            err.msg,
                        ),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        RegisterTargetWithMaintenanceWindowError::InternalServerError(err.msg),
                    )
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        RegisterTargetWithMaintenanceWindowError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterTargetWithMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterTargetWithMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            RegisterTargetWithMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            RegisterTargetWithMaintenanceWindowError::IdempotentParameterMismatch(ref cause) => {
                cause
            }
            RegisterTargetWithMaintenanceWindowError::InternalServerError(ref cause) => cause,
            RegisterTargetWithMaintenanceWindowError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterTaskWithMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum RegisterTaskWithMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>You attempted to register a LAMBDA or STEP_FUNCTIONS task in a region where the corresponding service is not available. </p>
    FeatureNotAvailable(String),
    /// <p>Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. </p>
    IdempotentParameterMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Error returned when the caller has exceeded the default resource limits. For example, too many maintenance windows or patch baselines have been created.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    ResourceLimitExceeded(String),
}

impl RegisterTaskWithMaintenanceWindowError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RegisterTaskWithMaintenanceWindowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(
                        RegisterTaskWithMaintenanceWindowError::DoesNotExist(err.msg),
                    )
                }
                "FeatureNotAvailableException" => {
                    return RusotoError::Service(
                        RegisterTaskWithMaintenanceWindowError::FeatureNotAvailable(err.msg),
                    )
                }
                "IdempotentParameterMismatch" => {
                    return RusotoError::Service(
                        RegisterTaskWithMaintenanceWindowError::IdempotentParameterMismatch(
                            err.msg,
                        ),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        RegisterTaskWithMaintenanceWindowError::InternalServerError(err.msg),
                    )
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        RegisterTaskWithMaintenanceWindowError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterTaskWithMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterTaskWithMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            RegisterTaskWithMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            RegisterTaskWithMaintenanceWindowError::FeatureNotAvailable(ref cause) => cause,
            RegisterTaskWithMaintenanceWindowError::IdempotentParameterMismatch(ref cause) => cause,
            RegisterTaskWithMaintenanceWindowError::InternalServerError(ref cause) => cause,
            RegisterTaskWithMaintenanceWindowError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The resource type is not valid. For example, if you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
    InvalidResourceType(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
}

impl RemoveTagsFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidResourceId" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::InvalidResourceId(
                        err.msg,
                    ))
                }
                "InvalidResourceType" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::InvalidResourceType(
                        err.msg,
                    ))
                }
                "TooManyUpdates" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::TooManyUpdates(
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
impl fmt::Display for RemoveTagsFromResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsFromResourceError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsFromResourceError::InternalServerError(ref cause) => cause,
            RemoveTagsFromResourceError::InvalidResourceId(ref cause) => cause,
            RemoveTagsFromResourceError::InvalidResourceType(ref cause) => cause,
            RemoveTagsFromResourceError::TooManyUpdates(ref cause) => cause,
        }
    }
}
/// Errors returned by ResetServiceSetting
#[derive(Debug, PartialEq)]
pub enum ResetServiceSettingError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified service setting was not found. Either the service name or the setting has not been provisioned by the AWS service team.</p>
    ServiceSettingNotFound(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
}

impl ResetServiceSettingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetServiceSettingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ResetServiceSettingError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceSettingNotFound" => {
                    return RusotoError::Service(ResetServiceSettingError::ServiceSettingNotFound(
                        err.msg,
                    ))
                }
                "TooManyUpdates" => {
                    return RusotoError::Service(ResetServiceSettingError::TooManyUpdates(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ResetServiceSettingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetServiceSettingError {
    fn description(&self) -> &str {
        match *self {
            ResetServiceSettingError::InternalServerError(ref cause) => cause,
            ResetServiceSettingError::ServiceSettingNotFound(ref cause) => cause,
            ResetServiceSettingError::TooManyUpdates(ref cause) => cause,
        }
    }
}
/// Errors returned by ResumeSession
#[derive(Debug, PartialEq)]
pub enum ResumeSessionError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl ResumeSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResumeSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(ResumeSessionError::DoesNotExist(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ResumeSessionError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ResumeSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResumeSessionError {
    fn description(&self) -> &str {
        match *self {
            ResumeSessionError::DoesNotExist(ref cause) => cause,
            ResumeSessionError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by SendAutomationSignal
#[derive(Debug, PartialEq)]
pub enum SendAutomationSignalError {
    /// <p>There is no automation execution information for the requested automation execution ID.</p>
    AutomationExecutionNotFound(String),
    /// <p>The specified step name and execution ID don't exist. Verify the information and try again.</p>
    AutomationStepNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The signal is not valid for the current Automation execution.</p>
    InvalidAutomationSignal(String),
}

impl SendAutomationSignalError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendAutomationSignalError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AutomationExecutionNotFoundException" => {
                    return RusotoError::Service(
                        SendAutomationSignalError::AutomationExecutionNotFound(err.msg),
                    )
                }
                "AutomationStepNotFoundException" => {
                    return RusotoError::Service(SendAutomationSignalError::AutomationStepNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(SendAutomationSignalError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidAutomationSignalException" => {
                    return RusotoError::Service(
                        SendAutomationSignalError::InvalidAutomationSignal(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SendAutomationSignalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendAutomationSignalError {
    fn description(&self) -> &str {
        match *self {
            SendAutomationSignalError::AutomationExecutionNotFound(ref cause) => cause,
            SendAutomationSignalError::AutomationStepNotFound(ref cause) => cause,
            SendAutomationSignalError::InternalServerError(ref cause) => cause,
            SendAutomationSignalError::InvalidAutomationSignal(ref cause) => cause,
        }
    }
}
/// Errors returned by SendCommand
#[derive(Debug, PartialEq)]
pub enum SendCommandError {
    /// <p>You cannot specify an instance ID in more than one association.</p>
    DuplicateInstanceId(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>One or more configuration items is not valid. Verify that a valid Amazon Resource Name (ARN) was provided for an Amazon SNS topic.</p>
    InvalidNotificationConfig(String),
    /// <p>The S3 bucket does not exist.</p>
    InvalidOutputFolder(String),
    /// <p>You must specify values for all required parameters in the Systems Manager document. You can only supply values to parameters defined in the Systems Manager document.</p>
    InvalidParameters(String),
    /// <p>The role name can't contain invalid characters. Also verify that you specified an IAM role for notifications that includes the required trust policy. For information about configuring the IAM role for Run Command notifications, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/rc-sns-notifications.html">Configuring Amazon SNS Notifications for Run Command</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    InvalidRole(String),
    /// <p>The size limit of a document is 64 KB.</p>
    MaxDocumentSizeExceeded(String),
    /// <p>The document does not support the platform type of the given instance ID(s). For example, you sent an document for a Windows instance to a Linux instance.</p>
    UnsupportedPlatformType(String),
}

impl SendCommandError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendCommandError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateInstanceId" => {
                    return RusotoError::Service(SendCommandError::DuplicateInstanceId(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(SendCommandError::InternalServerError(err.msg))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(SendCommandError::InvalidDocument(err.msg))
                }
                "InvalidDocumentVersion" => {
                    return RusotoError::Service(SendCommandError::InvalidDocumentVersion(err.msg))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(SendCommandError::InvalidInstanceId(err.msg))
                }
                "InvalidNotificationConfig" => {
                    return RusotoError::Service(SendCommandError::InvalidNotificationConfig(
                        err.msg,
                    ))
                }
                "InvalidOutputFolder" => {
                    return RusotoError::Service(SendCommandError::InvalidOutputFolder(err.msg))
                }
                "InvalidParameters" => {
                    return RusotoError::Service(SendCommandError::InvalidParameters(err.msg))
                }
                "InvalidRole" => {
                    return RusotoError::Service(SendCommandError::InvalidRole(err.msg))
                }
                "MaxDocumentSizeExceeded" => {
                    return RusotoError::Service(SendCommandError::MaxDocumentSizeExceeded(err.msg))
                }
                "UnsupportedPlatformType" => {
                    return RusotoError::Service(SendCommandError::UnsupportedPlatformType(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SendCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendCommandError {
    fn description(&self) -> &str {
        match *self {
            SendCommandError::DuplicateInstanceId(ref cause) => cause,
            SendCommandError::InternalServerError(ref cause) => cause,
            SendCommandError::InvalidDocument(ref cause) => cause,
            SendCommandError::InvalidDocumentVersion(ref cause) => cause,
            SendCommandError::InvalidInstanceId(ref cause) => cause,
            SendCommandError::InvalidNotificationConfig(ref cause) => cause,
            SendCommandError::InvalidOutputFolder(ref cause) => cause,
            SendCommandError::InvalidParameters(ref cause) => cause,
            SendCommandError::InvalidRole(ref cause) => cause,
            SendCommandError::MaxDocumentSizeExceeded(ref cause) => cause,
            SendCommandError::UnsupportedPlatformType(ref cause) => cause,
        }
    }
}
/// Errors returned by StartAssociationsOnce
#[derive(Debug, PartialEq)]
pub enum StartAssociationsOnceError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>The association is not valid or does not exist. </p>
    InvalidAssociation(String),
}

impl StartAssociationsOnceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartAssociationsOnceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociationDoesNotExist" => {
                    return RusotoError::Service(
                        StartAssociationsOnceError::AssociationDoesNotExist(err.msg),
                    )
                }
                "InvalidAssociation" => {
                    return RusotoError::Service(StartAssociationsOnceError::InvalidAssociation(
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
impl fmt::Display for StartAssociationsOnceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartAssociationsOnceError {
    fn description(&self) -> &str {
        match *self {
            StartAssociationsOnceError::AssociationDoesNotExist(ref cause) => cause,
            StartAssociationsOnceError::InvalidAssociation(ref cause) => cause,
        }
    }
}
/// Errors returned by StartAutomationExecution
#[derive(Debug, PartialEq)]
pub enum StartAutomationExecutionError {
    /// <p>An Automation document with the specified name could not be found.</p>
    AutomationDefinitionNotFound(String),
    /// <p>An Automation document with the specified name and version could not be found.</p>
    AutomationDefinitionVersionNotFound(String),
    /// <p>The number of simultaneously running Automation executions exceeded the allowable limit.</p>
    AutomationExecutionLimitExceeded(String),
    /// <p>Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. </p>
    IdempotentParameterMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The supplied parameters for invoking the specified Automation document are incorrect. For example, they may not match the set of parameters permitted for the specified Automation document.</p>
    InvalidAutomationExecutionParameters(String),
    /// <p>The target is not valid or does not exist. It might not be configured for EC2 Systems Manager or you might not have permission to perform the operation.</p>
    InvalidTarget(String),
}

impl StartAutomationExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartAutomationExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AutomationDefinitionNotFoundException" => {
                    return RusotoError::Service(
                        StartAutomationExecutionError::AutomationDefinitionNotFound(err.msg),
                    )
                }
                "AutomationDefinitionVersionNotFoundException" => {
                    return RusotoError::Service(
                        StartAutomationExecutionError::AutomationDefinitionVersionNotFound(err.msg),
                    )
                }
                "AutomationExecutionLimitExceededException" => {
                    return RusotoError::Service(
                        StartAutomationExecutionError::AutomationExecutionLimitExceeded(err.msg),
                    )
                }
                "IdempotentParameterMismatch" => {
                    return RusotoError::Service(
                        StartAutomationExecutionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        StartAutomationExecutionError::InternalServerError(err.msg),
                    )
                }
                "InvalidAutomationExecutionParametersException" => {
                    return RusotoError::Service(
                        StartAutomationExecutionError::InvalidAutomationExecutionParameters(
                            err.msg,
                        ),
                    )
                }
                "InvalidTarget" => {
                    return RusotoError::Service(StartAutomationExecutionError::InvalidTarget(
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
impl fmt::Display for StartAutomationExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartAutomationExecutionError {
    fn description(&self) -> &str {
        match *self {
            StartAutomationExecutionError::AutomationDefinitionNotFound(ref cause) => cause,
            StartAutomationExecutionError::AutomationDefinitionVersionNotFound(ref cause) => cause,
            StartAutomationExecutionError::AutomationExecutionLimitExceeded(ref cause) => cause,
            StartAutomationExecutionError::IdempotentParameterMismatch(ref cause) => cause,
            StartAutomationExecutionError::InternalServerError(ref cause) => cause,
            StartAutomationExecutionError::InvalidAutomationExecutionParameters(ref cause) => cause,
            StartAutomationExecutionError::InvalidTarget(ref cause) => cause,
        }
    }
}
/// Errors returned by StartSession
#[derive(Debug, PartialEq)]
pub enum StartSessionError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The specified target instance for the session is not fully configured for use with Session Manager. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager-getting-started.html">Getting Started with Session Manager</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    TargetNotConnected(String),
}

impl StartSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(StartSessionError::InternalServerError(err.msg))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(StartSessionError::InvalidDocument(err.msg))
                }
                "TargetNotConnected" => {
                    return RusotoError::Service(StartSessionError::TargetNotConnected(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartSessionError {
    fn description(&self) -> &str {
        match *self {
            StartSessionError::InternalServerError(ref cause) => cause,
            StartSessionError::InvalidDocument(ref cause) => cause,
            StartSessionError::TargetNotConnected(ref cause) => cause,
        }
    }
}
/// Errors returned by StopAutomationExecution
#[derive(Debug, PartialEq)]
pub enum StopAutomationExecutionError {
    /// <p>There is no automation execution information for the requested automation execution ID.</p>
    AutomationExecutionNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified update status operation is not valid.</p>
    InvalidAutomationStatusUpdate(String),
}

impl StopAutomationExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopAutomationExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AutomationExecutionNotFoundException" => {
                    return RusotoError::Service(
                        StopAutomationExecutionError::AutomationExecutionNotFound(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(StopAutomationExecutionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidAutomationStatusUpdateException" => {
                    return RusotoError::Service(
                        StopAutomationExecutionError::InvalidAutomationStatusUpdate(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopAutomationExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopAutomationExecutionError {
    fn description(&self) -> &str {
        match *self {
            StopAutomationExecutionError::AutomationExecutionNotFound(ref cause) => cause,
            StopAutomationExecutionError::InternalServerError(ref cause) => cause,
            StopAutomationExecutionError::InvalidAutomationStatusUpdate(ref cause) => cause,
        }
    }
}
/// Errors returned by TerminateSession
#[derive(Debug, PartialEq)]
pub enum TerminateSessionError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl TerminateSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TerminateSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(TerminateSessionError::DoesNotExist(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(TerminateSessionError::InternalServerError(
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
impl fmt::Display for TerminateSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateSessionError {
    fn description(&self) -> &str {
        match *self {
            TerminateSessionError::DoesNotExist(ref cause) => cause,
            TerminateSessionError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAssociation
#[derive(Debug, PartialEq)]
pub enum UpdateAssociationError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>You have reached the maximum number versions allowed for an association. Each association has a limit of 1,000 versions. </p>
    AssociationVersionLimitExceeded(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The version you specified is not valid. Use ListAssociationVersions to view all versions of an association according to the association ID. Or, use the <code>$LATEST</code> parameter to view the latest version of the association.</p>
    InvalidAssociationVersion(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// <p>The output location is not valid or does not exist.</p>
    InvalidOutputLocation(String),
    /// <p>You must specify values for all required parameters in the Systems Manager document. You can only supply values to parameters defined in the Systems Manager document.</p>
    InvalidParameters(String),
    /// <p>The schedule is invalid. Verify your cron or rate expression and try again.</p>
    InvalidSchedule(String),
    /// <p>The target is not valid or does not exist. It might not be configured for EC2 Systems Manager or you might not have permission to perform the operation.</p>
    InvalidTarget(String),
    /// <p>The update is not valid.</p>
    InvalidUpdate(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
}

impl UpdateAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAssociationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociationDoesNotExist" => {
                    return RusotoError::Service(UpdateAssociationError::AssociationDoesNotExist(
                        err.msg,
                    ))
                }
                "AssociationVersionLimitExceeded" => {
                    return RusotoError::Service(
                        UpdateAssociationError::AssociationVersionLimitExceeded(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(UpdateAssociationError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidAssociationVersion" => {
                    return RusotoError::Service(UpdateAssociationError::InvalidAssociationVersion(
                        err.msg,
                    ))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(UpdateAssociationError::InvalidDocument(err.msg))
                }
                "InvalidDocumentVersion" => {
                    return RusotoError::Service(UpdateAssociationError::InvalidDocumentVersion(
                        err.msg,
                    ))
                }
                "InvalidOutputLocation" => {
                    return RusotoError::Service(UpdateAssociationError::InvalidOutputLocation(
                        err.msg,
                    ))
                }
                "InvalidParameters" => {
                    return RusotoError::Service(UpdateAssociationError::InvalidParameters(err.msg))
                }
                "InvalidSchedule" => {
                    return RusotoError::Service(UpdateAssociationError::InvalidSchedule(err.msg))
                }
                "InvalidTarget" => {
                    return RusotoError::Service(UpdateAssociationError::InvalidTarget(err.msg))
                }
                "InvalidUpdate" => {
                    return RusotoError::Service(UpdateAssociationError::InvalidUpdate(err.msg))
                }
                "TooManyUpdates" => {
                    return RusotoError::Service(UpdateAssociationError::TooManyUpdates(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAssociationError {
    fn description(&self) -> &str {
        match *self {
            UpdateAssociationError::AssociationDoesNotExist(ref cause) => cause,
            UpdateAssociationError::AssociationVersionLimitExceeded(ref cause) => cause,
            UpdateAssociationError::InternalServerError(ref cause) => cause,
            UpdateAssociationError::InvalidAssociationVersion(ref cause) => cause,
            UpdateAssociationError::InvalidDocument(ref cause) => cause,
            UpdateAssociationError::InvalidDocumentVersion(ref cause) => cause,
            UpdateAssociationError::InvalidOutputLocation(ref cause) => cause,
            UpdateAssociationError::InvalidParameters(ref cause) => cause,
            UpdateAssociationError::InvalidSchedule(ref cause) => cause,
            UpdateAssociationError::InvalidTarget(ref cause) => cause,
            UpdateAssociationError::InvalidUpdate(ref cause) => cause,
            UpdateAssociationError::TooManyUpdates(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAssociationStatus
#[derive(Debug, PartialEq)]
pub enum UpdateAssociationStatusError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The updated status is the same as the current status.</p>
    StatusUnchanged(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
}

impl UpdateAssociationStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAssociationStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AssociationDoesNotExist" => {
                    return RusotoError::Service(
                        UpdateAssociationStatusError::AssociationDoesNotExist(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(UpdateAssociationStatusError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(UpdateAssociationStatusError::InvalidDocument(
                        err.msg,
                    ))
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(UpdateAssociationStatusError::InvalidInstanceId(
                        err.msg,
                    ))
                }
                "StatusUnchanged" => {
                    return RusotoError::Service(UpdateAssociationStatusError::StatusUnchanged(
                        err.msg,
                    ))
                }
                "TooManyUpdates" => {
                    return RusotoError::Service(UpdateAssociationStatusError::TooManyUpdates(
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
impl fmt::Display for UpdateAssociationStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAssociationStatusError {
    fn description(&self) -> &str {
        match *self {
            UpdateAssociationStatusError::AssociationDoesNotExist(ref cause) => cause,
            UpdateAssociationStatusError::InternalServerError(ref cause) => cause,
            UpdateAssociationStatusError::InvalidDocument(ref cause) => cause,
            UpdateAssociationStatusError::InvalidInstanceId(ref cause) => cause,
            UpdateAssociationStatusError::StatusUnchanged(ref cause) => cause,
            UpdateAssociationStatusError::TooManyUpdates(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDocument
#[derive(Debug, PartialEq)]
pub enum UpdateDocumentError {
    /// <p>The document has too many versions. Delete one or more document versions and try again.</p>
    DocumentVersionLimitExceeded(String),
    /// <p>The content of the association document matches another document. Change the content of the document and try again.</p>
    DuplicateDocumentContent(String),
    /// <p>The version name has already been used in this document. Specify a different version name, and then try again.</p>
    DuplicateDocumentVersionName(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The content for the document is not valid.</p>
    InvalidDocumentContent(String),
    /// <p>You attempted to delete a document while it is still shared. You must stop sharing the document before you can delete it.</p>
    InvalidDocumentOperation(String),
    /// <p>The version of the document schema is not supported.</p>
    InvalidDocumentSchemaVersion(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// <p>The size limit of a document is 64 KB.</p>
    MaxDocumentSizeExceeded(String),
}

impl UpdateDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDocumentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DocumentVersionLimitExceeded" => {
                    return RusotoError::Service(UpdateDocumentError::DocumentVersionLimitExceeded(
                        err.msg,
                    ))
                }
                "DuplicateDocumentContent" => {
                    return RusotoError::Service(UpdateDocumentError::DuplicateDocumentContent(
                        err.msg,
                    ))
                }
                "DuplicateDocumentVersionName" => {
                    return RusotoError::Service(UpdateDocumentError::DuplicateDocumentVersionName(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(UpdateDocumentError::InternalServerError(err.msg))
                }
                "InvalidDocument" => {
                    return RusotoError::Service(UpdateDocumentError::InvalidDocument(err.msg))
                }
                "InvalidDocumentContent" => {
                    return RusotoError::Service(UpdateDocumentError::InvalidDocumentContent(
                        err.msg,
                    ))
                }
                "InvalidDocumentOperation" => {
                    return RusotoError::Service(UpdateDocumentError::InvalidDocumentOperation(
                        err.msg,
                    ))
                }
                "InvalidDocumentSchemaVersion" => {
                    return RusotoError::Service(UpdateDocumentError::InvalidDocumentSchemaVersion(
                        err.msg,
                    ))
                }
                "InvalidDocumentVersion" => {
                    return RusotoError::Service(UpdateDocumentError::InvalidDocumentVersion(
                        err.msg,
                    ))
                }
                "MaxDocumentSizeExceeded" => {
                    return RusotoError::Service(UpdateDocumentError::MaxDocumentSizeExceeded(
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
impl fmt::Display for UpdateDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDocumentError {
    fn description(&self) -> &str {
        match *self {
            UpdateDocumentError::DocumentVersionLimitExceeded(ref cause) => cause,
            UpdateDocumentError::DuplicateDocumentContent(ref cause) => cause,
            UpdateDocumentError::DuplicateDocumentVersionName(ref cause) => cause,
            UpdateDocumentError::InternalServerError(ref cause) => cause,
            UpdateDocumentError::InvalidDocument(ref cause) => cause,
            UpdateDocumentError::InvalidDocumentContent(ref cause) => cause,
            UpdateDocumentError::InvalidDocumentOperation(ref cause) => cause,
            UpdateDocumentError::InvalidDocumentSchemaVersion(ref cause) => cause,
            UpdateDocumentError::InvalidDocumentVersion(ref cause) => cause,
            UpdateDocumentError::MaxDocumentSizeExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDocumentDefaultVersion
#[derive(Debug, PartialEq)]
pub enum UpdateDocumentDefaultVersionError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The version of the document schema is not supported.</p>
    InvalidDocumentSchemaVersion(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
}

impl UpdateDocumentDefaultVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDocumentDefaultVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateDocumentDefaultVersionError::InternalServerError(err.msg),
                    )
                }
                "InvalidDocument" => {
                    return RusotoError::Service(
                        UpdateDocumentDefaultVersionError::InvalidDocument(err.msg),
                    )
                }
                "InvalidDocumentSchemaVersion" => {
                    return RusotoError::Service(
                        UpdateDocumentDefaultVersionError::InvalidDocumentSchemaVersion(err.msg),
                    )
                }
                "InvalidDocumentVersion" => {
                    return RusotoError::Service(
                        UpdateDocumentDefaultVersionError::InvalidDocumentVersion(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDocumentDefaultVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDocumentDefaultVersionError {
    fn description(&self) -> &str {
        match *self {
            UpdateDocumentDefaultVersionError::InternalServerError(ref cause) => cause,
            UpdateDocumentDefaultVersionError::InvalidDocument(ref cause) => cause,
            UpdateDocumentDefaultVersionError::InvalidDocumentSchemaVersion(ref cause) => cause,
            UpdateDocumentDefaultVersionError::InvalidDocumentVersion(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum UpdateMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl UpdateMaintenanceWindowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMaintenanceWindowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(UpdateMaintenanceWindowError::DoesNotExist(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(UpdateMaintenanceWindowError::InternalServerError(
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
impl fmt::Display for UpdateMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            UpdateMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            UpdateMaintenanceWindowError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMaintenanceWindowTarget
#[derive(Debug, PartialEq)]
pub enum UpdateMaintenanceWindowTargetError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl UpdateMaintenanceWindowTargetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateMaintenanceWindowTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(UpdateMaintenanceWindowTargetError::DoesNotExist(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateMaintenanceWindowTargetError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateMaintenanceWindowTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMaintenanceWindowTargetError {
    fn description(&self) -> &str {
        match *self {
            UpdateMaintenanceWindowTargetError::DoesNotExist(ref cause) => cause,
            UpdateMaintenanceWindowTargetError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMaintenanceWindowTask
#[derive(Debug, PartialEq)]
pub enum UpdateMaintenanceWindowTaskError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl UpdateMaintenanceWindowTaskError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateMaintenanceWindowTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(UpdateMaintenanceWindowTaskError::DoesNotExist(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateMaintenanceWindowTaskError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateMaintenanceWindowTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMaintenanceWindowTaskError {
    fn description(&self) -> &str {
        match *self {
            UpdateMaintenanceWindowTaskError::DoesNotExist(ref cause) => cause,
            UpdateMaintenanceWindowTaskError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateManagedInstanceRole
#[derive(Debug, PartialEq)]
pub enum UpdateManagedInstanceRoleError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>SSM Agent is not running. Verify that SSM Agent is running.</p> <p>SSM Agent is not registered with the SSM endpoint. Try reinstalling SSM Agent.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
}

impl UpdateManagedInstanceRoleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateManagedInstanceRoleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateManagedInstanceRoleError::InternalServerError(err.msg),
                    )
                }
                "InvalidInstanceId" => {
                    return RusotoError::Service(UpdateManagedInstanceRoleError::InvalidInstanceId(
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
impl fmt::Display for UpdateManagedInstanceRoleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateManagedInstanceRoleError {
    fn description(&self) -> &str {
        match *self {
            UpdateManagedInstanceRoleError::InternalServerError(ref cause) => cause,
            UpdateManagedInstanceRoleError::InvalidInstanceId(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateOpsItem
#[derive(Debug, PartialEq)]
pub enum UpdateOpsItemError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The OpsItem already exists.</p>
    OpsItemAlreadyExists(String),
    /// <p>A specified parameter argument isn't valid. Verify the available arguments and try again.</p>
    OpsItemInvalidParameter(String),
    /// <p>The request caused OpsItems to exceed one or more limits. For information about OpsItem limits, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-learn-more.html#OpsCenter-learn-more-limits">What are the resource limits for OpsCenter?</a>.</p>
    OpsItemLimitExceeded(String),
    /// <p>The specified OpsItem ID doesn't exist. Verify the ID and try again.</p>
    OpsItemNotFound(String),
}

impl UpdateOpsItemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateOpsItemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(UpdateOpsItemError::InternalServerError(err.msg))
                }
                "OpsItemAlreadyExistsException" => {
                    return RusotoError::Service(UpdateOpsItemError::OpsItemAlreadyExists(err.msg))
                }
                "OpsItemInvalidParameterException" => {
                    return RusotoError::Service(UpdateOpsItemError::OpsItemInvalidParameter(
                        err.msg,
                    ))
                }
                "OpsItemLimitExceededException" => {
                    return RusotoError::Service(UpdateOpsItemError::OpsItemLimitExceeded(err.msg))
                }
                "OpsItemNotFoundException" => {
                    return RusotoError::Service(UpdateOpsItemError::OpsItemNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateOpsItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateOpsItemError {
    fn description(&self) -> &str {
        match *self {
            UpdateOpsItemError::InternalServerError(ref cause) => cause,
            UpdateOpsItemError::OpsItemAlreadyExists(ref cause) => cause,
            UpdateOpsItemError::OpsItemInvalidParameter(ref cause) => cause,
            UpdateOpsItemError::OpsItemLimitExceeded(ref cause) => cause,
            UpdateOpsItemError::OpsItemNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePatchBaseline
#[derive(Debug, PartialEq)]
pub enum UpdatePatchBaselineError {
    /// <p>Error returned when the ID specified for a resource, such as a maintenance window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
}

impl UpdatePatchBaselineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePatchBaselineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DoesNotExistException" => {
                    return RusotoError::Service(UpdatePatchBaselineError::DoesNotExist(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(UpdatePatchBaselineError::InternalServerError(
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
impl fmt::Display for UpdatePatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            UpdatePatchBaselineError::DoesNotExist(ref cause) => cause,
            UpdatePatchBaselineError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateServiceSetting
#[derive(Debug, PartialEq)]
pub enum UpdateServiceSettingError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified service setting was not found. Either the service name or the setting has not been provisioned by the AWS service team.</p>
    ServiceSettingNotFound(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
}

impl UpdateServiceSettingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateServiceSettingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(UpdateServiceSettingError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceSettingNotFound" => {
                    return RusotoError::Service(UpdateServiceSettingError::ServiceSettingNotFound(
                        err.msg,
                    ))
                }
                "TooManyUpdates" => {
                    return RusotoError::Service(UpdateServiceSettingError::TooManyUpdates(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateServiceSettingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateServiceSettingError {
    fn description(&self) -> &str {
        match *self {
            UpdateServiceSettingError::InternalServerError(ref cause) => cause,
            UpdateServiceSettingError::ServiceSettingNotFound(ref cause) => cause,
            UpdateServiceSettingError::TooManyUpdates(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon SSM API. Amazon SSM clients implement this trait.
pub trait Ssm {
    /// <p>Adds or overwrites one or more tags for the specified resource. Tags are metadata that you can assign to your documents, managed instances, maintenance windows, Parameter Store parameters, and patch baselines. Tags enable you to categorize your resources in different ways, for example, by purpose, owner, or environment. Each tag consists of a key and an optional value, both of which you define. For example, you could define a set of tags for your account's managed instances that helps you track each instance's owner and stack level. For example: Key=Owner and Value=DbAdmin, SysAdmin, or Dev. Or Key=Stack and Value=Production, Pre-Production, or Test.</p> <p>Each resource can have a maximum of 50 tags. </p> <p>We recommend that you devise a set of tag keys that meets your needs for each resource type. Using a consistent set of tag keys makes it easier for you to manage your resources. You can search and filter the resources based on the tags you add. Tags don't have any semantic meaning to Amazon EC2 and are interpreted strictly as a string of characters. </p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html">Tagging Your Amazon EC2 Resources</a> in the <i>Amazon EC2 User Guide</i>.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceRequest,
    ) -> RusotoFuture<AddTagsToResourceResult, AddTagsToResourceError>;

    /// <p>Attempts to cancel the command specified by the Command ID. There is no guarantee that the command will be terminated and the underlying process stopped.</p>
    fn cancel_command(
        &self,
        input: CancelCommandRequest,
    ) -> RusotoFuture<CancelCommandResult, CancelCommandError>;

    /// <p>Stops a maintenance window execution that is already in progress and cancels any tasks in the window that have not already starting running. (Tasks already in progress will continue to completion.)</p>
    fn cancel_maintenance_window_execution(
        &self,
        input: CancelMaintenanceWindowExecutionRequest,
    ) -> RusotoFuture<CancelMaintenanceWindowExecutionResult, CancelMaintenanceWindowExecutionError>;

    /// <p>Registers your on-premises server or virtual machine with Amazon EC2 so that you can manage these resources using Run Command. An on-premises server or virtual machine that has been registered with EC2 is called a managed instance. For more information about activations, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-managedinstances.html">Setting Up AWS Systems Manager for Hybrid Environments</a>.</p>
    fn create_activation(
        &self,
        input: CreateActivationRequest,
    ) -> RusotoFuture<CreateActivationResult, CreateActivationError>;

    /// <p>Associates the specified Systems Manager document with the specified instances or targets.</p> <p>When you associate a document with one or more instances using instance IDs or tags, SSM Agent running on the instance processes the document and configures the instance as specified.</p> <p>If you associate a document with an instance that already has an associated document, the system returns the AssociationAlreadyExists exception.</p>
    fn create_association(
        &self,
        input: CreateAssociationRequest,
    ) -> RusotoFuture<CreateAssociationResult, CreateAssociationError>;

    /// <p>Associates the specified Systems Manager document with the specified instances or targets.</p> <p>When you associate a document with one or more instances using instance IDs or tags, SSM Agent running on the instance processes the document and configures the instance as specified.</p> <p>If you associate a document with an instance that already has an associated document, the system returns the AssociationAlreadyExists exception.</p>
    fn create_association_batch(
        &self,
        input: CreateAssociationBatchRequest,
    ) -> RusotoFuture<CreateAssociationBatchResult, CreateAssociationBatchError>;

    /// <p>Creates a Systems Manager document.</p> <p>After you create a document, you can use CreateAssociation to associate it with one or more running instances.</p>
    fn create_document(
        &self,
        input: CreateDocumentRequest,
    ) -> RusotoFuture<CreateDocumentResult, CreateDocumentError>;

    /// <p>Creates a new maintenance window.</p>
    fn create_maintenance_window(
        &self,
        input: CreateMaintenanceWindowRequest,
    ) -> RusotoFuture<CreateMaintenanceWindowResult, CreateMaintenanceWindowError>;

    /// <p>Creates a new OpsItem. You must have permission in AWS Identity and Access Management (IAM) to create a new OpsItem. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-getting-started.html">Getting Started with OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>.</p> <p>Operations engineers and IT professionals use OpsCenter to view, investigate, and remediate operational issues impacting the performance and health of their AWS resources. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter.html">AWS Systems Manager OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>. </p>
    fn create_ops_item(
        &self,
        input: CreateOpsItemRequest,
    ) -> RusotoFuture<CreateOpsItemResponse, CreateOpsItemError>;

    /// <p><p>Creates a patch baseline.</p> <note> <p>For information about valid key and value pairs in <code>PatchFilters</code> for each supported operating system type, see <a href="http://docs.aws.amazon.com/systems-manager/latest/APIReference/API_PatchFilter.html">PatchFilter</a>.</p> </note></p>
    fn create_patch_baseline(
        &self,
        input: CreatePatchBaselineRequest,
    ) -> RusotoFuture<CreatePatchBaselineResult, CreatePatchBaselineError>;

    /// <p>Creates a resource data sync configuration to a single bucket in Amazon S3. This is an asynchronous operation that returns immediately. After a successful initial sync is completed, the system continuously syncs data to the Amazon S3 bucket. To check the status of the sync, use the <a>ListResourceDataSync</a>.</p> <p>By default, data is not encrypted in Amazon S3. We strongly recommend that you enable encryption in Amazon S3 to ensure secure data storage. We also recommend that you secure access to the Amazon S3 bucket by creating a restrictive bucket policy. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-inventory-datasync.html">Configuring Resource Data Sync for Inventory</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    fn create_resource_data_sync(
        &self,
        input: CreateResourceDataSyncRequest,
    ) -> RusotoFuture<CreateResourceDataSyncResult, CreateResourceDataSyncError>;

    /// <p>Deletes an activation. You are not required to delete an activation. If you delete an activation, you can no longer use it to register additional managed instances. Deleting an activation does not de-register managed instances. You must manually de-register managed instances.</p>
    fn delete_activation(
        &self,
        input: DeleteActivationRequest,
    ) -> RusotoFuture<DeleteActivationResult, DeleteActivationError>;

    /// <p>Disassociates the specified Systems Manager document from the specified instance.</p> <p>When you disassociate a document from an instance, it does not change the configuration of the instance. To change the configuration state of an instance after you disassociate a document, you must create a new document with the desired configuration and associate it with the instance.</p>
    fn delete_association(
        &self,
        input: DeleteAssociationRequest,
    ) -> RusotoFuture<DeleteAssociationResult, DeleteAssociationError>;

    /// <p>Deletes the Systems Manager document and all instance associations to the document.</p> <p>Before you delete the document, we recommend that you use <a>DeleteAssociation</a> to disassociate all instances that are associated with the document.</p>
    fn delete_document(
        &self,
        input: DeleteDocumentRequest,
    ) -> RusotoFuture<DeleteDocumentResult, DeleteDocumentError>;

    /// <p>Delete a custom inventory type, or the data associated with a custom Inventory type. Deleting a custom inventory type is also referred to as deleting a custom inventory schema.</p>
    fn delete_inventory(
        &self,
        input: DeleteInventoryRequest,
    ) -> RusotoFuture<DeleteInventoryResult, DeleteInventoryError>;

    /// <p>Deletes a maintenance window.</p>
    fn delete_maintenance_window(
        &self,
        input: DeleteMaintenanceWindowRequest,
    ) -> RusotoFuture<DeleteMaintenanceWindowResult, DeleteMaintenanceWindowError>;

    /// <p>Delete a parameter from the system.</p>
    fn delete_parameter(
        &self,
        input: DeleteParameterRequest,
    ) -> RusotoFuture<DeleteParameterResult, DeleteParameterError>;

    /// <p>Delete a list of parameters.</p>
    fn delete_parameters(
        &self,
        input: DeleteParametersRequest,
    ) -> RusotoFuture<DeleteParametersResult, DeleteParametersError>;

    /// <p>Deletes a patch baseline.</p>
    fn delete_patch_baseline(
        &self,
        input: DeletePatchBaselineRequest,
    ) -> RusotoFuture<DeletePatchBaselineResult, DeletePatchBaselineError>;

    /// <p>Deletes a Resource Data Sync configuration. After the configuration is deleted, changes to inventory data on managed instances are no longer synced with the target Amazon S3 bucket. Deleting a sync configuration does not delete data in the target Amazon S3 bucket.</p>
    fn delete_resource_data_sync(
        &self,
        input: DeleteResourceDataSyncRequest,
    ) -> RusotoFuture<DeleteResourceDataSyncResult, DeleteResourceDataSyncError>;

    /// <p>Removes the server or virtual machine from the list of registered servers. You can reregister the instance again at any time. If you don't plan to use Run Command on the server, we suggest uninstalling SSM Agent first.</p>
    fn deregister_managed_instance(
        &self,
        input: DeregisterManagedInstanceRequest,
    ) -> RusotoFuture<DeregisterManagedInstanceResult, DeregisterManagedInstanceError>;

    /// <p>Removes a patch group from a patch baseline.</p>
    fn deregister_patch_baseline_for_patch_group(
        &self,
        input: DeregisterPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<
        DeregisterPatchBaselineForPatchGroupResult,
        DeregisterPatchBaselineForPatchGroupError,
    >;

    /// <p>Removes a target from a maintenance window.</p>
    fn deregister_target_from_maintenance_window(
        &self,
        input: DeregisterTargetFromMaintenanceWindowRequest,
    ) -> RusotoFuture<
        DeregisterTargetFromMaintenanceWindowResult,
        DeregisterTargetFromMaintenanceWindowError,
    >;

    /// <p>Removes a task from a maintenance window.</p>
    fn deregister_task_from_maintenance_window(
        &self,
        input: DeregisterTaskFromMaintenanceWindowRequest,
    ) -> RusotoFuture<
        DeregisterTaskFromMaintenanceWindowResult,
        DeregisterTaskFromMaintenanceWindowError,
    >;

    /// <p>Describes details about the activation, such as the date and time the activation was created, its expiration date, the IAM role assigned to the instances in the activation, and the number of instances registered by using this activation.</p>
    fn describe_activations(
        &self,
        input: DescribeActivationsRequest,
    ) -> RusotoFuture<DescribeActivationsResult, DescribeActivationsError>;

    /// <p>Describes the association for the specified target or instance. If you created the association by using the <code>Targets</code> parameter, then you must retrieve the association by using the association ID. If you created the association by specifying an instance ID and a Systems Manager document, then you retrieve the association by specifying the document name and the instance ID. </p>
    fn describe_association(
        &self,
        input: DescribeAssociationRequest,
    ) -> RusotoFuture<DescribeAssociationResult, DescribeAssociationError>;

    /// <p>Use this API action to view information about a specific execution of a specific association.</p>
    fn describe_association_execution_targets(
        &self,
        input: DescribeAssociationExecutionTargetsRequest,
    ) -> RusotoFuture<
        DescribeAssociationExecutionTargetsResult,
        DescribeAssociationExecutionTargetsError,
    >;

    /// <p>Use this API action to view all executions for a specific association ID. </p>
    fn describe_association_executions(
        &self,
        input: DescribeAssociationExecutionsRequest,
    ) -> RusotoFuture<DescribeAssociationExecutionsResult, DescribeAssociationExecutionsError>;

    /// <p>Provides details about all active and terminated Automation executions.</p>
    fn describe_automation_executions(
        &self,
        input: DescribeAutomationExecutionsRequest,
    ) -> RusotoFuture<DescribeAutomationExecutionsResult, DescribeAutomationExecutionsError>;

    /// <p>Information about all active and terminated step executions in an Automation workflow.</p>
    fn describe_automation_step_executions(
        &self,
        input: DescribeAutomationStepExecutionsRequest,
    ) -> RusotoFuture<DescribeAutomationStepExecutionsResult, DescribeAutomationStepExecutionsError>;

    /// <p>Lists all patches eligible to be included in a patch baseline.</p>
    fn describe_available_patches(
        &self,
        input: DescribeAvailablePatchesRequest,
    ) -> RusotoFuture<DescribeAvailablePatchesResult, DescribeAvailablePatchesError>;

    /// <p>Describes the specified Systems Manager document.</p>
    fn describe_document(
        &self,
        input: DescribeDocumentRequest,
    ) -> RusotoFuture<DescribeDocumentResult, DescribeDocumentError>;

    /// <p>Describes the permissions for a Systems Manager document. If you created the document, you are the owner. If a document is shared, it can either be shared privately (by specifying a user's AWS account ID) or publicly (<i>All</i>). </p>
    fn describe_document_permission(
        &self,
        input: DescribeDocumentPermissionRequest,
    ) -> RusotoFuture<DescribeDocumentPermissionResponse, DescribeDocumentPermissionError>;

    /// <p>All associations for the instance(s).</p>
    fn describe_effective_instance_associations(
        &self,
        input: DescribeEffectiveInstanceAssociationsRequest,
    ) -> RusotoFuture<
        DescribeEffectiveInstanceAssociationsResult,
        DescribeEffectiveInstanceAssociationsError,
    >;

    /// <p>Retrieves the current effective patches (the patch and the approval state) for the specified patch baseline. Note that this API applies only to Windows patch baselines.</p>
    fn describe_effective_patches_for_patch_baseline(
        &self,
        input: DescribeEffectivePatchesForPatchBaselineRequest,
    ) -> RusotoFuture<
        DescribeEffectivePatchesForPatchBaselineResult,
        DescribeEffectivePatchesForPatchBaselineError,
    >;

    /// <p>The status of the associations for the instance(s).</p>
    fn describe_instance_associations_status(
        &self,
        input: DescribeInstanceAssociationsStatusRequest,
    ) -> RusotoFuture<
        DescribeInstanceAssociationsStatusResult,
        DescribeInstanceAssociationsStatusError,
    >;

    /// <p><p>Describes one or more of your instances. You can use this to get information about instances like the operating system platform, the SSM Agent version (Linux), status etc. If you specify one or more instance IDs, it returns information for those instances. If you do not specify instance IDs, it returns information for all your instances. If you specify an instance ID that is not valid or an instance that you do not own, you receive an error. </p> <note> <p>The IamRole field for this API action is the Amazon Identity and Access Management (IAM) role assigned to on-premises instances. This call does not return the IAM role for Amazon EC2 instances.</p> </note></p>
    fn describe_instance_information(
        &self,
        input: DescribeInstanceInformationRequest,
    ) -> RusotoFuture<DescribeInstanceInformationResult, DescribeInstanceInformationError>;

    /// <p>Retrieves the high-level patch state of one or more instances.</p>
    fn describe_instance_patch_states(
        &self,
        input: DescribeInstancePatchStatesRequest,
    ) -> RusotoFuture<DescribeInstancePatchStatesResult, DescribeInstancePatchStatesError>;

    /// <p>Retrieves the high-level patch state for the instances in the specified patch group.</p>
    fn describe_instance_patch_states_for_patch_group(
        &self,
        input: DescribeInstancePatchStatesForPatchGroupRequest,
    ) -> RusotoFuture<
        DescribeInstancePatchStatesForPatchGroupResult,
        DescribeInstancePatchStatesForPatchGroupError,
    >;

    /// <p>Retrieves information about the patches on the specified instance and their state relative to the patch baseline being used for the instance.</p>
    fn describe_instance_patches(
        &self,
        input: DescribeInstancePatchesRequest,
    ) -> RusotoFuture<DescribeInstancePatchesResult, DescribeInstancePatchesError>;

    /// <p>Describes a specific delete inventory operation.</p>
    fn describe_inventory_deletions(
        &self,
        input: DescribeInventoryDeletionsRequest,
    ) -> RusotoFuture<DescribeInventoryDeletionsResult, DescribeInventoryDeletionsError>;

    /// <p>Retrieves the individual task executions (one per target) for a particular task run as part of a maintenance window execution.</p>
    fn describe_maintenance_window_execution_task_invocations(
        &self,
        input: DescribeMaintenanceWindowExecutionTaskInvocationsRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionTaskInvocationsResult,
        DescribeMaintenanceWindowExecutionTaskInvocationsError,
    >;

    /// <p>For a given maintenance window execution, lists the tasks that were run.</p>
    fn describe_maintenance_window_execution_tasks(
        &self,
        input: DescribeMaintenanceWindowExecutionTasksRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionTasksResult,
        DescribeMaintenanceWindowExecutionTasksError,
    >;

    /// <p>Lists the executions of a maintenance window. This includes information about when the maintenance window was scheduled to be active, and information about tasks registered and run with the maintenance window.</p>
    fn describe_maintenance_window_executions(
        &self,
        input: DescribeMaintenanceWindowExecutionsRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionsResult,
        DescribeMaintenanceWindowExecutionsError,
    >;

    /// <p>Retrieves information about upcoming executions of a maintenance window.</p>
    fn describe_maintenance_window_schedule(
        &self,
        input: DescribeMaintenanceWindowScheduleRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowScheduleResult, DescribeMaintenanceWindowScheduleError>;

    /// <p>Lists the targets registered with the maintenance window.</p>
    fn describe_maintenance_window_targets(
        &self,
        input: DescribeMaintenanceWindowTargetsRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowTargetsResult, DescribeMaintenanceWindowTargetsError>;

    /// <p>Lists the tasks in a maintenance window.</p>
    fn describe_maintenance_window_tasks(
        &self,
        input: DescribeMaintenanceWindowTasksRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowTasksResult, DescribeMaintenanceWindowTasksError>;

    /// <p>Retrieves the maintenance windows in an AWS account.</p>
    fn describe_maintenance_windows(
        &self,
        input: DescribeMaintenanceWindowsRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowsResult, DescribeMaintenanceWindowsError>;

    /// <p>Retrieves information about the maintenance window targets or tasks that an instance is associated with.</p>
    fn describe_maintenance_windows_for_target(
        &self,
        input: DescribeMaintenanceWindowsForTargetRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowsForTargetResult,
        DescribeMaintenanceWindowsForTargetError,
    >;

    /// <p>Query a set of OpsItems. You must have permission in AWS Identity and Access Management (IAM) to query a list of OpsItems. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-getting-started.html">Getting Started with OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>.</p> <p>Operations engineers and IT professionals use OpsCenter to view, investigate, and remediate operational issues impacting the performance and health of their AWS resources. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter.html">AWS Systems Manager OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>. </p>
    fn describe_ops_items(
        &self,
        input: DescribeOpsItemsRequest,
    ) -> RusotoFuture<DescribeOpsItemsResponse, DescribeOpsItemsError>;

    /// <p>Get information about a parameter.</p> <p>Request results are returned on a best-effort basis. If you specify <code>MaxResults</code> in the request, the response includes information up to the limit specified. The number of items returned, however, can be between zero and the value of <code>MaxResults</code>. If the service reaches an internal limit while processing the results, it stops the operation and returns the matching values up to that point and a <code>NextToken</code>. You can specify the <code>NextToken</code> in a subsequent call to get the next set of results.</p>
    fn describe_parameters(
        &self,
        input: DescribeParametersRequest,
    ) -> RusotoFuture<DescribeParametersResult, DescribeParametersError>;

    /// <p>Lists the patch baselines in your AWS account.</p>
    fn describe_patch_baselines(
        &self,
        input: DescribePatchBaselinesRequest,
    ) -> RusotoFuture<DescribePatchBaselinesResult, DescribePatchBaselinesError>;

    /// <p>Returns high-level aggregated patch compliance state for a patch group.</p>
    fn describe_patch_group_state(
        &self,
        input: DescribePatchGroupStateRequest,
    ) -> RusotoFuture<DescribePatchGroupStateResult, DescribePatchGroupStateError>;

    /// <p>Lists all patch groups that have been registered with patch baselines.</p>
    fn describe_patch_groups(
        &self,
        input: DescribePatchGroupsRequest,
    ) -> RusotoFuture<DescribePatchGroupsResult, DescribePatchGroupsError>;

    /// <p><p>Lists the properties of available patches organized by product, product family, classification, severity, and other properties of available patches. You can use the reported properties in the filters you specify in requests for actions such as <a>CreatePatchBaseline</a>, <a>UpdatePatchBaseline</a>, <a>DescribeAvailablePatches</a>, and <a>DescribePatchBaselines</a>.</p> <p>The following section lists the properties that can be used in filters for each major operating system type:</p> <dl> <dt>WINDOWS</dt> <dd> <p>Valid properties: PRODUCT, PRODUCT<em>FAMILY, CLASSIFICATION, MSRC</em>SEVERITY</p> </dd> <dt>AMAZON<em>LINUX</dt> <dd> <p>Valid properties: PRODUCT, CLASSIFICATION, SEVERITY</p> </dd> <dt>AMAZON</em>LINUX<em>2</dt> <dd> <p>Valid properties: PRODUCT, CLASSIFICATION, SEVERITY</p> </dd> <dt>UBUNTU </dt> <dd> <p>Valid properties: PRODUCT, PRIORITY</p> </dd> <dt>REDHAT</em>ENTERPRISE_LINUX</dt> <dd> <p>Valid properties: PRODUCT, CLASSIFICATION, SEVERITY</p> </dd> <dt>SUSE</dt> <dd> <p>Valid properties: PRODUCT, CLASSIFICATION, SEVERITY</p> </dd> <dt>CENTOS</dt> <dd> <p>Valid properties: PRODUCT, CLASSIFICATION, SEVERITY</p> </dd> </dl></p>
    fn describe_patch_properties(
        &self,
        input: DescribePatchPropertiesRequest,
    ) -> RusotoFuture<DescribePatchPropertiesResult, DescribePatchPropertiesError>;

    /// <p>Retrieves a list of all active sessions (both connected and disconnected) or terminated sessions from the past 30 days.</p>
    fn describe_sessions(
        &self,
        input: DescribeSessionsRequest,
    ) -> RusotoFuture<DescribeSessionsResponse, DescribeSessionsError>;

    /// <p>Get detailed information about a particular Automation execution.</p>
    fn get_automation_execution(
        &self,
        input: GetAutomationExecutionRequest,
    ) -> RusotoFuture<GetAutomationExecutionResult, GetAutomationExecutionError>;

    /// <p>Returns detailed information about command execution for an invocation or plugin. </p>
    fn get_command_invocation(
        &self,
        input: GetCommandInvocationRequest,
    ) -> RusotoFuture<GetCommandInvocationResult, GetCommandInvocationError>;

    /// <p>Retrieves the Session Manager connection status for an instance to determine whether it is connected and ready to receive Session Manager connections.</p>
    fn get_connection_status(
        &self,
        input: GetConnectionStatusRequest,
    ) -> RusotoFuture<GetConnectionStatusResponse, GetConnectionStatusError>;

    /// <p>Retrieves the default patch baseline. Note that Systems Manager supports creating multiple default patch baselines. For example, you can create a default patch baseline for each operating system.</p> <p>If you do not specify an operating system value, the default patch baseline for Windows is returned.</p>
    fn get_default_patch_baseline(
        &self,
        input: GetDefaultPatchBaselineRequest,
    ) -> RusotoFuture<GetDefaultPatchBaselineResult, GetDefaultPatchBaselineError>;

    /// <p>Retrieves the current snapshot for the patch baseline the instance uses. This API is primarily used by the AWS-RunPatchBaseline Systems Manager document. </p>
    fn get_deployable_patch_snapshot_for_instance(
        &self,
        input: GetDeployablePatchSnapshotForInstanceRequest,
    ) -> RusotoFuture<
        GetDeployablePatchSnapshotForInstanceResult,
        GetDeployablePatchSnapshotForInstanceError,
    >;

    /// <p>Gets the contents of the specified Systems Manager document.</p>
    fn get_document(
        &self,
        input: GetDocumentRequest,
    ) -> RusotoFuture<GetDocumentResult, GetDocumentError>;

    /// <p>Query inventory information.</p>
    fn get_inventory(
        &self,
        input: GetInventoryRequest,
    ) -> RusotoFuture<GetInventoryResult, GetInventoryError>;

    /// <p>Return a list of inventory type names for the account, or return a list of attribute names for a specific Inventory item type. </p>
    fn get_inventory_schema(
        &self,
        input: GetInventorySchemaRequest,
    ) -> RusotoFuture<GetInventorySchemaResult, GetInventorySchemaError>;

    /// <p>Retrieves a maintenance window.</p>
    fn get_maintenance_window(
        &self,
        input: GetMaintenanceWindowRequest,
    ) -> RusotoFuture<GetMaintenanceWindowResult, GetMaintenanceWindowError>;

    /// <p>Retrieves details about a specific a maintenance window execution.</p>
    fn get_maintenance_window_execution(
        &self,
        input: GetMaintenanceWindowExecutionRequest,
    ) -> RusotoFuture<GetMaintenanceWindowExecutionResult, GetMaintenanceWindowExecutionError>;

    /// <p>Retrieves the details about a specific task run as part of a maintenance window execution.</p>
    fn get_maintenance_window_execution_task(
        &self,
        input: GetMaintenanceWindowExecutionTaskRequest,
    ) -> RusotoFuture<GetMaintenanceWindowExecutionTaskResult, GetMaintenanceWindowExecutionTaskError>;

    /// <p>Retrieves information about a specific task running on a specific target.</p>
    fn get_maintenance_window_execution_task_invocation(
        &self,
        input: GetMaintenanceWindowExecutionTaskInvocationRequest,
    ) -> RusotoFuture<
        GetMaintenanceWindowExecutionTaskInvocationResult,
        GetMaintenanceWindowExecutionTaskInvocationError,
    >;

    /// <p>Lists the tasks in a maintenance window.</p>
    fn get_maintenance_window_task(
        &self,
        input: GetMaintenanceWindowTaskRequest,
    ) -> RusotoFuture<GetMaintenanceWindowTaskResult, GetMaintenanceWindowTaskError>;

    /// <p>Get information about an OpsItem by using the ID. You must have permission in AWS Identity and Access Management (IAM) to view information about an OpsItem. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-getting-started.html">Getting Started with OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>.</p> <p>Operations engineers and IT professionals use OpsCenter to view, investigate, and remediate operational issues impacting the performance and health of their AWS resources. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter.html">AWS Systems Manager OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>. </p>
    fn get_ops_item(
        &self,
        input: GetOpsItemRequest,
    ) -> RusotoFuture<GetOpsItemResponse, GetOpsItemError>;

    /// <p>View a summary of OpsItems based on specified filters and aggregators.</p>
    fn get_ops_summary(
        &self,
        input: GetOpsSummaryRequest,
    ) -> RusotoFuture<GetOpsSummaryResult, GetOpsSummaryError>;

    /// <p>Get information about a parameter by using the parameter name. Don't confuse this API action with the <a>GetParameters</a> API action.</p>
    fn get_parameter(
        &self,
        input: GetParameterRequest,
    ) -> RusotoFuture<GetParameterResult, GetParameterError>;

    /// <p>Query a list of all parameters used by the AWS account.</p>
    fn get_parameter_history(
        &self,
        input: GetParameterHistoryRequest,
    ) -> RusotoFuture<GetParameterHistoryResult, GetParameterHistoryError>;

    /// <p>Get details of a parameter. Don't confuse this API action with the <a>GetParameter</a> API action.</p>
    fn get_parameters(
        &self,
        input: GetParametersRequest,
    ) -> RusotoFuture<GetParametersResult, GetParametersError>;

    /// <p><p>Retrieve parameters in a specific hierarchy. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-working.html">Working with Systems Manager Parameters</a> in the <i>AWS Systems Manager User Guide</i>. </p> <p>Request results are returned on a best-effort basis. If you specify <code>MaxResults</code> in the request, the response includes information up to the limit specified. The number of items returned, however, can be between zero and the value of <code>MaxResults</code>. If the service reaches an internal limit while processing the results, it stops the operation and returns the matching values up to that point and a <code>NextToken</code>. You can specify the <code>NextToken</code> in a subsequent call to get the next set of results.</p> <note> <p>This API action doesn&#39;t support filtering by tags. </p> </note></p>
    fn get_parameters_by_path(
        &self,
        input: GetParametersByPathRequest,
    ) -> RusotoFuture<GetParametersByPathResult, GetParametersByPathError>;

    /// <p>Retrieves information about a patch baseline.</p>
    fn get_patch_baseline(
        &self,
        input: GetPatchBaselineRequest,
    ) -> RusotoFuture<GetPatchBaselineResult, GetPatchBaselineError>;

    /// <p>Retrieves the patch baseline that should be used for the specified patch group.</p>
    fn get_patch_baseline_for_patch_group(
        &self,
        input: GetPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<GetPatchBaselineForPatchGroupResult, GetPatchBaselineForPatchGroupError>;

    /// <p> <code>ServiceSetting</code> is an account-level setting for an AWS service. This setting defines how a user interacts with or uses a service or a feature of a service. For example, if an AWS service charges money to the account based on feature or service usage, then the AWS service team might create a default setting of "false". This means the user can't use this feature unless they change the setting to "true" and intentionally opt in for a paid feature.</p> <p>Services map a <code>SettingId</code> object to a setting value. AWS services teams define the default value for a <code>SettingId</code>. You can't create a new <code>SettingId</code>, but you can overwrite the default value if you have the <code>ssm:UpdateServiceSetting</code> permission for the setting. Use the <a>UpdateServiceSetting</a> API action to change the default setting. Or use the <a>ResetServiceSetting</a> to change the value back to the original value defined by the AWS service team.</p> <p>Query the current service setting for the account. </p>
    fn get_service_setting(
        &self,
        input: GetServiceSettingRequest,
    ) -> RusotoFuture<GetServiceSettingResult, GetServiceSettingError>;

    /// <p><p>A parameter label is a user-defined alias to help you manage different versions of a parameter. When you modify a parameter, Systems Manager automatically saves a new version and increments the version number by one. A label can help you remember the purpose of a parameter when there are multiple versions. </p> <p>Parameter labels have the following requirements and restrictions.</p> <ul> <li> <p>A version of a parameter can have a maximum of 10 labels.</p> </li> <li> <p>You can&#39;t attach the same label to different versions of the same parameter. For example, if version 1 has the label Production, then you can&#39;t attach Production to version 2.</p> </li> <li> <p>You can move a label from one version of a parameter to another.</p> </li> <li> <p>You can&#39;t create a label when you create a new parameter. You must attach a label to a specific version of a parameter.</p> </li> <li> <p>You can&#39;t delete a parameter label. If you no longer want to use a parameter label, then you must move it to a different version of a parameter.</p> </li> <li> <p>A label can have a maximum of 100 characters.</p> </li> <li> <p>Labels can contain letters (case sensitive), numbers, periods (.), hyphens (-), or underscores (_).</p> </li> <li> <p>Labels can&#39;t begin with a number, &quot;aws,&quot; or &quot;ssm&quot; (not case sensitive). If a label fails to meet these requirements, then the label is not associated with a parameter and the system displays it in the list of InvalidLabels.</p> </li> </ul></p>
    fn label_parameter_version(
        &self,
        input: LabelParameterVersionRequest,
    ) -> RusotoFuture<LabelParameterVersionResult, LabelParameterVersionError>;

    /// <p>Retrieves all versions of an association for a specific association ID.</p>
    fn list_association_versions(
        &self,
        input: ListAssociationVersionsRequest,
    ) -> RusotoFuture<ListAssociationVersionsResult, ListAssociationVersionsError>;

    /// <p>Lists the associations for the specified Systems Manager document or instance.</p>
    fn list_associations(
        &self,
        input: ListAssociationsRequest,
    ) -> RusotoFuture<ListAssociationsResult, ListAssociationsError>;

    /// <p>An invocation is copy of a command sent to a specific instance. A command can apply to one or more instances. A command invocation applies to one instance. For example, if a user runs SendCommand against three instances, then a command invocation is created for each requested instance ID. ListCommandInvocations provide status about command execution.</p>
    fn list_command_invocations(
        &self,
        input: ListCommandInvocationsRequest,
    ) -> RusotoFuture<ListCommandInvocationsResult, ListCommandInvocationsError>;

    /// <p>Lists the commands requested by users of the AWS account.</p>
    fn list_commands(
        &self,
        input: ListCommandsRequest,
    ) -> RusotoFuture<ListCommandsResult, ListCommandsError>;

    /// <p>For a specified resource ID, this API action returns a list of compliance statuses for different resource types. Currently, you can only specify one resource ID per call. List results depend on the criteria specified in the filter. </p>
    fn list_compliance_items(
        &self,
        input: ListComplianceItemsRequest,
    ) -> RusotoFuture<ListComplianceItemsResult, ListComplianceItemsError>;

    /// <p>Returns a summary count of compliant and non-compliant resources for a compliance type. For example, this call can return State Manager associations, patches, or custom compliance types according to the filter criteria that you specify. </p>
    fn list_compliance_summaries(
        &self,
        input: ListComplianceSummariesRequest,
    ) -> RusotoFuture<ListComplianceSummariesResult, ListComplianceSummariesError>;

    /// <p>List all versions for a document.</p>
    fn list_document_versions(
        &self,
        input: ListDocumentVersionsRequest,
    ) -> RusotoFuture<ListDocumentVersionsResult, ListDocumentVersionsError>;

    /// <p>Describes one or more of your Systems Manager documents.</p>
    fn list_documents(
        &self,
        input: ListDocumentsRequest,
    ) -> RusotoFuture<ListDocumentsResult, ListDocumentsError>;

    /// <p>A list of inventory items returned by the request.</p>
    fn list_inventory_entries(
        &self,
        input: ListInventoryEntriesRequest,
    ) -> RusotoFuture<ListInventoryEntriesResult, ListInventoryEntriesError>;

    /// <p>Returns a resource-level summary count. The summary includes information about compliant and non-compliant statuses and detailed compliance-item severity counts, according to the filter criteria you specify.</p>
    fn list_resource_compliance_summaries(
        &self,
        input: ListResourceComplianceSummariesRequest,
    ) -> RusotoFuture<ListResourceComplianceSummariesResult, ListResourceComplianceSummariesError>;

    /// <p>Lists your resource data sync configurations. Includes information about the last time a sync attempted to start, the last sync status, and the last time a sync successfully completed.</p> <p>The number of sync configurations might be too large to return using a single call to <code>ListResourceDataSync</code>. You can limit the number of sync configurations returned by using the <code>MaxResults</code> parameter. To determine whether there are more sync configurations to list, check the value of <code>NextToken</code> in the output. If there are more sync configurations to list, you can request them by specifying the <code>NextToken</code> returned in the call to the parameter of a subsequent call. </p>
    fn list_resource_data_sync(
        &self,
        input: ListResourceDataSyncRequest,
    ) -> RusotoFuture<ListResourceDataSyncResult, ListResourceDataSyncError>;

    /// <p>Returns a list of the tags assigned to the specified resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResult, ListTagsForResourceError>;

    /// <p>Shares a Systems Manager document publicly or privately. If you share a document privately, you must specify the AWS user account IDs for those people who can use the document. If you share a document publicly, you must specify <i>All</i> as the account ID.</p>
    fn modify_document_permission(
        &self,
        input: ModifyDocumentPermissionRequest,
    ) -> RusotoFuture<ModifyDocumentPermissionResponse, ModifyDocumentPermissionError>;

    /// <p><p>Registers a compliance type and other compliance details on a designated resource. This action lets you register custom compliance details with a resource. This call overwrites existing compliance information on the resource, so you must provide a full list of compliance items each time that you send the request.</p> <p>ComplianceType can be one of the following:</p> <ul> <li> <p>ExecutionId: The execution ID when the patch, association, or custom compliance item was applied.</p> </li> <li> <p>ExecutionType: Specify patch, association, or Custom:<code>string</code>.</p> </li> <li> <p>ExecutionTime. The time the patch, association, or custom compliance item was applied to the instance.</p> </li> <li> <p>Id: The patch, association, or custom compliance ID.</p> </li> <li> <p>Title: A title.</p> </li> <li> <p>Status: The status of the compliance item. For example, <code>approved</code> for patches, or <code>Failed</code> for associations.</p> </li> <li> <p>Severity: A patch severity. For example, <code>critical</code>.</p> </li> <li> <p>DocumentName: A SSM document name. For example, AWS-RunPatchBaseline.</p> </li> <li> <p>DocumentVersion: An SSM document version number. For example, 4.</p> </li> <li> <p>Classification: A patch classification. For example, <code>security updates</code>.</p> </li> <li> <p>PatchBaselineId: A patch baseline ID.</p> </li> <li> <p>PatchSeverity: A patch severity. For example, <code>Critical</code>.</p> </li> <li> <p>PatchState: A patch state. For example, <code>InstancesWithFailedPatches</code>.</p> </li> <li> <p>PatchGroup: The name of a patch group.</p> </li> <li> <p>InstalledTime: The time the association, patch, or custom compliance item was applied to the resource. Specify the time by using the following format: yyyy-MM-dd&#39;T&#39;HH:mm:ss&#39;Z&#39;</p> </li> </ul></p>
    fn put_compliance_items(
        &self,
        input: PutComplianceItemsRequest,
    ) -> RusotoFuture<PutComplianceItemsResult, PutComplianceItemsError>;

    /// <p>Bulk update custom inventory items on one more instance. The request adds an inventory item, if it doesn't already exist, or updates an inventory item, if it does exist.</p>
    fn put_inventory(
        &self,
        input: PutInventoryRequest,
    ) -> RusotoFuture<PutInventoryResult, PutInventoryError>;

    /// <p>Add a parameter to the system.</p>
    fn put_parameter(
        &self,
        input: PutParameterRequest,
    ) -> RusotoFuture<PutParameterResult, PutParameterError>;

    /// <p>Defines the default patch baseline for the relevant operating system.</p> <p>To reset the AWS predefined patch baseline as the default, specify the full patch baseline ARN as the baseline ID value. For example, for CentOS, specify <code>arn:aws:ssm:us-east-2:733109147000:patchbaseline/pb-0574b43a65ea646ed</code> instead of <code>pb-0574b43a65ea646ed</code>.</p>
    fn register_default_patch_baseline(
        &self,
        input: RegisterDefaultPatchBaselineRequest,
    ) -> RusotoFuture<RegisterDefaultPatchBaselineResult, RegisterDefaultPatchBaselineError>;

    /// <p>Registers a patch baseline for a patch group.</p>
    fn register_patch_baseline_for_patch_group(
        &self,
        input: RegisterPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<
        RegisterPatchBaselineForPatchGroupResult,
        RegisterPatchBaselineForPatchGroupError,
    >;

    /// <p>Registers a target with a maintenance window.</p>
    fn register_target_with_maintenance_window(
        &self,
        input: RegisterTargetWithMaintenanceWindowRequest,
    ) -> RusotoFuture<
        RegisterTargetWithMaintenanceWindowResult,
        RegisterTargetWithMaintenanceWindowError,
    >;

    /// <p>Adds a new task to a maintenance window.</p>
    fn register_task_with_maintenance_window(
        &self,
        input: RegisterTaskWithMaintenanceWindowRequest,
    ) -> RusotoFuture<RegisterTaskWithMaintenanceWindowResult, RegisterTaskWithMaintenanceWindowError>;

    /// <p>Removes tag keys from the specified resource.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceRequest,
    ) -> RusotoFuture<RemoveTagsFromResourceResult, RemoveTagsFromResourceError>;

    /// <p> <code>ServiceSetting</code> is an account-level setting for an AWS service. This setting defines how a user interacts with or uses a service or a feature of a service. For example, if an AWS service charges money to the account based on feature or service usage, then the AWS service team might create a default setting of "false". This means the user can't use this feature unless they change the setting to "true" and intentionally opt in for a paid feature.</p> <p>Services map a <code>SettingId</code> object to a setting value. AWS services teams define the default value for a <code>SettingId</code>. You can't create a new <code>SettingId</code>, but you can overwrite the default value if you have the <code>ssm:UpdateServiceSetting</code> permission for the setting. Use the <a>GetServiceSetting</a> API action to view the current value. Use the <a>UpdateServiceSetting</a> API action to change the default setting. </p> <p>Reset the service setting for the account to the default value as provisioned by the AWS service team. </p>
    fn reset_service_setting(
        &self,
        input: ResetServiceSettingRequest,
    ) -> RusotoFuture<ResetServiceSettingResult, ResetServiceSettingError>;

    /// <p><p>Reconnects a session to an instance after it has been disconnected. Connections can be resumed for disconnected sessions, but not terminated sessions.</p> <note> <p>This command is primarily for use by client machines to automatically reconnect during intermittent network issues. It is not intended for any other use.</p> </note></p>
    fn resume_session(
        &self,
        input: ResumeSessionRequest,
    ) -> RusotoFuture<ResumeSessionResponse, ResumeSessionError>;

    /// <p>Sends a signal to an Automation execution to change the current behavior or status of the execution. </p>
    fn send_automation_signal(
        &self,
        input: SendAutomationSignalRequest,
    ) -> RusotoFuture<SendAutomationSignalResult, SendAutomationSignalError>;

    /// <p>Runs commands on one or more managed instances.</p>
    fn send_command(
        &self,
        input: SendCommandRequest,
    ) -> RusotoFuture<SendCommandResult, SendCommandError>;

    /// <p>Use this API action to run an association immediately and only one time. This action can be helpful when troubleshooting associations.</p>
    fn start_associations_once(
        &self,
        input: StartAssociationsOnceRequest,
    ) -> RusotoFuture<StartAssociationsOnceResult, StartAssociationsOnceError>;

    /// <p>Initiates execution of an Automation document.</p>
    fn start_automation_execution(
        &self,
        input: StartAutomationExecutionRequest,
    ) -> RusotoFuture<StartAutomationExecutionResult, StartAutomationExecutionError>;

    /// <p><p>Initiates a connection to a target (for example, an instance) for a Session Manager session. Returns a URL and token that can be used to open a WebSocket connection for sending input and receiving outputs.</p> <note> <p>AWS CLI usage: <code>start-session</code> is an interactive command that requires the Session Manager plugin to be installed on the client machine making the call. For information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager-working-with-install-plugin.html"> Install the Session Manager Plugin for the AWS CLI</a> in the <i>AWS Systems Manager User Guide</i>.</p> </note></p>
    fn start_session(
        &self,
        input: StartSessionRequest,
    ) -> RusotoFuture<StartSessionResponse, StartSessionError>;

    /// <p>Stop an Automation that is currently running.</p>
    fn stop_automation_execution(
        &self,
        input: StopAutomationExecutionRequest,
    ) -> RusotoFuture<StopAutomationExecutionResult, StopAutomationExecutionError>;

    /// <p>Permanently ends a session and closes the data connection between the Session Manager client and SSM Agent on the instance. A terminated session cannot be resumed.</p>
    fn terminate_session(
        &self,
        input: TerminateSessionRequest,
    ) -> RusotoFuture<TerminateSessionResponse, TerminateSessionError>;

    /// <p><p>Updates an association. You can update the association name and version, the document version, schedule, parameters, and Amazon S3 output.</p> <important> <p>When you update an association, the association immediately runs against the specified targets.</p> </important></p>
    fn update_association(
        &self,
        input: UpdateAssociationRequest,
    ) -> RusotoFuture<UpdateAssociationResult, UpdateAssociationError>;

    /// <p>Updates the status of the Systems Manager document associated with the specified instance.</p>
    fn update_association_status(
        &self,
        input: UpdateAssociationStatusRequest,
    ) -> RusotoFuture<UpdateAssociationStatusResult, UpdateAssociationStatusError>;

    /// <p>Updates one or more values for an SSM document.</p>
    fn update_document(
        &self,
        input: UpdateDocumentRequest,
    ) -> RusotoFuture<UpdateDocumentResult, UpdateDocumentError>;

    /// <p>Set the default version of a document. </p>
    fn update_document_default_version(
        &self,
        input: UpdateDocumentDefaultVersionRequest,
    ) -> RusotoFuture<UpdateDocumentDefaultVersionResult, UpdateDocumentDefaultVersionError>;

    /// <p>Updates an existing maintenance window. Only specified parameters are modified.</p>
    fn update_maintenance_window(
        &self,
        input: UpdateMaintenanceWindowRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowResult, UpdateMaintenanceWindowError>;

    /// <p><p>Modifies the target of an existing maintenance window. You can change the following:</p> <ul> <li> <p>Name</p> </li> <li> <p>Description</p> </li> <li> <p>Owner</p> </li> <li> <p>IDs for an ID target</p> </li> <li> <p>Tags for a Tag target</p> </li> <li> <p>From any supported tag type to another. The three supported tag types are ID target, Tag target, and resource group. For more information, see <a>Target</a>.</p> </li> </ul> <note> <p>If a parameter is null, then the corresponding field is not modified.</p> </note></p>
    fn update_maintenance_window_target(
        &self,
        input: UpdateMaintenanceWindowTargetRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowTargetResult, UpdateMaintenanceWindowTargetError>;

    /// <p>Modifies a task assigned to a maintenance window. You can't change the task type, but you can change the following values:</p> <ul> <li> <p>TaskARN. For example, you can change a RUN_COMMAND task from AWS-RunPowerShellScript to AWS-RunShellScript.</p> </li> <li> <p>ServiceRoleArn</p> </li> <li> <p>TaskInvocationParameters</p> </li> <li> <p>Priority</p> </li> <li> <p>MaxConcurrency</p> </li> <li> <p>MaxErrors</p> </li> </ul> <p>If a parameter is null, then the corresponding field is not modified. Also, if you set Replace to true, then all fields required by the <a>RegisterTaskWithMaintenanceWindow</a> action are required for this request. Optional fields that aren't specified are set to null.</p>
    fn update_maintenance_window_task(
        &self,
        input: UpdateMaintenanceWindowTaskRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowTaskResult, UpdateMaintenanceWindowTaskError>;

    /// <p>Assigns or changes an Amazon Identity and Access Management (IAM) role for the managed instance.</p>
    fn update_managed_instance_role(
        &self,
        input: UpdateManagedInstanceRoleRequest,
    ) -> RusotoFuture<UpdateManagedInstanceRoleResult, UpdateManagedInstanceRoleError>;

    /// <p>Edit or change an OpsItem. You must have permission in AWS Identity and Access Management (IAM) to update an OpsItem. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-getting-started.html">Getting Started with OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>.</p> <p>Operations engineers and IT professionals use OpsCenter to view, investigate, and remediate operational issues impacting the performance and health of their AWS resources. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter.html">AWS Systems Manager OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>. </p>
    fn update_ops_item(
        &self,
        input: UpdateOpsItemRequest,
    ) -> RusotoFuture<UpdateOpsItemResponse, UpdateOpsItemError>;

    /// <p><p>Modifies an existing patch baseline. Fields not specified in the request are left unchanged.</p> <note> <p>For information about valid key and value pairs in <code>PatchFilters</code> for each supported operating system type, see <a href="http://docs.aws.amazon.com/systems-manager/latest/APIReference/API_PatchFilter.html">PatchFilter</a>.</p> </note></p>
    fn update_patch_baseline(
        &self,
        input: UpdatePatchBaselineRequest,
    ) -> RusotoFuture<UpdatePatchBaselineResult, UpdatePatchBaselineError>;

    /// <p> <code>ServiceSetting</code> is an account-level setting for an AWS service. This setting defines how a user interacts with or uses a service or a feature of a service. For example, if an AWS service charges money to the account based on feature or service usage, then the AWS service team might create a default setting of "false". This means the user can't use this feature unless they change the setting to "true" and intentionally opt in for a paid feature.</p> <p>Services map a <code>SettingId</code> object to a setting value. AWS services teams define the default value for a <code>SettingId</code>. You can't create a new <code>SettingId</code>, but you can overwrite the default value if you have the <code>ssm:UpdateServiceSetting</code> permission for the setting. Use the <a>GetServiceSetting</a> API action to view the current value. Or, use the <a>ResetServiceSetting</a> to change the value back to the original value defined by the AWS service team.</p> <p>Update the service setting for the account. </p>
    fn update_service_setting(
        &self,
        input: UpdateServiceSettingRequest,
    ) -> RusotoFuture<UpdateServiceSettingResult, UpdateServiceSettingError>;
}
/// A client for the Amazon SSM API.
#[derive(Clone)]
pub struct SsmClient {
    client: Client,
    region: region::Region,
}

impl SsmClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SsmClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SsmClient
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

    pub fn new_with_client(client: Client, region: region::Region) -> SsmClient {
        SsmClient { client, region }
    }
}

impl fmt::Debug for SsmClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SsmClient")
            .field("region", &self.region)
            .finish()
    }
}

impl Ssm for SsmClient {
    /// <p>Adds or overwrites one or more tags for the specified resource. Tags are metadata that you can assign to your documents, managed instances, maintenance windows, Parameter Store parameters, and patch baselines. Tags enable you to categorize your resources in different ways, for example, by purpose, owner, or environment. Each tag consists of a key and an optional value, both of which you define. For example, you could define a set of tags for your account's managed instances that helps you track each instance's owner and stack level. For example: Key=Owner and Value=DbAdmin, SysAdmin, or Dev. Or Key=Stack and Value=Production, Pre-Production, or Test.</p> <p>Each resource can have a maximum of 50 tags. </p> <p>We recommend that you devise a set of tag keys that meets your needs for each resource type. Using a consistent set of tag keys makes it easier for you to manage your resources. You can search and filter the resources based on the tags you add. Tags don't have any semantic meaning to Amazon EC2 and are interpreted strictly as a string of characters. </p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html">Tagging Your Amazon EC2 Resources</a> in the <i>Amazon EC2 User Guide</i>.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceRequest,
    ) -> RusotoFuture<AddTagsToResourceResult, AddTagsToResourceError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.AddTagsToResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AddTagsToResourceResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AddTagsToResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Attempts to cancel the command specified by the Command ID. There is no guarantee that the command will be terminated and the underlying process stopped.</p>
    fn cancel_command(
        &self,
        input: CancelCommandRequest,
    ) -> RusotoFuture<CancelCommandResult, CancelCommandError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CancelCommand");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CancelCommandResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CancelCommandError::from_response(response))),
                )
            }
        })
    }

    /// <p>Stops a maintenance window execution that is already in progress and cancels any tasks in the window that have not already starting running. (Tasks already in progress will continue to completion.)</p>
    fn cancel_maintenance_window_execution(
        &self,
        input: CancelMaintenanceWindowExecutionRequest,
    ) -> RusotoFuture<CancelMaintenanceWindowExecutionResult, CancelMaintenanceWindowExecutionError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CancelMaintenanceWindowExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CancelMaintenanceWindowExecutionResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CancelMaintenanceWindowExecutionError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Registers your on-premises server or virtual machine with Amazon EC2 so that you can manage these resources using Run Command. An on-premises server or virtual machine that has been registered with EC2 is called a managed instance. For more information about activations, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-managedinstances.html">Setting Up AWS Systems Manager for Hybrid Environments</a>.</p>
    fn create_activation(
        &self,
        input: CreateActivationRequest,
    ) -> RusotoFuture<CreateActivationResult, CreateActivationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateActivation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateActivationResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateActivationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Associates the specified Systems Manager document with the specified instances or targets.</p> <p>When you associate a document with one or more instances using instance IDs or tags, SSM Agent running on the instance processes the document and configures the instance as specified.</p> <p>If you associate a document with an instance that already has an associated document, the system returns the AssociationAlreadyExists exception.</p>
    fn create_association(
        &self,
        input: CreateAssociationRequest,
    ) -> RusotoFuture<CreateAssociationResult, CreateAssociationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateAssociation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAssociationResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAssociationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Associates the specified Systems Manager document with the specified instances or targets.</p> <p>When you associate a document with one or more instances using instance IDs or tags, SSM Agent running on the instance processes the document and configures the instance as specified.</p> <p>If you associate a document with an instance that already has an associated document, the system returns the AssociationAlreadyExists exception.</p>
    fn create_association_batch(
        &self,
        input: CreateAssociationBatchRequest,
    ) -> RusotoFuture<CreateAssociationBatchResult, CreateAssociationBatchError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateAssociationBatch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAssociationBatchResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateAssociationBatchError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a Systems Manager document.</p> <p>After you create a document, you can use CreateAssociation to associate it with one or more running instances.</p>
    fn create_document(
        &self,
        input: CreateDocumentRequest,
    ) -> RusotoFuture<CreateDocumentResult, CreateDocumentError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDocumentResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDocumentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new maintenance window.</p>
    fn create_maintenance_window(
        &self,
        input: CreateMaintenanceWindowRequest,
    ) -> RusotoFuture<CreateMaintenanceWindowResult, CreateMaintenanceWindowError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateMaintenanceWindow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateMaintenanceWindowResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateMaintenanceWindowError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a new OpsItem. You must have permission in AWS Identity and Access Management (IAM) to create a new OpsItem. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-getting-started.html">Getting Started with OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>.</p> <p>Operations engineers and IT professionals use OpsCenter to view, investigate, and remediate operational issues impacting the performance and health of their AWS resources. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter.html">AWS Systems Manager OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>. </p>
    fn create_ops_item(
        &self,
        input: CreateOpsItemRequest,
    ) -> RusotoFuture<CreateOpsItemResponse, CreateOpsItemError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateOpsItem");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateOpsItemResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateOpsItemError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Creates a patch baseline.</p> <note> <p>For information about valid key and value pairs in <code>PatchFilters</code> for each supported operating system type, see <a href="http://docs.aws.amazon.com/systems-manager/latest/APIReference/API_PatchFilter.html">PatchFilter</a>.</p> </note></p>
    fn create_patch_baseline(
        &self,
        input: CreatePatchBaselineRequest,
    ) -> RusotoFuture<CreatePatchBaselineResult, CreatePatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreatePatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreatePatchBaselineResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreatePatchBaselineError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a resource data sync configuration to a single bucket in Amazon S3. This is an asynchronous operation that returns immediately. After a successful initial sync is completed, the system continuously syncs data to the Amazon S3 bucket. To check the status of the sync, use the <a>ListResourceDataSync</a>.</p> <p>By default, data is not encrypted in Amazon S3. We strongly recommend that you enable encryption in Amazon S3 to ensure secure data storage. We also recommend that you secure access to the Amazon S3 bucket by creating a restrictive bucket policy. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-inventory-datasync.html">Configuring Resource Data Sync for Inventory</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    fn create_resource_data_sync(
        &self,
        input: CreateResourceDataSyncRequest,
    ) -> RusotoFuture<CreateResourceDataSyncResult, CreateResourceDataSyncError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateResourceDataSync");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateResourceDataSyncResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateResourceDataSyncError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes an activation. You are not required to delete an activation. If you delete an activation, you can no longer use it to register additional managed instances. Deleting an activation does not de-register managed instances. You must manually de-register managed instances.</p>
    fn delete_activation(
        &self,
        input: DeleteActivationRequest,
    ) -> RusotoFuture<DeleteActivationResult, DeleteActivationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteActivation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteActivationResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteActivationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disassociates the specified Systems Manager document from the specified instance.</p> <p>When you disassociate a document from an instance, it does not change the configuration of the instance. To change the configuration state of an instance after you disassociate a document, you must create a new document with the desired configuration and associate it with the instance.</p>
    fn delete_association(
        &self,
        input: DeleteAssociationRequest,
    ) -> RusotoFuture<DeleteAssociationResult, DeleteAssociationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteAssociation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteAssociationResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAssociationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the Systems Manager document and all instance associations to the document.</p> <p>Before you delete the document, we recommend that you use <a>DeleteAssociation</a> to disassociate all instances that are associated with the document.</p>
    fn delete_document(
        &self,
        input: DeleteDocumentRequest,
    ) -> RusotoFuture<DeleteDocumentResult, DeleteDocumentError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDocumentResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDocumentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete a custom inventory type, or the data associated with a custom Inventory type. Deleting a custom inventory type is also referred to as deleting a custom inventory schema.</p>
    fn delete_inventory(
        &self,
        input: DeleteInventoryRequest,
    ) -> RusotoFuture<DeleteInventoryResult, DeleteInventoryError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteInventory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteInventoryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteInventoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a maintenance window.</p>
    fn delete_maintenance_window(
        &self,
        input: DeleteMaintenanceWindowRequest,
    ) -> RusotoFuture<DeleteMaintenanceWindowResult, DeleteMaintenanceWindowError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteMaintenanceWindow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteMaintenanceWindowResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteMaintenanceWindowError::from_response(response))
                }))
            }
        })
    }

    /// <p>Delete a parameter from the system.</p>
    fn delete_parameter(
        &self,
        input: DeleteParameterRequest,
    ) -> RusotoFuture<DeleteParameterResult, DeleteParameterError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteParameter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteParameterResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteParameterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete a list of parameters.</p>
    fn delete_parameters(
        &self,
        input: DeleteParametersRequest,
    ) -> RusotoFuture<DeleteParametersResult, DeleteParametersError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteParameters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteParametersResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteParametersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a patch baseline.</p>
    fn delete_patch_baseline(
        &self,
        input: DeletePatchBaselineRequest,
    ) -> RusotoFuture<DeletePatchBaselineResult, DeletePatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeletePatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeletePatchBaselineResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeletePatchBaselineError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a Resource Data Sync configuration. After the configuration is deleted, changes to inventory data on managed instances are no longer synced with the target Amazon S3 bucket. Deleting a sync configuration does not delete data in the target Amazon S3 bucket.</p>
    fn delete_resource_data_sync(
        &self,
        input: DeleteResourceDataSyncRequest,
    ) -> RusotoFuture<DeleteResourceDataSyncResult, DeleteResourceDataSyncError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteResourceDataSync");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteResourceDataSyncResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteResourceDataSyncError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Removes the server or virtual machine from the list of registered servers. You can reregister the instance again at any time. If you don't plan to use Run Command on the server, we suggest uninstalling SSM Agent first.</p>
    fn deregister_managed_instance(
        &self,
        input: DeregisterManagedInstanceRequest,
    ) -> RusotoFuture<DeregisterManagedInstanceResult, DeregisterManagedInstanceError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeregisterManagedInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeregisterManagedInstanceResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterManagedInstanceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Removes a patch group from a patch baseline.</p>
    fn deregister_patch_baseline_for_patch_group(
        &self,
        input: DeregisterPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<
        DeregisterPatchBaselineForPatchGroupResult,
        DeregisterPatchBaselineForPatchGroupError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DeregisterPatchBaselineForPatchGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeregisterPatchBaselineForPatchGroupResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterPatchBaselineForPatchGroupError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Removes a target from a maintenance window.</p>
    fn deregister_target_from_maintenance_window(
        &self,
        input: DeregisterTargetFromMaintenanceWindowRequest,
    ) -> RusotoFuture<
        DeregisterTargetFromMaintenanceWindowResult,
        DeregisterTargetFromMaintenanceWindowError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DeregisterTargetFromMaintenanceWindow",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeregisterTargetFromMaintenanceWindowResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterTargetFromMaintenanceWindowError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Removes a task from a maintenance window.</p>
    fn deregister_task_from_maintenance_window(
        &self,
        input: DeregisterTaskFromMaintenanceWindowRequest,
    ) -> RusotoFuture<
        DeregisterTaskFromMaintenanceWindowResult,
        DeregisterTaskFromMaintenanceWindowError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DeregisterTaskFromMaintenanceWindow",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeregisterTaskFromMaintenanceWindowResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterTaskFromMaintenanceWindowError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Describes details about the activation, such as the date and time the activation was created, its expiration date, the IAM role assigned to the instances in the activation, and the number of instances registered by using this activation.</p>
    fn describe_activations(
        &self,
        input: DescribeActivationsRequest,
    ) -> RusotoFuture<DescribeActivationsResult, DescribeActivationsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeActivations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeActivationsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeActivationsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes the association for the specified target or instance. If you created the association by using the <code>Targets</code> parameter, then you must retrieve the association by using the association ID. If you created the association by specifying an instance ID and a Systems Manager document, then you retrieve the association by specifying the document name and the instance ID. </p>
    fn describe_association(
        &self,
        input: DescribeAssociationRequest,
    ) -> RusotoFuture<DescribeAssociationResult, DescribeAssociationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeAssociation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAssociationResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeAssociationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Use this API action to view information about a specific execution of a specific association.</p>
    fn describe_association_execution_targets(
        &self,
        input: DescribeAssociationExecutionTargetsRequest,
    ) -> RusotoFuture<
        DescribeAssociationExecutionTargetsResult,
        DescribeAssociationExecutionTargetsError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeAssociationExecutionTargets",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAssociationExecutionTargetsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAssociationExecutionTargetsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Use this API action to view all executions for a specific association ID. </p>
    fn describe_association_executions(
        &self,
        input: DescribeAssociationExecutionsRequest,
    ) -> RusotoFuture<DescribeAssociationExecutionsResult, DescribeAssociationExecutionsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeAssociationExecutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAssociationExecutionsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAssociationExecutionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Provides details about all active and terminated Automation executions.</p>
    fn describe_automation_executions(
        &self,
        input: DescribeAutomationExecutionsRequest,
    ) -> RusotoFuture<DescribeAutomationExecutionsResult, DescribeAutomationExecutionsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeAutomationExecutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAutomationExecutionsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAutomationExecutionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Information about all active and terminated step executions in an Automation workflow.</p>
    fn describe_automation_step_executions(
        &self,
        input: DescribeAutomationStepExecutionsRequest,
    ) -> RusotoFuture<DescribeAutomationStepExecutionsResult, DescribeAutomationStepExecutionsError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeAutomationStepExecutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAutomationStepExecutionsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAutomationStepExecutionsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists all patches eligible to be included in a patch baseline.</p>
    fn describe_available_patches(
        &self,
        input: DescribeAvailablePatchesRequest,
    ) -> RusotoFuture<DescribeAvailablePatchesResult, DescribeAvailablePatchesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeAvailablePatches");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAvailablePatchesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAvailablePatchesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the specified Systems Manager document.</p>
    fn describe_document(
        &self,
        input: DescribeDocumentRequest,
    ) -> RusotoFuture<DescribeDocumentResult, DescribeDocumentError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDocumentResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeDocumentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the permissions for a Systems Manager document. If you created the document, you are the owner. If a document is shared, it can either be shared privately (by specifying a user's AWS account ID) or publicly (<i>All</i>). </p>
    fn describe_document_permission(
        &self,
        input: DescribeDocumentPermissionRequest,
    ) -> RusotoFuture<DescribeDocumentPermissionResponse, DescribeDocumentPermissionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeDocumentPermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDocumentPermissionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDocumentPermissionError::from_response(response))
                }))
            }
        })
    }

    /// <p>All associations for the instance(s).</p>
    fn describe_effective_instance_associations(
        &self,
        input: DescribeEffectiveInstanceAssociationsRequest,
    ) -> RusotoFuture<
        DescribeEffectiveInstanceAssociationsResult,
        DescribeEffectiveInstanceAssociationsError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeEffectiveInstanceAssociations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEffectiveInstanceAssociationsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEffectiveInstanceAssociationsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the current effective patches (the patch and the approval state) for the specified patch baseline. Note that this API applies only to Windows patch baselines.</p>
    fn describe_effective_patches_for_patch_baseline(
        &self,
        input: DescribeEffectivePatchesForPatchBaselineRequest,
    ) -> RusotoFuture<
        DescribeEffectivePatchesForPatchBaselineResult,
        DescribeEffectivePatchesForPatchBaselineError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeEffectivePatchesForPatchBaseline",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEffectivePatchesForPatchBaselineResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEffectivePatchesForPatchBaselineError::from_response(response))
                }))
            }
        })
    }

    /// <p>The status of the associations for the instance(s).</p>
    fn describe_instance_associations_status(
        &self,
        input: DescribeInstanceAssociationsStatusRequest,
    ) -> RusotoFuture<
        DescribeInstanceAssociationsStatusResult,
        DescribeInstanceAssociationsStatusError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeInstanceAssociationsStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeInstanceAssociationsStatusResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstanceAssociationsStatusError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Describes one or more of your instances. You can use this to get information about instances like the operating system platform, the SSM Agent version (Linux), status etc. If you specify one or more instance IDs, it returns information for those instances. If you do not specify instance IDs, it returns information for all your instances. If you specify an instance ID that is not valid or an instance that you do not own, you receive an error. </p> <note> <p>The IamRole field for this API action is the Amazon Identity and Access Management (IAM) role assigned to on-premises instances. This call does not return the IAM role for Amazon EC2 instances.</p> </note></p>
    fn describe_instance_information(
        &self,
        input: DescribeInstanceInformationRequest,
    ) -> RusotoFuture<DescribeInstanceInformationResult, DescribeInstanceInformationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeInstanceInformation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeInstanceInformationResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstanceInformationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the high-level patch state of one or more instances.</p>
    fn describe_instance_patch_states(
        &self,
        input: DescribeInstancePatchStatesRequest,
    ) -> RusotoFuture<DescribeInstancePatchStatesResult, DescribeInstancePatchStatesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeInstancePatchStates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeInstancePatchStatesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstancePatchStatesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the high-level patch state for the instances in the specified patch group.</p>
    fn describe_instance_patch_states_for_patch_group(
        &self,
        input: DescribeInstancePatchStatesForPatchGroupRequest,
    ) -> RusotoFuture<
        DescribeInstancePatchStatesForPatchGroupResult,
        DescribeInstancePatchStatesForPatchGroupError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeInstancePatchStatesForPatchGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeInstancePatchStatesForPatchGroupResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstancePatchStatesForPatchGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves information about the patches on the specified instance and their state relative to the patch baseline being used for the instance.</p>
    fn describe_instance_patches(
        &self,
        input: DescribeInstancePatchesRequest,
    ) -> RusotoFuture<DescribeInstancePatchesResult, DescribeInstancePatchesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeInstancePatches");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeInstancePatchesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstancePatchesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes a specific delete inventory operation.</p>
    fn describe_inventory_deletions(
        &self,
        input: DescribeInventoryDeletionsRequest,
    ) -> RusotoFuture<DescribeInventoryDeletionsResult, DescribeInventoryDeletionsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeInventoryDeletions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeInventoryDeletionsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInventoryDeletionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the individual task executions (one per target) for a particular task run as part of a maintenance window execution.</p>
    fn describe_maintenance_window_execution_task_invocations(
        &self,
        input: DescribeMaintenanceWindowExecutionTaskInvocationsRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionTaskInvocationsResult,
        DescribeMaintenanceWindowExecutionTaskInvocationsError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeMaintenanceWindowExecutionTaskInvocations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
                        if response.status.is_success() {
                            Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<DescribeMaintenanceWindowExecutionTaskInvocationsResult, _>()
                }))
                        } else {
                            Box::new(response.buffer().from_err().and_then(|response| {
                                Err(DescribeMaintenanceWindowExecutionTaskInvocationsError::from_response(response))
                            }))
                        }
                    })
    }

    /// <p>For a given maintenance window execution, lists the tasks that were run.</p>
    fn describe_maintenance_window_execution_tasks(
        &self,
        input: DescribeMaintenanceWindowExecutionTasksRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionTasksResult,
        DescribeMaintenanceWindowExecutionTasksError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeMaintenanceWindowExecutionTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeMaintenanceWindowExecutionTasksResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowExecutionTasksError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists the executions of a maintenance window. This includes information about when the maintenance window was scheduled to be active, and information about tasks registered and run with the maintenance window.</p>
    fn describe_maintenance_window_executions(
        &self,
        input: DescribeMaintenanceWindowExecutionsRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionsResult,
        DescribeMaintenanceWindowExecutionsError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeMaintenanceWindowExecutions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeMaintenanceWindowExecutionsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowExecutionsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about upcoming executions of a maintenance window.</p>
    fn describe_maintenance_window_schedule(
        &self,
        input: DescribeMaintenanceWindowScheduleRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowScheduleResult, DescribeMaintenanceWindowScheduleError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeMaintenanceWindowSchedule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeMaintenanceWindowScheduleResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowScheduleError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists the targets registered with the maintenance window.</p>
    fn describe_maintenance_window_targets(
        &self,
        input: DescribeMaintenanceWindowTargetsRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowTargetsResult, DescribeMaintenanceWindowTargetsError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindowTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeMaintenanceWindowTargetsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowTargetsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists the tasks in a maintenance window.</p>
    fn describe_maintenance_window_tasks(
        &self,
        input: DescribeMaintenanceWindowTasksRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowTasksResult, DescribeMaintenanceWindowTasksError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindowTasks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeMaintenanceWindowTasksResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowTasksError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the maintenance windows in an AWS account.</p>
    fn describe_maintenance_windows(
        &self,
        input: DescribeMaintenanceWindowsRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowsResult, DescribeMaintenanceWindowsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindows");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeMaintenanceWindowsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves information about the maintenance window targets or tasks that an instance is associated with.</p>
    fn describe_maintenance_windows_for_target(
        &self,
        input: DescribeMaintenanceWindowsForTargetRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowsForTargetResult,
        DescribeMaintenanceWindowsForTargetError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeMaintenanceWindowsForTarget",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeMaintenanceWindowsForTargetResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowsForTargetError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Query a set of OpsItems. You must have permission in AWS Identity and Access Management (IAM) to query a list of OpsItems. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-getting-started.html">Getting Started with OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>.</p> <p>Operations engineers and IT professionals use OpsCenter to view, investigate, and remediate operational issues impacting the performance and health of their AWS resources. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter.html">AWS Systems Manager OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>. </p>
    fn describe_ops_items(
        &self,
        input: DescribeOpsItemsRequest,
    ) -> RusotoFuture<DescribeOpsItemsResponse, DescribeOpsItemsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeOpsItems");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeOpsItemsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeOpsItemsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get information about a parameter.</p> <p>Request results are returned on a best-effort basis. If you specify <code>MaxResults</code> in the request, the response includes information up to the limit specified. The number of items returned, however, can be between zero and the value of <code>MaxResults</code>. If the service reaches an internal limit while processing the results, it stops the operation and returns the matching values up to that point and a <code>NextToken</code>. You can specify the <code>NextToken</code> in a subsequent call to get the next set of results.</p>
    fn describe_parameters(
        &self,
        input: DescribeParametersRequest,
    ) -> RusotoFuture<DescribeParametersResult, DescribeParametersError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeParameters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeParametersResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeParametersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the patch baselines in your AWS account.</p>
    fn describe_patch_baselines(
        &self,
        input: DescribePatchBaselinesRequest,
    ) -> RusotoFuture<DescribePatchBaselinesResult, DescribePatchBaselinesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribePatchBaselines");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePatchBaselinesResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribePatchBaselinesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns high-level aggregated patch compliance state for a patch group.</p>
    fn describe_patch_group_state(
        &self,
        input: DescribePatchGroupStateRequest,
    ) -> RusotoFuture<DescribePatchGroupStateResult, DescribePatchGroupStateError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribePatchGroupState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePatchGroupStateResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribePatchGroupStateError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists all patch groups that have been registered with patch baselines.</p>
    fn describe_patch_groups(
        &self,
        input: DescribePatchGroupsRequest,
    ) -> RusotoFuture<DescribePatchGroupsResult, DescribePatchGroupsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribePatchGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePatchGroupsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribePatchGroupsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Lists the properties of available patches organized by product, product family, classification, severity, and other properties of available patches. You can use the reported properties in the filters you specify in requests for actions such as <a>CreatePatchBaseline</a>, <a>UpdatePatchBaseline</a>, <a>DescribeAvailablePatches</a>, and <a>DescribePatchBaselines</a>.</p> <p>The following section lists the properties that can be used in filters for each major operating system type:</p> <dl> <dt>WINDOWS</dt> <dd> <p>Valid properties: PRODUCT, PRODUCT<em>FAMILY, CLASSIFICATION, MSRC</em>SEVERITY</p> </dd> <dt>AMAZON<em>LINUX</dt> <dd> <p>Valid properties: PRODUCT, CLASSIFICATION, SEVERITY</p> </dd> <dt>AMAZON</em>LINUX<em>2</dt> <dd> <p>Valid properties: PRODUCT, CLASSIFICATION, SEVERITY</p> </dd> <dt>UBUNTU </dt> <dd> <p>Valid properties: PRODUCT, PRIORITY</p> </dd> <dt>REDHAT</em>ENTERPRISE_LINUX</dt> <dd> <p>Valid properties: PRODUCT, CLASSIFICATION, SEVERITY</p> </dd> <dt>SUSE</dt> <dd> <p>Valid properties: PRODUCT, CLASSIFICATION, SEVERITY</p> </dd> <dt>CENTOS</dt> <dd> <p>Valid properties: PRODUCT, CLASSIFICATION, SEVERITY</p> </dd> </dl></p>
    fn describe_patch_properties(
        &self,
        input: DescribePatchPropertiesRequest,
    ) -> RusotoFuture<DescribePatchPropertiesResult, DescribePatchPropertiesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribePatchProperties");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePatchPropertiesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribePatchPropertiesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list of all active sessions (both connected and disconnected) or terminated sessions from the past 30 days.</p>
    fn describe_sessions(
        &self,
        input: DescribeSessionsRequest,
    ) -> RusotoFuture<DescribeSessionsResponse, DescribeSessionsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeSessions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeSessionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeSessionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get detailed information about a particular Automation execution.</p>
    fn get_automation_execution(
        &self,
        input: GetAutomationExecutionRequest,
    ) -> RusotoFuture<GetAutomationExecutionResult, GetAutomationExecutionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetAutomationExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAutomationExecutionResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetAutomationExecutionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns detailed information about command execution for an invocation or plugin. </p>
    fn get_command_invocation(
        &self,
        input: GetCommandInvocationRequest,
    ) -> RusotoFuture<GetCommandInvocationResult, GetCommandInvocationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetCommandInvocation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCommandInvocationResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetCommandInvocationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves the Session Manager connection status for an instance to determine whether it is connected and ready to receive Session Manager connections.</p>
    fn get_connection_status(
        &self,
        input: GetConnectionStatusRequest,
    ) -> RusotoFuture<GetConnectionStatusResponse, GetConnectionStatusError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetConnectionStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConnectionStatusResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetConnectionStatusError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves the default patch baseline. Note that Systems Manager supports creating multiple default patch baselines. For example, you can create a default patch baseline for each operating system.</p> <p>If you do not specify an operating system value, the default patch baseline for Windows is returned.</p>
    fn get_default_patch_baseline(
        &self,
        input: GetDefaultPatchBaselineRequest,
    ) -> RusotoFuture<GetDefaultPatchBaselineResult, GetDefaultPatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetDefaultPatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDefaultPatchBaselineResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDefaultPatchBaselineError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the current snapshot for the patch baseline the instance uses. This API is primarily used by the AWS-RunPatchBaseline Systems Manager document. </p>
    fn get_deployable_patch_snapshot_for_instance(
        &self,
        input: GetDeployablePatchSnapshotForInstanceRequest,
    ) -> RusotoFuture<
        GetDeployablePatchSnapshotForInstanceResult,
        GetDeployablePatchSnapshotForInstanceError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.GetDeployablePatchSnapshotForInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDeployablePatchSnapshotForInstanceResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDeployablePatchSnapshotForInstanceError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Gets the contents of the specified Systems Manager document.</p>
    fn get_document(
        &self,
        input: GetDocumentRequest,
    ) -> RusotoFuture<GetDocumentResult, GetDocumentError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDocumentResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDocumentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Query inventory information.</p>
    fn get_inventory(
        &self,
        input: GetInventoryRequest,
    ) -> RusotoFuture<GetInventoryResult, GetInventoryError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetInventory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInventoryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetInventoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Return a list of inventory type names for the account, or return a list of attribute names for a specific Inventory item type. </p>
    fn get_inventory_schema(
        &self,
        input: GetInventorySchemaRequest,
    ) -> RusotoFuture<GetInventorySchemaResult, GetInventorySchemaError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetInventorySchema");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInventorySchemaResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetInventorySchemaError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a maintenance window.</p>
    fn get_maintenance_window(
        &self,
        input: GetMaintenanceWindowRequest,
    ) -> RusotoFuture<GetMaintenanceWindowResult, GetMaintenanceWindowError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetMaintenanceWindow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMaintenanceWindowResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetMaintenanceWindowError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves details about a specific a maintenance window execution.</p>
    fn get_maintenance_window_execution(
        &self,
        input: GetMaintenanceWindowExecutionRequest,
    ) -> RusotoFuture<GetMaintenanceWindowExecutionResult, GetMaintenanceWindowExecutionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetMaintenanceWindowExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMaintenanceWindowExecutionResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetMaintenanceWindowExecutionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the details about a specific task run as part of a maintenance window execution.</p>
    fn get_maintenance_window_execution_task(
        &self,
        input: GetMaintenanceWindowExecutionTaskRequest,
    ) -> RusotoFuture<GetMaintenanceWindowExecutionTaskResult, GetMaintenanceWindowExecutionTaskError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.GetMaintenanceWindowExecutionTask",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMaintenanceWindowExecutionTaskResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetMaintenanceWindowExecutionTaskError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves information about a specific task running on a specific target.</p>
    fn get_maintenance_window_execution_task_invocation(
        &self,
        input: GetMaintenanceWindowExecutionTaskInvocationRequest,
    ) -> RusotoFuture<
        GetMaintenanceWindowExecutionTaskInvocationResult,
        GetMaintenanceWindowExecutionTaskInvocationError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.GetMaintenanceWindowExecutionTaskInvocation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMaintenanceWindowExecutionTaskInvocationResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetMaintenanceWindowExecutionTaskInvocationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the tasks in a maintenance window.</p>
    fn get_maintenance_window_task(
        &self,
        input: GetMaintenanceWindowTaskRequest,
    ) -> RusotoFuture<GetMaintenanceWindowTaskResult, GetMaintenanceWindowTaskError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetMaintenanceWindowTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMaintenanceWindowTaskResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetMaintenanceWindowTaskError::from_response(response))
                }))
            }
        })
    }

    /// <p>Get information about an OpsItem by using the ID. You must have permission in AWS Identity and Access Management (IAM) to view information about an OpsItem. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-getting-started.html">Getting Started with OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>.</p> <p>Operations engineers and IT professionals use OpsCenter to view, investigate, and remediate operational issues impacting the performance and health of their AWS resources. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter.html">AWS Systems Manager OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>. </p>
    fn get_ops_item(
        &self,
        input: GetOpsItemRequest,
    ) -> RusotoFuture<GetOpsItemResponse, GetOpsItemError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetOpsItem");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetOpsItemResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetOpsItemError::from_response(response))),
                )
            }
        })
    }

    /// <p>View a summary of OpsItems based on specified filters and aggregators.</p>
    fn get_ops_summary(
        &self,
        input: GetOpsSummaryRequest,
    ) -> RusotoFuture<GetOpsSummaryResult, GetOpsSummaryError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetOpsSummary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetOpsSummaryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetOpsSummaryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get information about a parameter by using the parameter name. Don't confuse this API action with the <a>GetParameters</a> API action.</p>
    fn get_parameter(
        &self,
        input: GetParameterRequest,
    ) -> RusotoFuture<GetParameterResult, GetParameterError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetParameter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetParameterResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetParameterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Query a list of all parameters used by the AWS account.</p>
    fn get_parameter_history(
        &self,
        input: GetParameterHistoryRequest,
    ) -> RusotoFuture<GetParameterHistoryResult, GetParameterHistoryError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetParameterHistory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetParameterHistoryResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetParameterHistoryError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Get details of a parameter. Don't confuse this API action with the <a>GetParameter</a> API action.</p>
    fn get_parameters(
        &self,
        input: GetParametersRequest,
    ) -> RusotoFuture<GetParametersResult, GetParametersError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetParameters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetParametersResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetParametersError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Retrieve parameters in a specific hierarchy. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-working.html">Working with Systems Manager Parameters</a> in the <i>AWS Systems Manager User Guide</i>. </p> <p>Request results are returned on a best-effort basis. If you specify <code>MaxResults</code> in the request, the response includes information up to the limit specified. The number of items returned, however, can be between zero and the value of <code>MaxResults</code>. If the service reaches an internal limit while processing the results, it stops the operation and returns the matching values up to that point and a <code>NextToken</code>. You can specify the <code>NextToken</code> in a subsequent call to get the next set of results.</p> <note> <p>This API action doesn&#39;t support filtering by tags. </p> </note></p>
    fn get_parameters_by_path(
        &self,
        input: GetParametersByPathRequest,
    ) -> RusotoFuture<GetParametersByPathResult, GetParametersByPathError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetParametersByPath");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetParametersByPathResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetParametersByPathError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves information about a patch baseline.</p>
    fn get_patch_baseline(
        &self,
        input: GetPatchBaselineRequest,
    ) -> RusotoFuture<GetPatchBaselineResult, GetPatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetPatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPatchBaselineResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPatchBaselineError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the patch baseline that should be used for the specified patch group.</p>
    fn get_patch_baseline_for_patch_group(
        &self,
        input: GetPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<GetPatchBaselineForPatchGroupResult, GetPatchBaselineForPatchGroupError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetPatchBaselineForPatchGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPatchBaselineForPatchGroupResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetPatchBaselineForPatchGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p> <code>ServiceSetting</code> is an account-level setting for an AWS service. This setting defines how a user interacts with or uses a service or a feature of a service. For example, if an AWS service charges money to the account based on feature or service usage, then the AWS service team might create a default setting of "false". This means the user can't use this feature unless they change the setting to "true" and intentionally opt in for a paid feature.</p> <p>Services map a <code>SettingId</code> object to a setting value. AWS services teams define the default value for a <code>SettingId</code>. You can't create a new <code>SettingId</code>, but you can overwrite the default value if you have the <code>ssm:UpdateServiceSetting</code> permission for the setting. Use the <a>UpdateServiceSetting</a> API action to change the default setting. Or use the <a>ResetServiceSetting</a> to change the value back to the original value defined by the AWS service team.</p> <p>Query the current service setting for the account. </p>
    fn get_service_setting(
        &self,
        input: GetServiceSettingRequest,
    ) -> RusotoFuture<GetServiceSettingResult, GetServiceSettingError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetServiceSetting");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetServiceSettingResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetServiceSettingError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>A parameter label is a user-defined alias to help you manage different versions of a parameter. When you modify a parameter, Systems Manager automatically saves a new version and increments the version number by one. A label can help you remember the purpose of a parameter when there are multiple versions. </p> <p>Parameter labels have the following requirements and restrictions.</p> <ul> <li> <p>A version of a parameter can have a maximum of 10 labels.</p> </li> <li> <p>You can&#39;t attach the same label to different versions of the same parameter. For example, if version 1 has the label Production, then you can&#39;t attach Production to version 2.</p> </li> <li> <p>You can move a label from one version of a parameter to another.</p> </li> <li> <p>You can&#39;t create a label when you create a new parameter. You must attach a label to a specific version of a parameter.</p> </li> <li> <p>You can&#39;t delete a parameter label. If you no longer want to use a parameter label, then you must move it to a different version of a parameter.</p> </li> <li> <p>A label can have a maximum of 100 characters.</p> </li> <li> <p>Labels can contain letters (case sensitive), numbers, periods (.), hyphens (-), or underscores (_).</p> </li> <li> <p>Labels can&#39;t begin with a number, &quot;aws,&quot; or &quot;ssm&quot; (not case sensitive). If a label fails to meet these requirements, then the label is not associated with a parameter and the system displays it in the list of InvalidLabels.</p> </li> </ul></p>
    fn label_parameter_version(
        &self,
        input: LabelParameterVersionRequest,
    ) -> RusotoFuture<LabelParameterVersionResult, LabelParameterVersionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.LabelParameterVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<LabelParameterVersionResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(LabelParameterVersionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves all versions of an association for a specific association ID.</p>
    fn list_association_versions(
        &self,
        input: ListAssociationVersionsRequest,
    ) -> RusotoFuture<ListAssociationVersionsResult, ListAssociationVersionsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListAssociationVersions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAssociationVersionsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListAssociationVersionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the associations for the specified Systems Manager document or instance.</p>
    fn list_associations(
        &self,
        input: ListAssociationsRequest,
    ) -> RusotoFuture<ListAssociationsResult, ListAssociationsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListAssociations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAssociationsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAssociationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>An invocation is copy of a command sent to a specific instance. A command can apply to one or more instances. A command invocation applies to one instance. For example, if a user runs SendCommand against three instances, then a command invocation is created for each requested instance ID. ListCommandInvocations provide status about command execution.</p>
    fn list_command_invocations(
        &self,
        input: ListCommandInvocationsRequest,
    ) -> RusotoFuture<ListCommandInvocationsResult, ListCommandInvocationsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListCommandInvocations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListCommandInvocationsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListCommandInvocationsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the commands requested by users of the AWS account.</p>
    fn list_commands(
        &self,
        input: ListCommandsRequest,
    ) -> RusotoFuture<ListCommandsResult, ListCommandsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListCommands");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListCommandsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListCommandsError::from_response(response))),
                )
            }
        })
    }

    /// <p>For a specified resource ID, this API action returns a list of compliance statuses for different resource types. Currently, you can only specify one resource ID per call. List results depend on the criteria specified in the filter. </p>
    fn list_compliance_items(
        &self,
        input: ListComplianceItemsRequest,
    ) -> RusotoFuture<ListComplianceItemsResult, ListComplianceItemsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListComplianceItems");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListComplianceItemsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListComplianceItemsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a summary count of compliant and non-compliant resources for a compliance type. For example, this call can return State Manager associations, patches, or custom compliance types according to the filter criteria that you specify. </p>
    fn list_compliance_summaries(
        &self,
        input: ListComplianceSummariesRequest,
    ) -> RusotoFuture<ListComplianceSummariesResult, ListComplianceSummariesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListComplianceSummaries");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListComplianceSummariesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListComplianceSummariesError::from_response(response))
                }))
            }
        })
    }

    /// <p>List all versions for a document.</p>
    fn list_document_versions(
        &self,
        input: ListDocumentVersionsRequest,
    ) -> RusotoFuture<ListDocumentVersionsResult, ListDocumentVersionsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListDocumentVersions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDocumentVersionsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListDocumentVersionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes one or more of your Systems Manager documents.</p>
    fn list_documents(
        &self,
        input: ListDocumentsRequest,
    ) -> RusotoFuture<ListDocumentsResult, ListDocumentsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListDocuments");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDocumentsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDocumentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>A list of inventory items returned by the request.</p>
    fn list_inventory_entries(
        &self,
        input: ListInventoryEntriesRequest,
    ) -> RusotoFuture<ListInventoryEntriesResult, ListInventoryEntriesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListInventoryEntries");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListInventoryEntriesResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListInventoryEntriesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a resource-level summary count. The summary includes information about compliant and non-compliant statuses and detailed compliance-item severity counts, according to the filter criteria you specify.</p>
    fn list_resource_compliance_summaries(
        &self,
        input: ListResourceComplianceSummariesRequest,
    ) -> RusotoFuture<ListResourceComplianceSummariesResult, ListResourceComplianceSummariesError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListResourceComplianceSummaries");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListResourceComplianceSummariesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListResourceComplianceSummariesError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists your resource data sync configurations. Includes information about the last time a sync attempted to start, the last sync status, and the last time a sync successfully completed.</p> <p>The number of sync configurations might be too large to return using a single call to <code>ListResourceDataSync</code>. You can limit the number of sync configurations returned by using the <code>MaxResults</code> parameter. To determine whether there are more sync configurations to list, check the value of <code>NextToken</code> in the output. If there are more sync configurations to list, you can request them by specifying the <code>NextToken</code> returned in the call to the parameter of a subsequent call. </p>
    fn list_resource_data_sync(
        &self,
        input: ListResourceDataSyncRequest,
    ) -> RusotoFuture<ListResourceDataSyncResult, ListResourceDataSyncError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListResourceDataSync");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListResourceDataSyncResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListResourceDataSyncError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a list of the tags assigned to the specified resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResult, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResult, _>()
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

    /// <p>Shares a Systems Manager document publicly or privately. If you share a document privately, you must specify the AWS user account IDs for those people who can use the document. If you share a document publicly, you must specify <i>All</i> as the account ID.</p>
    fn modify_document_permission(
        &self,
        input: ModifyDocumentPermissionRequest,
    ) -> RusotoFuture<ModifyDocumentPermissionResponse, ModifyDocumentPermissionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ModifyDocumentPermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ModifyDocumentPermissionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyDocumentPermissionError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Registers a compliance type and other compliance details on a designated resource. This action lets you register custom compliance details with a resource. This call overwrites existing compliance information on the resource, so you must provide a full list of compliance items each time that you send the request.</p> <p>ComplianceType can be one of the following:</p> <ul> <li> <p>ExecutionId: The execution ID when the patch, association, or custom compliance item was applied.</p> </li> <li> <p>ExecutionType: Specify patch, association, or Custom:<code>string</code>.</p> </li> <li> <p>ExecutionTime. The time the patch, association, or custom compliance item was applied to the instance.</p> </li> <li> <p>Id: The patch, association, or custom compliance ID.</p> </li> <li> <p>Title: A title.</p> </li> <li> <p>Status: The status of the compliance item. For example, <code>approved</code> for patches, or <code>Failed</code> for associations.</p> </li> <li> <p>Severity: A patch severity. For example, <code>critical</code>.</p> </li> <li> <p>DocumentName: A SSM document name. For example, AWS-RunPatchBaseline.</p> </li> <li> <p>DocumentVersion: An SSM document version number. For example, 4.</p> </li> <li> <p>Classification: A patch classification. For example, <code>security updates</code>.</p> </li> <li> <p>PatchBaselineId: A patch baseline ID.</p> </li> <li> <p>PatchSeverity: A patch severity. For example, <code>Critical</code>.</p> </li> <li> <p>PatchState: A patch state. For example, <code>InstancesWithFailedPatches</code>.</p> </li> <li> <p>PatchGroup: The name of a patch group.</p> </li> <li> <p>InstalledTime: The time the association, patch, or custom compliance item was applied to the resource. Specify the time by using the following format: yyyy-MM-dd&#39;T&#39;HH:mm:ss&#39;Z&#39;</p> </li> </ul></p>
    fn put_compliance_items(
        &self,
        input: PutComplianceItemsRequest,
    ) -> RusotoFuture<PutComplianceItemsResult, PutComplianceItemsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.PutComplianceItems");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutComplianceItemsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutComplianceItemsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Bulk update custom inventory items on one more instance. The request adds an inventory item, if it doesn't already exist, or updates an inventory item, if it does exist.</p>
    fn put_inventory(
        &self,
        input: PutInventoryRequest,
    ) -> RusotoFuture<PutInventoryResult, PutInventoryError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.PutInventory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutInventoryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutInventoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Add a parameter to the system.</p>
    fn put_parameter(
        &self,
        input: PutParameterRequest,
    ) -> RusotoFuture<PutParameterResult, PutParameterError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.PutParameter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutParameterResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutParameterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Defines the default patch baseline for the relevant operating system.</p> <p>To reset the AWS predefined patch baseline as the default, specify the full patch baseline ARN as the baseline ID value. For example, for CentOS, specify <code>arn:aws:ssm:us-east-2:733109147000:patchbaseline/pb-0574b43a65ea646ed</code> instead of <code>pb-0574b43a65ea646ed</code>.</p>
    fn register_default_patch_baseline(
        &self,
        input: RegisterDefaultPatchBaselineRequest,
    ) -> RusotoFuture<RegisterDefaultPatchBaselineResult, RegisterDefaultPatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.RegisterDefaultPatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RegisterDefaultPatchBaselineResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RegisterDefaultPatchBaselineError::from_response(response))
                }))
            }
        })
    }

    /// <p>Registers a patch baseline for a patch group.</p>
    fn register_patch_baseline_for_patch_group(
        &self,
        input: RegisterPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<
        RegisterPatchBaselineForPatchGroupResult,
        RegisterPatchBaselineForPatchGroupError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.RegisterPatchBaselineForPatchGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RegisterPatchBaselineForPatchGroupResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RegisterPatchBaselineForPatchGroupError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Registers a target with a maintenance window.</p>
    fn register_target_with_maintenance_window(
        &self,
        input: RegisterTargetWithMaintenanceWindowRequest,
    ) -> RusotoFuture<
        RegisterTargetWithMaintenanceWindowResult,
        RegisterTargetWithMaintenanceWindowError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.RegisterTargetWithMaintenanceWindow",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RegisterTargetWithMaintenanceWindowResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RegisterTargetWithMaintenanceWindowError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Adds a new task to a maintenance window.</p>
    fn register_task_with_maintenance_window(
        &self,
        input: RegisterTaskWithMaintenanceWindowRequest,
    ) -> RusotoFuture<RegisterTaskWithMaintenanceWindowResult, RegisterTaskWithMaintenanceWindowError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.RegisterTaskWithMaintenanceWindow",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RegisterTaskWithMaintenanceWindowResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RegisterTaskWithMaintenanceWindowError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Removes tag keys from the specified resource.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceRequest,
    ) -> RusotoFuture<RemoveTagsFromResourceResult, RemoveTagsFromResourceError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.RemoveTagsFromResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RemoveTagsFromResourceResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RemoveTagsFromResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p> <code>ServiceSetting</code> is an account-level setting for an AWS service. This setting defines how a user interacts with or uses a service or a feature of a service. For example, if an AWS service charges money to the account based on feature or service usage, then the AWS service team might create a default setting of "false". This means the user can't use this feature unless they change the setting to "true" and intentionally opt in for a paid feature.</p> <p>Services map a <code>SettingId</code> object to a setting value. AWS services teams define the default value for a <code>SettingId</code>. You can't create a new <code>SettingId</code>, but you can overwrite the default value if you have the <code>ssm:UpdateServiceSetting</code> permission for the setting. Use the <a>GetServiceSetting</a> API action to view the current value. Use the <a>UpdateServiceSetting</a> API action to change the default setting. </p> <p>Reset the service setting for the account to the default value as provisioned by the AWS service team. </p>
    fn reset_service_setting(
        &self,
        input: ResetServiceSettingRequest,
    ) -> RusotoFuture<ResetServiceSettingResult, ResetServiceSettingError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ResetServiceSetting");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ResetServiceSettingResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ResetServiceSettingError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Reconnects a session to an instance after it has been disconnected. Connections can be resumed for disconnected sessions, but not terminated sessions.</p> <note> <p>This command is primarily for use by client machines to automatically reconnect during intermittent network issues. It is not intended for any other use.</p> </note></p>
    fn resume_session(
        &self,
        input: ResumeSessionRequest,
    ) -> RusotoFuture<ResumeSessionResponse, ResumeSessionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ResumeSession");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ResumeSessionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ResumeSessionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sends a signal to an Automation execution to change the current behavior or status of the execution. </p>
    fn send_automation_signal(
        &self,
        input: SendAutomationSignalRequest,
    ) -> RusotoFuture<SendAutomationSignalResult, SendAutomationSignalError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.SendAutomationSignal");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SendAutomationSignalResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(SendAutomationSignalError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Runs commands on one or more managed instances.</p>
    fn send_command(
        &self,
        input: SendCommandRequest,
    ) -> RusotoFuture<SendCommandResult, SendCommandError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.SendCommand");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SendCommandResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SendCommandError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use this API action to run an association immediately and only one time. This action can be helpful when troubleshooting associations.</p>
    fn start_associations_once(
        &self,
        input: StartAssociationsOnceRequest,
    ) -> RusotoFuture<StartAssociationsOnceResult, StartAssociationsOnceError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.StartAssociationsOnce");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartAssociationsOnceResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartAssociationsOnceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Initiates execution of an Automation document.</p>
    fn start_automation_execution(
        &self,
        input: StartAutomationExecutionRequest,
    ) -> RusotoFuture<StartAutomationExecutionResult, StartAutomationExecutionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.StartAutomationExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartAutomationExecutionResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartAutomationExecutionError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Initiates a connection to a target (for example, an instance) for a Session Manager session. Returns a URL and token that can be used to open a WebSocket connection for sending input and receiving outputs.</p> <note> <p>AWS CLI usage: <code>start-session</code> is an interactive command that requires the Session Manager plugin to be installed on the client machine making the call. For information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager-working-with-install-plugin.html"> Install the Session Manager Plugin for the AWS CLI</a> in the <i>AWS Systems Manager User Guide</i>.</p> </note></p>
    fn start_session(
        &self,
        input: StartSessionRequest,
    ) -> RusotoFuture<StartSessionResponse, StartSessionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.StartSession");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartSessionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartSessionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Stop an Automation that is currently running.</p>
    fn stop_automation_execution(
        &self,
        input: StopAutomationExecutionRequest,
    ) -> RusotoFuture<StopAutomationExecutionResult, StopAutomationExecutionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.StopAutomationExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopAutomationExecutionResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopAutomationExecutionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Permanently ends a session and closes the data connection between the Session Manager client and SSM Agent on the instance. A terminated session cannot be resumed.</p>
    fn terminate_session(
        &self,
        input: TerminateSessionRequest,
    ) -> RusotoFuture<TerminateSessionResponse, TerminateSessionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.TerminateSession");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TerminateSessionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TerminateSessionError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Updates an association. You can update the association name and version, the document version, schedule, parameters, and Amazon S3 output.</p> <important> <p>When you update an association, the association immediately runs against the specified targets.</p> </important></p>
    fn update_association(
        &self,
        input: UpdateAssociationRequest,
    ) -> RusotoFuture<UpdateAssociationResult, UpdateAssociationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateAssociation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateAssociationResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAssociationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the status of the Systems Manager document associated with the specified instance.</p>
    fn update_association_status(
        &self,
        input: UpdateAssociationStatusRequest,
    ) -> RusotoFuture<UpdateAssociationStatusResult, UpdateAssociationStatusError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateAssociationStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateAssociationStatusResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAssociationStatusError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates one or more values for an SSM document.</p>
    fn update_document(
        &self,
        input: UpdateDocumentRequest,
    ) -> RusotoFuture<UpdateDocumentResult, UpdateDocumentError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDocumentResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDocumentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Set the default version of a document. </p>
    fn update_document_default_version(
        &self,
        input: UpdateDocumentDefaultVersionRequest,
    ) -> RusotoFuture<UpdateDocumentDefaultVersionResult, UpdateDocumentDefaultVersionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateDocumentDefaultVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDocumentDefaultVersionResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDocumentDefaultVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates an existing maintenance window. Only specified parameters are modified.</p>
    fn update_maintenance_window(
        &self,
        input: UpdateMaintenanceWindowRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowResult, UpdateMaintenanceWindowError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateMaintenanceWindow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateMaintenanceWindowResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateMaintenanceWindowError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Modifies the target of an existing maintenance window. You can change the following:</p> <ul> <li> <p>Name</p> </li> <li> <p>Description</p> </li> <li> <p>Owner</p> </li> <li> <p>IDs for an ID target</p> </li> <li> <p>Tags for a Tag target</p> </li> <li> <p>From any supported tag type to another. The three supported tag types are ID target, Tag target, and resource group. For more information, see <a>Target</a>.</p> </li> </ul> <note> <p>If a parameter is null, then the corresponding field is not modified.</p> </note></p>
    fn update_maintenance_window_target(
        &self,
        input: UpdateMaintenanceWindowTargetRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowTargetResult, UpdateMaintenanceWindowTargetError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateMaintenanceWindowTarget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateMaintenanceWindowTargetResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateMaintenanceWindowTargetError::from_response(response))
                }))
            }
        })
    }

    /// <p>Modifies a task assigned to a maintenance window. You can't change the task type, but you can change the following values:</p> <ul> <li> <p>TaskARN. For example, you can change a RUN_COMMAND task from AWS-RunPowerShellScript to AWS-RunShellScript.</p> </li> <li> <p>ServiceRoleArn</p> </li> <li> <p>TaskInvocationParameters</p> </li> <li> <p>Priority</p> </li> <li> <p>MaxConcurrency</p> </li> <li> <p>MaxErrors</p> </li> </ul> <p>If a parameter is null, then the corresponding field is not modified. Also, if you set Replace to true, then all fields required by the <a>RegisterTaskWithMaintenanceWindow</a> action are required for this request. Optional fields that aren't specified are set to null.</p>
    fn update_maintenance_window_task(
        &self,
        input: UpdateMaintenanceWindowTaskRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowTaskResult, UpdateMaintenanceWindowTaskError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateMaintenanceWindowTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateMaintenanceWindowTaskResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateMaintenanceWindowTaskError::from_response(response))
                }))
            }
        })
    }

    /// <p>Assigns or changes an Amazon Identity and Access Management (IAM) role for the managed instance.</p>
    fn update_managed_instance_role(
        &self,
        input: UpdateManagedInstanceRoleRequest,
    ) -> RusotoFuture<UpdateManagedInstanceRoleResult, UpdateManagedInstanceRoleError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateManagedInstanceRole");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateManagedInstanceRoleResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateManagedInstanceRoleError::from_response(response))
                }))
            }
        })
    }

    /// <p>Edit or change an OpsItem. You must have permission in AWS Identity and Access Management (IAM) to update an OpsItem. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-getting-started.html">Getting Started with OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>.</p> <p>Operations engineers and IT professionals use OpsCenter to view, investigate, and remediate operational issues impacting the performance and health of their AWS resources. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter.html">AWS Systems Manager OpsCenter</a> in the <i>AWS Systems Manager User Guide</i>. </p>
    fn update_ops_item(
        &self,
        input: UpdateOpsItemRequest,
    ) -> RusotoFuture<UpdateOpsItemResponse, UpdateOpsItemError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateOpsItem");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateOpsItemResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateOpsItemError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Modifies an existing patch baseline. Fields not specified in the request are left unchanged.</p> <note> <p>For information about valid key and value pairs in <code>PatchFilters</code> for each supported operating system type, see <a href="http://docs.aws.amazon.com/systems-manager/latest/APIReference/API_PatchFilter.html">PatchFilter</a>.</p> </note></p>
    fn update_patch_baseline(
        &self,
        input: UpdatePatchBaselineRequest,
    ) -> RusotoFuture<UpdatePatchBaselineResult, UpdatePatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdatePatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdatePatchBaselineResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdatePatchBaselineError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p> <code>ServiceSetting</code> is an account-level setting for an AWS service. This setting defines how a user interacts with or uses a service or a feature of a service. For example, if an AWS service charges money to the account based on feature or service usage, then the AWS service team might create a default setting of "false". This means the user can't use this feature unless they change the setting to "true" and intentionally opt in for a paid feature.</p> <p>Services map a <code>SettingId</code> object to a setting value. AWS services teams define the default value for a <code>SettingId</code>. You can't create a new <code>SettingId</code>, but you can overwrite the default value if you have the <code>ssm:UpdateServiceSetting</code> permission for the setting. Use the <a>GetServiceSetting</a> API action to view the current value. Or, use the <a>ResetServiceSetting</a> to change the value back to the original value defined by the AWS service team.</p> <p>Update the service setting for the account. </p>
    fn update_service_setting(
        &self,
        input: UpdateServiceSettingRequest,
    ) -> RusotoFuture<UpdateServiceSettingResult, UpdateServiceSettingError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateServiceSetting");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateServiceSettingResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateServiceSettingError::from_response(response))
                    }),
                )
            }
        })
    }
}
