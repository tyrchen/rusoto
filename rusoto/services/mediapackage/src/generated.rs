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
/// <p>A Channel resource configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Channel {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A Common Media Application Format (CMAF) encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmafEncryption {
    /// <p>Time (in seconds) between each encryption key rotation.</p>
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i64>,
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A Common Media Application Format (CMAF) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CmafPackage {
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CmafEncryption>,
    /// <p>A list of HLS manifest configurations</p>
    #[serde(rename = "HlsManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<HlsManifest>>,
    /// <p>Duration (in seconds) of each segment. Actual segments will be
    /// rounded to the nearest multiple of the source segment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    /// <p>An optional custom string that is prepended to the name of each segment. If not specified, it defaults to the ChannelId.</p>
    #[serde(rename = "SegmentPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_prefix: Option<String>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

/// <p>A Common Media Application Format (CMAF) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CmafPackageCreateOrUpdateParameters {
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CmafEncryption>,
    /// <p>A list of HLS manifest configurations</p>
    #[serde(rename = "HlsManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<HlsManifestCreateOrUpdateParameters>>,
    /// <p>Duration (in seconds) of each segment. Actual segments will be
    /// rounded to the nearest multiple of the source segment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    /// <p>An optional custom string that is prepended to the name of each segment. If not specified, it defaults to the ChannelId.</p>
    #[serde(rename = "SegmentPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_prefix: Option<String>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

/// <p>A new Channel configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateChannelRequest {
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the Channel. The ID must be unique within the region and it
    /// cannot be changed after a Channel is created.</p>
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateChannelResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Configuration parameters used to create a new OriginEndpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateOriginEndpointRequest {
    /// <p>The ID of the Channel that the OriginEndpoint will be associated with.
    /// This cannot be changed after the OriginEndpoint is created.</p>
    #[serde(rename = "ChannelId")]
    pub channel_id: String,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackageCreateOrUpdateParameters>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint.  The ID must be unique within the region
    /// and it cannot be changed after the OriginEndpoint is created.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A short string that will be used as the filename of the OriginEndpoint URL (defaults to &quot;index&quot;).</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Amount of delay (seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateOriginEndpointResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the Channel the OriginEndpoint is associated with.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A short string appended to the end of the OriginEndpoint URL.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Amount of delay (seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>The URL of the packaged OriginEndpoint for consumption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

/// <p>A Dynamic Adaptive Streaming over HTTP (DASH) encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashEncryption {
    /// <p>Time (in seconds) between each encryption key rotation.</p>
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i64>,
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A Dynamic Adaptive Streaming over HTTP (DASH) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashPackage {
    #[serde(rename = "AdTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<String>>,
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<String>,
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<DashEncryption>,
    /// <p>Determines the position of some tags in the Media Presentation Description (MPD).  When set to FULL, elements like SegmentTemplate and ContentProtection are included in each Representation.  When set to COMPACT, duplicate elements are combined and presented at the AdaptationSet level.</p>
    #[serde(rename = "ManifestLayout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_layout: Option<String>,
    /// <p>Time window (in seconds) contained in each manifest.</p>
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i64>,
    /// <p>Minimum duration (in seconds) that a player will buffer media before starting the presentation.</p>
    #[serde(rename = "MinBufferTimeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time_seconds: Option<i64>,
    /// <p>Minimum duration (in seconds) between potential changes to the Dynamic Adaptive Streaming over HTTP (DASH) Media Presentation Description (MPD).</p>
    #[serde(rename = "MinUpdatePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_update_period_seconds: Option<i64>,
    /// <p>A list of triggers that controls when the outgoing Dynamic Adaptive Streaming over HTTP (DASH)
    /// Media Presentation Description (MPD) will be partitioned into multiple periods. If empty, the content will not
    /// be partitioned into more than one period. If the list contains &quot;ADS&quot;, new periods will be created where
    /// the Channel source contains SCTE-35 ad markers.</p>
    #[serde(rename = "PeriodTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_triggers: Option<Vec<String>>,
    /// <p>The Dynamic Adaptive Streaming over HTTP (DASH) profile type.  When set to &quot;HBBTV<em>1</em>5&quot;, HbbTV 1.5 compliant output is enabled.</p>
    #[serde(rename = "Profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// <p>Duration (in seconds) of each segment. Actual segments will be
    /// rounded to the nearest multiple of the source segment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    /// <p>Determines the type of SegmentTemplate included in the Media Presentation Description (MPD).  When set to NUMBER<em>WITH</em>TIMELINE, a full timeline is presented in each SegmentTemplate, with $Number$ media URLs.  When set to TIME<em>WITH</em>TIMELINE, a full timeline is presented in each SegmentTemplate, with $Time$ media URLs. When set to NUMBER<em>WITH</em>DURATION, only a duration is included in each SegmentTemplate, with $Number$ media URLs.</p>
    #[serde(rename = "SegmentTemplateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_template_format: Option<String>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
    /// <p>Duration (in seconds) to delay live content before presentation.</p>
    #[serde(rename = "SuggestedPresentationDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_presentation_delay_seconds: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteChannelRequest {
    /// <p>The ID of the Channel to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteChannelResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteOriginEndpointRequest {
    /// <p>The ID of the OriginEndpoint to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteOriginEndpointResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeChannelRequest {
    /// <p>The ID of a Channel.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeChannelResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeOriginEndpointRequest {
    /// <p>The ID of the OriginEndpoint.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOriginEndpointResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the Channel the OriginEndpoint is associated with.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A short string appended to the end of the OriginEndpoint URL.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Amount of delay (seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>The URL of the packaged OriginEndpoint for consumption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

/// <p>An HTTP Live Streaming (HLS) encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsEncryption {
    /// <p>A constant initialization vector for encryption (optional).
    /// When not specified the initialization vector will be periodically rotated.</p>
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    /// <p>The encryption method to use.</p>
    #[serde(rename = "EncryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<String>,
    /// <p>Interval (in seconds) between each encryption key rotation.</p>
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i64>,
    /// <p>When enabled, the EXT-X-KEY tag will be repeated in output manifests.</p>
    #[serde(rename = "RepeatExtXKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_ext_x_key: Option<bool>,
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>An HTTP Live Streaming (HLS) ingest resource configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HlsIngest {
    /// <p>A list of endpoints to which the source stream should be sent.</p>
    #[serde(rename = "IngestEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_endpoints: Option<Vec<IngestEndpoint>>,
}

/// <p>A HTTP Live Streaming (HLS) manifest configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HlsManifest {
    /// <p>This setting controls how ad markers are included in the packaged OriginEndpoint.
    /// &quot;NONE&quot; will omit all SCTE-35 ad markers from the output.
    /// &quot;PASSTHROUGH&quot; causes the manifest to contain a copy of the SCTE-35 ad
    /// markers (comments) taken directly from the input HTTP Live Streaming (HLS) manifest.
    /// &quot;SCTE35_ENHANCED&quot; generates ad markers and blackout tags based on SCTE-35
    /// messages in the input source.</p>
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<String>,
    /// <p>The ID of the manifest. The ID must be unique within the OriginEndpoint and it cannot be changed after it is created.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>When enabled, an I-Frame only stream will be included in the output.</p>
    #[serde(rename = "IncludeIframeOnlyStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,
    /// <p>An optional short string appended to the end of the OriginEndpoint URL. If not specified, defaults to the manifestName for the OriginEndpoint.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    /// <p>The HTTP Live Streaming (HLS) playlist type.
    /// When either &quot;EVENT&quot; or &quot;VOD&quot; is specified, a corresponding EXT-X-PLAYLIST-TYPE
    /// entry will be included in the media playlist.</p>
    #[serde(rename = "PlaylistType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<String>,
    /// <p>Time window (in seconds) contained in each parent manifest.</p>
    #[serde(rename = "PlaylistWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_window_seconds: Option<i64>,
    /// <p>The interval (in seconds) between each EXT-X-PROGRAM-DATE-TIME tag
    /// inserted into manifests. Additionally, when an interval is specified
    /// ID3Timed Metadata messages will be generated every 5 seconds using the
    /// ingest time of the content.
    /// If the interval is not specified, or set to 0, then
    /// no EXT-X-PROGRAM-DATE-TIME tags will be inserted into manifests and no
    /// ID3Timed Metadata messages will be generated. Note that irrespective
    /// of this parameter, if any ID3 Timed Metadata is found in HTTP Live Streaming (HLS) input,
    /// it will be passed through to HLS output.</p>
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i64>,
    /// <p>The URL of the packaged OriginEndpoint for consumption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>A HTTP Live Streaming (HLS) manifest configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct HlsManifestCreateOrUpdateParameters {
    /// <p>This setting controls how ad markers are included in the packaged OriginEndpoint.
    /// &quot;NONE&quot; will omit all SCTE-35 ad markers from the output.
    /// &quot;PASSTHROUGH&quot; causes the manifest to contain a copy of the SCTE-35 ad
    /// markers (comments) taken directly from the input HTTP Live Streaming (HLS) manifest.
    /// &quot;SCTE35_ENHANCED&quot; generates ad markers and blackout tags based on SCTE-35
    /// messages in the input source.</p>
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<String>,
    #[serde(rename = "AdTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<String>>,
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<String>,
    /// <p>The ID of the manifest. The ID must be unique within the OriginEndpoint and it cannot be changed after it is created.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>When enabled, an I-Frame only stream will be included in the output.</p>
    #[serde(rename = "IncludeIframeOnlyStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,
    /// <p>An optional short string appended to the end of the OriginEndpoint URL. If not specified, defaults to the manifestName for the OriginEndpoint.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    /// <p>The HTTP Live Streaming (HLS) playlist type.
    /// When either &quot;EVENT&quot; or &quot;VOD&quot; is specified, a corresponding EXT-X-PLAYLIST-TYPE
    /// entry will be included in the media playlist.</p>
    #[serde(rename = "PlaylistType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<String>,
    /// <p>Time window (in seconds) contained in each parent manifest.</p>
    #[serde(rename = "PlaylistWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_window_seconds: Option<i64>,
    /// <p>The interval (in seconds) between each EXT-X-PROGRAM-DATE-TIME tag
    /// inserted into manifests. Additionally, when an interval is specified
    /// ID3Timed Metadata messages will be generated every 5 seconds using the
    /// ingest time of the content.
    /// If the interval is not specified, or set to 0, then
    /// no EXT-X-PROGRAM-DATE-TIME tags will be inserted into manifests and no
    /// ID3Timed Metadata messages will be generated. Note that irrespective
    /// of this parameter, if any ID3 Timed Metadata is found in HTTP Live Streaming (HLS) input,
    /// it will be passed through to HLS output.</p>
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i64>,
}

/// <p>An HTTP Live Streaming (HLS) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsPackage {
    /// <p>This setting controls how ad markers are included in the packaged OriginEndpoint.
    /// &quot;NONE&quot; will omit all SCTE-35 ad markers from the output.
    /// &quot;PASSTHROUGH&quot; causes the manifest to contain a copy of the SCTE-35 ad
    /// markers (comments) taken directly from the input HTTP Live Streaming (HLS) manifest.
    /// &quot;SCTE35_ENHANCED&quot; generates ad markers and blackout tags based on SCTE-35
    /// messages in the input source.</p>
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<String>,
    #[serde(rename = "AdTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<String>>,
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<String>,
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<HlsEncryption>,
    /// <p>When enabled, an I-Frame only stream will be included in the output.</p>
    #[serde(rename = "IncludeIframeOnlyStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,
    /// <p>The HTTP Live Streaming (HLS) playlist type.
    /// When either &quot;EVENT&quot; or &quot;VOD&quot; is specified, a corresponding EXT-X-PLAYLIST-TYPE
    /// entry will be included in the media playlist.</p>
    #[serde(rename = "PlaylistType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<String>,
    /// <p>Time window (in seconds) contained in each parent manifest.</p>
    #[serde(rename = "PlaylistWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_window_seconds: Option<i64>,
    /// <p>The interval (in seconds) between each EXT-X-PROGRAM-DATE-TIME tag
    /// inserted into manifests. Additionally, when an interval is specified
    /// ID3Timed Metadata messages will be generated every 5 seconds using the
    /// ingest time of the content.
    /// If the interval is not specified, or set to 0, then
    /// no EXT-X-PROGRAM-DATE-TIME tags will be inserted into manifests and no
    /// ID3Timed Metadata messages will be generated. Note that irrespective
    /// of this parameter, if any ID3 Timed Metadata is found in HTTP Live Streaming (HLS) input,
    /// it will be passed through to HLS output.</p>
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i64>,
    /// <p>Duration (in seconds) of each fragment. Actual fragments will be
    /// rounded to the nearest multiple of the source fragment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
    /// <p>When enabled, audio streams will be placed in rendition groups in the output.</p>
    #[serde(rename = "UseAudioRenditionGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_audio_rendition_group: Option<bool>,
}

/// <p>An endpoint for ingesting source content for a Channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IngestEndpoint {
    /// <p>The system generated unique identifier for the IngestEndpoint</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The system generated password for ingest authentication.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The ingest URL to which the source stream should be sent.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>The system generated username for ingest authentication.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListChannelsRequest {
    /// <p>Upper bound on number of records to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used to resume pagination from the end of a previous request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListChannelsResponse {
    /// <p>A list of Channel records.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<Channel>>,
    /// <p>A token that can be used to resume pagination from the end of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOriginEndpointsRequest {
    /// <p>When specified, the request will return only OriginEndpoints associated with the given Channel ID.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// <p>The upper bound on the number of records to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used to resume pagination from the end of a previous request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOriginEndpointsResponse {
    /// <p>A token that can be used to resume pagination from the end of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of OriginEndpoint records.</p>
    #[serde(rename = "OriginEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoints: Option<Vec<OriginEndpoint>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A Microsoft Smooth Streaming (MSS) encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MssEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A Microsoft Smooth Streaming (MSS) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MssPackage {
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<MssEncryption>,
    /// <p>The time window (in seconds) contained in each manifest.</p>
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i64>,
    /// <p>The duration (in seconds) of each segment.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

/// <p>An OriginEndpoint resource configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OriginEndpoint {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the Channel the OriginEndpoint is associated with.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A short string appended to the end of the OriginEndpoint URL.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Amount of delay (seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>The URL of the packaged OriginEndpoint for consumption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RotateChannelCredentialsRequest {
    /// <p>The ID of the channel to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RotateChannelCredentialsResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RotateIngestEndpointCredentialsRequest {
    /// <p>The ID of the channel the IngestEndpoint is on.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The id of the IngestEndpoint whose credentials should be rotated</p>
    #[serde(rename = "IngestEndpointId")]
    pub ingest_endpoint_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RotateIngestEndpointCredentialsResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A configuration for accessing an external Secure Packager and Encoder Key Exchange (SPEKE) service that will provide encryption keys.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpekeKeyProvider {
    /// <p>An Amazon Resource Name (ARN) of a Certificate Manager certificate
    /// that MediaPackage will use for enforcing secure end-to-end data
    /// transfer with the key provider service.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The resource ID to include in key requests.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>An Amazon Resource Name (ARN) of an IAM role that AWS Elemental
    /// MediaPackage will assume when accessing the key provider service.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The system IDs to include in key requests.</p>
    #[serde(rename = "SystemIds")]
    pub system_ids: Vec<String>,
    /// <p>The URL of the external key provider service.</p>
    #[serde(rename = "Url")]
    pub url: String,
}

/// <p>A StreamSelection configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamSelection {
    /// <p>The maximum video bitrate (bps) to include in output.</p>
    #[serde(rename = "MaxVideoBitsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_video_bits_per_second: Option<i64>,
    /// <p>The minimum video bitrate (bps) to include in output.</p>
    #[serde(rename = "MinVideoBitsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_video_bits_per_second: Option<i64>,
    /// <p>A directive that determines the order of streams in the output.</p>
    #[serde(rename = "StreamOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The key(s) of tag to be deleted</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Configuration parameters used to update the Channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateChannelRequest {
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the Channel to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateChannelResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Configuration parameters used to update an existing OriginEndpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateOriginEndpointRequest {
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackageCreateOrUpdateParameters>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A short string that will be appended to the end of the Endpoint URL.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (in seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    /// <p>Amount of delay (in seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateOriginEndpointResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the Channel the OriginEndpoint is associated with.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A short string appended to the end of the OriginEndpoint URL.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Amount of delay (seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>The URL of the packaged OriginEndpoint for consumption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

/// Errors returned by CreateChannel
#[derive(Debug, PartialEq)]
pub enum CreateChannelError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl CreateChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(CreateChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateChannelError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateChannelError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateChannelError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(CreateChannelError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateChannelError {
    fn description(&self) -> &str {
        match *self {
            CreateChannelError::Forbidden(ref cause) => cause,
            CreateChannelError::InternalServerError(ref cause) => cause,
            CreateChannelError::NotFound(ref cause) => cause,
            CreateChannelError::ServiceUnavailable(ref cause) => cause,
            CreateChannelError::TooManyRequests(ref cause) => cause,
            CreateChannelError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateOriginEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateOriginEndpointError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl CreateOriginEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOriginEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(CreateOriginEndpointError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateOriginEndpointError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateOriginEndpointError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateOriginEndpointError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateOriginEndpointError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(CreateOriginEndpointError::UnprocessableEntity(
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
impl fmt::Display for CreateOriginEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateOriginEndpointError {
    fn description(&self) -> &str {
        match *self {
            CreateOriginEndpointError::Forbidden(ref cause) => cause,
            CreateOriginEndpointError::InternalServerError(ref cause) => cause,
            CreateOriginEndpointError::NotFound(ref cause) => cause,
            CreateOriginEndpointError::ServiceUnavailable(ref cause) => cause,
            CreateOriginEndpointError::TooManyRequests(ref cause) => cause,
            CreateOriginEndpointError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteChannel
#[derive(Debug, PartialEq)]
pub enum DeleteChannelError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl DeleteChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteChannelError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteChannelError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteChannelError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DeleteChannelError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteChannelError::Forbidden(ref cause) => cause,
            DeleteChannelError::InternalServerError(ref cause) => cause,
            DeleteChannelError::NotFound(ref cause) => cause,
            DeleteChannelError::ServiceUnavailable(ref cause) => cause,
            DeleteChannelError::TooManyRequests(ref cause) => cause,
            DeleteChannelError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteOriginEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteOriginEndpointError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl DeleteOriginEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteOriginEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::UnprocessableEntity(
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
impl fmt::Display for DeleteOriginEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteOriginEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteOriginEndpointError::Forbidden(ref cause) => cause,
            DeleteOriginEndpointError::InternalServerError(ref cause) => cause,
            DeleteOriginEndpointError::NotFound(ref cause) => cause,
            DeleteOriginEndpointError::ServiceUnavailable(ref cause) => cause,
            DeleteOriginEndpointError::TooManyRequests(ref cause) => cause,
            DeleteOriginEndpointError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeChannel
#[derive(Debug, PartialEq)]
pub enum DescribeChannelError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl DescribeChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeChannelError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeChannelError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeChannelError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DescribeChannelError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeChannelError {
    fn description(&self) -> &str {
        match *self {
            DescribeChannelError::Forbidden(ref cause) => cause,
            DescribeChannelError::InternalServerError(ref cause) => cause,
            DescribeChannelError::NotFound(ref cause) => cause,
            DescribeChannelError::ServiceUnavailable(ref cause) => cause,
            DescribeChannelError::TooManyRequests(ref cause) => cause,
            DescribeChannelError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeOriginEndpoint
#[derive(Debug, PartialEq)]
pub enum DescribeOriginEndpointError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl DescribeOriginEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeOriginEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::UnprocessableEntity(
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
impl fmt::Display for DescribeOriginEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOriginEndpointError {
    fn description(&self) -> &str {
        match *self {
            DescribeOriginEndpointError::Forbidden(ref cause) => cause,
            DescribeOriginEndpointError::InternalServerError(ref cause) => cause,
            DescribeOriginEndpointError::NotFound(ref cause) => cause,
            DescribeOriginEndpointError::ServiceUnavailable(ref cause) => cause,
            DescribeOriginEndpointError::TooManyRequests(ref cause) => cause,
            DescribeOriginEndpointError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by ListChannels
#[derive(Debug, PartialEq)]
pub enum ListChannelsError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl ListChannelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListChannelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(ListChannelsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListChannelsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListChannelsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListChannelsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListChannelsError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(ListChannelsError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListChannelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListChannelsError {
    fn description(&self) -> &str {
        match *self {
            ListChannelsError::Forbidden(ref cause) => cause,
            ListChannelsError::InternalServerError(ref cause) => cause,
            ListChannelsError::NotFound(ref cause) => cause,
            ListChannelsError::ServiceUnavailable(ref cause) => cause,
            ListChannelsError::TooManyRequests(ref cause) => cause,
            ListChannelsError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOriginEndpoints
#[derive(Debug, PartialEq)]
pub enum ListOriginEndpointsError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl ListOriginEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOriginEndpointsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(ListOriginEndpointsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListOriginEndpointsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListOriginEndpointsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListOriginEndpointsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListOriginEndpointsError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(ListOriginEndpointsError::UnprocessableEntity(
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
impl fmt::Display for ListOriginEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOriginEndpointsError {
    fn description(&self) -> &str {
        match *self {
            ListOriginEndpointsError::Forbidden(ref cause) => cause,
            ListOriginEndpointsError::InternalServerError(ref cause) => cause,
            ListOriginEndpointsError::NotFound(ref cause) => cause,
            ListOriginEndpointsError::ServiceUnavailable(ref cause) => cause,
            ListOriginEndpointsError::TooManyRequests(ref cause) => cause,
            ListOriginEndpointsError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
        match *self {}
    }
}
/// Errors returned by RotateChannelCredentials
#[derive(Debug, PartialEq)]
pub enum RotateChannelCredentialsError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl RotateChannelCredentialsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RotateChannelCredentialsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(RotateChannelCredentialsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        RotateChannelCredentialsError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(RotateChannelCredentialsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RotateChannelCredentialsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RotateChannelCredentialsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        RotateChannelCredentialsError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RotateChannelCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RotateChannelCredentialsError {
    fn description(&self) -> &str {
        match *self {
            RotateChannelCredentialsError::Forbidden(ref cause) => cause,
            RotateChannelCredentialsError::InternalServerError(ref cause) => cause,
            RotateChannelCredentialsError::NotFound(ref cause) => cause,
            RotateChannelCredentialsError::ServiceUnavailable(ref cause) => cause,
            RotateChannelCredentialsError::TooManyRequests(ref cause) => cause,
            RotateChannelCredentialsError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by RotateIngestEndpointCredentials
#[derive(Debug, PartialEq)]
pub enum RotateIngestEndpointCredentialsError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl RotateIngestEndpointCredentialsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RotateIngestEndpointCredentialsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(RotateIngestEndpointCredentialsError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        RotateIngestEndpointCredentialsError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(RotateIngestEndpointCredentialsError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        RotateIngestEndpointCredentialsError::ServiceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        RotateIngestEndpointCredentialsError::TooManyRequests(err.msg),
                    )
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        RotateIngestEndpointCredentialsError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RotateIngestEndpointCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RotateIngestEndpointCredentialsError {
    fn description(&self) -> &str {
        match *self {
            RotateIngestEndpointCredentialsError::Forbidden(ref cause) => cause,
            RotateIngestEndpointCredentialsError::InternalServerError(ref cause) => cause,
            RotateIngestEndpointCredentialsError::NotFound(ref cause) => cause,
            RotateIngestEndpointCredentialsError::ServiceUnavailable(ref cause) => cause,
            RotateIngestEndpointCredentialsError::TooManyRequests(ref cause) => cause,
            RotateIngestEndpointCredentialsError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
        match *self {}
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
        match *self {}
    }
}
/// Errors returned by UpdateChannel
#[derive(Debug, PartialEq)]
pub enum UpdateChannelError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl UpdateChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateChannelError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateChannelError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateChannelError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(UpdateChannelError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateChannelError::Forbidden(ref cause) => cause,
            UpdateChannelError::InternalServerError(ref cause) => cause,
            UpdateChannelError::NotFound(ref cause) => cause,
            UpdateChannelError::ServiceUnavailable(ref cause) => cause,
            UpdateChannelError::TooManyRequests(ref cause) => cause,
            UpdateChannelError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateOriginEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateOriginEndpointError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
}

impl UpdateOriginEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateOriginEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::UnprocessableEntity(
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
impl fmt::Display for UpdateOriginEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateOriginEndpointError {
    fn description(&self) -> &str {
        match *self {
            UpdateOriginEndpointError::Forbidden(ref cause) => cause,
            UpdateOriginEndpointError::InternalServerError(ref cause) => cause,
            UpdateOriginEndpointError::NotFound(ref cause) => cause,
            UpdateOriginEndpointError::ServiceUnavailable(ref cause) => cause,
            UpdateOriginEndpointError::TooManyRequests(ref cause) => cause,
            UpdateOriginEndpointError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the MediaPackage API. MediaPackage clients implement this trait.
pub trait MediaPackage {
    /// <p>Creates a new Channel.</p>
    fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> RusotoFuture<CreateChannelResponse, CreateChannelError>;

    /// <p>Creates a new OriginEndpoint record.</p>
    fn create_origin_endpoint(
        &self,
        input: CreateOriginEndpointRequest,
    ) -> RusotoFuture<CreateOriginEndpointResponse, CreateOriginEndpointError>;

    /// <p>Deletes an existing Channel.</p>
    fn delete_channel(
        &self,
        input: DeleteChannelRequest,
    ) -> RusotoFuture<DeleteChannelResponse, DeleteChannelError>;

    /// <p>Deletes an existing OriginEndpoint.</p>
    fn delete_origin_endpoint(
        &self,
        input: DeleteOriginEndpointRequest,
    ) -> RusotoFuture<DeleteOriginEndpointResponse, DeleteOriginEndpointError>;

    /// <p>Gets details about a Channel.</p>
    fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> RusotoFuture<DescribeChannelResponse, DescribeChannelError>;

    /// <p>Gets details about an existing OriginEndpoint.</p>
    fn describe_origin_endpoint(
        &self,
        input: DescribeOriginEndpointRequest,
    ) -> RusotoFuture<DescribeOriginEndpointResponse, DescribeOriginEndpointError>;

    /// <p>Returns a collection of Channels.</p>
    fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> RusotoFuture<ListChannelsResponse, ListChannelsError>;

    /// <p>Returns a collection of OriginEndpoint records.</p>
    fn list_origin_endpoints(
        &self,
        input: ListOriginEndpointsRequest,
    ) -> RusotoFuture<ListOriginEndpointsResponse, ListOriginEndpointsError>;

    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Changes the Channel&#39;s first IngestEndpoint&#39;s username and password. WARNING - This API is deprecated. Please use RotateIngestEndpointCredentials instead</p>
    fn rotate_channel_credentials(
        &self,
        input: RotateChannelCredentialsRequest,
    ) -> RusotoFuture<RotateChannelCredentialsResponse, RotateChannelCredentialsError>;

    /// <p>Rotate the IngestEndpoint&#39;s username and password, as specified by the IngestEndpoint&#39;s id.</p>
    fn rotate_ingest_endpoint_credentials(
        &self,
        input: RotateIngestEndpointCredentialsRequest,
    ) -> RusotoFuture<RotateIngestEndpointCredentialsResponse, RotateIngestEndpointCredentialsError>;

    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

    /// <p>Updates an existing Channel.</p>
    fn update_channel(
        &self,
        input: UpdateChannelRequest,
    ) -> RusotoFuture<UpdateChannelResponse, UpdateChannelError>;

    /// <p>Updates an existing OriginEndpoint.</p>
    fn update_origin_endpoint(
        &self,
        input: UpdateOriginEndpointRequest,
    ) -> RusotoFuture<UpdateOriginEndpointResponse, UpdateOriginEndpointError>;
}
/// A client for the MediaPackage API.
#[derive(Clone)]
pub struct MediaPackageClient {
    client: Client,
    region: region::Region,
}

impl MediaPackageClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaPackageClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaPackageClient
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

    pub fn new_with_client(client: Client, region: region::Region) -> MediaPackageClient {
        MediaPackageClient { client, region }
    }
}

impl fmt::Debug for MediaPackageClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MediaPackageClient")
            .field("region", &self.region)
            .finish()
    }
}

impl MediaPackage for MediaPackageClient {
    /// <p>Creates a new Channel.</p>
    fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> RusotoFuture<CreateChannelResponse, CreateChannelError> {
        let request_uri = "/channels";

        let mut request = SignedRequest::new("POST", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new OriginEndpoint record.</p>
    fn create_origin_endpoint(
        &self,
        input: CreateOriginEndpointRequest,
    ) -> RusotoFuture<CreateOriginEndpointResponse, CreateOriginEndpointError> {
        let request_uri = "/origin_endpoints";

        let mut request = SignedRequest::new("POST", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateOriginEndpointResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateOriginEndpointError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes an existing Channel.</p>
    fn delete_channel(
        &self,
        input: DeleteChannelRequest,
    ) -> RusotoFuture<DeleteChannelResponse, DeleteChannelError> {
        let request_uri = format!("/channels/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an existing OriginEndpoint.</p>
    fn delete_origin_endpoint(
        &self,
        input: DeleteOriginEndpointRequest,
    ) -> RusotoFuture<DeleteOriginEndpointResponse, DeleteOriginEndpointError> {
        let request_uri = format!("/origin_endpoints/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteOriginEndpointResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteOriginEndpointError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets details about a Channel.</p>
    fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> RusotoFuture<DescribeChannelResponse, DescribeChannelError> {
        let request_uri = format!("/channels/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets details about an existing OriginEndpoint.</p>
    fn describe_origin_endpoint(
        &self,
        input: DescribeOriginEndpointRequest,
    ) -> RusotoFuture<DescribeOriginEndpointResponse, DescribeOriginEndpointError> {
        let request_uri = format!("/origin_endpoints/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeOriginEndpointResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeOriginEndpointError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a collection of Channels.</p>
    fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> RusotoFuture<ListChannelsResponse, ListChannelsError> {
        let request_uri = "/channels";

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListChannelsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListChannelsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a collection of OriginEndpoint records.</p>
    fn list_origin_endpoints(
        &self,
        input: ListOriginEndpointsRequest,
    ) -> RusotoFuture<ListOriginEndpointsResponse, ListOriginEndpointsError> {
        let request_uri = "/origin_endpoints";

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.channel_id {
            params.put("channelId", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListOriginEndpointsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListOriginEndpointsError::from_response(response))
                    }),
                )
            }
        })
    }

    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
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

    /// <p>Changes the Channel&#39;s first IngestEndpoint&#39;s username and password. WARNING - This API is deprecated. Please use RotateIngestEndpointCredentials instead</p>
    fn rotate_channel_credentials(
        &self,
        input: RotateChannelCredentialsRequest,
    ) -> RusotoFuture<RotateChannelCredentialsResponse, RotateChannelCredentialsError> {
        let request_uri = format!("/channels/{id}/credentials", id = input.id);

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RotateChannelCredentialsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RotateChannelCredentialsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Rotate the IngestEndpoint&#39;s username and password, as specified by the IngestEndpoint&#39;s id.</p>
    fn rotate_ingest_endpoint_credentials(
        &self,
        input: RotateIngestEndpointCredentialsRequest,
    ) -> RusotoFuture<RotateIngestEndpointCredentialsResponse, RotateIngestEndpointCredentialsError>
    {
        let request_uri = format!(
            "/channels/{id}/ingest_endpoints/{ingest_endpoint_id}/credentials",
            id = input.id,
            ingest_endpoint_id = input.ingest_endpoint_id
        );

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RotateIngestEndpointCredentialsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RotateIngestEndpointCredentialsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "mediapackage", &self.region, &request_uri);
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

    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "mediapackage", &self.region, &request_uri);
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

    /// <p>Updates an existing Channel.</p>
    fn update_channel(
        &self,
        input: UpdateChannelRequest,
    ) -> RusotoFuture<UpdateChannelResponse, UpdateChannelError> {
        let request_uri = format!("/channels/{id}", id = input.id);

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an existing OriginEndpoint.</p>
    fn update_origin_endpoint(
        &self,
        input: UpdateOriginEndpointRequest,
    ) -> RusotoFuture<UpdateOriginEndpointResponse, UpdateOriginEndpointError> {
        let request_uri = format!("/origin_endpoints/{id}", id = input.id);

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateOriginEndpointResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateOriginEndpointError::from_response(response))
                    }),
                )
            }
        })
    }
}
