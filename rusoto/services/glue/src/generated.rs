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
/// <p>Defines an action to be initiated by a trigger.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    /// <p>The job arguments used when this trigger fires. For this job run, they replace the default arguments set in the job definition itself.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own Job arguments, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "Arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the crawler to be used with this action.</p>
    #[serde(rename = "CrawlerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name: Option<String>,
    /// <p>The name of a job to be executed.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Specifies configuration properties of a job run notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used with this action.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The <code>JobRun</code> timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours). This overrides the timeout value set in the parent job.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchCreatePartitionRequest {
    /// <p>The ID of the catalog in which the partition is to be created. Currently, this should be the AWS account ID.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the metadata database in which the partition is to be created.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A list of <code>PartitionInput</code> structures that define the partitions to be created.</p>
    #[serde(rename = "PartitionInputList")]
    pub partition_input_list: Vec<PartitionInput>,
    /// <p>The name of the metadata table in which the partition is to be created.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchCreatePartitionResponse {
    /// <p>The errors encountered when trying to create the requested partitions.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<PartitionError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDeleteConnectionRequest {
    /// <p>The ID of the Data Catalog in which the connections reside. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A list of names of the connections to delete.</p>
    #[serde(rename = "ConnectionNameList")]
    pub connection_name_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteConnectionResponse {
    /// <p>A map of the names of connections that were not successfully deleted to error details.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<::std::collections::HashMap<String, ErrorDetail>>,
    /// <p>A list of names of the connection definitions that were successfully deleted.</p>
    #[serde(rename = "Succeeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDeletePartitionRequest {
    /// <p>The ID of the Data Catalog where the partition to be deleted resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which the table in question resides.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A list of <code>PartitionInput</code> structures that define the partitions to be deleted.</p>
    #[serde(rename = "PartitionsToDelete")]
    pub partitions_to_delete: Vec<PartitionValueList>,
    /// <p>The name of the table that contains the partitions to be deleted.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeletePartitionResponse {
    /// <p>The errors encountered when trying to delete the requested partitions.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<PartitionError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDeleteTableRequest {
    /// <p>The ID of the Data Catalog where the table resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which the tables to delete reside. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A list of the table to delete.</p>
    #[serde(rename = "TablesToDelete")]
    pub tables_to_delete: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteTableResponse {
    /// <p>A list of errors encountered in attempting to delete the specified tables.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<TableError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDeleteTableVersionRequest {
    /// <p>The ID of the Data Catalog where the tables reside. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// <p>A list of the IDs of versions to be deleted. A <code>VersionId</code> is a string representation of an integer. Each version is incremented by 1.</p>
    #[serde(rename = "VersionIds")]
    pub version_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteTableVersionResponse {
    /// <p>A list of errors encountered while trying to delete the specified table versions.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<TableVersionError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetCrawlersRequest {
    /// <p>A list of crawler names, which might be the names returned from the <code>ListCrawlers</code> operation.</p>
    #[serde(rename = "CrawlerNames")]
    pub crawler_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetCrawlersResponse {
    /// <p>A list of crawler definitions.</p>
    #[serde(rename = "Crawlers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawlers: Option<Vec<Crawler>>,
    /// <p>A list of names of crawlers that were not found.</p>
    #[serde(rename = "CrawlersNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawlers_not_found: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetDevEndpointsRequest {
    /// <p>The list of <code>DevEndpoint</code> names, which might be the names returned from the <code>ListDevEndpoint</code> operation.</p>
    #[serde(rename = "DevEndpointNames")]
    pub dev_endpoint_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetDevEndpointsResponse {
    /// <p>A list of <code>DevEndpoint</code> definitions.</p>
    #[serde(rename = "DevEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoints: Option<Vec<DevEndpoint>>,
    /// <p>A list of <code>DevEndpoints</code> not found.</p>
    #[serde(rename = "DevEndpointsNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoints_not_found: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetJobsRequest {
    /// <p>A list of job names, which might be the names returned from the <code>ListJobs</code> operation.</p>
    #[serde(rename = "JobNames")]
    pub job_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetJobsResponse {
    /// <p>A list of job definitions.</p>
    #[serde(rename = "Jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    /// <p>A list of names of jobs not found.</p>
    #[serde(rename = "JobsNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs_not_found: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetPartitionRequest {
    /// <p>The ID of the Data Catalog where the partitions in question reside. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the partitions reside.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A list of partition values identifying the partitions to retrieve.</p>
    #[serde(rename = "PartitionsToGet")]
    pub partitions_to_get: Vec<PartitionValueList>,
    /// <p>The name of the partitions' table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetPartitionResponse {
    /// <p>A list of the requested partitions.</p>
    #[serde(rename = "Partitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<Partition>>,
    /// <p>A list of the partition values in the request for which partitions were not returned.</p>
    #[serde(rename = "UnprocessedKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_keys: Option<Vec<PartitionValueList>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetTriggersRequest {
    /// <p>A list of trigger names, which may be the names returned from the <code>ListTriggers</code> operation.</p>
    #[serde(rename = "TriggerNames")]
    pub trigger_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetTriggersResponse {
    /// <p>A list of trigger definitions.</p>
    #[serde(rename = "Triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<Trigger>>,
    /// <p>A list of names of triggers not found.</p>
    #[serde(rename = "TriggersNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers_not_found: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetWorkflowsRequest {
    /// <p>Specifies whether to include a graph when returning the workflow resource metadata.</p>
    #[serde(rename = "IncludeGraph")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_graph: Option<bool>,
    /// <p>A list of workflow names, which may be the names returned from the <code>ListWorkflows</code> operation.</p>
    #[serde(rename = "Names")]
    pub names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetWorkflowsResponse {
    /// <p>A list of names of workflows not found.</p>
    #[serde(rename = "MissingWorkflows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_workflows: Option<Vec<String>>,
    /// <p>A list of workflow resource metadata.</p>
    #[serde(rename = "Workflows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<Workflow>>,
}

/// <p>Records an error that occurred when attempting to stop a specified job run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchStopJobRunError {
    /// <p>Specifies details about the error that was encountered.</p>
    #[serde(rename = "ErrorDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    /// <p>The name of the job definition that is used in the job run in question.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The <code>JobRunId</code> of the job run in question.</p>
    #[serde(rename = "JobRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchStopJobRunRequest {
    /// <p>The name of the job definition for which to stop job runs.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>A list of the <code>JobRunIds</code> that should be stopped for that job definition.</p>
    #[serde(rename = "JobRunIds")]
    pub job_run_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchStopJobRunResponse {
    /// <p>A list of the errors that were encountered in trying to stop <code>JobRuns</code>, including the <code>JobRunId</code> for which each error was encountered and details about the error.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchStopJobRunError>>,
    /// <p>A list of the JobRuns that were successfully submitted for stopping.</p>
    #[serde(rename = "SuccessfulSubmissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_submissions: Option<Vec<BatchStopJobRunSuccessfulSubmission>>,
}

/// <p>Records a successful request to stop a specified <code>JobRun</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchStopJobRunSuccessfulSubmission {
    /// <p>The name of the job definition used in the job run that was stopped.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The <code>JobRunId</code> of the job run that was stopped.</p>
    #[serde(rename = "JobRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelMLTaskRunRequest {
    /// <p>A unique identifier for the task run.</p>
    #[serde(rename = "TaskRunId")]
    pub task_run_id: String,
    /// <p>The unique identifier of the machine learning transform.</p>
    #[serde(rename = "TransformId")]
    pub transform_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelMLTaskRunResponse {
    /// <p>The status for this run.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The unique identifier for the task run.</p>
    #[serde(rename = "TaskRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
    /// <p>The unique identifier of the machine learning transform.</p>
    #[serde(rename = "TransformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

/// <p>Specifies a table definition in the AWS Glue Data Catalog.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CatalogEntry {
    /// <p>The database in which the table metadata resides.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table in question.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>A structure containing migration status information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CatalogImportStatus {
    /// <p> <code>True</code> if the migration has completed, or <code>False</code> otherwise.</p>
    #[serde(rename = "ImportCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_completed: Option<bool>,
    /// <p>The time that the migration was started.</p>
    #[serde(rename = "ImportTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_time: Option<f64>,
    /// <p>The name of the person who initiated the migration.</p>
    #[serde(rename = "ImportedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_by: Option<String>,
}

/// <p>Specifies an AWS Glue Data Catalog target.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogTarget {
    /// <p>The name of the database to be synchronized.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A list of the tables to be synchronized.</p>
    #[serde(rename = "Tables")]
    pub tables: Vec<String>,
}

/// <p>Classifiers are triggered during a crawl task. A classifier checks whether a given file is in a format it can handle. If it is, the classifier creates a schema in the form of a <code>StructType</code> object that matches that data format.</p> <p>You can use the standard classifiers that AWS Glue provides, or you can write your own classifiers to best categorize your data sources and specify the appropriate schemas to use for them. A classifier can be a <code>grok</code> classifier, an <code>XML</code> classifier, a <code>JSON</code> classifier, or a custom <code>CSV</code> classifier, as specified in one of the fields in the <code>Classifier</code> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Classifier {
    /// <p>A classifier for comma-separated values (CSV).</p>
    #[serde(rename = "CsvClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_classifier: Option<CsvClassifier>,
    /// <p>A classifier that uses <code>grok</code>.</p>
    #[serde(rename = "GrokClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<GrokClassifier>,
    /// <p>A classifier for JSON content.</p>
    #[serde(rename = "JsonClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_classifier: Option<JsonClassifier>,
    /// <p>A classifier for XML content.</p>
    #[serde(rename = "XMLClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_classifier: Option<XMLClassifier>,
}

/// <p>Specifies how Amazon CloudWatch data should be encrypted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudWatchEncryption {
    /// <p>The encryption mode to use for CloudWatch data.</p>
    #[serde(rename = "CloudWatchEncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption_mode: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

/// <p>Represents a directional edge in a directed acyclic graph (DAG).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGenEdge {
    /// <p>The ID of the node at which the edge starts.</p>
    #[serde(rename = "Source")]
    pub source: String,
    /// <p>The ID of the node at which the edge ends.</p>
    #[serde(rename = "Target")]
    pub target: String,
    /// <p>The target of the edge.</p>
    #[serde(rename = "TargetParameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter: Option<String>,
}

/// <p>Represents a node in a directed acyclic graph (DAG)</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGenNode {
    /// <p>Properties of the node, in the form of name-value pairs.</p>
    #[serde(rename = "Args")]
    pub args: Vec<CodeGenNodeArg>,
    /// <p>A node identifier that is unique within the node's graph.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The line number of the node.</p>
    #[serde(rename = "LineNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i64>,
    /// <p>The type of node that this is.</p>
    #[serde(rename = "NodeType")]
    pub node_type: String,
}

/// <p>An argument or property of a node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGenNodeArg {
    /// <p>The name of the argument or property.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>True if the value is used as a parameter.</p>
    #[serde(rename = "Param")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<bool>,
    /// <p>The value of the argument or property.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>A column in a <code>Table</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    /// <p>A free-form text comment.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The name of the <code>Column</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>These key-value pairs define properties associated with the column.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The data type of the <code>Column</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Defines a condition under which a trigger fires.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    /// <p>The state of the crawler to which this condition applies.</p>
    #[serde(rename = "CrawlState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_state: Option<String>,
    /// <p>The name of the crawler to which this condition applies.</p>
    #[serde(rename = "CrawlerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name: Option<String>,
    /// <p>The name of the job whose <code>JobRuns</code> this condition applies to, and on which this trigger waits.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>A logical operator.</p>
    #[serde(rename = "LogicalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_operator: Option<String>,
    /// <p>The condition state. Currently, the values supported are <code>SUCCEEDED</code>, <code>STOPPED</code>, <code>TIMEOUT</code>, and <code>FAILED</code>.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>The confusion matrix shows you what your transform is predicting accurately and what types of errors it is making.</p> <p>For more information, see <a href="https://en.wikipedia.org/wiki/Confusion_matrix">Confusion matrix</a> in Wikipedia.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfusionMatrix {
    /// <p>The number of matches in the data that the transform didn't find, in the confusion matrix for your transform.</p>
    #[serde(rename = "NumFalseNegatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_false_negatives: Option<i64>,
    /// <p>The number of nonmatches in the data that the transform incorrectly classified as a match, in the confusion matrix for your transform.</p>
    #[serde(rename = "NumFalsePositives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_false_positives: Option<i64>,
    /// <p>The number of nonmatches in the data that the transform correctly rejected, in the confusion matrix for your transform.</p>
    #[serde(rename = "NumTrueNegatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_true_negatives: Option<i64>,
    /// <p>The number of matches in the data that the transform correctly found, in the confusion matrix for your transform.</p>
    #[serde(rename = "NumTruePositives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_true_positives: Option<i64>,
}

/// <p>Defines a connection to a data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Connection {
    /// <p><p>These key-value pairs define parameters for the connection:</p> <ul> <li> <p> <code>HOST</code> - The host URI: either the fully qualified domain name (FQDN) or the IPv4 address of the database host.</p> </li> <li> <p> <code>PORT</code> - The port number, between 1024 and 65535, of the port on which the database host is listening for database connections.</p> </li> <li> <p> <code>USER<em>NAME</code> - The name under which to log in to the database. The value string for <code>USER</em>NAME</code> is &quot;<code>USERNAME</code>&quot;.</p> </li> <li> <p> <code>PASSWORD</code> - A password, if one is used, for the user name.</p> </li> <li> <p> <code>ENCRYPTED<em>PASSWORD</code> - When you enable connection password protection by setting <code>ConnectionPasswordEncryption</code> in the Data Catalog encryption settings, this field stores the encrypted password.</p> </li> <li> <p> <code>JDBC</em>DRIVER<em>JAR</em>URI</code> - The Amazon Simple Storage Service (Amazon S3) path of the JAR file that contains the JDBC driver to use.</p> </li> <li> <p> <code>JDBC<em>DRIVER</em>CLASS<em>NAME</code> - The class name of the JDBC driver to use.</p> </li> <li> <p> <code>JDBC</em>ENGINE</code> - The name of the JDBC engine to use.</p> </li> <li> <p> <code>JDBC<em>ENGINE</em>VERSION</code> - The version of the JDBC engine to use.</p> </li> <li> <p> <code>CONFIG<em>FILES</code> - (Reserved for future use.)</p> </li> <li> <p> <code>INSTANCE</em>ID</code> - The instance ID to use.</p> </li> <li> <p> <code>JDBC<em>CONNECTION</em>URL</code> - The URL for the JDBC connection.</p> </li> <li> <p> <code>JDBC<em>ENFORCE</em>SSL</code> - A Boolean string (true, false) specifying whether Secure Sockets Layer (SSL) with hostname matching is enforced for the JDBC connection on the client. The default is false.</p> </li> </ul></p>
    #[serde(rename = "ConnectionProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_properties: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the connection. Currently, only JDBC is supported; SFTP is not supported.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>The time that this connection definition was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The description of the connection.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The user, group, or role that last updated this connection definition.</p>
    #[serde(rename = "LastUpdatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    /// <p>The last time that this connection definition was updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>A list of criteria that can be used in selecting this connection.</p>
    #[serde(rename = "MatchCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
    /// <p>The name of the connection definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A map of physical connection requirements, such as virtual private cloud (VPC) and <code>SecurityGroup</code>, that are needed to make this connection successfully.</p>
    #[serde(rename = "PhysicalConnectionRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_connection_requirements: Option<PhysicalConnectionRequirements>,
}

/// <p>A structure that is used to specify a connection to create or update.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConnectionInput {
    /// <p>These key-value pairs define parameters for the connection.</p>
    #[serde(rename = "ConnectionProperties")]
    pub connection_properties: ::std::collections::HashMap<String, String>,
    /// <p>The type of the connection. Currently, only JDBC is supported; SFTP is not supported.</p>
    #[serde(rename = "ConnectionType")]
    pub connection_type: String,
    /// <p>The description of the connection.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A list of criteria that can be used in selecting this connection.</p>
    #[serde(rename = "MatchCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
    /// <p>The name of the connection.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A map of physical connection requirements, such as virtual private cloud (VPC) and <code>SecurityGroup</code>, that are needed to successfully make this connection.</p>
    #[serde(rename = "PhysicalConnectionRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_connection_requirements: Option<PhysicalConnectionRequirements>,
}

/// <p>The data structure used by the Data Catalog to encrypt the password as part of <code>CreateConnection</code> or <code>UpdateConnection</code> and store it in the <code>ENCRYPTED_PASSWORD</code> field in the connection properties. You can enable catalog encryption or only password encryption.</p> <p>When a <code>CreationConnection</code> request arrives containing a password, the Data Catalog first encrypts the password using your AWS KMS key. It then encrypts the whole connection object again if catalog encryption is also enabled.</p> <p>This encryption requires that you set AWS KMS key permissions to enable or restrict access on the password key according to your security requirements. For example, you might want only administrators to have decrypt permission on the password key.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionPasswordEncryption {
    /// <p>An AWS KMS key that is used to encrypt the connection password. </p> <p>If connection password protection is enabled, the caller of <code>CreateConnection</code> and <code>UpdateConnection</code> needs at least <code>kms:Encrypt</code> permission on the specified AWS KMS key, to encrypt passwords before storing them in the Data Catalog. </p> <p>You can set the decrypt permission to enable or restrict access on the password key according to your security requirements.</p>
    #[serde(rename = "AwsKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_kms_key_id: Option<String>,
    /// <p>When the <code>ReturnConnectionPasswordEncrypted</code> flag is set to "true", passwords remain encrypted in the responses of <code>GetConnection</code> and <code>GetConnections</code>. This encryption takes effect independently from catalog encryption. </p>
    #[serde(rename = "ReturnConnectionPasswordEncrypted")]
    pub return_connection_password_encrypted: bool,
}

/// <p>Specifies the connections used by a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionsList {
    /// <p>A list of connections used by the job.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<String>>,
}

/// <p>The details of a crawl in the workflow.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Crawl {
    /// <p>The date and time on which the crawl completed.</p>
    #[serde(rename = "CompletedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    /// <p>The error message associated with the crawl.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The log group associated with the crawl.</p>
    #[serde(rename = "LogGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    /// <p>The log stream associated with the crawl.</p>
    #[serde(rename = "LogStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream: Option<String>,
    /// <p>The date and time on which the crawl started.</p>
    #[serde(rename = "StartedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    /// <p>The state of the crawler.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Specifies a crawler program that examines a data source and uses classifiers to try to determine its schema. If successful, the crawler records metadata concerning the data source in the AWS Glue Data Catalog.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Crawler {
    /// <p>A list of UTF-8 strings that specify the custom classifiers that are associated with the crawler.</p>
    #[serde(rename = "Classifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    /// <p>Crawler configuration information. This versioned JSON string allows users to specify aspects of a crawler's behavior. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html">Configuring a Crawler</a>.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// <p>If the crawler is running, contains the total time elapsed since the last crawl began.</p>
    #[serde(rename = "CrawlElapsedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_elapsed_time: Option<i64>,
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used by this crawler.</p>
    #[serde(rename = "CrawlerSecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_security_configuration: Option<String>,
    /// <p>The time that the crawler was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the database in which the crawler's output is stored.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>A description of the crawler.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The status of the last crawl, and potentially error information if an error occurred.</p>
    #[serde(rename = "LastCrawl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_crawl: Option<LastCrawlInfo>,
    /// <p>The time that the crawler was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The name of the crawler.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that's used to access customer resources, such as Amazon Simple Storage Service (Amazon S3) data.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>For scheduled crawlers, the schedule when the crawler runs.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The policy that specifies update and delete behaviors for the crawler.</p>
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    /// <p>Indicates whether the crawler is running, or whether a run is pending.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The prefix added to the names of tables that are created.</p>
    #[serde(rename = "TablePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<String>,
    /// <p>A collection of targets to crawl.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<CrawlerTargets>,
    /// <p>The version of the crawler.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Metrics for a specified crawler.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CrawlerMetrics {
    /// <p>The name of the crawler.</p>
    #[serde(rename = "CrawlerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name: Option<String>,
    /// <p>The duration of the crawler's most recent run, in seconds.</p>
    #[serde(rename = "LastRuntimeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_runtime_seconds: Option<f64>,
    /// <p>The median duration of this crawler's runs, in seconds.</p>
    #[serde(rename = "MedianRuntimeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub median_runtime_seconds: Option<f64>,
    /// <p>True if the crawler is still estimating how long it will take to complete this run.</p>
    #[serde(rename = "StillEstimating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub still_estimating: Option<bool>,
    /// <p>The number of tables created by this crawler.</p>
    #[serde(rename = "TablesCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_created: Option<i64>,
    /// <p>The number of tables deleted by this crawler.</p>
    #[serde(rename = "TablesDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_deleted: Option<i64>,
    /// <p>The number of tables updated by this crawler.</p>
    #[serde(rename = "TablesUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_updated: Option<i64>,
    /// <p>The estimated time left to complete a running crawl.</p>
    #[serde(rename = "TimeLeftSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_left_seconds: Option<f64>,
}

/// <p>The details of a Crawler node present in the workflow.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CrawlerNodeDetails {
    /// <p>A list of crawls represented by the crawl node.</p>
    #[serde(rename = "Crawls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawls: Option<Vec<Crawl>>,
}

/// <p>Specifies data stores to crawl.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrawlerTargets {
    /// <p>Specifies AWS Glue Data Catalog targets.</p>
    #[serde(rename = "CatalogTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_targets: Option<Vec<CatalogTarget>>,
    /// <p>Specifies Amazon DynamoDB targets.</p>
    #[serde(rename = "DynamoDBTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_targets: Option<Vec<DynamoDBTarget>>,
    /// <p>Specifies JDBC targets.</p>
    #[serde(rename = "JdbcTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jdbc_targets: Option<Vec<JdbcTarget>>,
    /// <p>Specifies Amazon Simple Storage Service (Amazon S3) targets.</p>
    #[serde(rename = "S3Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_targets: Option<Vec<S3Target>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateClassifierRequest {
    /// <p>A <code>CsvClassifier</code> object specifying the classifier to create.</p>
    #[serde(rename = "CsvClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_classifier: Option<CreateCsvClassifierRequest>,
    /// <p>A <code>GrokClassifier</code> object specifying the classifier to create.</p>
    #[serde(rename = "GrokClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<CreateGrokClassifierRequest>,
    /// <p>A <code>JsonClassifier</code> object specifying the classifier to create.</p>
    #[serde(rename = "JsonClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_classifier: Option<CreateJsonClassifierRequest>,
    /// <p>An <code>XMLClassifier</code> object specifying the classifier to create.</p>
    #[serde(rename = "XMLClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_classifier: Option<CreateXMLClassifierRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateClassifierResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConnectionRequest {
    /// <p>The ID of the Data Catalog in which to create the connection. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A <code>ConnectionInput</code> object defining the connection to create.</p>
    #[serde(rename = "ConnectionInput")]
    pub connection_input: ConnectionInput,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConnectionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCrawlerRequest {
    /// <p>A list of custom classifiers that the user has registered. By default, all built-in classifiers are included in a crawl, but these custom classifiers always override the default classifiers for a given classification.</p>
    #[serde(rename = "Classifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    /// <p>The crawler configuration information. This versioned JSON string allows users to specify aspects of a crawler's behavior. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html">Configuring a Crawler</a>.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used by this crawler.</p>
    #[serde(rename = "CrawlerSecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_security_configuration: Option<String>,
    /// <p>The AWS Glue database where results are written, such as: <code>arn:aws:daylight:us-east-1::database/sometable/*</code>.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>A description of the new crawler.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Name of the new crawler.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The IAM role or Amazon Resource Name (ARN) of an IAM role used by the new crawler to access customer resources.</p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>A <code>cron</code> expression used to specify the schedule. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, specify <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The policy for the crawler's update and deletion behavior.</p>
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    /// <p>The table prefix used for catalog tables that are created.</p>
    #[serde(rename = "TablePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<String>,
    /// <p>The tags to use with this crawler request. You can use tags to limit access to the crawler. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">AWS Tags in AWS Glue</a>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of collection of targets to crawl.</p>
    #[serde(rename = "Targets")]
    pub targets: CrawlerTargets,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCrawlerResponse {}

/// <p>Specifies a custom CSV classifier for <code>CreateClassifier</code> to create.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCsvClassifierRequest {
    /// <p>Enables the processing of files that contain only one column.</p>
    #[serde(rename = "AllowSingleColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_single_column: Option<bool>,
    /// <p>Indicates whether the CSV file contains a header.</p>
    #[serde(rename = "ContainsHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_header: Option<String>,
    /// <p>A custom symbol to denote what separates each column entry in the row.</p>
    #[serde(rename = "Delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// <p>Specifies not to trim values before identifying the type of column values. The default value is true.</p>
    #[serde(rename = "DisableValueTrimming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_value_trimming: Option<bool>,
    /// <p>A list of strings representing column names.</p>
    #[serde(rename = "Header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A custom symbol to denote what combines content into a single column value. Must be different from the column delimiter.</p>
    #[serde(rename = "QuoteSymbol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_symbol: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDatabaseRequest {
    /// <p>The ID of the Data Catalog in which to create the database. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The metadata for the database.</p>
    #[serde(rename = "DatabaseInput")]
    pub database_input: DatabaseInput,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDatabaseResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDevEndpointRequest {
    /// <p>A map of arguments used to configure the <code>DevEndpoint</code>.</p>
    #[serde(rename = "Arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name to be assigned to the new <code>DevEndpoint</code>.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
    /// <p>The path to one or more Java <code>.jar</code> files in an S3 bucket that should be loaded in your <code>DevEndpoint</code>.</p>
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    /// <p><p>The paths to one or more Python libraries in an Amazon S3 bucket that should be loaded in your <code>DevEndpoint</code>. Multiple values must be complete paths separated by a comma.</p> <note> <p>You can only use pure Python libraries with a <code>DevEndpoint</code>. Libraries that rely on C extensions, such as the <a href="http://pandas.pydata.org/">pandas</a> Python data analysis library, are not yet supported.</p> </note></p>
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    /// <p>The number of AWS Glue Data Processing Units (DPUs) to allocate to this <code>DevEndpoint</code>.</p>
    #[serde(rename = "NumberOfNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i64>,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated to the development endpoint.</p> <p>The maximum number of workers you can define are 299 for <code>G.1X</code>, and 149 for <code>G.2X</code>. </p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>The public key to be used by this <code>DevEndpoint</code> for authentication. This attribute is provided for backward compatibility because the recommended attribute to use is public keys.</p>
    #[serde(rename = "PublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// <p><p>A list of public keys to be used by the development endpoints for authentication. The use of this attribute is preferred over a single public key because the public keys allow you to have a different private key per client.</p> <note> <p>If you previously created an endpoint with a public key, you must remove that key to be able to set a list of public keys. Call the <code>UpdateDevEndpoint</code> API with the public key content in the <code>deletePublicKeys</code> attribute, and the list of new keys in the <code>addPublicKeys</code> attribute.</p> </note></p>
    #[serde(rename = "PublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,
    /// <p>The IAM role for the <code>DevEndpoint</code>.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used with this <code>DevEndpoint</code>.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>Security group IDs for the security groups to be used by the new <code>DevEndpoint</code>.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The subnet ID for the new <code>DevEndpoint</code> to use.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The tags to use with this DevEndpoint. You may use tags to limit access to the DevEndpoint. For more information about tags in AWS Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">AWS Tags in AWS Glue</a> in the developer guide.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of predefined worker that is allocated to the development endpoint. Accepts a value of Standard, G.1X, or G.2X.</p> <ul> <li> <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p> </li> <li> <p>For the <code>G.1X</code> worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p> </li> <li> <p>For the <code>G.2X</code> worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p> </li> </ul> <p>Known issue: when a development endpoint is created with the <code>G.2X</code> <code>WorkerType</code> configuration, the Spark drivers for the development endpoint will run on 4 vCPU, 16 GB of memory, and a 64 GB disk. </p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDevEndpointResponse {
    /// <p>The map of arguments used to configure this <code>DevEndpoint</code>.</p>
    #[serde(rename = "Arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>The AWS Availability Zone where this <code>DevEndpoint</code> is located.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The point in time at which this <code>DevEndpoint</code> was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The name assigned to the new <code>DevEndpoint</code>.</p>
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    /// <p>Path to one or more Java <code>.jar</code> files in an S3 bucket that will be loaded in your <code>DevEndpoint</code>.</p>
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    /// <p>The paths to one or more Python libraries in an S3 bucket that will be loaded in your <code>DevEndpoint</code>.</p>
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    /// <p>The reason for a current failure in this <code>DevEndpoint</code>.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The number of AWS Glue Data Processing Units (DPUs) allocated to this DevEndpoint.</p>
    #[serde(rename = "NumberOfNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i64>,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated to the development endpoint.</p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the role assigned to the new <code>DevEndpoint</code>.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The name of the <code>SecurityConfiguration</code> structure being used with this <code>DevEndpoint</code>.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The security groups assigned to the new <code>DevEndpoint</code>.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The current status of the new <code>DevEndpoint</code>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The subnet ID assigned to the new <code>DevEndpoint</code>.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The ID of the virtual private cloud (VPC) used by this <code>DevEndpoint</code>.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// <p>The type of predefined worker that is allocated to the development endpoint. May be a value of Standard, G.1X, or G.2X.</p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
    /// <p>The address of the YARN endpoint used by this <code>DevEndpoint</code>.</p>
    #[serde(rename = "YarnEndpointAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yarn_endpoint_address: Option<String>,
    /// <p>The Apache Zeppelin port for the remote Apache Spark interpreter.</p>
    #[serde(rename = "ZeppelinRemoteSparkInterpreterPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_remote_spark_interpreter_port: Option<i64>,
}

/// <p>Specifies a <code>grok</code> classifier for <code>CreateClassifier</code> to create.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGrokClassifierRequest {
    /// <p>An identifier of the data format that the classifier matches, such as Twitter, JSON, Omniture logs, Amazon CloudWatch Logs, and so on.</p>
    #[serde(rename = "Classification")]
    pub classification: String,
    /// <p>Optional custom grok patterns used by this classifier.</p>
    #[serde(rename = "CustomPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_patterns: Option<String>,
    /// <p>The grok pattern used by this classifier.</p>
    #[serde(rename = "GrokPattern")]
    pub grok_pattern: String,
    /// <p>The name of the new classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateJobRequest {
    /// <p>The <code>JobCommand</code> that executes this job.</p>
    #[serde(rename = "Command")]
    pub command: JobCommand,
    /// <p>The connections used for this job.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,
    /// <p>The default arguments for this job.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own Job arguments, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "DefaultArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>Description of the job being defined.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An <code>ExecutionProperty</code> specifying the maximum number of concurrent runs allowed for this job.</p>
    #[serde(rename = "ExecutionProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    /// <p>Glue version determines the versions of Apache Spark and Python that AWS Glue supports. The Python version indicates the version supported for jobs of type Spark. </p> <p>For more information about the available AWS Glue versions and corresponding Spark and Python versions, see <a href="https://docs.aws.amazon.com/glue/latest/dg/add-job.html">Glue version</a> in the developer guide.</p> <p>Jobs that are created without specifying a Glue version default to Glue 0.9.</p>
    #[serde(rename = "GlueVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    /// <p>This field is reserved for future use.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p><p>The number of AWS Glue data processing units (DPUs) that can be allocated when this job runs. A DPU is a relative measure of processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more information, see the <a href="https://aws.amazon.com/glue/pricing/">AWS Glue pricing page</a>.</p> <p>Do not set <code>Max Capacity</code> if using <code>WorkerType</code> and <code>NumberOfWorkers</code>.</p> <p>The value that can be allocated for <code>MaxCapacity</code> depends on whether you are running a Python shell job or an Apache Spark ETL job:</p> <ul> <li> <p>When you specify a Python shell job (<code>JobCommand.Name</code>=&quot;pythonshell&quot;), you can allocate either 0.0625 or 1 DPU. The default is 0.0625 DPU.</p> </li> <li> <p>When you specify an Apache Spark ETL job (<code>JobCommand.Name</code>=&quot;glueetl&quot;), you can allocate from 2 to 100 DPUs. The default is 10 DPUs. This job type cannot have a fractional DPU allocation.</p> </li> </ul></p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>The maximum number of times to retry this job if it fails.</p>
    #[serde(rename = "MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// <p>The name you assign to this job definition. It must be unique in your account.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specifies configuration properties of a job notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated when a job runs.</p> <p>The maximum number of workers you can define are 299 for <code>G.1X</code>, and 149 for <code>G.2X</code>. </p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>The name or Amazon Resource Name (ARN) of the IAM role associated with this job.</p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used with this job.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The tags to use with this job. You may use tags to limit access to the job. For more information about tags in AWS Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">AWS Tags in AWS Glue</a> in the developer guide.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The job timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p><p>The type of predefined worker that is allocated when a job runs. Accepts a value of Standard, G.1X, or G.2X.</p> <ul> <li> <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p> </li> <li> <p>For the <code>G.1X</code> worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p> </li> <li> <p>For the <code>G.2X</code> worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p> </li> </ul></p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateJobResponse {
    /// <p>The unique name that was provided for this job definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Specifies a JSON classifier for <code>CreateClassifier</code> to create.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateJsonClassifierRequest {
    /// <p>A <code>JsonPath</code> string defining the JSON data for the classifier to classify. AWS Glue supports a subset of <code>JsonPath</code>, as described in <a href="https://docs.aws.amazon.com/glue/latest/dg/custom-classifier.html#custom-classifier-json">Writing JsonPath Custom Classifiers</a>.</p>
    #[serde(rename = "JsonPath")]
    pub json_path: String,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateMLTransformRequest {
    /// <p>A description of the machine learning transform that is being defined. The default is an empty string.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A list of AWS Glue table definitions used by the transform.</p>
    #[serde(rename = "InputRecordTables")]
    pub input_record_tables: Vec<GlueTable>,
    /// <p>The number of AWS Glue data processing units (DPUs) that are allocated to task runs for this transform. You can allocate from 2 to 100 DPUs; the default is 10. A DPU is a relative measure of processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more information, see the <a href="https://aws.amazon.com/glue/pricing/">AWS Glue pricing page</a>. </p> <p>When the <code>WorkerType</code> field is set to a value other than <code>Standard</code>, the <code>MaxCapacity</code> field is set automatically and becomes read-only.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>The maximum number of times to retry a task for this transform after a task run fails.</p>
    #[serde(rename = "MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// <p>The unique name that you give the transform when you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated when this task runs.</p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>The algorithmic parameters that are specific to the transform type used. Conditionally dependent on the transform type.</p>
    #[serde(rename = "Parameters")]
    pub parameters: TransformParameters,
    /// <p>The name or Amazon Resource Name (ARN) of the IAM role with the required permissions. Ensure that this role has permission to your Amazon Simple Storage Service (Amazon S3) sources, targets, temporary directory, scripts, and any libraries that are used by the task run for this transform.</p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>The timeout of the task run for this transform in minutes. This is the maximum time that a task run for this transform can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p><p>The type of predefined worker that is allocated when this task runs. Accepts a value of Standard, G.1X, or G.2X.</p> <ul> <li> <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p> </li> <li> <p>For the <code>G.1X</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 64GB disk, and 1 executor per worker.</p> </li> <li> <p>For the <code>G.2X</code> worker type, each worker provides 8 vCPU, 32 GB of memory and a 128GB disk, and 1 executor per worker.</p> </li> </ul></p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMLTransformResponse {
    /// <p>A unique identifier that is generated for the transform.</p>
    #[serde(rename = "TransformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePartitionRequest {
    /// <p>The AWS account ID of the catalog in which the partition is to be created.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the metadata database in which the partition is to be created.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A <code>PartitionInput</code> structure defining the partition to be created.</p>
    #[serde(rename = "PartitionInput")]
    pub partition_input: PartitionInput,
    /// <p>The name of the metadata table in which the partition is to be created.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePartitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateScriptRequest {
    /// <p>A list of the edges in the DAG.</p>
    #[serde(rename = "DagEdges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_edges: Option<Vec<CodeGenEdge>>,
    /// <p>A list of the nodes in the DAG.</p>
    #[serde(rename = "DagNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_nodes: Option<Vec<CodeGenNode>>,
    /// <p>The programming language of the resulting code from the DAG.</p>
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateScriptResponse {
    /// <p>The Python script generated from the DAG.</p>
    #[serde(rename = "PythonScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_script: Option<String>,
    /// <p>The Scala code generated from the DAG.</p>
    #[serde(rename = "ScalaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scala_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSecurityConfigurationRequest {
    /// <p>The encryption configuration for the new security configuration.</p>
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,
    /// <p>The name for the new security configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSecurityConfigurationResponse {
    /// <p>The time at which the new security configuration was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The name assigned to the new security configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTableRequest {
    /// <p>The ID of the Data Catalog in which to create the <code>Table</code>. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The catalog database in which to create the new table. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The <code>TableInput</code> object that defines the metadata table to create in the catalog.</p>
    #[serde(rename = "TableInput")]
    pub table_input: TableInput,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTableResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTriggerRequest {
    /// <p>The actions initiated by this trigger when it fires.</p>
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,
    /// <p>A description of the new trigger.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the trigger.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A predicate to specify when the new trigger should fire.</p> <p>This field is required when the trigger type is <code>CONDITIONAL</code>.</p>
    #[serde(rename = "Predicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    /// <p>A <code>cron</code> expression used to specify the schedule (see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p> <p>This field is required when the trigger type is SCHEDULED.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>Set to <code>true</code> to start <code>SCHEDULED</code> and <code>CONDITIONAL</code> triggers when created. True is not supported for <code>ON_DEMAND</code> triggers.</p>
    #[serde(rename = "StartOnCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on_creation: Option<bool>,
    /// <p>The tags to use with this trigger. You may use tags to limit access to the trigger. For more information about tags in AWS Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">AWS Tags in AWS Glue</a> in the developer guide. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the new trigger.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The name of the workflow associated with the trigger.</p>
    #[serde(rename = "WorkflowName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTriggerResponse {
    /// <p>The name of the trigger.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserDefinedFunctionRequest {
    /// <p>The ID of the Data Catalog in which to create the function. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which to create the function.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A <code>FunctionInput</code> object that defines the function to create in the Data Catalog.</p>
    #[serde(rename = "FunctionInput")]
    pub function_input: UserDefinedFunctionInput,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserDefinedFunctionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateWorkflowRequest {
    /// <p>A collection of properties to be used as part of each execution of the workflow.</p>
    #[serde(rename = "DefaultRunProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_run_properties: Option<::std::collections::HashMap<String, String>>,
    /// <p>A description of the workflow.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name to be assigned to the workflow. It should be unique within your account.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The tags to be used with this workflow.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWorkflowResponse {
    /// <p>The name of the workflow which was provided as part of the request.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Specifies an XML classifier for <code>CreateClassifier</code> to create.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateXMLClassifierRequest {
    /// <p>An identifier of the data format that the classifier matches.</p>
    #[serde(rename = "Classification")]
    pub classification: String,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The XML tag designating the element that contains each record in an XML document being parsed. This can't identify a self-closing element (closed by <code>/&gt;</code>). An empty row element that contains only attributes can be parsed as long as it ends with a closing tag (for example, <code>&lt;row item_a="A" item_b="B"&gt;&lt;/row&gt;</code> is okay, but <code>&lt;row item_a="A" item_b="B" /&gt;</code> is not).</p>
    #[serde(rename = "RowTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_tag: Option<String>,
}

/// <p>A classifier for custom <code>CSV</code> content.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CsvClassifier {
    /// <p>Enables the processing of files that contain only one column.</p>
    #[serde(rename = "AllowSingleColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_single_column: Option<bool>,
    /// <p>Indicates whether the CSV file contains a header.</p>
    #[serde(rename = "ContainsHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_header: Option<String>,
    /// <p>The time that this classifier was registered.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A custom symbol to denote what separates each column entry in the row.</p>
    #[serde(rename = "Delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// <p>Specifies not to trim values before identifying the type of column values. The default value is <code>true</code>.</p>
    #[serde(rename = "DisableValueTrimming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_value_trimming: Option<bool>,
    /// <p>A list of strings representing column names.</p>
    #[serde(rename = "Header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    /// <p>The time that this classifier was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A custom symbol to denote what combines content into a single column value. It must be different from the column delimiter.</p>
    #[serde(rename = "QuoteSymbol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_symbol: Option<String>,
    /// <p>The version of this classifier.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Contains configuration information for maintaining Data Catalog security.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataCatalogEncryptionSettings {
    /// <p>When connection password protection is enabled, the Data Catalog uses a customer-provided key to encrypt the password as part of <code>CreateConnection</code> or <code>UpdateConnection</code> and store it in the <code>ENCRYPTED_PASSWORD</code> field in the connection properties. You can enable catalog encryption or only password encryption.</p>
    #[serde(rename = "ConnectionPasswordEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_password_encryption: Option<ConnectionPasswordEncryption>,
    /// <p>Specifies the encryption-at-rest configuration for the Data Catalog.</p>
    #[serde(rename = "EncryptionAtRest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest: Option<EncryptionAtRest>,
}

/// <p>The AWS Lake Formation principal.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataLakePrincipal {
    /// <p>An identifier for the AWS Lake Formation principal.</p>
    #[serde(rename = "DataLakePrincipalIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_principal_identifier: Option<String>,
}

/// <p>The <code>Database</code> object represents a logical grouping of tables that might reside in a Hive metastore or an RDBMS.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Database {
    /// <p>Creates a set of default permissions on the table for principals. </p>
    #[serde(rename = "CreateTableDefaultPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_table_default_permissions: Option<Vec<PrincipalPermissions>>,
    /// <p>The time at which the metadata database was created in the catalog.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>A description of the database.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The location of the database (for example, an HDFS path).</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    /// <p>The name of the database. For Hive compatibility, this is folded to lowercase when it is stored.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>These key-value pairs define parameters and properties of the database.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The structure used to create or update a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DatabaseInput {
    /// <p>Creates a set of default permissions on the table for principals. </p>
    #[serde(rename = "CreateTableDefaultPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_table_default_permissions: Option<Vec<PrincipalPermissions>>,
    /// <p>A description of the database.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The location of the database (for example, an HDFS path). </p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    /// <p>The name of the database. For Hive compatibility, this is folded to lowercase when it is stored.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>These key-value pairs define parameters and properties of the database.</p> <p>These key-value pairs define parameters and properties of the database.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteClassifierRequest {
    /// <p>Name of the classifier to remove.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteClassifierResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConnectionRequest {
    /// <p>The ID of the Data Catalog in which the connection resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the connection to delete.</p>
    #[serde(rename = "ConnectionName")]
    pub connection_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConnectionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCrawlerRequest {
    /// <p>The name of the crawler to remove.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCrawlerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDatabaseRequest {
    /// <p>The ID of the Data Catalog in which the database resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the database to delete. For Hive compatibility, this must be all lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDatabaseResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDevEndpointRequest {
    /// <p>The name of the <code>DevEndpoint</code>.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDevEndpointResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteJobRequest {
    /// <p>The name of the job definition to delete.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteJobResponse {
    /// <p>The name of the job definition that was deleted.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMLTransformRequest {
    /// <p>The unique identifier of the transform to delete.</p>
    #[serde(rename = "TransformId")]
    pub transform_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMLTransformResponse {
    /// <p>The unique identifier of the transform that was deleted.</p>
    #[serde(rename = "TransformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePartitionRequest {
    /// <p>The ID of the Data Catalog where the partition to be deleted resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which the table in question resides.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The values that define the partition.</p>
    #[serde(rename = "PartitionValues")]
    pub partition_values: Vec<String>,
    /// <p>The name of the table that contains the partition to be deleted.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePartitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResourcePolicyRequest {
    /// <p>The hash value returned when this policy was set.</p>
    #[serde(rename = "PolicyHashCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash_condition: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteResourcePolicyResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSecurityConfigurationRequest {
    /// <p>The name of the security configuration to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSecurityConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTableRequest {
    /// <p>The ID of the Data Catalog where the table resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table to be deleted. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTableResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTableVersionRequest {
    /// <p>The ID of the Data Catalog where the tables reside. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// <p>The ID of the table version to be deleted. A <code>VersionID</code> is a string representation of an integer. Each version is incremented by 1.</p>
    #[serde(rename = "VersionId")]
    pub version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTableVersionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTriggerRequest {
    /// <p>The name of the trigger to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTriggerResponse {
    /// <p>The name of the trigger that was deleted.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserDefinedFunctionRequest {
    /// <p>The ID of the Data Catalog where the function to be deleted is located. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the function is located.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the function definition to be deleted.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUserDefinedFunctionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteWorkflowRequest {
    /// <p>Name of the workflow to be deleted.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteWorkflowResponse {
    /// <p>Name of the workflow specified in input.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A development endpoint where a developer can remotely debug extract, transform, and load (ETL) scripts.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DevEndpoint {
    /// <p>A map of arguments used to configure the <code>DevEndpoint</code>.</p> <p>Currently, only <code>"--enable-glue-datacatalog": ""</code> is supported as a valid argument.</p>
    #[serde(rename = "Arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>The AWS Availability Zone where this <code>DevEndpoint</code> is located.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The point in time at which this DevEndpoint was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The name of the <code>DevEndpoint</code>.</p>
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    /// <p><p>The path to one or more Java <code>.jar</code> files in an S3 bucket that should be loaded in your <code>DevEndpoint</code>.</p> <note> <p>You can only use pure Java/Scala libraries with a <code>DevEndpoint</code>.</p> </note></p>
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    /// <p><p>The paths to one or more Python libraries in an Amazon S3 bucket that should be loaded in your <code>DevEndpoint</code>. Multiple values must be complete paths separated by a comma.</p> <note> <p>You can only use pure Python libraries with a <code>DevEndpoint</code>. Libraries that rely on C extensions, such as the <a href="http://pandas.pydata.org/">pandas</a> Python data analysis library, are not currently supported.</p> </note></p>
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    /// <p>The reason for a current failure in this <code>DevEndpoint</code>.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The point in time at which this <code>DevEndpoint</code> was last modified.</p>
    #[serde(rename = "LastModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<f64>,
    /// <p>The status of the last update.</p>
    #[serde(rename = "LastUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status: Option<String>,
    /// <p>The number of AWS Glue Data Processing Units (DPUs) allocated to this <code>DevEndpoint</code>.</p>
    #[serde(rename = "NumberOfNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i64>,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated to the development endpoint.</p> <p>The maximum number of workers you can define are 299 for <code>G.1X</code>, and 149 for <code>G.2X</code>. </p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>A private IP address to access the <code>DevEndpoint</code> within a VPC if the <code>DevEndpoint</code> is created within one. The <code>PrivateAddress</code> field is present only when you create the <code>DevEndpoint</code> within your VPC.</p>
    #[serde(rename = "PrivateAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_address: Option<String>,
    /// <p>The public IP address used by this <code>DevEndpoint</code>. The <code>PublicAddress</code> field is present only when you create a non-virtual private cloud (VPC) <code>DevEndpoint</code>.</p>
    #[serde(rename = "PublicAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_address: Option<String>,
    /// <p>The public key to be used by this <code>DevEndpoint</code> for authentication. This attribute is provided for backward compatibility because the recommended attribute to use is public keys.</p>
    #[serde(rename = "PublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// <p><p>A list of public keys to be used by the <code>DevEndpoints</code> for authentication. Using this attribute is preferred over a single public key because the public keys allow you to have a different private key per client.</p> <note> <p>If you previously created an endpoint with a public key, you must remove that key to be able to set a list of public keys. Call the <code>UpdateDevEndpoint</code> API operation with the public key content in the <code>deletePublicKeys</code> attribute, and the list of new keys in the <code>addPublicKeys</code> attribute.</p> </note></p>
    #[serde(rename = "PublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role used in this <code>DevEndpoint</code>.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used with this <code>DevEndpoint</code>.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>A list of security group identifiers used in this <code>DevEndpoint</code>.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The current status of this <code>DevEndpoint</code>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The subnet ID for this <code>DevEndpoint</code>.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The ID of the virtual private cloud (VPC) used by this <code>DevEndpoint</code>.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// <p>The type of predefined worker that is allocated to the development endpoint. Accepts a value of Standard, G.1X, or G.2X.</p> <ul> <li> <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p> </li> <li> <p>For the <code>G.1X</code> worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p> </li> <li> <p>For the <code>G.2X</code> worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p> </li> </ul> <p>Known issue: when a development endpoint is created with the <code>G.2X</code> <code>WorkerType</code> configuration, the Spark drivers for the development endpoint will run on 4 vCPU, 16 GB of memory, and a 64 GB disk. </p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
    /// <p>The YARN endpoint address used by this <code>DevEndpoint</code>.</p>
    #[serde(rename = "YarnEndpointAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yarn_endpoint_address: Option<String>,
    /// <p>The Apache Zeppelin port for the remote Apache Spark interpreter.</p>
    #[serde(rename = "ZeppelinRemoteSparkInterpreterPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_remote_spark_interpreter_port: Option<i64>,
}

/// <p>Custom libraries to be loaded into a development endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DevEndpointCustomLibraries {
    /// <p><p>The path to one or more Java <code>.jar</code> files in an S3 bucket that should be loaded in your <code>DevEndpoint</code>.</p> <note> <p>You can only use pure Java/Scala libraries with a <code>DevEndpoint</code>.</p> </note></p>
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    /// <p><p>The paths to one or more Python libraries in an Amazon Simple Storage Service (Amazon S3) bucket that should be loaded in your <code>DevEndpoint</code>. Multiple values must be complete paths separated by a comma.</p> <note> <p>You can only use pure Python libraries with a <code>DevEndpoint</code>. Libraries that rely on C extensions, such as the <a href="http://pandas.pydata.org/">pandas</a> Python data analysis library, are not currently supported.</p> </note></p>
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
}

/// <p>Specifies an Amazon DynamoDB table to crawl.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamoDBTarget {
    /// <p>The name of the DynamoDB table to crawl.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>An edge represents a directed connection between two AWS Glue components which are part of the workflow the edge belongs to.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Edge {
    /// <p>The unique of the node within the workflow where the edge ends.</p>
    #[serde(rename = "DestinationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    /// <p>The unique of the node within the workflow where the edge starts.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}

/// <p>Specifies the encryption-at-rest configuration for the Data Catalog.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionAtRest {
    /// <p>The encryption-at-rest mode for encrypting Data Catalog data.</p>
    #[serde(rename = "CatalogEncryptionMode")]
    pub catalog_encryption_mode: String,
    /// <p>The ID of the AWS KMS key to use for encryption at rest.</p>
    #[serde(rename = "SseAwsKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_aws_kms_key_id: Option<String>,
}

/// <p>Specifies an encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    /// <p>The encryption configuration for Amazon CloudWatch.</p>
    #[serde(rename = "CloudWatchEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption: Option<CloudWatchEncryption>,
    /// <p>The encryption configuration for job bookmarks.</p>
    #[serde(rename = "JobBookmarksEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmarks_encryption: Option<JobBookmarksEncryption>,
    /// <p>The encryption configuration for Amazon Simple Storage Service (Amazon S3) data.</p>
    #[serde(rename = "S3Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption: Option<Vec<S3Encryption>>,
}

/// <p>Contains details about an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorDetail {
    /// <p>The code associated with this error.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A message describing the error.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// <p>Evaluation metrics provide an estimate of the quality of your machine learning transform.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EvaluationMetrics {
    /// <p>The evaluation metrics for the find matches algorithm.</p>
    #[serde(rename = "FindMatchesMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_matches_metrics: Option<FindMatchesMetrics>,
    /// <p>The type of machine learning transform.</p>
    #[serde(rename = "TransformType")]
    pub transform_type: String,
}

