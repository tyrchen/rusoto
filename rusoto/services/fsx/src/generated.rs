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
/// <p>The Microsoft AD attributes of the Amazon FSx for Windows File Server file system.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActiveDirectoryBackupAttributes {
    /// <p>The ID of the AWS Managed Microsoft Active Directory instance to which the file system is joined.</p>
    #[serde(rename = "ActiveDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    /// <p>The fully qualified domain name of the self-managed AD directory.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

/// <p>A backup of an Amazon FSx for Windows File Server file system. You can create a new file system from a backup to protect against data loss.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Backup {
    /// <p>The ID of the backup.</p>
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    /// <p>The time when a particular backup was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The configuration of the self-managed Microsoft Active Directory (AD) to which the Windows File Server instance is joined.</p>
    #[serde(rename = "DirectoryInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_information: Option<ActiveDirectoryBackupAttributes>,
    /// <p>Details explaining any failures that occur when creating a backup.</p>
    #[serde(rename = "FailureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<BackupFailureDetails>,
    /// <p>Metadata of the file system associated with the backup. This metadata is persisted even if the file system is deleted.</p>
    #[serde(rename = "FileSystem")]
    pub file_system: FileSystem,
    /// <p>The ID of the AWS Key Management Service (AWS KMS) key used to encrypt this backup's data.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The lifecycle status of the backup.</p>
    #[serde(rename = "Lifecycle")]
    pub lifecycle: String,
    #[serde(rename = "ProgressPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) for the backup resource.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>Tags associated with a particular file system.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of the backup.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>If backup creation fails, this structure contains the details of that failure.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BackupFailureDetails {
    /// <p>A message describing the backup creation failure.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The request object for the <code>CreateBackup</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBackupRequest {
    /// <p>(Optional) A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent creation. This string is automatically filled on your behalf when you use the AWS Command Line Interface (AWS CLI) or an AWS SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The ID of the file system to back up.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>The tags to apply to the backup at backup creation. The key value of the <code>Name</code> tag appears in the console as the backup name.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>The response object for the <code>CreateBackup</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBackupResponse {
    /// <p>A description of the backup.</p>
    #[serde(rename = "Backup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

/// <p>The request object for the <code>CreateFileSystemFromBackup</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFileSystemFromBackupRequest {
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    /// <p>(Optional) A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent creation. This string is automatically filled on your behalf when you use the AWS Command Line Interface (AWS CLI) or an AWS SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups apply to all network interfaces. This value isn't returned in later describe requests.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of IDs for the subnets that the file system will be accessible from. Currently, you can specify only one subnet. The file server is also launched in that subnet's Availability Zone.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The tags to be applied to the file system at file system creation. The key value of the <code>Name</code> tag appears in the console as the file system name.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The configuration for this Microsoft Windows file system.</p>
    #[serde(rename = "WindowsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<CreateFileSystemWindowsConfiguration>,
}

/// <p>The response object for the <code>CreateFileSystemFromBackup</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFileSystemFromBackupResponse {
    /// <p>A description of the file system.</p>
    #[serde(rename = "FileSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
}

/// <p>The Lustre configuration for the file system being created. This value is required if <code>FileSystemType</code> is set to <code>LUSTRE</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFileSystemLustreConfiguration {
    /// <p>(Optional) The path in Amazon S3 where the root of your Amazon FSx file system is exported. The path must use the same Amazon S3 bucket as specified in ImportPath. You can provide an optional prefix to which new and changed data is to be exported from your Amazon FSx for Lustre file system. If an <code>ExportPath</code> value is not provided, Amazon FSx sets a default export path, <code>s3://import-bucket/FSxLustre[creation-timestamp]</code>. The timestamp is in UTC format, for example <code>s3://import-bucket/FSxLustre20181105T222312Z</code>.</p> <p>The Amazon S3 export bucket must be the same as the import bucket specified by <code>ImportPath</code>. If you only specify a bucket name, such as <code>s3://import-bucket</code>, you get a 1:1 mapping of file system objects to S3 bucket objects. This mapping means that the input data in S3 is overwritten on export. If you provide a custom prefix in the export path, such as <code>s3://import-bucket/[custom-optional-prefix]</code>, Amazon FSx exports the contents of your file system to that export prefix in the Amazon S3 bucket.</p>
    #[serde(rename = "ExportPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_path: Option<String>,
    /// <p>(Optional) The path to the Amazon S3 bucket (including the optional prefix) that you're using as the data repository for your Amazon FSx for Lustre file system. The root of your FSx for Lustre file system will be mapped to the root of the Amazon S3 bucket you select. An example is <code>s3://import-bucket/optional-prefix</code>. If you specify a prefix after the Amazon S3 bucket name, only object keys with that prefix are loaded into the file system.</p>
    #[serde(rename = "ImportPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_path: Option<String>,
    /// <p>(Optional) For files imported from a data repository, this value determines the stripe count and maximum amount of data per file (in MiB) stored on a single physical disk. The maximum number of disks that a single file can be striped across is limited by the total number of disks that make up the file system.</p> <p>The chunk size default is 1,024 MiB (1 GiB) and can go as high as 512,000 MiB (500 GiB). Amazon S3 objects have a maximum size of 5 TB.</p>
    #[serde(rename = "ImportedFileChunkSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_file_chunk_size: Option<i64>,
    /// <p>The preferred time to perform weekly maintenance, in the UTC time zone.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// <p>The request object used to create a new Amazon FSx file system.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFileSystemRequest {
    /// <p>(Optional) A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent creation. This string is automatically filled on your behalf when you use the AWS Command Line Interface (AWS CLI) or an AWS SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The type of Amazon FSx file system to create.</p>
    #[serde(rename = "FileSystemType")]
    pub file_system_type: String,
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<CreateFileSystemLustreConfiguration>,
    /// <p>A list of IDs specifying the security groups to apply to all network interfaces created for file system access. This list isn't returned in later requests to describe the file system.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The storage capacity of the file system being created.</p> <p>For Windows file systems, the storage capacity has a minimum of 300 GiB, and a maximum of 65,536 GiB.</p> <p>For Lustre file systems, the storage capacity has a minimum of 3,600 GiB. Storage capacity is provisioned in increments of 3,600 GiB.</p>
    #[serde(rename = "StorageCapacity")]
    pub storage_capacity: i64,
    /// <p>The IDs of the subnets that the file system will be accessible from. File systems support only one subnet. The file server is also launched in that subnet's Availability Zone.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The tags to apply to the file system being created. The key value of the <code>Name</code> tag appears in the console as the file system name.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The Microsoft Windows configuration for the file system being created. This value is required if <code>FileSystemType</code> is set to <code>WINDOWS</code>.</p>
    #[serde(rename = "WindowsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<CreateFileSystemWindowsConfiguration>,
}

/// <p>The response object returned after the file system is created.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFileSystemResponse {
    /// <p>The configuration of the file system that was created.</p>
    #[serde(rename = "FileSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
}

