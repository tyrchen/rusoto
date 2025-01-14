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
/// <p>The address that you want the Snowball or Snowballs associated with a specific job to be shipped to. Addresses are validated at the time of creation. The address you provide must be located within the serviceable area of your region. Although no individual elements of the <code>Address</code> are required, if the address is invalid or unsupported, then an exception is thrown.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// <p>The unique ID for an address.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The city in an address that a Snowball is to be delivered to.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// <p>The name of the company to receive a Snowball at an address.</p>
    #[serde(rename = "Company")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// <p>The country in an address that a Snowball is to be delivered to.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>If the address you are creating is a primary address, then set this option to true. This field is not supported in most regions.</p>
    #[serde(rename = "IsRestricted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_restricted: Option<bool>,
    /// <p>This field is no longer used and the value is ignored.</p>
    #[serde(rename = "Landmark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmark: Option<String>,
    /// <p>The name of a person to receive a Snowball at an address.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The phone number associated with an address that a Snowball is to be delivered to.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// <p>The postal code in an address that a Snowball is to be delivered to.</p>
    #[serde(rename = "PostalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// <p>This field is no longer used and the value is ignored.</p>
    #[serde(rename = "PrefectureOrDistrict")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefecture_or_district: Option<String>,
    /// <p>The state or province in an address that a Snowball is to be delivered to.</p>
    #[serde(rename = "StateOrProvince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    /// <p>The first line in a street address that a Snowball is to be delivered to.</p>
    #[serde(rename = "Street1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_1: Option<String>,
    /// <p>The second line in a street address that a Snowball is to be delivered to.</p>
    #[serde(rename = "Street2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    /// <p>The third line in a street address that a Snowball is to be delivered to.</p>
    #[serde(rename = "Street3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_3: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelClusterRequest {
    /// <p>The 39-character ID for the cluster that you want to cancel, for example <code>CID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelClusterResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelJobRequest {
    /// <p>The 39-character job ID for the job that you want to cancel, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelJobResult {}

/// <p>Contains a cluster's state, a cluster's ID, and other important information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClusterListEntry {
    /// <p>The 39-character ID for the cluster that you want to list, for example <code>CID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The current state of this cluster. For information about the state of a specific node, see <a>JobListEntry$JobState</a>.</p>
    #[serde(rename = "ClusterState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<String>,
    /// <p>The creation date for this cluster.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>Defines an optional description of the cluster, for example <code>Environmental Data Cluster-01</code>.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p>Contains metadata about a specific cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClusterMetadata {
    /// <p>The automatically generated ID for a specific address.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The automatically generated ID for a cluster.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The current status of the cluster.</p>
    #[serde(rename = "ClusterState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<String>,
    /// <p>The creation date for this cluster.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The optional description of the cluster.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the address that you want a cluster shipped to, after it will be shipped to its primary address. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>The type of job for this cluster. Currently, the only job type supported for clusters is <code>LOCAL_USE</code>.</p>
    #[serde(rename = "JobType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    /// <p>The <code>KmsKeyARN</code> Amazon Resource Name (ARN) associated with this cluster. This ARN was created using the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> API action in AWS Key Management Service (AWS KMS).</p>
    #[serde(rename = "KmsKeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The Amazon Simple Notification Service (Amazon SNS) notification settings for this cluster.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>The arrays of <a>JobResource</a> objects that can include updated <a>S3Resource</a> objects or <a>LambdaResource</a> objects.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResource>,
    /// <p>The role ARN associated with this cluster. This ARN was created using the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> API action in AWS Identity and Access Management (IAM).</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>The shipping speed for each node in this cluster. This speed doesn&#39;t dictate how soon you&#39;ll get each device, rather it represents how quickly each device moves to its destination while in transit. Regional shipping speeds are as follows:</p> <ul> <li> <p>In Australia, you have access to express shipping. Typically, devices shipped express are delivered in about a day.</p> </li> <li> <p>In the European Union (EU), you have access to express shipping. Typically, devices shipped express are delivered in about a day. In addition, most countries in the EU have access to standard shipping, which typically takes less than a week, one way.</p> </li> <li> <p>In India, devices are delivered in one to seven days.</p> </li> <li> <p>In the US, you have access to one-day shipping and two-day shipping.</p> </li> </ul></p>
    #[serde(rename = "ShippingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option: Option<String>,
    /// <p>The type of AWS Snowball device to use for this cluster. The only supported device types for cluster jobs are <code>EDGE</code>, <code>EDGE_C</code>, and <code>EDGE_CG</code>.</p>
    #[serde(rename = "SnowballType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_type: Option<String>,
}