/// <p>An execution property of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionProperty {
    /// <p>The maximum number of concurrent runs allowed for the job. The default is 1. An error is returned when this threshold is reached. The maximum value you can specify is controlled by a service limit.</p>
    #[serde(rename = "MaxConcurrentRuns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_runs: Option<i64>,
}

/// <p>Specifies configuration properties for an exporting labels task run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportLabelsTaskRunProperties {
    /// <p>The Amazon Simple Storage Service (Amazon S3) path where you will export the labels.</p>
    #[serde(rename = "OutputS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_path: Option<String>,
}

/// <p>The evaluation metrics for the find matches algorithm. The quality of your machine learning transform is measured by getting your transform to predict some matches and comparing the results to known matches from the same dataset. The quality metrics are based on a subset of your data, so they are not precise.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FindMatchesMetrics {
    /// <p>The area under the precision/recall curve (AUPRC) is a single number measuring the overall quality of the transform, that is independent of the choice made for precision vs. recall. Higher values indicate that you have a more attractive precision vs. recall tradeoff.</p> <p>For more information, see <a href="https://en.wikipedia.org/wiki/Precision_and_recall">Precision and recall</a> in Wikipedia.</p>
    #[serde(rename = "AreaUnderPRCurve")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_under_pr_curve: Option<f64>,
    /// <p>The confusion matrix shows you what your transform is predicting accurately and what types of errors it is making.</p> <p>For more information, see <a href="https://en.wikipedia.org/wiki/Confusion_matrix">Confusion matrix</a> in Wikipedia.</p>
    #[serde(rename = "ConfusionMatrix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confusion_matrix: Option<ConfusionMatrix>,
    /// <p>The maximum F1 metric indicates the transform's accuracy between 0 and 1, where 1 is the best accuracy.</p> <p>For more information, see <a href="https://en.wikipedia.org/wiki/F1_score">F1 score</a> in Wikipedia.</p>
    #[serde(rename = "F1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1: Option<f64>,
    /// <p>The precision metric indicates when often your transform is correct when it predicts a match. Specifically, it measures how well the transform finds true positives from the total true positives possible.</p> <p>For more information, see <a href="https://en.wikipedia.org/wiki/Precision_and_recall">Precision and recall</a> in Wikipedia.</p>
    #[serde(rename = "Precision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    /// <p>The recall metric indicates that for an actual match, how often your transform predicts the match. Specifically, it measures how well the transform finds true positives from the total records in the source data.</p> <p>For more information, see <a href="https://en.wikipedia.org/wiki/Precision_and_recall">Precision and recall</a> in Wikipedia.</p>
    #[serde(rename = "Recall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
}

/// <p>The parameters to configure the find matches transform.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FindMatchesParameters {
    /// <p>The value that is selected when tuning your transform for a balance between accuracy and cost. A value of 0.5 means that the system balances accuracy and cost concerns. A value of 1.0 means a bias purely for accuracy, which typically results in a higher cost, sometimes substantially higher. A value of 0.0 means a bias purely for cost, which results in a less accurate <code>FindMatches</code> transform, sometimes with unacceptable accuracy.</p> <p>Accuracy measures how well the transform finds true positives and true negatives. Increasing accuracy requires more machine resources and cost. But it also results in increased recall. </p> <p>Cost measures how many compute resources, and thus money, are consumed to run the transform.</p>
    #[serde(rename = "AccuracyCostTradeoff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_cost_tradeoff: Option<f64>,
    /// <p>The value to switch on or off to force the output to match the provided labels from users. If the value is <code>True</code>, the <code>find matches</code> transform forces the output to match the provided labels. The results override the normal conflation results. If the value is <code>False</code>, the <code>find matches</code> transform does not ensure all the labels provided are respected, and the results rely on the trained model.</p> <p>Note that setting this value to true may increase the conflation execution time.</p>
    #[serde(rename = "EnforceProvidedLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_provided_labels: Option<bool>,
    /// <p>The value selected when tuning your transform for a balance between precision and recall. A value of 0.5 means no preference; a value of 1.0 means a bias purely for precision, and a value of 0.0 means a bias for recall. Because this is a tradeoff, choosing values close to 1.0 means very low recall, and choosing values close to 0.0 results in very low precision.</p> <p>The precision metric indicates how often your model is correct when it predicts a match. </p> <p>The recall metric indicates that for an actual match, how often your model predicts the match.</p>
    #[serde(rename = "PrecisionRecallTradeoff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision_recall_tradeoff: Option<f64>,
    /// <p>The name of a column that uniquely identifies rows in the source table. Used to help identify matching records.</p>
    #[serde(rename = "PrimaryKeyColumnName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key_column_name: Option<String>,
}

/// <p>Specifies configuration properties for a Find Matches task run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FindMatchesTaskRunProperties {
    /// <p>The job ID for the Find Matches task run.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The name assigned to the job for the Find Matches task run.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The job run ID for the Find Matches task run.</p>
    #[serde(rename = "JobRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCatalogImportStatusRequest {
    /// <p>The ID of the catalog to migrate. Currently, this should be the AWS account ID.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCatalogImportStatusResponse {
    /// <p>The status of the specified catalog migration.</p>
    #[serde(rename = "ImportStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<CatalogImportStatus>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetClassifierRequest {
    /// <p>Name of the classifier to retrieve.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetClassifierResponse {
    /// <p>The requested classifier.</p>
    #[serde(rename = "Classifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifier: Option<Classifier>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetClassifiersRequest {
    /// <p>The size of the list to return (optional).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional continuation token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetClassifiersResponse {
    /// <p>The requested list of classifier objects.</p>
    #[serde(rename = "Classifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<Classifier>>,
    /// <p>A continuation token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectionRequest {
    /// <p>The ID of the Data Catalog in which the connection resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>Allows you to retrieve the connection metadata without returning the password. For instance, the AWS Glue console uses this flag to retrieve the connection, and does not display the password. Set this parameter when the caller might not have permission to use the AWS KMS key to decrypt the password, but it does have permission to access the rest of the connection properties.</p>
    #[serde(rename = "HidePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_password: Option<bool>,
    /// <p>The name of the connection definition to retrieve.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConnectionResponse {
    /// <p>The requested connection definition.</p>
    #[serde(rename = "Connection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

