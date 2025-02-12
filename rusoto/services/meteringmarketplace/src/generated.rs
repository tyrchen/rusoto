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
/// <p>A BatchMeterUsageRequest contains UsageRecords, which indicate quantities of usage within your application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchMeterUsageRequest {
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    #[serde(rename = "ProductCode")]
    pub product_code: String,
    /// <p>The set of UsageRecords to submit. BatchMeterUsage accepts up to 25 UsageRecords at a time.</p>
    #[serde(rename = "UsageRecords")]
    pub usage_records: Vec<UsageRecord>,
}

/// <p>Contains the UsageRecords processed by BatchMeterUsage and any records that have failed due to transient error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchMeterUsageResult {
    /// <p>Contains all UsageRecords processed by BatchMeterUsage. These records were either honored by AWS Marketplace Metering Service or were invalid.</p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<UsageRecordResult>>,
    /// <p>Contains all UsageRecords that were not processed by BatchMeterUsage. This is a list of UsageRecords. You can retry the failed request by making another BatchMeterUsage call with this list as input in the BatchMeterUsageRequest.</p>
    #[serde(rename = "UnprocessedRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_records: Option<Vec<UsageRecord>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MeterUsageRequest {
    /// <p>Checks whether you have the permissions required for the action, but does not make the request. If you have the permissions, the request returns DryRunOperation; otherwise, it returns UnauthorizedException. Defaults to <code>false</code> if not specified.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    #[serde(rename = "ProductCode")]
    pub product_code: String,
    /// <p>Timestamp, in UTC, for which the usage is being reported. Your application can meter usage for up to one hour in the past. Make sure the timestamp value is not before the start of the software usage.</p>
    #[serde(rename = "Timestamp")]
    pub timestamp: f64,
    /// <p>It will be one of the fcp dimension name provided during the publishing of the product.</p>
    #[serde(rename = "UsageDimension")]
    pub usage_dimension: String,
    /// <p>Consumption value for the hour. Defaults to <code>0</code> if not specified.</p>
    #[serde(rename = "UsageQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_quantity: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MeterUsageResult {
    /// <p>Metering record id.</p>
    #[serde(rename = "MeteringRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_record_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterUsageRequest {
    /// <p>(Optional) To scope down the registration to a specific running software instance and guard against replay attacks.</p>
    #[serde(rename = "Nonce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    #[serde(rename = "ProductCode")]
    pub product_code: String,
    /// <p>Public Key Version provided by AWS Marketplace</p>
    #[serde(rename = "PublicKeyVersion")]
    pub public_key_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterUsageResult {
    /// <p>(Optional) Only included when public key version has expired</p>
    #[serde(rename = "PublicKeyRotationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_rotation_timestamp: Option<f64>,
    /// <p>JWT Token</p>
    #[serde(rename = "Signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// <p>Contains input to the ResolveCustomer operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResolveCustomerRequest {
    /// <p>When a buyer visits your website during the registration process, the buyer submits a registration token through the browser. The registration token is resolved to obtain a CustomerIdentifier and product code.</p>
    #[serde(rename = "RegistrationToken")]
    pub registration_token: String,
}

/// <p>The result of the ResolveCustomer operation. Contains the CustomerIdentifier and product code.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolveCustomerResult {
    /// <p>The CustomerIdentifier is used to identify an individual customer in your application. Calls to BatchMeterUsage require CustomerIdentifiers for each UsageRecord.</p>
    #[serde(rename = "CustomerIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identifier: Option<String>,
    /// <p>The product code is returned to confirm that the buyer is registering for your product. Subsequent BatchMeterUsage calls should be made using this product code.</p>
    #[serde(rename = "ProductCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
}

/// <p>A UsageRecord indicates a quantity of usage for a given product, customer, dimension and time.</p> <p>Multiple requests with the same UsageRecords as input will be deduplicated to prevent double charges.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsageRecord {
    /// <p>The CustomerIdentifier is obtained through the ResolveCustomer operation and represents an individual buyer in your application.</p>
    #[serde(rename = "CustomerIdentifier")]
    pub customer_identifier: String,
    /// <p>During the process of registering a product on AWS Marketplace, up to eight dimensions are specified. These represent different units of value in your application.</p>
    #[serde(rename = "Dimension")]
    pub dimension: String,
    /// <p>The quantity of usage consumed by the customer for the given dimension and time. Defaults to <code>0</code> if not specified.</p>
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// <p>Timestamp, in UTC, for which the usage is being reported.</p> <p>Your application can meter usage for up to one hour in the past. Make sure the timestamp value is not before the start of the software usage.</p>
    #[serde(rename = "Timestamp")]
    pub timestamp: f64,
}