/// <p>A JSON-formatted object that describes a compatible Amazon Machine Image (AMI). For more information on compatible AMIs, see <a href="http://docs.aws.amazon.com/snowball/latest/developer-guide/using-ec2.html">Using Amazon EC2 Compute Instances</a> in the <i>AWS Snowball Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CompatibleImage {
    /// <p>The unique identifier for an individual Snowball Edge AMI.</p>
    #[serde(rename = "AmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    /// <p>The optional name of a compatible image.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAddressRequest {
    /// <p>The address that you want the Snowball shipped to.</p>
    #[serde(rename = "Address")]
    pub address: Address,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAddressResult {
    /// <p>The automatically generated ID for a specific address. You'll use this ID when you create a job to specify which address you want the Snowball for that job shipped to.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateClusterRequest {
    /// <p>The ID for the address that you want the cluster shipped to.</p>
    #[serde(rename = "AddressId")]
    pub address_id: String,
    /// <p>An optional description of this specific cluster, for example <code>Environmental Data Cluster-01</code>.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The forwarding address ID for a cluster. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>The type of job for this cluster. Currently, the only job type supported for clusters is <code>LOCAL_USE</code>.</p>
    #[serde(rename = "JobType")]
    pub job_type: String,
    /// <p>The <code>KmsKeyARN</code> value that you want to associate with this cluster. <code>KmsKeyARN</code> values are created by using the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> API action in AWS Key Management Service (AWS KMS). </p>
    #[serde(rename = "KmsKeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The Amazon Simple Notification Service (Amazon SNS) notification settings for this cluster.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>The resources associated with the cluster job. These resources include Amazon S3 buckets and optional AWS Lambda functions written in the Python language. </p>
    #[serde(rename = "Resources")]
    pub resources: JobResource,
    /// <p>The <code>RoleARN</code> that you want to associate with this cluster. <code>RoleArn</code> values are created by using the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> API action in AWS Identity and Access Management (IAM).</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p><p>The shipping speed for each node in this cluster. This speed doesn&#39;t dictate how soon you&#39;ll get each Snowball Edge device, rather it represents how quickly each device moves to its destination while in transit. Regional shipping speeds are as follows:</p> <ul> <li> <p>In Australia, you have access to express shipping. Typically, devices shipped express are delivered in about a day.</p> </li> <li> <p>In the European Union (EU), you have access to express shipping. Typically, Snowball Edges shipped express are delivered in about a day. In addition, most countries in the EU have access to standard shipping, which typically takes less than a week, one way.</p> </li> <li> <p>In India, devices are delivered in one to seven days.</p> </li> <li> <p>In the US, you have access to one-day shipping and two-day shipping.</p> </li> </ul></p>
    #[serde(rename = "ShippingOption")]
    pub shipping_option: String,
    /// <p>The type of AWS Snowball device to use for this cluster. The only supported device types for cluster jobs are <code>EDGE</code>, <code>EDGE_C</code>, and <code>EDGE_CG</code>.</p>
    #[serde(rename = "SnowballType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateClusterResult {
    /// <p>The automatically generated ID for a cluster.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateJobRequest {
    /// <p>The ID for the address that you want the Snowball shipped to.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The ID of a cluster. If you're creating a job for a node in a cluster, you need to provide only this <code>clusterId</code> value. The other job attributes are inherited from the cluster.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>Defines an optional description of this specific job, for example <code>Important Photos 2016-08-11</code>.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The forwarding address ID for a job. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>Defines the type of job that you're creating. </p>
    #[serde(rename = "JobType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    /// <p>The <code>KmsKeyARN</code> that you want to associate with this job. <code>KmsKeyARN</code>s are created using the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> AWS Key Management Service (KMS) API action.</p>
    #[serde(rename = "KmsKeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>Defines the Amazon Simple Notification Service (Amazon SNS) notification settings for this job.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>Defines the Amazon S3 buckets associated with this job.</p> <p>With <code>IMPORT</code> jobs, you specify the bucket or buckets that your transferred data will be imported into.</p> <p>With <code>EXPORT</code> jobs, you specify the bucket or buckets that your transferred data will be exported from. Optionally, you can also specify a <code>KeyRange</code> value. If you choose to export a range, you define the length of the range by providing either an inclusive <code>BeginMarker</code> value, an inclusive <code>EndMarker</code> value, or both. Ranges are UTF-8 binary sorted.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResource>,
    /// <p>The <code>RoleARN</code> that you want to associate with this job. <code>RoleArn</code>s are created using the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> AWS Identity and Access Management (IAM) API action.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>The shipping speed for this job. This speed doesn&#39;t dictate how soon you&#39;ll get the Snowball, rather it represents how quickly the Snowball moves to its destination while in transit. Regional shipping speeds are as follows:</p> <ul> <li> <p>In Australia, you have access to express shipping. Typically, Snowballs shipped express are delivered in about a day.</p> </li> <li> <p>In the European Union (EU), you have access to express shipping. Typically, Snowballs shipped express are delivered in about a day. In addition, most countries in the EU have access to standard shipping, which typically takes less than a week, one way.</p> </li> <li> <p>In India, Snowballs are delivered in one to seven days.</p> </li> <li> <p>In the US, you have access to one-day shipping and two-day shipping.</p> </li> </ul></p>
    #[serde(rename = "ShippingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option: Option<String>,
    /// <p>If your job is being created in one of the US regions, you have the option of specifying what size Snowball you'd like for this job. In all other regions, Snowballs come with 80 TB in storage capacity.</p>
    #[serde(rename = "SnowballCapacityPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_capacity_preference: Option<String>,
    /// <p>The type of AWS Snowball device to use for this job. The only supported device types for cluster jobs are <code>EDGE</code>, <code>EDGE_C</code>, and <code>EDGE_CG</code>.</p>
    #[serde(rename = "SnowballType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateJobResult {
    /// <p>The automatically generated ID for a job, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p>Defines the real-time status of a Snowball's data transfer while the device is at AWS. This data is only available while a job has a <code>JobState</code> value of <code>InProgress</code>, for both import and export jobs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataTransfer {
    /// <p>The number of bytes transferred between a Snowball and Amazon S3.</p>
    #[serde(rename = "BytesTransferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    /// <p>The number of objects transferred between a Snowball and Amazon S3.</p>
    #[serde(rename = "ObjectsTransferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects_transferred: Option<i64>,
    /// <p>The total bytes of data for a transfer between a Snowball and Amazon S3. This value is set to 0 (zero) until all the keys that will be transferred have been listed.</p>
    #[serde(rename = "TotalBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<i64>,
    /// <p>The total number of objects for a transfer between a Snowball and Amazon S3. This value is set to 0 (zero) until all the keys that will be transferred have been listed.</p>
    #[serde(rename = "TotalObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_objects: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAddressRequest {
    /// <p>The automatically generated ID for a specific address.</p>
    #[serde(rename = "AddressId")]
    pub address_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAddressResult {
    /// <p>The address that you want the Snowball or Snowballs associated with a specific job to be shipped to.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAddressesRequest {
    /// <p>The number of <code>ADDRESS</code> objects to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>HTTP requests are stateless. To identify what object comes "next" in the list of <code>ADDRESS</code> objects, you have the option of specifying a value for <code>NextToken</code> as the starting point for your list of returned addresses.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAddressesResult {
    /// <p>The Snowball shipping addresses that were created for this account.</p>
    #[serde(rename = "Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    /// <p>HTTP requests are stateless. If you use the automatically generated <code>NextToken</code> value in your next <code>DescribeAddresses</code> call, your list of returned addresses will start from this point in the array.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeClusterRequest {
    /// <p>The automatically generated ID for a cluster.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeClusterResult {
    /// <p>Information about a specific cluster, including shipping information, cluster status, and other important metadata.</p>
    #[serde(rename = "ClusterMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_metadata: Option<ClusterMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeJobRequest {
    /// <p>The automatically generated ID for a job, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeJobResult {
    /// <p>Information about a specific job, including shipping information, job status, and other important metadata.</p>
    #[serde(rename = "JobMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_metadata: Option<JobMetadata>,
    /// <p>Information about a specific job part (in the case of an export job), including shipping information, job status, and other important metadata.</p>
    #[serde(rename = "SubJobMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_job_metadata: Option<Vec<JobMetadata>>,
}

/// <p>A JSON-formatted object that contains the IDs for an Amazon Machine Image (AMI), including the Amazon EC2 AMI ID and the Snowball Edge AMI ID. Each AMI has these two IDs to simplify identifying the AMI in both the AWS Cloud and on the device.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ec2AmiResource {
    /// <p>The ID of the AMI in Amazon EC2.</p>
    #[serde(rename = "AmiId")]
    pub ami_id: String,
    /// <p>The ID of the AMI on the supported device.</p>
    #[serde(rename = "SnowballAmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_ami_id: Option<String>,
}

/// <p>The container for the <a>EventTriggerDefinition$EventResourceARN</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventTriggerDefinition {
    /// <p>The Amazon Resource Name (ARN) for any local Amazon S3 resource that is an AWS Lambda function's event trigger associated with this job.</p>
    #[serde(rename = "EventResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_resource_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobManifestRequest {
    /// <p>The ID for a job that you want to get the manifest file for, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobManifestResult {
    /// <p>The Amazon S3 presigned URL for the manifest file associated with the specified <code>JobId</code> value.</p>
    #[serde(rename = "ManifestURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_uri: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobUnlockCodeRequest {
    /// <p>The ID for the job that you want to get the <code>UnlockCode</code> value for, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobUnlockCodeResult {
    /// <p>The <code>UnlockCode</code> value for the specified job. The <code>UnlockCode</code> value can be accessed for up to 90 days after the job has been created.</p>
    #[serde(rename = "UnlockCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSnowballUsageRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSnowballUsageResult {
    /// <p>The service limit for number of Snowballs this account can have at once. The default service limit is 1 (one).</p>
    #[serde(rename = "SnowballLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_limit: Option<i64>,
    /// <p>The number of Snowballs that this account is currently using.</p>
    #[serde(rename = "SnowballsInUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowballs_in_use: Option<i64>,
}