/// <p>Filters the connection definitions that are returned by the <code>GetConnections</code> API operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectionsFilter {
    /// <p>The type of connections to return. Currently, only JDBC is supported; SFTP is not supported.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>A criteria string that must match the criteria recorded in the connection definition for that connection definition to be returned.</p>
    #[serde(rename = "MatchCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectionsRequest {
    /// <p>The ID of the Data Catalog in which the connections reside. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A filter that controls which connections are returned.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<GetConnectionsFilter>,
    /// <p>Allows you to retrieve the connection metadata without returning the password. For instance, the AWS Glue console uses this flag to retrieve the connection, and does not display the password. Set this parameter when the caller might not have permission to use the AWS KMS key to decrypt the password, but it does have permission to access the rest of the connection properties.</p>
    #[serde(rename = "HidePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_password: Option<bool>,
    /// <p>The maximum number of connections to return in one response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConnectionsResponse {
    /// <p>A list of requested connection definitions.</p>
    #[serde(rename = "ConnectionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_list: Option<Vec<Connection>>,
    /// <p>A continuation token, if the list of connections returned does not include the last of the filtered connections.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCrawlerMetricsRequest {
    /// <p>A list of the names of crawlers about which to retrieve metrics.</p>
    #[serde(rename = "CrawlerNameList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name_list: Option<Vec<String>>,
    /// <p>The maximum size of a list to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCrawlerMetricsResponse {
    /// <p>A list of metrics for the specified crawler.</p>
    #[serde(rename = "CrawlerMetricsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_metrics_list: Option<Vec<CrawlerMetrics>>,
    /// <p>A continuation token, if the returned list does not contain the last metric available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCrawlerRequest {
    /// <p>The name of the crawler to retrieve metadata for.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCrawlerResponse {
    /// <p>The metadata for the specified crawler.</p>
    #[serde(rename = "Crawler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler: Option<Crawler>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCrawlersRequest {
    /// <p>The number of crawlers to return on each call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCrawlersResponse {
    /// <p>A list of crawler metadata.</p>
    #[serde(rename = "Crawlers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawlers: Option<Vec<Crawler>>,
    /// <p>A continuation token, if the returned list has not reached the end of those defined in this customer account.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDataCatalogEncryptionSettingsRequest {
    /// <p>The ID of the Data Catalog to retrieve the security configuration for. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDataCatalogEncryptionSettingsResponse {
    /// <p>The requested security configuration.</p>
    #[serde(rename = "DataCatalogEncryptionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_encryption_settings: Option<DataCatalogEncryptionSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDatabaseRequest {
    /// <p>The ID of the Data Catalog in which the database resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the database to retrieve. For Hive compatibility, this should be all lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDatabaseResponse {
    /// <p>The definition of the specified database in the Data Catalog.</p>
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Database>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDatabasesRequest {
    /// <p>The ID of the Data Catalog from which to retrieve <code>Databases</code>. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The maximum number of databases to return in one response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDatabasesResponse {
    /// <p>A list of <code>Database</code> objects from the specified catalog.</p>
    #[serde(rename = "DatabaseList")]
    pub database_list: Vec<Database>,
    /// <p>A continuation token for paginating the returned list of tokens, returned if the current segment of the list is not the last.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDataflowGraphRequest {
    /// <p>The Python script to transform.</p>
    #[serde(rename = "PythonScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_script: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDataflowGraphResponse {
    /// <p>A list of the edges in the resulting DAG.</p>
    #[serde(rename = "DagEdges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_edges: Option<Vec<CodeGenEdge>>,
    /// <p>A list of the nodes in the resulting DAG.</p>
    #[serde(rename = "DagNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_nodes: Option<Vec<CodeGenNode>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDevEndpointRequest {
    /// <p>Name of the <code>DevEndpoint</code> to retrieve information for.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDevEndpointResponse {
    /// <p>A <code>DevEndpoint</code> definition.</p>
    #[serde(rename = "DevEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoint: Option<DevEndpoint>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDevEndpointsRequest {
    /// <p>The maximum size of information to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDevEndpointsResponse {
    /// <p>A list of <code>DevEndpoint</code> definitions.</p>
    #[serde(rename = "DevEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoints: Option<Vec<DevEndpoint>>,
    /// <p>A continuation token, if not all <code>DevEndpoint</code> definitions have yet been returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobBookmarkRequest {
    /// <p>The name of the job in question.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>The unique run identifier associated with this job run.</p>
    #[serde(rename = "RunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobBookmarkResponse {
    /// <p>A structure that defines a point that a job can resume processing.</p>
    #[serde(rename = "JobBookmarkEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmark_entry: Option<JobBookmarkEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobRequest {
    /// <p>The name of the job definition to retrieve.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobResponse {
    /// <p>The requested job definition.</p>
    #[serde(rename = "Job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobRunRequest {
    /// <p>Name of the job definition being run.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>True if a list of predecessor runs should be returned.</p>
    #[serde(rename = "PredecessorsIncluded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predecessors_included: Option<bool>,
    /// <p>The ID of the job run.</p>
    #[serde(rename = "RunId")]
    pub run_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobRunResponse {
    /// <p>The requested job-run metadata.</p>
    #[serde(rename = "JobRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run: Option<JobRun>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobRunsRequest {
    /// <p>The name of the job definition for which to retrieve all job runs.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>The maximum size of the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobRunsResponse {
    /// <p>A list of job-run metadata objects.</p>
    #[serde(rename = "JobRuns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_runs: Option<Vec<JobRun>>,
    /// <p>A continuation token, if not all requested job runs have been returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobsRequest {
    /// <p>The maximum size of the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobsResponse {
    /// <p>A list of job definitions.</p>
    #[serde(rename = "Jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    /// <p>A continuation token, if not all job definitions have yet been returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMLTaskRunRequest {
    /// <p>The unique identifier of the task run.</p>
    #[serde(rename = "TaskRunId")]
    pub task_run_id: String,
    /// <p>The unique identifier of the machine learning transform.</p>
    #[serde(rename = "TransformId")]
    pub transform_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMLTaskRunResponse {
    /// <p>The date and time when this task run was completed.</p>
    #[serde(rename = "CompletedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    /// <p>The error strings that are associated with the task run.</p>
    #[serde(rename = "ErrorString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    /// <p>The amount of time (in seconds) that the task run consumed resources.</p>
    #[serde(rename = "ExecutionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i64>,
    /// <p>The date and time when this task run was last modified.</p>
    #[serde(rename = "LastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>The names of the log groups that are associated with the task run.</p>
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>The list of properties that are associated with the task run.</p>
    #[serde(rename = "Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TaskRunProperties>,
    /// <p>The date and time when this task run started.</p>
    #[serde(rename = "StartedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    /// <p>The status for this task run.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The unique run identifier associated with this run.</p>
    #[serde(rename = "TaskRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
    /// <p>The unique identifier of the task run.</p>
    #[serde(rename = "TransformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMLTaskRunsRequest {
    /// <p>The filter criteria, in the <code>TaskRunFilterCriteria</code> structure, for the task run.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TaskRunFilterCriteria>,
    /// <p>The maximum number of results to return. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token for pagination of the results. The default is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sorting criteria, in the <code>TaskRunSortCriteria</code> structure, for the task run.</p>
    #[serde(rename = "Sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TaskRunSortCriteria>,
    /// <p>The unique identifier of the machine learning transform.</p>
    #[serde(rename = "TransformId")]
    pub transform_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMLTaskRunsResponse {
    /// <p>A pagination token, if more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of task runs that are associated with the transform.</p>
    #[serde(rename = "TaskRuns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_runs: Option<Vec<TaskRun>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMLTransformRequest {
    /// <p>The unique identifier of the transform, generated at the time that the transform was created.</p>
    #[serde(rename = "TransformId")]
    pub transform_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMLTransformResponse {
    /// <p>The date and time when the transform was created.</p>
    #[serde(rename = "CreatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    /// <p>A description of the transform.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The latest evaluation metrics.</p>
    #[serde(rename = "EvaluationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_metrics: Option<EvaluationMetrics>,
    /// <p>A list of AWS Glue table definitions used by the transform.</p>
    #[serde(rename = "InputRecordTables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_record_tables: Option<Vec<GlueTable>>,
    /// <p>The number of labels available for this transform.</p>
    #[serde(rename = "LabelCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_count: Option<i64>,
    /// <p>The date and time when the transform was last modified.</p>
    #[serde(rename = "LastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>The number of AWS Glue data processing units (DPUs) that are allocated to task runs for this transform. You can allocate from 2 to 100 DPUs; the default is 10. A DPU is a relative measure of processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more information, see the <a href="https://aws.amazon.com/glue/pricing/">AWS Glue pricing page</a>. </p> <p>When the <code>WorkerType</code> field is set to a value other than <code>Standard</code>, the <code>MaxCapacity</code> field is set automatically and becomes read-only.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>The maximum number of times to retry a task for this transform after a task run fails.</p>
    #[serde(rename = "MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// <p>The unique name given to the transform when it was created.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated when this task runs.</p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>The configuration parameters that are specific to the algorithm used.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<TransformParameters>,
    /// <p>The name or Amazon Resource Name (ARN) of the IAM role with the required permissions.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The <code>Map&lt;Column, Type&gt;</code> object that represents the schema that this transform accepts. Has an upper bound of 100 columns.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Vec<SchemaColumn>>,
    /// <p>The last known status of the transform (to indicate whether it can be used or not). One of "NOT_READY", "READY", or "DELETING".</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The timeout for a task run for this transform in minutes. This is the maximum time that a task run for this transform can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The unique identifier of the transform, generated at the time that the transform was created.</p>
    #[serde(rename = "TransformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
    /// <p><p>The type of predefined worker that is allocated when this task runs. Accepts a value of Standard, G.1X, or G.2X.</p> <ul> <li> <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p> </li> <li> <p>For the <code>G.1X</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 64GB disk, and 1 executor per worker.</p> </li> <li> <p>For the <code>G.2X</code> worker type, each worker provides 8 vCPU, 32 GB of memory and a 128GB disk, and 1 executor per worker.</p> </li> </ul></p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMLTransformsRequest {
    /// <p>The filter transformation criteria.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TransformFilterCriteria>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A paginated token to offset the results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sorting criteria.</p>
    #[serde(rename = "Sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TransformSortCriteria>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMLTransformsResponse {
    /// <p>A pagination token, if more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of machine learning transforms.</p>
    #[serde(rename = "Transforms")]
    pub transforms: Vec<MLTransform>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMappingRequest {
    /// <p>Parameters for the mapping.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>A list of target tables.</p>
    #[serde(rename = "Sinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sinks: Option<Vec<CatalogEntry>>,
    /// <p>Specifies the source table.</p>
    #[serde(rename = "Source")]
    pub source: CatalogEntry,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMappingResponse {
    /// <p>A list of mappings to the specified targets.</p>
    #[serde(rename = "Mapping")]
    pub mapping: Vec<MappingEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPartitionRequest {
    /// <p>The ID of the Data Catalog where the partition in question resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the partition resides.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The values that define the partition.</p>
    #[serde(rename = "PartitionValues")]
    pub partition_values: Vec<String>,
    /// <p>The name of the partition's table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPartitionResponse {
    /// <p>The requested information, in the form of a <code>Partition</code> object.</p>
    #[serde(rename = "Partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<Partition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPartitionsRequest {
    /// <p>The ID of the Data Catalog where the partitions in question reside. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the partitions reside.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>An expression that filters the partitions to be returned.</p> <p>The expression uses SQL syntax similar to the SQL <code>WHERE</code> filter clause. The SQL statement parser <a href="http://jsqlparser.sourceforge.net/home.php">JSQLParser</a> parses the expression. </p> <p> <i>Operators</i>: The following are the operators that you can use in the <code>Expression</code> API call:</p> <dl> <dt>=</dt> <dd> <p>Checks whether the values of the two operands are equal; if yes, then the condition becomes true.</p> <p>Example: Assume 'variable a' holds 10 and 'variable b' holds 20. </p> <p>(a = b) is not true.</p> </dd> <dt>&lt; &gt;</dt> <dd> <p>Checks whether the values of two operands are equal; if the values are not equal, then the condition becomes true.</p> <p>Example: (a &lt; &gt; b) is true.</p> </dd> <dt>&gt;</dt> <dd> <p>Checks whether the value of the left operand is greater than the value of the right operand; if yes, then the condition becomes true.</p> <p>Example: (a &gt; b) is not true.</p> </dd> <dt>&lt;</dt> <dd> <p>Checks whether the value of the left operand is less than the value of the right operand; if yes, then the condition becomes true.</p> <p>Example: (a &lt; b) is true.</p> </dd> <dt>&gt;=</dt> <dd> <p>Checks whether the value of the left operand is greater than or equal to the value of the right operand; if yes, then the condition becomes true.</p> <p>Example: (a &gt;= b) is not true.</p> </dd> <dt>&lt;=</dt> <dd> <p>Checks whether the value of the left operand is less than or equal to the value of the right operand; if yes, then the condition becomes true.</p> <p>Example: (a &lt;= b) is true.</p> </dd> <dt>AND, OR, IN, BETWEEN, LIKE, NOT, IS NULL</dt> <dd> <p>Logical operators.</p> </dd> </dl> <p> <i>Supported Partition Key Types</i>: The following are the supported partition keys.</p> <ul> <li> <p> <code>string</code> </p> </li> <li> <p> <code>date</code> </p> </li> <li> <p> <code>timestamp</code> </p> </li> <li> <p> <code>int</code> </p> </li> <li> <p> <code>bigint</code> </p> </li> <li> <p> <code>long</code> </p> </li> <li> <p> <code>tinyint</code> </p> </li> <li> <p> <code>smallint</code> </p> </li> <li> <p> <code>decimal</code> </p> </li> </ul> <p>If an invalid type is encountered, an exception is thrown. </p> <p>The following list shows the valid operators on each type. When you define a crawler, the <code>partitionKey</code> type is created as a <code>STRING</code>, to be compatible with the catalog partitions. </p> <p> <i>Sample API Call</i>: </p>
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The maximum number of partitions to return in a single response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is not the first call to retrieve these partitions.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The segment of the table's partitions to scan in this request.</p>
    #[serde(rename = "Segment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Segment>,
    /// <p>The name of the partitions' table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPartitionsResponse {
    /// <p>A continuation token, if the returned list of partitions does not include the last one.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of requested partitions.</p>
    #[serde(rename = "Partitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<Partition>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPlanRequest {
    /// <p>The programming language of the code to perform the mapping.</p>
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// <p>The parameters for the mapping.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The list of mappings from a source table to target tables.</p>
    #[serde(rename = "Mapping")]
    pub mapping: Vec<MappingEntry>,
    /// <p>The target tables.</p>
    #[serde(rename = "Sinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sinks: Option<Vec<CatalogEntry>>,
    /// <p>The source table.</p>
    #[serde(rename = "Source")]
    pub source: CatalogEntry,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPlanResponse {
    /// <p>A Python script to perform the mapping.</p>
    #[serde(rename = "PythonScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_script: Option<String>,
    /// <p>The Scala code to perform the mapping.</p>
    #[serde(rename = "ScalaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scala_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourcePolicyRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourcePolicyResponse {
    /// <p>The date and time at which the policy was created.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>Contains the hash value associated with this policy.</p>
    #[serde(rename = "PolicyHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash: Option<String>,
    /// <p>Contains the requested policy document, in JSON format.</p>
    #[serde(rename = "PolicyInJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_in_json: Option<String>,
    /// <p>The date and time at which the policy was last updated.</p>
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSecurityConfigurationRequest {
    /// <p>The name of the security configuration to retrieve.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSecurityConfigurationResponse {
    /// <p>The requested security configuration.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<SecurityConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSecurityConfigurationsRequest {
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSecurityConfigurationsResponse {
    /// <p>A continuation token, if there are more security configurations to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of security configurations.</p>
    #[serde(rename = "SecurityConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configurations: Option<Vec<SecurityConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTableRequest {
    /// <p>The ID of the Data Catalog where the table resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table for which to retrieve the definition. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTableResponse {
    /// <p>The <code>Table</code> object that defines the specified table.</p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTableVersionRequest {
    /// <p>The ID of the Data Catalog where the tables reside. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// <p>The ID value of the table version to be retrieved. A <code>VersionID</code> is a string representation of an integer. Each version is incremented by 1. </p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTableVersionResponse {
    /// <p>The requested table version.</p>
    #[serde(rename = "TableVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_version: Option<TableVersion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTableVersionsRequest {
    /// <p>The ID of the Data Catalog where the tables reside. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The maximum number of table versions to return in one response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is not the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTableVersionsResponse {
    /// <p>A continuation token, if the list of available versions does not include the last one.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of strings identifying available versions of the specified table.</p>
    #[serde(rename = "TableVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_versions: Option<Vec<TableVersion>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTablesRequest {
    /// <p>The ID of the Data Catalog where the tables reside. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The database in the catalog whose tables to list. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A regular expression pattern. If present, only those tables whose names match the pattern are returned.</p>
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The maximum number of tables to return in a single response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, included if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTablesResponse {
    /// <p>A continuation token, present if the current list segment is not the last.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the requested <code>Table</code> objects.</p>
    #[serde(rename = "TableList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_list: Option<Vec<Table>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTagsRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource for which to retrieve tags.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTagsResponse {
    /// <p>The requested tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTriggerRequest {
    /// <p>The name of the trigger to retrieve.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTriggerResponse {
    /// <p>The requested trigger definition.</p>
    #[serde(rename = "Trigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTriggersRequest {
    /// <p>The name of the job to retrieve triggers for. The trigger that can start this job is returned, and if there is no such trigger, all triggers are returned.</p>
    #[serde(rename = "DependentJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_job_name: Option<String>,
    /// <p>The maximum size of the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTriggersResponse {
    /// <p>A continuation token, if not all the requested triggers have yet been returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of triggers for the specified job.</p>
    #[serde(rename = "Triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<Trigger>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUserDefinedFunctionRequest {
    /// <p>The ID of the Data Catalog where the function to be retrieved is located. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the function is located.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the function.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUserDefinedFunctionResponse {
    /// <p>The requested function definition.</p>
    #[serde(rename = "UserDefinedFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_function: Option<UserDefinedFunction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUserDefinedFunctionsRequest {
    /// <p>The ID of the Data Catalog where the functions to be retrieved are located. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the functions are located.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The maximum number of functions to return in one response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An optional function-name pattern string that filters the function definitions returned.</p>
    #[serde(rename = "Pattern")]
    pub pattern: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUserDefinedFunctionsResponse {
    /// <p>A continuation token, if the list of functions returned does not include the last requested function.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of requested function definitions.</p>
    #[serde(rename = "UserDefinedFunctions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_functions: Option<Vec<UserDefinedFunction>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetWorkflowRequest {
    /// <p>Specifies whether to include a graph when returning the workflow resource metadata.</p>
    #[serde(rename = "IncludeGraph")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_graph: Option<bool>,
    /// <p>The name of the workflow to retrieve.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetWorkflowResponse {
    /// <p>The resource metadata for the workflow.</p>
    #[serde(rename = "Workflow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Workflow>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetWorkflowRunPropertiesRequest {
    /// <p>Name of the workflow which was run.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The ID of the workflow run whose run properties should be returned.</p>
    #[serde(rename = "RunId")]
    pub run_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetWorkflowRunPropertiesResponse {
    /// <p>The workflow run properties which were set during the specified run.</p>
    #[serde(rename = "RunProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_properties: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetWorkflowRunRequest {
    /// <p>Specifies whether to include the workflow graph in response or not.</p>
    #[serde(rename = "IncludeGraph")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_graph: Option<bool>,
    /// <p>Name of the workflow being run.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The ID of the workflow run.</p>
    #[serde(rename = "RunId")]
    pub run_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetWorkflowRunResponse {
    /// <p>The requested workflow run metadata.</p>
    #[serde(rename = "Run")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<WorkflowRun>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetWorkflowRunsRequest {
    /// <p>Specifies whether to include the workflow graph in response or not.</p>
    #[serde(rename = "IncludeGraph")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_graph: Option<bool>,
    /// <p>The maximum number of workflow runs to be included in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Name of the workflow whose metadata of runs should be returned.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The maximum size of the response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetWorkflowRunsResponse {
    /// <p>A continuation token, if not all requested workflow runs have been returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of workflow run metadata objects.</p>
    #[serde(rename = "Runs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs: Option<Vec<WorkflowRun>>,
}

/// <p>The database and table in the AWS Glue Data Catalog that is used for input or output data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlueTable {
    /// <p>A unique identifier for the AWS Glue Data Catalog.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the connection to the AWS Glue Data Catalog.</p>
    #[serde(rename = "ConnectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    /// <p>A database name in the AWS Glue Data Catalog.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A table name in the AWS Glue Data Catalog.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>A classifier that uses <code>grok</code> patterns.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GrokClassifier {
    /// <p>An identifier of the data format that the classifier matches, such as Twitter, JSON, Omniture logs, and so on.</p>
    #[serde(rename = "Classification")]
    pub classification: String,
    /// <p>The time that this classifier was registered.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Optional custom grok patterns defined by this classifier. For more information, see custom patterns in <a href="http://docs.aws.amazon.com/glue/latest/dg/custom-classifier.html">Writing Custom Classifiers</a>.</p>
    #[serde(rename = "CustomPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_patterns: Option<String>,
    /// <p>The grok pattern applied to a data store by this classifier. For more information, see built-in patterns in <a href="http://docs.aws.amazon.com/glue/latest/dg/custom-classifier.html">Writing Custom Classifiers</a>.</p>
    #[serde(rename = "GrokPattern")]
    pub grok_pattern: String,
    /// <p>The time that this classifier was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The version of this classifier.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportCatalogToGlueRequest {
    /// <p>The ID of the catalog to import. Currently, this should be the AWS account ID.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportCatalogToGlueResponse {}

/// <p>Specifies configuration properties for an importing labels task run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportLabelsTaskRunProperties {
    /// <p>The Amazon Simple Storage Service (Amazon S3) path from where you will import the labels.</p>
    #[serde(rename = "InputS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_s3_path: Option<String>,
    /// <p>Indicates whether to overwrite your existing labels.</p>
    #[serde(rename = "Replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
}

/// <p>Specifies a JDBC data store to crawl.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JdbcTarget {
    /// <p>The name of the connection to use to connect to the JDBC target.</p>
    #[serde(rename = "ConnectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    /// <p>A list of glob patterns used to exclude from the crawl. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/add-crawler.html">Catalog Tables with a Crawler</a>.</p>
    #[serde(rename = "Exclusions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    /// <p>The path of the JDBC target.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>Specifies a job definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Job {
    /// <p>The <code>JobCommand</code> that executes this job.</p>
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<JobCommand>,
    /// <p>The connections used for this job.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,
    /// <p>The time and date that this job definition was created.</p>
    #[serde(rename = "CreatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    /// <p>The default arguments for this job, specified as name-value pairs.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own Job arguments, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "DefaultArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>A description of the job.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An <code>ExecutionProperty</code> specifying the maximum number of concurrent runs allowed for this job.</p>
    #[serde(rename = "ExecutionProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    /// <p>Glue version determines the versions of Apache Spark and Python that AWS Glue supports. The Python version indicates the version supported for jobs of type Spark. </p> <p>For more information about the available AWS Glue versions and corresponding Spark and Python versions, see <a href="https://docs.aws.amazon.com/glue/latest/dg/add-job.html">Glue version</a> in the developer guide.</p> <p>Jobs that are created without specifying a Glue version default to Glue 0.9.</p>
    #[serde(rename = "GlueVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    /// <p>The last point in time when this job definition was modified.</p>
    #[serde(rename = "LastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>This field is reserved for future use.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p><p>The number of AWS Glue data processing units (DPUs) that can be allocated when this job runs. A DPU is a relative measure of processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more information, see the <a href="https://aws.amazon.com/glue/pricing/">AWS Glue pricing page</a>.</p> <p>Do not set <code>Max Capacity</code> if using <code>WorkerType</code> and <code>NumberOfWorkers</code>.</p> <p>The value that can be allocated for <code>MaxCapacity</code> depends on whether you are running a Python shell job or an Apache Spark ETL job:</p> <ul> <li> <p>When you specify a Python shell job (<code>JobCommand.Name</code>=&quot;pythonshell&quot;), you can allocate either 0.0625 or 1 DPU. The default is 0.0625 DPU.</p> </li> <li> <p>When you specify an Apache Spark ETL job (<code>JobCommand.Name</code>=&quot;glueetl&quot;), you can allocate from 2 to 100 DPUs. The default is 10 DPUs. This job type cannot have a fractional DPU allocation.</p> </li> </ul></p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>The maximum number of times to retry this job after a JobRun fails.</p>
    #[serde(rename = "MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// <p>The name you assign to this job definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies configuration properties of a job notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated when a job runs.</p> <p>The maximum number of workers you can define are 299 for <code>G.1X</code>, and 149 for <code>G.2X</code>. </p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>The name or Amazon Resource Name (ARN) of the IAM role associated with this job.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used with this job.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The job timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p><p>The type of predefined worker that is allocated when a job runs. Accepts a value of Standard, G.1X, or G.2X.</p> <ul> <li> <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p> </li> <li> <p>For the <code>G.1X</code> worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p> </li> <li> <p>For the <code>G.2X</code> worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p> </li> </ul></p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

/// <p>Defines a point that a job can resume processing.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobBookmarkEntry {
    /// <p>The attempt ID number.</p>
    #[serde(rename = "Attempt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i64>,
    /// <p>The bookmark itself.</p>
    #[serde(rename = "JobBookmark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmark: Option<String>,
    /// <p>The name of the job in question.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The unique run identifier associated with the previous job run.</p>
    #[serde(rename = "PreviousRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_run_id: Option<String>,
    /// <p>The run ID number.</p>
    #[serde(rename = "Run")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<i64>,
    /// <p>The run ID number.</p>
    #[serde(rename = "RunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// <p>The version of the job.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies how job bookmark data should be encrypted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobBookmarksEncryption {
    /// <p>The encryption mode to use for job bookmarks data.</p>
    #[serde(rename = "JobBookmarksEncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmarks_encryption_mode: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

/// <p>Specifies code executed when a job is run.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobCommand {
    /// <p>The name of the job command. For an Apache Spark ETL job, this must be <code>glueetl</code>. For a Python shell job, it must be <code>pythonshell</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Python version being used to execute a Python shell job. Allowed values are 2 or 3.</p>
    #[serde(rename = "PythonVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_version: Option<String>,
    /// <p>Specifies the Amazon Simple Storage Service (Amazon S3) path to a script that executes a job.</p>
    #[serde(rename = "ScriptLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_location: Option<String>,
}

/// <p>The details of a Job node present in the workflow.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobNodeDetails {
    /// <p>The information for the job runs represented by the job node.</p>
    #[serde(rename = "JobRuns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_runs: Option<Vec<JobRun>>,
}

/// <p>Contains information about a job run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobRun {
    /// <p>The job arguments associated with this run. For this job run, they replace the default arguments set in the job definition itself.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own job arguments, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "Arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>The number of the attempt to run this job.</p>
    #[serde(rename = "Attempt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i64>,
    /// <p>The date and time that this job run completed.</p>
    #[serde(rename = "CompletedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    /// <p>An error message associated with this job run.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The amount of time (in seconds) that the job run consumed resources.</p>
    #[serde(rename = "ExecutionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i64>,
    /// <p>Glue version determines the versions of Apache Spark and Python that AWS Glue supports. The Python version indicates the version supported for jobs of type Spark. </p> <p>For more information about the available AWS Glue versions and corresponding Spark and Python versions, see <a href="https://docs.aws.amazon.com/glue/latest/dg/add-job.html">Glue version</a> in the developer guide.</p> <p>Jobs that are created without specifying a Glue version default to Glue 0.9.</p>
    #[serde(rename = "GlueVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    /// <p>The ID of this job run.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the job definition being used in this run.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The current state of the job run.</p>
    #[serde(rename = "JobRunState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_state: Option<String>,
    /// <p>The last time that this job run was modified.</p>
    #[serde(rename = "LastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>The name of the log group for secure logging that can be server-side encrypted in Amazon CloudWatch using AWS KMS. This name can be <code>/aws-glue/jobs/</code>, in which case the default encryption is <code>NONE</code>. If you add a role name and <code>SecurityConfiguration</code> name (in other words, <code>/aws-glue/jobs-yourRoleName-yourSecurityConfigurationName/</code>), then that security configuration is used to encrypt the log group.</p>
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p><p>The number of AWS Glue data processing units (DPUs) that can be allocated when this job runs. A DPU is a relative measure of processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more information, see the <a href="https://docs.aws.amazon.com/https:/aws.amazon.com/glue/pricing/">AWS Glue pricing page</a>.</p> <p>Do not set <code>Max Capacity</code> if using <code>WorkerType</code> and <code>NumberOfWorkers</code>.</p> <p>The value that can be allocated for <code>MaxCapacity</code> depends on whether you are running a Python shell job or an Apache Spark ETL job:</p> <ul> <li> <p>When you specify a Python shell job (<code>JobCommand.Name</code>=&quot;pythonshell&quot;), you can allocate either 0.0625 or 1 DPU. The default is 0.0625 DPU.</p> </li> <li> <p>When you specify an Apache Spark ETL job (<code>JobCommand.Name</code>=&quot;glueetl&quot;), you can allocate from 2 to 100 DPUs. The default is 10 DPUs. This job type cannot have a fractional DPU allocation.</p> </li> </ul></p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>Specifies configuration properties of a job run notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated when a job runs.</p> <p>The maximum number of workers you can define are 299 for <code>G.1X</code>, and 149 for <code>G.2X</code>. </p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>A list of predecessors to this job run.</p>
    #[serde(rename = "PredecessorRuns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predecessor_runs: Option<Vec<Predecessor>>,
    /// <p>The ID of the previous run of this job. For example, the <code>JobRunId</code> specified in the <code>StartJobRun</code> action.</p>
    #[serde(rename = "PreviousRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_run_id: Option<String>,
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used with this job run.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The date and time at which this job run was started.</p>
    #[serde(rename = "StartedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    /// <p>The <code>JobRun</code> timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours). This overrides the timeout value set in the parent job.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The name of the trigger that started this job run.</p>
    #[serde(rename = "TriggerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
    /// <p><p>The type of predefined worker that is allocated when a job runs. Accepts a value of Standard, G.1X, or G.2X.</p> <ul> <li> <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p> </li> <li> <p>For the <code>G.1X</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 64GB disk, and 1 executor per worker.</p> </li> <li> <p>For the <code>G.2X</code> worker type, each worker provides 8 vCPU, 32 GB of memory and a 128GB disk, and 1 executor per worker.</p> </li> </ul></p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

/// <p>Specifies information used to update an existing job definition. The previous job definition is completely overwritten by this information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct JobUpdate {
    /// <p>The <code>JobCommand</code> that executes this job (required).</p>
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<JobCommand>,
    /// <p>The connections used for this job.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,
    /// <p>The default arguments for this job.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own Job arguments, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "DefaultArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>Description of the job being defined.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An <code>ExecutionProperty</code> specifying the maximum number of concurrent runs allowed for this job.</p>
    #[serde(rename = "ExecutionProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    /// <p>Glue version determines the versions of Apache Spark and Python that AWS Glue supports. The Python version indicates the version supported for jobs of type Spark. </p> <p>For more information about the available AWS Glue versions and corresponding Spark and Python versions, see <a href="https://docs.aws.amazon.com/glue/latest/dg/add-job.html">Glue version</a> in the developer guide.</p>
    #[serde(rename = "GlueVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<String>,
    /// <p>This field is reserved for future use.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p><p>The number of AWS Glue data processing units (DPUs) that can be allocated when this job runs. A DPU is a relative measure of processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more information, see the <a href="https://aws.amazon.com/glue/pricing/">AWS Glue pricing page</a>.</p> <p>Do not set <code>Max Capacity</code> if using <code>WorkerType</code> and <code>NumberOfWorkers</code>.</p> <p>The value that can be allocated for <code>MaxCapacity</code> depends on whether you are running a Python shell job or an Apache Spark ETL job:</p> <ul> <li> <p>When you specify a Python shell job (<code>JobCommand.Name</code>=&quot;pythonshell&quot;), you can allocate either 0.0625 or 1 DPU. The default is 0.0625 DPU.</p> </li> <li> <p>When you specify an Apache Spark ETL job (<code>JobCommand.Name</code>=&quot;glueetl&quot;), you can allocate from 2 to 100 DPUs. The default is 10 DPUs. This job type cannot have a fractional DPU allocation.</p> </li> </ul></p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>The maximum number of times to retry this job if it fails.</p>
    #[serde(rename = "MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// <p>Specifies the configuration properties of a job notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated when a job runs.</p> <p>The maximum number of workers you can define are 299 for <code>G.1X</code>, and 149 for <code>G.2X</code>. </p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>The name or Amazon Resource Name (ARN) of the IAM role associated with this job (required).</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used with this job.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The job timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p><p>The type of predefined worker that is allocated when a job runs. Accepts a value of Standard, G.1X, or G.2X.</p> <ul> <li> <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p> </li> <li> <p>For the <code>G.1X</code> worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p> </li> <li> <p>For the <code>G.2X</code> worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p> </li> </ul></p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

/// <p>A classifier for <code>JSON</code> content.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JsonClassifier {
    /// <p>The time that this classifier was registered.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A <code>JsonPath</code> string defining the JSON data for the classifier to classify. AWS Glue supports a subset of <code>JsonPath</code>, as described in <a href="https://docs.aws.amazon.com/glue/latest/dg/custom-classifier.html#custom-classifier-json">Writing JsonPath Custom Classifiers</a>.</p>
    #[serde(rename = "JsonPath")]
    pub json_path: String,
    /// <p>The time that this classifier was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The version of this classifier.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies configuration properties for a labeling set generation task run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LabelingSetGenerationTaskRunProperties {
    /// <p>The Amazon Simple Storage Service (Amazon S3) path where you will generate the labeling set.</p>
    #[serde(rename = "OutputS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_path: Option<String>,
}

/// <p>Status and error information about the most recent crawl.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LastCrawlInfo {
    /// <p>If an error occurred, the error information about the last crawl.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The log group for the last crawl.</p>
    #[serde(rename = "LogGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    /// <p>The log stream for the last crawl.</p>
    #[serde(rename = "LogStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream: Option<String>,
    /// <p>The prefix for a message about this crawl.</p>
    #[serde(rename = "MessagePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_prefix: Option<String>,
    /// <p>The time at which the crawl started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>Status of the last crawl.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCrawlersRequest {
    /// <p>The maximum size of a list to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies to return only these tagged resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCrawlersResponse {
    /// <p>The names of all crawlers in the account, or the crawlers with the specified tags.</p>
    #[serde(rename = "CrawlerNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_names: Option<Vec<String>>,
    /// <p>A continuation token, if the returned list does not contain the last metric available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDevEndpointsRequest {
    /// <p>The maximum size of a list to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies to return only these tagged resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDevEndpointsResponse {
    /// <p>The names of all the <code>DevEndpoint</code>s in the account, or the <code>DevEndpoint</code>s with the specified tags.</p>
    #[serde(rename = "DevEndpointNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoint_names: Option<Vec<String>>,
    /// <p>A continuation token, if the returned list does not contain the last metric available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListJobsRequest {
    /// <p>The maximum size of a list to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies to return only these tagged resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJobsResponse {
    /// <p>The names of all jobs in the account, or the jobs with the specified tags.</p>
    #[serde(rename = "JobNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_names: Option<Vec<String>>,
    /// <p>A continuation token, if the returned list does not contain the last metric available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTriggersRequest {
    /// <p> The name of the job for which to retrieve triggers. The trigger that can start this job is returned. If there is no such trigger, all triggers are returned.</p>
    #[serde(rename = "DependentJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_job_name: Option<String>,
    /// <p>The maximum size of a list to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies to return only these tagged resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTriggersResponse {
    /// <p>A continuation token, if the returned list does not contain the last metric available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The names of all triggers in the account, or the triggers with the specified tags.</p>
    #[serde(rename = "TriggerNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListWorkflowsRequest {
    /// <p>The maximum size of a list to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWorkflowsResponse {
    /// <p>A continuation token, if not all workflow names have been returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of names of workflows in the account.</p>
    #[serde(rename = "Workflows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<String>>,
}

/// <p>The location of resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Location {
    /// <p>An Amazon DynamoDB table location.</p>
    #[serde(rename = "DynamoDB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db: Option<Vec<CodeGenNodeArg>>,
    /// <p>A JDBC location.</p>
    #[serde(rename = "Jdbc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jdbc: Option<Vec<CodeGenNodeArg>>,
    /// <p>An Amazon Simple Storage Service (Amazon S3) location.</p>
    #[serde(rename = "S3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<Vec<CodeGenNodeArg>>,
}

/// <p>A structure for a machine learning transform.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MLTransform {
    /// <p>A timestamp. The time and date that this machine learning transform was created.</p>
    #[serde(rename = "CreatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    /// <p>A user-defined, long-form description text for the machine learning transform. Descriptions are not guaranteed to be unique and can be changed at any time.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An <code>EvaluationMetrics</code> object. Evaluation metrics provide an estimate of the quality of your machine learning transform.</p>
    #[serde(rename = "EvaluationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_metrics: Option<EvaluationMetrics>,
    /// <p>A list of AWS Glue table definitions used by the transform.</p>
    #[serde(rename = "InputRecordTables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_record_tables: Option<Vec<GlueTable>>,
    /// <p>A count identifier for the labeling files generated by AWS Glue for this transform. As you create a better transform, you can iteratively download, label, and upload the labeling file.</p>
    #[serde(rename = "LabelCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_count: Option<i64>,
    /// <p>A timestamp. The last point in time when this machine learning transform was modified.</p>
    #[serde(rename = "LastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>The number of AWS Glue data processing units (DPUs) that are allocated to task runs for this transform. You can allocate from 2 to 100 DPUs; the default is 10. A DPU is a relative measure of processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more information, see the <a href="https://aws.amazon.com/glue/pricing/">AWS Glue pricing page</a>. </p> <p>When the <code>WorkerType</code> field is set to a value other than <code>Standard</code>, the <code>MaxCapacity</code> field is set automatically and becomes read-only.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>The maximum number of times to retry after an <code>MLTaskRun</code> of the machine learning transform fails.</p>
    #[serde(rename = "MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// <p>A user-defined name for the machine learning transform. Names are not guaranteed unique and can be changed at any time.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated when a task of the transform runs.</p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>A <code>TransformParameters</code> object. You can use parameters to tune (customize) the behavior of the machine learning transform by specifying what data it learns from and your preference on various tradeoffs (such as precious vs. recall, or accuracy vs. cost).</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<TransformParameters>,
    /// <p>The name or Amazon Resource Name (ARN) of the IAM role with the required permissions. This role needs permission to your Amazon Simple Storage Service (Amazon S3) sources, targets, temporary directory, scripts, and any libraries used by the task run for this transform.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>A map of key-value pairs representing the columns and data types that this transform can run against. Has an upper bound of 100 columns.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Vec<SchemaColumn>>,
    /// <p>The current status of the machine learning transform.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The timeout in minutes of the machine learning transform.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The unique transform ID that is generated for the machine learning transform. The ID is guaranteed to be unique and does not change.</p>
    #[serde(rename = "TransformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
    /// <p><p>The type of predefined worker that is allocated when a task of this transform runs. Accepts a value of Standard, G.1X, or G.2X.</p> <ul> <li> <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p> </li> <li> <p>For the <code>G.1X</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 64GB disk, and 1 executor per worker.</p> </li> <li> <p>For the <code>G.2X</code> worker type, each worker provides 8 vCPU, 32 GB of memory and a 128GB disk, and 1 executor per worker.</p> </li> </ul></p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

/// <p>Defines a mapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MappingEntry {
    /// <p>The source path.</p>
    #[serde(rename = "SourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    /// <p>The name of the source table.</p>
    #[serde(rename = "SourceTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table: Option<String>,
    /// <p>The source type.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The target path.</p>
    #[serde(rename = "TargetPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    /// <p>The target table.</p>
    #[serde(rename = "TargetTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table: Option<String>,
    /// <p>The target type.</p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

/// <p>A node represents an AWS Glue component like Trigger, Job etc. which is part of a workflow.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Node {
    /// <p>Details of the crawler when the node represents a crawler.</p>
    #[serde(rename = "CrawlerDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_details: Option<CrawlerNodeDetails>,
    /// <p>Details of the Job when the node represents a Job.</p>
    #[serde(rename = "JobDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<JobNodeDetails>,
    /// <p>The name of the AWS Glue component represented by the node.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Details of the Trigger when the node represents a Trigger.</p>
    #[serde(rename = "TriggerDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_details: Option<TriggerNodeDetails>,
    /// <p>The type of AWS Glue component represented by the node.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The unique Id assigned to the node within the workflow.</p>
    #[serde(rename = "UniqueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

/// <p>Specifies configuration properties of a notification.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationProperty {
    /// <p>After a job run starts, the number of minutes to wait before sending a job run delay notification.</p>
    #[serde(rename = "NotifyDelayAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_delay_after: Option<i64>,
}

/// <p>Specifies the sort order of a sorted column.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// <p>The name of the column.</p>
    #[serde(rename = "Column")]
    pub column: String,
    /// <p>Indicates that the column is sorted in ascending order (<code>== 1</code>), or in descending order (<code>==0</code>).</p>
    #[serde(rename = "SortOrder")]
    pub sort_order: i64,
}

/// <p>Represents a slice of table data.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Partition {
    /// <p>The time at which the partition was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the catalog database in which to create the partition.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The last time at which the partition was accessed.</p>
    #[serde(rename = "LastAccessTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    /// <p>The last time at which column statistics were computed for this partition.</p>
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    /// <p>These key-value pairs define partition parameters.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Provides information about the physical location where the partition is stored.</p>
    #[serde(rename = "StorageDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    /// <p>The name of the database table in which to create the partition.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>The values of the partition.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Contains information about a partition error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PartitionError {
    /// <p>The details about the partition error.</p>
    #[serde(rename = "ErrorDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    /// <p>The values that define the partition.</p>
    #[serde(rename = "PartitionValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_values: Option<Vec<String>>,
}

/// <p>The structure used to create and update a partition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PartitionInput {
    /// <p>The last time at which the partition was accessed.</p>
    #[serde(rename = "LastAccessTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    /// <p>The last time at which column statistics were computed for this partition.</p>
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    /// <p>These key-value pairs define partition parameters.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Provides information about the physical location where the partition is stored.</p>
    #[serde(rename = "StorageDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    /// <p>The values of the partition. Although this parameter is not required by the SDK, you must specify this parameter for a valid input.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Contains a list of values defining partitions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartitionValueList {
    /// <p>The list of values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Specifies the physical requirements for a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalConnectionRequirements {
    /// <p>The connection's Availability Zone. This field is redundant because the specified subnet implies the Availability Zone to be used. Currently the field must be populated, but it will be deprecated in the future.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The security group ID list used by the connection.</p>
    #[serde(rename = "SecurityGroupIdList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id_list: Option<Vec<String>>,
    /// <p>The subnet ID used by the connection.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

/// <p>A job run that was used in the predicate of a conditional trigger that triggered this job run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Predecessor {
    /// <p>The name of the job definition used by the predecessor job run.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The job-run ID of the predecessor job run.</p>
    #[serde(rename = "RunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

/// <p>Defines the predicate of the trigger, which determines when it fires.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Predicate {
    /// <p>A list of the conditions that determine when the trigger will fire.</p>
    #[serde(rename = "Conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// <p>An optional field if only one condition is listed. If multiple conditions are listed, then this field is required.</p>
    #[serde(rename = "Logical")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical: Option<String>,
}

/// <p>Permissions granted to a principal.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrincipalPermissions {
    /// <p>The permissions that are granted to the principal.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// <p>The principal who is granted permissions.</p>
    #[serde(rename = "Principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
}

/// <p>Defines a property predicate.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PropertyPredicate {
    /// <p>The comparator used to compare this property to others.</p>
    #[serde(rename = "Comparator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<String>,
    /// <p>The key of the property.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value of the property.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutDataCatalogEncryptionSettingsRequest {
    /// <p>The ID of the Data Catalog to set the security configuration for. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The security configuration to set.</p>
    #[serde(rename = "DataCatalogEncryptionSettings")]
    pub data_catalog_encryption_settings: DataCatalogEncryptionSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutDataCatalogEncryptionSettingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutResourcePolicyRequest {
    /// <p>A value of <code>MUST_EXIST</code> is used to update a policy. A value of <code>NOT_EXIST</code> is used to create a new policy. If a value of <code>NONE</code> or a null value is used, the call will not depend on the existence of a policy.</p>
    #[serde(rename = "PolicyExistsCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_exists_condition: Option<String>,
    /// <p>The hash value returned when the previous policy was set using <code>PutResourcePolicy</code>. Its purpose is to prevent concurrent modifications of a policy. Do not use this parameter if no previous policy has been set.</p>
    #[serde(rename = "PolicyHashCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash_condition: Option<String>,
    /// <p>Contains the policy document to set, in JSON format.</p>
    #[serde(rename = "PolicyInJson")]
    pub policy_in_json: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutResourcePolicyResponse {
    /// <p>A hash of the policy that has just been set. This must be included in a subsequent call that overwrites or updates this policy.</p>
    #[serde(rename = "PolicyHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutWorkflowRunPropertiesRequest {
    /// <p>Name of the workflow which was run.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The ID of the workflow run for which the run properties should be updated.</p>
    #[serde(rename = "RunId")]
    pub run_id: String,
    /// <p>The properties to put for the specified run.</p>
    #[serde(rename = "RunProperties")]
    pub run_properties: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutWorkflowRunPropertiesResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResetJobBookmarkRequest {
    /// <p>The name of the job in question.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>The unique run identifier associated with this job run.</p>
    #[serde(rename = "RunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResetJobBookmarkResponse {
    /// <p>The reset bookmark entry.</p>
    #[serde(rename = "JobBookmarkEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmark_entry: Option<JobBookmarkEntry>,
}

/// <p>The URIs for function resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceUri {
    /// <p>The type of the resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The URI for accessing the resource.</p>
    #[serde(rename = "Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// <p>Specifies how Amazon Simple Storage Service (Amazon S3) data should be encrypted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Encryption {
    /// <p>The Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The encryption mode to use for Amazon S3 data.</p>
    #[serde(rename = "S3EncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption_mode: Option<String>,
}

/// <p>Specifies a data store in Amazon Simple Storage Service (Amazon S3).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Target {
    /// <p>A list of glob patterns used to exclude from the crawl. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/add-crawler.html">Catalog Tables with a Crawler</a>.</p>
    #[serde(rename = "Exclusions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    /// <p>The path to the Amazon S3 target.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>A scheduling object using a <code>cron</code> statement to schedule an event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Schedule {
    /// <p>A <code>cron</code> expression used to specify the schedule. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, specify <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The state of the schedule.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>A policy that specifies update and deletion behaviors for the crawler.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaChangePolicy {
    /// <p>The deletion behavior when the crawler finds a deleted object.</p>
    #[serde(rename = "DeleteBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_behavior: Option<String>,
    /// <p>The update behavior when the crawler finds a changed schema.</p>
    #[serde(rename = "UpdateBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_behavior: Option<String>,
}

/// <p>A key-value pair representing a column and data type that this transform can run against. The <code>Schema</code> parameter of the <code>MLTransform</code> may contain up to 100 of these structures.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaColumn {
    /// <p>The type of data in the column.</p>
    #[serde(rename = "DataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// <p>The name of the column.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchTablesRequest {
    /// <p>A unique identifier, consisting of <code> <i>account_id</i>/datalake</code>.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A list of key-value pairs, and a comparator used to filter the search results. Returns all entities matching the predicate.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PropertyPredicate>>,
    /// <p>The maximum number of tables to return in a single response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, included if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A string used for a text search.</p> <p>Specifying a value in quotes filters based on an exact match to the value.</p>
    #[serde(rename = "SearchText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_text: Option<String>,
    /// <p>A list of criteria for sorting the results by a field name, in an ascending or descending order.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<SortCriterion>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchTablesResponse {
    /// <p>A continuation token, present if the current list segment is not the last.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the requested <code>Table</code> objects. The <code>SearchTables</code> response returns only the tables that you have access to.</p>
    #[serde(rename = "TableList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_list: Option<Vec<Table>>,
}

/// <p>Specifies a security configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityConfiguration {
    /// <p>The time at which this security configuration was created.</p>
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    /// <p>The encryption configuration associated with this security configuration.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The name of the security configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Defines a non-overlapping region of a table's partitions, allowing multiple requests to be executed in parallel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Segment {
    /// <p>The zero-based index number of the segment. For example, if the total number of segments is 4, <code>SegmentNumber</code> values range from 0 through 3.</p>
    #[serde(rename = "SegmentNumber")]
    pub segment_number: i64,
    /// <p>The total number of segments.</p>
    #[serde(rename = "TotalSegments")]
    pub total_segments: i64,
}

/// <p>Information about a serialization/deserialization program (SerDe) that serves as an extractor and loader.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerDeInfo {
    /// <p>Name of the SerDe.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>These key-value pairs define initialization parameters for the SerDe.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Usually the class that implements the SerDe. An example is <code>org.apache.hadoop.hive.serde2.columnar.ColumnarSerDe</code>.</p>
    #[serde(rename = "SerializationLibrary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization_library: Option<String>,
}