/// <p>A UsageRecordResult indicates the status of a given UsageRecord processed by BatchMeterUsage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UsageRecordResult {
    /// <p>The MeteringRecordId is a unique identifier for this metering event.</p>
    #[serde(rename = "MeteringRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_record_id: Option<String>,
    /// <p><p>The UsageRecordResult Status indicates the status of an individual UsageRecord processed by BatchMeterUsage.</p> <ul> <li> <p> <i>Success</i>- The UsageRecord was accepted and honored by BatchMeterUsage.</p> </li> <li> <p> <i>CustomerNotSubscribed</i>- The CustomerIdentifier specified is not subscribed to your product. The UsageRecord was not honored. Future UsageRecords for this customer will fail until the customer subscribes to your product.</p> </li> <li> <p> <i>DuplicateRecord</i>- Indicates that the UsageRecord was invalid and not honored. A previously metered UsageRecord had the same customer, dimension, and time, but a different quantity.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The UsageRecord that was part of the BatchMeterUsage request.</p>
    #[serde(rename = "UsageRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_record: Option<UsageRecord>,
}

/// Errors returned by BatchMeterUsage
#[derive(Debug, PartialEq)]
pub enum BatchMeterUsageError {
    /// <p>The API is disabled in the Region.</p>
    DisabledApi(String),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    /// <p>You have metered usage for a CustomerIdentifier that does not exist.</p>
    InvalidCustomerIdentifier(String),
    /// <p>The product code passed does not match the product code used for publishing the product.</p>
    InvalidProductCode(String),
    /// <p>The usage dimension does not match one of the UsageDimensions associated with products.</p>
    InvalidUsageDimension(String),
    /// <p>The calls to the API are throttled.</p>
    Throttling(String),
    /// <p>The timestamp value passed in the meterUsage() is out of allowed range.</p>
    TimestampOutOfBounds(String),
}