/// <p>Each <code>JobListEntry</code> object contains a job's state, a job's ID, and a value that indicates whether the job is a job part, in the case of an export job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobListEntry {
    /// <p>The creation date for this job.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The optional description of this specific job, for example <code>Important Photos 2016-08-11</code>.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A value that indicates that this job is a master job. A master job represents a successful request to create an export job. Master jobs aren't associated with any Snowballs. Instead, each master job will have at least one job part, and each job part is associated with a Snowball. It might take some time before the job parts associated with a particular master job are listed, because they are created after the master job is created.</p>
    #[serde(rename = "IsMaster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_master: Option<bool>,
    /// <p>The automatically generated ID for a job, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The current state of this job.</p>
    #[serde(rename = "JobState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_state: Option<String>,
    /// <p>The type of job.</p>
    #[serde(rename = "JobType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    /// <p>The type of device used with this job.</p>
    #[serde(rename = "SnowballType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_type: Option<String>,
}

/// <p>Contains job logs. Whenever Snowball is used to import data into or export data out of Amazon S3, you'll have the option of downloading a PDF job report. Job logs are returned as a part of the response syntax of the <code>DescribeJob</code> action in the <code>JobMetadata</code> data type. The job logs can be accessed for up to 60 minutes after this request has been made. To access any of the job logs after 60 minutes have passed, you'll have to make another call to the <code>DescribeJob</code> action.</p> <p>For import jobs, the PDF job report becomes available at the end of the import process. For export jobs, your job report typically becomes available while the Snowball for your job part is being delivered to you.</p> <p>The job report provides you insight into the state of your Amazon S3 data transfer. The report includes details about your job or job part for your records.</p> <p>For deeper visibility into the status of your transferred objects, you can look at the two associated logs: a success log and a failure log. The logs are saved in comma-separated value (CSV) format, and the name of each log includes the ID of the job or job part that the log describes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobLogs {
    /// <p>A link to an Amazon S3 presigned URL where the job completion report is located.</p>
    #[serde(rename = "JobCompletionReportURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_completion_report_uri: Option<String>,
    /// <p>A link to an Amazon S3 presigned URL where the job failure log is located.</p>
    #[serde(rename = "JobFailureLogURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_failure_log_uri: Option<String>,
    /// <p>A link to an Amazon S3 presigned URL where the job success log is located.</p>
    #[serde(rename = "JobSuccessLogURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_success_log_uri: Option<String>,
}

/// <p>Contains information about a specific job including shipping information, job status, and other important metadata. This information is returned as a part of the response syntax of the <code>DescribeJob</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobMetadata {
    /// <p>The ID for the address that you want the Snowball shipped to.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The 39-character ID for the cluster, for example <code>CID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The creation date for this job.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A value that defines the real-time status of a Snowball's data transfer while the device is at AWS. This data is only available while a job has a <code>JobState</code> value of <code>InProgress</code>, for both import and export jobs.</p>
    #[serde(rename = "DataTransferProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_progress: Option<DataTransfer>,
    /// <p>The description of the job, provided at job creation.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the address that you want a job shipped to, after it will be shipped to its primary address. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>The automatically generated ID for a job, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>Links to Amazon S3 presigned URLs for the job report and logs. For import jobs, the PDF job report becomes available at the end of the import process. For export jobs, your job report typically becomes available while the Snowball for your job part is being delivered to you.</p>
    #[serde(rename = "JobLogInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_log_info: Option<JobLogs>,
    /// <p>The current status of the jobs.</p>
    #[serde(rename = "JobState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_state: Option<String>,
    /// <p>The type of job.</p>
    #[serde(rename = "JobType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the AWS Key Management Service (AWS KMS) key associated with this job. This ARN was created using the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> API action in AWS KMS.</p>
    #[serde(rename = "KmsKeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The Amazon Simple Notification Service (Amazon SNS) notification settings associated with a specific job. The <code>Notification</code> object is returned as a part of the response syntax of the <code>DescribeJob</code> action in the <code>JobMetadata</code> data type.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>An array of <code>S3Resource</code> objects. Each <code>S3Resource</code> object represents an Amazon S3 bucket that your transferred data will be exported from or imported into.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResource>,
    /// <p>The role ARN associated with this job. This ARN was created using the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> API action in AWS Identity and Access Management (IAM).</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A job's shipping information, including inbound and outbound tracking numbers and shipping speed options.</p>
    #[serde(rename = "ShippingDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<ShippingDetails>,
    /// <p>The Snowball capacity preference for this job, specified at job creation. In US regions, you can choose between 50 TB and 80 TB Snowballs. All other regions use 80 TB capacity Snowballs.</p>
    #[serde(rename = "SnowballCapacityPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_capacity_preference: Option<String>,
    /// <p>The type of device used with this job.</p>
    #[serde(rename = "SnowballType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_type: Option<String>,
}

/// <p>Contains an array of AWS resource objects. Each object represents an Amazon S3 bucket, an AWS Lambda function, or an Amazon Machine Image (AMI) based on Amazon EC2 that is associated with a particular job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobResource {
    /// <p>The Amazon Machine Images (AMIs) associated with this job.</p>
    #[serde(rename = "Ec2AmiResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_ami_resources: Option<Vec<Ec2AmiResource>>,
    /// <p>The Python-language Lambda functions for this job.</p>
    #[serde(rename = "LambdaResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_resources: Option<Vec<LambdaResource>>,
    /// <p>An array of <code>S3Resource</code> objects.</p>
    #[serde(rename = "S3Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_resources: Option<Vec<S3Resource>>,
}

/// <p>Contains a key range. For export jobs, a <code>S3Resource</code> object can have an optional <code>KeyRange</code> value. The length of the range is defined at job creation, and has either an inclusive <code>BeginMarker</code>, an inclusive <code>EndMarker</code>, or both. Ranges are UTF-8 binary sorted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyRange {
    /// <p>The key that starts an optional key range for an export job. Ranges are inclusive and UTF-8 binary sorted.</p>
    #[serde(rename = "BeginMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_marker: Option<String>,
    /// <p>The key that ends an optional key range for an export job. Ranges are inclusive and UTF-8 binary sorted.</p>
    #[serde(rename = "EndMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_marker: Option<String>,
}