/// <p>Specifies skewed values in a table. Skewed values are those that occur with very high frequency.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkewedInfo {
    /// <p>A list of names of columns that contain skewed values.</p>
    #[serde(rename = "SkewedColumnNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_names: Option<Vec<String>>,
    /// <p>A mapping of skewed values to the columns that contain them.</p>
    #[serde(rename = "SkewedColumnValueLocationMaps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_value_location_maps: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of values that appear so frequently as to be considered skewed.</p>
    #[serde(rename = "SkewedColumnValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SortCriterion {
    #[serde(rename = "FieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartCrawlerRequest {
    /// <p>Name of the crawler to start.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartCrawlerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartCrawlerScheduleRequest {
    /// <p>Name of the crawler to schedule.</p>
    #[serde(rename = "CrawlerName")]
    pub crawler_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartCrawlerScheduleResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartExportLabelsTaskRunRequest {
    /// <p>The Amazon S3 path where you export the labels.</p>
    #[serde(rename = "OutputS3Path")]
    pub output_s3_path: String,
    /// <p>The unique identifier of the machine learning transform.</p>
    #[serde(rename = "TransformId")]
    pub transform_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartExportLabelsTaskRunResponse {
    /// <p>The unique identifier for the task run.</p>
    #[serde(rename = "TaskRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartImportLabelsTaskRunRequest {
    /// <p>The Amazon Simple Storage Service (Amazon S3) path from where you import the labels.</p>
    #[serde(rename = "InputS3Path")]
    pub input_s3_path: String,
    /// <p>Indicates whether to overwrite your existing labels.</p>
    #[serde(rename = "ReplaceAllLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_all_labels: Option<bool>,
    /// <p>The unique identifier of the machine learning transform.</p>
    #[serde(rename = "TransformId")]
    pub transform_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartImportLabelsTaskRunResponse {
    /// <p>The unique identifier for the task run.</p>
    #[serde(rename = "TaskRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartJobRunRequest {
    /// <p>The job arguments specifically for this run. For this job run, they replace the default arguments set in the job definition itself.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own Job arguments, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "Arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the job definition to use.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>The ID of a previous <code>JobRun</code> to retry.</p>
    #[serde(rename = "JobRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
    /// <p><p>The number of AWS Glue data processing units (DPUs) that can be allocated when this job runs. A DPU is a relative measure of processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more information, see the <a href="https://docs.aws.amazon.com/https:/aws.amazon.com/glue/pricing/">AWS Glue pricing page</a>.</p> <p>Do not set <code>Max Capacity</code> if using <code>WorkerType</code> and <code>NumberOfWorkers</code>.</p> <p>The value that can be allocated for <code>MaxCapacity</code> depends on whether you are running a Python shell job, or an Apache Spark ETL job:</p> <ul> <li> <p>When you specify a Python shell job (<code>JobCommand.Name</code>=&quot;pythonshell&quot;), you can allocate either 0.0625 or 1 DPU. The default is 0.0625 DPU.</p> </li> <li> <p>When you specify an Apache Spark ETL job (<code>JobCommand.Name</code>=&quot;glueetl&quot;), you can allocate from 2 to 100 DPUs. The default is 10 DPUs. This job type cannot have a fractional DPU allocation.</p> </li> </ul></p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>Specifies configuration properties of a job run notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated when a job runs.</p> <p>The maximum number of workers you can define are 299 for <code>G.1X</code>, and 149 for <code>G.2X</code>. </p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used with this job run.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The <code>JobRun</code> timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours). This overrides the timeout value set in the parent job.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p><p>The type of predefined worker that is allocated when a job runs. Accepts a value of Standard, G.1X, or G.2X.</p> <ul> <li> <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p> </li> <li> <p>For the <code>G.1X</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 64GB disk, and 1 executor per worker.</p> </li> <li> <p>For the <code>G.2X</code> worker type, each worker provides 8 vCPU, 32 GB of memory and a 128GB disk, and 1 executor per worker.</p> </li> </ul></p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartJobRunResponse {
    /// <p>The ID assigned to this job run.</p>
    #[serde(rename = "JobRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartMLEvaluationTaskRunRequest {
    /// <p>The unique identifier of the machine learning transform.</p>
    #[serde(rename = "TransformId")]
    pub transform_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartMLEvaluationTaskRunResponse {
    /// <p>The unique identifier associated with this run.</p>
    #[serde(rename = "TaskRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartMLLabelingSetGenerationTaskRunRequest {
    /// <p>The Amazon Simple Storage Service (Amazon S3) path where you generate the labeling set.</p>
    #[serde(rename = "OutputS3Path")]
    pub output_s3_path: String,
    /// <p>The unique identifier of the machine learning transform.</p>
    #[serde(rename = "TransformId")]
    pub transform_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartMLLabelingSetGenerationTaskRunResponse {
    /// <p>The unique run identifier that is associated with this task run.</p>
    #[serde(rename = "TaskRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartTriggerRequest {
    /// <p>The name of the trigger to start.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartTriggerResponse {
    /// <p>The name of the trigger that was started.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartWorkflowRunRequest {
    /// <p>The name of the workflow to start.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartWorkflowRunResponse {
    /// <p>An Id for the new run.</p>
    #[serde(rename = "RunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopCrawlerRequest {
    /// <p>Name of the crawler to stop.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopCrawlerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopCrawlerScheduleRequest {
    /// <p>Name of the crawler whose schedule state to set.</p>
    #[serde(rename = "CrawlerName")]
    pub crawler_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopCrawlerScheduleResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopTriggerRequest {
    /// <p>The name of the trigger to stop.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopTriggerResponse {
    /// <p>The name of the trigger that was stopped.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Describes the physical storage of table data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageDescriptor {
    /// <p>A list of reducer grouping columns, clustering columns, and bucketing columns in the table.</p>
    #[serde(rename = "BucketColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_columns: Option<Vec<String>>,
    /// <p>A list of the <code>Columns</code> in the table.</p>
    #[serde(rename = "Columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>,
    /// <p> <code>True</code> if the data in the table is compressed, or <code>False</code> if not.</p>
    #[serde(rename = "Compressed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compressed: Option<bool>,
    /// <p>The input format: <code>SequenceFileInputFormat</code> (binary), or <code>TextInputFormat</code>, or a custom format.</p>
    #[serde(rename = "InputFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format: Option<String>,
    /// <p>The physical location of the table. By default, this takes the form of the warehouse location, followed by the database location in the warehouse, followed by the table name.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>Must be specified if the table contains any dimension columns.</p>
    #[serde(rename = "NumberOfBuckets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_buckets: Option<i64>,
    /// <p>The output format: <code>SequenceFileOutputFormat</code> (binary), or <code>IgnoreKeyTextOutputFormat</code>, or a custom format.</p>
    #[serde(rename = "OutputFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// <p>The user-supplied properties in key-value form.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The serialization/deserialization (SerDe) information.</p>
    #[serde(rename = "SerdeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serde_info: Option<SerDeInfo>,
    /// <p>The information about values that appear frequently in a column (skewed values).</p>
    #[serde(rename = "SkewedInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_info: Option<SkewedInfo>,
    /// <p>A list specifying the sort order of each bucket in the table.</p>
    #[serde(rename = "SortColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_columns: Option<Vec<Order>>,
    /// <p> <code>True</code> if the table data is stored in subdirectories, or <code>False</code> if not.</p>
    #[serde(rename = "StoredAsSubDirectories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_as_sub_directories: Option<bool>,
}

/// <p>Represents a collection of related data organized in columns and rows.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Table {
    /// <p>The time when the table definition was created in the Data Catalog.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>The person or entity who created the table.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The name of the database where the table metadata resides. For Hive compatibility, this must be all lowercase.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>A description of the table.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Indicates whether the table has been registered with AWS Lake Formation.</p>
    #[serde(rename = "IsRegisteredWithLakeFormation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_registered_with_lake_formation: Option<bool>,
    /// <p>The last time that the table was accessed. This is usually taken from HDFS, and might not be reliable.</p>
    #[serde(rename = "LastAccessTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    /// <p>The last time that column statistics were computed for this table.</p>
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    /// <p>The table name. For Hive compatibility, this must be entirely lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The owner of the table.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>These key-value pairs define properties associated with the table.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of columns by which the table is partitioned. Only primitive types are supported as partition keys.</p> <p>When you create a table used by Amazon Athena, and you do not specify any <code>partitionKeys</code>, you must at least set the value of <code>partitionKeys</code> to an empty list. For example:</p> <p> <code>"PartitionKeys": []</code> </p>
    #[serde(rename = "PartitionKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Column>>,
    /// <p>The retention time for this table.</p>
    #[serde(rename = "Retention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<i64>,
    /// <p>A storage descriptor containing information about the physical storage of this table.</p>
    #[serde(rename = "StorageDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    /// <p>The type of this table (<code>EXTERNAL_TABLE</code>, <code>VIRTUAL_VIEW</code>, etc.).</p>
    #[serde(rename = "TableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_type: Option<String>,
    /// <p>The last time that the table was updated.</p>
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
    /// <p>If the table is a view, the expanded text of the view; otherwise <code>null</code>.</p>
    #[serde(rename = "ViewExpandedText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_expanded_text: Option<String>,
    /// <p>If the table is a view, the original text of the view; otherwise <code>null</code>.</p>
    #[serde(rename = "ViewOriginalText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_original_text: Option<String>,
}

/// <p>An error record for table operations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TableError {
    /// <p>The details about the error.</p>
    #[serde(rename = "ErrorDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    /// <p>The name of the table. For Hive compatibility, this must be entirely lowercase.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

/// <p>A structure used to define a table.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TableInput {
    /// <p>A description of the table.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The last time that the table was accessed.</p>
    #[serde(rename = "LastAccessTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    /// <p>The last time that column statistics were computed for this table.</p>
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    /// <p>The table name. For Hive compatibility, this is folded to lowercase when it is stored.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The table owner.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>These key-value pairs define properties associated with the table.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of columns by which the table is partitioned. Only primitive types are supported as partition keys.</p> <p>When you create a table used by Amazon Athena, and you do not specify any <code>partitionKeys</code>, you must at least set the value of <code>partitionKeys</code> to an empty list. For example:</p> <p> <code>"PartitionKeys": []</code> </p>
    #[serde(rename = "PartitionKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Column>>,
    /// <p>The retention time for this table.</p>
    #[serde(rename = "Retention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<i64>,
    /// <p>A storage descriptor containing information about the physical storage of this table.</p>
    #[serde(rename = "StorageDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    /// <p>The type of this table (<code>EXTERNAL_TABLE</code>, <code>VIRTUAL_VIEW</code>, etc.).</p>
    #[serde(rename = "TableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_type: Option<String>,
    /// <p>If the table is a view, the expanded text of the view; otherwise <code>null</code>.</p>
    #[serde(rename = "ViewExpandedText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_expanded_text: Option<String>,
    /// <p>If the table is a view, the original text of the view; otherwise <code>null</code>.</p>
    #[serde(rename = "ViewOriginalText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_original_text: Option<String>,
}

/// <p>Specifies a version of a table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TableVersion {
    /// <p>The table in question.</p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
    /// <p>The ID value that identifies this table version. A <code>VersionId</code> is a string representation of an integer. Each version is incremented by 1.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p>An error record for table-version operations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TableVersionError {
    /// <p>The details about the error.</p>
    #[serde(rename = "ErrorDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    /// <p>The name of the table in question.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>The ID value of the version in question. A <code>VersionID</code> is a string representation of an integer. Each version is incremented by 1.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The ARN of the AWS Glue resource to which to add the tags. For more information about AWS Glue resource ARNs, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-common.html#aws-glue-api-regex-aws-glue-arn-id">AWS Glue ARN string pattern</a>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Tags to add to this resource.</p>
    #[serde(rename = "TagsToAdd")]
    pub tags_to_add: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>The sampling parameters that are associated with the machine learning transform.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskRun {
    /// <p>The last point in time that the requested task run was completed.</p>
    #[serde(rename = "CompletedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    /// <p>The list of error strings associated with this task run.</p>
    #[serde(rename = "ErrorString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    /// <p>The amount of time (in seconds) that the task run consumed resources.</p>
    #[serde(rename = "ExecutionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i64>,
    /// <p>The last point in time that the requested task run was updated.</p>
    #[serde(rename = "LastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>The names of the log group for secure logging, associated with this task run.</p>
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>Specifies configuration properties associated with this task run.</p>
    #[serde(rename = "Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TaskRunProperties>,
    /// <p>The date and time that this task run started.</p>
    #[serde(rename = "StartedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    /// <p>The current status of the requested task run.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The unique identifier for this task run.</p>
    #[serde(rename = "TaskRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_id: Option<String>,
    /// <p>The unique identifier for the transform.</p>
    #[serde(rename = "TransformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

/// <p>The criteria that are used to filter the task runs for the machine learning transform.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TaskRunFilterCriteria {
    /// <p>Filter on task runs started after this date.</p>
    #[serde(rename = "StartedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_after: Option<f64>,
    /// <p>Filter on task runs started before this date.</p>
    #[serde(rename = "StartedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_before: Option<f64>,
    /// <p>The current status of the task run.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of task run.</p>
    #[serde(rename = "TaskRunType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_run_type: Option<String>,
}

/// <p>The configuration properties for the task run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskRunProperties {
    /// <p>The configuration properties for an exporting labels task run.</p>
    #[serde(rename = "ExportLabelsTaskRunProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_labels_task_run_properties: Option<ExportLabelsTaskRunProperties>,
    /// <p>The configuration properties for a find matches task run.</p>
    #[serde(rename = "FindMatchesTaskRunProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_matches_task_run_properties: Option<FindMatchesTaskRunProperties>,
    /// <p>The configuration properties for an importing labels task run.</p>
    #[serde(rename = "ImportLabelsTaskRunProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_labels_task_run_properties: Option<ImportLabelsTaskRunProperties>,
    /// <p>The configuration properties for a labeling set generation task run.</p>
    #[serde(rename = "LabelingSetGenerationTaskRunProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_set_generation_task_run_properties: Option<LabelingSetGenerationTaskRunProperties>,
    /// <p>The type of task run.</p>
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}

/// <p>The sorting criteria that are used to sort the list of task runs for the machine learning transform.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TaskRunSortCriteria {
    /// <p>The column to be used to sort the list of task runs for the machine learning transform.</p>
    #[serde(rename = "Column")]
    pub column: String,
    /// <p>The sort direction to be used to sort the list of task runs for the machine learning transform.</p>
    #[serde(rename = "SortDirection")]
    pub sort_direction: String,
}

/// <p>The criteria used to filter the machine learning transforms.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TransformFilterCriteria {
    /// <p>The time and date after which the transforms were created.</p>
    #[serde(rename = "CreatedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    /// <p>The time and date before which the transforms were created.</p>
    #[serde(rename = "CreatedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    /// <p>Filter on transforms last modified after this date.</p>
    #[serde(rename = "LastModifiedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_after: Option<f64>,
    /// <p>Filter on transforms last modified before this date.</p>
    #[serde(rename = "LastModifiedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_before: Option<f64>,
    /// <p>A unique transform name that is used to filter the machine learning transforms.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Filters on datasets with a specific schema. The <code>Map&lt;Column, Type&gt;</code> object is an array of key-value pairs representing the schema this transform accepts, where <code>Column</code> is the name of a column, and <code>Type</code> is the type of the data such as an integer or string. Has an upper bound of 100 columns.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Vec<SchemaColumn>>,
    /// <p>Filters the list of machine learning transforms by the last known status of the transforms (to indicate whether a transform can be used or not). One of "NOT_READY", "READY", or "DELETING".</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of machine learning transform that is used to filter the machine learning transforms.</p>
    #[serde(rename = "TransformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_type: Option<String>,
}

/// <p>The algorithm-specific parameters that are associated with the machine learning transform.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformParameters {
    /// <p>The parameters for the find matches algorithm.</p>
    #[serde(rename = "FindMatchesParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_matches_parameters: Option<FindMatchesParameters>,
    /// <p>The type of machine learning transform.</p> <p>For information about the types of machine learning transforms, see <a href="http://docs.aws.amazon.com/glue/latest/dg/add-job-machine-learning-transform.html">Creating Machine Learning Transforms</a>.</p>
    #[serde(rename = "TransformType")]
    pub transform_type: String,
}

/// <p>The sorting criteria that are associated with the machine learning transform.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TransformSortCriteria {
    /// <p>The column to be used in the sorting criteria that are associated with the machine learning transform.</p>
    #[serde(rename = "Column")]
    pub column: String,
    /// <p>The sort direction to be used in the sorting criteria that are associated with the machine learning transform.</p>
    #[serde(rename = "SortDirection")]
    pub sort_direction: String,
}

/// <p>Information about a specific trigger.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Trigger {
    /// <p>The actions initiated by this trigger.</p>
    #[serde(rename = "Actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    /// <p>A description of this trigger.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the trigger.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The predicate of this trigger, which defines when it will fire.</p>
    #[serde(rename = "Predicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    /// <p>A <code>cron</code> expression used to specify the schedule (see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The current state of the trigger.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The type of trigger that this is.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The name of the workflow associated with the trigger.</p>
    #[serde(rename = "WorkflowName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

/// <p>The details of a Trigger node present in the workflow.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TriggerNodeDetails {
    /// <p>The information of the trigger represented by the trigger node.</p>
    #[serde(rename = "Trigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
}

/// <p>A structure used to provide information used to update a trigger. This object updates the previous trigger definition by overwriting it completely.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TriggerUpdate {
    /// <p>The actions initiated by this trigger.</p>
    #[serde(rename = "Actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    /// <p>A description of this trigger.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The predicate of this trigger, which defines when it will fire.</p>
    #[serde(rename = "Predicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    /// <p>A <code>cron</code> expression used to specify the schedule (see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource from which to remove the tags.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Tags to remove from this resource.</p>
    #[serde(rename = "TagsToRemove")]
    pub tags_to_remove: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateClassifierRequest {
    /// <p>A <code>CsvClassifier</code> object with updated fields.</p>
    #[serde(rename = "CsvClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_classifier: Option<UpdateCsvClassifierRequest>,
    /// <p>A <code>GrokClassifier</code> object with updated fields.</p>
    #[serde(rename = "GrokClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<UpdateGrokClassifierRequest>,
    /// <p>A <code>JsonClassifier</code> object with updated fields.</p>
    #[serde(rename = "JsonClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_classifier: Option<UpdateJsonClassifierRequest>,
    /// <p>An <code>XMLClassifier</code> object with updated fields.</p>
    #[serde(rename = "XMLClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_classifier: Option<UpdateXMLClassifierRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateClassifierResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateConnectionRequest {
    /// <p>The ID of the Data Catalog in which the connection resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A <code>ConnectionInput</code> object that redefines the connection in question.</p>
    #[serde(rename = "ConnectionInput")]
    pub connection_input: ConnectionInput,
    /// <p>The name of the connection definition to update.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConnectionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCrawlerRequest {
    /// <p>A list of custom classifiers that the user has registered. By default, all built-in classifiers are included in a crawl, but these custom classifiers always override the default classifiers for a given classification.</p>
    #[serde(rename = "Classifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    /// <p>The crawler configuration information. This versioned JSON string allows users to specify aspects of a crawler's behavior. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html">Configuring a Crawler</a>.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used by this crawler.</p>
    #[serde(rename = "CrawlerSecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_security_configuration: Option<String>,
    /// <p>The AWS Glue database where results are stored, such as: <code>arn:aws:daylight:us-east-1::database/sometable/*</code>.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>A description of the new crawler.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Name of the new crawler.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The IAM role or Amazon Resource Name (ARN) of an IAM role that is used by the new crawler to access customer resources.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>A <code>cron</code> expression used to specify the schedule. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, specify <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The policy for the crawler's update and deletion behavior.</p>
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    /// <p>The table prefix used for catalog tables that are created.</p>
    #[serde(rename = "TablePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<String>,
    /// <p>A list of targets to crawl.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<CrawlerTargets>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCrawlerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCrawlerScheduleRequest {
    /// <p>The name of the crawler whose schedule to update.</p>
    #[serde(rename = "CrawlerName")]
    pub crawler_name: String,
    /// <p>The updated <code>cron</code> expression used to specify the schedule. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, specify <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCrawlerScheduleResponse {}

/// <p>Specifies a custom CSV classifier to be updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCsvClassifierRequest {
    /// <p>Enables the processing of files that contain only one column.</p>
    #[serde(rename = "AllowSingleColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_single_column: Option<bool>,
    /// <p>Indicates whether the CSV file contains a header.</p>
    #[serde(rename = "ContainsHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_header: Option<String>,
    /// <p>A custom symbol to denote what separates each column entry in the row.</p>
    #[serde(rename = "Delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// <p>Specifies not to trim values before identifying the type of column values. The default value is true.</p>
    #[serde(rename = "DisableValueTrimming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_value_trimming: Option<bool>,
    /// <p>A list of strings representing column names.</p>
    #[serde(rename = "Header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A custom symbol to denote what combines content into a single column value. It must be different from the column delimiter.</p>
    #[serde(rename = "QuoteSymbol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_symbol: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDatabaseRequest {
    /// <p>The ID of the Data Catalog in which the metadata database resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A <code>DatabaseInput</code> object specifying the new definition of the metadata database in the catalog.</p>
    #[serde(rename = "DatabaseInput")]
    pub database_input: DatabaseInput,
    /// <p>The name of the database to update in the catalog. For Hive compatibility, this is folded to lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDatabaseResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDevEndpointRequest {
    /// <p>The map of arguments to add the map of arguments used to configure the <code>DevEndpoint</code>.</p>
    #[serde(rename = "AddArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>The list of public keys for the <code>DevEndpoint</code> to use.</p>
    #[serde(rename = "AddPublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_public_keys: Option<Vec<String>>,
    /// <p>Custom Python or Java libraries to be loaded in the <code>DevEndpoint</code>.</p>
    #[serde(rename = "CustomLibraries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_libraries: Option<DevEndpointCustomLibraries>,
    /// <p>The list of argument keys to be deleted from the map of arguments used to configure the <code>DevEndpoint</code>.</p>
    #[serde(rename = "DeleteArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_arguments: Option<Vec<String>>,
    /// <p>The list of public keys to be deleted from the <code>DevEndpoint</code>.</p>
    #[serde(rename = "DeletePublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_public_keys: Option<Vec<String>>,
    /// <p>The name of the <code>DevEndpoint</code> to be updated.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
    /// <p>The public key for the <code>DevEndpoint</code> to use.</p>
    #[serde(rename = "PublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// <p> <code>True</code> if the list of custom libraries to be loaded in the development endpoint needs to be updated, or <code>False</code> if otherwise.</p>
    #[serde(rename = "UpdateEtlLibraries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_etl_libraries: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDevEndpointResponse {}

/// <p>Specifies a grok classifier to update when passed to <code>UpdateClassifier</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGrokClassifierRequest {
    /// <p>An identifier of the data format that the classifier matches, such as Twitter, JSON, Omniture logs, Amazon CloudWatch Logs, and so on.</p>
    #[serde(rename = "Classification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// <p>Optional custom grok patterns used by this classifier.</p>
    #[serde(rename = "CustomPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_patterns: Option<String>,
    /// <p>The grok pattern used by this classifier.</p>
    #[serde(rename = "GrokPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_pattern: Option<String>,
    /// <p>The name of the <code>GrokClassifier</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateJobRequest {
    /// <p>The name of the job definition to update.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>Specifies the values with which to update the job definition.</p>
    #[serde(rename = "JobUpdate")]
    pub job_update: JobUpdate,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateJobResponse {
    /// <p>Returns the name of the updated job definition.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

/// <p>Specifies a JSON classifier to be updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateJsonClassifierRequest {
    /// <p>A <code>JsonPath</code> string defining the JSON data for the classifier to classify. AWS Glue supports a subset of <code>JsonPath</code>, as described in <a href="https://docs.aws.amazon.com/glue/latest/dg/custom-classifier.html#custom-classifier-json">Writing JsonPath Custom Classifiers</a>.</p>
    #[serde(rename = "JsonPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMLTransformRequest {
    /// <p>A description of the transform. The default is an empty string.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The number of AWS Glue data processing units (DPUs) that are allocated to task runs for this transform. You can allocate from 2 to 100 DPUs; the default is 10. A DPU is a relative measure of processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more information, see the <a href="https://aws.amazon.com/glue/pricing/">AWS Glue pricing page</a>. </p> <p>When the <code>WorkerType</code> field is set to a value other than <code>Standard</code>, the <code>MaxCapacity</code> field is set automatically and becomes read-only.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>The maximum number of times to retry a task for this transform after a task run fails.</p>
    #[serde(rename = "MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// <p>The unique name that you gave the transform when you created it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated when this task runs.</p>
    #[serde(rename = "NumberOfWorkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,
    /// <p>The configuration parameters that are specific to the transform type (algorithm) used. Conditionally dependent on the transform type.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<TransformParameters>,
    /// <p>The name or Amazon Resource Name (ARN) of the IAM role with the required permissions.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The timeout for a task run for this transform in minutes. This is the maximum time that a task run for this transform can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>A unique identifier that was generated when the transform was created.</p>
    #[serde(rename = "TransformId")]
    pub transform_id: String,
    /// <p><p>The type of predefined worker that is allocated when this task runs. Accepts a value of Standard, G.1X, or G.2X.</p> <ul> <li> <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p> </li> <li> <p>For the <code>G.1X</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 64GB disk, and 1 executor per worker.</p> </li> <li> <p>For the <code>G.2X</code> worker type, each worker provides 8 vCPU, 32 GB of memory and a 128GB disk, and 1 executor per worker.</p> </li> </ul></p>
    #[serde(rename = "WorkerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMLTransformResponse {
    /// <p>The unique identifier for the transform that was updated.</p>
    #[serde(rename = "TransformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePartitionRequest {
    /// <p>The ID of the Data Catalog where the partition to be updated resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which the table in question resides.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The new partition object to update the partition to.</p>
    #[serde(rename = "PartitionInput")]
    pub partition_input: PartitionInput,
    /// <p>A list of the values defining the partition.</p>
    #[serde(rename = "PartitionValueList")]
    pub partition_value_list: Vec<String>,
    /// <p>The name of the table in which the partition to be updated is located.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePartitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateTableRequest {
    /// <p>The ID of the Data Catalog where the table resides. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>By default, <code>UpdateTable</code> always creates an archived version of the table before updating it. However, if <code>skipArchive</code> is set to true, <code>UpdateTable</code> does not create the archived version.</p>
    #[serde(rename = "SkipArchive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_archive: Option<bool>,
    /// <p>An updated <code>TableInput</code> object to define the metadata table in the catalog.</p>
    #[serde(rename = "TableInput")]
    pub table_input: TableInput,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTableResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateTriggerRequest {
    /// <p>The name of the trigger to update.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The new values with which to update the trigger.</p>
    #[serde(rename = "TriggerUpdate")]
    pub trigger_update: TriggerUpdate,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTriggerResponse {
    /// <p>The resulting trigger definition.</p>
    #[serde(rename = "Trigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserDefinedFunctionRequest {
    /// <p>The ID of the Data Catalog where the function to be updated is located. If none is provided, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the function to be updated is located.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A <code>FunctionInput</code> object that redefines the function in the Data Catalog.</p>
    #[serde(rename = "FunctionInput")]
    pub function_input: UserDefinedFunctionInput,
    /// <p>The name of the function.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserDefinedFunctionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateWorkflowRequest {
    /// <p>A collection of properties to be used as part of each execution of the workflow.</p>
    #[serde(rename = "DefaultRunProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_run_properties: Option<::std::collections::HashMap<String, String>>,
    /// <p>The description of the workflow.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Name of the workflow to be updated.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateWorkflowResponse {
    /// <p>The name of the workflow which was specified in input.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Specifies an XML classifier to be updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateXMLClassifierRequest {
    /// <p>An identifier of the data format that the classifier matches.</p>
    #[serde(rename = "Classification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The XML tag designating the element that contains each record in an XML document being parsed. This cannot identify a self-closing element (closed by <code>/&gt;</code>). An empty row element that contains only attributes can be parsed as long as it ends with a closing tag (for example, <code>&lt;row item_a="A" item_b="B"&gt;&lt;/row&gt;</code> is okay, but <code>&lt;row item_a="A" item_b="B" /&gt;</code> is not).</p>
    #[serde(rename = "RowTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_tag: Option<String>,
}

/// <p>Represents the equivalent of a Hive user-defined function (<code>UDF</code>) definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserDefinedFunction {
    /// <p>The Java class that contains the function code.</p>
    #[serde(rename = "ClassName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// <p>The time at which the function was created.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>The name of the function.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>The owner of the function.</p>
    #[serde(rename = "OwnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    /// <p>The owner type.</p>
    #[serde(rename = "OwnerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<String>,
    /// <p>The resource URIs for the function.</p>
    #[serde(rename = "ResourceUris")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uris: Option<Vec<ResourceUri>>,
}

/// <p>A structure used to create or update a user-defined function.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UserDefinedFunctionInput {
    /// <p>The Java class that contains the function code.</p>
    #[serde(rename = "ClassName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// <p>The name of the function.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>The owner of the function.</p>
    #[serde(rename = "OwnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    /// <p>The owner type.</p>
    #[serde(rename = "OwnerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<String>,
    /// <p>The resource URIs for the function.</p>
    #[serde(rename = "ResourceUris")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uris: Option<Vec<ResourceUri>>,
}

/// <p>A workflow represents a flow in which AWS Glue components should be executed to complete a logical task.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Workflow {
    /// <p>The date and time when the workflow was created.</p>
    #[serde(rename = "CreatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    /// <p>A collection of properties to be used as part of each execution of the workflow.</p>
    #[serde(rename = "DefaultRunProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_run_properties: Option<::std::collections::HashMap<String, String>>,
    /// <p>A description of the workflow.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The graph representing all the AWS Glue components that belong to the workflow as nodes and directed connections between them as edges.</p>
    #[serde(rename = "Graph")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph: Option<WorkflowGraph>,
    /// <p>The date and time when the workflow was last modified.</p>
    #[serde(rename = "LastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>The information about the last execution of the workflow.</p>
    #[serde(rename = "LastRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run: Option<WorkflowRun>,
    /// <p>The name of the workflow representing the flow.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A workflow graph represents the complete workflow containing all the AWS Glue components present in the workflow and all the directed connections between them.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkflowGraph {
    /// <p>A list of all the directed connections between the nodes belonging to the workflow.</p>
    #[serde(rename = "Edges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<Edge>>,
    /// <p>A list of the the AWS Glue components belong to the workflow represented as nodes.</p>
    #[serde(rename = "Nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
}

/// <p>A workflow run is an execution of a workflow providing all the runtime information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkflowRun {
    /// <p>The date and time when the workflow run completed.</p>
    #[serde(rename = "CompletedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    /// <p>The graph representing all the AWS Glue components that belong to the workflow as nodes and directed connections between them as edges.</p>
    #[serde(rename = "Graph")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph: Option<WorkflowGraph>,
    /// <p>Name of the workflow which was executed.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The date and time when the workflow run was started.</p>
    #[serde(rename = "StartedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    /// <p>The statistics of the run.</p>
    #[serde(rename = "Statistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<WorkflowRunStatistics>,
    /// <p>The status of the workflow run.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The ID of this workflow run.</p>
    #[serde(rename = "WorkflowRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_run_id: Option<String>,
    /// <p>The workflow run properties which were set during the run.</p>
    #[serde(rename = "WorkflowRunProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_run_properties: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Workflow run statistics provides statistics about the workflow run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkflowRunStatistics {
    /// <p>Total number of Actions which have failed.</p>
    #[serde(rename = "FailedActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_actions: Option<i64>,
    /// <p>Total number Actions in running state.</p>
    #[serde(rename = "RunningActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_actions: Option<i64>,
    /// <p>Total number of Actions which have stopped.</p>
    #[serde(rename = "StoppedActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_actions: Option<i64>,
    /// <p>Total number of Actions which have succeeded.</p>
    #[serde(rename = "SucceededActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_actions: Option<i64>,
    /// <p>Total number of Actions which timed out.</p>
    #[serde(rename = "TimeoutActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_actions: Option<i64>,
    /// <p>Total number of Actions in the workflow run.</p>
    #[serde(rename = "TotalActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_actions: Option<i64>,
}

/// <p>A classifier for <code>XML</code> content.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct XMLClassifier {
    /// <p>An identifier of the data format that the classifier matches.</p>
    #[serde(rename = "Classification")]
    pub classification: String,
    /// <p>The time that this classifier was registered.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The time that this classifier was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The XML tag designating the element that contains each record in an XML document being parsed. This can't identify a self-closing element (closed by <code>/&gt;</code>). An empty row element that contains only attributes can be parsed as long as it ends with a closing tag (for example, <code>&lt;row item_a="A" item_b="B"&gt;&lt;/row&gt;</code> is okay, but <code>&lt;row item_a="A" item_b="B" /&gt;</code> is not).</p>
    #[serde(rename = "RowTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_tag: Option<String>,
    /// <p>The version of this classifier.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// Errors returned by BatchCreatePartition
