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
/// <p>Describes a modification to the configuration of bring your own license (BYOL) for the specified account. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccountModification {
    /// <p>The IP address range, specified as an IPv4 CIDR block, for the management network interface used for the account.</p>
    #[serde(rename = "DedicatedTenancyManagementCidrRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_management_cidr_range: Option<String>,
    /// <p>The status of BYOL (whether BYOL is being enabled or disabled).</p>
    #[serde(rename = "DedicatedTenancySupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_support: Option<String>,
    /// <p>The error code that is returned if the configuration of BYOL cannot be modified.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message that is returned if the configuration of BYOL cannot be modified.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The state of the modification to the configuration of BYOL.</p>
    #[serde(rename = "ModificationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_state: Option<String>,
    /// <p>The timestamp when the modification of the BYOL configuration was started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateIpGroupsRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The identifiers of one or more IP access control groups.</p>
    #[serde(rename = "GroupIds")]
    pub group_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateIpGroupsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AuthorizeIpRulesRequest {
    /// <p>The identifier of the group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The rules to add to the group.</p>
    #[serde(rename = "UserRules")]
    pub user_rules: Vec<IpRuleItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AuthorizeIpRulesResult {}

/// <p>Describes an Amazon WorkSpaces client.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientProperties {
    /// <p>Specifies whether users can cache their credentials on the Amazon WorkSpaces client. When enabled, users can choose to reconnect to their WorkSpaces without re-entering their credentials. </p>
    #[serde(rename = "ReconnectEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_enabled: Option<String>,
}

/// <p>Information about the Amazon WorkSpaces client.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClientPropertiesResult {
    /// <p>Information about the Amazon WorkSpaces client.</p>
    #[serde(rename = "ClientProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_properties: Option<ClientProperties>,
    /// <p>The resource identifier, in the form of a directory ID.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

/// <p>Describes the compute type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComputeType {
    /// <p>The compute type.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CopyWorkspaceImageRequest {
    /// <p>A description of the image.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the image.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The identifier of the source image.</p>
    #[serde(rename = "SourceImageId")]
    pub source_image_id: String,
    /// <p>The identifier of the source Region.</p>
    #[serde(rename = "SourceRegion")]
    pub source_region: String,
    /// <p>The tags for the image.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CopyWorkspaceImageResult {
    /// <p>The identifier of the image.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateIpGroupRequest {
    /// <p>The description of the group.</p>
    #[serde(rename = "GroupDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_desc: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The tags. Each WorkSpaces resource can have a maximum of 50 tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The rules to add to the group.</p>
    #[serde(rename = "UserRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rules: Option<Vec<IpRuleItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIpGroupResult {
    /// <p>The identifier of the group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTagsRequest {
    /// <p>The identifier of the WorkSpaces resource. The supported resource types are WorkSpaces, registered directories, images, custom bundles, and IP access control groups.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The tags. Each WorkSpaces resource can have a maximum of 50 tags.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTagsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateWorkspacesRequest {
    /// <p>The WorkSpaces to create. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "Workspaces")]
    pub workspaces: Vec<WorkspaceRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be created.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedCreateWorkspaceRequest>>,
    /// <p>Information about the WorkSpaces that were created.</p> <p>Because this operation is asynchronous, the identifier returned is not immediately available for use with other operations. For example, if you call <a>DescribeWorkspaces</a> before the WorkSpace is created, the information returned can be incomplete.</p>
    #[serde(rename = "PendingRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_requests: Option<Vec<Workspace>>,
}

