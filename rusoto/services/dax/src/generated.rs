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
/// <p>Contains all of the attributes of a specific DAX cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Cluster {
    /// <p>The number of nodes in the cluster that are active (i.e., capable of serving requests).</p>
    #[serde(rename = "ActiveNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_nodes: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster. </p>
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The configuration endpoint for this DAX cluster, consisting of a DNS name and a port number. Client applications can specify this endpoint, rather than an individual node endpoint, and allow the DAX client software to intelligently route requests and responses to nodes in the DAX cluster.</p>
    #[serde(rename = "ClusterDiscoveryEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_discovery_endpoint: Option<Endpoint>,
    /// <p>The name of the DAX cluster.</p>
    #[serde(rename = "ClusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The description of the cluster.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A valid Amazon Resource Name (ARN) that identifies an IAM role. At runtime, DAX will assume this role and use the role's permissions to access DynamoDB on your behalf.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>A list of nodes to be removed from the cluster.</p>
    #[serde(rename = "NodeIdsToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids_to_remove: Option<Vec<String>>,
    /// <p>The node type for the nodes in the cluster. (All nodes in a DAX cluster are of the same type.)</p>
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <p>A list of nodes that are currently in the cluster.</p>
    #[serde(rename = "Nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
    /// <p>Describes a notification topic and its status. Notification topics are used for publishing DAX events to subscribers using Amazon Simple Notification Service (SNS).</p>
    #[serde(rename = "NotificationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
    /// <p>The parameter group being used by nodes in the cluster.</p>
    #[serde(rename = "ParameterGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group: Option<ParameterGroupStatus>,
    /// <p>A range of time when maintenance of DAX cluster software will be performed. For example: <code>sun:01:00-sun:09:00</code>. Cluster maintenance normally takes less than 30 minutes, and is performed automatically within the maintenance window.</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>The description of the server-side encryption status on the specified DAX cluster.</p>
    #[serde(rename = "SSEDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_description: Option<SSEDescription>,
    /// <p>A list of security groups, and the status of each, for the nodes in the cluster.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<SecurityGroupMembership>>,
    /// <p>The current status of the cluster.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The subnet group where the DAX cluster is running.</p>
    #[serde(rename = "SubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group: Option<String>,
    /// <p>The total number of nodes in the cluster.</p>
    #[serde(rename = "TotalNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_nodes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateClusterRequest {
    /// <p>The Availability Zones (AZs) in which the cluster nodes will be created. All nodes belonging to the cluster are placed in these Availability Zones. Use this parameter if you want to distribute the nodes across multiple AZs.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p><p>The cluster identifier. This parameter is stored as a lowercase string.</p> <p> <b>Constraints:</b> </p> <ul> <li> <p>A name must contain from 1 to 20 alphanumeric characters or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>A name cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// <p>A description of the cluster.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A valid Amazon Resource Name (ARN) that identifies an IAM role. At runtime, DAX will assume this role and use the role's permissions to access DynamoDB on your behalf.</p>
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,
    /// <p>The compute and memory capacity of the nodes in the cluster.</p>
    #[serde(rename = "NodeType")]
    pub node_type: String,
    /// <p><p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which notifications will be sent.</p> <note> <p>The Amazon SNS topic owner must be same as the DAX cluster owner.</p> </note></p>
    #[serde(rename = "NotificationTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<String>,
    /// <p>The parameter group to be associated with the DAX cluster.</p>
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    /// <p><p>Specifies the weekly time range during which maintenance on the DAX cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period. Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:05:00-sun:09:00</code> </p> <note> <p>If you don&#39;t specify a preferred maintenance window when you create or modify a cache cluster, DAX assigns a 60-minute maintenance window on a randomly selected day of the week.</p> </note></p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p><p>The number of nodes in the DAX cluster. A replication factor of 1 will create a single-node cluster, without any read replicas. For additional fault tolerance, you can create a multiple node cluster with one or more read replicas. To do this, set <i>ReplicationFactor</i> to 2 or more.</p> <note> <p>AWS recommends that you have at least two read replicas per cluster.</p> </note></p>
    #[serde(rename = "ReplicationFactor")]
    pub replication_factor: i64,
    /// <p>Represents the settings used to enable server-side encryption on the cluster.</p>
    #[serde(rename = "SSESpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_specification: Option<SSESpecification>,
    /// <p>A list of security group IDs to be assigned to each node in the DAX cluster. (Each of the security group ID is system-generated.)</p> <p>If this parameter is not specified, DAX assigns the default VPC security group to each node.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p><p>The name of the subnet group to be used for the replication group.</p> <important> <p>DAX clusters can only run in an Amazon VPC environment. All of the subnets that you specify in a subnet group must exist in the same VPC.</p> </important></p>
    #[serde(rename = "SubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<String>,
    /// <p>A set of tags to associate with the DAX cluster. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateClusterResponse {
    /// <p>A description of the DAX cluster that you have created.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateParameterGroupRequest {
    /// <p>A description of the parameter group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the parameter group to apply to all of the clusters in this replication group.</p>
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateParameterGroupResponse {
    /// <p>Represents the output of a <i>CreateParameterGroup</i> action.</p>
    #[serde(rename = "ParameterGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group: Option<ParameterGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSubnetGroupRequest {
    /// <p>A description for the subnet group</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A name for the subnet group. This value is stored as a lowercase string. </p>
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: String,
    /// <p>A list of VPC subnet IDs for the subnet group.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSubnetGroupResponse {
    /// <p>Represents the output of a <i>CreateSubnetGroup</i> operation.</p>
    #[serde(rename = "SubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group: Option<SubnetGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DecreaseReplicationFactorRequest {
    /// <p>The Availability Zone(s) from which to remove nodes.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The name of the DAX cluster from which you want to remove nodes.</p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// <p>The new number of nodes for the DAX cluster.</p>
    #[serde(rename = "NewReplicationFactor")]
    pub new_replication_factor: i64,
    /// <p>The unique identifiers of the nodes to be removed from the cluster.</p>
    #[serde(rename = "NodeIdsToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids_to_remove: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DecreaseReplicationFactorResponse {
    /// <p>A description of the DAX cluster, after you have decreased its replication factor.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteClusterRequest {
    /// <p>The name of the cluster to be deleted.</p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteClusterResponse {
    /// <p>A description of the DAX cluster that is being deleted.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteParameterGroupRequest {
    /// <p>The name of the parameter group to delete.</p>
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteParameterGroupResponse {
    /// <p>A user-specified message for this action (i.e., a reason for deleting the parameter group).</p>
    #[serde(rename = "DeletionMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSubnetGroupRequest {
    /// <p>The name of the subnet group to delete.</p>
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSubnetGroupResponse {
    /// <p>A user-specified message for this action (i.e., a reason for deleting the subnet group).</p>
    #[serde(rename = "DeletionMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeClustersRequest {
    /// <p>The names of the DAX clusters being described.</p>
    #[serde(rename = "ClusterNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_names: Option<Vec<String>>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeClustersResponse {
    /// <p>The descriptions of your DAX clusters, in response to a <i>DescribeClusters</i> request.</p>
    #[serde(rename = "Clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<Cluster>>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDefaultParametersRequest {
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDefaultParametersResponse {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of parameters. Each element in the list represents one parameter.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEventsRequest {
    /// <p>The number of minutes' worth of events to retrieve.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>The end of the time interval for which to retrieve events, specified in ISO 8601 format.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier of the event source for which events will be returned. If not specified, then all sources are included in the response.</p>
    #[serde(rename = "SourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// <p>The event source to retrieve events for. If no value is specified, all events are returned.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The beginning of the time interval to retrieve events for, specified in ISO 8601 format.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventsResponse {
    /// <p>An array of events. Each element in the array represents one event.</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeParameterGroupsRequest {
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The names of the parameter groups.</p>
    #[serde(rename = "ParameterGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeParameterGroupsResponse {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of parameter groups. Each element in the array represents one parameter group.</p>
    #[serde(rename = "ParameterGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_groups: Option<Vec<ParameterGroup>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeParametersRequest {
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the parameter group.</p>
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: String,
    /// <p>How the parameter is defined. For example, <code>system</code> denotes a system-defined parameter.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeParametersResponse {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of parameters within a parameter group. Each element in the list represents one parameter.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSubnetGroupsRequest {
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the subnet group.</p>
    #[serde(rename = "SubnetGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSubnetGroupsResponse {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of subnet groups. Each element in the array represents a single subnet group.</p>
    #[serde(rename = "SubnetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_groups: Option<Vec<SubnetGroup>>,
}

/// <p>Represents the information required for client programs to connect to the configuration endpoint for a DAX cluster, or to an individual node within the cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Endpoint {
    /// <p>The DNS hostname of the endpoint.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The port number that applications should use to connect to the endpoint.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// <p>Represents a single occurrence of something interesting within the system. Some examples of events are creating a DAX cluster, adding or removing a node, or rebooting a node.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Event {
    /// <p>The date and time when the event occurred.</p>
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    /// <p>A user-defined message associated with the event.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The source of the event. For example, if the event occurred at the node level, the source would be the node ID.</p>
    #[serde(rename = "SourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// <p>Specifies the origin of this event - a cluster, a parameter group, a node ID, etc.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct IncreaseReplicationFactorRequest {
    /// <p>The Availability Zones (AZs) in which the cluster nodes will be created. All nodes belonging to the cluster are placed in these Availability Zones. Use this parameter if you want to distribute the nodes across multiple AZs.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The name of the DAX cluster that will receive additional nodes.</p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// <p>The new number of nodes for the DAX cluster.</p>
    #[serde(rename = "NewReplicationFactor")]
    pub new_replication_factor: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IncreaseReplicationFactorResponse {
    /// <p>A description of the DAX cluster. with its new replication factor.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsRequest {
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the DAX resource to which the tags belong.</p>
    #[serde(rename = "ResourceName")]
    pub resource_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsResponse {
    /// <p>If this value is present, there are additional results to be displayed. To retrieve them, call <code>ListTags</code> again, with <code>NextToken</code> set to this value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tags currently associated with the DAX cluster.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents an individual node within a DAX cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Node {
    /// <p>The Availability Zone (AZ) in which the node has been deployed.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The endpoint for the node, consisting of a DNS name and a port number. Client applications can connect directly to a node endpoint, if desired (as an alternative to allowing DAX client software to intelligently route requests and responses to nodes in the DAX cluster.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
    /// <p>The date and time (in UNIX epoch format) when the node was launched.</p>
    #[serde(rename = "NodeCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_create_time: Option<f64>,
    /// <p>A system-generated identifier for the node.</p>
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// <p>The current status of the node. For example: <code>available</code>.</p>
    #[serde(rename = "NodeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_status: Option<String>,
    /// <p>The status of the parameter group associated with this node. For example, <code>in-sync</code>.</p>
    #[serde(rename = "ParameterGroupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_status: Option<String>,
}

/// <p>Represents a parameter value that is applicable to a particular node type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodeTypeSpecificValue {
    /// <p>A node type to which the parameter value applies.</p>
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <p>The parameter value for this node type.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Describes a notification topic and its status. Notification topics are used for publishing DAX events to subscribers using Amazon Simple Notification Service (SNS).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NotificationConfiguration {
    /// <p>The Amazon Resource Name (ARN) that identifies the topic. </p>
    #[serde(rename = "TopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    /// <p>The current state of the topic.</p>
    #[serde(rename = "TopicStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_status: Option<String>,
}

/// <p>Describes an individual setting that controls some aspect of DAX behavior.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Parameter {
    /// <p>A range of values within which the parameter can be set.</p>
    #[serde(rename = "AllowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    /// <p>The conditions under which changes to this parameter can be applied. For example, <code>requires-reboot</code> indicates that a new value for this parameter will only take effect if a node is rebooted.</p>
    #[serde(rename = "ChangeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    /// <p>The data type of the parameter. For example, <code>integer</code>:</p>
    #[serde(rename = "DataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// <p>A description of the parameter</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether the customer is allowed to modify the parameter.</p>
    #[serde(rename = "IsModifiable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_modifiable: Option<String>,
    /// <p>A list of node types, and specific parameter values for each node.</p>
    #[serde(rename = "NodeTypeSpecificValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type_specific_values: Option<Vec<NodeTypeSpecificValue>>,
    /// <p>The name of the parameter.</p>
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// <p>Determines whether the parameter can be applied to any nodes, or only nodes of a particular type.</p>
    #[serde(rename = "ParameterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<String>,
    /// <p>The value for the parameter.</p>
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    /// <p>How the parameter is defined. For example, <code>system</code> denotes a system-defined parameter.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// <p>A named set of parameters that are applied to all of the nodes in a DAX cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ParameterGroup {
    /// <p>A description of the parameter group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the parameter group.</p>
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
}

/// <p>The status of a parameter group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ParameterGroupStatus {
    /// <p>The node IDs of one or more nodes to be rebooted.</p>
    #[serde(rename = "NodeIdsToReboot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids_to_reboot: Option<Vec<String>>,
    /// <p>The status of parameter updates. </p>
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
    /// <p>The name of the parameter group.</p>
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
}

/// <p>An individual DAX parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ParameterNameValue {
    /// <p>The name of the parameter.</p>
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// <p>The value of the parameter.</p>
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebootNodeRequest {
    /// <p>The name of the DAX cluster containing the node to be rebooted.</p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// <p>The system-assigned ID of the node to be rebooted.</p>
    #[serde(rename = "NodeId")]
    pub node_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RebootNodeResponse {
    /// <p>A description of the DAX cluster after a node has been rebooted.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

/// <p>The description of the server-side encryption status on the specified DAX cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SSEDescription {
    /// <p><p>The current state of server-side encryption:</p> <ul> <li> <p> <code>ENABLING</code> - Server-side encryption is being enabled.</p> </li> <li> <p> <code>ENABLED</code> - Server-side encryption is enabled.</p> </li> <li> <p> <code>DISABLING</code> - Server-side encryption is being disabled.</p> </li> <li> <p> <code>DISABLED</code> - Server-side encryption is disabled.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents the settings used to enable server-side encryption.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SSESpecification {
    /// <p>Indicates whether server-side encryption is enabled (true) or disabled (false) on the cluster.</p>
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

/// <p>An individual VPC security group and its status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityGroupMembership {
    /// <p>The unique ID for this security group.</p>
    #[serde(rename = "SecurityGroupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_identifier: Option<String>,
    /// <p>The status of this security group.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents the subnet associated with a DAX cluster. This parameter refers to subnets defined in Amazon Virtual Private Cloud (Amazon VPC) and used with DAX.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Subnet {
    /// <p>The Availability Zone (AZ) for subnet subnet.</p>
    #[serde(rename = "SubnetAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_availability_zone: Option<String>,
    /// <p>The system-assigned identifier for the subnet.</p>
    #[serde(rename = "SubnetIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_identifier: Option<String>,
}

/// <p><p>Represents the output of one of the following actions:</p> <ul> <li> <p> <i>CreateSubnetGroup</i> </p> </li> <li> <p> <i>ModifySubnetGroup</i> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubnetGroup {
    /// <p>The description of the subnet group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the subnet group.</p>
    #[serde(rename = "SubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<String>,
    /// <p>A list of subnets associated with the subnet group. </p>
    #[serde(rename = "Subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<Subnet>>,
    /// <p>The Amazon Virtual Private Cloud identifier (VPC ID) of the subnet group.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>A description of a tag. Every tag is a key-value pair. You can add up to 50 tags to a single DAX cluster.</p> <p>AWS-assigned tag names and values are automatically assigned the <code>aws:</code> prefix, which the user cannot assign. AWS-assigned tag names do not count towards the tag limit of 50. User-assigned tag names have the prefix <code>user:</code>.</p> <p>You cannot backdate the application of a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key for the tag. Tag keys are case sensitive. Every DAX cluster can only have one tag with the same key. If you try to add an existing tag (same key), the existing tag value will be updated to the new value.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value of the tag. Tag values are case-sensitive and can be null. </p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The name of the DAX resource to which tags should be added.</p>
    #[serde(rename = "ResourceName")]
    pub resource_name: String,
    /// <p>The tags to be assigned to the DAX resource. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {
    /// <p>The list of tags that are associated with the DAX resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The name of the DAX resource from which the tags should be removed.</p>
    #[serde(rename = "ResourceName")]
    pub resource_name: String,
    /// <p>A list of tag keys. If the DAX cluster has any tags with these keys, then the tags are removed from the cluster.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {
    /// <p>The tag keys that have been removed from the cluster.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateClusterRequest {
    /// <p>The name of the DAX cluster to be modified.</p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// <p>A description of the changes being made to the cluster.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that identifies the topic.</p>
    #[serde(rename = "NotificationTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<String>,
    /// <p>The current state of the topic.</p>
    #[serde(rename = "NotificationTopicStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_status: Option<String>,
    /// <p>The name of a parameter group for this cluster.</p>
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    /// <p>A range of time when maintenance of DAX cluster software will be performed. For example: <code>sun:01:00-sun:09:00</code>. Cluster maintenance normally takes less than 30 minutes, and is performed automatically within the maintenance window.</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>A list of user-specified security group IDs to be assigned to each node in the DAX cluster. If this parameter is not specified, DAX assigns the default VPC security group to each node.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateClusterResponse {
    /// <p>A description of the DAX cluster, after it has been modified.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateParameterGroupRequest {
    /// <p>The name of the parameter group.</p>
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: String,
    /// <p>An array of name-value pairs for the parameters in the group. Each element in the array represents a single parameter.</p>
    #[serde(rename = "ParameterNameValues")]
    pub parameter_name_values: Vec<ParameterNameValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateParameterGroupResponse {
    /// <p>The parameter group that has been modified.</p>
    #[serde(rename = "ParameterGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group: Option<ParameterGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSubnetGroupRequest {
    /// <p>A description of the subnet group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the subnet group.</p>
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: String,
    /// <p>A list of subnet IDs in the subnet group.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSubnetGroupResponse {
    /// <p>The subnet group that has been modified.</p>
    #[serde(rename = "SubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group: Option<SubnetGroup>,
}

/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>You already have a DAX cluster with the given identifier.</p>
    ClusterAlreadyExistsFault(String),
    /// <p>You have attempted to exceed the maximum number of DAX clusters for your AWS account.</p>
    ClusterQuotaForCustomerExceededFault(String),
    /// <p>There are not enough system resources to create the cluster you requested (or to resize an already-existing cluster). </p>
    InsufficientClusterCapacityFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>One or more parameters in a parameter group are in an invalid state.</p>
    InvalidParameterGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>You have attempted to exceed the maximum number of nodes for a DAX cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    /// <p>You have attempted to exceed the maximum number of nodes for your AWS account.</p>
    NodeQuotaForCustomerExceededFault(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),

    ServiceLinkedRoleNotFoundFault(String),
    /// <p>The requested subnet group name does not refer to an existing subnet group.</p>
    SubnetGroupNotFoundFault(String),
    /// <p>You have exceeded the maximum number of tags for this DAX cluster.</p>
    TagQuotaPerResourceExceeded(String),
}

impl CreateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterAlreadyExistsFault" => {
                    return RusotoError::Service(CreateClusterError::ClusterAlreadyExistsFault(
                        err.msg,
                    ))
                }
                "ClusterQuotaForCustomerExceededFault" => {
                    return RusotoError::Service(
                        CreateClusterError::ClusterQuotaForCustomerExceededFault(err.msg),
                    )
                }
                "InsufficientClusterCapacityFault" => {
                    return RusotoError::Service(
                        CreateClusterError::InsufficientClusterCapacityFault(err.msg),
                    )
                }
                "InvalidClusterStateFault" => {
                    return RusotoError::Service(CreateClusterError::InvalidClusterStateFault(
                        err.msg,
                    ))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(CreateClusterError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidParameterGroupStateFault" => {
                    return RusotoError::Service(
                        CreateClusterError::InvalidParameterGroupStateFault(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateClusterError::InvalidParameterValue(err.msg))
                }
                "InvalidVPCNetworkStateFault" => {
                    return RusotoError::Service(CreateClusterError::InvalidVPCNetworkStateFault(
                        err.msg,
                    ))
                }
                "NodeQuotaForClusterExceededFault" => {
                    return RusotoError::Service(
                        CreateClusterError::NodeQuotaForClusterExceededFault(err.msg),
                    )
                }
                "NodeQuotaForCustomerExceededFault" => {
                    return RusotoError::Service(
                        CreateClusterError::NodeQuotaForCustomerExceededFault(err.msg),
                    )
                }
                "ParameterGroupNotFoundFault" => {
                    return RusotoError::Service(CreateClusterError::ParameterGroupNotFoundFault(
                        err.msg,
                    ))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        CreateClusterError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "SubnetGroupNotFoundFault" => {
                    return RusotoError::Service(CreateClusterError::SubnetGroupNotFoundFault(
                        err.msg,
                    ))
                }
                "TagQuotaPerResourceExceeded" => {
                    return RusotoError::Service(CreateClusterError::TagQuotaPerResourceExceeded(
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
impl fmt::Display for CreateClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterError::ClusterAlreadyExistsFault(ref cause) => cause,
            CreateClusterError::ClusterQuotaForCustomerExceededFault(ref cause) => cause,
            CreateClusterError::InsufficientClusterCapacityFault(ref cause) => cause,
            CreateClusterError::InvalidClusterStateFault(ref cause) => cause,
            CreateClusterError::InvalidParameterCombination(ref cause) => cause,
            CreateClusterError::InvalidParameterGroupStateFault(ref cause) => cause,
            CreateClusterError::InvalidParameterValue(ref cause) => cause,
            CreateClusterError::InvalidVPCNetworkStateFault(ref cause) => cause,
            CreateClusterError::NodeQuotaForClusterExceededFault(ref cause) => cause,
            CreateClusterError::NodeQuotaForCustomerExceededFault(ref cause) => cause,
            CreateClusterError::ParameterGroupNotFoundFault(ref cause) => cause,
            CreateClusterError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
            CreateClusterError::SubnetGroupNotFoundFault(ref cause) => cause,
            CreateClusterError::TagQuotaPerResourceExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateParameterGroup
#[derive(Debug, PartialEq)]
pub enum CreateParameterGroupError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>One or more parameters in a parameter group are in an invalid state.</p>
    InvalidParameterGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group already exists.</p>
    ParameterGroupAlreadyExistsFault(String),
    /// <p>You have attempted to exceed the maximum number of parameter groups.</p>
    ParameterGroupQuotaExceededFault(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl CreateParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateParameterGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        CreateParameterGroupError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidParameterGroupStateFault" => {
                    return RusotoError::Service(
                        CreateParameterGroupError::InvalidParameterGroupStateFault(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateParameterGroupError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ParameterGroupAlreadyExistsFault" => {
                    return RusotoError::Service(
                        CreateParameterGroupError::ParameterGroupAlreadyExistsFault(err.msg),
                    )
                }
                "ParameterGroupQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateParameterGroupError::ParameterGroupQuotaExceededFault(err.msg),
                    )
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        CreateParameterGroupError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateParameterGroupError::InvalidParameterCombination(ref cause) => cause,
            CreateParameterGroupError::InvalidParameterGroupStateFault(ref cause) => cause,
            CreateParameterGroupError::InvalidParameterValue(ref cause) => cause,
            CreateParameterGroupError::ParameterGroupAlreadyExistsFault(ref cause) => cause,
            CreateParameterGroupError::ParameterGroupQuotaExceededFault(ref cause) => cause,
            CreateParameterGroupError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSubnetGroup
#[derive(Debug, PartialEq)]
pub enum CreateSubnetGroupError {
    /// <p>An invalid subnet identifier was specified.</p>
    InvalidSubnet(String),

    ServiceLinkedRoleNotFoundFault(String),
    /// <p>The specified subnet group already exists.</p>
    SubnetGroupAlreadyExistsFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of subnets in a subnet group.</p>
    SubnetGroupQuotaExceededFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of subnets in a subnet group.</p>
    SubnetQuotaExceededFault(String),
}

impl CreateSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSubnetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidSubnet" => {
                    return RusotoError::Service(CreateSubnetGroupError::InvalidSubnet(err.msg))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        CreateSubnetGroupError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "SubnetGroupAlreadyExistsFault" => {
                    return RusotoError::Service(
                        CreateSubnetGroupError::SubnetGroupAlreadyExistsFault(err.msg),
                    )
                }
                "SubnetGroupQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateSubnetGroupError::SubnetGroupQuotaExceededFault(err.msg),
                    )
                }
                "SubnetQuotaExceededFault" => {
                    return RusotoError::Service(CreateSubnetGroupError::SubnetQuotaExceededFault(
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
impl fmt::Display for CreateSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateSubnetGroupError::InvalidSubnet(ref cause) => cause,
            CreateSubnetGroupError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
            CreateSubnetGroupError::SubnetGroupAlreadyExistsFault(ref cause) => cause,
            CreateSubnetGroupError::SubnetGroupQuotaExceededFault(ref cause) => cause,
            CreateSubnetGroupError::SubnetQuotaExceededFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DecreaseReplicationFactor
#[derive(Debug, PartialEq)]
pub enum DecreaseReplicationFactorError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>None of the nodes in the cluster have the given node ID.</p>
    NodeNotFoundFault(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl DecreaseReplicationFactorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DecreaseReplicationFactorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundFault" => {
                    return RusotoError::Service(
                        DecreaseReplicationFactorError::ClusterNotFoundFault(err.msg),
                    )
                }
                "InvalidClusterStateFault" => {
                    return RusotoError::Service(
                        DecreaseReplicationFactorError::InvalidClusterStateFault(err.msg),
                    )
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        DecreaseReplicationFactorError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DecreaseReplicationFactorError::InvalidParameterValue(err.msg),
                    )
                }
                "NodeNotFoundFault" => {
                    return RusotoError::Service(DecreaseReplicationFactorError::NodeNotFoundFault(
                        err.msg,
                    ))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        DecreaseReplicationFactorError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DecreaseReplicationFactorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DecreaseReplicationFactorError {
    fn description(&self) -> &str {
        match *self {
            DecreaseReplicationFactorError::ClusterNotFoundFault(ref cause) => cause,
            DecreaseReplicationFactorError::InvalidClusterStateFault(ref cause) => cause,
            DecreaseReplicationFactorError::InvalidParameterCombination(ref cause) => cause,
            DecreaseReplicationFactorError::InvalidParameterValue(ref cause) => cause,
            DecreaseReplicationFactorError::NodeNotFoundFault(ref cause) => cause,
            DecreaseReplicationFactorError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl DeleteClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundFault" => {
                    return RusotoError::Service(DeleteClusterError::ClusterNotFoundFault(err.msg))
                }
                "InvalidClusterStateFault" => {
                    return RusotoError::Service(DeleteClusterError::InvalidClusterStateFault(
                        err.msg,
                    ))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(DeleteClusterError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteClusterError::InvalidParameterValue(err.msg))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        DeleteClusterError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClusterError {
    fn description(&self) -> &str {
        match *self {
            DeleteClusterError::ClusterNotFoundFault(ref cause) => cause,
            DeleteClusterError::InvalidClusterStateFault(ref cause) => cause,
            DeleteClusterError::InvalidParameterCombination(ref cause) => cause,
            DeleteClusterError::InvalidParameterValue(ref cause) => cause,
            DeleteClusterError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteParameterGroup
#[derive(Debug, PartialEq)]
pub enum DeleteParameterGroupError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>One or more parameters in a parameter group are in an invalid state.</p>
    InvalidParameterGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl DeleteParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteParameterGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        DeleteParameterGroupError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidParameterGroupStateFault" => {
                    return RusotoError::Service(
                        DeleteParameterGroupError::InvalidParameterGroupStateFault(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteParameterGroupError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ParameterGroupNotFoundFault" => {
                    return RusotoError::Service(
                        DeleteParameterGroupError::ParameterGroupNotFoundFault(err.msg),
                    )
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        DeleteParameterGroupError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteParameterGroupError::InvalidParameterCombination(ref cause) => cause,
            DeleteParameterGroupError::InvalidParameterGroupStateFault(ref cause) => cause,
            DeleteParameterGroupError::InvalidParameterValue(ref cause) => cause,
            DeleteParameterGroupError::ParameterGroupNotFoundFault(ref cause) => cause,
            DeleteParameterGroupError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSubnetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteSubnetGroupError {
    ServiceLinkedRoleNotFoundFault(String),
    /// <p>The specified subnet group is currently in use.</p>
    SubnetGroupInUseFault(String),
    /// <p>The requested subnet group name does not refer to an existing subnet group.</p>
    SubnetGroupNotFoundFault(String),
}

impl DeleteSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSubnetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        DeleteSubnetGroupError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "SubnetGroupInUseFault" => {
                    return RusotoError::Service(DeleteSubnetGroupError::SubnetGroupInUseFault(
                        err.msg,
                    ))
                }
                "SubnetGroupNotFoundFault" => {
                    return RusotoError::Service(DeleteSubnetGroupError::SubnetGroupNotFoundFault(
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
impl fmt::Display for DeleteSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteSubnetGroupError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
            DeleteSubnetGroupError::SubnetGroupInUseFault(ref cause) => cause,
            DeleteSubnetGroupError::SubnetGroupNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusters
#[derive(Debug, PartialEq)]
pub enum DescribeClustersError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl DescribeClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClustersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundFault" => {
                    return RusotoError::Service(DescribeClustersError::ClusterNotFoundFault(
                        err.msg,
                    ))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        DescribeClustersError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeClustersError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeClustersError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClustersError {
    fn description(&self) -> &str {
        match *self {
            DescribeClustersError::ClusterNotFoundFault(ref cause) => cause,
            DescribeClustersError::InvalidParameterCombination(ref cause) => cause,
            DescribeClustersError::InvalidParameterValue(ref cause) => cause,
            DescribeClustersError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDefaultParameters
#[derive(Debug, PartialEq)]
pub enum DescribeDefaultParametersError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl DescribeDefaultParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDefaultParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        DescribeDefaultParametersError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeDefaultParametersError::InvalidParameterValue(err.msg),
                    )
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeDefaultParametersError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDefaultParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDefaultParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeDefaultParametersError::InvalidParameterCombination(ref cause) => cause,
            DescribeDefaultParametersError::InvalidParameterValue(ref cause) => cause,
            DescribeDefaultParametersError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl DescribeEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(DescribeEventsError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeEventsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeEventsError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventsError::InvalidParameterCombination(ref cause) => cause,
            DescribeEventsError::InvalidParameterValue(ref cause) => cause,
            DescribeEventsError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeParameterGroups
#[derive(Debug, PartialEq)]
pub enum DescribeParameterGroupsError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl DescribeParameterGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeParameterGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        DescribeParameterGroupsError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeParameterGroupsError::InvalidParameterValue(err.msg),
                    )
                }
                "ParameterGroupNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeParameterGroupsError::ParameterGroupNotFoundFault(err.msg),
                    )
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeParameterGroupsError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeParameterGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeParameterGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeParameterGroupsError::InvalidParameterCombination(ref cause) => cause,
            DescribeParameterGroupsError::InvalidParameterValue(ref cause) => cause,
            DescribeParameterGroupsError::ParameterGroupNotFoundFault(ref cause) => cause,
            DescribeParameterGroupsError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeParameters
#[derive(Debug, PartialEq)]
pub enum DescribeParametersError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl DescribeParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        DescribeParametersError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeParametersError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ParameterGroupNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeParametersError::ParameterGroupNotFoundFault(err.msg),
                    )
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeParametersError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
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
            DescribeParametersError::InvalidParameterCombination(ref cause) => cause,
            DescribeParametersError::InvalidParameterValue(ref cause) => cause,
            DescribeParametersError::ParameterGroupNotFoundFault(ref cause) => cause,
            DescribeParametersError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSubnetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeSubnetGroupsError {
    ServiceLinkedRoleNotFoundFault(String),
    /// <p>The requested subnet group name does not refer to an existing subnet group.</p>
    SubnetGroupNotFoundFault(String),
}

impl DescribeSubnetGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSubnetGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeSubnetGroupsError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "SubnetGroupNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeSubnetGroupsError::SubnetGroupNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeSubnetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSubnetGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeSubnetGroupsError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
            DescribeSubnetGroupsError::SubnetGroupNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by IncreaseReplicationFactor
#[derive(Debug, PartialEq)]
pub enum IncreaseReplicationFactorError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>There are not enough system resources to create the cluster you requested (or to resize an already-existing cluster). </p>
    InsufficientClusterCapacityFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>You have attempted to exceed the maximum number of nodes for a DAX cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    /// <p>You have attempted to exceed the maximum number of nodes for your AWS account.</p>
    NodeQuotaForCustomerExceededFault(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl IncreaseReplicationFactorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<IncreaseReplicationFactorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundFault" => {
                    return RusotoError::Service(
                        IncreaseReplicationFactorError::ClusterNotFoundFault(err.msg),
                    )
                }
                "InsufficientClusterCapacityFault" => {
                    return RusotoError::Service(
                        IncreaseReplicationFactorError::InsufficientClusterCapacityFault(err.msg),
                    )
                }
                "InvalidClusterStateFault" => {
                    return RusotoError::Service(
                        IncreaseReplicationFactorError::InvalidClusterStateFault(err.msg),
                    )
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        IncreaseReplicationFactorError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        IncreaseReplicationFactorError::InvalidParameterValue(err.msg),
                    )
                }
                "InvalidVPCNetworkStateFault" => {
                    return RusotoError::Service(
                        IncreaseReplicationFactorError::InvalidVPCNetworkStateFault(err.msg),
                    )
                }
                "NodeQuotaForClusterExceededFault" => {
                    return RusotoError::Service(
                        IncreaseReplicationFactorError::NodeQuotaForClusterExceededFault(err.msg),
                    )
                }
                "NodeQuotaForCustomerExceededFault" => {
                    return RusotoError::Service(
                        IncreaseReplicationFactorError::NodeQuotaForCustomerExceededFault(err.msg),
                    )
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        IncreaseReplicationFactorError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for IncreaseReplicationFactorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for IncreaseReplicationFactorError {
    fn description(&self) -> &str {
        match *self {
            IncreaseReplicationFactorError::ClusterNotFoundFault(ref cause) => cause,
            IncreaseReplicationFactorError::InsufficientClusterCapacityFault(ref cause) => cause,
            IncreaseReplicationFactorError::InvalidClusterStateFault(ref cause) => cause,
            IncreaseReplicationFactorError::InvalidParameterCombination(ref cause) => cause,
            IncreaseReplicationFactorError::InvalidParameterValue(ref cause) => cause,
            IncreaseReplicationFactorError::InvalidVPCNetworkStateFault(ref cause) => cause,
            IncreaseReplicationFactorError::NodeQuotaForClusterExceededFault(ref cause) => cause,
            IncreaseReplicationFactorError::NodeQuotaForCustomerExceededFault(ref cause) => cause,
            IncreaseReplicationFactorError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The Amazon Resource Name (ARN) supplied in the request is not valid.</p>
    InvalidARNFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundFault" => {
                    return RusotoError::Service(ListTagsError::ClusterNotFoundFault(err.msg))
                }
                "InvalidARNFault" => {
                    return RusotoError::Service(ListTagsError::InvalidARNFault(err.msg))
                }
                "InvalidClusterStateFault" => {
                    return RusotoError::Service(ListTagsError::InvalidClusterStateFault(err.msg))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(ListTagsError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListTagsError::InvalidParameterValue(err.msg))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(ListTagsError::ServiceLinkedRoleNotFoundFault(
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
impl fmt::Display for ListTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsError {
    fn description(&self) -> &str {
        match *self {
            ListTagsError::ClusterNotFoundFault(ref cause) => cause,
            ListTagsError::InvalidARNFault(ref cause) => cause,
            ListTagsError::InvalidClusterStateFault(ref cause) => cause,
            ListTagsError::InvalidParameterCombination(ref cause) => cause,
            ListTagsError::InvalidParameterValue(ref cause) => cause,
            ListTagsError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootNode
#[derive(Debug, PartialEq)]
pub enum RebootNodeError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>None of the nodes in the cluster have the given node ID.</p>
    NodeNotFoundFault(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl RebootNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootNodeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundFault" => {
                    return RusotoError::Service(RebootNodeError::ClusterNotFoundFault(err.msg))
                }
                "InvalidClusterStateFault" => {
                    return RusotoError::Service(RebootNodeError::InvalidClusterStateFault(err.msg))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(RebootNodeError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(RebootNodeError::InvalidParameterValue(err.msg))
                }
                "NodeNotFoundFault" => {
                    return RusotoError::Service(RebootNodeError::NodeNotFoundFault(err.msg))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(RebootNodeError::ServiceLinkedRoleNotFoundFault(
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
impl fmt::Display for RebootNodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootNodeError {
    fn description(&self) -> &str {
        match *self {
            RebootNodeError::ClusterNotFoundFault(ref cause) => cause,
            RebootNodeError::InvalidClusterStateFault(ref cause) => cause,
            RebootNodeError::InvalidParameterCombination(ref cause) => cause,
            RebootNodeError::InvalidParameterValue(ref cause) => cause,
            RebootNodeError::NodeNotFoundFault(ref cause) => cause,
            RebootNodeError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The Amazon Resource Name (ARN) supplied in the request is not valid.</p>
    InvalidARNFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),

    ServiceLinkedRoleNotFoundFault(String),
    /// <p>You have exceeded the maximum number of tags for this DAX cluster.</p>
    TagQuotaPerResourceExceeded(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundFault" => {
                    return RusotoError::Service(TagResourceError::ClusterNotFoundFault(err.msg))
                }
                "InvalidARNFault" => {
                    return RusotoError::Service(TagResourceError::InvalidARNFault(err.msg))
                }
                "InvalidClusterStateFault" => {
                    return RusotoError::Service(TagResourceError::InvalidClusterStateFault(
                        err.msg,
                    ))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameterValue(err.msg))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(TagResourceError::ServiceLinkedRoleNotFoundFault(
                        err.msg,
                    ))
                }
                "TagQuotaPerResourceExceeded" => {
                    return RusotoError::Service(TagResourceError::TagQuotaPerResourceExceeded(
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
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::ClusterNotFoundFault(ref cause) => cause,
            TagResourceError::InvalidARNFault(ref cause) => cause,
            TagResourceError::InvalidClusterStateFault(ref cause) => cause,
            TagResourceError::InvalidParameterCombination(ref cause) => cause,
            TagResourceError::InvalidParameterValue(ref cause) => cause,
            TagResourceError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
            TagResourceError::TagQuotaPerResourceExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The Amazon Resource Name (ARN) supplied in the request is not valid.</p>
    InvalidARNFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),

    ServiceLinkedRoleNotFoundFault(String),
    /// <p>The tag does not exist.</p>
    TagNotFoundFault(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundFault" => {
                    return RusotoError::Service(UntagResourceError::ClusterNotFoundFault(err.msg))
                }
                "InvalidARNFault" => {
                    return RusotoError::Service(UntagResourceError::InvalidARNFault(err.msg))
                }
                "InvalidClusterStateFault" => {
                    return RusotoError::Service(UntagResourceError::InvalidClusterStateFault(
                        err.msg,
                    ))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameterValue(err.msg))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        UntagResourceError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "TagNotFoundFault" => {
                    return RusotoError::Service(UntagResourceError::TagNotFoundFault(err.msg))
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
            UntagResourceError::ClusterNotFoundFault(ref cause) => cause,
            UntagResourceError::InvalidARNFault(ref cause) => cause,
            UntagResourceError::InvalidClusterStateFault(ref cause) => cause,
            UntagResourceError::InvalidParameterCombination(ref cause) => cause,
            UntagResourceError::InvalidParameterValue(ref cause) => cause,
            UntagResourceError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
            UntagResourceError::TagNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCluster
#[derive(Debug, PartialEq)]
pub enum UpdateClusterError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>One or more parameters in a parameter group are in an invalid state.</p>
    InvalidParameterGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl UpdateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundFault" => {
                    return RusotoError::Service(UpdateClusterError::ClusterNotFoundFault(err.msg))
                }
                "InvalidClusterStateFault" => {
                    return RusotoError::Service(UpdateClusterError::InvalidClusterStateFault(
                        err.msg,
                    ))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(UpdateClusterError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidParameterGroupStateFault" => {
                    return RusotoError::Service(
                        UpdateClusterError::InvalidParameterGroupStateFault(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateClusterError::InvalidParameterValue(err.msg))
                }
                "ParameterGroupNotFoundFault" => {
                    return RusotoError::Service(UpdateClusterError::ParameterGroupNotFoundFault(
                        err.msg,
                    ))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        UpdateClusterError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
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
            UpdateClusterError::ClusterNotFoundFault(ref cause) => cause,
            UpdateClusterError::InvalidClusterStateFault(ref cause) => cause,
            UpdateClusterError::InvalidParameterCombination(ref cause) => cause,
            UpdateClusterError::InvalidParameterGroupStateFault(ref cause) => cause,
            UpdateClusterError::InvalidParameterValue(ref cause) => cause,
            UpdateClusterError::ParameterGroupNotFoundFault(ref cause) => cause,
            UpdateClusterError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateParameterGroup
#[derive(Debug, PartialEq)]
pub enum UpdateParameterGroupError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>One or more parameters in a parameter group are in an invalid state.</p>
    InvalidParameterGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),

    ServiceLinkedRoleNotFoundFault(String),
}

impl UpdateParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateParameterGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(
                        UpdateParameterGroupError::InvalidParameterCombination(err.msg),
                    )
                }
                "InvalidParameterGroupStateFault" => {
                    return RusotoError::Service(
                        UpdateParameterGroupError::InvalidParameterGroupStateFault(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateParameterGroupError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ParameterGroupNotFoundFault" => {
                    return RusotoError::Service(
                        UpdateParameterGroupError::ParameterGroupNotFoundFault(err.msg),
                    )
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        UpdateParameterGroupError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateParameterGroupError::InvalidParameterCombination(ref cause) => cause,
            UpdateParameterGroupError::InvalidParameterGroupStateFault(ref cause) => cause,
            UpdateParameterGroupError::InvalidParameterValue(ref cause) => cause,
            UpdateParameterGroupError::ParameterGroupNotFoundFault(ref cause) => cause,
            UpdateParameterGroupError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSubnetGroup
#[derive(Debug, PartialEq)]
pub enum UpdateSubnetGroupError {
    /// <p>An invalid subnet identifier was specified.</p>
    InvalidSubnet(String),

    ServiceLinkedRoleNotFoundFault(String),
    /// <p>The requested subnet group name does not refer to an existing subnet group.</p>
    SubnetGroupNotFoundFault(String),
    /// <p>The requested subnet is being used by another subnet group.</p>
    SubnetInUse(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of subnets in a subnet group.</p>
    SubnetQuotaExceededFault(String),
}

impl UpdateSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSubnetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidSubnet" => {
                    return RusotoError::Service(UpdateSubnetGroupError::InvalidSubnet(err.msg))
                }
                "ServiceLinkedRoleNotFoundFault" => {
                    return RusotoError::Service(
                        UpdateSubnetGroupError::ServiceLinkedRoleNotFoundFault(err.msg),
                    )
                }
                "SubnetGroupNotFoundFault" => {
                    return RusotoError::Service(UpdateSubnetGroupError::SubnetGroupNotFoundFault(
                        err.msg,
                    ))
                }
                "SubnetInUse" => {
                    return RusotoError::Service(UpdateSubnetGroupError::SubnetInUse(err.msg))
                }
                "SubnetQuotaExceededFault" => {
                    return RusotoError::Service(UpdateSubnetGroupError::SubnetQuotaExceededFault(
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
impl fmt::Display for UpdateSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateSubnetGroupError::InvalidSubnet(ref cause) => cause,
            UpdateSubnetGroupError::ServiceLinkedRoleNotFoundFault(ref cause) => cause,
            UpdateSubnetGroupError::SubnetGroupNotFoundFault(ref cause) => cause,
            UpdateSubnetGroupError::SubnetInUse(ref cause) => cause,
            UpdateSubnetGroupError::SubnetQuotaExceededFault(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon DAX API. Amazon DAX clients implement this trait.
pub trait DynamodbAccelerator {
    /// <p>Creates a DAX cluster. All nodes in the cluster run the same DAX caching software.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError>;

    /// <p>Creates a new parameter group. A parameter group is a collection of parameters that you apply to all of the nodes in a DAX cluster.</p>
    fn create_parameter_group(
        &self,
        input: CreateParameterGroupRequest,
    ) -> RusotoFuture<CreateParameterGroupResponse, CreateParameterGroupError>;

    /// <p>Creates a new subnet group.</p>
    fn create_subnet_group(
        &self,
        input: CreateSubnetGroupRequest,
    ) -> RusotoFuture<CreateSubnetGroupResponse, CreateSubnetGroupError>;

    /// <p><p>Removes one or more nodes from a DAX cluster.</p> <note> <p>You cannot use <code>DecreaseReplicationFactor</code> to remove the last node in a DAX cluster. If you need to do this, use <code>DeleteCluster</code> instead.</p> </note></p>
    fn decrease_replication_factor(
        &self,
        input: DecreaseReplicationFactorRequest,
    ) -> RusotoFuture<DecreaseReplicationFactorResponse, DecreaseReplicationFactorError>;

    /// <p>Deletes a previously provisioned DAX cluster. <i>DeleteCluster</i> deletes all associated nodes, node endpoints and the DAX cluster itself. When you receive a successful response from this action, DAX immediately begins deleting the cluster; you cannot cancel or revert this action.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError>;

    /// <p>Deletes the specified parameter group. You cannot delete a parameter group if it is associated with any DAX clusters.</p>
    fn delete_parameter_group(
        &self,
        input: DeleteParameterGroupRequest,
    ) -> RusotoFuture<DeleteParameterGroupResponse, DeleteParameterGroupError>;

    /// <p><p>Deletes a subnet group.</p> <note> <p>You cannot delete a subnet group if it is associated with any DAX clusters.</p> </note></p>
    fn delete_subnet_group(
        &self,
        input: DeleteSubnetGroupRequest,
    ) -> RusotoFuture<DeleteSubnetGroupResponse, DeleteSubnetGroupError>;

    /// <p>Returns information about all provisioned DAX clusters if no cluster identifier is specified, or about a specific DAX cluster if a cluster identifier is supplied.</p> <p>If the cluster is in the CREATING state, only cluster level information will be displayed until all of the nodes are successfully provisioned.</p> <p>If the cluster is in the DELETING state, only cluster level information will be displayed.</p> <p>If nodes are currently being added to the DAX cluster, node endpoint information and creation time for the additional nodes will not be displayed until they are completely provisioned. When the DAX cluster state is <i>available</i>, the cluster is ready for use.</p> <p>If nodes are currently being removed from the DAX cluster, no endpoint information for the removed nodes is displayed.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> RusotoFuture<DescribeClustersResponse, DescribeClustersError>;

    /// <p>Returns the default system parameter information for the DAX caching software.</p>
    fn describe_default_parameters(
        &self,
        input: DescribeDefaultParametersRequest,
    ) -> RusotoFuture<DescribeDefaultParametersResponse, DescribeDefaultParametersError>;

    /// <p>Returns events related to DAX clusters and parameter groups. You can obtain events specific to a particular DAX cluster or parameter group by providing the name as a parameter.</p> <p>By default, only the events occurring within the last hour are returned; however, you can retrieve up to 14 days' worth of events if necessary.</p>
    fn describe_events(
        &self,
        input: DescribeEventsRequest,
    ) -> RusotoFuture<DescribeEventsResponse, DescribeEventsError>;

    /// <p>Returns a list of parameter group descriptions. If a parameter group name is specified, the list will contain only the descriptions for that group.</p>
    fn describe_parameter_groups(
        &self,
        input: DescribeParameterGroupsRequest,
    ) -> RusotoFuture<DescribeParameterGroupsResponse, DescribeParameterGroupsError>;

    /// <p>Returns the detailed parameter list for a particular parameter group.</p>
    fn describe_parameters(
        &self,
        input: DescribeParametersRequest,
    ) -> RusotoFuture<DescribeParametersResponse, DescribeParametersError>;

    /// <p>Returns a list of subnet group descriptions. If a subnet group name is specified, the list will contain only the description of that group.</p>
    fn describe_subnet_groups(
        &self,
        input: DescribeSubnetGroupsRequest,
    ) -> RusotoFuture<DescribeSubnetGroupsResponse, DescribeSubnetGroupsError>;

    /// <p>Adds one or more nodes to a DAX cluster.</p>
    fn increase_replication_factor(
        &self,
        input: IncreaseReplicationFactorRequest,
    ) -> RusotoFuture<IncreaseReplicationFactorResponse, IncreaseReplicationFactorError>;

    /// <p>List all of the tags for a DAX cluster. You can call <code>ListTags</code> up to 10 times per second, per account.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError>;

    /// <p>Reboots a single node of a DAX cluster. The reboot action takes place as soon as possible. During the reboot, the node status is set to REBOOTING.</p>
    fn reboot_node(
        &self,
        input: RebootNodeRequest,
    ) -> RusotoFuture<RebootNodeResponse, RebootNodeError>;

    /// <p>Associates a set of tags with a DAX resource. You can call <code>TagResource</code> up to 5 times per second, per account. </p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes the association of tags from a DAX resource. You can call <code>UntagResource</code> up to 5 times per second, per account. </p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Modifies the settings for a DAX cluster. You can use this action to change one or more cluster configuration parameters by specifying the parameters and the new values.</p>
    fn update_cluster(
        &self,
        input: UpdateClusterRequest,
    ) -> RusotoFuture<UpdateClusterResponse, UpdateClusterError>;

    /// <p>Modifies the parameters of a parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs.</p>
    fn update_parameter_group(
        &self,
        input: UpdateParameterGroupRequest,
    ) -> RusotoFuture<UpdateParameterGroupResponse, UpdateParameterGroupError>;

    /// <p>Modifies an existing subnet group.</p>
    fn update_subnet_group(
        &self,
        input: UpdateSubnetGroupRequest,
    ) -> RusotoFuture<UpdateSubnetGroupResponse, UpdateSubnetGroupError>;
}
/// A client for the Amazon DAX API.
#[derive(Clone)]
pub struct DynamodbAcceleratorClient {
    client: Client,
    region: region::Region,
}

impl DynamodbAcceleratorClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DynamodbAcceleratorClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DynamodbAcceleratorClient
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

    pub fn new_with_client(client: Client, region: region::Region) -> DynamodbAcceleratorClient {
        DynamodbAcceleratorClient { client, region }
    }
}

impl fmt::Debug for DynamodbAcceleratorClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DynamodbAcceleratorClient")
            .field("region", &self.region)
            .finish()
    }
}

impl DynamodbAccelerator for DynamodbAcceleratorClient {
    /// <p>Creates a DAX cluster. All nodes in the cluster run the same DAX caching software.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.CreateCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateClusterResponse, _>()
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

    /// <p>Creates a new parameter group. A parameter group is a collection of parameters that you apply to all of the nodes in a DAX cluster.</p>
    fn create_parameter_group(
        &self,
        input: CreateParameterGroupRequest,
    ) -> RusotoFuture<CreateParameterGroupResponse, CreateParameterGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.CreateParameterGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateParameterGroupResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateParameterGroupError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a new subnet group.</p>
    fn create_subnet_group(
        &self,
        input: CreateSubnetGroupRequest,
    ) -> RusotoFuture<CreateSubnetGroupResponse, CreateSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.CreateSubnetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateSubnetGroupResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateSubnetGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Removes one or more nodes from a DAX cluster.</p> <note> <p>You cannot use <code>DecreaseReplicationFactor</code> to remove the last node in a DAX cluster. If you need to do this, use <code>DeleteCluster</code> instead.</p> </note></p>
    fn decrease_replication_factor(
        &self,
        input: DecreaseReplicationFactorRequest,
    ) -> RusotoFuture<DecreaseReplicationFactorResponse, DecreaseReplicationFactorError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DecreaseReplicationFactor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DecreaseReplicationFactorResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DecreaseReplicationFactorError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a previously provisioned DAX cluster. <i>DeleteCluster</i> deletes all associated nodes, node endpoints and the DAX cluster itself. When you receive a successful response from this action, DAX immediately begins deleting the cluster; you cannot cancel or revert this action.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DeleteCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteClusterResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified parameter group. You cannot delete a parameter group if it is associated with any DAX clusters.</p>
    fn delete_parameter_group(
        &self,
        input: DeleteParameterGroupRequest,
    ) -> RusotoFuture<DeleteParameterGroupResponse, DeleteParameterGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DeleteParameterGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteParameterGroupResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteParameterGroupError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Deletes a subnet group.</p> <note> <p>You cannot delete a subnet group if it is associated with any DAX clusters.</p> </note></p>
    fn delete_subnet_group(
        &self,
        input: DeleteSubnetGroupRequest,
    ) -> RusotoFuture<DeleteSubnetGroupResponse, DeleteSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DeleteSubnetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteSubnetGroupResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteSubnetGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about all provisioned DAX clusters if no cluster identifier is specified, or about a specific DAX cluster if a cluster identifier is supplied.</p> <p>If the cluster is in the CREATING state, only cluster level information will be displayed until all of the nodes are successfully provisioned.</p> <p>If the cluster is in the DELETING state, only cluster level information will be displayed.</p> <p>If nodes are currently being added to the DAX cluster, node endpoint information and creation time for the additional nodes will not be displayed until they are completely provisioned. When the DAX cluster state is <i>available</i>, the cluster is ready for use.</p> <p>If nodes are currently being removed from the DAX cluster, no endpoint information for the removed nodes is displayed.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> RusotoFuture<DescribeClustersResponse, DescribeClustersError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeClusters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeClustersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeClustersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the default system parameter information for the DAX caching software.</p>
    fn describe_default_parameters(
        &self,
        input: DescribeDefaultParametersRequest,
    ) -> RusotoFuture<DescribeDefaultParametersResponse, DescribeDefaultParametersError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeDefaultParameters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDefaultParametersResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDefaultParametersError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns events related to DAX clusters and parameter groups. You can obtain events specific to a particular DAX cluster or parameter group by providing the name as a parameter.</p> <p>By default, only the events occurring within the last hour are returned; however, you can retrieve up to 14 days' worth of events if necessary.</p>
    fn describe_events(
        &self,
        input: DescribeEventsRequest,
    ) -> RusotoFuture<DescribeEventsResponse, DescribeEventsError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEventsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeEventsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of parameter group descriptions. If a parameter group name is specified, the list will contain only the descriptions for that group.</p>
    fn describe_parameter_groups(
        &self,
        input: DescribeParameterGroupsRequest,
    ) -> RusotoFuture<DescribeParameterGroupsResponse, DescribeParameterGroupsError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeParameterGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeParameterGroupsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeParameterGroupsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the detailed parameter list for a particular parameter group.</p>
    fn describe_parameters(
        &self,
        input: DescribeParametersRequest,
    ) -> RusotoFuture<DescribeParametersResponse, DescribeParametersError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeParameters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeParametersResponse, _>()
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

    /// <p>Returns a list of subnet group descriptions. If a subnet group name is specified, the list will contain only the description of that group.</p>
    fn describe_subnet_groups(
        &self,
        input: DescribeSubnetGroupsRequest,
    ) -> RusotoFuture<DescribeSubnetGroupsResponse, DescribeSubnetGroupsError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeSubnetGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeSubnetGroupsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeSubnetGroupsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Adds one or more nodes to a DAX cluster.</p>
    fn increase_replication_factor(
        &self,
        input: IncreaseReplicationFactorRequest,
    ) -> RusotoFuture<IncreaseReplicationFactorResponse, IncreaseReplicationFactorError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.IncreaseReplicationFactor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<IncreaseReplicationFactorResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(IncreaseReplicationFactorError::from_response(response))
                }))
            }
        })
    }

    /// <p>List all of the tags for a DAX cluster. You can call <code>ListTags</code> up to 10 times per second, per account.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Reboots a single node of a DAX cluster. The reboot action takes place as soon as possible. During the reboot, the node status is set to REBOOTING.</p>
    fn reboot_node(
        &self,
        input: RebootNodeRequest,
    ) -> RusotoFuture<RebootNodeResponse, RebootNodeError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.RebootNode");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RebootNodeResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RebootNodeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Associates a set of tags with a DAX resource. You can call <code>TagResource</code> up to 5 times per second, per account. </p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.TagResource");
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

    /// <p>Removes the association of tags from a DAX resource. You can call <code>UntagResource</code> up to 5 times per second, per account. </p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.UntagResource");
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

    /// <p>Modifies the settings for a DAX cluster. You can use this action to change one or more cluster configuration parameters by specifying the parameters and the new values.</p>
    fn update_cluster(
        &self,
        input: UpdateClusterRequest,
    ) -> RusotoFuture<UpdateClusterResponse, UpdateClusterError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.UpdateCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateClusterResponse, _>()
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

    /// <p>Modifies the parameters of a parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs.</p>
    fn update_parameter_group(
        &self,
        input: UpdateParameterGroupRequest,
    ) -> RusotoFuture<UpdateParameterGroupResponse, UpdateParameterGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.UpdateParameterGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateParameterGroupResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateParameterGroupError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Modifies an existing subnet group.</p>
    fn update_subnet_group(
        &self,
        input: UpdateSubnetGroupRequest,
    ) -> RusotoFuture<UpdateSubnetGroupResponse, UpdateSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.UpdateSubnetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateSubnetGroupResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateSubnetGroupError::from_response(response))),
                )
            }
        })
    }
}