/// <p>The configuration object for the Microsoft Windows file system used in <code>CreateFileSystem</code> and <code>CreateFileSystemFromBackup</code> operations.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFileSystemWindowsConfiguration {
    /// <p>The ID for an existing AWS Managed Microsoft Active Directory (AD) instance that the file system should join when it's created.</p>
    #[serde(rename = "ActiveDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    /// <p>The number of days to retain automatic backups. The default is to retain backups for 7 days. Setting this value to 0 disables the creation of automatic backups. The maximum retention period for backups is 35 days.</p>
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i64>,
    /// <p>A boolean flag indicating whether tags for the file system should be copied to backups. This value defaults to false. If it's set to true, all tags for the file system are copied to all automatic and user-initiated backups where the user doesn't specify tags. If this value is true, and you specify one or more tags, only the specified tags are copied to backups.</p>
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    /// <p>The preferred time to take daily automatic backups, formatted HH:MM in the UTC time zone.</p>
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration:
        Option<SelfManagedActiveDirectoryConfiguration>,
    /// <p>The throughput of an Amazon FSx file system, measured in megabytes per second, in 2 to the <i>n</i>th increments, between 2^3 (8) and 2^11 (2048).</p>
    #[serde(rename = "ThroughputCapacity")]
    pub throughput_capacity: i64,
    /// <p>The preferred start time to perform weekly maintenance, formatted d:HH:MM in the UTC time zone.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// <p>The data repository configuration object for Lustre file systems returned in the response of the <code>CreateFileSystem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataRepositoryConfiguration {
    /// <p>The export path to the Amazon S3 bucket (and prefix) that you are using to store new and changed Lustre file system files in S3.</p>
    #[serde(rename = "ExportPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_path: Option<String>,
    /// <p>The import path to the Amazon S3 bucket (and optional prefix) that you're using as the data repository for your FSx for Lustre file system, for example <code>s3://import-bucket/optional-prefix</code>. If a prefix is specified after the Amazon S3 bucket name, only object keys with that prefix are loaded into the file system.</p>
    #[serde(rename = "ImportPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_path: Option<String>,
    /// <p>For files imported from a data repository, this value determines the stripe count and maximum amount of data per file (in MiB) stored on a single physical disk. The maximum number of disks that a single file can be striped across is limited by the total number of disks that make up the file system.</p> <p>The default chunk size is 1,024 MiB (1 GiB) and can go as high as 512,000 MiB (500 GiB). Amazon S3 objects have a maximum size of 5 TB.</p>
    #[serde(rename = "ImportedFileChunkSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_file_chunk_size: Option<i64>,
}

/// <p>The request object for <code>DeleteBackup</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBackupRequest {
    /// <p>The ID of the backup you want to delete.</p>
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    /// <p>(Optional) A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent deletion. This is automatically filled on your behalf when using the AWS CLI or SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
}

/// <p>The response object for <code>DeleteBackup</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBackupResponse {
    /// <p>The ID of the backup deleted.</p>
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// <p>The lifecycle of the backup. Should be <code>DELETED</code>.</p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
}

/// <p>The request object for <code>DeleteFileSystem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFileSystemRequest {
    /// <p>(Optional) A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent deletion. This is automatically filled on your behalf when using the AWS CLI or SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The ID of the file system you want to delete.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "WindowsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<DeleteFileSystemWindowsConfiguration>,
}

/// <p>The response object for the <code>DeleteFileSystem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFileSystemResponse {
    /// <p>The ID of the file system being deleted.</p>
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// <p>The file system lifecycle for the deletion request. Should be <code>DELETING</code>.</p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "WindowsResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_response: Option<DeleteFileSystemWindowsResponse>,
}

/// <p>The configuration object for the Microsoft Windows file system used in the <code>DeleteFileSystem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFileSystemWindowsConfiguration {
    /// <p>A set of tags for your final backup.</p>
    #[serde(rename = "FinalBackupTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
    /// <p>By default, Amazon FSx for Windows takes a final backup on your behalf when the <code>DeleteFileSystem</code> operation is invoked. Doing this helps protect you from data loss, and we highly recommend taking the final backup. If you want to skip this backup, use this flag to do so.</p>
    #[serde(rename = "SkipFinalBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_backup: Option<bool>,
}

/// <p>The response object for the Microsoft Windows file system used in the <code>DeleteFileSystem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFileSystemWindowsResponse {
    /// <p>The ID of the final backup for this file system.</p>
    #[serde(rename = "FinalBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_id: Option<String>,
    /// <p>The set of tags applied to the final backup.</p>
    #[serde(rename = "FinalBackupTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
}

/// <p>The request object for <code>DescribeBackups</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBackupsRequest {
    /// <p>(Optional) IDs of the backups you want to retrieve (String). This overrides any filters. If any IDs are not found, BackupNotFound will be thrown.</p>
    #[serde(rename = "BackupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_ids: Option<Vec<String>>,
    /// <p>(Optional) Filters structure. Supported names are file-system-id and backup-type.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>(Optional) Maximum number of backups to return in the response (integer). This parameter value must be greater than 0. The number of items that Amazon FSx returns is the minimum of the <code>MaxResults</code> parameter specified in the request and the service's internal maximum number of items per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) Opaque pagination token returned from a previous <code>DescribeBackups</code> operation (String). If a token present, the action continues the list from where the returning call left off.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Response object for <code>DescribeBackups</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBackupsResponse {
    /// <p>Any array of backups.</p>
    #[serde(rename = "Backups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<Backup>>,
    /// <p>This is present if there are more backups than returned in the response (String). You can use the <code>NextToken</code> value in the later request to fetch the backups. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The request object for <code>DescribeFileSystems</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeFileSystemsRequest {
    /// <p>(Optional) IDs of the file systems whose descriptions you want to retrieve (String).</p>
    #[serde(rename = "FileSystemIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_ids: Option<Vec<String>>,
    /// <p>(Optional) Maximum number of file systems to return in the response (integer). This parameter value must be greater than 0. The number of items that Amazon FSx returns is the minimum of the <code>MaxResults</code> parameter specified in the request and the service's internal maximum number of items per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) Opaque pagination token returned from a previous <code>DescribeFileSystems</code> operation (String). If a token present, the action continues the list from where the returning call left off.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The response object for <code>DescribeFileSystems</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFileSystemsResponse {
    /// <p>An array of file system descriptions.</p>
    #[serde(rename = "FileSystems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_systems: Option<Vec<FileSystem>>,
    /// <p>Present if there are more file systems than returned in the response (String). You can use the <code>NextToken</code> value in the later request to fetch the descriptions. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A description of a specific Amazon FSx file system.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileSystem {
    /// <p>The time that the file system was created, in seconds (since 1970-01-01T00:00:00Z), also known as Unix time.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The DNS name for the file system.</p>
    #[serde(rename = "DNSName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "FailureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FileSystemFailureDetails>,
    /// <p>The system-generated, unique 17-digit ID of the file system.</p>
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// <p>The type of Amazon FSx file system, either <code>LUSTRE</code> or <code>WINDOWS</code>.</p>
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// <p>The ID of the AWS Key Management Service (AWS KMS) key used to encrypt the file system's data for an Amazon FSx for Windows File Server file system.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p><p>The lifecycle status of the file system:</p> <ul> <li> <p> <code>AVAILABLE</code> indicates that the file system is reachable and available for use.</p> </li> <li> <p> <code>CREATING</code> indicates that Amazon FSx is in the process of creating the new file system.</p> </li> <li> <p> <code>DELETING</code> indicates that Amazon FSx is in the process of deleting the file system.</p> </li> <li> <p> <code>FAILED</code> indicates that Amazon FSx was not able to create the file system.</p> </li> <li> <p> <code>MISCONFIGURED</code> indicates that the file system is in a failed but recoverable state.</p> </li> <li> <p> <code>UPDATING</code> indicates that the file system is undergoing a customer initiated update.</p> </li> </ul></p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<LustreFileSystemConfiguration>,
    /// <p>The IDs of the elastic network interface from which a specific file system is accessible. The elastic network interface is automatically created in the same VPC that the Amazon FSx file system was created in. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-eni.html">Elastic Network Interfaces</a> in the <i>Amazon EC2 User Guide.</i> </p> <p>For an Amazon FSx for Windows File Server file system, you can have one network interface ID. For an Amazon FSx for Lustre file system, you can have more than one.</p>
    #[serde(rename = "NetworkInterfaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_ids: Option<Vec<String>>,
    /// <p>The AWS account that created the file system. If the file system was created by an AWS Identity and Access Management (IAM) user, the AWS account to which the IAM user belongs is the owner.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the file system resource.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The storage capacity of the file system in gigabytes (GB).</p>
    #[serde(rename = "StorageCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<i64>,
    /// <p>The ID of the subnet to contain the endpoint for the file system. One and only one is supported. The file system is launched in the Availability Zone associated with this subnet.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The tags to associate with the file system. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html">Tagging Your Amazon EC2 Resources</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The ID of the primary VPC for the file system.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// <p>The configuration for this Microsoft Windows file system.</p>
    #[serde(rename = "WindowsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<WindowsFileSystemConfiguration>,
}