impl BatchMeterUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchMeterUsageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DisabledApiException" => {
                    return RusotoError::Service(BatchMeterUsageError::DisabledApi(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(BatchMeterUsageError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidCustomerIdentifierException" => {
                    return RusotoError::Service(BatchMeterUsageError::InvalidCustomerIdentifier(
                        err.msg,
                    ))
                }
                "InvalidProductCodeException" => {
                    return RusotoError::Service(BatchMeterUsageError::InvalidProductCode(err.msg))
                }
                "InvalidUsageDimensionException" => {
                    return RusotoError::Service(BatchMeterUsageError::InvalidUsageDimension(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchMeterUsageError::Throttling(err.msg))
                }
                "TimestampOutOfBoundsException" => {
                    return RusotoError::Service(BatchMeterUsageError::TimestampOutOfBounds(
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
impl fmt::Display for BatchMeterUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchMeterUsageError {
    fn description(&self) -> &str {
        match *self {
            BatchMeterUsageError::DisabledApi(ref cause) => cause,
            BatchMeterUsageError::InternalServiceError(ref cause) => cause,
            BatchMeterUsageError::InvalidCustomerIdentifier(ref cause) => cause,
            BatchMeterUsageError::InvalidProductCode(ref cause) => cause,
            BatchMeterUsageError::InvalidUsageDimension(ref cause) => cause,
            BatchMeterUsageError::Throttling(ref cause) => cause,
            BatchMeterUsageError::TimestampOutOfBounds(ref cause) => cause,
        }
    }
}
/// Errors returned by MeterUsage
#[derive(Debug, PartialEq)]
pub enum MeterUsageError {
    /// <p>A metering record has already been emitted by the same EC2 instance for the given {usageDimension, timestamp} with a different usageQuantity.</p>
    DuplicateRequest(String),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    /// <p>The endpoint being called is in a Region different from your EC2 instance. The Region of the Metering Service endpoint and the Region of the EC2 instance must match.</p>
    InvalidEndpointRegion(String),
    /// <p>The product code passed does not match the product code used for publishing the product.</p>
    InvalidProductCode(String),
    /// <p>The usage dimension does not match one of the UsageDimensions associated with products.</p>
    InvalidUsageDimension(String),
    /// <p>The calls to the API are throttled.</p>
    Throttling(String),
    /// <p>The timestamp value passed in the meterUsage() is out of allowed range.</p>
    TimestampOutOfBounds(String),
}

impl MeterUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<MeterUsageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequestException" => {
                    return RusotoError::Service(MeterUsageError::DuplicateRequest(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(MeterUsageError::InternalServiceError(err.msg))
                }
                "InvalidEndpointRegionException" => {
                    return RusotoError::Service(MeterUsageError::InvalidEndpointRegion(err.msg))
                }
                "InvalidProductCodeException" => {
                    return RusotoError::Service(MeterUsageError::InvalidProductCode(err.msg))
                }
                "InvalidUsageDimensionException" => {
                    return RusotoError::Service(MeterUsageError::InvalidUsageDimension(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(MeterUsageError::Throttling(err.msg))
                }
                "TimestampOutOfBoundsException" => {
                    return RusotoError::Service(MeterUsageError::TimestampOutOfBounds(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for MeterUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for MeterUsageError {
    fn description(&self) -> &str {
        match *self {
            MeterUsageError::DuplicateRequest(ref cause) => cause,
            MeterUsageError::InternalServiceError(ref cause) => cause,
            MeterUsageError::InvalidEndpointRegion(ref cause) => cause,
            MeterUsageError::InvalidProductCode(ref cause) => cause,
            MeterUsageError::InvalidUsageDimension(ref cause) => cause,
            MeterUsageError::Throttling(ref cause) => cause,
            MeterUsageError::TimestampOutOfBounds(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterUsage
#[derive(Debug, PartialEq)]
pub enum RegisterUsageError {
    /// <p>Exception thrown when the customer does not have a valid subscription for the product.</p>
    CustomerNotEntitled(String),
    /// <p>The API is disabled in the Region.</p>
    DisabledApi(String),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    /// <p>The product code passed does not match the product code used for publishing the product.</p>
    InvalidProductCode(String),
    /// <p>Public Key version is invalid.</p>
    InvalidPublicKeyVersion(String),
    /// <p>RegisterUsage must be called in the same AWS Region the ECS task was launched in. This prevents a container from hardcoding a Region (e.g. withRegion(“us-east-1”) when calling RegisterUsage.</p>
    InvalidRegion(String),
    /// <p>AWS Marketplace does not support metering usage from the underlying platform. Currently, only Amazon ECS is supported.</p>
    PlatformNotSupported(String),
    /// <p>The calls to the API are throttled.</p>
    Throttling(String),
}

impl RegisterUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterUsageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomerNotEntitledException" => {
                    return RusotoError::Service(RegisterUsageError::CustomerNotEntitled(err.msg))
                }
                "DisabledApiException" => {
                    return RusotoError::Service(RegisterUsageError::DisabledApi(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(RegisterUsageError::InternalServiceError(err.msg))
                }
                "InvalidProductCodeException" => {
                    return RusotoError::Service(RegisterUsageError::InvalidProductCode(err.msg))
                }
                "InvalidPublicKeyVersionException" => {
                    return RusotoError::Service(RegisterUsageError::InvalidPublicKeyVersion(
                        err.msg,
                    ))
                }
                "InvalidRegionException" => {
                    return RusotoError::Service(RegisterUsageError::InvalidRegion(err.msg))
                }
                "PlatformNotSupportedException" => {
                    return RusotoError::Service(RegisterUsageError::PlatformNotSupported(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RegisterUsageError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterUsageError {
    fn description(&self) -> &str {
        match *self {
            RegisterUsageError::CustomerNotEntitled(ref cause) => cause,
            RegisterUsageError::DisabledApi(ref cause) => cause,
            RegisterUsageError::InternalServiceError(ref cause) => cause,
            RegisterUsageError::InvalidProductCode(ref cause) => cause,
            RegisterUsageError::InvalidPublicKeyVersion(ref cause) => cause,
            RegisterUsageError::InvalidRegion(ref cause) => cause,
            RegisterUsageError::PlatformNotSupported(ref cause) => cause,
            RegisterUsageError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ResolveCustomer
#[derive(Debug, PartialEq)]
pub enum ResolveCustomerError {
    /// <p>The API is disabled in the Region.</p>
    DisabledApi(String),
    /// <p>The submitted registration token has expired. This can happen if the buyer's browser takes too long to redirect to your page, the buyer has resubmitted the registration token, or your application has held on to the registration token for too long. Your SaaS registration website should redeem this token as soon as it is submitted by the buyer's browser.</p>
    ExpiredToken(String),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    /// <p>Registration token is invalid.</p>
    InvalidToken(String),
    /// <p>The calls to the API are throttled.</p>
    Throttling(String),
}

impl ResolveCustomerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResolveCustomerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DisabledApiException" => {
                    return RusotoError::Service(ResolveCustomerError::DisabledApi(err.msg))
                }
                "ExpiredTokenException" => {
                    return RusotoError::Service(ResolveCustomerError::ExpiredToken(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ResolveCustomerError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidTokenException" => {
                    return RusotoError::Service(ResolveCustomerError::InvalidToken(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ResolveCustomerError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ResolveCustomerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResolveCustomerError {
    fn description(&self) -> &str {
        match *self {
            ResolveCustomerError::DisabledApi(ref cause) => cause,
            ResolveCustomerError::ExpiredToken(ref cause) => cause,
            ResolveCustomerError::InternalServiceError(ref cause) => cause,
            ResolveCustomerError::InvalidToken(ref cause) => cause,
            ResolveCustomerError::Throttling(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWSMarketplace Metering API. AWSMarketplace Metering clients implement this trait.
pub trait MarketplaceMetering {
    /// <p>BatchMeterUsage is called from a SaaS application listed on the AWS Marketplace to post metering records for a set of customers.</p> <p>For identical requests, the API is idempotent; requests can be retried with the same records or a subset of the input records.</p> <p>Every request to BatchMeterUsage is for one product. If you need to meter usage for multiple products, you must make multiple calls to BatchMeterUsage.</p> <p>BatchMeterUsage can process up to 25 UsageRecords at a time.</p>
    fn batch_meter_usage(
        &self,
        input: BatchMeterUsageRequest,
    ) -> RusotoFuture<BatchMeterUsageResult, BatchMeterUsageError>;

    /// <p>API to emit metering records. For identical requests, the API is idempotent. It simply returns the metering record ID.</p> <p>MeterUsage is authenticated on the buyer's AWS account, generally when running from an EC2 instance on the AWS Marketplace.</p>
    fn meter_usage(
        &self,
        input: MeterUsageRequest,
    ) -> RusotoFuture<MeterUsageResult, MeterUsageError>;

    /// <p><p>Paid container software products sold through AWS Marketplace must integrate with the AWS Marketplace Metering Service and call the RegisterUsage operation for software entitlement and metering. Calling RegisterUsage from containers running outside of ECS is not currently supported. Free and BYOL products for ECS aren&#39;t required to call RegisterUsage, but you may choose to do so if you would like to receive usage data in your seller reports. The sections below explain the behavior of RegisterUsage. RegisterUsage performs two primary functions: metering and entitlement.</p> <ul> <li> <p> <i>Entitlement</i>: RegisterUsage allows you to verify that the customer running your paid software is subscribed to your product on AWS Marketplace, enabling you to guard against unauthorized use. Your container image that integrates with RegisterUsage is only required to guard against unauthorized use at container startup, as such a CustomerNotSubscribedException/PlatformNotSupportedException will only be thrown on the initial call to RegisterUsage. Subsequent calls from the same Amazon ECS task instance (e.g. task-id) will not throw a CustomerNotSubscribedException, even if the customer unsubscribes while the Amazon ECS task is still running.</p> </li> <li> <p> <i>Metering</i>: RegisterUsage meters software use per ECS task, per hour, with usage prorated to the second. A minimum of 1 minute of usage applies to tasks that are short lived. For example, if a customer has a 10 node ECS cluster and creates an ECS service configured as a Daemon Set, then ECS will launch a task on all 10 cluster nodes and the customer will be charged: (10 * hourly_rate). Metering for software use is automatically handled by the AWS Marketplace Metering Control Plane -- your software is not required to perform any metering specific actions, other than call RegisterUsage once for metering of software use to commence. The AWS Marketplace Metering Control Plane will also continue to bill customers for running ECS tasks, regardless of the customers subscription state, removing the need for your software to perform entitlement checks at runtime.</p> </li> </ul></p>
    fn register_usage(
        &self,
        input: RegisterUsageRequest,
    ) -> RusotoFuture<RegisterUsageResult, RegisterUsageError>;

    /// <p>ResolveCustomer is called by a SaaS application during the registration process. When a buyer visits your website during the registration process, the buyer submits a registration token through their browser. The registration token is resolved through this API to obtain a CustomerIdentifier and product code.</p>
    fn resolve_customer(
        &self,
        input: ResolveCustomerRequest,
    ) -> RusotoFuture<ResolveCustomerResult, ResolveCustomerError>;
}
/// A client for the AWSMarketplace Metering API.
#[derive(Clone)]
pub struct MarketplaceMeteringClient {
    client: Client,
    region: region::Region,
}

impl MarketplaceMeteringClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MarketplaceMeteringClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MarketplaceMeteringClient
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

    pub fn new_with_client(client: Client, region: region::Region) -> MarketplaceMeteringClient {
        MarketplaceMeteringClient { client, region }
    }
}

impl fmt::Debug for MarketplaceMeteringClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MarketplaceMeteringClient")
            .field("region", &self.region)
            .finish()
    }
}

impl MarketplaceMetering for MarketplaceMeteringClient {
    /// <p>BatchMeterUsage is called from a SaaS application listed on the AWS Marketplace to post metering records for a set of customers.</p> <p>For identical requests, the API is idempotent; requests can be retried with the same records or a subset of the input records.</p> <p>Every request to BatchMeterUsage is for one product. If you need to meter usage for multiple products, you must make multiple calls to BatchMeterUsage.</p> <p>BatchMeterUsage can process up to 25 UsageRecords at a time.</p>
    fn batch_meter_usage(
        &self,
        input: BatchMeterUsageRequest,
    ) -> RusotoFuture<BatchMeterUsageResult, BatchMeterUsageError> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("metering.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPMeteringService.BatchMeterUsage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchMeterUsageResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchMeterUsageError::from_response(response))),
                )
            }
        })
    }

    /// <p>API to emit metering records. For identical requests, the API is idempotent. It simply returns the metering record ID.</p> <p>MeterUsage is authenticated on the buyer's AWS account, generally when running from an EC2 instance on the AWS Marketplace.</p>
    fn meter_usage(
        &self,
        input: MeterUsageRequest,
    ) -> RusotoFuture<MeterUsageResult, MeterUsageError> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("metering.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPMeteringService.MeterUsage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<MeterUsageResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(MeterUsageError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Paid container software products sold through AWS Marketplace must integrate with the AWS Marketplace Metering Service and call the RegisterUsage operation for software entitlement and metering. Calling RegisterUsage from containers running outside of ECS is not currently supported. Free and BYOL products for ECS aren&#39;t required to call RegisterUsage, but you may choose to do so if you would like to receive usage data in your seller reports. The sections below explain the behavior of RegisterUsage. RegisterUsage performs two primary functions: metering and entitlement.</p> <ul> <li> <p> <i>Entitlement</i>: RegisterUsage allows you to verify that the customer running your paid software is subscribed to your product on AWS Marketplace, enabling you to guard against unauthorized use. Your container image that integrates with RegisterUsage is only required to guard against unauthorized use at container startup, as such a CustomerNotSubscribedException/PlatformNotSupportedException will only be thrown on the initial call to RegisterUsage. Subsequent calls from the same Amazon ECS task instance (e.g. task-id) will not throw a CustomerNotSubscribedException, even if the customer unsubscribes while the Amazon ECS task is still running.</p> </li> <li> <p> <i>Metering</i>: RegisterUsage meters software use per ECS task, per hour, with usage prorated to the second. A minimum of 1 minute of usage applies to tasks that are short lived. For example, if a customer has a 10 node ECS cluster and creates an ECS service configured as a Daemon Set, then ECS will launch a task on all 10 cluster nodes and the customer will be charged: (10 * hourly_rate). Metering for software use is automatically handled by the AWS Marketplace Metering Control Plane -- your software is not required to perform any metering specific actions, other than call RegisterUsage once for metering of software use to commence. The AWS Marketplace Metering Control Plane will also continue to bill customers for running ECS tasks, regardless of the customers subscription state, removing the need for your software to perform entitlement checks at runtime.</p> </li> </ul></p>
    fn register_usage(
        &self,
        input: RegisterUsageRequest,
    ) -> RusotoFuture<RegisterUsageResult, RegisterUsageError> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("metering.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPMeteringService.RegisterUsage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RegisterUsageResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RegisterUsageError::from_response(response))),
                )
            }
        })
    }

    /// <p>ResolveCustomer is called by a SaaS application during the registration process. When a buyer visits your website during the registration process, the buyer submits a registration token through their browser. The registration token is resolved through this API to obtain a CustomerIdentifier and product code.</p>
    fn resolve_customer(
        &self,
        input: ResolveCustomerRequest,
    ) -> RusotoFuture<ResolveCustomerResult, ResolveCustomerError> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("metering.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPMeteringService.ResolveCustomer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ResolveCustomerResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ResolveCustomerError::from_response(response))),
                )
            }
        })
    }
}