/// <p>Describes the default values used to create a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DefaultWorkspaceCreationProperties {
    /// <p>The identifier of any security groups to apply to WorkSpaces when they are created.</p>
    #[serde(rename = "CustomSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_id: Option<String>,
    /// <p>The organizational unit (OU) in the directory for the WorkSpace machine accounts.</p>
    #[serde(rename = "DefaultOu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ou: Option<String>,
    /// <p>The public IP address to attach to all WorkSpaces that are created or rebuilt.</p>
    #[serde(rename = "EnableInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_internet_access: Option<bool>,
    /// <p>Specifies whether the directory is enabled for Amazon WorkDocs.</p>
    #[serde(rename = "EnableWorkDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_work_docs: Option<bool>,
    /// <p>Specifies whether the WorkSpace user is an administrator on the WorkSpace.</p>
    #[serde(rename = "UserEnabledAsLocalAdministrator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_enabled_as_local_administrator: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIpGroupRequest {
    /// <p>The identifier of the IP access control group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteIpGroupResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTagsRequest {
    /// <p>The identifier of the WorkSpaces resource. The supported resource types are WorkSpaces, registered directories, images, custom bundles, and IP access control groups.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The tag keys.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTagsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteWorkspaceImageRequest {
    /// <p>The identifier of the image.</p>
    #[serde(rename = "ImageId")]
    pub image_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteWorkspaceImageResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAccountModificationsRequest {
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAccountModificationsResult {
    /// <p>The list of modifications to the configuration of BYOL.</p>
    #[serde(rename = "AccountModifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_modifications: Option<Vec<AccountModification>>,
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAccountRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAccountResult {
    /// <p>The IP address range, specified as an IPv4 CIDR block, used for the management network interface.</p> <p>The management network interface is connected to a secure Amazon WorkSpaces management network. It is used for interactive streaming of the WorkSpace desktop to Amazon WorkSpaces clients, and to allow Amazon WorkSpaces to manage the WorkSpace.</p>
    #[serde(rename = "DedicatedTenancyManagementCidrRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_management_cidr_range: Option<String>,
    /// <p>The status of BYOL (whether BYOL is enabled or disabled).</p>
    #[serde(rename = "DedicatedTenancySupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_support: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeClientPropertiesRequest {
    /// <p>The resource identifier, in the form of directory IDs.</p>
    #[serde(rename = "ResourceIds")]
    pub resource_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeClientPropertiesResult {
    /// <p>Information about the specified Amazon WorkSpaces clients.</p>
    #[serde(rename = "ClientPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_properties_list: Option<Vec<ClientPropertiesResult>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeIpGroupsRequest {
    /// <p>The identifiers of one or more IP access control groups.</p>
    #[serde(rename = "GroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeIpGroupsResult {
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the IP access control groups.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<WorkspacesIpGroup>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTagsRequest {
    /// <p>The identifier of the WorkSpaces resource. The supported resource types are WorkSpaces, registered directories, images, custom bundles, and IP access control groups.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTagsResult {
    /// <p>The tags.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkspaceBundlesRequest {
    /// <p>The identifiers of the bundles. You cannot combine this parameter with any other filter.</p>
    #[serde(rename = "BundleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_ids: Option<Vec<String>>,
    /// <p>The token for the next set of results. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The owner of the bundles. You cannot combine this parameter with any other filter.</p> <p>Specify <code>AMAZON</code> to describe the bundles provided by AWS or null to describe the bundles that belong to your account.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkspaceBundlesResult {
    /// <p>Information about the bundles.</p>
    #[serde(rename = "Bundles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<WorkspaceBundle>>,
    /// <p>The token to use to retrieve the next set of results, or null if there are no more results available. This token is valid for one day and must be used within that time frame.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkspaceDirectoriesRequest {
    /// <p>The identifiers of the directories. If the value is null, all directories are retrieved.</p>
    #[serde(rename = "DirectoryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_ids: Option<Vec<String>>,
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkspaceDirectoriesResult {
    /// <p>Information about the directories.</p>
    #[serde(rename = "Directories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<WorkspaceDirectory>>,
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkspaceImagesRequest {
    /// <p>The identifier of the image.</p>
    #[serde(rename = "ImageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkspaceImagesResult {
    /// <p>Information about the images.</p>
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<WorkspaceImage>>,
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkspacesConnectionStatusRequest {
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifiers of the WorkSpaces. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "WorkspaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkspacesConnectionStatusResult {
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the connection status of the WorkSpace.</p>
    #[serde(rename = "WorkspacesConnectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces_connection_status: Option<Vec<WorkspaceConnectionStatus>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkspacesRequest {
    /// <p>The identifier of the bundle. All WorkSpaces that are created from this bundle are retrieved. You cannot combine this parameter with any other filter.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The identifier of the directory. In addition, you can optionally specify a specific directory user (see <code>UserName</code>). You cannot combine this parameter with any other filter.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the directory user. You must specify this parameter with <code>DirectoryId</code>.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// <p>The identifiers of the WorkSpaces. You cannot combine this parameter with any other filter.</p> <p>Because the <a>CreateWorkspaces</a> operation is asynchronous, the identifier it returns is not immediately available. If you immediately call <a>DescribeWorkspaces</a> with this identifier, no information is returned.</p>
    #[serde(rename = "WorkspaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkspacesResult {
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the WorkSpaces.</p> <p>Because <a>CreateWorkspaces</a> is an asynchronous operation, some of the returned information could be incomplete.</p>
    #[serde(rename = "Workspaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<Workspace>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateIpGroupsRequest {
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The identifiers of one or more IP access control groups.</p>
    #[serde(rename = "GroupIds")]
    pub group_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateIpGroupsResult {}

/// <p>Describes a WorkSpace that cannot be created.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailedCreateWorkspaceRequest {
    /// <p>The error code that is returned if the WorkSpace cannot be created.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message that is returned if the WorkSpace cannot be created.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Information about the WorkSpace.</p>
    #[serde(rename = "WorkspaceRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_request: Option<WorkspaceRequest>,
}

/// <p>Describes a WorkSpace that could not be rebooted. (<a>RebootWorkspaces</a>), rebuilt (<a>RebuildWorkspaces</a>), terminated (<a>TerminateWorkspaces</a>), started (<a>StartWorkspaces</a>), or stopped (<a>StopWorkspaces</a>).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailedWorkspaceChangeRequest {
    /// <p>The error code that is returned if the WorkSpace cannot be rebooted.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message that is returned if the WorkSpace cannot be rebooted.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportWorkspaceImageRequest {
    /// <p>The identifier of the EC2 image.</p>
    #[serde(rename = "Ec2ImageId")]
    pub ec_2_image_id: String,
    /// <p>The description of the WorkSpace image.</p>
    #[serde(rename = "ImageDescription")]
    pub image_description: String,
    /// <p>The name of the WorkSpace image.</p>
    #[serde(rename = "ImageName")]
    pub image_name: String,
    /// <p>The ingestion process to be used when importing the image.</p>
    #[serde(rename = "IngestionProcess")]
    pub ingestion_process: String,
    /// <p>The tags. Each WorkSpaces resource can have a maximum of 50 tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportWorkspaceImageResult {
    /// <p>The identifier of the WorkSpace image.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

/// <p>Describes a rule for an IP access control group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpRuleItem {
    /// <p>The IP address range, in CIDR notation.</p>
    #[serde(rename = "ipRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_rule: Option<String>,
    /// <p>The description.</p>
    #[serde(rename = "ruleDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_desc: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAvailableManagementCidrRangesRequest {
    /// <p>The IP address range to search. Specify an IP address range that is compatible with your network and in CIDR notation (that is, specify the range as an IPv4 CIDR block).</p>
    #[serde(rename = "ManagementCidrRangeConstraint")]
    pub management_cidr_range_constraint: String,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAvailableManagementCidrRangesResult {
    /// <p>The list of available IP address ranges, specified as IPv4 CIDR blocks.</p>
    #[serde(rename = "ManagementCidrRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_cidr_ranges: Option<Vec<String>>,
    /// <p>The token to use to retrieve the next set of results, or null if no more results are available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Describes a WorkSpace modification.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModificationState {
    /// <p>The resource.</p>
    #[serde(rename = "Resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// <p>The modification state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyAccountRequest {
    /// <p>The IP address range, specified as an IPv4 CIDR block, for the management network interface. Specify an IP address range that is compatible with your network and in CIDR notation (that is, specify the range as an IPv4 CIDR block). The CIDR block size must be /16 (for example, 203.0.113.25/16). It must also be specified as available by the <code>ListAvailableManagementCidrRanges</code> operation.</p>
    #[serde(rename = "DedicatedTenancyManagementCidrRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_management_cidr_range: Option<String>,
    /// <p>The status of BYOL.</p>
    #[serde(rename = "DedicatedTenancySupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_support: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyAccountResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyClientPropertiesRequest {
    /// <p>Information about the Amazon WorkSpaces client.</p>
    #[serde(rename = "ClientProperties")]
    pub client_properties: ClientProperties,
    /// <p>The resource identifiers, in the form of directory IDs.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyClientPropertiesResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyWorkspacePropertiesRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
    /// <p>The properties of the WorkSpace.</p>
    #[serde(rename = "WorkspaceProperties")]
    pub workspace_properties: WorkspaceProperties,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyWorkspacePropertiesResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyWorkspaceStateRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
    /// <p>The WorkSpace state.</p>
    #[serde(rename = "WorkspaceState")]
    pub workspace_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyWorkspaceStateResult {}

/// <p>The operating system that the image is running.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OperatingSystem {
    /// <p>The operating system.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Describes the information used to reboot a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebootRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebootWorkspacesRequest {
    /// <p>The WorkSpaces to reboot. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "RebootWorkspaceRequests")]
    pub reboot_workspace_requests: Vec<RebootRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RebootWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be rebooted.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Describes the information used to rebuild a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebuildRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebuildWorkspacesRequest {
    /// <p>The WorkSpace to rebuild. You can specify a single WorkSpace.</p>
    #[serde(rename = "RebuildWorkspaceRequests")]
    pub rebuild_workspace_requests: Vec<RebuildRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RebuildWorkspacesResult {
    /// <p>Information about the WorkSpace that could not be rebuilt.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RevokeIpRulesRequest {
    /// <p>The identifier of the group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The rules to remove from the group.</p>
    #[serde(rename = "UserRules")]
    pub user_rules: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RevokeIpRulesResult {}

/// <p>Describes the root volume for a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RootStorage {
    /// <p>The size of the root volume.</p>
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
}

/// <p>Information used to start a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartWorkspacesRequest {
    /// <p>The WorkSpaces to start. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "StartWorkspaceRequests")]
    pub start_workspace_requests: Vec<StartRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be started.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Describes the information used to stop a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopWorkspacesRequest {
    /// <p>The WorkSpaces to stop. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "StopWorkspaceRequests")]
    pub stop_workspace_requests: Vec<StopRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be stopped.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

/// <p>Describes a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Describes the information used to terminate a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TerminateRequest {
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    pub workspace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TerminateWorkspacesRequest {
    /// <p>The WorkSpaces to terminate. You can specify up to 25 WorkSpaces.</p>
    #[serde(rename = "TerminateWorkspaceRequests")]
    pub terminate_workspace_requests: Vec<TerminateRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TerminateWorkspacesResult {
    /// <p>Information about the WorkSpaces that could not be terminated.</p>
    #[serde(rename = "FailedRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRulesOfIpGroupRequest {
    /// <p>The identifier of the group.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>One or more rules.</p>
    #[serde(rename = "UserRules")]
    pub user_rules: Vec<IpRuleItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRulesOfIpGroupResult {}

/// <p>Describes the user storage for a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserStorage {
    /// <p>The size of the user storage.</p>
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
}

/// <p>Describes a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Workspace {
    /// <p>The identifier of the bundle used to create the WorkSpace.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The name of the WorkSpace, as seen by the operating system.</p>
    #[serde(rename = "ComputerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// <p>The identifier of the AWS Directory Service directory for the WorkSpace.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The error code that is returned if the WorkSpace cannot be created.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message that is returned if the WorkSpace cannot be created.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The IP address of the WorkSpace.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>The modification states of the WorkSpace.</p>
    #[serde(rename = "ModificationStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_states: Option<Vec<ModificationState>>,
    /// <p>Indicates whether the data stored on the root volume is encrypted.</p>
    #[serde(rename = "RootVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_encryption_enabled: Option<bool>,
    /// <p>The operational state of the WorkSpace.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The identifier of the subnet for the WorkSpace.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The user for the WorkSpace.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// <p>Indicates whether the data stored on the user volume is encrypted.</p>
    #[serde(rename = "UserVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_encryption_enabled: Option<bool>,
    /// <p>The KMS key used to encrypt data stored on your WorkSpace.</p>
    #[serde(rename = "VolumeEncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_encryption_key: Option<String>,
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// <p>The properties of the WorkSpace.</p>
    #[serde(rename = "WorkspaceProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_properties: Option<WorkspaceProperties>,
}

/// <p>Describes a WorkSpace bundle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkspaceBundle {
    /// <p>The bundle identifier.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The compute type. For more information, see <a href="http://aws.amazon.com/workspaces/details/#Amazon_WorkSpaces_Bundles">Amazon WorkSpaces Bundles</a>.</p>
    #[serde(rename = "ComputeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<ComputeType>,
    /// <p>A description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the bundle.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the bundle. This is the account identifier of the owner, or <code>AMAZON</code> if the bundle is provided by AWS.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The size of the root volume.</p>
    #[serde(rename = "RootStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_storage: Option<RootStorage>,
    /// <p>The size of the user storage.</p>
    #[serde(rename = "UserStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_storage: Option<UserStorage>,
}

/// <p>Describes the connection status of a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkspaceConnectionStatus {
    /// <p>The connection state of the WorkSpace. The connection state is unknown if the WorkSpace is stopped.</p>
    #[serde(rename = "ConnectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>The timestamp of the connection status check.</p>
    #[serde(rename = "ConnectionStateCheckTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state_check_timestamp: Option<f64>,
    /// <p>The timestamp of the last known user connection.</p>
    #[serde(rename = "LastKnownUserConnectionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_user_connection_timestamp: Option<f64>,
    /// <p>The identifier of the WorkSpace.</p>
    #[serde(rename = "WorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

/// <p>Describes an AWS Directory Service directory that is used with Amazon WorkSpaces.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkspaceDirectory {
    /// <p>The directory alias.</p>
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// <p>The user name for the service account.</p>
    #[serde(rename = "CustomerUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user_name: Option<String>,
    /// <p>The directory identifier.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The name of the directory.</p>
    #[serde(rename = "DirectoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_name: Option<String>,
    /// <p>The directory type.</p>
    #[serde(rename = "DirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_type: Option<String>,
    /// <p>The IP addresses of the DNS servers for the directory.</p>
    #[serde(rename = "DnsIpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addresses: Option<Vec<String>>,
    /// <p>The identifier of the IAM role. This is the role that allows Amazon WorkSpaces to make calls to other services, such as Amazon EC2, on your behalf.</p>
    #[serde(rename = "IamRoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_id: Option<String>,
    /// <p>The registration code for the directory. This is the code that users enter in their Amazon WorkSpaces client application to connect to the directory.</p>
    #[serde(rename = "RegistrationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<String>,
    /// <p>The state of the directory's registration with Amazon WorkSpaces</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The identifiers of the subnets used with the directory.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The default creation properties for all WorkSpaces in the directory.</p>
    #[serde(rename = "WorkspaceCreationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_creation_properties: Option<DefaultWorkspaceCreationProperties>,
    /// <p>The identifier of the security group that is assigned to new WorkSpaces.</p>
    #[serde(rename = "WorkspaceSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_security_group_id: Option<String>,
    /// <p>The identifiers of the IP access control groups associated with the directory.</p>
    #[serde(rename = "ipGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_group_ids: Option<Vec<String>>,
}

/// <p>Describes a WorkSpace image.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkspaceImage {
    /// <p>The description of the image.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The error code that is returned for the image.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The text of the error message that is returned for the image.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier of the image.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// <p>The name of the image.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The operating system that the image is running. </p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<OperatingSystem>,
    /// <p>Specifies whether the image is running on dedicated hardware. When bring your own license (BYOL) is enabled, this value is set to DEDICATED. </p>
    #[serde(rename = "RequiredTenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_tenancy: Option<String>,
    /// <p>The status of the image.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Describes a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    /// <p>The compute type. For more information, see <a href="http://aws.amazon.com/workspaces/details/#Amazon_WorkSpaces_Bundles">Amazon WorkSpaces Bundles</a>.</p>
    #[serde(rename = "ComputeTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type_name: Option<String>,
    /// <p>The size of the root volume.</p>
    #[serde(rename = "RootVolumeSizeGib")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_size_gib: Option<i64>,
    /// <p>The running mode. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/running-mode.html">Manage the WorkSpace Running Mode</a>.</p>
    #[serde(rename = "RunningMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode: Option<String>,
    /// <p>The time after a user logs off when WorkSpaces are automatically stopped. Configured in 60 minute intervals.</p>
    #[serde(rename = "RunningModeAutoStopTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode_auto_stop_timeout_in_minutes: Option<i64>,
    /// <p>The size of the user storage.</p>
    #[serde(rename = "UserVolumeSizeGib")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_size_gib: Option<i64>,
}

/// <p>Describes the information used to create a WorkSpace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceRequest {
    /// <p>The identifier of the bundle for the WorkSpace. You can use <a>DescribeWorkspaceBundles</a> to list the available bundles.</p>
    #[serde(rename = "BundleId")]
    pub bundle_id: String,
    /// <p>The identifier of the AWS Directory Service directory for the WorkSpace. You can use <a>DescribeWorkspaceDirectories</a> to list the available directories.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>Indicates whether the data stored on the root volume is encrypted.</p>
    #[serde(rename = "RootVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_encryption_enabled: Option<bool>,
    /// <p>The tags for the WorkSpace.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The username of the user for the WorkSpace. This username must exist in the AWS Directory Service directory for the WorkSpace.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
    /// <p>Indicates whether the data stored on the user volume is encrypted.</p>
    #[serde(rename = "UserVolumeEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_encryption_enabled: Option<bool>,
    /// <p>The KMS key used to encrypt data stored on your WorkSpace.</p>
    #[serde(rename = "VolumeEncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_encryption_key: Option<String>,
    /// <p>The WorkSpace properties.</p>
    #[serde(rename = "WorkspaceProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_properties: Option<WorkspaceProperties>,
}

/// <p>Describes an IP access control group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkspacesIpGroup {
    /// <p>The description of the group.</p>
    #[serde(rename = "groupDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_desc: Option<String>,
    /// <p>The identifier of the group.</p>
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The rules.</p>
    #[serde(rename = "userRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rules: Option<Vec<IpRuleItem>>,
}

/// Errors returned by AssociateIpGroups
#[derive(Debug, PartialEq)]
pub enum AssociateIpGroupsError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>This operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl AssociateIpGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateIpGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociateIpGroupsError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(AssociateIpGroupsError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(AssociateIpGroupsError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(AssociateIpGroupsError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(AssociateIpGroupsError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateIpGroupsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateIpGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateIpGroupsError {
    fn description(&self) -> &str {
        match *self {
            AssociateIpGroupsError::AccessDenied(ref cause) => cause,
            AssociateIpGroupsError::InvalidParameterValues(ref cause) => cause,
            AssociateIpGroupsError::InvalidResourceState(ref cause) => cause,
            AssociateIpGroupsError::OperationNotSupported(ref cause) => cause,
            AssociateIpGroupsError::ResourceLimitExceeded(ref cause) => cause,
            AssociateIpGroupsError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by AuthorizeIpRules
#[derive(Debug, PartialEq)]
pub enum AuthorizeIpRulesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl AuthorizeIpRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AuthorizeIpRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AuthorizeIpRulesError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(AuthorizeIpRulesError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(AuthorizeIpRulesError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(AuthorizeIpRulesError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AuthorizeIpRulesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AuthorizeIpRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AuthorizeIpRulesError {
    fn description(&self) -> &str {
        match *self {
            AuthorizeIpRulesError::AccessDenied(ref cause) => cause,
            AuthorizeIpRulesError::InvalidParameterValues(ref cause) => cause,
            AuthorizeIpRulesError::InvalidResourceState(ref cause) => cause,
            AuthorizeIpRulesError::ResourceLimitExceeded(ref cause) => cause,
            AuthorizeIpRulesError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by CopyWorkspaceImage
#[derive(Debug, PartialEq)]
pub enum CopyWorkspaceImageError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>This operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available.</p>
    ResourceUnavailable(String),
}

impl CopyWorkspaceImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopyWorkspaceImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(CopyWorkspaceImageError::ResourceUnavailable(
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
impl fmt::Display for CopyWorkspaceImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopyWorkspaceImageError {
    fn description(&self) -> &str {
        match *self {
            CopyWorkspaceImageError::AccessDenied(ref cause) => cause,
            CopyWorkspaceImageError::InvalidParameterValues(ref cause) => cause,
            CopyWorkspaceImageError::OperationNotSupported(ref cause) => cause,
            CopyWorkspaceImageError::ResourceAlreadyExists(ref cause) => cause,
            CopyWorkspaceImageError::ResourceLimitExceeded(ref cause) => cause,
            CopyWorkspaceImageError::ResourceNotFound(ref cause) => cause,
            CopyWorkspaceImageError::ResourceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateIpGroup
#[derive(Debug, PartialEq)]
pub enum CreateIpGroupError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The resource could not be created.</p>
    ResourceCreationFailed(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
}

impl CreateIpGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIpGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateIpGroupError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(CreateIpGroupError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateIpGroupError::ResourceAlreadyExists(err.msg))
                }
                "ResourceCreationFailedException" => {
                    return RusotoError::Service(CreateIpGroupError::ResourceCreationFailed(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateIpGroupError::ResourceLimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateIpGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateIpGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateIpGroupError::AccessDenied(ref cause) => cause,
            CreateIpGroupError::InvalidParameterValues(ref cause) => cause,
            CreateIpGroupError::ResourceAlreadyExists(ref cause) => cause,
            CreateIpGroupError::ResourceCreationFailed(ref cause) => cause,
            CreateIpGroupError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTags
#[derive(Debug, PartialEq)]
pub enum CreateTagsError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl CreateTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(CreateTagsError::InvalidParameterValues(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateTagsError::ResourceLimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateTagsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTagsError {
    fn description(&self) -> &str {
        match *self {
            CreateTagsError::InvalidParameterValues(ref cause) => cause,
            CreateTagsError::ResourceLimitExceeded(ref cause) => cause,
            CreateTagsError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateWorkspaces
#[derive(Debug, PartialEq)]
pub enum CreateWorkspacesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
}

impl CreateWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(CreateWorkspacesError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateWorkspacesError::ResourceLimitExceeded(
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
impl fmt::Display for CreateWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateWorkspacesError {
    fn description(&self) -> &str {
        match *self {
            CreateWorkspacesError::InvalidParameterValues(ref cause) => cause,
            CreateWorkspacesError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIpGroup
#[derive(Debug, PartialEq)]
pub enum DeleteIpGroupError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource is associated with a directory.</p>
    ResourceAssociated(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl DeleteIpGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIpGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteIpGroupError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(DeleteIpGroupError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "ResourceAssociatedException" => {
                    return RusotoError::Service(DeleteIpGroupError::ResourceAssociated(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteIpGroupError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteIpGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIpGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteIpGroupError::AccessDenied(ref cause) => cause,
            DeleteIpGroupError::InvalidParameterValues(ref cause) => cause,
            DeleteIpGroupError::ResourceAssociated(ref cause) => cause,
            DeleteIpGroupError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl DeleteTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(DeleteTagsError::InvalidParameterValues(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTagsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagsError {
    fn description(&self) -> &str {
        match *self {
            DeleteTagsError::InvalidParameterValues(ref cause) => cause,
            DeleteTagsError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteWorkspaceImage
#[derive(Debug, PartialEq)]
pub enum DeleteWorkspaceImageError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The resource is associated with a directory.</p>
    ResourceAssociated(String),
}

impl DeleteWorkspaceImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWorkspaceImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteWorkspaceImageError::AccessDenied(err.msg))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(DeleteWorkspaceImageError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "ResourceAssociatedException" => {
                    return RusotoError::Service(DeleteWorkspaceImageError::ResourceAssociated(
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
impl fmt::Display for DeleteWorkspaceImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteWorkspaceImageError {
    fn description(&self) -> &str {
        match *self {
            DeleteWorkspaceImageError::AccessDenied(ref cause) => cause,
            DeleteWorkspaceImageError::InvalidResourceState(ref cause) => cause,
            DeleteWorkspaceImageError::ResourceAssociated(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAccount
#[derive(Debug, PartialEq)]
pub enum DescribeAccountError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
}

impl DescribeAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeAccountError::AccessDenied(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAccountError {
    fn description(&self) -> &str {
        match *self {
            DescribeAccountError::AccessDenied(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAccountModifications
#[derive(Debug, PartialEq)]
pub enum DescribeAccountModificationsError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
}

impl DescribeAccountModificationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAccountModificationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeAccountModificationsError::AccessDenied(
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
impl fmt::Display for DescribeAccountModificationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAccountModificationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAccountModificationsError::AccessDenied(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClientProperties
#[derive(Debug, PartialEq)]
pub enum DescribeClientPropertiesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl DescribeClientPropertiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClientPropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeClientPropertiesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        DescribeClientPropertiesError::InvalidParameterValues(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeClientPropertiesError::ResourceNotFound(
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
impl fmt::Display for DescribeClientPropertiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClientPropertiesError {
    fn description(&self) -> &str {
        match *self {
            DescribeClientPropertiesError::AccessDenied(ref cause) => cause,
            DescribeClientPropertiesError::InvalidParameterValues(ref cause) => cause,
            DescribeClientPropertiesError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeIpGroups
#[derive(Debug, PartialEq)]
pub enum DescribeIpGroupsError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
}

impl DescribeIpGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeIpGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeIpGroupsError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(DescribeIpGroupsError::InvalidParameterValues(
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
impl fmt::Display for DescribeIpGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeIpGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeIpGroupsError::AccessDenied(ref cause) => cause,
            DescribeIpGroupsError::InvalidParameterValues(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl DescribeTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTagsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTagsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTagsError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkspaceBundles
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspaceBundlesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
}

impl DescribeWorkspaceBundlesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorkspaceBundlesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        DescribeWorkspaceBundlesError::InvalidParameterValues(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeWorkspaceBundlesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkspaceBundlesError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkspaceBundlesError::InvalidParameterValues(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkspaceDirectories
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspaceDirectoriesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
}

impl DescribeWorkspaceDirectoriesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeWorkspaceDirectoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        DescribeWorkspaceDirectoriesError::InvalidParameterValues(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeWorkspaceDirectoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkspaceDirectoriesError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkspaceDirectoriesError::InvalidParameterValues(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkspaceImages
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspaceImagesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
}

impl DescribeWorkspaceImagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorkspaceImagesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeWorkspaceImagesError::AccessDenied(
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
impl fmt::Display for DescribeWorkspaceImagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkspaceImagesError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkspaceImagesError::AccessDenied(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkspaces
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspacesError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The specified resource is not available.</p>
    ResourceUnavailable(String),
}

impl DescribeWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(DescribeWorkspacesError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DescribeWorkspacesError::ResourceUnavailable(
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
impl fmt::Display for DescribeWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkspacesError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkspacesError::InvalidParameterValues(ref cause) => cause,
            DescribeWorkspacesError::ResourceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkspacesConnectionStatus
#[derive(Debug, PartialEq)]
pub enum DescribeWorkspacesConnectionStatusError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
}

impl DescribeWorkspacesConnectionStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeWorkspacesConnectionStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        DescribeWorkspacesConnectionStatusError::InvalidParameterValues(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeWorkspacesConnectionStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkspacesConnectionStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkspacesConnectionStatusError::InvalidParameterValues(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateIpGroups
#[derive(Debug, PartialEq)]
pub enum DisassociateIpGroupsError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl DisassociateIpGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateIpGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisassociateIpGroupsError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(DisassociateIpGroupsError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(DisassociateIpGroupsError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateIpGroupsError::ResourceNotFound(
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
impl fmt::Display for DisassociateIpGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateIpGroupsError {
    fn description(&self) -> &str {
        match *self {
            DisassociateIpGroupsError::AccessDenied(ref cause) => cause,
            DisassociateIpGroupsError::InvalidParameterValues(ref cause) => cause,
            DisassociateIpGroupsError::InvalidResourceState(ref cause) => cause,
            DisassociateIpGroupsError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportWorkspaceImage
#[derive(Debug, PartialEq)]
pub enum ImportWorkspaceImageError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>This operation is not supported.</p>
    OperationNotSupported(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl ImportWorkspaceImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportWorkspaceImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::OperationNotSupported(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ImportWorkspaceImageError::ResourceNotFound(
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
impl fmt::Display for ImportWorkspaceImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportWorkspaceImageError {
    fn description(&self) -> &str {
        match *self {
            ImportWorkspaceImageError::AccessDenied(ref cause) => cause,
            ImportWorkspaceImageError::InvalidParameterValues(ref cause) => cause,
            ImportWorkspaceImageError::OperationNotSupported(ref cause) => cause,
            ImportWorkspaceImageError::ResourceAlreadyExists(ref cause) => cause,
            ImportWorkspaceImageError::ResourceLimitExceeded(ref cause) => cause,
            ImportWorkspaceImageError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAvailableManagementCidrRanges
#[derive(Debug, PartialEq)]
pub enum ListAvailableManagementCidrRangesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
}

impl ListAvailableManagementCidrRangesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAvailableManagementCidrRangesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListAvailableManagementCidrRangesError::AccessDenied(err.msg),
                    )
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        ListAvailableManagementCidrRangesError::InvalidParameterValues(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAvailableManagementCidrRangesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAvailableManagementCidrRangesError {
    fn description(&self) -> &str {
        match *self {
            ListAvailableManagementCidrRangesError::AccessDenied(ref cause) => cause,
            ListAvailableManagementCidrRangesError::InvalidParameterValues(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyAccount
#[derive(Debug, PartialEq)]
pub enum ModifyAccountError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available.</p>
    ResourceUnavailable(String),
}

impl ModifyAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ModifyAccountError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(ModifyAccountError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(ModifyAccountError::InvalidResourceState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ModifyAccountError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(ModifyAccountError::ResourceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ModifyAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyAccountError {
    fn description(&self) -> &str {
        match *self {
            ModifyAccountError::AccessDenied(ref cause) => cause,
            ModifyAccountError::InvalidParameterValues(ref cause) => cause,
            ModifyAccountError::InvalidResourceState(ref cause) => cause,
            ModifyAccountError::ResourceNotFound(ref cause) => cause,
            ModifyAccountError::ResourceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyClientProperties
#[derive(Debug, PartialEq)]
pub enum ModifyClientPropertiesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl ModifyClientPropertiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyClientPropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ModifyClientPropertiesError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        ModifyClientPropertiesError::InvalidParameterValues(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ModifyClientPropertiesError::ResourceNotFound(
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
impl fmt::Display for ModifyClientPropertiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyClientPropertiesError {
    fn description(&self) -> &str {
        match *self {
            ModifyClientPropertiesError::AccessDenied(ref cause) => cause,
            ModifyClientPropertiesError::InvalidParameterValues(ref cause) => cause,
            ModifyClientPropertiesError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyWorkspaceProperties
#[derive(Debug, PartialEq)]
pub enum ModifyWorkspacePropertiesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The properties of this WorkSpace are currently being modified. Try again in a moment.</p>
    OperationInProgress(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available.</p>
    ResourceUnavailable(String),
    /// <p>The configuration of this WorkSpace is not supported for this operation. For more information, see the <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/">Amazon WorkSpaces Administration Guide</a>. </p>
    UnsupportedWorkspaceConfiguration(String),
}

impl ModifyWorkspacePropertiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyWorkspacePropertiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ModifyWorkspacePropertiesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(
                        ModifyWorkspacePropertiesError::InvalidParameterValues(err.msg),
                    )
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(
                        ModifyWorkspacePropertiesError::InvalidResourceState(err.msg),
                    )
                }
                "OperationInProgressException" => {
                    return RusotoError::Service(
                        ModifyWorkspacePropertiesError::OperationInProgress(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ModifyWorkspacePropertiesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(
                        ModifyWorkspacePropertiesError::ResourceUnavailable(err.msg),
                    )
                }
                "UnsupportedWorkspaceConfigurationException" => {
                    return RusotoError::Service(
                        ModifyWorkspacePropertiesError::UnsupportedWorkspaceConfiguration(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ModifyWorkspacePropertiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyWorkspacePropertiesError {
    fn description(&self) -> &str {
        match *self {
            ModifyWorkspacePropertiesError::AccessDenied(ref cause) => cause,
            ModifyWorkspacePropertiesError::InvalidParameterValues(ref cause) => cause,
            ModifyWorkspacePropertiesError::InvalidResourceState(ref cause) => cause,
            ModifyWorkspacePropertiesError::OperationInProgress(ref cause) => cause,
            ModifyWorkspacePropertiesError::ResourceNotFound(ref cause) => cause,
            ModifyWorkspacePropertiesError::ResourceUnavailable(ref cause) => cause,
            ModifyWorkspacePropertiesError::UnsupportedWorkspaceConfiguration(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyWorkspaceState
#[derive(Debug, PartialEq)]
pub enum ModifyWorkspaceStateError {
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl ModifyWorkspaceStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyWorkspaceStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(ModifyWorkspaceStateError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(ModifyWorkspaceStateError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ModifyWorkspaceStateError::ResourceNotFound(
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
impl fmt::Display for ModifyWorkspaceStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyWorkspaceStateError {
    fn description(&self) -> &str {
        match *self {
            ModifyWorkspaceStateError::InvalidParameterValues(ref cause) => cause,
            ModifyWorkspaceStateError::InvalidResourceState(ref cause) => cause,
            ModifyWorkspaceStateError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootWorkspaces
#[derive(Debug, PartialEq)]
pub enum RebootWorkspacesError {}

impl RebootWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RebootWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootWorkspacesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by RebuildWorkspaces
#[derive(Debug, PartialEq)]
pub enum RebuildWorkspacesError {}

impl RebuildWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebuildWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RebuildWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebuildWorkspacesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by RevokeIpRules
#[derive(Debug, PartialEq)]
pub enum RevokeIpRulesError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl RevokeIpRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RevokeIpRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RevokeIpRulesError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(RevokeIpRulesError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(RevokeIpRulesError::InvalidResourceState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RevokeIpRulesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RevokeIpRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RevokeIpRulesError {
    fn description(&self) -> &str {
        match *self {
            RevokeIpRulesError::AccessDenied(ref cause) => cause,
            RevokeIpRulesError::InvalidParameterValues(ref cause) => cause,
            RevokeIpRulesError::InvalidResourceState(ref cause) => cause,
            RevokeIpRulesError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by StartWorkspaces
#[derive(Debug, PartialEq)]
pub enum StartWorkspacesError {}

impl StartWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartWorkspacesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by StopWorkspaces
#[derive(Debug, PartialEq)]
pub enum StopWorkspacesError {}

impl StopWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopWorkspacesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by TerminateWorkspaces
#[derive(Debug, PartialEq)]
pub enum TerminateWorkspacesError {}

impl TerminateWorkspacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TerminateWorkspacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TerminateWorkspacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateWorkspacesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by UpdateRulesOfIpGroup
#[derive(Debug, PartialEq)]
pub enum UpdateRulesOfIpGroupError {
    /// <p>The user is not authorized to access a resource.</p>
    AccessDenied(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValues(String),
    /// <p>The state of the resource is not valid for this operation.</p>
    InvalidResourceState(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFound(String),
}

impl UpdateRulesOfIpGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRulesOfIpGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateRulesOfIpGroupError::AccessDenied(err.msg))
                }
                "InvalidParameterValuesException" => {
                    return RusotoError::Service(UpdateRulesOfIpGroupError::InvalidParameterValues(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(UpdateRulesOfIpGroupError::InvalidResourceState(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(UpdateRulesOfIpGroupError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateRulesOfIpGroupError::ResourceNotFound(
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
impl fmt::Display for UpdateRulesOfIpGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRulesOfIpGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateRulesOfIpGroupError::AccessDenied(ref cause) => cause,
            UpdateRulesOfIpGroupError::InvalidParameterValues(ref cause) => cause,
            UpdateRulesOfIpGroupError::InvalidResourceState(ref cause) => cause,
            UpdateRulesOfIpGroupError::ResourceLimitExceeded(ref cause) => cause,
            UpdateRulesOfIpGroupError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon WorkSpaces API. Amazon WorkSpaces clients implement this trait.
pub trait Workspaces {
    /// <p>Associates the specified IP access control group with the specified directory.</p>
    fn associate_ip_groups(
        &self,
        input: AssociateIpGroupsRequest,
    ) -> RusotoFuture<AssociateIpGroupsResult, AssociateIpGroupsError>;

    /// <p>Adds one or more rules to the specified IP access control group.</p> <p>This action gives users permission to access their WorkSpaces from the CIDR address ranges specified in the rules.</p>
    fn authorize_ip_rules(
        &self,
        input: AuthorizeIpRulesRequest,
    ) -> RusotoFuture<AuthorizeIpRulesResult, AuthorizeIpRulesError>;

    /// <p>Copies the specified image from the specified Region to the current Region.</p>
    fn copy_workspace_image(
        &self,
        input: CopyWorkspaceImageRequest,
    ) -> RusotoFuture<CopyWorkspaceImageResult, CopyWorkspaceImageError>;

    /// <p>Creates an IP access control group.</p> <p>An IP access control group provides you with the ability to control the IP addresses from which users are allowed to access their WorkSpaces. To specify the CIDR address ranges, add rules to your IP access control group and then associate the group with your directory. You can add rules when you create the group or at any time using <a>AuthorizeIpRules</a>.</p> <p>There is a default IP access control group associated with your directory. If you don't associate an IP access control group with your directory, the default group is used. The default group includes a default rule that allows users to access their WorkSpaces from anywhere. You cannot modify the default IP access control group for your directory.</p>
    fn create_ip_group(
        &self,
        input: CreateIpGroupRequest,
    ) -> RusotoFuture<CreateIpGroupResult, CreateIpGroupError>;

    /// <p>Creates the specified tags for the specified WorkSpaces resource.</p>
    fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> RusotoFuture<CreateTagsResult, CreateTagsError>;

    /// <p>Creates one or more WorkSpaces.</p> <p>This operation is asynchronous and returns before the WorkSpaces are created.</p>
    fn create_workspaces(
        &self,
        input: CreateWorkspacesRequest,
    ) -> RusotoFuture<CreateWorkspacesResult, CreateWorkspacesError>;

    /// <p>Deletes the specified IP access control group.</p> <p>You cannot delete an IP access control group that is associated with a directory.</p>
    fn delete_ip_group(
        &self,
        input: DeleteIpGroupRequest,
    ) -> RusotoFuture<DeleteIpGroupResult, DeleteIpGroupError>;

    /// <p>Deletes the specified tags from the specified WorkSpaces resource.</p>
    fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> RusotoFuture<DeleteTagsResult, DeleteTagsError>;

    /// <p>Deletes the specified image from your account. To delete an image, you must first delete any bundles that are associated with the image and un-share the image if it is shared with other accounts. </p>
    fn delete_workspace_image(
        &self,
        input: DeleteWorkspaceImageRequest,
    ) -> RusotoFuture<DeleteWorkspaceImageResult, DeleteWorkspaceImageError>;

    /// <p>Retrieves a list that describes the configuration of bring your own license (BYOL) for the specified account.</p>
    fn describe_account(&self) -> RusotoFuture<DescribeAccountResult, DescribeAccountError>;

    /// <p>Retrieves a list that describes modifications to the configuration of bring your own license (BYOL) for the specified account.</p>
    fn describe_account_modifications(
        &self,
        input: DescribeAccountModificationsRequest,
    ) -> RusotoFuture<DescribeAccountModificationsResult, DescribeAccountModificationsError>;

    /// <p>Retrieves a list that describes one or more specified Amazon WorkSpaces clients.</p>
    fn describe_client_properties(
        &self,
        input: DescribeClientPropertiesRequest,
    ) -> RusotoFuture<DescribeClientPropertiesResult, DescribeClientPropertiesError>;

    /// <p>Describes one or more of your IP access control groups.</p>
    fn describe_ip_groups(
        &self,
        input: DescribeIpGroupsRequest,
    ) -> RusotoFuture<DescribeIpGroupsResult, DescribeIpGroupsError>;

    /// <p>Describes the specified tags for the specified WorkSpaces resource.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResult, DescribeTagsError>;

    /// <p>Retrieves a list that describes the available WorkSpace bundles.</p> <p>You can filter the results using either bundle ID or owner, but not both.</p>
    fn describe_workspace_bundles(
        &self,
        input: DescribeWorkspaceBundlesRequest,
    ) -> RusotoFuture<DescribeWorkspaceBundlesResult, DescribeWorkspaceBundlesError>;

    /// <p>Describes the available AWS Directory Service directories that are registered with Amazon WorkSpaces.</p>
    fn describe_workspace_directories(
        &self,
        input: DescribeWorkspaceDirectoriesRequest,
    ) -> RusotoFuture<DescribeWorkspaceDirectoriesResult, DescribeWorkspaceDirectoriesError>;

    /// <p>Retrieves a list that describes one or more specified images, if the image identifiers are provided. Otherwise, all images in the account are described. </p>
    fn describe_workspace_images(
        &self,
        input: DescribeWorkspaceImagesRequest,
    ) -> RusotoFuture<DescribeWorkspaceImagesResult, DescribeWorkspaceImagesError>;

    /// <p>Describes the specified WorkSpaces.</p> <p>You can filter the results by using the bundle identifier, directory identifier, or owner, but you can specify only one filter at a time.</p>
    fn describe_workspaces(
        &self,
        input: DescribeWorkspacesRequest,
    ) -> RusotoFuture<DescribeWorkspacesResult, DescribeWorkspacesError>;

    /// <p>Describes the connection status of the specified WorkSpaces.</p>
    fn describe_workspaces_connection_status(
        &self,
        input: DescribeWorkspacesConnectionStatusRequest,
    ) -> RusotoFuture<
        DescribeWorkspacesConnectionStatusResult,
        DescribeWorkspacesConnectionStatusError,
    >;

    /// <p>Disassociates the specified IP access control group from the specified directory.</p>
    fn disassociate_ip_groups(
        &self,
        input: DisassociateIpGroupsRequest,
    ) -> RusotoFuture<DisassociateIpGroupsResult, DisassociateIpGroupsError>;

    /// <p>Imports the specified Windows 7 or Windows 10 bring your own license (BYOL) image into Amazon WorkSpaces. The image must be an already licensed EC2 image that is in your AWS account, and you must own the image. </p>
    fn import_workspace_image(
        &self,
        input: ImportWorkspaceImageRequest,
    ) -> RusotoFuture<ImportWorkspaceImageResult, ImportWorkspaceImageError>;

    /// <p>Retrieves a list of IP address ranges, specified as IPv4 CIDR blocks, that you can use for the network management interface when you enable bring your own license (BYOL). </p> <p>The management network interface is connected to a secure Amazon WorkSpaces management network. It is used for interactive streaming of the WorkSpace desktop to Amazon WorkSpaces clients, and to allow Amazon WorkSpaces to manage the WorkSpace.</p>
    fn list_available_management_cidr_ranges(
        &self,
        input: ListAvailableManagementCidrRangesRequest,
    ) -> RusotoFuture<ListAvailableManagementCidrRangesResult, ListAvailableManagementCidrRangesError>;

    /// <p>Modifies the configuration of bring your own license (BYOL) for the specified account.</p>
    fn modify_account(
        &self,
        input: ModifyAccountRequest,
    ) -> RusotoFuture<ModifyAccountResult, ModifyAccountError>;

    /// <p>Modifies the properties of the specified Amazon WorkSpaces clients.</p>
    fn modify_client_properties(
        &self,
        input: ModifyClientPropertiesRequest,
    ) -> RusotoFuture<ModifyClientPropertiesResult, ModifyClientPropertiesError>;

    /// <p>Modifies the specified WorkSpace properties.</p>
    fn modify_workspace_properties(
        &self,
        input: ModifyWorkspacePropertiesRequest,
    ) -> RusotoFuture<ModifyWorkspacePropertiesResult, ModifyWorkspacePropertiesError>;

    /// <p>Sets the state of the specified WorkSpace.</p> <p>To maintain a WorkSpace without being interrupted, set the WorkSpace state to <code>ADMIN_MAINTENANCE</code>. WorkSpaces in this state do not respond to requests to reboot, stop, start, rebuild, or restore. An AutoStop WorkSpace in this state is not stopped. Users cannot log into a WorkSpace in the <code>ADMIN_MAINTENANCE</code> state.</p>
    fn modify_workspace_state(
        &self,
        input: ModifyWorkspaceStateRequest,
    ) -> RusotoFuture<ModifyWorkspaceStateResult, ModifyWorkspaceStateError>;

    /// <p>Reboots the specified WorkSpaces.</p> <p>You cannot reboot a WorkSpace unless its state is <code>AVAILABLE</code> or <code>UNHEALTHY</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have rebooted.</p>
    fn reboot_workspaces(
        &self,
        input: RebootWorkspacesRequest,
    ) -> RusotoFuture<RebootWorkspacesResult, RebootWorkspacesError>;

    /// <p>Rebuilds the specified WorkSpace.</p> <p>You cannot rebuild a WorkSpace unless its state is <code>AVAILABLE</code>, <code>ERROR</code>, or <code>UNHEALTHY</code>.</p> <p>Rebuilding a WorkSpace is a potentially destructive action that can result in the loss of data. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/reset-workspace.html">Rebuild a WorkSpace</a>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely rebuilt.</p>
    fn rebuild_workspaces(
        &self,
        input: RebuildWorkspacesRequest,
    ) -> RusotoFuture<RebuildWorkspacesResult, RebuildWorkspacesError>;

    /// <p>Removes one or more rules from the specified IP access control group.</p>
    fn revoke_ip_rules(
        &self,
        input: RevokeIpRulesRequest,
    ) -> RusotoFuture<RevokeIpRulesResult, RevokeIpRulesError>;

    /// <p>Starts the specified WorkSpaces.</p> <p>You cannot start a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>STOPPED</code>.</p>
    fn start_workspaces(
        &self,
        input: StartWorkspacesRequest,
    ) -> RusotoFuture<StartWorkspacesResult, StartWorkspacesError>;

    /// <p> Stops the specified WorkSpaces.</p> <p>You cannot stop a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>AVAILABLE</code>, <code>IMPAIRED</code>, <code>UNHEALTHY</code>, or <code>ERROR</code>.</p>
    fn stop_workspaces(
        &self,
        input: StopWorkspacesRequest,
    ) -> RusotoFuture<StopWorkspacesResult, StopWorkspacesError>;

    /// <p>Terminates the specified WorkSpaces.</p> <p>Terminating a WorkSpace is a permanent action and cannot be undone. The user's data is destroyed. If you need to archive any user data, contact Amazon Web Services before terminating the WorkSpace.</p> <p>You can terminate a WorkSpace that is in any state except <code>SUSPENDED</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely terminated.</p>
    fn terminate_workspaces(
        &self,
        input: TerminateWorkspacesRequest,
    ) -> RusotoFuture<TerminateWorkspacesResult, TerminateWorkspacesError>;

    /// <p>Replaces the current rules of the specified IP access control group with the specified rules.</p>
    fn update_rules_of_ip_group(
        &self,
        input: UpdateRulesOfIpGroupRequest,
    ) -> RusotoFuture<UpdateRulesOfIpGroupResult, UpdateRulesOfIpGroupError>;
}
/// A client for the Amazon WorkSpaces API.
#[derive(Clone)]
pub struct WorkspacesClient {
    client: Client,
    region: region::Region,
}

impl WorkspacesClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> WorkspacesClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> WorkspacesClient
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

    pub fn new_with_client(client: Client, region: region::Region) -> WorkspacesClient {
        WorkspacesClient { client, region }
    }
}

impl fmt::Debug for WorkspacesClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorkspacesClient")
            .field("region", &self.region)
            .finish()
    }
}

impl Workspaces for WorkspacesClient {
    /// <p>Associates the specified IP access control group with the specified directory.</p>
    fn associate_ip_groups(
        &self,
        input: AssociateIpGroupsRequest,
    ) -> RusotoFuture<AssociateIpGroupsResult, AssociateIpGroupsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.AssociateIpGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateIpGroupsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AssociateIpGroupsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds one or more rules to the specified IP access control group.</p> <p>This action gives users permission to access their WorkSpaces from the CIDR address ranges specified in the rules.</p>
    fn authorize_ip_rules(
        &self,
        input: AuthorizeIpRulesRequest,
    ) -> RusotoFuture<AuthorizeIpRulesResult, AuthorizeIpRulesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.AuthorizeIpRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AuthorizeIpRulesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AuthorizeIpRulesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Copies the specified image from the specified Region to the current Region.</p>
    fn copy_workspace_image(
        &self,
        input: CopyWorkspaceImageRequest,
    ) -> RusotoFuture<CopyWorkspaceImageResult, CopyWorkspaceImageError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CopyWorkspaceImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CopyWorkspaceImageResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CopyWorkspaceImageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an IP access control group.</p> <p>An IP access control group provides you with the ability to control the IP addresses from which users are allowed to access their WorkSpaces. To specify the CIDR address ranges, add rules to your IP access control group and then associate the group with your directory. You can add rules when you create the group or at any time using <a>AuthorizeIpRules</a>.</p> <p>There is a default IP access control group associated with your directory. If you don't associate an IP access control group with your directory, the default group is used. The default group includes a default rule that allows users to access their WorkSpaces from anywhere. You cannot modify the default IP access control group for your directory.</p>
    fn create_ip_group(
        &self,
        input: CreateIpGroupRequest,
    ) -> RusotoFuture<CreateIpGroupResult, CreateIpGroupError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CreateIpGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateIpGroupResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateIpGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates the specified tags for the specified WorkSpaces resource.</p>
    fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> RusotoFuture<CreateTagsResult, CreateTagsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CreateTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateTagsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates one or more WorkSpaces.</p> <p>This operation is asynchronous and returns before the WorkSpaces are created.</p>
    fn create_workspaces(
        &self,
        input: CreateWorkspacesRequest,
    ) -> RusotoFuture<CreateWorkspacesResult, CreateWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.CreateWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateWorkspacesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateWorkspacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified IP access control group.</p> <p>You cannot delete an IP access control group that is associated with a directory.</p>
    fn delete_ip_group(
        &self,
        input: DeleteIpGroupRequest,
    ) -> RusotoFuture<DeleteIpGroupResult, DeleteIpGroupError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DeleteIpGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteIpGroupResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteIpGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified tags from the specified WorkSpaces resource.</p>
    fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> RusotoFuture<DeleteTagsResult, DeleteTagsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DeleteTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteTagsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified image from your account. To delete an image, you must first delete any bundles that are associated with the image and un-share the image if it is shared with other accounts. </p>
    fn delete_workspace_image(
        &self,
        input: DeleteWorkspaceImageRequest,
    ) -> RusotoFuture<DeleteWorkspaceImageResult, DeleteWorkspaceImageError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DeleteWorkspaceImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteWorkspaceImageResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteWorkspaceImageError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves a list that describes the configuration of bring your own license (BYOL) for the specified account.</p>
    fn describe_account(&self) -> RusotoFuture<DescribeAccountResult, DescribeAccountError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeAccount");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAccountResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list that describes modifications to the configuration of bring your own license (BYOL) for the specified account.</p>
    fn describe_account_modifications(
        &self,
        input: DescribeAccountModificationsRequest,
    ) -> RusotoFuture<DescribeAccountModificationsResult, DescribeAccountModificationsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DescribeAccountModifications",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAccountModificationsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAccountModificationsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list that describes one or more specified Amazon WorkSpaces clients.</p>
    fn describe_client_properties(
        &self,
        input: DescribeClientPropertiesRequest,
    ) -> RusotoFuture<DescribeClientPropertiesResult, DescribeClientPropertiesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeClientProperties");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeClientPropertiesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeClientPropertiesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes one or more of your IP access control groups.</p>
    fn describe_ip_groups(
        &self,
        input: DescribeIpGroupsRequest,
    ) -> RusotoFuture<DescribeIpGroupsResult, DescribeIpGroupsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeIpGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeIpGroupsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeIpGroupsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the specified tags for the specified WorkSpaces resource.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResult, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeTagsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list that describes the available WorkSpace bundles.</p> <p>You can filter the results using either bundle ID or owner, but not both.</p>
    fn describe_workspace_bundles(
        &self,
        input: DescribeWorkspaceBundlesRequest,
    ) -> RusotoFuture<DescribeWorkspaceBundlesResult, DescribeWorkspaceBundlesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeWorkspaceBundles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeWorkspaceBundlesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWorkspaceBundlesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the available AWS Directory Service directories that are registered with Amazon WorkSpaces.</p>
    fn describe_workspace_directories(
        &self,
        input: DescribeWorkspaceDirectoriesRequest,
    ) -> RusotoFuture<DescribeWorkspaceDirectoriesResult, DescribeWorkspaceDirectoriesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DescribeWorkspaceDirectories",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeWorkspaceDirectoriesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWorkspaceDirectoriesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list that describes one or more specified images, if the image identifiers are provided. Otherwise, all images in the account are described. </p>
    fn describe_workspace_images(
        &self,
        input: DescribeWorkspaceImagesRequest,
    ) -> RusotoFuture<DescribeWorkspaceImagesResult, DescribeWorkspaceImagesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeWorkspaceImages");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeWorkspaceImagesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWorkspaceImagesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the specified WorkSpaces.</p> <p>You can filter the results by using the bundle identifier, directory identifier, or owner, but you can specify only one filter at a time.</p>
    fn describe_workspaces(
        &self,
        input: DescribeWorkspacesRequest,
    ) -> RusotoFuture<DescribeWorkspacesResult, DescribeWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DescribeWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeWorkspacesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeWorkspacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the connection status of the specified WorkSpaces.</p>
    fn describe_workspaces_connection_status(
        &self,
        input: DescribeWorkspacesConnectionStatusRequest,
    ) -> RusotoFuture<
        DescribeWorkspacesConnectionStatusResult,
        DescribeWorkspacesConnectionStatusError,
    > {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.DescribeWorkspacesConnectionStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeWorkspacesConnectionStatusResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWorkspacesConnectionStatusError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Disassociates the specified IP access control group from the specified directory.</p>
    fn disassociate_ip_groups(
        &self,
        input: DisassociateIpGroupsRequest,
    ) -> RusotoFuture<DisassociateIpGroupsResult, DisassociateIpGroupsError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.DisassociateIpGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateIpGroupsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DisassociateIpGroupsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Imports the specified Windows 7 or Windows 10 bring your own license (BYOL) image into Amazon WorkSpaces. The image must be an already licensed EC2 image that is in your AWS account, and you must own the image. </p>
    fn import_workspace_image(
        &self,
        input: ImportWorkspaceImageRequest,
    ) -> RusotoFuture<ImportWorkspaceImageResult, ImportWorkspaceImageError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.ImportWorkspaceImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ImportWorkspaceImageResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ImportWorkspaceImageError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves a list of IP address ranges, specified as IPv4 CIDR blocks, that you can use for the network management interface when you enable bring your own license (BYOL). </p> <p>The management network interface is connected to a secure Amazon WorkSpaces management network. It is used for interactive streaming of the WorkSpace desktop to Amazon WorkSpaces clients, and to allow Amazon WorkSpaces to manage the WorkSpace.</p>
    fn list_available_management_cidr_ranges(
        &self,
        input: ListAvailableManagementCidrRangesRequest,
    ) -> RusotoFuture<ListAvailableManagementCidrRangesResult, ListAvailableManagementCidrRangesError>
    {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.ListAvailableManagementCidrRanges",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAvailableManagementCidrRangesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListAvailableManagementCidrRangesError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Modifies the configuration of bring your own license (BYOL) for the specified account.</p>
    fn modify_account(
        &self,
        input: ModifyAccountRequest,
    ) -> RusotoFuture<ModifyAccountResult, ModifyAccountError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.ModifyAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ModifyAccountResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ModifyAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modifies the properties of the specified Amazon WorkSpaces clients.</p>
    fn modify_client_properties(
        &self,
        input: ModifyClientPropertiesRequest,
    ) -> RusotoFuture<ModifyClientPropertiesResult, ModifyClientPropertiesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.ModifyClientProperties");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ModifyClientPropertiesResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ModifyClientPropertiesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Modifies the specified WorkSpace properties.</p>
    fn modify_workspace_properties(
        &self,
        input: ModifyWorkspacePropertiesRequest,
    ) -> RusotoFuture<ModifyWorkspacePropertiesResult, ModifyWorkspacePropertiesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "WorkspacesService.ModifyWorkspaceProperties",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ModifyWorkspacePropertiesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyWorkspacePropertiesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Sets the state of the specified WorkSpace.</p> <p>To maintain a WorkSpace without being interrupted, set the WorkSpace state to <code>ADMIN_MAINTENANCE</code>. WorkSpaces in this state do not respond to requests to reboot, stop, start, rebuild, or restore. An AutoStop WorkSpace in this state is not stopped. Users cannot log into a WorkSpace in the <code>ADMIN_MAINTENANCE</code> state.</p>
    fn modify_workspace_state(
        &self,
        input: ModifyWorkspaceStateRequest,
    ) -> RusotoFuture<ModifyWorkspaceStateResult, ModifyWorkspaceStateError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.ModifyWorkspaceState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ModifyWorkspaceStateResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ModifyWorkspaceStateError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Reboots the specified WorkSpaces.</p> <p>You cannot reboot a WorkSpace unless its state is <code>AVAILABLE</code> or <code>UNHEALTHY</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have rebooted.</p>
    fn reboot_workspaces(
        &self,
        input: RebootWorkspacesRequest,
    ) -> RusotoFuture<RebootWorkspacesResult, RebootWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RebootWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RebootWorkspacesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RebootWorkspacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Rebuilds the specified WorkSpace.</p> <p>You cannot rebuild a WorkSpace unless its state is <code>AVAILABLE</code>, <code>ERROR</code>, or <code>UNHEALTHY</code>.</p> <p>Rebuilding a WorkSpace is a potentially destructive action that can result in the loss of data. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/reset-workspace.html">Rebuild a WorkSpace</a>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely rebuilt.</p>
    fn rebuild_workspaces(
        &self,
        input: RebuildWorkspacesRequest,
    ) -> RusotoFuture<RebuildWorkspacesResult, RebuildWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RebuildWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RebuildWorkspacesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RebuildWorkspacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes one or more rules from the specified IP access control group.</p>
    fn revoke_ip_rules(
        &self,
        input: RevokeIpRulesRequest,
    ) -> RusotoFuture<RevokeIpRulesResult, RevokeIpRulesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.RevokeIpRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RevokeIpRulesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RevokeIpRulesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts the specified WorkSpaces.</p> <p>You cannot start a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>STOPPED</code>.</p>
    fn start_workspaces(
        &self,
        input: StartWorkspacesRequest,
    ) -> RusotoFuture<StartWorkspacesResult, StartWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.StartWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartWorkspacesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartWorkspacesError::from_response(response))),
                )
            }
        })
    }

    /// <p> Stops the specified WorkSpaces.</p> <p>You cannot stop a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>AVAILABLE</code>, <code>IMPAIRED</code>, <code>UNHEALTHY</code>, or <code>ERROR</code>.</p>
    fn stop_workspaces(
        &self,
        input: StopWorkspacesRequest,
    ) -> RusotoFuture<StopWorkspacesResult, StopWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.StopWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopWorkspacesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopWorkspacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Terminates the specified WorkSpaces.</p> <p>Terminating a WorkSpace is a permanent action and cannot be undone. The user's data is destroyed. If you need to archive any user data, contact Amazon Web Services before terminating the WorkSpace.</p> <p>You can terminate a WorkSpace that is in any state except <code>SUSPENDED</code>.</p> <p>This operation is asynchronous and returns before the WorkSpaces have been completely terminated.</p>
    fn terminate_workspaces(
        &self,
        input: TerminateWorkspacesRequest,
    ) -> RusotoFuture<TerminateWorkspacesResult, TerminateWorkspacesError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.TerminateWorkspaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TerminateWorkspacesResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(TerminateWorkspacesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Replaces the current rules of the specified IP access control group with the specified rules.</p>
    fn update_rules_of_ip_group(
        &self,
        input: UpdateRulesOfIpGroupRequest,
    ) -> RusotoFuture<UpdateRulesOfIpGroupResult, UpdateRulesOfIpGroupError> {
        let mut request = SignedRequest::new("POST", "workspaces", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "WorkspacesService.UpdateRulesOfIpGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateRulesOfIpGroupResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateRulesOfIpGroupError::from_response(response))
                    }),
                )
            }
        })
    }
}