/// <p>A structure providing details of any failures that occur when creating the file system has failed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileSystemFailureDetails {
    /// <p>A message describing any failures that occurred during file system creation.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>A filter used to restrict the results of describe calls. You can use multiple filters to return results that meet all applied filter requirements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Filter {
    /// <p>The name for this filter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The values of the filter. These are all the values for any of the applied filters.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>The request object for <code>ListTagsForResource</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>(Optional) Maximum number of tags to return in the response (integer). This parameter value must be greater than 0. The number of items that Amazon FSx returns is the minimum of the <code>MaxResults</code> parameter specified in the request and the service's internal maximum number of items per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) Opaque pagination token returned from a previous <code>ListTagsForResource</code> operation (String). If a token present, the action continues the list from where the returning call left off.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the Amazon FSx resource that will have its tags listed.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

/// <p>The response object for <code>ListTagsForResource</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>This is present if there are more tags than returned in the response (String). You can use the <code>NextToken</code> value in the later request to fetch the tags. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tags on the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>The configuration for the Amazon FSx for Lustre file system.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LustreFileSystemConfiguration {
    #[serde(rename = "DataRepositoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_configuration: Option<DataRepositoryConfiguration>,
    /// <p>The UTC time that you want to begin your weekly maintenance window.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// <p>The configuration of the self-managed Microsoft Active Directory (AD) directory to which the Windows File Server instance is joined.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SelfManagedActiveDirectoryAttributes {
    /// <p>A list of up to two IP addresses of DNS servers or domain controllers in the self-managed AD directory.</p>
    #[serde(rename = "DnsIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ips: Option<Vec<String>>,
    /// <p>The fully qualified domain name of the self-managed AD directory.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The name of the domain group whose members have administrative privileges for the FSx file system.</p>
    #[serde(rename = "FileSystemAdministratorsGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_administrators_group: Option<String>,
    /// <p>The fully qualified distinguished name of the organizational unit within the self-managed AD directory to which the Windows File Server instance is joined.</p>
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<String>,
    /// <p>The user name for the service account on your self-managed AD domain that FSx uses to join to your AD domain.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>The configuration that Amazon FSx uses to join the Windows File Server instance to your self-managed (including on-premises) Microsoft Active Directory (AD) directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SelfManagedActiveDirectoryConfiguration {
    /// <p><p>A list of up to two IP addresses of DNS servers or domain controllers in the self-managed AD directory. The IP addresses need to be either in the same VPC CIDR range as the one in which your Amazon FSx file system is being created, or in the private IP version 4 (Iv4) address ranges, as specified in <a href="http://www.faqs.org/rfcs/rfc1918.html">RFC 1918</a>:</p> <ul> <li> <p>10.0.0.0 - 10.255.255.255 (10/8 prefix)</p> </li> <li> <p>172.16.0.0 - 172.31.255.255 (172.16/12 prefix)</p> </li> <li> <p>192.168.0.0 - 192.168.255.255 (192.168/16 prefix)</p> </li> </ul></p>
    #[serde(rename = "DnsIps")]
    pub dns_ips: Vec<String>,
    /// <p>The fully qualified domain name of the self-managed AD directory, such as <code>corp.example.com</code>.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>(Optional) The name of the domain group whose members are granted administrative privileges for the file system. Administrative privileges include taking ownership of files and folders, and setting audit controls (audit ACLs) on files and folders. The group that you specify must already exist in your domain. If you don't provide one, your AD domain's Domain Admins group is used.</p>
    #[serde(rename = "FileSystemAdministratorsGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_administrators_group: Option<String>,
    /// <p><p>(Optional) The fully qualified distinguished name of the organizational unit within your self-managed AD directory that the Windows File Server instance will join. Amazon FSx only accepts OU as the direct parent of the file system. An example is <code>OU=FSx,DC=yourdomain,DC=corp,DC=com</code>. To learn more, see <a href="https://tools.ietf.org/html/rfc2253">RFC 2253</a>. If none is provided, the FSx file system is created in the default location of your self-managed AD directory. </p> <important> <p>Only Organizational Unit (OU) objects can be the direct parent of the file system that you&#39;re creating.</p> </important></p>
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<String>,
    /// <p>The password for the service account on your self-managed AD domain that Amazon FSx will use to join to your AD domain.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>The user name for the service account on your self-managed AD domain that Amazon FSx will use to join to your AD domain. This account must have the permission to join computers to the domain in the organizational unit provided in <code>OrganizationalUnitDistinguishedName</code>, or in the default location of your AD domain.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

/// <p>The configuration that Amazon FSx uses to join the Windows File Server instance to the self-managed Microsoft Active Directory (AD) directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SelfManagedActiveDirectoryConfigurationUpdates {
    /// <p>A list of up to two IP addresses of DNS servers or domain controllers in the self-managed AD directory.</p>
    #[serde(rename = "DnsIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ips: Option<Vec<String>>,
    /// <p>The password for the service account on your self-managed AD domain that Amazon FSx will use to join to your AD domain.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The user name for the service account on your self-managed AD domain that Amazon FSx will use to join to your AD domain. This account must have the permission to join computers to the domain in the organizational unit provided in <code>OrganizationalUnitDistinguishedName</code>.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>Specifies a key-value pair for a resource tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>A value that specifies the <code>TagKey</code>, the name of the tag. Tag keys must be unique for the resource to which they are attached.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>A value that specifies the <code>TagValue</code>, the value assigned to the corresponding tag key. Tag values can be null and don't have to be unique in a tag set. For example, you can have a key-value pair in a tag set of <code>finances : April</code> and also of <code>payroll : April</code>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The request object for the <code>TagResource</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the Amazon FSx resource that you want to tag.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of tags for the resource. If a tag with a given key already exists, the value is replaced by the one specified in this parameter.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>The response object for the <code>TagResource</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>The request object for <code>UntagResource</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The ARN of the Amazon FSx resource to untag.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of keys of tags on the resource to untag. In case the tag key doesn't exist, the call will still succeed to be idempotent.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>The response object for <code>UntagResource</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>The configuration object for Amazon FSx for Lustre file systems used in the <code>UpdateFileSystem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFileSystemLustreConfiguration {
    /// <p>The preferred time to perform weekly maintenance, in the UTC time zone.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// <p>The request object for the <code>UpdateFileSystem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFileSystemRequest {
    /// <p>(Optional) A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent updates. This string is automatically filled on your behalf when you use the AWS Command Line Interface (AWS CLI) or an AWS SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "LustreConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<UpdateFileSystemLustreConfiguration>,
    /// <p>The configuration update for this Microsoft Windows file system. The only supported options are for backup and maintenance and for self-managed Active Directory configuration.</p>
    #[serde(rename = "WindowsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<UpdateFileSystemWindowsConfiguration>,
}