#[derive(Debug, PartialEq)]
pub enum BatchCreatePartitionError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl BatchCreatePartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchCreatePartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(BatchCreatePartitionError::AlreadyExists(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(BatchCreatePartitionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(BatchCreatePartitionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchCreatePartitionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchCreatePartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchCreatePartitionError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        BatchCreatePartitionError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchCreatePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchCreatePartitionError {
    fn description(&self) -> &str {
        match *self {
            BatchCreatePartitionError::AlreadyExists(ref cause) => cause,
            BatchCreatePartitionError::EntityNotFound(ref cause) => cause,
            BatchCreatePartitionError::GlueEncryption(ref cause) => cause,
            BatchCreatePartitionError::InternalService(ref cause) => cause,
            BatchCreatePartitionError::InvalidInput(ref cause) => cause,
            BatchCreatePartitionError::OperationTimeout(ref cause) => cause,
            BatchCreatePartitionError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeleteConnection
#[derive(Debug, PartialEq)]
pub enum BatchDeleteConnectionError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchDeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(BatchDeleteConnectionError::InternalService(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchDeleteConnectionError::OperationTimeout(
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
impl fmt::Display for BatchDeleteConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeleteConnectionError {
    fn description(&self) -> &str {
        match *self {
            BatchDeleteConnectionError::InternalService(ref cause) => cause,
            BatchDeleteConnectionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeletePartition
#[derive(Debug, PartialEq)]
pub enum BatchDeletePartitionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchDeletePartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeletePartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(BatchDeletePartitionError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchDeletePartitionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchDeletePartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchDeletePartitionError::OperationTimeout(
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
impl fmt::Display for BatchDeletePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeletePartitionError {
    fn description(&self) -> &str {
        match *self {
            BatchDeletePartitionError::EntityNotFound(ref cause) => cause,
            BatchDeletePartitionError::InternalService(ref cause) => cause,
            BatchDeletePartitionError::InvalidInput(ref cause) => cause,
            BatchDeletePartitionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeleteTable
#[derive(Debug, PartialEq)]
pub enum BatchDeleteTableError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchDeleteTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(BatchDeleteTableError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchDeleteTableError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchDeleteTableError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchDeleteTableError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchDeleteTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeleteTableError {
    fn description(&self) -> &str {
        match *self {
            BatchDeleteTableError::EntityNotFound(ref cause) => cause,
            BatchDeleteTableError::InternalService(ref cause) => cause,
            BatchDeleteTableError::InvalidInput(ref cause) => cause,
            BatchDeleteTableError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeleteTableVersion
#[derive(Debug, PartialEq)]
pub enum BatchDeleteTableVersionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchDeleteTableVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteTableVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(BatchDeleteTableVersionError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchDeleteTableVersionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchDeleteTableVersionError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchDeleteTableVersionError::OperationTimeout(
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
impl fmt::Display for BatchDeleteTableVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeleteTableVersionError {
    fn description(&self) -> &str {
        match *self {
            BatchDeleteTableVersionError::EntityNotFound(ref cause) => cause,
            BatchDeleteTableVersionError::InternalService(ref cause) => cause,
            BatchDeleteTableVersionError::InvalidInput(ref cause) => cause,
            BatchDeleteTableVersionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetCrawlers
#[derive(Debug, PartialEq)]
pub enum BatchGetCrawlersError {
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchGetCrawlersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetCrawlersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGetCrawlersError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchGetCrawlersError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchGetCrawlersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetCrawlersError {
    fn description(&self) -> &str {
        match *self {
            BatchGetCrawlersError::InvalidInput(ref cause) => cause,
            BatchGetCrawlersError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetDevEndpoints
#[derive(Debug, PartialEq)]
pub enum BatchGetDevEndpointsError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchGetDevEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetDevEndpointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(BatchGetDevEndpointsError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchGetDevEndpointsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGetDevEndpointsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchGetDevEndpointsError::OperationTimeout(
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
impl fmt::Display for BatchGetDevEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetDevEndpointsError {
    fn description(&self) -> &str {
        match *self {
            BatchGetDevEndpointsError::AccessDenied(ref cause) => cause,
            BatchGetDevEndpointsError::InternalService(ref cause) => cause,
            BatchGetDevEndpointsError::InvalidInput(ref cause) => cause,
            BatchGetDevEndpointsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetJobs
#[derive(Debug, PartialEq)]
pub enum BatchGetJobsError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchGetJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(BatchGetJobsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGetJobsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchGetJobsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchGetJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetJobsError {
    fn description(&self) -> &str {
        match *self {
            BatchGetJobsError::InternalService(ref cause) => cause,
            BatchGetJobsError::InvalidInput(ref cause) => cause,
            BatchGetJobsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetPartition
#[derive(Debug, PartialEq)]
pub enum BatchGetPartitionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchGetPartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetPartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(BatchGetPartitionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(BatchGetPartitionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchGetPartitionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGetPartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchGetPartitionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchGetPartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetPartitionError {
    fn description(&self) -> &str {
        match *self {
            BatchGetPartitionError::EntityNotFound(ref cause) => cause,
            BatchGetPartitionError::GlueEncryption(ref cause) => cause,
            BatchGetPartitionError::InternalService(ref cause) => cause,
            BatchGetPartitionError::InvalidInput(ref cause) => cause,
            BatchGetPartitionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetTriggers
#[derive(Debug, PartialEq)]
pub enum BatchGetTriggersError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchGetTriggersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetTriggersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(BatchGetTriggersError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGetTriggersError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchGetTriggersError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchGetTriggersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetTriggersError {
    fn description(&self) -> &str {
        match *self {
            BatchGetTriggersError::InternalService(ref cause) => cause,
            BatchGetTriggersError::InvalidInput(ref cause) => cause,
            BatchGetTriggersError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetWorkflows
#[derive(Debug, PartialEq)]
pub enum BatchGetWorkflowsError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchGetWorkflowsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetWorkflowsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(BatchGetWorkflowsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGetWorkflowsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchGetWorkflowsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchGetWorkflowsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetWorkflowsError {
    fn description(&self) -> &str {
        match *self {
            BatchGetWorkflowsError::InternalService(ref cause) => cause,
            BatchGetWorkflowsError::InvalidInput(ref cause) => cause,
            BatchGetWorkflowsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchStopJobRun
#[derive(Debug, PartialEq)]
pub enum GlueBatchStopJobRunError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GlueBatchStopJobRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GlueBatchStopJobRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GlueBatchStopJobRunError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GlueBatchStopJobRunError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GlueBatchStopJobRunError::OperationTimeout(
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
impl fmt::Display for GlueBatchStopJobRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GlueBatchStopJobRunError {
    fn description(&self) -> &str {
        match *self {
            GlueBatchStopJobRunError::InternalService(ref cause) => cause,
            GlueBatchStopJobRunError::InvalidInput(ref cause) => cause,
            GlueBatchStopJobRunError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelMLTaskRun
#[derive(Debug, PartialEq)]
pub enum CancelMLTaskRunError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl CancelMLTaskRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelMLTaskRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(CancelMLTaskRunError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CancelMLTaskRunError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CancelMLTaskRunError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CancelMLTaskRunError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CancelMLTaskRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelMLTaskRunError {
    fn description(&self) -> &str {
        match *self {
            CancelMLTaskRunError::EntityNotFound(ref cause) => cause,
            CancelMLTaskRunError::InternalService(ref cause) => cause,
            CancelMLTaskRunError::InvalidInput(ref cause) => cause,
            CancelMLTaskRunError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateClassifier
#[derive(Debug, PartialEq)]
pub enum CreateClassifierError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl CreateClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateClassifierError::AlreadyExists(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateClassifierError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateClassifierError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClassifierError {
    fn description(&self) -> &str {
        match *self {
            CreateClassifierError::AlreadyExists(ref cause) => cause,
            CreateClassifierError::InvalidInput(ref cause) => cause,
            CreateClassifierError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConnection
#[derive(Debug, PartialEq)]
pub enum CreateConnectionError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateConnectionError::AlreadyExists(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(CreateConnectionError::GlueEncryption(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateConnectionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateConnectionError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        CreateConnectionError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConnectionError {
    fn description(&self) -> &str {
        match *self {
            CreateConnectionError::AlreadyExists(ref cause) => cause,
            CreateConnectionError::GlueEncryption(ref cause) => cause,
            CreateConnectionError::InvalidInput(ref cause) => cause,
            CreateConnectionError::OperationTimeout(ref cause) => cause,
            CreateConnectionError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCrawler
#[derive(Debug, PartialEq)]
pub enum CreateCrawlerError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateCrawlerError::AlreadyExists(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateCrawlerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateCrawlerError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateCrawlerError::ResourceNumberLimitExceeded(
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
impl fmt::Display for CreateCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCrawlerError {
    fn description(&self) -> &str {
        match *self {
            CreateCrawlerError::AlreadyExists(ref cause) => cause,
            CreateCrawlerError::InvalidInput(ref cause) => cause,
            CreateCrawlerError::OperationTimeout(ref cause) => cause,
            CreateCrawlerError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDatabase
#[derive(Debug, PartialEq)]
pub enum CreateDatabaseError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateDatabaseError::AlreadyExists(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(CreateDatabaseError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateDatabaseError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDatabaseError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateDatabaseError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateDatabaseError::ResourceNumberLimitExceeded(
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
impl fmt::Display for CreateDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDatabaseError {
    fn description(&self) -> &str {
        match *self {
            CreateDatabaseError::AlreadyExists(ref cause) => cause,
            CreateDatabaseError::GlueEncryption(ref cause) => cause,
            CreateDatabaseError::InternalService(ref cause) => cause,
            CreateDatabaseError::InvalidInput(ref cause) => cause,
            CreateDatabaseError::OperationTimeout(ref cause) => cause,
            CreateDatabaseError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDevEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateDevEndpointError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>The same unique identifier was associated with two different records.</p>
    IdempotentParameterMismatch(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateDevEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDevEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDevEndpointError::AccessDenied(err.msg))
                }
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateDevEndpointError::AlreadyExists(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateDevEndpointError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateDevEndpointError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDevEndpointError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateDevEndpointError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        CreateDevEndpointError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            CreateDevEndpointError::AccessDenied(ref cause) => cause,
            CreateDevEndpointError::AlreadyExists(ref cause) => cause,
            CreateDevEndpointError::IdempotentParameterMismatch(ref cause) => cause,
            CreateDevEndpointError::InternalService(ref cause) => cause,
            CreateDevEndpointError::InvalidInput(ref cause) => cause,
            CreateDevEndpointError::OperationTimeout(ref cause) => cause,
            CreateDevEndpointError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>The same unique identifier was associated with two different records.</p>
    IdempotentParameterMismatch(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateJobError::AlreadyExists(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateJobError::ConcurrentModification(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(CreateJobError::IdempotentParameterMismatch(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateJobError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateJobError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateJobError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateJobError::ResourceNumberLimitExceeded(
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
impl fmt::Display for CreateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateJobError {
    fn description(&self) -> &str {
        match *self {
            CreateJobError::AlreadyExists(ref cause) => cause,
            CreateJobError::ConcurrentModification(ref cause) => cause,
            CreateJobError::IdempotentParameterMismatch(ref cause) => cause,
            CreateJobError::InternalService(ref cause) => cause,
            CreateJobError::InvalidInput(ref cause) => cause,
            CreateJobError::OperationTimeout(ref cause) => cause,
            CreateJobError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateMLTransform
#[derive(Debug, PartialEq)]
pub enum CreateMLTransformError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>The same unique identifier was associated with two different records.</p>
    IdempotentParameterMismatch(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateMLTransformError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMLTransformError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateMLTransformError::AccessDenied(err.msg))
                }
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateMLTransformError::AlreadyExists(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateMLTransformError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateMLTransformError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateMLTransformError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateMLTransformError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        CreateMLTransformError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateMLTransformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMLTransformError {
    fn description(&self) -> &str {
        match *self {
            CreateMLTransformError::AccessDenied(ref cause) => cause,
            CreateMLTransformError::AlreadyExists(ref cause) => cause,
            CreateMLTransformError::IdempotentParameterMismatch(ref cause) => cause,
            CreateMLTransformError::InternalService(ref cause) => cause,
            CreateMLTransformError::InvalidInput(ref cause) => cause,
            CreateMLTransformError::OperationTimeout(ref cause) => cause,
            CreateMLTransformError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePartition
#[derive(Debug, PartialEq)]
pub enum CreatePartitionError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreatePartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreatePartitionError::AlreadyExists(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(CreatePartitionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(CreatePartitionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreatePartitionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreatePartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreatePartitionError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreatePartitionError::ResourceNumberLimitExceeded(
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
impl fmt::Display for CreatePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePartitionError {
    fn description(&self) -> &str {
        match *self {
            CreatePartitionError::AlreadyExists(ref cause) => cause,
            CreatePartitionError::EntityNotFound(ref cause) => cause,
            CreatePartitionError::GlueEncryption(ref cause) => cause,
            CreatePartitionError::InternalService(ref cause) => cause,
            CreatePartitionError::InvalidInput(ref cause) => cause,
            CreatePartitionError::OperationTimeout(ref cause) => cause,
            CreatePartitionError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateScript
#[derive(Debug, PartialEq)]
pub enum CreateScriptError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl CreateScriptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateScriptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(CreateScriptError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateScriptError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateScriptError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateScriptError {
    fn description(&self) -> &str {
        match *self {
            CreateScriptError::InternalService(ref cause) => cause,
            CreateScriptError::InvalidInput(ref cause) => cause,
            CreateScriptError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateSecurityConfigurationError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateSecurityConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateSecurityConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateSecurityConfigurationError::AlreadyExists(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateSecurityConfigurationError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateSecurityConfigurationError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(
                        CreateSecurityConfigurationError::OperationTimeout(err.msg),
                    )
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        CreateSecurityConfigurationError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateSecurityConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSecurityConfigurationError {
    fn description(&self) -> &str {
        match *self {
            CreateSecurityConfigurationError::AlreadyExists(ref cause) => cause,
            CreateSecurityConfigurationError::InternalService(ref cause) => cause,
            CreateSecurityConfigurationError::InvalidInput(ref cause) => cause,
            CreateSecurityConfigurationError::OperationTimeout(ref cause) => cause,
            CreateSecurityConfigurationError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTable
#[derive(Debug, PartialEq)]
pub enum CreateTableError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateTableError::AlreadyExists(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(CreateTableError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(CreateTableError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateTableError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateTableError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateTableError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateTableError::ResourceNumberLimitExceeded(
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
impl fmt::Display for CreateTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTableError {
    fn description(&self) -> &str {
        match *self {
            CreateTableError::AlreadyExists(ref cause) => cause,
            CreateTableError::EntityNotFound(ref cause) => cause,
            CreateTableError::GlueEncryption(ref cause) => cause,
            CreateTableError::InternalService(ref cause) => cause,
            CreateTableError::InvalidInput(ref cause) => cause,
            CreateTableError::OperationTimeout(ref cause) => cause,
            CreateTableError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTrigger
#[derive(Debug, PartialEq)]
pub enum CreateTriggerError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The same unique identifier was associated with two different records.</p>
    IdempotentParameterMismatch(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateTriggerError::AlreadyExists(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateTriggerError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(CreateTriggerError::EntityNotFound(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(CreateTriggerError::IdempotentParameterMismatch(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateTriggerError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateTriggerError::ResourceNumberLimitExceeded(
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
impl fmt::Display for CreateTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTriggerError {
    fn description(&self) -> &str {
        match *self {
            CreateTriggerError::AlreadyExists(ref cause) => cause,
            CreateTriggerError::ConcurrentModification(ref cause) => cause,
            CreateTriggerError::EntityNotFound(ref cause) => cause,
            CreateTriggerError::IdempotentParameterMismatch(ref cause) => cause,
            CreateTriggerError::InternalService(ref cause) => cause,
            CreateTriggerError::InvalidInput(ref cause) => cause,
            CreateTriggerError::OperationTimeout(ref cause) => cause,
            CreateTriggerError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum CreateUserDefinedFunctionError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateUserDefinedFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserDefinedFunctionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::AlreadyExists(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::EntityNotFound(
                        err.msg,
                    ))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::GlueEncryption(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        CreateUserDefinedFunctionError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            CreateUserDefinedFunctionError::AlreadyExists(ref cause) => cause,
            CreateUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            CreateUserDefinedFunctionError::GlueEncryption(ref cause) => cause,
            CreateUserDefinedFunctionError::InternalService(ref cause) => cause,
            CreateUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            CreateUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
            CreateUserDefinedFunctionError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateWorkflow
#[derive(Debug, PartialEq)]
pub enum CreateWorkflowError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateWorkflowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWorkflowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateWorkflowError::AlreadyExists(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateWorkflowError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateWorkflowError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateWorkflowError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateWorkflowError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateWorkflowError::ResourceNumberLimitExceeded(
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
impl fmt::Display for CreateWorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateWorkflowError {
    fn description(&self) -> &str {
        match *self {
            CreateWorkflowError::AlreadyExists(ref cause) => cause,
            CreateWorkflowError::ConcurrentModification(ref cause) => cause,
            CreateWorkflowError::InternalService(ref cause) => cause,
            CreateWorkflowError::InvalidInput(ref cause) => cause,
            CreateWorkflowError::OperationTimeout(ref cause) => cause,
            CreateWorkflowError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteClassifier
#[derive(Debug, PartialEq)]
pub enum DeleteClassifierError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteClassifierError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteClassifierError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClassifierError {
    fn description(&self) -> &str {
        match *self {
            DeleteClassifierError::EntityNotFound(ref cause) => cause,
            DeleteClassifierError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteConnectionError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteConnectionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConnectionError {
    fn description(&self) -> &str {
        match *self {
            DeleteConnectionError::EntityNotFound(ref cause) => cause,
            DeleteConnectionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCrawler
#[derive(Debug, PartialEq)]
pub enum DeleteCrawlerError {
    /// <p>The operation cannot be performed because the crawler is already running.</p>
    CrawlerRunning(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
}

impl DeleteCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CrawlerRunningException" => {
                    return RusotoError::Service(DeleteCrawlerError::CrawlerRunning(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteCrawlerError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteCrawlerError::OperationTimeout(err.msg))
                }
                "SchedulerTransitioningException" => {
                    return RusotoError::Service(DeleteCrawlerError::SchedulerTransitioning(
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
impl fmt::Display for DeleteCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCrawlerError {
    fn description(&self) -> &str {
        match *self {
            DeleteCrawlerError::CrawlerRunning(ref cause) => cause,
            DeleteCrawlerError::EntityNotFound(ref cause) => cause,
            DeleteCrawlerError::OperationTimeout(ref cause) => cause,
            DeleteCrawlerError::SchedulerTransitioning(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDatabase
#[derive(Debug, PartialEq)]
pub enum DeleteDatabaseError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteDatabaseError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteDatabaseError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDatabaseError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteDatabaseError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDatabaseError {
    fn description(&self) -> &str {
        match *self {
            DeleteDatabaseError::EntityNotFound(ref cause) => cause,
            DeleteDatabaseError::InternalService(ref cause) => cause,
            DeleteDatabaseError::InvalidInput(ref cause) => cause,
            DeleteDatabaseError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDevEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteDevEndpointError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteDevEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDevEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteDevEndpointError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteDevEndpointError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDevEndpointError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteDevEndpointError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteDevEndpointError::EntityNotFound(ref cause) => cause,
            DeleteDevEndpointError::InternalService(ref cause) => cause,
            DeleteDevEndpointError::InvalidInput(ref cause) => cause,
            DeleteDevEndpointError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteJob
#[derive(Debug, PartialEq)]
pub enum DeleteJobError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteJobError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteJobError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteJobError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteJobError {
    fn description(&self) -> &str {
        match *self {
            DeleteJobError::InternalService(ref cause) => cause,
            DeleteJobError::InvalidInput(ref cause) => cause,
            DeleteJobError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMLTransform
#[derive(Debug, PartialEq)]
pub enum DeleteMLTransformError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteMLTransformError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMLTransformError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteMLTransformError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteMLTransformError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteMLTransformError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteMLTransformError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteMLTransformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMLTransformError {
    fn description(&self) -> &str {
        match *self {
            DeleteMLTransformError::EntityNotFound(ref cause) => cause,
            DeleteMLTransformError::InternalService(ref cause) => cause,
            DeleteMLTransformError::InvalidInput(ref cause) => cause,
            DeleteMLTransformError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePartition
#[derive(Debug, PartialEq)]
pub enum DeletePartitionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeletePartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeletePartitionError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeletePartitionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeletePartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeletePartitionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePartitionError {
    fn description(&self) -> &str {
        match *self {
            DeletePartitionError::EntityNotFound(ref cause) => cause,
            DeletePartitionError::InternalService(ref cause) => cause,
            DeletePartitionError::InvalidInput(ref cause) => cause,
            DeletePartitionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteResourcePolicy
#[derive(Debug, PartialEq)]
pub enum DeleteResourcePolicyError {
    /// <p>A specified condition was not satisfied.</p>
    ConditionCheckFailure(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConditionCheckFailureException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::ConditionCheckFailure(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::OperationTimeout(
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
impl fmt::Display for DeleteResourcePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourcePolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourcePolicyError::ConditionCheckFailure(ref cause) => cause,
            DeleteResourcePolicyError::EntityNotFound(ref cause) => cause,
            DeleteResourcePolicyError::InternalService(ref cause) => cause,
            DeleteResourcePolicyError::InvalidInput(ref cause) => cause,
            DeleteResourcePolicyError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteSecurityConfigurationError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteSecurityConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteSecurityConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteSecurityConfigurationError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteSecurityConfigurationError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteSecurityConfigurationError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(
                        DeleteSecurityConfigurationError::OperationTimeout(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteSecurityConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSecurityConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteSecurityConfigurationError::EntityNotFound(ref cause) => cause,
            DeleteSecurityConfigurationError::InternalService(ref cause) => cause,
            DeleteSecurityConfigurationError::InvalidInput(ref cause) => cause,
            DeleteSecurityConfigurationError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTable
#[derive(Debug, PartialEq)]
pub enum DeleteTableError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteTableError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteTableError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteTableError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteTableError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTableError {
    fn description(&self) -> &str {
        match *self {
            DeleteTableError::EntityNotFound(ref cause) => cause,
            DeleteTableError::InternalService(ref cause) => cause,
            DeleteTableError::InvalidInput(ref cause) => cause,
            DeleteTableError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTableVersion
#[derive(Debug, PartialEq)]
pub enum DeleteTableVersionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteTableVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTableVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteTableVersionError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteTableVersionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteTableVersionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteTableVersionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTableVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTableVersionError {
    fn description(&self) -> &str {
        match *self {
            DeleteTableVersionError::EntityNotFound(ref cause) => cause,
            DeleteTableVersionError::InternalService(ref cause) => cause,
            DeleteTableVersionError::InvalidInput(ref cause) => cause,
            DeleteTableVersionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTrigger
#[derive(Debug, PartialEq)]
pub enum DeleteTriggerError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteTriggerError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteTriggerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTriggerError {
    fn description(&self) -> &str {
        match *self {
            DeleteTriggerError::ConcurrentModification(ref cause) => cause,
            DeleteTriggerError::InternalService(ref cause) => cause,
            DeleteTriggerError::InvalidInput(ref cause) => cause,
            DeleteTriggerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum DeleteUserDefinedFunctionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteUserDefinedFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserDefinedFunctionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteUserDefinedFunctionError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteUserDefinedFunctionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteUserDefinedFunctionError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteUserDefinedFunctionError::OperationTimeout(
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
impl fmt::Display for DeleteUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            DeleteUserDefinedFunctionError::InternalService(ref cause) => cause,
            DeleteUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            DeleteUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteWorkflow
#[derive(Debug, PartialEq)]
pub enum DeleteWorkflowError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteWorkflowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWorkflowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteWorkflowError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteWorkflowError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteWorkflowError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteWorkflowError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteWorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteWorkflowError {
    fn description(&self) -> &str {
        match *self {
            DeleteWorkflowError::ConcurrentModification(ref cause) => cause,
            DeleteWorkflowError::InternalService(ref cause) => cause,
            DeleteWorkflowError::InvalidInput(ref cause) => cause,
            DeleteWorkflowError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCatalogImportStatus
#[derive(Debug, PartialEq)]
pub enum GetCatalogImportStatusError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetCatalogImportStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCatalogImportStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetCatalogImportStatusError::InternalService(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetCatalogImportStatusError::OperationTimeout(
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
impl fmt::Display for GetCatalogImportStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCatalogImportStatusError {
    fn description(&self) -> &str {
        match *self {
            GetCatalogImportStatusError::InternalService(ref cause) => cause,
            GetCatalogImportStatusError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetClassifier
#[derive(Debug, PartialEq)]
pub enum GetClassifierError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetClassifierError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetClassifierError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetClassifierError {
    fn description(&self) -> &str {
        match *self {
            GetClassifierError::EntityNotFound(ref cause) => cause,
            GetClassifierError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetClassifiers
#[derive(Debug, PartialEq)]
pub enum GetClassifiersError {
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetClassifiersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetClassifiersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetClassifiersError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetClassifiersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetClassifiersError {
    fn description(&self) -> &str {
        match *self {
            GetClassifiersError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnection
#[derive(Debug, PartialEq)]
pub enum GetConnectionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetConnectionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetConnectionError::GlueEncryption(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetConnectionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetConnectionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectionError {
    fn description(&self) -> &str {
        match *self {
            GetConnectionError::EntityNotFound(ref cause) => cause,
            GetConnectionError::GlueEncryption(ref cause) => cause,
            GetConnectionError::InvalidInput(ref cause) => cause,
            GetConnectionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnections
#[derive(Debug, PartialEq)]
pub enum GetConnectionsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetConnectionsError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetConnectionsError::GlueEncryption(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetConnectionsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetConnectionsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConnectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectionsError {
    fn description(&self) -> &str {
        match *self {
            GetConnectionsError::EntityNotFound(ref cause) => cause,
            GetConnectionsError::GlueEncryption(ref cause) => cause,
            GetConnectionsError::InvalidInput(ref cause) => cause,
            GetConnectionsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCrawler
#[derive(Debug, PartialEq)]
pub enum GetCrawlerError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetCrawlerError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetCrawlerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCrawlerError {
    fn description(&self) -> &str {
        match *self {
            GetCrawlerError::EntityNotFound(ref cause) => cause,
            GetCrawlerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCrawlerMetrics
#[derive(Debug, PartialEq)]
pub enum GetCrawlerMetricsError {
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetCrawlerMetricsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCrawlerMetricsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetCrawlerMetricsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCrawlerMetricsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCrawlerMetricsError {
    fn description(&self) -> &str {
        match *self {
            GetCrawlerMetricsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCrawlers
#[derive(Debug, PartialEq)]
pub enum GetCrawlersError {
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetCrawlersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCrawlersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetCrawlersError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCrawlersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCrawlersError {
    fn description(&self) -> &str {
        match *self {
            GetCrawlersError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDataCatalogEncryptionSettings
#[derive(Debug, PartialEq)]
pub enum GetDataCatalogEncryptionSettingsError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDataCatalogEncryptionSettingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDataCatalogEncryptionSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        GetDataCatalogEncryptionSettingsError::InternalService(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetDataCatalogEncryptionSettingsError::InvalidInput(err.msg),
                    )
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(
                        GetDataCatalogEncryptionSettingsError::OperationTimeout(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDataCatalogEncryptionSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDataCatalogEncryptionSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetDataCatalogEncryptionSettingsError::InternalService(ref cause) => cause,
            GetDataCatalogEncryptionSettingsError::InvalidInput(ref cause) => cause,
            GetDataCatalogEncryptionSettingsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDatabase
#[derive(Debug, PartialEq)]
pub enum GetDatabaseError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetDatabaseError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetDatabaseError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetDatabaseError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDatabaseError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetDatabaseError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDatabaseError {
    fn description(&self) -> &str {
        match *self {
            GetDatabaseError::EntityNotFound(ref cause) => cause,
            GetDatabaseError::GlueEncryption(ref cause) => cause,
            GetDatabaseError::InternalService(ref cause) => cause,
            GetDatabaseError::InvalidInput(ref cause) => cause,
            GetDatabaseError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDatabases
#[derive(Debug, PartialEq)]
pub enum GetDatabasesError {
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDatabasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDatabasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetDatabasesError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetDatabasesError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDatabasesError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetDatabasesError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDatabasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDatabasesError {
    fn description(&self) -> &str {
        match *self {
            GetDatabasesError::GlueEncryption(ref cause) => cause,
            GetDatabasesError::InternalService(ref cause) => cause,
            GetDatabasesError::InvalidInput(ref cause) => cause,
            GetDatabasesError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDataflowGraph
#[derive(Debug, PartialEq)]
pub enum GetDataflowGraphError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDataflowGraphError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDataflowGraphError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetDataflowGraphError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDataflowGraphError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetDataflowGraphError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDataflowGraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDataflowGraphError {
    fn description(&self) -> &str {
        match *self {
            GetDataflowGraphError::InternalService(ref cause) => cause,
            GetDataflowGraphError::InvalidInput(ref cause) => cause,
            GetDataflowGraphError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevEndpoint
#[derive(Debug, PartialEq)]
pub enum GetDevEndpointError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDevEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDevEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetDevEndpointError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetDevEndpointError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDevEndpointError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetDevEndpointError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            GetDevEndpointError::EntityNotFound(ref cause) => cause,
            GetDevEndpointError::InternalService(ref cause) => cause,
            GetDevEndpointError::InvalidInput(ref cause) => cause,
            GetDevEndpointError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevEndpoints
#[derive(Debug, PartialEq)]
pub enum GetDevEndpointsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDevEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDevEndpointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetDevEndpointsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetDevEndpointsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDevEndpointsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetDevEndpointsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDevEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDevEndpointsError {
    fn description(&self) -> &str {
        match *self {
            GetDevEndpointsError::EntityNotFound(ref cause) => cause,
            GetDevEndpointsError::InternalService(ref cause) => cause,
            GetDevEndpointsError::InvalidInput(ref cause) => cause,
            GetDevEndpointsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJob
#[derive(Debug, PartialEq)]
pub enum GetJobError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetJobError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetJobError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetJobError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetJobError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobError {
    fn description(&self) -> &str {
        match *self {
            GetJobError::EntityNotFound(ref cause) => cause,
            GetJobError::InternalService(ref cause) => cause,
            GetJobError::InvalidInput(ref cause) => cause,
            GetJobError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobBookmark
#[derive(Debug, PartialEq)]
pub enum GetJobBookmarkError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetJobBookmarkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobBookmarkError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetJobBookmarkError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetJobBookmarkError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetJobBookmarkError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetJobBookmarkError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobBookmarkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobBookmarkError {
    fn description(&self) -> &str {
        match *self {
            GetJobBookmarkError::EntityNotFound(ref cause) => cause,
            GetJobBookmarkError::InternalService(ref cause) => cause,
            GetJobBookmarkError::InvalidInput(ref cause) => cause,
            GetJobBookmarkError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobRun
#[derive(Debug, PartialEq)]
pub enum GetJobRunError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetJobRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetJobRunError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetJobRunError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetJobRunError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetJobRunError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobRunError {
    fn description(&self) -> &str {
        match *self {
            GetJobRunError::EntityNotFound(ref cause) => cause,
            GetJobRunError::InternalService(ref cause) => cause,
            GetJobRunError::InvalidInput(ref cause) => cause,
            GetJobRunError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobRuns
#[derive(Debug, PartialEq)]
pub enum GetJobRunsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetJobRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobRunsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetJobRunsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetJobRunsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetJobRunsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetJobRunsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobRunsError {
    fn description(&self) -> &str {
        match *self {
            GetJobRunsError::EntityNotFound(ref cause) => cause,
            GetJobRunsError::InternalService(ref cause) => cause,
            GetJobRunsError::InvalidInput(ref cause) => cause,
            GetJobRunsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobs
#[derive(Debug, PartialEq)]
pub enum GetJobsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetJobsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetJobsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetJobsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetJobsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobsError {
    fn description(&self) -> &str {
        match *self {
            GetJobsError::EntityNotFound(ref cause) => cause,
            GetJobsError::InternalService(ref cause) => cause,
            GetJobsError::InvalidInput(ref cause) => cause,
            GetJobsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMLTaskRun
#[derive(Debug, PartialEq)]
pub enum GetMLTaskRunError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetMLTaskRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMLTaskRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetMLTaskRunError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetMLTaskRunError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetMLTaskRunError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetMLTaskRunError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMLTaskRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMLTaskRunError {
    fn description(&self) -> &str {
        match *self {
            GetMLTaskRunError::EntityNotFound(ref cause) => cause,
            GetMLTaskRunError::InternalService(ref cause) => cause,
            GetMLTaskRunError::InvalidInput(ref cause) => cause,
            GetMLTaskRunError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMLTaskRuns
#[derive(Debug, PartialEq)]
pub enum GetMLTaskRunsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetMLTaskRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMLTaskRunsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetMLTaskRunsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetMLTaskRunsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetMLTaskRunsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetMLTaskRunsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMLTaskRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMLTaskRunsError {
    fn description(&self) -> &str {
        match *self {
            GetMLTaskRunsError::EntityNotFound(ref cause) => cause,
            GetMLTaskRunsError::InternalService(ref cause) => cause,
            GetMLTaskRunsError::InvalidInput(ref cause) => cause,
            GetMLTaskRunsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMLTransform
#[derive(Debug, PartialEq)]
pub enum GetMLTransformError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetMLTransformError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMLTransformError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetMLTransformError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetMLTransformError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetMLTransformError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetMLTransformError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMLTransformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMLTransformError {
    fn description(&self) -> &str {
        match *self {
            GetMLTransformError::EntityNotFound(ref cause) => cause,
            GetMLTransformError::InternalService(ref cause) => cause,
            GetMLTransformError::InvalidInput(ref cause) => cause,
            GetMLTransformError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMLTransforms
#[derive(Debug, PartialEq)]
pub enum GetMLTransformsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetMLTransformsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMLTransformsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetMLTransformsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetMLTransformsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetMLTransformsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetMLTransformsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMLTransformsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMLTransformsError {
    fn description(&self) -> &str {
        match *self {
            GetMLTransformsError::EntityNotFound(ref cause) => cause,
            GetMLTransformsError::InternalService(ref cause) => cause,
            GetMLTransformsError::InvalidInput(ref cause) => cause,
            GetMLTransformsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMapping
#[derive(Debug, PartialEq)]
pub enum GetMappingError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMappingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetMappingError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetMappingError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetMappingError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetMappingError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMappingError {
    fn description(&self) -> &str {
        match *self {
            GetMappingError::EntityNotFound(ref cause) => cause,
            GetMappingError::InternalService(ref cause) => cause,
            GetMappingError::InvalidInput(ref cause) => cause,
            GetMappingError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPartition
#[derive(Debug, PartialEq)]
pub enum GetPartitionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetPartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetPartitionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetPartitionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetPartitionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetPartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetPartitionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPartitionError {
    fn description(&self) -> &str {
        match *self {
            GetPartitionError::EntityNotFound(ref cause) => cause,
            GetPartitionError::GlueEncryption(ref cause) => cause,
            GetPartitionError::InternalService(ref cause) => cause,
            GetPartitionError::InvalidInput(ref cause) => cause,
            GetPartitionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPartitions
#[derive(Debug, PartialEq)]
pub enum GetPartitionsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetPartitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPartitionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetPartitionsError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetPartitionsError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetPartitionsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetPartitionsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetPartitionsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPartitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPartitionsError {
    fn description(&self) -> &str {
        match *self {
            GetPartitionsError::EntityNotFound(ref cause) => cause,
            GetPartitionsError::GlueEncryption(ref cause) => cause,
            GetPartitionsError::InternalService(ref cause) => cause,
            GetPartitionsError::InvalidInput(ref cause) => cause,
            GetPartitionsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPlan
#[derive(Debug, PartialEq)]
pub enum GetPlanError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetPlanError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetPlanError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetPlanError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPlanError {
    fn description(&self) -> &str {
        match *self {
            GetPlanError::InternalService(ref cause) => cause,
            GetPlanError::InvalidInput(ref cause) => cause,
            GetPlanError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResourcePolicy
#[derive(Debug, PartialEq)]
pub enum GetResourcePolicyError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetResourcePolicyError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetResourcePolicyError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetResourcePolicyError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetResourcePolicyError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetResourcePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourcePolicyError {
    fn description(&self) -> &str {
        match *self {
            GetResourcePolicyError::EntityNotFound(ref cause) => cause,
            GetResourcePolicyError::InternalService(ref cause) => cause,
            GetResourcePolicyError::InvalidInput(ref cause) => cause,
            GetResourcePolicyError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum GetSecurityConfigurationError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetSecurityConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSecurityConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetSecurityConfigurationError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetSecurityConfigurationError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetSecurityConfigurationError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetSecurityConfigurationError::OperationTimeout(
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
impl fmt::Display for GetSecurityConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSecurityConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetSecurityConfigurationError::EntityNotFound(ref cause) => cause,
            GetSecurityConfigurationError::InternalService(ref cause) => cause,
            GetSecurityConfigurationError::InvalidInput(ref cause) => cause,
            GetSecurityConfigurationError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSecurityConfigurations
#[derive(Debug, PartialEq)]
pub enum GetSecurityConfigurationsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetSecurityConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSecurityConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetSecurityConfigurationsError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetSecurityConfigurationsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetSecurityConfigurationsError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetSecurityConfigurationsError::OperationTimeout(
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
impl fmt::Display for GetSecurityConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSecurityConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            GetSecurityConfigurationsError::EntityNotFound(ref cause) => cause,
            GetSecurityConfigurationsError::InternalService(ref cause) => cause,
            GetSecurityConfigurationsError::InvalidInput(ref cause) => cause,
            GetSecurityConfigurationsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTable
#[derive(Debug, PartialEq)]
pub enum GetTableError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTableError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetTableError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTableError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTableError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTableError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTableError {
    fn description(&self) -> &str {
        match *self {
            GetTableError::EntityNotFound(ref cause) => cause,
            GetTableError::GlueEncryption(ref cause) => cause,
            GetTableError::InternalService(ref cause) => cause,
            GetTableError::InvalidInput(ref cause) => cause,
            GetTableError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTableVersion
#[derive(Debug, PartialEq)]
pub enum GetTableVersionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTableVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTableVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTableVersionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetTableVersionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTableVersionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTableVersionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTableVersionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTableVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTableVersionError {
    fn description(&self) -> &str {
        match *self {
            GetTableVersionError::EntityNotFound(ref cause) => cause,
            GetTableVersionError::GlueEncryption(ref cause) => cause,
            GetTableVersionError::InternalService(ref cause) => cause,
            GetTableVersionError::InvalidInput(ref cause) => cause,
            GetTableVersionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTableVersions
#[derive(Debug, PartialEq)]
pub enum GetTableVersionsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTableVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTableVersionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTableVersionsError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetTableVersionsError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTableVersionsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTableVersionsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTableVersionsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTableVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTableVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetTableVersionsError::EntityNotFound(ref cause) => cause,
            GetTableVersionsError::GlueEncryption(ref cause) => cause,
            GetTableVersionsError::InternalService(ref cause) => cause,
            GetTableVersionsError::InvalidInput(ref cause) => cause,
            GetTableVersionsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTables
#[derive(Debug, PartialEq)]
pub enum GetTablesError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTablesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTablesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTablesError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetTablesError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTablesError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTablesError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTablesError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTablesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTablesError {
    fn description(&self) -> &str {
        match *self {
            GetTablesError::EntityNotFound(ref cause) => cause,
            GetTablesError::GlueEncryption(ref cause) => cause,
            GetTablesError::InternalService(ref cause) => cause,
            GetTablesError::InvalidInput(ref cause) => cause,
            GetTablesError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTags
#[derive(Debug, PartialEq)]
pub enum GetTagsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTagsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTagsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTagsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTagsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTagsError {
    fn description(&self) -> &str {
        match *self {
            GetTagsError::EntityNotFound(ref cause) => cause,
            GetTagsError::InternalService(ref cause) => cause,
            GetTagsError::InvalidInput(ref cause) => cause,
            GetTagsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTrigger
#[derive(Debug, PartialEq)]
pub enum GetTriggerError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTriggerError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTriggerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTriggerError {
    fn description(&self) -> &str {
        match *self {
            GetTriggerError::EntityNotFound(ref cause) => cause,
            GetTriggerError::InternalService(ref cause) => cause,
            GetTriggerError::InvalidInput(ref cause) => cause,
            GetTriggerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTriggers
#[derive(Debug, PartialEq)]
pub enum GetTriggersError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTriggersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTriggersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTriggersError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTriggersError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTriggersError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTriggersError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTriggersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTriggersError {
    fn description(&self) -> &str {
        match *self {
            GetTriggersError::EntityNotFound(ref cause) => cause,
            GetTriggersError::InternalService(ref cause) => cause,
            GetTriggersError::InvalidInput(ref cause) => cause,
            GetTriggersError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum GetUserDefinedFunctionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetUserDefinedFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUserDefinedFunctionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetUserDefinedFunctionError::EntityNotFound(
                        err.msg,
                    ))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetUserDefinedFunctionError::GlueEncryption(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetUserDefinedFunctionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetUserDefinedFunctionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetUserDefinedFunctionError::OperationTimeout(
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
impl fmt::Display for GetUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            GetUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            GetUserDefinedFunctionError::GlueEncryption(ref cause) => cause,
            GetUserDefinedFunctionError::InternalService(ref cause) => cause,
            GetUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            GetUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUserDefinedFunctions
#[derive(Debug, PartialEq)]
pub enum GetUserDefinedFunctionsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetUserDefinedFunctionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUserDefinedFunctionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetUserDefinedFunctionsError::EntityNotFound(
                        err.msg,
                    ))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetUserDefinedFunctionsError::GlueEncryption(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetUserDefinedFunctionsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetUserDefinedFunctionsError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetUserDefinedFunctionsError::OperationTimeout(
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
impl fmt::Display for GetUserDefinedFunctionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserDefinedFunctionsError {
    fn description(&self) -> &str {
        match *self {
            GetUserDefinedFunctionsError::EntityNotFound(ref cause) => cause,
            GetUserDefinedFunctionsError::GlueEncryption(ref cause) => cause,
            GetUserDefinedFunctionsError::InternalService(ref cause) => cause,
            GetUserDefinedFunctionsError::InvalidInput(ref cause) => cause,
            GetUserDefinedFunctionsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetWorkflow
#[derive(Debug, PartialEq)]
pub enum GetWorkflowError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetWorkflowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWorkflowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetWorkflowError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetWorkflowError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetWorkflowError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetWorkflowError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetWorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetWorkflowError {
    fn description(&self) -> &str {
        match *self {
            GetWorkflowError::EntityNotFound(ref cause) => cause,
            GetWorkflowError::InternalService(ref cause) => cause,
            GetWorkflowError::InvalidInput(ref cause) => cause,
            GetWorkflowError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetWorkflowRun
#[derive(Debug, PartialEq)]
pub enum GetWorkflowRunError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetWorkflowRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWorkflowRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetWorkflowRunError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetWorkflowRunError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetWorkflowRunError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetWorkflowRunError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetWorkflowRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetWorkflowRunError {
    fn description(&self) -> &str {
        match *self {
            GetWorkflowRunError::EntityNotFound(ref cause) => cause,
            GetWorkflowRunError::InternalService(ref cause) => cause,
            GetWorkflowRunError::InvalidInput(ref cause) => cause,
            GetWorkflowRunError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetWorkflowRunProperties
#[derive(Debug, PartialEq)]
pub enum GetWorkflowRunPropertiesError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetWorkflowRunPropertiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWorkflowRunPropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetWorkflowRunPropertiesError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetWorkflowRunPropertiesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetWorkflowRunPropertiesError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetWorkflowRunPropertiesError::OperationTimeout(
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
impl fmt::Display for GetWorkflowRunPropertiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetWorkflowRunPropertiesError {
    fn description(&self) -> &str {
        match *self {
            GetWorkflowRunPropertiesError::EntityNotFound(ref cause) => cause,
            GetWorkflowRunPropertiesError::InternalService(ref cause) => cause,
            GetWorkflowRunPropertiesError::InvalidInput(ref cause) => cause,
            GetWorkflowRunPropertiesError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetWorkflowRuns
#[derive(Debug, PartialEq)]
pub enum GetWorkflowRunsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetWorkflowRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWorkflowRunsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetWorkflowRunsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetWorkflowRunsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetWorkflowRunsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetWorkflowRunsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetWorkflowRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetWorkflowRunsError {
    fn description(&self) -> &str {
        match *self {
            GetWorkflowRunsError::EntityNotFound(ref cause) => cause,
            GetWorkflowRunsError::InternalService(ref cause) => cause,
            GetWorkflowRunsError::InvalidInput(ref cause) => cause,
            GetWorkflowRunsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportCatalogToGlue
#[derive(Debug, PartialEq)]
pub enum ImportCatalogToGlueError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ImportCatalogToGlueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportCatalogToGlueError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ImportCatalogToGlueError::InternalService(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(ImportCatalogToGlueError::OperationTimeout(
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
impl fmt::Display for ImportCatalogToGlueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportCatalogToGlueError {
    fn description(&self) -> &str {
        match *self {
            ImportCatalogToGlueError::InternalService(ref cause) => cause,
            ImportCatalogToGlueError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCrawlers
#[derive(Debug, PartialEq)]
pub enum ListCrawlersError {
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ListCrawlersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCrawlersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationTimeoutException" => {
                    return RusotoError::Service(ListCrawlersError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListCrawlersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCrawlersError {
    fn description(&self) -> &str {
        match *self {
            ListCrawlersError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDevEndpoints
#[derive(Debug, PartialEq)]
pub enum ListDevEndpointsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ListDevEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDevEndpointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListDevEndpointsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListDevEndpointsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListDevEndpointsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(ListDevEndpointsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDevEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDevEndpointsError {
    fn description(&self) -> &str {
        match *self {
            ListDevEndpointsError::EntityNotFound(ref cause) => cause,
            ListDevEndpointsError::InternalService(ref cause) => cause,
            ListDevEndpointsError::InvalidInput(ref cause) => cause,
            ListDevEndpointsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListJobsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListJobsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListJobsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(ListJobsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobsError {
    fn description(&self) -> &str {
        match *self {
            ListJobsError::EntityNotFound(ref cause) => cause,
            ListJobsError::InternalService(ref cause) => cause,
            ListJobsError::InvalidInput(ref cause) => cause,
            ListJobsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTriggers
#[derive(Debug, PartialEq)]
pub enum ListTriggersError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ListTriggersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTriggersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListTriggersError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListTriggersError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListTriggersError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(ListTriggersError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTriggersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTriggersError {
    fn description(&self) -> &str {
        match *self {
            ListTriggersError::EntityNotFound(ref cause) => cause,
            ListTriggersError::InternalService(ref cause) => cause,
            ListTriggersError::InvalidInput(ref cause) => cause,
            ListTriggersError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by ListWorkflows
#[derive(Debug, PartialEq)]
pub enum ListWorkflowsError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ListWorkflowsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWorkflowsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListWorkflowsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListWorkflowsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(ListWorkflowsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListWorkflowsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWorkflowsError {
    fn description(&self) -> &str {
        match *self {
            ListWorkflowsError::InternalService(ref cause) => cause,
            ListWorkflowsError::InvalidInput(ref cause) => cause,
            ListWorkflowsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by PutDataCatalogEncryptionSettings
#[derive(Debug, PartialEq)]
pub enum PutDataCatalogEncryptionSettingsError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl PutDataCatalogEncryptionSettingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutDataCatalogEncryptionSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        PutDataCatalogEncryptionSettingsError::InternalService(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        PutDataCatalogEncryptionSettingsError::InvalidInput(err.msg),
                    )
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(
                        PutDataCatalogEncryptionSettingsError::OperationTimeout(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutDataCatalogEncryptionSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutDataCatalogEncryptionSettingsError {
    fn description(&self) -> &str {
        match *self {
            PutDataCatalogEncryptionSettingsError::InternalService(ref cause) => cause,
            PutDataCatalogEncryptionSettingsError::InvalidInput(ref cause) => cause,
            PutDataCatalogEncryptionSettingsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by PutResourcePolicy
#[derive(Debug, PartialEq)]
pub enum PutResourcePolicyError {
    /// <p>A specified condition was not satisfied.</p>
    ConditionCheckFailure(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl PutResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConditionCheckFailureException" => {
                    return RusotoError::Service(PutResourcePolicyError::ConditionCheckFailure(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(PutResourcePolicyError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(PutResourcePolicyError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PutResourcePolicyError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(PutResourcePolicyError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutResourcePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutResourcePolicyError {
    fn description(&self) -> &str {
        match *self {
            PutResourcePolicyError::ConditionCheckFailure(ref cause) => cause,
            PutResourcePolicyError::EntityNotFound(ref cause) => cause,
            PutResourcePolicyError::InternalService(ref cause) => cause,
            PutResourcePolicyError::InvalidInput(ref cause) => cause,
            PutResourcePolicyError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by PutWorkflowRunProperties
#[derive(Debug, PartialEq)]
pub enum PutWorkflowRunPropertiesError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl PutWorkflowRunPropertiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutWorkflowRunPropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(PutWorkflowRunPropertiesError::AlreadyExists(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        PutWorkflowRunPropertiesError::ConcurrentModification(err.msg),
                    )
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(PutWorkflowRunPropertiesError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(PutWorkflowRunPropertiesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PutWorkflowRunPropertiesError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(PutWorkflowRunPropertiesError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        PutWorkflowRunPropertiesError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutWorkflowRunPropertiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutWorkflowRunPropertiesError {
    fn description(&self) -> &str {
        match *self {
            PutWorkflowRunPropertiesError::AlreadyExists(ref cause) => cause,
            PutWorkflowRunPropertiesError::ConcurrentModification(ref cause) => cause,
            PutWorkflowRunPropertiesError::EntityNotFound(ref cause) => cause,
            PutWorkflowRunPropertiesError::InternalService(ref cause) => cause,
            PutWorkflowRunPropertiesError::InvalidInput(ref cause) => cause,
            PutWorkflowRunPropertiesError::OperationTimeout(ref cause) => cause,
            PutWorkflowRunPropertiesError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by ResetJobBookmark
#[derive(Debug, PartialEq)]
pub enum ResetJobBookmarkError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ResetJobBookmarkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetJobBookmarkError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ResetJobBookmarkError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ResetJobBookmarkError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ResetJobBookmarkError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(ResetJobBookmarkError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ResetJobBookmarkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetJobBookmarkError {
    fn description(&self) -> &str {
        match *self {
            ResetJobBookmarkError::EntityNotFound(ref cause) => cause,
            ResetJobBookmarkError::InternalService(ref cause) => cause,
            ResetJobBookmarkError::InvalidInput(ref cause) => cause,
            ResetJobBookmarkError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchTables
#[derive(Debug, PartialEq)]
pub enum SearchTablesError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl SearchTablesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchTablesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(SearchTablesError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(SearchTablesError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(SearchTablesError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchTablesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchTablesError {
    fn description(&self) -> &str {
        match *self {
            SearchTablesError::InternalService(ref cause) => cause,
            SearchTablesError::InvalidInput(ref cause) => cause,
            SearchTablesError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by StartCrawler
#[derive(Debug, PartialEq)]
pub enum StartCrawlerError {
    /// <p>The operation cannot be performed because the crawler is already running.</p>
    CrawlerRunning(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl StartCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CrawlerRunningException" => {
                    return RusotoError::Service(StartCrawlerError::CrawlerRunning(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartCrawlerError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartCrawlerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartCrawlerError {
    fn description(&self) -> &str {
        match *self {
            StartCrawlerError::CrawlerRunning(ref cause) => cause,
            StartCrawlerError::EntityNotFound(ref cause) => cause,
            StartCrawlerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by StartCrawlerSchedule
#[derive(Debug, PartialEq)]
pub enum StartCrawlerScheduleError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>There is no applicable schedule.</p>
    NoSchedule(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>The specified scheduler is already running.</p>
    SchedulerRunning(String),
    /// <p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
}

impl StartCrawlerScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartCrawlerScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartCrawlerScheduleError::EntityNotFound(err.msg))
                }
                "NoScheduleException" => {
                    return RusotoError::Service(StartCrawlerScheduleError::NoSchedule(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartCrawlerScheduleError::OperationTimeout(
                        err.msg,
                    ))
                }
                "SchedulerRunningException" => {
                    return RusotoError::Service(StartCrawlerScheduleError::SchedulerRunning(
                        err.msg,
                    ))
                }
                "SchedulerTransitioningException" => {
                    return RusotoError::Service(StartCrawlerScheduleError::SchedulerTransitioning(
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
impl fmt::Display for StartCrawlerScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartCrawlerScheduleError {
    fn description(&self) -> &str {
        match *self {
            StartCrawlerScheduleError::EntityNotFound(ref cause) => cause,
            StartCrawlerScheduleError::NoSchedule(ref cause) => cause,
            StartCrawlerScheduleError::OperationTimeout(ref cause) => cause,
            StartCrawlerScheduleError::SchedulerRunning(ref cause) => cause,
            StartCrawlerScheduleError::SchedulerTransitioning(ref cause) => cause,
        }
    }
}
/// Errors returned by StartExportLabelsTaskRun
#[derive(Debug, PartialEq)]
pub enum StartExportLabelsTaskRunError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl StartExportLabelsTaskRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartExportLabelsTaskRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartExportLabelsTaskRunError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartExportLabelsTaskRunError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartExportLabelsTaskRunError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartExportLabelsTaskRunError::OperationTimeout(
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
impl fmt::Display for StartExportLabelsTaskRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartExportLabelsTaskRunError {
    fn description(&self) -> &str {
        match *self {
            StartExportLabelsTaskRunError::EntityNotFound(ref cause) => cause,
            StartExportLabelsTaskRunError::InternalService(ref cause) => cause,
            StartExportLabelsTaskRunError::InvalidInput(ref cause) => cause,
            StartExportLabelsTaskRunError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by StartImportLabelsTaskRun
#[derive(Debug, PartialEq)]
pub enum StartImportLabelsTaskRunError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl StartImportLabelsTaskRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartImportLabelsTaskRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartImportLabelsTaskRunError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartImportLabelsTaskRunError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartImportLabelsTaskRunError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartImportLabelsTaskRunError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        StartImportLabelsTaskRunError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartImportLabelsTaskRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartImportLabelsTaskRunError {
    fn description(&self) -> &str {
        match *self {
            StartImportLabelsTaskRunError::EntityNotFound(ref cause) => cause,
            StartImportLabelsTaskRunError::InternalService(ref cause) => cause,
            StartImportLabelsTaskRunError::InvalidInput(ref cause) => cause,
            StartImportLabelsTaskRunError::OperationTimeout(ref cause) => cause,
            StartImportLabelsTaskRunError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by StartJobRun
#[derive(Debug, PartialEq)]
pub enum StartJobRunError {
    /// <p>Too many jobs are being run concurrently.</p>
    ConcurrentRunsExceeded(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl StartJobRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartJobRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentRunsExceededException" => {
                    return RusotoError::Service(StartJobRunError::ConcurrentRunsExceeded(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartJobRunError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartJobRunError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartJobRunError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartJobRunError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(StartJobRunError::ResourceNumberLimitExceeded(
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
impl fmt::Display for StartJobRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartJobRunError {
    fn description(&self) -> &str {
        match *self {
            StartJobRunError::ConcurrentRunsExceeded(ref cause) => cause,
            StartJobRunError::EntityNotFound(ref cause) => cause,
            StartJobRunError::InternalService(ref cause) => cause,
            StartJobRunError::InvalidInput(ref cause) => cause,
            StartJobRunError::OperationTimeout(ref cause) => cause,
            StartJobRunError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by StartMLEvaluationTaskRun
#[derive(Debug, PartialEq)]
pub enum StartMLEvaluationTaskRunError {
    /// <p>Too many jobs are being run concurrently.</p>
    ConcurrentRunsExceeded(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The machine learning transform is not ready to run.</p>
    MLTransformNotReady(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl StartMLEvaluationTaskRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartMLEvaluationTaskRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentRunsExceededException" => {
                    return RusotoError::Service(
                        StartMLEvaluationTaskRunError::ConcurrentRunsExceeded(err.msg),
                    )
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartMLEvaluationTaskRunError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartMLEvaluationTaskRunError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartMLEvaluationTaskRunError::InvalidInput(
                        err.msg,
                    ))
                }
                "MLTransformNotReadyException" => {
                    return RusotoError::Service(
                        StartMLEvaluationTaskRunError::MLTransformNotReady(err.msg),
                    )
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartMLEvaluationTaskRunError::OperationTimeout(
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
impl fmt::Display for StartMLEvaluationTaskRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartMLEvaluationTaskRunError {
    fn description(&self) -> &str {
        match *self {
            StartMLEvaluationTaskRunError::ConcurrentRunsExceeded(ref cause) => cause,
            StartMLEvaluationTaskRunError::EntityNotFound(ref cause) => cause,
            StartMLEvaluationTaskRunError::InternalService(ref cause) => cause,
            StartMLEvaluationTaskRunError::InvalidInput(ref cause) => cause,
            StartMLEvaluationTaskRunError::MLTransformNotReady(ref cause) => cause,
            StartMLEvaluationTaskRunError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by StartMLLabelingSetGenerationTaskRun
#[derive(Debug, PartialEq)]
pub enum StartMLLabelingSetGenerationTaskRunError {
    /// <p>Too many jobs are being run concurrently.</p>
    ConcurrentRunsExceeded(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl StartMLLabelingSetGenerationTaskRunError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartMLLabelingSetGenerationTaskRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentRunsExceededException" => {
                    return RusotoError::Service(
                        StartMLLabelingSetGenerationTaskRunError::ConcurrentRunsExceeded(err.msg),
                    )
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(
                        StartMLLabelingSetGenerationTaskRunError::EntityNotFound(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(
                        StartMLLabelingSetGenerationTaskRunError::InternalService(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        StartMLLabelingSetGenerationTaskRunError::InvalidInput(err.msg),
                    )
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(
                        StartMLLabelingSetGenerationTaskRunError::OperationTimeout(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartMLLabelingSetGenerationTaskRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartMLLabelingSetGenerationTaskRunError {
    fn description(&self) -> &str {
        match *self {
            StartMLLabelingSetGenerationTaskRunError::ConcurrentRunsExceeded(ref cause) => cause,
            StartMLLabelingSetGenerationTaskRunError::EntityNotFound(ref cause) => cause,
            StartMLLabelingSetGenerationTaskRunError::InternalService(ref cause) => cause,
            StartMLLabelingSetGenerationTaskRunError::InvalidInput(ref cause) => cause,
            StartMLLabelingSetGenerationTaskRunError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by StartTrigger
#[derive(Debug, PartialEq)]
pub enum StartTriggerError {
    /// <p>Too many jobs are being run concurrently.</p>
    ConcurrentRunsExceeded(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl StartTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentRunsExceededException" => {
                    return RusotoError::Service(StartTriggerError::ConcurrentRunsExceeded(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartTriggerError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartTriggerError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(StartTriggerError::ResourceNumberLimitExceeded(
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
impl fmt::Display for StartTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartTriggerError {
    fn description(&self) -> &str {
        match *self {
            StartTriggerError::ConcurrentRunsExceeded(ref cause) => cause,
            StartTriggerError::EntityNotFound(ref cause) => cause,
            StartTriggerError::InternalService(ref cause) => cause,
            StartTriggerError::InvalidInput(ref cause) => cause,
            StartTriggerError::OperationTimeout(ref cause) => cause,
            StartTriggerError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by StartWorkflowRun
#[derive(Debug, PartialEq)]
pub enum StartWorkflowRunError {
    /// <p>Too many jobs are being run concurrently.</p>
    ConcurrentRunsExceeded(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl StartWorkflowRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartWorkflowRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentRunsExceededException" => {
                    return RusotoError::Service(StartWorkflowRunError::ConcurrentRunsExceeded(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartWorkflowRunError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartWorkflowRunError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartWorkflowRunError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartWorkflowRunError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        StartWorkflowRunError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartWorkflowRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartWorkflowRunError {
    fn description(&self) -> &str {
        match *self {
            StartWorkflowRunError::ConcurrentRunsExceeded(ref cause) => cause,
            StartWorkflowRunError::EntityNotFound(ref cause) => cause,
            StartWorkflowRunError::InternalService(ref cause) => cause,
            StartWorkflowRunError::InvalidInput(ref cause) => cause,
            StartWorkflowRunError::OperationTimeout(ref cause) => cause,
            StartWorkflowRunError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by StopCrawler
#[derive(Debug, PartialEq)]
pub enum StopCrawlerError {
    /// <p>The specified crawler is not running.</p>
    CrawlerNotRunning(String),
    /// <p>The specified crawler is stopping.</p>
    CrawlerStopping(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl StopCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CrawlerNotRunningException" => {
                    return RusotoError::Service(StopCrawlerError::CrawlerNotRunning(err.msg))
                }
                "CrawlerStoppingException" => {
                    return RusotoError::Service(StopCrawlerError::CrawlerStopping(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StopCrawlerError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StopCrawlerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopCrawlerError {
    fn description(&self) -> &str {
        match *self {
            StopCrawlerError::CrawlerNotRunning(ref cause) => cause,
            StopCrawlerError::CrawlerStopping(ref cause) => cause,
            StopCrawlerError::EntityNotFound(ref cause) => cause,
            StopCrawlerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by StopCrawlerSchedule
#[derive(Debug, PartialEq)]
pub enum StopCrawlerScheduleError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>The specified scheduler is not running.</p>
    SchedulerNotRunning(String),
    /// <p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
}

impl StopCrawlerScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopCrawlerScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(StopCrawlerScheduleError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StopCrawlerScheduleError::OperationTimeout(
                        err.msg,
                    ))
                }
                "SchedulerNotRunningException" => {
                    return RusotoError::Service(StopCrawlerScheduleError::SchedulerNotRunning(
                        err.msg,
                    ))
                }
                "SchedulerTransitioningException" => {
                    return RusotoError::Service(StopCrawlerScheduleError::SchedulerTransitioning(
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
impl fmt::Display for StopCrawlerScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopCrawlerScheduleError {
    fn description(&self) -> &str {
        match *self {
            StopCrawlerScheduleError::EntityNotFound(ref cause) => cause,
            StopCrawlerScheduleError::OperationTimeout(ref cause) => cause,
            StopCrawlerScheduleError::SchedulerNotRunning(ref cause) => cause,
            StopCrawlerScheduleError::SchedulerTransitioning(ref cause) => cause,
        }
    }
}
/// Errors returned by StopTrigger
#[derive(Debug, PartialEq)]
pub enum StopTriggerError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl StopTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(StopTriggerError::ConcurrentModification(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StopTriggerError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StopTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StopTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StopTriggerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopTriggerError {
    fn description(&self) -> &str {
        match *self {
            StopTriggerError::ConcurrentModification(ref cause) => cause,
            StopTriggerError::EntityNotFound(ref cause) => cause,
            StopTriggerError::InternalService(ref cause) => cause,
            StopTriggerError::InvalidInput(ref cause) => cause,
            StopTriggerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(TagResourceError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(TagResourceError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(TagResourceError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(TagResourceError::OperationTimeout(err.msg))
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
            TagResourceError::EntityNotFound(ref cause) => cause,
            TagResourceError::InternalService(ref cause) => cause,
            TagResourceError::InvalidInput(ref cause) => cause,
            TagResourceError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UntagResourceError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UntagResourceError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UntagResourceError::OperationTimeout(err.msg))
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
            UntagResourceError::EntityNotFound(ref cause) => cause,
            UntagResourceError::InternalService(ref cause) => cause,
            UntagResourceError::InvalidInput(ref cause) => cause,
            UntagResourceError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateClassifier
#[derive(Debug, PartialEq)]
pub enum UpdateClassifierError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>There was a version conflict.</p>
    VersionMismatch(String),
}

impl UpdateClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateClassifierError::EntityNotFound(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateClassifierError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateClassifierError::OperationTimeout(err.msg))
                }
                "VersionMismatchException" => {
                    return RusotoError::Service(UpdateClassifierError::VersionMismatch(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateClassifierError {
    fn description(&self) -> &str {
        match *self {
            UpdateClassifierError::EntityNotFound(ref cause) => cause,
            UpdateClassifierError::InvalidInput(ref cause) => cause,
            UpdateClassifierError::OperationTimeout(ref cause) => cause,
            UpdateClassifierError::VersionMismatch(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConnection
#[derive(Debug, PartialEq)]
pub enum UpdateConnectionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateConnectionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(UpdateConnectionError::GlueEncryption(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateConnectionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateConnectionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConnectionError {
    fn description(&self) -> &str {
        match *self {
            UpdateConnectionError::EntityNotFound(ref cause) => cause,
            UpdateConnectionError::GlueEncryption(ref cause) => cause,
            UpdateConnectionError::InvalidInput(ref cause) => cause,
            UpdateConnectionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCrawler
#[derive(Debug, PartialEq)]
pub enum UpdateCrawlerError {
    /// <p>The operation cannot be performed because the crawler is already running.</p>
    CrawlerRunning(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>There was a version conflict.</p>
    VersionMismatch(String),
}

impl UpdateCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CrawlerRunningException" => {
                    return RusotoError::Service(UpdateCrawlerError::CrawlerRunning(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateCrawlerError::EntityNotFound(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateCrawlerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateCrawlerError::OperationTimeout(err.msg))
                }
                "VersionMismatchException" => {
                    return RusotoError::Service(UpdateCrawlerError::VersionMismatch(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCrawlerError {
    fn description(&self) -> &str {
        match *self {
            UpdateCrawlerError::CrawlerRunning(ref cause) => cause,
            UpdateCrawlerError::EntityNotFound(ref cause) => cause,
            UpdateCrawlerError::InvalidInput(ref cause) => cause,
            UpdateCrawlerError::OperationTimeout(ref cause) => cause,
            UpdateCrawlerError::VersionMismatch(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCrawlerSchedule
#[derive(Debug, PartialEq)]
pub enum UpdateCrawlerScheduleError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
    /// <p>There was a version conflict.</p>
    VersionMismatch(String),
}

impl UpdateCrawlerScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCrawlerScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateCrawlerScheduleError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateCrawlerScheduleError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateCrawlerScheduleError::OperationTimeout(
                        err.msg,
                    ))
                }
                "SchedulerTransitioningException" => {
                    return RusotoError::Service(
                        UpdateCrawlerScheduleError::SchedulerTransitioning(err.msg),
                    )
                }
                "VersionMismatchException" => {
                    return RusotoError::Service(UpdateCrawlerScheduleError::VersionMismatch(
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
impl fmt::Display for UpdateCrawlerScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCrawlerScheduleError {
    fn description(&self) -> &str {
        match *self {
            UpdateCrawlerScheduleError::EntityNotFound(ref cause) => cause,
            UpdateCrawlerScheduleError::InvalidInput(ref cause) => cause,
            UpdateCrawlerScheduleError::OperationTimeout(ref cause) => cause,
            UpdateCrawlerScheduleError::SchedulerTransitioning(ref cause) => cause,
            UpdateCrawlerScheduleError::VersionMismatch(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDatabase
#[derive(Debug, PartialEq)]
pub enum UpdateDatabaseError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateDatabaseError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(UpdateDatabaseError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateDatabaseError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateDatabaseError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateDatabaseError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDatabaseError {
    fn description(&self) -> &str {
        match *self {
            UpdateDatabaseError::EntityNotFound(ref cause) => cause,
            UpdateDatabaseError::GlueEncryption(ref cause) => cause,
            UpdateDatabaseError::InternalService(ref cause) => cause,
            UpdateDatabaseError::InvalidInput(ref cause) => cause,
            UpdateDatabaseError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDevEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateDevEndpointError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateDevEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDevEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateDevEndpointError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateDevEndpointError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateDevEndpointError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateDevEndpointError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            UpdateDevEndpointError::EntityNotFound(ref cause) => cause,
            UpdateDevEndpointError::InternalService(ref cause) => cause,
            UpdateDevEndpointError::InvalidInput(ref cause) => cause,
            UpdateDevEndpointError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateJob
#[derive(Debug, PartialEq)]
pub enum UpdateJobError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateJobError::ConcurrentModification(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateJobError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateJobError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateJobError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateJobError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateJobError {
    fn description(&self) -> &str {
        match *self {
            UpdateJobError::ConcurrentModification(ref cause) => cause,
            UpdateJobError::EntityNotFound(ref cause) => cause,
            UpdateJobError::InternalService(ref cause) => cause,
            UpdateJobError::InvalidInput(ref cause) => cause,
            UpdateJobError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMLTransform
#[derive(Debug, PartialEq)]
pub enum UpdateMLTransformError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateMLTransformError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMLTransformError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateMLTransformError::AccessDenied(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateMLTransformError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateMLTransformError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateMLTransformError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateMLTransformError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateMLTransformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMLTransformError {
    fn description(&self) -> &str {
        match *self {
            UpdateMLTransformError::AccessDenied(ref cause) => cause,
            UpdateMLTransformError::EntityNotFound(ref cause) => cause,
            UpdateMLTransformError::InternalService(ref cause) => cause,
            UpdateMLTransformError::InvalidInput(ref cause) => cause,
            UpdateMLTransformError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePartition
#[derive(Debug, PartialEq)]
pub enum UpdatePartitionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdatePartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdatePartitionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(UpdatePartitionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdatePartitionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdatePartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdatePartitionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdatePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePartitionError {
    fn description(&self) -> &str {
        match *self {
            UpdatePartitionError::EntityNotFound(ref cause) => cause,
            UpdatePartitionError::GlueEncryption(ref cause) => cause,
            UpdatePartitionError::InternalService(ref cause) => cause,
            UpdatePartitionError::InvalidInput(ref cause) => cause,
            UpdatePartitionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTable
#[derive(Debug, PartialEq)]
pub enum UpdateTableError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl UpdateTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateTableError::ConcurrentModification(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateTableError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(UpdateTableError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateTableError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateTableError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateTableError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(UpdateTableError::ResourceNumberLimitExceeded(
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
impl fmt::Display for UpdateTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTableError {
    fn description(&self) -> &str {
        match *self {
            UpdateTableError::ConcurrentModification(ref cause) => cause,
            UpdateTableError::EntityNotFound(ref cause) => cause,
            UpdateTableError::GlueEncryption(ref cause) => cause,
            UpdateTableError::InternalService(ref cause) => cause,
            UpdateTableError::InvalidInput(ref cause) => cause,
            UpdateTableError::OperationTimeout(ref cause) => cause,
            UpdateTableError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTrigger
#[derive(Debug, PartialEq)]
pub enum UpdateTriggerError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateTriggerError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateTriggerError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateTriggerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTriggerError {
    fn description(&self) -> &str {
        match *self {
            UpdateTriggerError::ConcurrentModification(ref cause) => cause,
            UpdateTriggerError::EntityNotFound(ref cause) => cause,
            UpdateTriggerError::InternalService(ref cause) => cause,
            UpdateTriggerError::InvalidInput(ref cause) => cause,
            UpdateTriggerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum UpdateUserDefinedFunctionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateUserDefinedFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserDefinedFunctionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateUserDefinedFunctionError::EntityNotFound(
                        err.msg,
                    ))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(UpdateUserDefinedFunctionError::GlueEncryption(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserDefinedFunctionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateUserDefinedFunctionError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateUserDefinedFunctionError::OperationTimeout(
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
impl fmt::Display for UpdateUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            UpdateUserDefinedFunctionError::GlueEncryption(ref cause) => cause,
            UpdateUserDefinedFunctionError::InternalService(ref cause) => cause,
            UpdateUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            UpdateUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateWorkflow
#[derive(Debug, PartialEq)]
pub enum UpdateWorkflowError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateWorkflowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateWorkflowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateWorkflowError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateWorkflowError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateWorkflowError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateWorkflowError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateWorkflowError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateWorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateWorkflowError {
    fn description(&self) -> &str {
        match *self {
            UpdateWorkflowError::ConcurrentModification(ref cause) => cause,
            UpdateWorkflowError::EntityNotFound(ref cause) => cause,
            UpdateWorkflowError::InternalService(ref cause) => cause,
            UpdateWorkflowError::InvalidInput(ref cause) => cause,
            UpdateWorkflowError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Glue API. AWS Glue clients implement this trait.
pub trait Glue {
    /// <p>Creates one or more partitions in a batch operation.</p>
    fn batch_create_partition(
        &self,
        input: BatchCreatePartitionRequest,
    ) -> RusotoFuture<BatchCreatePartitionResponse, BatchCreatePartitionError>;

    /// <p>Deletes a list of connection definitions from the Data Catalog.</p>
    fn batch_delete_connection(
        &self,
        input: BatchDeleteConnectionRequest,
    ) -> RusotoFuture<BatchDeleteConnectionResponse, BatchDeleteConnectionError>;

    /// <p>Deletes one or more partitions in a batch operation.</p>
    fn batch_delete_partition(
        &self,
        input: BatchDeletePartitionRequest,
    ) -> RusotoFuture<BatchDeletePartitionResponse, BatchDeletePartitionError>;

    /// <p><p>Deletes multiple tables at once.</p> <note> <p>After completing this operation, you no longer have access to the table versions and partitions that belong to the deleted table. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure the immediate deletion of all related resources, before calling <code>BatchDeleteTable</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, and <code>DeletePartition</code> or <code>BatchDeletePartition</code>, to delete any resources that belong to the table.</p> </note></p>
    fn batch_delete_table(
        &self,
        input: BatchDeleteTableRequest,
    ) -> RusotoFuture<BatchDeleteTableResponse, BatchDeleteTableError>;

    /// <p>Deletes a specified batch of versions of a table.</p>
    fn batch_delete_table_version(
        &self,
        input: BatchDeleteTableVersionRequest,
    ) -> RusotoFuture<BatchDeleteTableVersionResponse, BatchDeleteTableVersionError>;

    /// <p>Returns a list of resource metadata for a given list of crawler names. After calling the <code>ListCrawlers</code> operation, you can call this operation to access the data to which you have been granted permissions. This operation supports all IAM permissions, including permission conditions that uses tags.</p>
    fn batch_get_crawlers(
        &self,
        input: BatchGetCrawlersRequest,
    ) -> RusotoFuture<BatchGetCrawlersResponse, BatchGetCrawlersError>;

    /// <p>Returns a list of resource metadata for a given list of development endpoint names. After calling the <code>ListDevEndpoints</code> operation, you can call this operation to access the data to which you have been granted permissions. This operation supports all IAM permissions, including permission conditions that uses tags.</p>
    fn batch_get_dev_endpoints(
        &self,
        input: BatchGetDevEndpointsRequest,
    ) -> RusotoFuture<BatchGetDevEndpointsResponse, BatchGetDevEndpointsError>;

    /// <p>Returns a list of resource metadata for a given list of job names. After calling the <code>ListJobs</code> operation, you can call this operation to access the data to which you have been granted permissions. This operation supports all IAM permissions, including permission conditions that uses tags. </p>
    fn batch_get_jobs(
        &self,
        input: BatchGetJobsRequest,
    ) -> RusotoFuture<BatchGetJobsResponse, BatchGetJobsError>;

    /// <p>Retrieves partitions in a batch request.</p>
    fn batch_get_partition(
        &self,
        input: BatchGetPartitionRequest,
    ) -> RusotoFuture<BatchGetPartitionResponse, BatchGetPartitionError>;

    /// <p>Returns a list of resource metadata for a given list of trigger names. After calling the <code>ListTriggers</code> operation, you can call this operation to access the data to which you have been granted permissions. This operation supports all IAM permissions, including permission conditions that uses tags.</p>
    fn batch_get_triggers(
        &self,
        input: BatchGetTriggersRequest,
    ) -> RusotoFuture<BatchGetTriggersResponse, BatchGetTriggersError>;

    /// <p>Returns a list of resource metadata for a given list of workflow names. After calling the <code>ListWorkflows</code> operation, you can call this operation to access the data to which you have been granted permissions. This operation supports all IAM permissions, including permission conditions that uses tags.</p>
    fn batch_get_workflows(
        &self,
        input: BatchGetWorkflowsRequest,
    ) -> RusotoFuture<BatchGetWorkflowsResponse, BatchGetWorkflowsError>;

    /// <p>Stops one or more job runs for a specified job definition.</p>
    fn batch_stop_job_run(
        &self,
        input: BatchStopJobRunRequest,
    ) -> RusotoFuture<BatchStopJobRunResponse, GlueBatchStopJobRunError>;

    /// <p>Cancels (stops) a task run. Machine learning task runs are asynchronous tasks that AWS Glue runs on your behalf as part of various machine learning workflows. You can cancel a machine learning task run at any time by calling <code>CancelMLTaskRun</code> with a task run's parent transform's <code>TransformID</code> and the task run's <code>TaskRunId</code>. </p>
    fn cancel_ml_task_run(
        &self,
        input: CancelMLTaskRunRequest,
    ) -> RusotoFuture<CancelMLTaskRunResponse, CancelMLTaskRunError>;

    /// <p>Creates a classifier in the user's account. This can be a <code>GrokClassifier</code>, an <code>XMLClassifier</code>, a <code>JsonClassifier</code>, or a <code>CsvClassifier</code>, depending on which field of the request is present.</p>
    fn create_classifier(
        &self,
        input: CreateClassifierRequest,
    ) -> RusotoFuture<CreateClassifierResponse, CreateClassifierError>;

    /// <p>Creates a connection definition in the Data Catalog.</p>
    fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> RusotoFuture<CreateConnectionResponse, CreateConnectionError>;

    /// <p>Creates a new crawler with specified targets, role, configuration, and optional schedule. At least one crawl target must be specified, in the <code>s3Targets</code> field, the <code>jdbcTargets</code> field, or the <code>DynamoDBTargets</code> field.</p>
    fn create_crawler(
        &self,
        input: CreateCrawlerRequest,
    ) -> RusotoFuture<CreateCrawlerResponse, CreateCrawlerError>;

    /// <p>Creates a new database in a Data Catalog.</p>
    fn create_database(
        &self,
        input: CreateDatabaseRequest,
    ) -> RusotoFuture<CreateDatabaseResponse, CreateDatabaseError>;

    /// <p>Creates a new development endpoint.</p>
    fn create_dev_endpoint(
        &self,
        input: CreateDevEndpointRequest,
    ) -> RusotoFuture<CreateDevEndpointResponse, CreateDevEndpointError>;

    /// <p>Creates a new job definition.</p>
    fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> RusotoFuture<CreateJobResponse, CreateJobError>;

    /// <p>Creates an AWS Glue machine learning transform. This operation creates the transform and all the necessary parameters to train it.</p> <p>Call this operation as the first step in the process of using a machine learning transform (such as the <code>FindMatches</code> transform) for deduplicating data. You can provide an optional <code>Description</code>, in addition to the parameters that you want to use for your algorithm.</p> <p>You must also specify certain parameters for the tasks that AWS Glue runs on your behalf as part of learning from your data and creating a high-quality machine learning transform. These parameters include <code>Role</code>, and optionally, <code>AllocatedCapacity</code>, <code>Timeout</code>, and <code>MaxRetries</code>. For more information, see <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-jobs-job.html">Jobs</a>.</p>
    fn create_ml_transform(
        &self,
        input: CreateMLTransformRequest,
    ) -> RusotoFuture<CreateMLTransformResponse, CreateMLTransformError>;

    /// <p>Creates a new partition.</p>
    fn create_partition(
        &self,
        input: CreatePartitionRequest,
    ) -> RusotoFuture<CreatePartitionResponse, CreatePartitionError>;

    /// <p>Transforms a directed acyclic graph (DAG) into code.</p>
    fn create_script(
        &self,
        input: CreateScriptRequest,
    ) -> RusotoFuture<CreateScriptResponse, CreateScriptError>;

    /// <p>Creates a new security configuration. A security configuration is a set of security properties that can be used by AWS Glue. You can use a security configuration to encrypt data at rest. For information about using security configurations in AWS Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/encryption-security-configuration.html">Encrypting Data Written by Crawlers, Jobs, and Development Endpoints</a>.</p>
    fn create_security_configuration(
        &self,
        input: CreateSecurityConfigurationRequest,
    ) -> RusotoFuture<CreateSecurityConfigurationResponse, CreateSecurityConfigurationError>;

    /// <p>Creates a new table definition in the Data Catalog.</p>
    fn create_table(
        &self,
        input: CreateTableRequest,
    ) -> RusotoFuture<CreateTableResponse, CreateTableError>;

    /// <p>Creates a new trigger.</p>
    fn create_trigger(
        &self,
        input: CreateTriggerRequest,
    ) -> RusotoFuture<CreateTriggerResponse, CreateTriggerError>;

    /// <p>Creates a new function definition in the Data Catalog.</p>
    fn create_user_defined_function(
        &self,
        input: CreateUserDefinedFunctionRequest,
    ) -> RusotoFuture<CreateUserDefinedFunctionResponse, CreateUserDefinedFunctionError>;

    /// <p>Creates a new workflow.</p>
    fn create_workflow(
        &self,
        input: CreateWorkflowRequest,
    ) -> RusotoFuture<CreateWorkflowResponse, CreateWorkflowError>;

    /// <p>Removes a classifier from the Data Catalog.</p>
    fn delete_classifier(
        &self,
        input: DeleteClassifierRequest,
    ) -> RusotoFuture<DeleteClassifierResponse, DeleteClassifierError>;

    /// <p>Deletes a connection from the Data Catalog.</p>
    fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> RusotoFuture<DeleteConnectionResponse, DeleteConnectionError>;

    /// <p>Removes a specified crawler from the AWS Glue Data Catalog, unless the crawler state is <code>RUNNING</code>.</p>
    fn delete_crawler(
        &self,
        input: DeleteCrawlerRequest,
    ) -> RusotoFuture<DeleteCrawlerResponse, DeleteCrawlerError>;

    /// <p><p>Removes a specified database from a Data Catalog.</p> <note> <p>After completing this operation, you no longer have access to the tables (and all table versions and partitions that might belong to the tables) and the user-defined functions in the deleted database. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure the immediate deletion of all related resources, before calling <code>DeleteDatabase</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, <code>DeletePartition</code> or <code>BatchDeletePartition</code>, <code>DeleteUserDefinedFunction</code>, and <code>DeleteTable</code> or <code>BatchDeleteTable</code>, to delete any resources that belong to the database.</p> </note></p>
    fn delete_database(
        &self,
        input: DeleteDatabaseRequest,
    ) -> RusotoFuture<DeleteDatabaseResponse, DeleteDatabaseError>;

    /// <p>Deletes a specified development endpoint.</p>
    fn delete_dev_endpoint(
        &self,
        input: DeleteDevEndpointRequest,
    ) -> RusotoFuture<DeleteDevEndpointResponse, DeleteDevEndpointError>;

    /// <p>Deletes a specified job definition. If the job definition is not found, no exception is thrown.</p>
    fn delete_job(
        &self,
        input: DeleteJobRequest,
    ) -> RusotoFuture<DeleteJobResponse, DeleteJobError>;

    /// <p>Deletes an AWS Glue machine learning transform. Machine learning transforms are a special type of transform that use machine learning to learn the details of the transformation to be performed by learning from examples provided by humans. These transformations are then saved by AWS Glue. If you no longer need a transform, you can delete it by calling <code>DeleteMLTransforms</code>. However, any AWS Glue jobs that still reference the deleted transform will no longer succeed.</p>
    fn delete_ml_transform(
        &self,
        input: DeleteMLTransformRequest,
    ) -> RusotoFuture<DeleteMLTransformResponse, DeleteMLTransformError>;

    /// <p>Deletes a specified partition.</p>
    fn delete_partition(
        &self,
        input: DeletePartitionRequest,
    ) -> RusotoFuture<DeletePartitionResponse, DeletePartitionError>;

    /// <p>Deletes a specified policy.</p>
    fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyRequest,
    ) -> RusotoFuture<DeleteResourcePolicyResponse, DeleteResourcePolicyError>;

    /// <p>Deletes a specified security configuration.</p>
    fn delete_security_configuration(
        &self,
        input: DeleteSecurityConfigurationRequest,
    ) -> RusotoFuture<DeleteSecurityConfigurationResponse, DeleteSecurityConfigurationError>;

    /// <p><p>Removes a table definition from the Data Catalog.</p> <note> <p>After completing this operation, you no longer have access to the table versions and partitions that belong to the deleted table. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure the immediate deletion of all related resources, before calling <code>DeleteTable</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, and <code>DeletePartition</code> or <code>BatchDeletePartition</code>, to delete any resources that belong to the table.</p> </note></p>
    fn delete_table(
        &self,
        input: DeleteTableRequest,
    ) -> RusotoFuture<DeleteTableResponse, DeleteTableError>;

    /// <p>Deletes a specified version of a table.</p>
    fn delete_table_version(
        &self,
        input: DeleteTableVersionRequest,
    ) -> RusotoFuture<DeleteTableVersionResponse, DeleteTableVersionError>;

    /// <p>Deletes a specified trigger. If the trigger is not found, no exception is thrown.</p>
    fn delete_trigger(
        &self,
        input: DeleteTriggerRequest,
    ) -> RusotoFuture<DeleteTriggerResponse, DeleteTriggerError>;

    /// <p>Deletes an existing function definition from the Data Catalog.</p>
    fn delete_user_defined_function(
        &self,
        input: DeleteUserDefinedFunctionRequest,
    ) -> RusotoFuture<DeleteUserDefinedFunctionResponse, DeleteUserDefinedFunctionError>;

    /// <p>Deletes a workflow.</p>
    fn delete_workflow(
        &self,
        input: DeleteWorkflowRequest,
    ) -> RusotoFuture<DeleteWorkflowResponse, DeleteWorkflowError>;

    /// <p>Retrieves the status of a migration operation.</p>
    fn get_catalog_import_status(
        &self,
        input: GetCatalogImportStatusRequest,
    ) -> RusotoFuture<GetCatalogImportStatusResponse, GetCatalogImportStatusError>;

    /// <p>Retrieve a classifier by name.</p>
    fn get_classifier(
        &self,
        input: GetClassifierRequest,
    ) -> RusotoFuture<GetClassifierResponse, GetClassifierError>;

    /// <p>Lists all classifier objects in the Data Catalog.</p>
    fn get_classifiers(
        &self,
        input: GetClassifiersRequest,
    ) -> RusotoFuture<GetClassifiersResponse, GetClassifiersError>;

    /// <p>Retrieves a connection definition from the Data Catalog.</p>
    fn get_connection(
        &self,
        input: GetConnectionRequest,
    ) -> RusotoFuture<GetConnectionResponse, GetConnectionError>;

    /// <p>Retrieves a list of connection definitions from the Data Catalog.</p>
    fn get_connections(
        &self,
        input: GetConnectionsRequest,
    ) -> RusotoFuture<GetConnectionsResponse, GetConnectionsError>;

    /// <p>Retrieves metadata for a specified crawler.</p>
    fn get_crawler(
        &self,
        input: GetCrawlerRequest,
    ) -> RusotoFuture<GetCrawlerResponse, GetCrawlerError>;

    /// <p>Retrieves metrics about specified crawlers.</p>
    fn get_crawler_metrics(
        &self,
        input: GetCrawlerMetricsRequest,
    ) -> RusotoFuture<GetCrawlerMetricsResponse, GetCrawlerMetricsError>;

    /// <p>Retrieves metadata for all crawlers defined in the customer account.</p>
    fn get_crawlers(
        &self,
        input: GetCrawlersRequest,
    ) -> RusotoFuture<GetCrawlersResponse, GetCrawlersError>;

    /// <p>Retrieves the security configuration for a specified catalog.</p>
    fn get_data_catalog_encryption_settings(
        &self,
        input: GetDataCatalogEncryptionSettingsRequest,
    ) -> RusotoFuture<GetDataCatalogEncryptionSettingsResponse, GetDataCatalogEncryptionSettingsError>;

    /// <p>Retrieves the definition of a specified database.</p>
    fn get_database(
        &self,
        input: GetDatabaseRequest,
    ) -> RusotoFuture<GetDatabaseResponse, GetDatabaseError>;

    /// <p>Retrieves all databases defined in a given Data Catalog.</p>
    fn get_databases(
        &self,
        input: GetDatabasesRequest,
    ) -> RusotoFuture<GetDatabasesResponse, GetDatabasesError>;

    /// <p>Transforms a Python script into a directed acyclic graph (DAG). </p>
    fn get_dataflow_graph(
        &self,
        input: GetDataflowGraphRequest,
    ) -> RusotoFuture<GetDataflowGraphResponse, GetDataflowGraphError>;

    /// <p><p>Retrieves information about a specified development endpoint.</p> <note> <p>When you create a development endpoint in a virtual private cloud (VPC), AWS Glue returns only a private IP address, and the public IP address field is not populated. When you create a non-VPC development endpoint, AWS Glue returns only a public IP address.</p> </note></p>
    fn get_dev_endpoint(
        &self,
        input: GetDevEndpointRequest,
    ) -> RusotoFuture<GetDevEndpointResponse, GetDevEndpointError>;

    /// <p><p>Retrieves all the development endpoints in this AWS account.</p> <note> <p>When you create a development endpoint in a virtual private cloud (VPC), AWS Glue returns only a private IP address and the public IP address field is not populated. When you create a non-VPC development endpoint, AWS Glue returns only a public IP address.</p> </note></p>
    fn get_dev_endpoints(
        &self,
        input: GetDevEndpointsRequest,
    ) -> RusotoFuture<GetDevEndpointsResponse, GetDevEndpointsError>;

    /// <p>Retrieves an existing job definition.</p>
    fn get_job(&self, input: GetJobRequest) -> RusotoFuture<GetJobResponse, GetJobError>;

    /// <p>Returns information on a job bookmark entry.</p>
    fn get_job_bookmark(
        &self,
        input: GetJobBookmarkRequest,
    ) -> RusotoFuture<GetJobBookmarkResponse, GetJobBookmarkError>;

    /// <p>Retrieves the metadata for a given job run.</p>
    fn get_job_run(
        &self,
        input: GetJobRunRequest,
    ) -> RusotoFuture<GetJobRunResponse, GetJobRunError>;

    /// <p>Retrieves metadata for all runs of a given job definition.</p>
    fn get_job_runs(
        &self,
        input: GetJobRunsRequest,
    ) -> RusotoFuture<GetJobRunsResponse, GetJobRunsError>;

    /// <p>Retrieves all current job definitions.</p>
    fn get_jobs(&self, input: GetJobsRequest) -> RusotoFuture<GetJobsResponse, GetJobsError>;

    /// <p>Gets details for a specific task run on a machine learning transform. Machine learning task runs are asynchronous tasks that AWS Glue runs on your behalf as part of various machine learning workflows. You can check the stats of any task run by calling <code>GetMLTaskRun</code> with the <code>TaskRunID</code> and its parent transform's <code>TransformID</code>.</p>
    fn get_ml_task_run(
        &self,
        input: GetMLTaskRunRequest,
    ) -> RusotoFuture<GetMLTaskRunResponse, GetMLTaskRunError>;

    /// <p>Gets a list of runs for a machine learning transform. Machine learning task runs are asynchronous tasks that AWS Glue runs on your behalf as part of various machine learning workflows. You can get a sortable, filterable list of machine learning task runs by calling <code>GetMLTaskRuns</code> with their parent transform's <code>TransformID</code> and other optional parameters as documented in this section.</p> <p>This operation returns a list of historic runs and must be paginated.</p>
    fn get_ml_task_runs(
        &self,
        input: GetMLTaskRunsRequest,
    ) -> RusotoFuture<GetMLTaskRunsResponse, GetMLTaskRunsError>;

    /// <p>Gets an AWS Glue machine learning transform artifact and all its corresponding metadata. Machine learning transforms are a special type of transform that use machine learning to learn the details of the transformation to be performed by learning from examples provided by humans. These transformations are then saved by AWS Glue. You can retrieve their metadata by calling <code>GetMLTransform</code>.</p>
    fn get_ml_transform(
        &self,
        input: GetMLTransformRequest,
    ) -> RusotoFuture<GetMLTransformResponse, GetMLTransformError>;

    /// <p>Gets a sortable, filterable list of existing AWS Glue machine learning transforms. Machine learning transforms are a special type of transform that use machine learning to learn the details of the transformation to be performed by learning from examples provided by humans. These transformations are then saved by AWS Glue, and you can retrieve their metadata by calling <code>GetMLTransforms</code>.</p>
    fn get_ml_transforms(
        &self,
        input: GetMLTransformsRequest,
    ) -> RusotoFuture<GetMLTransformsResponse, GetMLTransformsError>;

    /// <p>Creates mappings.</p>
    fn get_mapping(
        &self,
        input: GetMappingRequest,
    ) -> RusotoFuture<GetMappingResponse, GetMappingError>;

    /// <p>Retrieves information about a specified partition.</p>
    fn get_partition(
        &self,
        input: GetPartitionRequest,
    ) -> RusotoFuture<GetPartitionResponse, GetPartitionError>;

    /// <p>Retrieves information about the partitions in a table.</p>
    fn get_partitions(
        &self,
        input: GetPartitionsRequest,
    ) -> RusotoFuture<GetPartitionsResponse, GetPartitionsError>;

    /// <p>Gets code to perform a specified mapping.</p>
    fn get_plan(&self, input: GetPlanRequest) -> RusotoFuture<GetPlanResponse, GetPlanError>;

    /// <p>Retrieves a specified resource policy.</p>
    fn get_resource_policy(
        &self,
    ) -> RusotoFuture<GetResourcePolicyResponse, GetResourcePolicyError>;

    /// <p>Retrieves a specified security configuration.</p>
    fn get_security_configuration(
        &self,
        input: GetSecurityConfigurationRequest,
    ) -> RusotoFuture<GetSecurityConfigurationResponse, GetSecurityConfigurationError>;

    /// <p>Retrieves a list of all security configurations.</p>
    fn get_security_configurations(
        &self,
        input: GetSecurityConfigurationsRequest,
    ) -> RusotoFuture<GetSecurityConfigurationsResponse, GetSecurityConfigurationsError>;

    /// <p>Retrieves the <code>Table</code> definition in a Data Catalog for a specified table.</p>
    fn get_table(&self, input: GetTableRequest) -> RusotoFuture<GetTableResponse, GetTableError>;

    /// <p>Retrieves a specified version of a table.</p>
    fn get_table_version(
        &self,
        input: GetTableVersionRequest,
    ) -> RusotoFuture<GetTableVersionResponse, GetTableVersionError>;

    /// <p>Retrieves a list of strings that identify available versions of a specified table.</p>
    fn get_table_versions(
        &self,
        input: GetTableVersionsRequest,
    ) -> RusotoFuture<GetTableVersionsResponse, GetTableVersionsError>;

    /// <p>Retrieves the definitions of some or all of the tables in a given <code>Database</code>.</p>
    fn get_tables(
        &self,
        input: GetTablesRequest,
    ) -> RusotoFuture<GetTablesResponse, GetTablesError>;

    /// <p>Retrieves a list of tags associated with a resource.</p>
    fn get_tags(&self, input: GetTagsRequest) -> RusotoFuture<GetTagsResponse, GetTagsError>;

    /// <p>Retrieves the definition of a trigger.</p>
    fn get_trigger(
        &self,
        input: GetTriggerRequest,
    ) -> RusotoFuture<GetTriggerResponse, GetTriggerError>;

    /// <p>Gets all the triggers associated with a job.</p>
    fn get_triggers(
        &self,
        input: GetTriggersRequest,
    ) -> RusotoFuture<GetTriggersResponse, GetTriggersError>;

    /// <p>Retrieves a specified function definition from the Data Catalog.</p>
    fn get_user_defined_function(
        &self,
        input: GetUserDefinedFunctionRequest,
    ) -> RusotoFuture<GetUserDefinedFunctionResponse, GetUserDefinedFunctionError>;

    /// <p>Retrieves multiple function definitions from the Data Catalog.</p>
    fn get_user_defined_functions(
        &self,
        input: GetUserDefinedFunctionsRequest,
    ) -> RusotoFuture<GetUserDefinedFunctionsResponse, GetUserDefinedFunctionsError>;

    /// <p>Retrieves resource metadata for a workflow.</p>
    fn get_workflow(
        &self,
        input: GetWorkflowRequest,
    ) -> RusotoFuture<GetWorkflowResponse, GetWorkflowError>;

    /// <p>Retrieves the metadata for a given workflow run. </p>
    fn get_workflow_run(
        &self,
        input: GetWorkflowRunRequest,
    ) -> RusotoFuture<GetWorkflowRunResponse, GetWorkflowRunError>;

    /// <p>Retrieves the workflow run properties which were set during the run.</p>
    fn get_workflow_run_properties(
        &self,
        input: GetWorkflowRunPropertiesRequest,
    ) -> RusotoFuture<GetWorkflowRunPropertiesResponse, GetWorkflowRunPropertiesError>;

    /// <p>Retrieves metadata for all runs of a given workflow.</p>
    fn get_workflow_runs(
        &self,
        input: GetWorkflowRunsRequest,
    ) -> RusotoFuture<GetWorkflowRunsResponse, GetWorkflowRunsError>;

    /// <p>Imports an existing Amazon Athena Data Catalog to AWS Glue</p>
    fn import_catalog_to_glue(
        &self,
        input: ImportCatalogToGlueRequest,
    ) -> RusotoFuture<ImportCatalogToGlueResponse, ImportCatalogToGlueError>;

    /// <p>Retrieves the names of all crawler resources in this AWS account, or the resources with the specified tag. This operation allows you to see which resources are available in your account, and their names.</p> <p>This operation takes the optional <code>Tags</code> field, which you can use as a filter on the response so that tagged resources can be retrieved as a group. If you choose to use tags filtering, only resources with the tag are retrieved.</p>
    fn list_crawlers(
        &self,
        input: ListCrawlersRequest,
    ) -> RusotoFuture<ListCrawlersResponse, ListCrawlersError>;

    /// <p>Retrieves the names of all <code>DevEndpoint</code> resources in this AWS account, or the resources with the specified tag. This operation allows you to see which resources are available in your account, and their names.</p> <p>This operation takes the optional <code>Tags</code> field, which you can use as a filter on the response so that tagged resources can be retrieved as a group. If you choose to use tags filtering, only resources with the tag are retrieved.</p>
    fn list_dev_endpoints(
        &self,
        input: ListDevEndpointsRequest,
    ) -> RusotoFuture<ListDevEndpointsResponse, ListDevEndpointsError>;

    /// <p>Retrieves the names of all job resources in this AWS account, or the resources with the specified tag. This operation allows you to see which resources are available in your account, and their names.</p> <p>This operation takes the optional <code>Tags</code> field, which you can use as a filter on the response so that tagged resources can be retrieved as a group. If you choose to use tags filtering, only resources with the tag are retrieved.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResponse, ListJobsError>;

    /// <p>Retrieves the names of all trigger resources in this AWS account, or the resources with the specified tag. This operation allows you to see which resources are available in your account, and their names.</p> <p>This operation takes the optional <code>Tags</code> field, which you can use as a filter on the response so that tagged resources can be retrieved as a group. If you choose to use tags filtering, only resources with the tag are retrieved.</p>
    fn list_triggers(
        &self,
        input: ListTriggersRequest,
    ) -> RusotoFuture<ListTriggersResponse, ListTriggersError>;

    /// <p>Lists names of workflows created in the account.</p>
    fn list_workflows(
        &self,
        input: ListWorkflowsRequest,
    ) -> RusotoFuture<ListWorkflowsResponse, ListWorkflowsError>;

    /// <p>Sets the security configuration for a specified catalog. After the configuration has been set, the specified encryption is applied to every catalog write thereafter.</p>
    fn put_data_catalog_encryption_settings(
        &self,
        input: PutDataCatalogEncryptionSettingsRequest,
    ) -> RusotoFuture<PutDataCatalogEncryptionSettingsResponse, PutDataCatalogEncryptionSettingsError>;

    /// <p>Sets the Data Catalog resource policy for access control.</p>
    fn put_resource_policy(
        &self,
        input: PutResourcePolicyRequest,
    ) -> RusotoFuture<PutResourcePolicyResponse, PutResourcePolicyError>;

    /// <p>Puts the specified workflow run properties for the given workflow run. If a property already exists for the specified run, then it overrides the value otherwise adds the property to existing properties.</p>
    fn put_workflow_run_properties(
        &self,
        input: PutWorkflowRunPropertiesRequest,
    ) -> RusotoFuture<PutWorkflowRunPropertiesResponse, PutWorkflowRunPropertiesError>;

    /// <p>Resets a bookmark entry.</p>
    fn reset_job_bookmark(
        &self,
        input: ResetJobBookmarkRequest,
    ) -> RusotoFuture<ResetJobBookmarkResponse, ResetJobBookmarkError>;

    /// <p>Searches a set of tables based on properties in the table metadata as well as on the parent database. You can search against text or filter conditions. </p> <p>You can only get tables that you have access to based on the security policies defined in Lake Formation. You need at least a read-only access to the table for it to be returned. If you do not have access to all the columns in the table, these columns will not be searched against when returning the list of tables back to you. If you have access to the columns but not the data in the columns, those columns and the associated metadata for those columns will be included in the search. </p>
    fn search_tables(
        &self,
        input: SearchTablesRequest,
    ) -> RusotoFuture<SearchTablesResponse, SearchTablesError>;

    /// <p>Starts a crawl using the specified crawler, regardless of what is scheduled. If the crawler is already running, returns a <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-exceptions.html#aws-glue-api-exceptions-CrawlerRunningException">CrawlerRunningException</a>.</p>
    fn start_crawler(
        &self,
        input: StartCrawlerRequest,
    ) -> RusotoFuture<StartCrawlerResponse, StartCrawlerError>;

    /// <p>Changes the schedule state of the specified crawler to <code>SCHEDULED</code>, unless the crawler is already running or the schedule state is already <code>SCHEDULED</code>.</p>
    fn start_crawler_schedule(
        &self,
        input: StartCrawlerScheduleRequest,
    ) -> RusotoFuture<StartCrawlerScheduleResponse, StartCrawlerScheduleError>;

    /// <p>Begins an asynchronous task to export all labeled data for a particular transform. This task is the only label-related API call that is not part of the typical active learning workflow. You typically use <code>StartExportLabelsTaskRun</code> when you want to work with all of your existing labels at the same time, such as when you want to remove or change labels that were previously submitted as truth. This API operation accepts the <code>TransformId</code> whose labels you want to export and an Amazon Simple Storage Service (Amazon S3) path to export the labels to. The operation returns a <code>TaskRunId</code>. You can check on the status of your task run by calling the <code>GetMLTaskRun</code> API.</p>
    fn start_export_labels_task_run(
        &self,
        input: StartExportLabelsTaskRunRequest,
    ) -> RusotoFuture<StartExportLabelsTaskRunResponse, StartExportLabelsTaskRunError>;

    /// <p>Enables you to provide additional labels (examples of truth) to be used to teach the machine learning transform and improve its quality. This API operation is generally used as part of the active learning workflow that starts with the <code>StartMLLabelingSetGenerationTaskRun</code> call and that ultimately results in improving the quality of your machine learning transform. </p> <p>After the <code>StartMLLabelingSetGenerationTaskRun</code> finishes, AWS Glue machine learning will have generated a series of questions for humans to answer. (Answering these questions is often called 'labeling' in the machine learning workflows). In the case of the <code>FindMatches</code> transform, these questions are of the form, “What is the correct way to group these rows together into groups composed entirely of matching records?” After the labeling process is finished, users upload their answers/labels with a call to <code>StartImportLabelsTaskRun</code>. After <code>StartImportLabelsTaskRun</code> finishes, all future runs of the machine learning transform use the new and improved labels and perform a higher-quality transformation.</p> <p>By default, <code>StartMLLabelingSetGenerationTaskRun</code> continually learns from and combines all labels that you upload unless you set <code>Replace</code> to true. If you set <code>Replace</code> to true, <code>StartImportLabelsTaskRun</code> deletes and forgets all previously uploaded labels and learns only from the exact set that you upload. Replacing labels can be helpful if you realize that you previously uploaded incorrect labels, and you believe that they are having a negative effect on your transform quality.</p> <p>You can check on the status of your task run by calling the <code>GetMLTaskRun</code> operation. </p>
    fn start_import_labels_task_run(
        &self,
        input: StartImportLabelsTaskRunRequest,
    ) -> RusotoFuture<StartImportLabelsTaskRunResponse, StartImportLabelsTaskRunError>;

    /// <p>Starts a job run using a job definition.</p>
    fn start_job_run(
        &self,
        input: StartJobRunRequest,
    ) -> RusotoFuture<StartJobRunResponse, StartJobRunError>;

    /// <p>Starts a task to estimate the quality of the transform. </p> <p>When you provide label sets as examples of truth, AWS Glue machine learning uses some of those examples to learn from them. The rest of the labels are used as a test to estimate quality.</p> <p>Returns a unique identifier for the run. You can call <code>GetMLTaskRun</code> to get more information about the stats of the <code>EvaluationTaskRun</code>.</p>
    fn start_ml_evaluation_task_run(
        &self,
        input: StartMLEvaluationTaskRunRequest,
    ) -> RusotoFuture<StartMLEvaluationTaskRunResponse, StartMLEvaluationTaskRunError>;

    /// <p>Starts the active learning workflow for your machine learning transform to improve the transform's quality by generating label sets and adding labels.</p> <p>When the <code>StartMLLabelingSetGenerationTaskRun</code> finishes, AWS Glue will have generated a "labeling set" or a set of questions for humans to answer.</p> <p>In the case of the <code>FindMatches</code> transform, these questions are of the form, “What is the correct way to group these rows together into groups composed entirely of matching records?” </p> <p>After the labeling process is finished, you can upload your labels with a call to <code>StartImportLabelsTaskRun</code>. After <code>StartImportLabelsTaskRun</code> finishes, all future runs of the machine learning transform will use the new and improved labels and perform a higher-quality transformation.</p>
    fn start_ml_labeling_set_generation_task_run(
        &self,
        input: StartMLLabelingSetGenerationTaskRunRequest,
    ) -> RusotoFuture<
        StartMLLabelingSetGenerationTaskRunResponse,
        StartMLLabelingSetGenerationTaskRunError,
    >;

    /// <p>Starts an existing trigger. See <a href="https://docs.aws.amazon.com/glue/latest/dg/trigger-job.html">Triggering Jobs</a> for information about how different types of trigger are started.</p>
    fn start_trigger(
        &self,
        input: StartTriggerRequest,
    ) -> RusotoFuture<StartTriggerResponse, StartTriggerError>;

    /// <p>Starts a new run of the specified workflow.</p>
    fn start_workflow_run(
        &self,
        input: StartWorkflowRunRequest,
    ) -> RusotoFuture<StartWorkflowRunResponse, StartWorkflowRunError>;

    /// <p>If the specified crawler is running, stops the crawl.</p>
    fn stop_crawler(
        &self,
        input: StopCrawlerRequest,
    ) -> RusotoFuture<StopCrawlerResponse, StopCrawlerError>;

    /// <p>Sets the schedule state of the specified crawler to <code>NOT_SCHEDULED</code>, but does not stop the crawler if it is already running.</p>
    fn stop_crawler_schedule(
        &self,
        input: StopCrawlerScheduleRequest,
    ) -> RusotoFuture<StopCrawlerScheduleResponse, StopCrawlerScheduleError>;

    /// <p>Stops a specified trigger.</p>
    fn stop_trigger(
        &self,
        input: StopTriggerRequest,
    ) -> RusotoFuture<StopTriggerResponse, StopTriggerError>;

    /// <p>Adds tags to a resource. A tag is a label you can assign to an AWS resource. In AWS Glue, you can tag only certain resources. For information about what resources you can tag, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">AWS Tags in AWS Glue</a>.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes tags from a resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Modifies an existing classifier (a <code>GrokClassifier</code>, an <code>XMLClassifier</code>, a <code>JsonClassifier</code>, or a <code>CsvClassifier</code>, depending on which field is present).</p>
    fn update_classifier(
        &self,
        input: UpdateClassifierRequest,
    ) -> RusotoFuture<UpdateClassifierResponse, UpdateClassifierError>;

    /// <p>Updates a connection definition in the Data Catalog.</p>
    fn update_connection(
        &self,
        input: UpdateConnectionRequest,
    ) -> RusotoFuture<UpdateConnectionResponse, UpdateConnectionError>;

    /// <p>Updates a crawler. If a crawler is running, you must stop it using <code>StopCrawler</code> before updating it.</p>
    fn update_crawler(
        &self,
        input: UpdateCrawlerRequest,
    ) -> RusotoFuture<UpdateCrawlerResponse, UpdateCrawlerError>;

    /// <p>Updates the schedule of a crawler using a <code>cron</code> expression. </p>
    fn update_crawler_schedule(
        &self,
        input: UpdateCrawlerScheduleRequest,
    ) -> RusotoFuture<UpdateCrawlerScheduleResponse, UpdateCrawlerScheduleError>;

    /// <p>Updates an existing database definition in a Data Catalog.</p>
    fn update_database(
        &self,
        input: UpdateDatabaseRequest,
    ) -> RusotoFuture<UpdateDatabaseResponse, UpdateDatabaseError>;

    /// <p>Updates a specified development endpoint.</p>
    fn update_dev_endpoint(
        &self,
        input: UpdateDevEndpointRequest,
    ) -> RusotoFuture<UpdateDevEndpointResponse, UpdateDevEndpointError>;

    /// <p>Updates an existing job definition.</p>
    fn update_job(
        &self,
        input: UpdateJobRequest,
    ) -> RusotoFuture<UpdateJobResponse, UpdateJobError>;

    /// <p>Updates an existing machine learning transform. Call this operation to tune the algorithm parameters to achieve better results.</p> <p>After calling this operation, you can call the <code>StartMLEvaluationTaskRun</code> operation to assess how well your new parameters achieved your goals (such as improving the quality of your machine learning transform, or making it more cost-effective).</p>
    fn update_ml_transform(
        &self,
        input: UpdateMLTransformRequest,
    ) -> RusotoFuture<UpdateMLTransformResponse, UpdateMLTransformError>;

    /// <p>Updates a partition.</p>
    fn update_partition(
        &self,
        input: UpdatePartitionRequest,
    ) -> RusotoFuture<UpdatePartitionResponse, UpdatePartitionError>;

    /// <p>Updates a metadata table in the Data Catalog.</p>
    fn update_table(
        &self,
        input: UpdateTableRequest,
    ) -> RusotoFuture<UpdateTableResponse, UpdateTableError>;

    /// <p>Updates a trigger definition.</p>
    fn update_trigger(
        &self,
        input: UpdateTriggerRequest,
    ) -> RusotoFuture<UpdateTriggerResponse, UpdateTriggerError>;

    /// <p>Updates an existing function definition in the Data Catalog.</p>
    fn update_user_defined_function(
        &self,
        input: UpdateUserDefinedFunctionRequest,
    ) -> RusotoFuture<UpdateUserDefinedFunctionResponse, UpdateUserDefinedFunctionError>;

    /// <p>Updates an existing workflow.</p>
    fn update_workflow(
        &self,
        input: UpdateWorkflowRequest,
    ) -> RusotoFuture<UpdateWorkflowResponse, UpdateWorkflowError>;
}
/// A client for the AWS Glue API.
#[derive(Clone)]
pub struct GlueClient {
    client: Client,
    region: region::Region,
}

impl GlueClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> GlueClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> GlueClient
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

    pub fn new_with_client(client: Client, region: region::Region) -> GlueClient {
        GlueClient { client, region }
    }
}

impl fmt::Debug for GlueClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GlueClient")
            .field("region", &self.region)
            .finish()
    }
}

impl Glue for GlueClient {
    /// <p>Creates one or more partitions in a batch operation.</p>
    fn batch_create_partition(
        &self,
        input: BatchCreatePartitionRequest,
    ) -> RusotoFuture<BatchCreatePartitionResponse, BatchCreatePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchCreatePartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchCreatePartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchCreatePartitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a list of connection definitions from the Data Catalog.</p>
    fn batch_delete_connection(
        &self,
        input: BatchDeleteConnectionRequest,
    ) -> RusotoFuture<BatchDeleteConnectionResponse, BatchDeleteConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchDeleteConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchDeleteConnectionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchDeleteConnectionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes one or more partitions in a batch operation.</p>
    fn batch_delete_partition(
        &self,
        input: BatchDeletePartitionRequest,
    ) -> RusotoFuture<BatchDeletePartitionResponse, BatchDeletePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchDeletePartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchDeletePartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchDeletePartitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Deletes multiple tables at once.</p> <note> <p>After completing this operation, you no longer have access to the table versions and partitions that belong to the deleted table. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure the immediate deletion of all related resources, before calling <code>BatchDeleteTable</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, and <code>DeletePartition</code> or <code>BatchDeletePartition</code>, to delete any resources that belong to the table.</p> </note></p>
    fn batch_delete_table(
        &self,
        input: BatchDeleteTableRequest,
    ) -> RusotoFuture<BatchDeleteTableResponse, BatchDeleteTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchDeleteTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchDeleteTableResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchDeleteTableError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified batch of versions of a table.</p>
    fn batch_delete_table_version(
        &self,
        input: BatchDeleteTableVersionRequest,
    ) -> RusotoFuture<BatchDeleteTableVersionResponse, BatchDeleteTableVersionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchDeleteTableVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchDeleteTableVersionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(BatchDeleteTableVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of resource metadata for a given list of crawler names. After calling the <code>ListCrawlers</code> operation, you can call this operation to access the data to which you have been granted permissions. This operation supports all IAM permissions, including permission conditions that uses tags.</p>
    fn batch_get_crawlers(
        &self,
        input: BatchGetCrawlersRequest,
    ) -> RusotoFuture<BatchGetCrawlersResponse, BatchGetCrawlersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchGetCrawlers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetCrawlersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchGetCrawlersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of resource metadata for a given list of development endpoint names. After calling the <code>ListDevEndpoints</code> operation, you can call this operation to access the data to which you have been granted permissions. This operation supports all IAM permissions, including permission conditions that uses tags.</p>
    fn batch_get_dev_endpoints(
        &self,
        input: BatchGetDevEndpointsRequest,
    ) -> RusotoFuture<BatchGetDevEndpointsResponse, BatchGetDevEndpointsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchGetDevEndpoints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetDevEndpointsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchGetDevEndpointsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a list of resource metadata for a given list of job names. After calling the <code>ListJobs</code> operation, you can call this operation to access the data to which you have been granted permissions. This operation supports all IAM permissions, including permission conditions that uses tags. </p>
    fn batch_get_jobs(
        &self,
        input: BatchGetJobsRequest,
    ) -> RusotoFuture<BatchGetJobsResponse, BatchGetJobsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchGetJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetJobsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchGetJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves partitions in a batch request.</p>
    fn batch_get_partition(
        &self,
        input: BatchGetPartitionRequest,
    ) -> RusotoFuture<BatchGetPartitionResponse, BatchGetPartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchGetPartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetPartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchGetPartitionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of resource metadata for a given list of trigger names. After calling the <code>ListTriggers</code> operation, you can call this operation to access the data to which you have been granted permissions. This operation supports all IAM permissions, including permission conditions that uses tags.</p>
    fn batch_get_triggers(
        &self,
        input: BatchGetTriggersRequest,
    ) -> RusotoFuture<BatchGetTriggersResponse, BatchGetTriggersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchGetTriggers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetTriggersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchGetTriggersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of resource metadata for a given list of workflow names. After calling the <code>ListWorkflows</code> operation, you can call this operation to access the data to which you have been granted permissions. This operation supports all IAM permissions, including permission conditions that uses tags.</p>
    fn batch_get_workflows(
        &self,
        input: BatchGetWorkflowsRequest,
    ) -> RusotoFuture<BatchGetWorkflowsResponse, BatchGetWorkflowsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchGetWorkflows");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetWorkflowsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchGetWorkflowsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Stops one or more job runs for a specified job definition.</p>
    fn batch_stop_job_run(
        &self,
        input: BatchStopJobRunRequest,
    ) -> RusotoFuture<BatchStopJobRunResponse, GlueBatchStopJobRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchStopJobRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchStopJobRunResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GlueBatchStopJobRunError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Cancels (stops) a task run. Machine learning task runs are asynchronous tasks that AWS Glue runs on your behalf as part of various machine learning workflows. You can cancel a machine learning task run at any time by calling <code>CancelMLTaskRun</code> with a task run's parent transform's <code>TransformID</code> and the task run's <code>TaskRunId</code>. </p>
    fn cancel_ml_task_run(
        &self,
        input: CancelMLTaskRunRequest,
    ) -> RusotoFuture<CancelMLTaskRunResponse, CancelMLTaskRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CancelMLTaskRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CancelMLTaskRunResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CancelMLTaskRunError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a classifier in the user's account. This can be a <code>GrokClassifier</code>, an <code>XMLClassifier</code>, a <code>JsonClassifier</code>, or a <code>CsvClassifier</code>, depending on which field of the request is present.</p>
    fn create_classifier(
        &self,
        input: CreateClassifierRequest,
    ) -> RusotoFuture<CreateClassifierResponse, CreateClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateClassifier");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateClassifierResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateClassifierError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a connection definition in the Data Catalog.</p>
    fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> RusotoFuture<CreateConnectionResponse, CreateConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateConnectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new crawler with specified targets, role, configuration, and optional schedule. At least one crawl target must be specified, in the <code>s3Targets</code> field, the <code>jdbcTargets</code> field, or the <code>DynamoDBTargets</code> field.</p>
    fn create_crawler(
        &self,
        input: CreateCrawlerRequest,
    ) -> RusotoFuture<CreateCrawlerResponse, CreateCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new database in a Data Catalog.</p>
    fn create_database(
        &self,
        input: CreateDatabaseRequest,
    ) -> RusotoFuture<CreateDatabaseResponse, CreateDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDatabaseResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDatabaseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new development endpoint.</p>
    fn create_dev_endpoint(
        &self,
        input: CreateDevEndpointRequest,
    ) -> RusotoFuture<CreateDevEndpointResponse, CreateDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateDevEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDevEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDevEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new job definition.</p>
    fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> RusotoFuture<CreateJobResponse, CreateJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateJobResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an AWS Glue machine learning transform. This operation creates the transform and all the necessary parameters to train it.</p> <p>Call this operation as the first step in the process of using a machine learning transform (such as the <code>FindMatches</code> transform) for deduplicating data. You can provide an optional <code>Description</code>, in addition to the parameters that you want to use for your algorithm.</p> <p>You must also specify certain parameters for the tasks that AWS Glue runs on your behalf as part of learning from your data and creating a high-quality machine learning transform. These parameters include <code>Role</code>, and optionally, <code>AllocatedCapacity</code>, <code>Timeout</code>, and <code>MaxRetries</code>. For more information, see <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-jobs-job.html">Jobs</a>.</p>
    fn create_ml_transform(
        &self,
        input: CreateMLTransformRequest,
    ) -> RusotoFuture<CreateMLTransformResponse, CreateMLTransformError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateMLTransform");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateMLTransformResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateMLTransformError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new partition.</p>
    fn create_partition(
        &self,
        input: CreatePartitionRequest,
    ) -> RusotoFuture<CreatePartitionResponse, CreatePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreatePartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreatePartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePartitionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Transforms a directed acyclic graph (DAG) into code.</p>
    fn create_script(
        &self,
        input: CreateScriptRequest,
    ) -> RusotoFuture<CreateScriptResponse, CreateScriptError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateScript");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateScriptResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateScriptError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new security configuration. A security configuration is a set of security properties that can be used by AWS Glue. You can use a security configuration to encrypt data at rest. For information about using security configurations in AWS Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/encryption-security-configuration.html">Encrypting Data Written by Crawlers, Jobs, and Development Endpoints</a>.</p>
    fn create_security_configuration(
        &self,
        input: CreateSecurityConfigurationRequest,
    ) -> RusotoFuture<CreateSecurityConfigurationResponse, CreateSecurityConfigurationError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateSecurityConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateSecurityConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateSecurityConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a new table definition in the Data Catalog.</p>
    fn create_table(
        &self,
        input: CreateTableRequest,
    ) -> RusotoFuture<CreateTableResponse, CreateTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateTableResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateTableError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new trigger.</p>
    fn create_trigger(
        &self,
        input: CreateTriggerRequest,
    ) -> RusotoFuture<CreateTriggerResponse, CreateTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new function definition in the Data Catalog.</p>
    fn create_user_defined_function(
        &self,
        input: CreateUserDefinedFunctionRequest,
    ) -> RusotoFuture<CreateUserDefinedFunctionResponse, CreateUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateUserDefinedFunction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateUserDefinedFunctionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateUserDefinedFunctionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a new workflow.</p>
    fn create_workflow(
        &self,
        input: CreateWorkflowRequest,
    ) -> RusotoFuture<CreateWorkflowResponse, CreateWorkflowError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateWorkflow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateWorkflowResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateWorkflowError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes a classifier from the Data Catalog.</p>
    fn delete_classifier(
        &self,
        input: DeleteClassifierRequest,
    ) -> RusotoFuture<DeleteClassifierResponse, DeleteClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteClassifier");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteClassifierResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteClassifierError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a connection from the Data Catalog.</p>
    fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> RusotoFuture<DeleteConnectionResponse, DeleteConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteConnectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes a specified crawler from the AWS Glue Data Catalog, unless the crawler state is <code>RUNNING</code>.</p>
    fn delete_crawler(
        &self,
        input: DeleteCrawlerRequest,
    ) -> RusotoFuture<DeleteCrawlerResponse, DeleteCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Removes a specified database from a Data Catalog.</p> <note> <p>After completing this operation, you no longer have access to the tables (and all table versions and partitions that might belong to the tables) and the user-defined functions in the deleted database. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure the immediate deletion of all related resources, before calling <code>DeleteDatabase</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, <code>DeletePartition</code> or <code>BatchDeletePartition</code>, <code>DeleteUserDefinedFunction</code>, and <code>DeleteTable</code> or <code>BatchDeleteTable</code>, to delete any resources that belong to the database.</p> </note></p>
    fn delete_database(
        &self,
        input: DeleteDatabaseRequest,
    ) -> RusotoFuture<DeleteDatabaseResponse, DeleteDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDatabaseResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDatabaseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified development endpoint.</p>
    fn delete_dev_endpoint(
        &self,
        input: DeleteDevEndpointRequest,
    ) -> RusotoFuture<DeleteDevEndpointResponse, DeleteDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteDevEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDevEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDevEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified job definition. If the job definition is not found, no exception is thrown.</p>
    fn delete_job(
        &self,
        input: DeleteJobRequest,
    ) -> RusotoFuture<DeleteJobResponse, DeleteJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteJobResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an AWS Glue machine learning transform. Machine learning transforms are a special type of transform that use machine learning to learn the details of the transformation to be performed by learning from examples provided by humans. These transformations are then saved by AWS Glue. If you no longer need a transform, you can delete it by calling <code>DeleteMLTransforms</code>. However, any AWS Glue jobs that still reference the deleted transform will no longer succeed.</p>
    fn delete_ml_transform(
        &self,
        input: DeleteMLTransformRequest,
    ) -> RusotoFuture<DeleteMLTransformResponse, DeleteMLTransformError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteMLTransform");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteMLTransformResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteMLTransformError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified partition.</p>
    fn delete_partition(
        &self,
        input: DeletePartitionRequest,
    ) -> RusotoFuture<DeletePartitionResponse, DeletePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeletePartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeletePartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePartitionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified policy.</p>
    fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyRequest,
    ) -> RusotoFuture<DeleteResourcePolicyResponse, DeleteResourcePolicyError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteResourcePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteResourcePolicyResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteResourcePolicyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a specified security configuration.</p>
    fn delete_security_configuration(
        &self,
        input: DeleteSecurityConfigurationRequest,
    ) -> RusotoFuture<DeleteSecurityConfigurationResponse, DeleteSecurityConfigurationError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteSecurityConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteSecurityConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSecurityConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Removes a table definition from the Data Catalog.</p> <note> <p>After completing this operation, you no longer have access to the table versions and partitions that belong to the deleted table. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure the immediate deletion of all related resources, before calling <code>DeleteTable</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, and <code>DeletePartition</code> or <code>BatchDeletePartition</code>, to delete any resources that belong to the table.</p> </note></p>
    fn delete_table(
        &self,
        input: DeleteTableRequest,
    ) -> RusotoFuture<DeleteTableResponse, DeleteTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteTableResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTableError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified version of a table.</p>
    fn delete_table_version(
        &self,
        input: DeleteTableVersionRequest,
    ) -> RusotoFuture<DeleteTableVersionResponse, DeleteTableVersionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteTableVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteTableVersionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTableVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified trigger. If the trigger is not found, no exception is thrown.</p>
    fn delete_trigger(
        &self,
        input: DeleteTriggerRequest,
    ) -> RusotoFuture<DeleteTriggerResponse, DeleteTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an existing function definition from the Data Catalog.</p>
    fn delete_user_defined_function(
        &self,
        input: DeleteUserDefinedFunctionRequest,
    ) -> RusotoFuture<DeleteUserDefinedFunctionResponse, DeleteUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteUserDefinedFunction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteUserDefinedFunctionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteUserDefinedFunctionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a workflow.</p>
    fn delete_workflow(
        &self,
        input: DeleteWorkflowRequest,
    ) -> RusotoFuture<DeleteWorkflowResponse, DeleteWorkflowError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteWorkflow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteWorkflowResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteWorkflowError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the status of a migration operation.</p>
    fn get_catalog_import_status(
        &self,
        input: GetCatalogImportStatusRequest,
    ) -> RusotoFuture<GetCatalogImportStatusResponse, GetCatalogImportStatusError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCatalogImportStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCatalogImportStatusResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetCatalogImportStatusError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieve a classifier by name.</p>
    fn get_classifier(
        &self,
        input: GetClassifierRequest,
    ) -> RusotoFuture<GetClassifierResponse, GetClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetClassifier");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetClassifierResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetClassifierError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all classifier objects in the Data Catalog.</p>
    fn get_classifiers(
        &self,
        input: GetClassifiersRequest,
    ) -> RusotoFuture<GetClassifiersResponse, GetClassifiersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetClassifiers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetClassifiersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetClassifiersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a connection definition from the Data Catalog.</p>
    fn get_connection(
        &self,
        input: GetConnectionRequest,
    ) -> RusotoFuture<GetConnectionResponse, GetConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConnectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of connection definitions from the Data Catalog.</p>
    fn get_connections(
        &self,
        input: GetConnectionsRequest,
    ) -> RusotoFuture<GetConnectionsResponse, GetConnectionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConnectionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetConnectionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves metadata for a specified crawler.</p>
    fn get_crawler(
        &self,
        input: GetCrawlerRequest,
    ) -> RusotoFuture<GetCrawlerResponse, GetCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves metrics about specified crawlers.</p>
    fn get_crawler_metrics(
        &self,
        input: GetCrawlerMetricsRequest,
    ) -> RusotoFuture<GetCrawlerMetricsResponse, GetCrawlerMetricsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCrawlerMetrics");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCrawlerMetricsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCrawlerMetricsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves metadata for all crawlers defined in the customer account.</p>
    fn get_crawlers(
        &self,
        input: GetCrawlersRequest,
    ) -> RusotoFuture<GetCrawlersResponse, GetCrawlersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCrawlers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCrawlersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCrawlersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the security configuration for a specified catalog.</p>
    fn get_data_catalog_encryption_settings(
        &self,
        input: GetDataCatalogEncryptionSettingsRequest,
    ) -> RusotoFuture<GetDataCatalogEncryptionSettingsResponse, GetDataCatalogEncryptionSettingsError>
    {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDataCatalogEncryptionSettings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDataCatalogEncryptionSettingsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDataCatalogEncryptionSettingsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the definition of a specified database.</p>
    fn get_database(
        &self,
        input: GetDatabaseRequest,
    ) -> RusotoFuture<GetDatabaseResponse, GetDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDatabaseResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDatabaseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves all databases defined in a given Data Catalog.</p>
    fn get_databases(
        &self,
        input: GetDatabasesRequest,
    ) -> RusotoFuture<GetDatabasesResponse, GetDatabasesError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDatabases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDatabasesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDatabasesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Transforms a Python script into a directed acyclic graph (DAG). </p>
    fn get_dataflow_graph(
        &self,
        input: GetDataflowGraphRequest,
    ) -> RusotoFuture<GetDataflowGraphResponse, GetDataflowGraphError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDataflowGraph");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDataflowGraphResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDataflowGraphError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Retrieves information about a specified development endpoint.</p> <note> <p>When you create a development endpoint in a virtual private cloud (VPC), AWS Glue returns only a private IP address, and the public IP address field is not populated. When you create a non-VPC development endpoint, AWS Glue returns only a public IP address.</p> </note></p>
    fn get_dev_endpoint(
        &self,
        input: GetDevEndpointRequest,
    ) -> RusotoFuture<GetDevEndpointResponse, GetDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDevEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDevEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDevEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Retrieves all the development endpoints in this AWS account.</p> <note> <p>When you create a development endpoint in a virtual private cloud (VPC), AWS Glue returns only a private IP address and the public IP address field is not populated. When you create a non-VPC development endpoint, AWS Glue returns only a public IP address.</p> </note></p>
    fn get_dev_endpoints(
        &self,
        input: GetDevEndpointsRequest,
    ) -> RusotoFuture<GetDevEndpointsResponse, GetDevEndpointsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDevEndpoints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDevEndpointsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDevEndpointsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves an existing job definition.</p>
    fn get_job(&self, input: GetJobRequest) -> RusotoFuture<GetJobResponse, GetJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetJobResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information on a job bookmark entry.</p>
    fn get_job_bookmark(
        &self,
        input: GetJobBookmarkRequest,
    ) -> RusotoFuture<GetJobBookmarkResponse, GetJobBookmarkError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJobBookmark");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetJobBookmarkResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobBookmarkError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the metadata for a given job run.</p>
    fn get_job_run(
        &self,
        input: GetJobRunRequest,
    ) -> RusotoFuture<GetJobRunResponse, GetJobRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJobRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetJobRunResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobRunError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves metadata for all runs of a given job definition.</p>
    fn get_job_runs(
        &self,
        input: GetJobRunsRequest,
    ) -> RusotoFuture<GetJobRunsResponse, GetJobRunsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJobRuns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetJobRunsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobRunsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves all current job definitions.</p>
    fn get_jobs(&self, input: GetJobsRequest) -> RusotoFuture<GetJobsResponse, GetJobsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetJobsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets details for a specific task run on a machine learning transform. Machine learning task runs are asynchronous tasks that AWS Glue runs on your behalf as part of various machine learning workflows. You can check the stats of any task run by calling <code>GetMLTaskRun</code> with the <code>TaskRunID</code> and its parent transform's <code>TransformID</code>.</p>
    fn get_ml_task_run(
        &self,
        input: GetMLTaskRunRequest,
    ) -> RusotoFuture<GetMLTaskRunResponse, GetMLTaskRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetMLTaskRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMLTaskRunResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMLTaskRunError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of runs for a machine learning transform. Machine learning task runs are asynchronous tasks that AWS Glue runs on your behalf as part of various machine learning workflows. You can get a sortable, filterable list of machine learning task runs by calling <code>GetMLTaskRuns</code> with their parent transform's <code>TransformID</code> and other optional parameters as documented in this section.</p> <p>This operation returns a list of historic runs and must be paginated.</p>
    fn get_ml_task_runs(
        &self,
        input: GetMLTaskRunsRequest,
    ) -> RusotoFuture<GetMLTaskRunsResponse, GetMLTaskRunsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetMLTaskRuns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMLTaskRunsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMLTaskRunsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets an AWS Glue machine learning transform artifact and all its corresponding metadata. Machine learning transforms are a special type of transform that use machine learning to learn the details of the transformation to be performed by learning from examples provided by humans. These transformations are then saved by AWS Glue. You can retrieve their metadata by calling <code>GetMLTransform</code>.</p>
    fn get_ml_transform(
        &self,
        input: GetMLTransformRequest,
    ) -> RusotoFuture<GetMLTransformResponse, GetMLTransformError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetMLTransform");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMLTransformResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMLTransformError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a sortable, filterable list of existing AWS Glue machine learning transforms. Machine learning transforms are a special type of transform that use machine learning to learn the details of the transformation to be performed by learning from examples provided by humans. These transformations are then saved by AWS Glue, and you can retrieve their metadata by calling <code>GetMLTransforms</code>.</p>
    fn get_ml_transforms(
        &self,
        input: GetMLTransformsRequest,
    ) -> RusotoFuture<GetMLTransformsResponse, GetMLTransformsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetMLTransforms");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMLTransformsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMLTransformsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates mappings.</p>
    fn get_mapping(
        &self,
        input: GetMappingRequest,
    ) -> RusotoFuture<GetMappingResponse, GetMappingError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetMapping");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMappingResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMappingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves information about a specified partition.</p>
    fn get_partition(
        &self,
        input: GetPartitionRequest,
    ) -> RusotoFuture<GetPartitionResponse, GetPartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetPartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPartitionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves information about the partitions in a table.</p>
    fn get_partitions(
        &self,
        input: GetPartitionsRequest,
    ) -> RusotoFuture<GetPartitionsResponse, GetPartitionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetPartitions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPartitionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPartitionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets code to perform a specified mapping.</p>
    fn get_plan(&self, input: GetPlanRequest) -> RusotoFuture<GetPlanResponse, GetPlanError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetPlan");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetPlanResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPlanError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a specified resource policy.</p>
    fn get_resource_policy(
        &self,
    ) -> RusotoFuture<GetResourcePolicyResponse, GetResourcePolicyError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetResourcePolicy");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetResourcePolicyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetResourcePolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a specified security configuration.</p>
    fn get_security_configuration(
        &self,
        input: GetSecurityConfigurationRequest,
    ) -> RusotoFuture<GetSecurityConfigurationResponse, GetSecurityConfigurationError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetSecurityConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSecurityConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSecurityConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list of all security configurations.</p>
    fn get_security_configurations(
        &self,
        input: GetSecurityConfigurationsRequest,
    ) -> RusotoFuture<GetSecurityConfigurationsResponse, GetSecurityConfigurationsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetSecurityConfigurations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSecurityConfigurationsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSecurityConfigurationsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the <code>Table</code> definition in a Data Catalog for a specified table.</p>
    fn get_table(&self, input: GetTableRequest) -> RusotoFuture<GetTableResponse, GetTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTableResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTableError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a specified version of a table.</p>
    fn get_table_version(
        &self,
        input: GetTableVersionRequest,
    ) -> RusotoFuture<GetTableVersionResponse, GetTableVersionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTableVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTableVersionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTableVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of strings that identify available versions of a specified table.</p>
    fn get_table_versions(
        &self,
        input: GetTableVersionsRequest,
    ) -> RusotoFuture<GetTableVersionsResponse, GetTableVersionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTableVersions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTableVersionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTableVersionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the definitions of some or all of the tables in a given <code>Database</code>.</p>
    fn get_tables(
        &self,
        input: GetTablesRequest,
    ) -> RusotoFuture<GetTablesResponse, GetTablesError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTables");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTablesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTablesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of tags associated with a resource.</p>
    fn get_tags(&self, input: GetTagsRequest) -> RusotoFuture<GetTagsResponse, GetTagsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetTagsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the definition of a trigger.</p>
    fn get_trigger(
        &self,
        input: GetTriggerRequest,
    ) -> RusotoFuture<GetTriggerResponse, GetTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets all the triggers associated with a job.</p>
    fn get_triggers(
        &self,
        input: GetTriggersRequest,
    ) -> RusotoFuture<GetTriggersResponse, GetTriggersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTriggers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTriggersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTriggersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a specified function definition from the Data Catalog.</p>
    fn get_user_defined_function(
        &self,
        input: GetUserDefinedFunctionRequest,
    ) -> RusotoFuture<GetUserDefinedFunctionResponse, GetUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetUserDefinedFunction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetUserDefinedFunctionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetUserDefinedFunctionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves multiple function definitions from the Data Catalog.</p>
    fn get_user_defined_functions(
        &self,
        input: GetUserDefinedFunctionsRequest,
    ) -> RusotoFuture<GetUserDefinedFunctionsResponse, GetUserDefinedFunctionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetUserDefinedFunctions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetUserDefinedFunctionsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetUserDefinedFunctionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves resource metadata for a workflow.</p>
    fn get_workflow(
        &self,
        input: GetWorkflowRequest,
    ) -> RusotoFuture<GetWorkflowResponse, GetWorkflowError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetWorkflow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetWorkflowResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetWorkflowError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the metadata for a given workflow run. </p>
    fn get_workflow_run(
        &self,
        input: GetWorkflowRunRequest,
    ) -> RusotoFuture<GetWorkflowRunResponse, GetWorkflowRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetWorkflowRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetWorkflowRunResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetWorkflowRunError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the workflow run properties which were set during the run.</p>
    fn get_workflow_run_properties(
        &self,
        input: GetWorkflowRunPropertiesRequest,
    ) -> RusotoFuture<GetWorkflowRunPropertiesResponse, GetWorkflowRunPropertiesError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetWorkflowRunProperties");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetWorkflowRunPropertiesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetWorkflowRunPropertiesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves metadata for all runs of a given workflow.</p>
    fn get_workflow_runs(
        &self,
        input: GetWorkflowRunsRequest,
    ) -> RusotoFuture<GetWorkflowRunsResponse, GetWorkflowRunsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetWorkflowRuns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetWorkflowRunsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetWorkflowRunsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Imports an existing Amazon Athena Data Catalog to AWS Glue</p>
    fn import_catalog_to_glue(
        &self,
        input: ImportCatalogToGlueRequest,
    ) -> RusotoFuture<ImportCatalogToGlueResponse, ImportCatalogToGlueError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.ImportCatalogToGlue");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ImportCatalogToGlueResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ImportCatalogToGlueError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves the names of all crawler resources in this AWS account, or the resources with the specified tag. This operation allows you to see which resources are available in your account, and their names.</p> <p>This operation takes the optional <code>Tags</code> field, which you can use as a filter on the response so that tagged resources can be retrieved as a group. If you choose to use tags filtering, only resources with the tag are retrieved.</p>
    fn list_crawlers(
        &self,
        input: ListCrawlersRequest,
    ) -> RusotoFuture<ListCrawlersResponse, ListCrawlersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.ListCrawlers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListCrawlersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListCrawlersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the names of all <code>DevEndpoint</code> resources in this AWS account, or the resources with the specified tag. This operation allows you to see which resources are available in your account, and their names.</p> <p>This operation takes the optional <code>Tags</code> field, which you can use as a filter on the response so that tagged resources can be retrieved as a group. If you choose to use tags filtering, only resources with the tag are retrieved.</p>
    fn list_dev_endpoints(
        &self,
        input: ListDevEndpointsRequest,
    ) -> RusotoFuture<ListDevEndpointsResponse, ListDevEndpointsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.ListDevEndpoints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDevEndpointsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDevEndpointsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the names of all job resources in this AWS account, or the resources with the specified tag. This operation allows you to see which resources are available in your account, and their names.</p> <p>This operation takes the optional <code>Tags</code> field, which you can use as a filter on the response so that tagged resources can be retrieved as a group. If you choose to use tags filtering, only resources with the tag are retrieved.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResponse, ListJobsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.ListJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListJobsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the names of all trigger resources in this AWS account, or the resources with the specified tag. This operation allows you to see which resources are available in your account, and their names.</p> <p>This operation takes the optional <code>Tags</code> field, which you can use as a filter on the response so that tagged resources can be retrieved as a group. If you choose to use tags filtering, only resources with the tag are retrieved.</p>
    fn list_triggers(
        &self,
        input: ListTriggersRequest,
    ) -> RusotoFuture<ListTriggersResponse, ListTriggersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.ListTriggers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTriggersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTriggersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists names of workflows created in the account.</p>
    fn list_workflows(
        &self,
        input: ListWorkflowsRequest,
    ) -> RusotoFuture<ListWorkflowsResponse, ListWorkflowsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.ListWorkflows");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListWorkflowsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListWorkflowsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets the security configuration for a specified catalog. After the configuration has been set, the specified encryption is applied to every catalog write thereafter.</p>
    fn put_data_catalog_encryption_settings(
        &self,
        input: PutDataCatalogEncryptionSettingsRequest,
    ) -> RusotoFuture<PutDataCatalogEncryptionSettingsResponse, PutDataCatalogEncryptionSettingsError>
    {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.PutDataCatalogEncryptionSettings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutDataCatalogEncryptionSettingsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutDataCatalogEncryptionSettingsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Sets the Data Catalog resource policy for access control.</p>
    fn put_resource_policy(
        &self,
        input: PutResourcePolicyRequest,
    ) -> RusotoFuture<PutResourcePolicyResponse, PutResourcePolicyError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.PutResourcePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutResourcePolicyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutResourcePolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Puts the specified workflow run properties for the given workflow run. If a property already exists for the specified run, then it overrides the value otherwise adds the property to existing properties.</p>
    fn put_workflow_run_properties(
        &self,
        input: PutWorkflowRunPropertiesRequest,
    ) -> RusotoFuture<PutWorkflowRunPropertiesResponse, PutWorkflowRunPropertiesError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.PutWorkflowRunProperties");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutWorkflowRunPropertiesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutWorkflowRunPropertiesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Resets a bookmark entry.</p>
    fn reset_job_bookmark(
        &self,
        input: ResetJobBookmarkRequest,
    ) -> RusotoFuture<ResetJobBookmarkResponse, ResetJobBookmarkError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.ResetJobBookmark");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ResetJobBookmarkResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ResetJobBookmarkError::from_response(response))),
                )
            }
        })
    }

    /// <p>Searches a set of tables based on properties in the table metadata as well as on the parent database. You can search against text or filter conditions. </p> <p>You can only get tables that you have access to based on the security policies defined in Lake Formation. You need at least a read-only access to the table for it to be returned. If you do not have access to all the columns in the table, these columns will not be searched against when returning the list of tables back to you. If you have access to the columns but not the data in the columns, those columns and the associated metadata for those columns will be included in the search. </p>
    fn search_tables(
        &self,
        input: SearchTablesRequest,
    ) -> RusotoFuture<SearchTablesResponse, SearchTablesError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.SearchTables");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchTablesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchTablesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts a crawl using the specified crawler, regardless of what is scheduled. If the crawler is already running, returns a <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-exceptions.html#aws-glue-api-exceptions-CrawlerRunningException">CrawlerRunningException</a>.</p>
    fn start_crawler(
        &self,
        input: StartCrawlerRequest,
    ) -> RusotoFuture<StartCrawlerResponse, StartCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Changes the schedule state of the specified crawler to <code>SCHEDULED</code>, unless the crawler is already running or the schedule state is already <code>SCHEDULED</code>.</p>
    fn start_crawler_schedule(
        &self,
        input: StartCrawlerScheduleRequest,
    ) -> RusotoFuture<StartCrawlerScheduleResponse, StartCrawlerScheduleError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartCrawlerSchedule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartCrawlerScheduleResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartCrawlerScheduleError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Begins an asynchronous task to export all labeled data for a particular transform. This task is the only label-related API call that is not part of the typical active learning workflow. You typically use <code>StartExportLabelsTaskRun</code> when you want to work with all of your existing labels at the same time, such as when you want to remove or change labels that were previously submitted as truth. This API operation accepts the <code>TransformId</code> whose labels you want to export and an Amazon Simple Storage Service (Amazon S3) path to export the labels to. The operation returns a <code>TaskRunId</code>. You can check on the status of your task run by calling the <code>GetMLTaskRun</code> API.</p>
    fn start_export_labels_task_run(
        &self,
        input: StartExportLabelsTaskRunRequest,
    ) -> RusotoFuture<StartExportLabelsTaskRunResponse, StartExportLabelsTaskRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartExportLabelsTaskRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartExportLabelsTaskRunResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartExportLabelsTaskRunError::from_response(response))
                }))
            }
        })
    }

    /// <p>Enables you to provide additional labels (examples of truth) to be used to teach the machine learning transform and improve its quality. This API operation is generally used as part of the active learning workflow that starts with the <code>StartMLLabelingSetGenerationTaskRun</code> call and that ultimately results in improving the quality of your machine learning transform. </p> <p>After the <code>StartMLLabelingSetGenerationTaskRun</code> finishes, AWS Glue machine learning will have generated a series of questions for humans to answer. (Answering these questions is often called 'labeling' in the machine learning workflows). In the case of the <code>FindMatches</code> transform, these questions are of the form, “What is the correct way to group these rows together into groups composed entirely of matching records?” After the labeling process is finished, users upload their answers/labels with a call to <code>StartImportLabelsTaskRun</code>. After <code>StartImportLabelsTaskRun</code> finishes, all future runs of the machine learning transform use the new and improved labels and perform a higher-quality transformation.</p> <p>By default, <code>StartMLLabelingSetGenerationTaskRun</code> continually learns from and combines all labels that you upload unless you set <code>Replace</code> to true. If you set <code>Replace</code> to true, <code>StartImportLabelsTaskRun</code> deletes and forgets all previously uploaded labels and learns only from the exact set that you upload. Replacing labels can be helpful if you realize that you previously uploaded incorrect labels, and you believe that they are having a negative effect on your transform quality.</p> <p>You can check on the status of your task run by calling the <code>GetMLTaskRun</code> operation. </p>
    fn start_import_labels_task_run(
        &self,
        input: StartImportLabelsTaskRunRequest,
    ) -> RusotoFuture<StartImportLabelsTaskRunResponse, StartImportLabelsTaskRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartImportLabelsTaskRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartImportLabelsTaskRunResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartImportLabelsTaskRunError::from_response(response))
                }))
            }
        })
    }

    /// <p>Starts a job run using a job definition.</p>
    fn start_job_run(
        &self,
        input: StartJobRunRequest,
    ) -> RusotoFuture<StartJobRunResponse, StartJobRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartJobRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartJobRunResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartJobRunError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts a task to estimate the quality of the transform. </p> <p>When you provide label sets as examples of truth, AWS Glue machine learning uses some of those examples to learn from them. The rest of the labels are used as a test to estimate quality.</p> <p>Returns a unique identifier for the run. You can call <code>GetMLTaskRun</code> to get more information about the stats of the <code>EvaluationTaskRun</code>.</p>
    fn start_ml_evaluation_task_run(
        &self,
        input: StartMLEvaluationTaskRunRequest,
    ) -> RusotoFuture<StartMLEvaluationTaskRunResponse, StartMLEvaluationTaskRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartMLEvaluationTaskRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartMLEvaluationTaskRunResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartMLEvaluationTaskRunError::from_response(response))
                }))
            }
        })
    }

    /// <p>Starts the active learning workflow for your machine learning transform to improve the transform's quality by generating label sets and adding labels.</p> <p>When the <code>StartMLLabelingSetGenerationTaskRun</code> finishes, AWS Glue will have generated a "labeling set" or a set of questions for humans to answer.</p> <p>In the case of the <code>FindMatches</code> transform, these questions are of the form, “What is the correct way to group these rows together into groups composed entirely of matching records?” </p> <p>After the labeling process is finished, you can upload your labels with a call to <code>StartImportLabelsTaskRun</code>. After <code>StartImportLabelsTaskRun</code> finishes, all future runs of the machine learning transform will use the new and improved labels and perform a higher-quality transformation.</p>
    fn start_ml_labeling_set_generation_task_run(
        &self,
        input: StartMLLabelingSetGenerationTaskRunRequest,
    ) -> RusotoFuture<
        StartMLLabelingSetGenerationTaskRunResponse,
        StartMLLabelingSetGenerationTaskRunError,
    > {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSGlue.StartMLLabelingSetGenerationTaskRun",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartMLLabelingSetGenerationTaskRunResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartMLLabelingSetGenerationTaskRunError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Starts an existing trigger. See <a href="https://docs.aws.amazon.com/glue/latest/dg/trigger-job.html">Triggering Jobs</a> for information about how different types of trigger are started.</p>
    fn start_trigger(
        &self,
        input: StartTriggerRequest,
    ) -> RusotoFuture<StartTriggerResponse, StartTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts a new run of the specified workflow.</p>
    fn start_workflow_run(
        &self,
        input: StartWorkflowRunRequest,
    ) -> RusotoFuture<StartWorkflowRunResponse, StartWorkflowRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartWorkflowRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartWorkflowRunResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartWorkflowRunError::from_response(response))),
                )
            }
        })
    }

    /// <p>If the specified crawler is running, stops the crawl.</p>
    fn stop_crawler(
        &self,
        input: StopCrawlerRequest,
    ) -> RusotoFuture<StopCrawlerResponse, StopCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StopCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets the schedule state of the specified crawler to <code>NOT_SCHEDULED</code>, but does not stop the crawler if it is already running.</p>
    fn stop_crawler_schedule(
        &self,
        input: StopCrawlerScheduleRequest,
    ) -> RusotoFuture<StopCrawlerScheduleResponse, StopCrawlerScheduleError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StopCrawlerSchedule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopCrawlerScheduleResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StopCrawlerScheduleError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Stops a specified trigger.</p>
    fn stop_trigger(
        &self,
        input: StopTriggerRequest,
    ) -> RusotoFuture<StopTriggerResponse, StopTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StopTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds tags to a resource. A tag is a label you can assign to an AWS resource. In AWS Glue, you can tag only certain resources. For information about what resources you can tag, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">AWS Tags in AWS Glue</a>.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceResponse, _>()
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

    /// <p>Removes tags from a resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceResponse, _>()
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

    /// <p>Modifies an existing classifier (a <code>GrokClassifier</code>, an <code>XMLClassifier</code>, a <code>JsonClassifier</code>, or a <code>CsvClassifier</code>, depending on which field is present).</p>
    fn update_classifier(
        &self,
        input: UpdateClassifierRequest,
    ) -> RusotoFuture<UpdateClassifierResponse, UpdateClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateClassifier");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateClassifierResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateClassifierError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a connection definition in the Data Catalog.</p>
    fn update_connection(
        &self,
        input: UpdateConnectionRequest,
    ) -> RusotoFuture<UpdateConnectionResponse, UpdateConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateConnectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a crawler. If a crawler is running, you must stop it using <code>StopCrawler</code> before updating it.</p>
    fn update_crawler(
        &self,
        input: UpdateCrawlerRequest,
    ) -> RusotoFuture<UpdateCrawlerResponse, UpdateCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the schedule of a crawler using a <code>cron</code> expression. </p>
    fn update_crawler_schedule(
        &self,
        input: UpdateCrawlerScheduleRequest,
    ) -> RusotoFuture<UpdateCrawlerScheduleResponse, UpdateCrawlerScheduleError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateCrawlerSchedule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateCrawlerScheduleResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateCrawlerScheduleError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates an existing database definition in a Data Catalog.</p>
    fn update_database(
        &self,
        input: UpdateDatabaseRequest,
    ) -> RusotoFuture<UpdateDatabaseResponse, UpdateDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDatabaseResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDatabaseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a specified development endpoint.</p>
    fn update_dev_endpoint(
        &self,
        input: UpdateDevEndpointRequest,
    ) -> RusotoFuture<UpdateDevEndpointResponse, UpdateDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateDevEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDevEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDevEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an existing job definition.</p>
    fn update_job(
        &self,
        input: UpdateJobRequest,
    ) -> RusotoFuture<UpdateJobResponse, UpdateJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateJobResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an existing machine learning transform. Call this operation to tune the algorithm parameters to achieve better results.</p> <p>After calling this operation, you can call the <code>StartMLEvaluationTaskRun</code> operation to assess how well your new parameters achieved your goals (such as improving the quality of your machine learning transform, or making it more cost-effective).</p>
    fn update_ml_transform(
        &self,
        input: UpdateMLTransformRequest,
    ) -> RusotoFuture<UpdateMLTransformResponse, UpdateMLTransformError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateMLTransform");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateMLTransformResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateMLTransformError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a partition.</p>
    fn update_partition(
        &self,
        input: UpdatePartitionRequest,
    ) -> RusotoFuture<UpdatePartitionResponse, UpdatePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdatePartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdatePartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdatePartitionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a metadata table in the Data Catalog.</p>
    fn update_table(
        &self,
        input: UpdateTableRequest,
    ) -> RusotoFuture<UpdateTableResponse, UpdateTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateTableResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateTableError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a trigger definition.</p>
    fn update_trigger(
        &self,
        input: UpdateTriggerRequest,
    ) -> RusotoFuture<UpdateTriggerResponse, UpdateTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an existing function definition in the Data Catalog.</p>
    fn update_user_defined_function(
        &self,
        input: UpdateUserDefinedFunctionRequest,
    ) -> RusotoFuture<UpdateUserDefinedFunctionResponse, UpdateUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateUserDefinedFunction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateUserDefinedFunctionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateUserDefinedFunctionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates an existing workflow.</p>
    fn update_workflow(
        &self,
        input: UpdateWorkflowRequest,
    ) -> RusotoFuture<UpdateWorkflowResponse, UpdateWorkflowError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateWorkflow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateWorkflowResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateWorkflowError::from_response(response))),
                )
            }
        })
    }
}