/// <p>Identifies </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LambdaResource {
    /// <p>The array of ARNs for <a>S3Resource</a> objects to trigger the <a>LambdaResource</a> objects associated with this job.</p>
    #[serde(rename = "EventTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_triggers: Option<Vec<EventTriggerDefinition>>,
    /// <p>An Amazon Resource Name (ARN) that represents an AWS Lambda function to be triggered by PUT object actions on the associated local Amazon S3 resource.</p>
    #[serde(rename = "LambdaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListClusterJobsRequest {
    /// <p>The 39-character ID for the cluster that you want to list, for example <code>CID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The number of <code>JobListEntry</code> objects to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>HTTP requests are stateless. To identify what object comes "next" in the list of <code>JobListEntry</code> objects, you have the option of specifying <code>NextToken</code> as the starting point for your returned list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListClusterJobsResult {
    /// <p>Each <code>JobListEntry</code> object contains a job's state, a job's ID, and a value that indicates whether the job is a job part, in the case of export jobs. </p>
    #[serde(rename = "JobListEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_list_entries: Option<Vec<JobListEntry>>,
    /// <p>HTTP requests are stateless. If you use the automatically generated <code>NextToken</code> value in your next <code>ListClusterJobsResult</code> call, your list of returned jobs will start from this point in the array.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListClustersRequest {
    /// <p>The number of <code>ClusterListEntry</code> objects to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>HTTP requests are stateless. To identify what object comes "next" in the list of <code>ClusterListEntry</code> objects, you have the option of specifying <code>NextToken</code> as the starting point for your returned list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListClustersResult {
    /// <p>Each <code>ClusterListEntry</code> object contains a cluster's state, a cluster's ID, and other important status information.</p>
    #[serde(rename = "ClusterListEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_list_entries: Option<Vec<ClusterListEntry>>,
    /// <p>HTTP requests are stateless. If you use the automatically generated <code>NextToken</code> value in your next <code>ClusterListEntry</code> call, your list of returned clusters will start from this point in the array.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCompatibleImagesRequest {
    /// <p>The maximum number of results for the list of compatible images. Currently, each supported device can store 10 AMIs.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>HTTP requests are stateless. To identify what object comes "next" in the list of compatible images, you can specify a value for <code>NextToken</code> as the starting point for your list of returned images.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCompatibleImagesResult {
    /// <p>A JSON-formatted object that describes a compatible AMI.</p>
    #[serde(rename = "CompatibleImages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_images: Option<Vec<CompatibleImage>>,
    /// <p>Because HTTP requests are stateless, this is the starting point for your next list of returned images.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListJobsRequest {
    /// <p>The number of <code>JobListEntry</code> objects to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>HTTP requests are stateless. To identify what object comes "next" in the list of <code>JobListEntry</code> objects, you have the option of specifying <code>NextToken</code> as the starting point for your returned list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJobsResult {
    /// <p>Each <code>JobListEntry</code> object contains a job's state, a job's ID, and a value that indicates whether the job is a job part, in the case of export jobs. </p>
    #[serde(rename = "JobListEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_list_entries: Option<Vec<JobListEntry>>,
    /// <p>HTTP requests are stateless. If you use this automatically generated <code>NextToken</code> value in your next <code>ListJobs</code> call, your returned <code>JobListEntry</code> objects will start from this point in the array.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The Amazon Simple Notification Service (Amazon SNS) notification settings associated with a specific job. The <code>Notification</code> object is returned as a part of the response syntax of the <code>DescribeJob</code> action in the <code>JobMetadata</code> data type.</p> <p>When the notification settings are defined during job creation, you can choose to notify based on a specific set of job states using the <code>JobStatesToNotify</code> array of strings, or you can specify that you want to have Amazon SNS notifications sent out for all job states with <code>NotifyAll</code> set to true.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    /// <p>The list of job states that will trigger a notification for this job.</p>
    #[serde(rename = "JobStatesToNotify")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_states_to_notify: Option<Vec<String>>,
    /// <p>Any change in job state will trigger a notification for this job.</p>
    #[serde(rename = "NotifyAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_all: Option<bool>,
    /// <p>The new SNS <code>TopicArn</code> that you want to associate with this job. You can create Amazon Resource Names (ARNs) for topics by using the <a href="http://docs.aws.amazon.com/sns/latest/api/API_CreateTopic.html">CreateTopic</a> Amazon SNS API action.</p> <p>You can subscribe email addresses to an Amazon SNS topic through the AWS Management Console, or by using the <a href="http://docs.aws.amazon.com/sns/latest/api/API_Subscribe.html">Subscribe</a> AWS Simple Notification Service (SNS) API action.</p>
    #[serde(rename = "SnsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

/// <p>Each <code>S3Resource</code> object represents an Amazon S3 bucket that your transferred data will be exported from or imported into. For export jobs, this object can have an optional <code>KeyRange</code> value. The length of the range is defined at job creation, and has either an inclusive <code>BeginMarker</code>, an inclusive <code>EndMarker</code>, or both. Ranges are UTF-8 binary sorted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Resource {
    /// <p>The Amazon Resource Name (ARN) of an Amazon S3 bucket.</p>
    #[serde(rename = "BucketArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn: Option<String>,
    /// <p>For export jobs, you can provide an optional <code>KeyRange</code> within a specific Amazon S3 bucket. The length of the range is defined at job creation, and has either an inclusive <code>BeginMarker</code>, an inclusive <code>EndMarker</code>, or both. Ranges are UTF-8 binary sorted.</p>
    #[serde(rename = "KeyRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_range: Option<KeyRange>,
}

/// <p>The <code>Status</code> and <code>TrackingNumber</code> information for an inbound or outbound shipment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Shipment {
    /// <p>Status information for a shipment.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The tracking number for this job. Using this tracking number with your region's carrier's website, you can track a Snowball as the carrier transports it.</p> <p>For India, the carrier is Amazon Logistics. For all other regions, UPS is the carrier.</p>
    #[serde(rename = "TrackingNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

/// <p>A job's shipping information, including inbound and outbound tracking numbers and shipping speed options.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ShippingDetails {
    /// <p>The <code>Status</code> and <code>TrackingNumber</code> values for a Snowball being returned to AWS for a particular job.</p>
    #[serde(rename = "InboundShipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_shipment: Option<Shipment>,
    /// <p>The <code>Status</code> and <code>TrackingNumber</code> values for a Snowball being delivered to the address that you specified for a particular job.</p>
    #[serde(rename = "OutboundShipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_shipment: Option<Shipment>,
    /// <p><p>The shipping speed for a particular job. This speed doesn&#39;t dictate how soon you&#39;ll get the Snowball from the job&#39;s creation date. This speed represents how quickly it moves to its destination while in transit. Regional shipping speeds are as follows:</p> <ul> <li> <p>In Australia, you have access to express shipping. Typically, Snowballs shipped express are delivered in about a day.</p> </li> <li> <p>In the European Union (EU), you have access to express shipping. Typically, Snowballs shipped express are delivered in about a day. In addition, most countries in the EU have access to standard shipping, which typically takes less than a week, one way.</p> </li> <li> <p>In India, Snowballs are delivered in one to seven days.</p> </li> <li> <p>In the United States of America (US), you have access to one-day shipping and two-day shipping.</p> </li> </ul></p>
    #[serde(rename = "ShippingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateClusterRequest {
    /// <p>The ID of the updated <a>Address</a> object.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The cluster ID of the cluster that you want to update, for example <code>CID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The updated description of this cluster.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated ID for the forwarding address for a cluster. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>The new or updated <a>Notification</a> object.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>The updated arrays of <a>JobResource</a> objects that can include updated <a>S3Resource</a> objects or <a>LambdaResource</a> objects.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResource>,
    /// <p>The new role Amazon Resource Name (ARN) that you want to associate with this cluster. To create a role ARN, use the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> API action in AWS Identity and Access Management (IAM).</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The updated shipping option value of this cluster's <a>ShippingDetails</a> object.</p>
    #[serde(rename = "ShippingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateClusterResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateJobRequest {
    /// <p>The ID of the updated <a>Address</a> object.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The updated description of this job's <a>JobMetadata</a> object.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated ID for the forwarding address for a job. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>The job ID of the job that you want to update, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>The new or updated <a>Notification</a> object.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>The updated <code>JobResource</code> object, or the updated <a>JobResource</a> object. </p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResource>,
    /// <p>The new role Amazon Resource Name (ARN) that you want to associate with this job. To create a role ARN, use the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a>AWS Identity and Access Management (IAM) API action.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The updated shipping option value of this job's <a>ShippingDetails</a> object.</p>
    #[serde(rename = "ShippingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option: Option<String>,
    /// <p>The updated <code>SnowballCapacityPreference</code> of this job's <a>JobMetadata</a> object. The 50 TB Snowballs are only available in the US regions.</p>
    #[serde(rename = "SnowballCapacityPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_capacity_preference: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateJobResult {}