/// <p>The response object for the <code>UpdateFileSystem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFileSystemResponse {
    /// <p>A description of the file system that was updated.</p>
    #[serde(rename = "FileSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
}

/// <p>Updates the Microsoft Windows configuration for an existing Amazon FSx for Windows File Server file system. Amazon FSx overwrites existing properties with non-null values provided in the request. If you don't specify a non-null value for a property, that property is not updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFileSystemWindowsConfiguration {
    /// <p>The number of days to retain automatic backups. Setting this to 0 disables automatic backups. You can retain automatic backups for a maximum of 35 days.</p>
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i64>,
    /// <p>The preferred time to take daily automatic backups, in the UTC time zone.</p>
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    /// <p>The configuration Amazon FSx uses to join the Windows File Server instance to the self-managed Microsoft AD directory.</p>
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration:
        Option<SelfManagedActiveDirectoryConfigurationUpdates>,
    /// <p>The preferred time to perform weekly maintenance, in the UTC time zone.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// <p>The configuration for this Microsoft Windows file system.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WindowsFileSystemConfiguration {
    /// <p>The ID for an existing Microsoft Active Directory instance that the file system should join when it's created.</p>
    #[serde(rename = "ActiveDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    /// <p>The number of days to retain automatic backups. Setting this to 0 disables automatic backups. You can retain automatic backups for a maximum of 35 days.</p>
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i64>,
    /// <p>A boolean flag indicating whether tags on the file system should be copied to backups. This value defaults to false. If it's set to true, all tags on the file system are copied to all automatic backups and any user-initiated backups where the user doesn't specify any tags. If this value is true, and you specify one or more tags, only the specified tags are copied to backups.</p>
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    /// <p>The preferred time to take daily automatic backups, in the UTC time zone.</p>
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    /// <p>The list of maintenance operations in progress for this file system.</p>
    #[serde(rename = "MaintenanceOperationsInProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_operations_in_progress: Option<Vec<String>>,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration: Option<SelfManagedActiveDirectoryAttributes>,
    /// <p>The throughput of an Amazon FSx file system, measured in megabytes per second.</p>
    #[serde(rename = "ThroughputCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i64>,
    /// <p>The preferred time to perform weekly maintenance, in the UTC time zone.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// Errors returned by CreateBackup
#[derive(Debug, PartialEq)]
pub enum CreateBackupError {
    /// <p>Another backup is already under way. Wait for completion before initiating additional backups of this file system.</p>
    BackupInProgress(String),
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>An error indicating that a particular service limit was exceeded. You can increase some service limits by contacting AWS Support. </p>
    ServiceLimitExceeded(String),
    /// <p>An error occured.</p>
    UnsupportedOperation(String),
}