/// Errors returned by CancelCluster
#[derive(Debug, PartialEq)]
pub enum CancelClusterError {
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
}

impl CancelClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidJobStateException" => {
                    return RusotoError::Service(CancelClusterError::InvalidJobState(err.msg))
                }
                "InvalidResourceException" => {
                    return RusotoError::Service(CancelClusterError::InvalidResource(err.msg))
                }
                "KMSRequestFailedException" => {
                    return RusotoError::Service(CancelClusterError::KMSRequestFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CancelClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelClusterError {
    fn description(&self) -> &str {
        match *self {
            CancelClusterError::InvalidJobState(ref cause) => cause,
            CancelClusterError::InvalidResource(ref cause) => cause,
            CancelClusterError::KMSRequestFailed(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
}

impl CancelJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidJobStateException" => {
                    return RusotoError::Service(CancelJobError::InvalidJobState(err.msg))
                }
                "InvalidResourceException" => {
                    return RusotoError::Service(CancelJobError::InvalidResource(err.msg))
                }
                "KMSRequestFailedException" => {
                    return RusotoError::Service(CancelJobError::KMSRequestFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CancelJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelJobError {
    fn description(&self) -> &str {
        match *self {
            CancelJobError::InvalidJobState(ref cause) => cause,
            CancelJobError::InvalidResource(ref cause) => cause,
            CancelJobError::KMSRequestFailed(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAddress
#[derive(Debug, PartialEq)]
pub enum CreateAddressError {
    /// <p>The address provided was invalid. Check the address with your region's carrier, and try again.</p>
    InvalidAddress(String),
    /// <p>The address is either outside the serviceable area for your region, or an error occurred. Check the address with your region's carrier and try again. If the issue persists, contact AWS Support.</p>
    UnsupportedAddress(String),
}

impl CreateAddressError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAddressError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidAddressException" => {
                    return RusotoError::Service(CreateAddressError::InvalidAddress(err.msg))
                }
                "UnsupportedAddressException" => {
                    return RusotoError::Service(CreateAddressError::UnsupportedAddress(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAddressError {
    fn description(&self) -> &str {
        match *self {
            CreateAddressError::InvalidAddress(ref cause) => cause,
            CreateAddressError::UnsupportedAddress(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>Your IAM user lacks the necessary Amazon EC2 permissions to perform the attempted action.</p>
    Ec2RequestFailed(String),
    /// <p>Job or cluster creation failed. One ore more inputs were invalid. Confirm that the <a>CreateClusterRequest$SnowballType</a> value supports your <a>CreateJobRequest$JobType</a>, and try again.</p>
    InvalidInputCombination(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
}

impl CreateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "Ec2RequestFailedException" => {
                    return RusotoError::Service(CreateClusterError::Ec2RequestFailed(err.msg))
                }
                "InvalidInputCombinationException" => {
                    return RusotoError::Service(CreateClusterError::InvalidInputCombination(
                        err.msg,
                    ))
                }
                "InvalidResourceException" => {
                    return RusotoError::Service(CreateClusterError::InvalidResource(err.msg))
                }
                "KMSRequestFailedException" => {
                    return RusotoError::Service(CreateClusterError::KMSRequestFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterError::Ec2RequestFailed(ref cause) => cause,
            CreateClusterError::InvalidInputCombination(ref cause) => cause,
            CreateClusterError::InvalidResource(ref cause) => cause,
            CreateClusterError::KMSRequestFailed(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    /// <p>Job creation failed. Currently, clusters support five nodes. If you have less than five nodes for your cluster and you have more nodes to create for this cluster, try again and create jobs until your cluster has exactly five notes.</p>
    ClusterLimitExceeded(String),
    /// <p>Your IAM user lacks the necessary Amazon EC2 permissions to perform the attempted action.</p>
    Ec2RequestFailed(String),
    /// <p>Job or cluster creation failed. One ore more inputs were invalid. Confirm that the <a>CreateClusterRequest$SnowballType</a> value supports your <a>CreateJobRequest$JobType</a>, and try again.</p>
    InvalidInputCombination(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
}

impl CreateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterLimitExceededException" => {
                    return RusotoError::Service(CreateJobError::ClusterLimitExceeded(err.msg))
                }
                "Ec2RequestFailedException" => {
                    return RusotoError::Service(CreateJobError::Ec2RequestFailed(err.msg))
                }
                "InvalidInputCombinationException" => {
                    return RusotoError::Service(CreateJobError::InvalidInputCombination(err.msg))
                }
                "InvalidResourceException" => {
                    return RusotoError::Service(CreateJobError::InvalidResource(err.msg))
                }
                "KMSRequestFailedException" => {
                    return RusotoError::Service(CreateJobError::KMSRequestFailed(err.msg))
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
            CreateJobError::ClusterLimitExceeded(ref cause) => cause,
            CreateJobError::Ec2RequestFailed(ref cause) => cause,
            CreateJobError::InvalidInputCombination(ref cause) => cause,
            CreateJobError::InvalidResource(ref cause) => cause,
            CreateJobError::KMSRequestFailed(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAddress
#[derive(Debug, PartialEq)]
pub enum DescribeAddressError {
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
}

impl DescribeAddressError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAddressError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceException" => {
                    return RusotoError::Service(DescribeAddressError::InvalidResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAddressError {
    fn description(&self) -> &str {
        match *self {
            DescribeAddressError::InvalidResource(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAddresses
#[derive(Debug, PartialEq)]
pub enum DescribeAddressesError {
    /// <p>The <code>NextToken</code> string was altered unexpectedly, and the operation has stopped. Run the operation without changing the <code>NextToken</code> string, and try again.</p>
    InvalidNextToken(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
}

impl DescribeAddressesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAddressesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeAddressesError::InvalidNextToken(err.msg))
                }
                "InvalidResourceException" => {
                    return RusotoError::Service(DescribeAddressesError::InvalidResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAddressesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAddressesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAddressesError::InvalidNextToken(ref cause) => cause,
            DescribeAddressesError::InvalidResource(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCluster
#[derive(Debug, PartialEq)]
pub enum DescribeClusterError {
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
}

impl DescribeClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceException" => {
                    return RusotoError::Service(DescribeClusterError::InvalidResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClusterError {
    fn description(&self) -> &str {
        match *self {
            DescribeClusterError::InvalidResource(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeJob
#[derive(Debug, PartialEq)]
pub enum DescribeJobError {
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
}

impl DescribeJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceException" => {
                    return RusotoError::Service(DescribeJobError::InvalidResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobError::InvalidResource(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobManifest
#[derive(Debug, PartialEq)]
pub enum GetJobManifestError {
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
}

impl GetJobManifestError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobManifestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidJobStateException" => {
                    return RusotoError::Service(GetJobManifestError::InvalidJobState(err.msg))
                }
                "InvalidResourceException" => {
                    return RusotoError::Service(GetJobManifestError::InvalidResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobManifestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobManifestError {
    fn description(&self) -> &str {
        match *self {
            GetJobManifestError::InvalidJobState(ref cause) => cause,
            GetJobManifestError::InvalidResource(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobUnlockCode
#[derive(Debug, PartialEq)]
pub enum GetJobUnlockCodeError {
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
}

impl GetJobUnlockCodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobUnlockCodeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidJobStateException" => {
                    return RusotoError::Service(GetJobUnlockCodeError::InvalidJobState(err.msg))
                }
                "InvalidResourceException" => {
                    return RusotoError::Service(GetJobUnlockCodeError::InvalidResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobUnlockCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobUnlockCodeError {
    fn description(&self) -> &str {
        match *self {
            GetJobUnlockCodeError::InvalidJobState(ref cause) => cause,
            GetJobUnlockCodeError::InvalidResource(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSnowballUsage
#[derive(Debug, PartialEq)]
pub enum GetSnowballUsageError {}

impl GetSnowballUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSnowballUsageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSnowballUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSnowballUsageError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListClusterJobs
#[derive(Debug, PartialEq)]
pub enum ListClusterJobsError {
    /// <p>The <code>NextToken</code> string was altered unexpectedly, and the operation has stopped. Run the operation without changing the <code>NextToken</code> string, and try again.</p>
    InvalidNextToken(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
}

impl ListClusterJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListClusterJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListClusterJobsError::InvalidNextToken(err.msg))
                }
                "InvalidResourceException" => {
                    return RusotoError::Service(ListClusterJobsError::InvalidResource(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListClusterJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListClusterJobsError {
    fn description(&self) -> &str {
        match *self {
            ListClusterJobsError::InvalidNextToken(ref cause) => cause,
            ListClusterJobsError::InvalidResource(ref cause) => cause,
        }
    }
}
/// Errors returned by ListClusters
#[derive(Debug, PartialEq)]
pub enum ListClustersError {
    /// <p>The <code>NextToken</code> string was altered unexpectedly, and the operation has stopped. Run the operation without changing the <code>NextToken</code> string, and try again.</p>
    InvalidNextToken(String),
}

impl ListClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListClustersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListClustersError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListClustersError {
    fn description(&self) -> &str {
        match *self {
            ListClustersError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCompatibleImages
#[derive(Debug, PartialEq)]
pub enum ListCompatibleImagesError {
    /// <p>Your IAM user lacks the necessary Amazon EC2 permissions to perform the attempted action.</p>
    Ec2RequestFailed(String),
    /// <p>The <code>NextToken</code> string was altered unexpectedly, and the operation has stopped. Run the operation without changing the <code>NextToken</code> string, and try again.</p>
    InvalidNextToken(String),
}

impl ListCompatibleImagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCompatibleImagesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "Ec2RequestFailedException" => {
                    return RusotoError::Service(ListCompatibleImagesError::Ec2RequestFailed(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListCompatibleImagesError::InvalidNextToken(
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
impl fmt::Display for ListCompatibleImagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCompatibleImagesError {
    fn description(&self) -> &str {
        match *self {
            ListCompatibleImagesError::Ec2RequestFailed(ref cause) => cause,
            ListCompatibleImagesError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>The <code>NextToken</code> string was altered unexpectedly, and the operation has stopped. Run the operation without changing the <code>NextToken</code> string, and try again.</p>
    InvalidNextToken(String),
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListJobsError::InvalidNextToken(err.msg))
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
            ListJobsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCluster
#[derive(Debug, PartialEq)]
pub enum UpdateClusterError {
    /// <p>Your IAM user lacks the necessary Amazon EC2 permissions to perform the attempted action.</p>
    Ec2RequestFailed(String),
    /// <p>Job or cluster creation failed. One ore more inputs were invalid. Confirm that the <a>CreateClusterRequest$SnowballType</a> value supports your <a>CreateJobRequest$JobType</a>, and try again.</p>
    InvalidInputCombination(String),
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
}

impl UpdateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "Ec2RequestFailedException" => {
                    return RusotoError::Service(UpdateClusterError::Ec2RequestFailed(err.msg))
                }
                "InvalidInputCombinationException" => {
                    return RusotoError::Service(UpdateClusterError::InvalidInputCombination(
                        err.msg,
                    ))
                }
                "InvalidJobStateException" => {
                    return RusotoError::Service(UpdateClusterError::InvalidJobState(err.msg))
                }
                "InvalidResourceException" => {
                    return RusotoError::Service(UpdateClusterError::InvalidResource(err.msg))
                }
                "KMSRequestFailedException" => {
                    return RusotoError::Service(UpdateClusterError::KMSRequestFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateClusterError {
    fn description(&self) -> &str {
        match *self {
            UpdateClusterError::Ec2RequestFailed(ref cause) => cause,
            UpdateClusterError::InvalidInputCombination(ref cause) => cause,
            UpdateClusterError::InvalidJobState(ref cause) => cause,
            UpdateClusterError::InvalidResource(ref cause) => cause,
            UpdateClusterError::KMSRequestFailed(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateJob
#[derive(Debug, PartialEq)]
pub enum UpdateJobError {
    /// <p>Job creation failed. Currently, clusters support five nodes. If you have less than five nodes for your cluster and you have more nodes to create for this cluster, try again and create jobs until your cluster has exactly five notes.</p>
    ClusterLimitExceeded(String),
    /// <p>Your IAM user lacks the necessary Amazon EC2 permissions to perform the attempted action.</p>
    Ec2RequestFailed(String),
    /// <p>Job or cluster creation failed. One ore more inputs were invalid. Confirm that the <a>CreateClusterRequest$SnowballType</a> value supports your <a>CreateJobRequest$JobType</a>, and try again.</p>
    InvalidInputCombination(String),
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
}

impl UpdateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterLimitExceededException" => {
                    return RusotoError::Service(UpdateJobError::ClusterLimitExceeded(err.msg))
                }
                "Ec2RequestFailedException" => {
                    return RusotoError::Service(UpdateJobError::Ec2RequestFailed(err.msg))
                }
                "InvalidInputCombinationException" => {
                    return RusotoError::Service(UpdateJobError::InvalidInputCombination(err.msg))
                }
                "InvalidJobStateException" => {
                    return RusotoError::Service(UpdateJobError::InvalidJobState(err.msg))
                }
                "InvalidResourceException" => {
                    return RusotoError::Service(UpdateJobError::InvalidResource(err.msg))
                }
                "KMSRequestFailedException" => {
                    return RusotoError::Service(UpdateJobError::KMSRequestFailed(err.msg))
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
            UpdateJobError::ClusterLimitExceeded(ref cause) => cause,
            UpdateJobError::Ec2RequestFailed(ref cause) => cause,
            UpdateJobError::InvalidInputCombination(ref cause) => cause,
            UpdateJobError::InvalidJobState(ref cause) => cause,
            UpdateJobError::InvalidResource(ref cause) => cause,
            UpdateJobError::KMSRequestFailed(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Snowball API. Amazon Snowball clients implement this trait.
pub trait Snowball {
    /// <p>Cancels a cluster job. You can only cancel a cluster job while it's in the <code>AwaitingQuorum</code> status. You'll have at least an hour after creating a cluster job to cancel it.</p>
    fn cancel_cluster(
        &self,
        input: CancelClusterRequest,
    ) -> RusotoFuture<CancelClusterResult, CancelClusterError>;

    /// <p>Cancels the specified job. You can only cancel a job before its <code>JobState</code> value changes to <code>PreparingAppliance</code>. Requesting the <code>ListJobs</code> or <code>DescribeJob</code> action returns a job's <code>JobState</code> as part of the response element data returned.</p>
    fn cancel_job(&self, input: CancelJobRequest) -> RusotoFuture<CancelJobResult, CancelJobError>;

    /// <p>Creates an address for a Snowball to be shipped to. In most regions, addresses are validated at the time of creation. The address you provide must be located within the serviceable area of your region. If the address is invalid or unsupported, then an exception is thrown.</p>
    fn create_address(
        &self,
        input: CreateAddressRequest,
    ) -> RusotoFuture<CreateAddressResult, CreateAddressError>;

    /// <p>Creates an empty cluster. Each cluster supports five nodes. You use the <a>CreateJob</a> action separately to create the jobs for each of these nodes. The cluster does not ship until these five node jobs have been created.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResult, CreateClusterError>;

    /// <p>Creates a job to import or export data between Amazon S3 and your on-premises data center. Your AWS account must have the right trust policies and permissions in place to create a job for Snowball. If you're creating a job for a node in a cluster, you only need to provide the <code>clusterId</code> value; the other job attributes are inherited from the cluster. </p>
    fn create_job(&self, input: CreateJobRequest) -> RusotoFuture<CreateJobResult, CreateJobError>;

    /// <p>Takes an <code>AddressId</code> and returns specific details about that address in the form of an <code>Address</code> object.</p>
    fn describe_address(
        &self,
        input: DescribeAddressRequest,
    ) -> RusotoFuture<DescribeAddressResult, DescribeAddressError>;

    /// <p>Returns a specified number of <code>ADDRESS</code> objects. Calling this API in one of the US regions will return addresses from the list of all addresses associated with this account in all US regions.</p>
    fn describe_addresses(
        &self,
        input: DescribeAddressesRequest,
    ) -> RusotoFuture<DescribeAddressesResult, DescribeAddressesError>;

    /// <p>Returns information about a specific cluster including shipping information, cluster status, and other important metadata.</p>
    fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> RusotoFuture<DescribeClusterResult, DescribeClusterError>;

    /// <p>Returns information about a specific job including shipping information, job status, and other important metadata. </p>
    fn describe_job(
        &self,
        input: DescribeJobRequest,
    ) -> RusotoFuture<DescribeJobResult, DescribeJobError>;

    /// <p>Returns a link to an Amazon S3 presigned URL for the manifest file associated with the specified <code>JobId</code> value. You can access the manifest file for up to 60 minutes after this request has been made. To access the manifest file after 60 minutes have passed, you'll have to make another call to the <code>GetJobManifest</code> action.</p> <p>The manifest is an encrypted file that you can download after your job enters the <code>WithCustomer</code> status. The manifest is decrypted by using the <code>UnlockCode</code> code value, when you pass both values to the Snowball through the Snowball client when the client is started for the first time.</p> <p>As a best practice, we recommend that you don't save a copy of an <code>UnlockCode</code> value in the same location as the manifest file for that job. Saving these separately helps prevent unauthorized parties from gaining access to the Snowball associated with that job.</p> <p>The credentials of a given job, including its manifest file and unlock code, expire 90 days after the job is created.</p>
    fn get_job_manifest(
        &self,
        input: GetJobManifestRequest,
    ) -> RusotoFuture<GetJobManifestResult, GetJobManifestError>;

    /// <p>Returns the <code>UnlockCode</code> code value for the specified job. A particular <code>UnlockCode</code> value can be accessed for up to 90 days after the associated job has been created.</p> <p>The <code>UnlockCode</code> value is a 29-character code with 25 alphanumeric characters and 4 hyphens. This code is used to decrypt the manifest file when it is passed along with the manifest to the Snowball through the Snowball client when the client is started for the first time.</p> <p>As a best practice, we recommend that you don't save a copy of the <code>UnlockCode</code> in the same location as the manifest file for that job. Saving these separately helps prevent unauthorized parties from gaining access to the Snowball associated with that job.</p>
    fn get_job_unlock_code(
        &self,
        input: GetJobUnlockCodeRequest,
    ) -> RusotoFuture<GetJobUnlockCodeResult, GetJobUnlockCodeError>;

    /// <p>Returns information about the Snowball service limit for your account, and also the number of Snowballs your account has in use.</p> <p>The default service limit for the number of Snowballs that you can have at one time is 1. If you want to increase your service limit, contact AWS Support.</p>
    fn get_snowball_usage(&self) -> RusotoFuture<GetSnowballUsageResult, GetSnowballUsageError>;

    /// <p>Returns an array of <code>JobListEntry</code> objects of the specified length. Each <code>JobListEntry</code> object is for a job in the specified cluster and contains a job's state, a job's ID, and other information.</p>
    fn list_cluster_jobs(
        &self,
        input: ListClusterJobsRequest,
    ) -> RusotoFuture<ListClusterJobsResult, ListClusterJobsError>;

    /// <p>Returns an array of <code>ClusterListEntry</code> objects of the specified length. Each <code>ClusterListEntry</code> object contains a cluster's state, a cluster's ID, and other important status information.</p>
    fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> RusotoFuture<ListClustersResult, ListClustersError>;

    /// <p>This action returns a list of the different Amazon EC2 Amazon Machine Images (AMIs) that are owned by your AWS account that would be supported for use on <code>EDGE</code>, <code>EDGE_C</code>, and <code>EDGE_CG</code> devices. For more information on compatible AMIs, see <a href="http://docs.aws.amazon.com/snowball/latest/developer-guide/using-ec2.html">Using Amazon EC2 Compute Instances</a> in the <i>AWS Snowball Developer Guide</i>.</p>
    fn list_compatible_images(
        &self,
        input: ListCompatibleImagesRequest,
    ) -> RusotoFuture<ListCompatibleImagesResult, ListCompatibleImagesError>;

    /// <p>Returns an array of <code>JobListEntry</code> objects of the specified length. Each <code>JobListEntry</code> object contains a job's state, a job's ID, and a value that indicates whether the job is a job part, in the case of export jobs. Calling this API action in one of the US regions will return jobs from the list of all jobs associated with this account in all US regions.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResult, ListJobsError>;

    /// <p>While a cluster's <code>ClusterState</code> value is in the <code>AwaitingQuorum</code> state, you can update some of the information associated with a cluster. Once the cluster changes to a different job state, usually 60 minutes after the cluster being created, this action is no longer available.</p>
    fn update_cluster(
        &self,
        input: UpdateClusterRequest,
    ) -> RusotoFuture<UpdateClusterResult, UpdateClusterError>;

    /// <p>While a job's <code>JobState</code> value is <code>New</code>, you can update some of the information associated with a job. Once the job changes to a different job state, usually within 60 minutes of the job being created, this action is no longer available.</p>
    fn update_job(&self, input: UpdateJobRequest) -> RusotoFuture<UpdateJobResult, UpdateJobError>;
}
/// A client for the Amazon Snowball API.
#[derive(Clone)]
pub struct SnowballClient {
    client: Client,
    region: region::Region,
}

impl SnowballClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SnowballClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SnowballClient
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

    pub fn new_with_client(client: Client, region: region::Region) -> SnowballClient {
        SnowballClient { client, region }
    }
}

impl fmt::Debug for SnowballClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SnowballClient")
            .field("region", &self.region)
            .finish()
    }
}

impl Snowball for SnowballClient {
    /// <p>Cancels a cluster job. You can only cancel a cluster job while it's in the <code>AwaitingQuorum</code> status. You'll have at least an hour after creating a cluster job to cancel it.</p>
    fn cancel_cluster(
        &self,
        input: CancelClusterRequest,
    ) -> RusotoFuture<CancelClusterResult, CancelClusterError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.CancelCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CancelClusterResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CancelClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Cancels the specified job. You can only cancel a job before its <code>JobState</code> value changes to <code>PreparingAppliance</code>. Requesting the <code>ListJobs</code> or <code>DescribeJob</code> action returns a job's <code>JobState</code> as part of the response element data returned.</p>
    fn cancel_job(&self, input: CancelJobRequest) -> RusotoFuture<CancelJobResult, CancelJobError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.CancelJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<CancelJobResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CancelJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an address for a Snowball to be shipped to. In most regions, addresses are validated at the time of creation. The address you provide must be located within the serviceable area of your region. If the address is invalid or unsupported, then an exception is thrown.</p>
    fn create_address(
        &self,
        input: CreateAddressRequest,
    ) -> RusotoFuture<CreateAddressResult, CreateAddressError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.CreateAddress",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAddressResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAddressError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an empty cluster. Each cluster supports five nodes. You use the <a>CreateJob</a> action separately to create the jobs for each of these nodes. The cluster does not ship until these five node jobs have been created.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResult, CreateClusterError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.CreateCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateClusterResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a job to import or export data between Amazon S3 and your on-premises data center. Your AWS account must have the right trust policies and permissions in place to create a job for Snowball. If you're creating a job for a node in a cluster, you only need to provide the <code>clusterId</code> value; the other job attributes are inherited from the cluster. </p>
    fn create_job(&self, input: CreateJobRequest) -> RusotoFuture<CreateJobResult, CreateJobError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.CreateJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<CreateJobResult, _>()
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

    /// <p>Takes an <code>AddressId</code> and returns specific details about that address in the form of an <code>Address</code> object.</p>
    fn describe_address(
        &self,
        input: DescribeAddressRequest,
    ) -> RusotoFuture<DescribeAddressResult, DescribeAddressError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.DescribeAddress",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAddressResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeAddressError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a specified number of <code>ADDRESS</code> objects. Calling this API in one of the US regions will return addresses from the list of all addresses associated with this account in all US regions.</p>
    fn describe_addresses(
        &self,
        input: DescribeAddressesRequest,
    ) -> RusotoFuture<DescribeAddressesResult, DescribeAddressesError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.DescribeAddresses",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAddressesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeAddressesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a specific cluster including shipping information, cluster status, and other important metadata.</p>
    fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> RusotoFuture<DescribeClusterResult, DescribeClusterError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.DescribeCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeClusterResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a specific job including shipping information, job status, and other important metadata. </p>
    fn describe_job(
        &self,
        input: DescribeJobRequest,
    ) -> RusotoFuture<DescribeJobResult, DescribeJobError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.DescribeJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeJobResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a link to an Amazon S3 presigned URL for the manifest file associated with the specified <code>JobId</code> value. You can access the manifest file for up to 60 minutes after this request has been made. To access the manifest file after 60 minutes have passed, you'll have to make another call to the <code>GetJobManifest</code> action.</p> <p>The manifest is an encrypted file that you can download after your job enters the <code>WithCustomer</code> status. The manifest is decrypted by using the <code>UnlockCode</code> code value, when you pass both values to the Snowball through the Snowball client when the client is started for the first time.</p> <p>As a best practice, we recommend that you don't save a copy of an <code>UnlockCode</code> value in the same location as the manifest file for that job. Saving these separately helps prevent unauthorized parties from gaining access to the Snowball associated with that job.</p> <p>The credentials of a given job, including its manifest file and unlock code, expire 90 days after the job is created.</p>
    fn get_job_manifest(
        &self,
        input: GetJobManifestRequest,
    ) -> RusotoFuture<GetJobManifestResult, GetJobManifestError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.GetJobManifest",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetJobManifestResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobManifestError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the <code>UnlockCode</code> code value for the specified job. A particular <code>UnlockCode</code> value can be accessed for up to 90 days after the associated job has been created.</p> <p>The <code>UnlockCode</code> value is a 29-character code with 25 alphanumeric characters and 4 hyphens. This code is used to decrypt the manifest file when it is passed along with the manifest to the Snowball through the Snowball client when the client is started for the first time.</p> <p>As a best practice, we recommend that you don't save a copy of the <code>UnlockCode</code> in the same location as the manifest file for that job. Saving these separately helps prevent unauthorized parties from gaining access to the Snowball associated with that job.</p>
    fn get_job_unlock_code(
        &self,
        input: GetJobUnlockCodeRequest,
    ) -> RusotoFuture<GetJobUnlockCodeResult, GetJobUnlockCodeError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.GetJobUnlockCode",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetJobUnlockCodeResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobUnlockCodeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the Snowball service limit for your account, and also the number of Snowballs your account has in use.</p> <p>The default service limit for the number of Snowballs that you can have at one time is 1. If you want to increase your service limit, contact AWS Support.</p>
    fn get_snowball_usage(&self) -> RusotoFuture<GetSnowballUsageResult, GetSnowballUsageError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.GetSnowballUsage",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSnowballUsageResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSnowballUsageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <code>JobListEntry</code> objects of the specified length. Each <code>JobListEntry</code> object is for a job in the specified cluster and contains a job's state, a job's ID, and other information.</p>
    fn list_cluster_jobs(
        &self,
        input: ListClusterJobsRequest,
    ) -> RusotoFuture<ListClusterJobsResult, ListClusterJobsError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.ListClusterJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListClusterJobsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListClusterJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <code>ClusterListEntry</code> objects of the specified length. Each <code>ClusterListEntry</code> object contains a cluster's state, a cluster's ID, and other important status information.</p>
    fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> RusotoFuture<ListClustersResult, ListClustersError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.ListClusters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListClustersResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListClustersError::from_response(response))),
                )
            }
        })
    }

    /// <p>This action returns a list of the different Amazon EC2 Amazon Machine Images (AMIs) that are owned by your AWS account that would be supported for use on <code>EDGE</code>, <code>EDGE_C</code>, and <code>EDGE_CG</code> devices. For more information on compatible AMIs, see <a href="http://docs.aws.amazon.com/snowball/latest/developer-guide/using-ec2.html">Using Amazon EC2 Compute Instances</a> in the <i>AWS Snowball Developer Guide</i>.</p>
    fn list_compatible_images(
        &self,
        input: ListCompatibleImagesRequest,
    ) -> RusotoFuture<ListCompatibleImagesResult, ListCompatibleImagesError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.ListCompatibleImages",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListCompatibleImagesResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListCompatibleImagesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns an array of <code>JobListEntry</code> objects of the specified length. Each <code>JobListEntry</code> object contains a job's state, a job's ID, and a value that indicates whether the job is a job part, in the case of export jobs. Calling this API action in one of the US regions will return jobs from the list of all jobs associated with this account in all US regions.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResult, ListJobsError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSIESnowballJobManagementService.ListJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<ListJobsResult, _>()
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

    /// <p>While a cluster's <code>ClusterState</code> value is in the <code>AwaitingQuorum</code> state, you can update some of the information associated with a cluster. Once the cluster changes to a different job state, usually 60 minutes after the cluster being created, this action is no longer available.</p>
    fn update_cluster(
        &self,
        input: UpdateClusterRequest,
    ) -> RusotoFuture<UpdateClusterResult, UpdateClusterError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.UpdateCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateClusterResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p>While a job's <code>JobState</code> value is <code>New</code>, you can update some of the information associated with a job. Once the job changes to a different job state, usually within 60 minutes of the job being created, this action is no longer available.</p>
    fn update_job(&self, input: UpdateJobRequest) -> RusotoFuture<UpdateJobResult, UpdateJobError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.UpdateJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<UpdateJobResult, _>()
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
}