impl CreateBackupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BackupInProgress" => {
                    return RusotoError::Service(CreateBackupError::BackupInProgress(err.msg))
                }
                "BadRequest" => {
                    return RusotoError::Service(CreateBackupError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(CreateBackupError::FileSystemNotFound(err.msg))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(CreateBackupError::IncompatibleParameterError(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateBackupError::InternalServerError(err.msg))
                }
                "ServiceLimitExceeded" => {
                    return RusotoError::Service(CreateBackupError::ServiceLimitExceeded(err.msg))
                }
                "UnsupportedOperation" => {
                    return RusotoError::Service(CreateBackupError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateBackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBackupError {
    fn description(&self) -> &str {
        match *self {
            CreateBackupError::BackupInProgress(ref cause) => cause,
            CreateBackupError::BadRequest(ref cause) => cause,
            CreateBackupError::FileSystemNotFound(ref cause) => cause,
            CreateBackupError::IncompatibleParameterError(ref cause) => cause,
            CreateBackupError::InternalServerError(ref cause) => cause,
            CreateBackupError::ServiceLimitExceeded(ref cause) => cause,
            CreateBackupError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFileSystem
#[derive(Debug, PartialEq)]
pub enum CreateFileSystemError {
    /// <p>An Active Directory error.</p>
    ActiveDirectoryError(String),
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>The path provided for data repository export isn't valid.</p>
    InvalidExportPath(String),
    /// <p>The path provided for data repository import isn't valid.</p>
    InvalidImportPath(String),
    /// <p>One or more network settings specified in the request are invalid. <code>InvalidVpcId</code> means that the ID passed for the virtual private cloud (VPC) is invalid. <code>InvalidSubnetIds</code> returns the list of IDs for subnets that are either invalid or not part of the VPC specified. <code>InvalidSecurityGroupIds</code> returns the list of IDs for security groups that are either invalid or not part of the VPC specified.</p>
    InvalidNetworkSettings(String),
    /// <p>File system configuration is required for this operation.</p>
    MissingFileSystemConfiguration(String),
    /// <p>An error indicating that a particular service limit was exceeded. You can increase some service limits by contacting AWS Support. </p>
    ServiceLimitExceeded(String),
}

impl CreateFileSystemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFileSystemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActiveDirectoryError" => {
                    return RusotoError::Service(CreateFileSystemError::ActiveDirectoryError(
                        err.msg,
                    ))
                }
                "BadRequest" => {
                    return RusotoError::Service(CreateFileSystemError::BadRequest(err.msg))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(CreateFileSystemError::IncompatibleParameterError(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateFileSystemError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidExportPath" => {
                    return RusotoError::Service(CreateFileSystemError::InvalidExportPath(err.msg))
                }
                "InvalidImportPath" => {
                    return RusotoError::Service(CreateFileSystemError::InvalidImportPath(err.msg))
                }
                "InvalidNetworkSettings" => {
                    return RusotoError::Service(CreateFileSystemError::InvalidNetworkSettings(
                        err.msg,
                    ))
                }
                "MissingFileSystemConfiguration" => {
                    return RusotoError::Service(
                        CreateFileSystemError::MissingFileSystemConfiguration(err.msg),
                    )
                }
                "ServiceLimitExceeded" => {
                    return RusotoError::Service(CreateFileSystemError::ServiceLimitExceeded(
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
impl fmt::Display for CreateFileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFileSystemError {
    fn description(&self) -> &str {
        match *self {
            CreateFileSystemError::ActiveDirectoryError(ref cause) => cause,
            CreateFileSystemError::BadRequest(ref cause) => cause,
            CreateFileSystemError::IncompatibleParameterError(ref cause) => cause,
            CreateFileSystemError::InternalServerError(ref cause) => cause,
            CreateFileSystemError::InvalidExportPath(ref cause) => cause,
            CreateFileSystemError::InvalidImportPath(ref cause) => cause,
            CreateFileSystemError::InvalidNetworkSettings(ref cause) => cause,
            CreateFileSystemError::MissingFileSystemConfiguration(ref cause) => cause,
            CreateFileSystemError::ServiceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFileSystemFromBackup
#[derive(Debug, PartialEq)]
pub enum CreateFileSystemFromBackupError {
    /// <p>An Active Directory error.</p>
    ActiveDirectoryError(String),
    /// <p>No Amazon FSx backups were found based upon the supplied parameters.</p>
    BackupNotFound(String),
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>One or more network settings specified in the request are invalid. <code>InvalidVpcId</code> means that the ID passed for the virtual private cloud (VPC) is invalid. <code>InvalidSubnetIds</code> returns the list of IDs for subnets that are either invalid or not part of the VPC specified. <code>InvalidSecurityGroupIds</code> returns the list of IDs for security groups that are either invalid or not part of the VPC specified.</p>
    InvalidNetworkSettings(String),
    /// <p>File system configuration is required for this operation.</p>
    MissingFileSystemConfiguration(String),
    /// <p>An error indicating that a particular service limit was exceeded. You can increase some service limits by contacting AWS Support. </p>
    ServiceLimitExceeded(String),
}

impl CreateFileSystemFromBackupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateFileSystemFromBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActiveDirectoryError" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::ActiveDirectoryError(err.msg),
                    )
                }
                "BackupNotFound" => {
                    return RusotoError::Service(CreateFileSystemFromBackupError::BackupNotFound(
                        err.msg,
                    ))
                }
                "BadRequest" => {
                    return RusotoError::Service(CreateFileSystemFromBackupError::BadRequest(
                        err.msg,
                    ))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::IncompatibleParameterError(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::InternalServerError(err.msg),
                    )
                }
                "InvalidNetworkSettings" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::InvalidNetworkSettings(err.msg),
                    )
                }
                "MissingFileSystemConfiguration" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::MissingFileSystemConfiguration(err.msg),
                    )
                }
                "ServiceLimitExceeded" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::ServiceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateFileSystemFromBackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFileSystemFromBackupError {
    fn description(&self) -> &str {
        match *self {
            CreateFileSystemFromBackupError::ActiveDirectoryError(ref cause) => cause,
            CreateFileSystemFromBackupError::BackupNotFound(ref cause) => cause,
            CreateFileSystemFromBackupError::BadRequest(ref cause) => cause,
            CreateFileSystemFromBackupError::IncompatibleParameterError(ref cause) => cause,
            CreateFileSystemFromBackupError::InternalServerError(ref cause) => cause,
            CreateFileSystemFromBackupError::InvalidNetworkSettings(ref cause) => cause,
            CreateFileSystemFromBackupError::MissingFileSystemConfiguration(ref cause) => cause,
            CreateFileSystemFromBackupError::ServiceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBackup
#[derive(Debug, PartialEq)]
pub enum DeleteBackupError {
    /// <p>Another backup is already under way. Wait for completion before initiating additional backups of this file system.</p>
    BackupInProgress(String),
    /// <p>No Amazon FSx backups were found based upon the supplied parameters.</p>
    BackupNotFound(String),
    /// <p>You can't delete a backup while it's being used to restore a file system.</p>
    BackupRestoring(String),
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
}

impl DeleteBackupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BackupInProgress" => {
                    return RusotoError::Service(DeleteBackupError::BackupInProgress(err.msg))
                }
                "BackupNotFound" => {
                    return RusotoError::Service(DeleteBackupError::BackupNotFound(err.msg))
                }
                "BackupRestoring" => {
                    return RusotoError::Service(DeleteBackupError::BackupRestoring(err.msg))
                }
                "BadRequest" => {
                    return RusotoError::Service(DeleteBackupError::BadRequest(err.msg))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(DeleteBackupError::IncompatibleParameterError(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteBackupError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteBackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBackupError {
    fn description(&self) -> &str {
        match *self {
            DeleteBackupError::BackupInProgress(ref cause) => cause,
            DeleteBackupError::BackupNotFound(ref cause) => cause,
            DeleteBackupError::BackupRestoring(ref cause) => cause,
            DeleteBackupError::BadRequest(ref cause) => cause,
            DeleteBackupError::IncompatibleParameterError(ref cause) => cause,
            DeleteBackupError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFileSystem
#[derive(Debug, PartialEq)]
pub enum DeleteFileSystemError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>An error indicating that a particular service limit was exceeded. You can increase some service limits by contacting AWS Support. </p>
    ServiceLimitExceeded(String),
}

impl DeleteFileSystemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFileSystemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DeleteFileSystemError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(DeleteFileSystemError::FileSystemNotFound(err.msg))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(DeleteFileSystemError::IncompatibleParameterError(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteFileSystemError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceLimitExceeded" => {
                    return RusotoError::Service(DeleteFileSystemError::ServiceLimitExceeded(
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
impl fmt::Display for DeleteFileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFileSystemError {
    fn description(&self) -> &str {
        match *self {
            DeleteFileSystemError::BadRequest(ref cause) => cause,
            DeleteFileSystemError::FileSystemNotFound(ref cause) => cause,
            DeleteFileSystemError::IncompatibleParameterError(ref cause) => cause,
            DeleteFileSystemError::InternalServerError(ref cause) => cause,
            DeleteFileSystemError::ServiceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBackups
#[derive(Debug, PartialEq)]
pub enum DescribeBackupsError {
    /// <p>No Amazon FSx backups were found based upon the supplied parameters.</p>
    BackupNotFound(String),
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
}

impl DescribeBackupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBackupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BackupNotFound" => {
                    return RusotoError::Service(DescribeBackupsError::BackupNotFound(err.msg))
                }
                "BadRequest" => {
                    return RusotoError::Service(DescribeBackupsError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(DescribeBackupsError::FileSystemNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeBackupsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeBackupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBackupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeBackupsError::BackupNotFound(ref cause) => cause,
            DescribeBackupsError::BadRequest(ref cause) => cause,
            DescribeBackupsError::FileSystemNotFound(ref cause) => cause,
            DescribeBackupsError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFileSystems
#[derive(Debug, PartialEq)]
pub enum DescribeFileSystemsError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
}

impl DescribeFileSystemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFileSystemsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DescribeFileSystemsError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(DescribeFileSystemsError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeFileSystemsError::InternalServerError(
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
impl fmt::Display for DescribeFileSystemsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFileSystemsError {
    fn description(&self) -> &str {
        match *self {
            DescribeFileSystemsError::BadRequest(ref cause) => cause,
            DescribeFileSystemsError::FileSystemNotFound(ref cause) => cause,
            DescribeFileSystemsError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>The resource specified for the tagging operation is not a resource type owned by Amazon FSx. Use the API of the relevant service to perform the operation. </p>
    NotServiceResourceError(String),
    /// <p>The resource specified does not support tagging. </p>
    ResourceDoesNotSupportTagging(String),
    /// <p>The resource specified by the Amazon Resource Name (ARN) can't be found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotServiceResourceError" => {
                    return RusotoError::Service(ListTagsForResourceError::NotServiceResourceError(
                        err.msg,
                    ))
                }
                "ResourceDoesNotSupportTagging" => {
                    return RusotoError::Service(
                        ListTagsForResourceError::ResourceDoesNotSupportTagging(err.msg),
                    )
                }
                "ResourceNotFound" => {
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
            ListTagsForResourceError::BadRequest(ref cause) => cause,
            ListTagsForResourceError::InternalServerError(ref cause) => cause,
            ListTagsForResourceError::NotServiceResourceError(ref cause) => cause,
            ListTagsForResourceError::ResourceDoesNotSupportTagging(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>The resource specified for the tagging operation is not a resource type owned by Amazon FSx. Use the API of the relevant service to perform the operation. </p>
    NotServiceResourceError(String),
    /// <p>The resource specified does not support tagging. </p>
    ResourceDoesNotSupportTagging(String),
    /// <p>The resource specified by the Amazon Resource Name (ARN) can't be found.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => return RusotoError::Service(TagResourceError::BadRequest(err.msg)),
                "InternalServerError" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
                }
                "NotServiceResourceError" => {
                    return RusotoError::Service(TagResourceError::NotServiceResourceError(err.msg))
                }
                "ResourceDoesNotSupportTagging" => {
                    return RusotoError::Service(TagResourceError::ResourceDoesNotSupportTagging(
                        err.msg,
                    ))
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::InternalServerError(ref cause) => cause,
            TagResourceError::NotServiceResourceError(ref cause) => cause,
            TagResourceError::ResourceDoesNotSupportTagging(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>The resource specified for the tagging operation is not a resource type owned by Amazon FSx. Use the API of the relevant service to perform the operation. </p>
    NotServiceResourceError(String),
    /// <p>The resource specified does not support tagging. </p>
    ResourceDoesNotSupportTagging(String),
    /// <p>The resource specified by the Amazon Resource Name (ARN) can't be found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
                }
                "NotServiceResourceError" => {
                    return RusotoError::Service(UntagResourceError::NotServiceResourceError(
                        err.msg,
                    ))
                }
                "ResourceDoesNotSupportTagging" => {
                    return RusotoError::Service(UntagResourceError::ResourceDoesNotSupportTagging(
                        err.msg,
                    ))
                }
                "ResourceNotFound" => {
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
            UntagResourceError::BadRequest(ref cause) => cause,
            UntagResourceError::InternalServerError(ref cause) => cause,
            UntagResourceError::NotServiceResourceError(ref cause) => cause,
            UntagResourceError::ResourceDoesNotSupportTagging(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFileSystem
#[derive(Debug, PartialEq)]
pub enum UpdateFileSystemError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>File system configuration is required for this operation.</p>
    MissingFileSystemConfiguration(String),
    /// <p>An error occured.</p>
    UnsupportedOperation(String),
}

impl UpdateFileSystemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFileSystemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(UpdateFileSystemError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(UpdateFileSystemError::FileSystemNotFound(err.msg))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(UpdateFileSystemError::IncompatibleParameterError(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(UpdateFileSystemError::InternalServerError(
                        err.msg,
                    ))
                }
                "MissingFileSystemConfiguration" => {
                    return RusotoError::Service(
                        UpdateFileSystemError::MissingFileSystemConfiguration(err.msg),
                    )
                }
                "UnsupportedOperation" => {
                    return RusotoError::Service(UpdateFileSystemError::UnsupportedOperation(
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
impl fmt::Display for UpdateFileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFileSystemError {
    fn description(&self) -> &str {
        match *self {
            UpdateFileSystemError::BadRequest(ref cause) => cause,
            UpdateFileSystemError::FileSystemNotFound(ref cause) => cause,
            UpdateFileSystemError::IncompatibleParameterError(ref cause) => cause,
            UpdateFileSystemError::InternalServerError(ref cause) => cause,
            UpdateFileSystemError::MissingFileSystemConfiguration(ref cause) => cause,
            UpdateFileSystemError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon FSx API. Amazon FSx clients implement this trait.
pub trait Fsx {
    /// <p><p>Creates a backup of an existing Amazon FSx for Windows File Server file system. Creating regular backups for your file system is a best practice that complements the replication that Amazon FSx for Windows File Server performs for your file system. It also enables you to restore from user modification of data.</p> <p>If a backup with the specified client request token exists, and the parameters match, this operation returns the description of the existing backup. If a backup specified client request token exists, and the parameters don&#39;t match, this operation returns <code>IncompatibleParameterError</code>. If a backup with the specified client request token doesn&#39;t exist, <code>CreateBackup</code> does the following: </p> <ul> <li> <p>Creates a new Amazon FSx backup with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the backup.</p> </li> </ul> <p>By using the idempotent operation, you can retry a <code>CreateBackup</code> operation without the risk of creating an extra backup. This approach can be useful when an initial call fails in a way that makes it unclear whether a backup was created. If you use the same client request token and the initial call created a backup, the operation returns a successful result because all the parameters are the same.</p> <p>The <code>CreateFileSystem</code> operation returns while the backup&#39;s lifecycle state is still <code>CREATING</code>. You can check the file system creation status by calling the <a>DescribeBackups</a> operation, which returns the backup state along with other information.</p> <note> <p/> </note></p>
    fn create_backup(
        &self,
        input: CreateBackupRequest,
    ) -> RusotoFuture<CreateBackupResponse, CreateBackupError>;

    /// <p><p>Creates a new, empty Amazon FSx file system.</p> <p>If a file system with the specified client request token exists and the parameters match, <code>CreateFileSystem</code> returns the description of the existing file system. If a file system specified client request token exists and the parameters don&#39;t match, this call returns <code>IncompatibleParameterError</code>. If a file system with the specified client request token doesn&#39;t exist, <code>CreateFileSystem</code> does the following: </p> <ul> <li> <p>Creates a new, empty Amazon FSx file system with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the file system.</p> </li> </ul> <p>This operation requires a client request token in the request that Amazon FSx uses to ensure idempotent creation. This means that calling the operation multiple times with the same client request token has no effect. By using the idempotent operation, you can retry a <code>CreateFileSystem</code> operation without the risk of creating an extra file system. This approach can be useful when an initial call fails in a way that makes it unclear whether a file system was created. Examples are if a transport level timeout occurred, or your connection was reset. If you use the same client request token and the initial call created a file system, the client receives success as long as the parameters are the same.</p> <note> <p>The <code>CreateFileSystem</code> call returns while the file system&#39;s lifecycle state is still <code>CREATING</code>. You can check the file-system creation status by calling the <a>DescribeFileSystems</a> operation, which returns the file system state along with other information.</p> </note></p>
    fn create_file_system(
        &self,
        input: CreateFileSystemRequest,
    ) -> RusotoFuture<CreateFileSystemResponse, CreateFileSystemError>;

    /// <p><p>Creates a new Amazon FSx file system from an existing Amazon FSx for Windows File Server backup.</p> <p>If a file system with the specified client request token exists and the parameters match, this operation returns the description of the file system. If a client request token specified by the file system exists and the parameters don&#39;t match, this call returns <code>IncompatibleParameterError</code>. If a file system with the specified client request token doesn&#39;t exist, this operation does the following:</p> <ul> <li> <p>Creates a new Amazon FSx file system from backup with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the file system.</p> </li> </ul> <p>Parameters like Active Directory, default share name, automatic backup, and backup settings default to the parameters of the file system that was backed up, unless overridden. You can explicitly supply other settings.</p> <p>By using the idempotent operation, you can retry a <code>CreateFileSystemFromBackup</code> call without the risk of creating an extra file system. This approach can be useful when an initial call fails in a way that makes it unclear whether a file system was created. Examples are if a transport level timeout occurred, or your connection was reset. If you use the same client request token and the initial call created a file system, the client receives success as long as the parameters are the same.</p> <note> <p>The <code>CreateFileSystemFromBackup</code> call returns while the file system&#39;s lifecycle state is still <code>CREATING</code>. You can check the file-system creation status by calling the <a>DescribeFileSystems</a> operation, which returns the file system state along with other information.</p> </note></p>
    fn create_file_system_from_backup(
        &self,
        input: CreateFileSystemFromBackupRequest,
    ) -> RusotoFuture<CreateFileSystemFromBackupResponse, CreateFileSystemFromBackupError>;

    /// <p><p>Deletes an Amazon FSx for Windows File Server backup, deleting its contents. After deletion, the backup no longer exists, and its data is gone.</p> <p>The <code>DeleteBackup</code> call returns instantly. The backup will not show up in later <code>DescribeBackups</code> calls.</p> <important> <p>The data in a deleted backup is also deleted and can&#39;t be recovered by any means.</p> </important></p>
    fn delete_backup(
        &self,
        input: DeleteBackupRequest,
    ) -> RusotoFuture<DeleteBackupResponse, DeleteBackupError>;

    /// <p><p>Deletes a file system, deleting its contents. After deletion, the file system no longer exists, and its data is gone. Any existing automatic backups will also be deleted.</p> <p>By default, when you delete an Amazon FSx for Windows File Server file system, a final backup is created upon deletion. This final backup is not subject to the file system&#39;s retention policy, and must be manually deleted.</p> <p>The <code>DeleteFileSystem</code> action returns while the file system has the <code>DELETING</code> status. You can check the file system deletion status by calling the <a>DescribeFileSystems</a> action, which returns a list of file systems in your account. If you pass the file system ID for a deleted file system, the <a>DescribeFileSystems</a> returns a <code>FileSystemNotFound</code> error.</p> <important> <p>The data in a deleted file system is also deleted and can&#39;t be recovered by any means.</p> </important></p>
    fn delete_file_system(
        &self,
        input: DeleteFileSystemRequest,
    ) -> RusotoFuture<DeleteFileSystemResponse, DeleteFileSystemError>;

    /// <p><p>Returns the description of specific Amazon FSx for Windows File Server backups, if a <code>BackupIds</code> value is provided for that backup. Otherwise, it returns all backups owned by your AWS account in the AWS Region of the endpoint that you&#39;re calling.</p> <p>When retrieving all backups, you can optionally specify the <code>MaxResults</code> parameter to limit the number of backups in a response. If more backups remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your backups. <code>DescribeBackups</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of backups returned in the response of one <code>DescribeBackups</code> call and the order of backups returned across the responses of a multi-call iteration is unspecified.</p> </li> </ul></p>
    fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> RusotoFuture<DescribeBackupsResponse, DescribeBackupsError>;

    /// <p><p>Returns the description of specific Amazon FSx file systems, if a <code>FileSystemIds</code> value is provided for that file system. Otherwise, it returns descriptions of all file systems owned by your AWS account in the AWS Region of the endpoint that you&#39;re calling.</p> <p>When retrieving all file system descriptions, you can optionally specify the <code>MaxResults</code> parameter to limit the number of descriptions in a response. If more file system descriptions remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your file system descriptions. <code>DescribeFileSystems</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of file systems returned in the response of one <code>DescribeFileSystems</code> call and the order of file systems returned across the responses of a multicall iteration is unspecified.</p> </li> </ul></p>
    fn describe_file_systems(
        &self,
        input: DescribeFileSystemsRequest,
    ) -> RusotoFuture<DescribeFileSystemsResponse, DescribeFileSystemsError>;

    /// <p><p>Lists tags for an Amazon FSx file systems and backups in the case of Amazon FSx for Windows File Server.</p> <p>When retrieving all tags, you can optionally specify the <code>MaxResults</code> parameter to limit the number of tags in a response. If more tags remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your tags. <code>ListTagsForResource</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of tags returned in the response of one <code>ListTagsForResource</code> call and the order of tags returned across the responses of a multi-call iteration is unspecified.</p> </li> </ul></p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Tags an Amazon FSx resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>This action removes a tag from an Amazon FSx resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Updates a file system configuration.</p>
    fn update_file_system(
        &self,
        input: UpdateFileSystemRequest,
    ) -> RusotoFuture<UpdateFileSystemResponse, UpdateFileSystemError>;
}
/// A client for the Amazon FSx API.
#[derive(Clone)]
pub struct FsxClient {
    client: Client,
    region: region::Region,
}

impl FsxClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> FsxClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> FsxClient
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

    pub fn new_with_client(client: Client, region: region::Region) -> FsxClient {
        FsxClient { client, region }
    }
}

impl fmt::Debug for FsxClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FsxClient")
            .field("region", &self.region)
            .finish()
    }
}

impl Fsx for FsxClient {
    /// <p><p>Creates a backup of an existing Amazon FSx for Windows File Server file system. Creating regular backups for your file system is a best practice that complements the replication that Amazon FSx for Windows File Server performs for your file system. It also enables you to restore from user modification of data.</p> <p>If a backup with the specified client request token exists, and the parameters match, this operation returns the description of the existing backup. If a backup specified client request token exists, and the parameters don&#39;t match, this operation returns <code>IncompatibleParameterError</code>. If a backup with the specified client request token doesn&#39;t exist, <code>CreateBackup</code> does the following: </p> <ul> <li> <p>Creates a new Amazon FSx backup with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the backup.</p> </li> </ul> <p>By using the idempotent operation, you can retry a <code>CreateBackup</code> operation without the risk of creating an extra backup. This approach can be useful when an initial call fails in a way that makes it unclear whether a backup was created. If you use the same client request token and the initial call created a backup, the operation returns a successful result because all the parameters are the same.</p> <p>The <code>CreateFileSystem</code> operation returns while the backup&#39;s lifecycle state is still <code>CREATING</code>. You can check the file system creation status by calling the <a>DescribeBackups</a> operation, which returns the backup state along with other information.</p> <note> <p/> </note></p>
    fn create_backup(
        &self,
        input: CreateBackupRequest,
    ) -> RusotoFuture<CreateBackupResponse, CreateBackupError> {
        let mut request = SignedRequest::new("POST", "fsx", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSimbaAPIService_v20180301.CreateBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateBackupResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateBackupError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Creates a new, empty Amazon FSx file system.</p> <p>If a file system with the specified client request token exists and the parameters match, <code>CreateFileSystem</code> returns the description of the existing file system. If a file system specified client request token exists and the parameters don&#39;t match, this call returns <code>IncompatibleParameterError</code>. If a file system with the specified client request token doesn&#39;t exist, <code>CreateFileSystem</code> does the following: </p> <ul> <li> <p>Creates a new, empty Amazon FSx file system with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the file system.</p> </li> </ul> <p>This operation requires a client request token in the request that Amazon FSx uses to ensure idempotent creation. This means that calling the operation multiple times with the same client request token has no effect. By using the idempotent operation, you can retry a <code>CreateFileSystem</code> operation without the risk of creating an extra file system. This approach can be useful when an initial call fails in a way that makes it unclear whether a file system was created. Examples are if a transport level timeout occurred, or your connection was reset. If you use the same client request token and the initial call created a file system, the client receives success as long as the parameters are the same.</p> <note> <p>The <code>CreateFileSystem</code> call returns while the file system&#39;s lifecycle state is still <code>CREATING</code>. You can check the file-system creation status by calling the <a>DescribeFileSystems</a> operation, which returns the file system state along with other information.</p> </note></p>
    fn create_file_system(
        &self,
        input: CreateFileSystemRequest,
    ) -> RusotoFuture<CreateFileSystemResponse, CreateFileSystemError> {
        let mut request = SignedRequest::new("POST", "fsx", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.CreateFileSystem",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateFileSystemResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateFileSystemError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Creates a new Amazon FSx file system from an existing Amazon FSx for Windows File Server backup.</p> <p>If a file system with the specified client request token exists and the parameters match, this operation returns the description of the file system. If a client request token specified by the file system exists and the parameters don&#39;t match, this call returns <code>IncompatibleParameterError</code>. If a file system with the specified client request token doesn&#39;t exist, this operation does the following:</p> <ul> <li> <p>Creates a new Amazon FSx file system from backup with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the file system.</p> </li> </ul> <p>Parameters like Active Directory, default share name, automatic backup, and backup settings default to the parameters of the file system that was backed up, unless overridden. You can explicitly supply other settings.</p> <p>By using the idempotent operation, you can retry a <code>CreateFileSystemFromBackup</code> call without the risk of creating an extra file system. This approach can be useful when an initial call fails in a way that makes it unclear whether a file system was created. Examples are if a transport level timeout occurred, or your connection was reset. If you use the same client request token and the initial call created a file system, the client receives success as long as the parameters are the same.</p> <note> <p>The <code>CreateFileSystemFromBackup</code> call returns while the file system&#39;s lifecycle state is still <code>CREATING</code>. You can check the file-system creation status by calling the <a>DescribeFileSystems</a> operation, which returns the file system state along with other information.</p> </note></p>
    fn create_file_system_from_backup(
        &self,
        input: CreateFileSystemFromBackupRequest,
    ) -> RusotoFuture<CreateFileSystemFromBackupResponse, CreateFileSystemFromBackupError> {
        let mut request = SignedRequest::new("POST", "fsx", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.CreateFileSystemFromBackup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateFileSystemFromBackupResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateFileSystemFromBackupError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Deletes an Amazon FSx for Windows File Server backup, deleting its contents. After deletion, the backup no longer exists, and its data is gone.</p> <p>The <code>DeleteBackup</code> call returns instantly. The backup will not show up in later <code>DescribeBackups</code> calls.</p> <important> <p>The data in a deleted backup is also deleted and can&#39;t be recovered by any means.</p> </important></p>
    fn delete_backup(
        &self,
        input: DeleteBackupRequest,
    ) -> RusotoFuture<DeleteBackupResponse, DeleteBackupError> {
        let mut request = SignedRequest::new("POST", "fsx", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSimbaAPIService_v20180301.DeleteBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteBackupResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBackupError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Deletes a file system, deleting its contents. After deletion, the file system no longer exists, and its data is gone. Any existing automatic backups will also be deleted.</p> <p>By default, when you delete an Amazon FSx for Windows File Server file system, a final backup is created upon deletion. This final backup is not subject to the file system&#39;s retention policy, and must be manually deleted.</p> <p>The <code>DeleteFileSystem</code> action returns while the file system has the <code>DELETING</code> status. You can check the file system deletion status by calling the <a>DescribeFileSystems</a> action, which returns a list of file systems in your account. If you pass the file system ID for a deleted file system, the <a>DescribeFileSystems</a> returns a <code>FileSystemNotFound</code> error.</p> <important> <p>The data in a deleted file system is also deleted and can&#39;t be recovered by any means.</p> </important></p>
    fn delete_file_system(
        &self,
        input: DeleteFileSystemRequest,
    ) -> RusotoFuture<DeleteFileSystemResponse, DeleteFileSystemError> {
        let mut request = SignedRequest::new("POST", "fsx", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.DeleteFileSystem",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteFileSystemResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteFileSystemError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Returns the description of specific Amazon FSx for Windows File Server backups, if a <code>BackupIds</code> value is provided for that backup. Otherwise, it returns all backups owned by your AWS account in the AWS Region of the endpoint that you&#39;re calling.</p> <p>When retrieving all backups, you can optionally specify the <code>MaxResults</code> parameter to limit the number of backups in a response. If more backups remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your backups. <code>DescribeBackups</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of backups returned in the response of one <code>DescribeBackups</code> call and the order of backups returned across the responses of a multi-call iteration is unspecified.</p> </li> </ul></p>
    fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> RusotoFuture<DescribeBackupsResponse, DescribeBackupsError> {
        let mut request = SignedRequest::new("POST", "fsx", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.DescribeBackups",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeBackupsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeBackupsError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Returns the description of specific Amazon FSx file systems, if a <code>FileSystemIds</code> value is provided for that file system. Otherwise, it returns descriptions of all file systems owned by your AWS account in the AWS Region of the endpoint that you&#39;re calling.</p> <p>When retrieving all file system descriptions, you can optionally specify the <code>MaxResults</code> parameter to limit the number of descriptions in a response. If more file system descriptions remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your file system descriptions. <code>DescribeFileSystems</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of file systems returned in the response of one <code>DescribeFileSystems</code> call and the order of file systems returned across the responses of a multicall iteration is unspecified.</p> </li> </ul></p>
    fn describe_file_systems(
        &self,
        input: DescribeFileSystemsRequest,
    ) -> RusotoFuture<DescribeFileSystemsResponse, DescribeFileSystemsError> {
        let mut request = SignedRequest::new("POST", "fsx", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.DescribeFileSystems",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeFileSystemsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeFileSystemsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Lists tags for an Amazon FSx file systems and backups in the case of Amazon FSx for Windows File Server.</p> <p>When retrieving all tags, you can optionally specify the <code>MaxResults</code> parameter to limit the number of tags in a response. If more tags remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your tags. <code>ListTagsForResource</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of tags returned in the response of one <code>ListTagsForResource</code> call and the order of tags returned across the responses of a multi-call iteration is unspecified.</p> </li> </ul></p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "fsx", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResponse, _>()
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

    /// <p>Tags an Amazon FSx resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "fsx", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSimbaAPIService_v20180301.TagResource");
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

    /// <p>This action removes a tag from an Amazon FSx resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "fsx", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSimbaAPIService_v20180301.UntagResource");
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

    /// <p>Updates a file system configuration.</p>
    fn update_file_system(
        &self,
        input: UpdateFileSystemRequest,
    ) -> RusotoFuture<UpdateFileSystemResponse, UpdateFileSystemError> {
        let mut request = SignedRequest::new("POST", "fsx", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.UpdateFileSystem",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateFileSystemResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateFileSystemError::from_response(response))),
                )
            }
        })
    }
}
