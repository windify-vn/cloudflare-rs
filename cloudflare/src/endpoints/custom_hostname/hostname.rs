use crate::framework::endpoint::{serialize_query, EndpointSpec, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use crate::framework::{BooleanValue, OrderDirection, TlsVersion};
use http::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Create a custom hostname for a zone.
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/create>
#[derive(Debug)]
pub struct CreateCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub params: CreateCustomHostnameParams,
}
impl EndpointSpec for CreateCustomHostname<'_> {
    type JsonResponse = CustomHostnameDetails;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/custom_hostnames", self.zone_identifier)
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Clone, Default)]
pub struct CreateCustomHostnameParams {
    pub hostname: String,
    pub ssl: HostnameSslProperties,
    pub custom_metadata: Option<HashMap<String, String>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Clone, Default)]
pub struct HostnameSslProperties {
    pub bundle_method: Option<BundleMethod>,
    pub certificate_authority: Option<CertificateCA>,
    pub cloudflare_branding: Option<bool>,
    pub custom_cert_bundle: Option<Vec<CertificateBundle>>,
    pub custom_certificate: Option<String>,
    pub custom_key: Option<String>,
    pub method: Option<DomainValidationMethod>,
    pub settings: Option<SslSettings>,
    #[serde(rename = "type")]
    pub validation_type: DomainValidationType,
    pub wildcard: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BundleMethod {
    #[default]
    Ubiquitous,
    Optimal,
    Force,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CertificateCA {
    Digicert,
    #[default]
    Google,
    LetsEncrypt,
    SslCom,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DomainValidationMethod {
    #[default]
    Http,
    Text,
    Email,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct CertificateBundle {
    pub custom_certificate: String,
    pub custom_key: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SslSettings {
    #[serde(default)]
    pub ciphers: Vec<String>,
    pub early_hints: Option<BooleanValue>,
    pub http2: Option<BooleanValue>,
    pub min_tls_version: Option<TlsVersion>,
    pub tls_1_3: Option<BooleanValue>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DomainValidationType {
    #[default]
    Dv,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomHostnameDetails {
    pub id: String,
    pub hostname: String,
    pub ssl: SslHostnameStatus,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub custom_metadata: HashMap<String, String>,
    pub custom_origin_server: Option<String>,
    pub custom_origin_sni: Option<String>,
    pub ownership_verification: Option<OwnershipVerification>,
    pub ownership_verification_http: Option<OwnershipVerificationHttp>,
    pub status: HostnameActivationStatus,
    #[serde(default)]
    pub verification_errors: Vec<String>,
}

impl ApiResult for CustomHostnameDetails {}
impl ApiResult for Vec<CustomHostnameDetails> {}

#[derive(Debug, Deserialize, Clone)]
pub struct OwnershipVerification {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub record_type: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OwnershipVerificationHttp {
    pub http_body: Option<String>,
    pub http_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SslHostnameStatus {
    pub id: Option<String>,
    pub bundle_method: Option<BundleMethod>,
    pub certificate_authority: Option<CertificateCA>,
    pub custom_certificate: Option<String>,
    pub custom_csr_id: Option<String>,
    pub custom_key: Option<String>,
    pub expires_on: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub hosts: Vec<String>,
    pub issuer: Option<String>,
    pub method: Option<DomainValidationMethod>,
    pub settings: Option<SslSettings>,
    pub signature: Option<String>,
    pub status: Option<SslCertificateStatus>,
    #[serde(rename = "type")]
    pub validation_type: Option<DomainValidationType>,
    pub uploaded_on: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub validation_errors: Vec<ValidationError>,
    #[serde(default)]
    pub validation_records: Vec<ValidationRecord>,
    pub wildcard: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ValidationError {
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ValidationRecord {
    #[serde(default)]
    pub emails: Vec<String>,
    pub http_body: Option<String>,
    pub http_url: Option<String>,
    pub txt_name: Option<String>,
    pub txt_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SslCertificateStatus {
    Initializing,
    PendingValidation,
    Deleted,
    PendingIssuance,
    PendingDeployment,
    PendingDeletion,
    PendingExpiration,
    Expired,
    Active,
    InitializingTimedOut,
    ValidationTimedOut,
    IssuanceTimedOut,
    DeploymentTimedOut,
    DeletionTimedOut,
    PendingCleanup,
    StagingDeployment,
    StagingActive,
    Deactivating,
    Inactive,
    BackupIssued,
    HoldingDeployment,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HostnameActivationStatus {
    Active,
    Pending,
    ActiveRedeploying,
    Moved,
    PendingDeletion,
    Deleted,
    PendingBlocked,
    PendingMigration,
    PendingProvisioned,
    TestPending,
    TestActive,
    TestActiveApex,
    TestBlocked,
    TestFailed,
    Provisioned,
    Blocked,
}

/// Delete a custom hostname for a zone.
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/delete/>
#[derive(Debug)]
pub struct DeleteCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub custom_hostname_id: &'a str,
}
impl EndpointSpec for DeleteCustomHostname<'_> {
    type JsonResponse = DeleteCustomHostnameResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_identifier, self.custom_hostname_id
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteCustomHostnameResponse {
    pub id: String,
}

impl ApiResult for DeleteCustomHostnameResponse {}

/// Edit a custom hostname for a zone.
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/edit/>
#[derive(Debug)]
pub struct EditCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub custom_hostname_id: &'a str,
    pub params: EditCustomHostnameParams,
}
impl EndpointSpec for EditCustomHostname<'_> {
    type JsonResponse = CustomHostnameDetails;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_identifier, self.custom_hostname_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Clone, Default)]
pub struct EditCustomHostnameParams {
    pub custom_metadata: Option<HashMap<String, String>>,
    pub custom_original_server: Option<String>,
    pub custom_original_sni: Option<String>,
    pub ssl: Option<HostnameSslProperties>,
}

/// Get custom hostname details.
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/get/>
#[derive(Debug)]
pub struct GetCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub custom_hostname_id: &'a str,
}
impl EndpointSpec for GetCustomHostname<'_> {
    type JsonResponse = CustomHostnameDetails;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_identifier, self.custom_hostname_id
        )
    }
}

/// Get custom hostname details.
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/get/>
#[derive(Debug, Default)]
pub struct ListCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub params: ListCustomHostnameParams,
}
impl EndpointSpec for ListCustomHostname<'_> {
    type JsonResponse = Vec<CustomHostnameDetails>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("zones/{}/custom_hostnames", self.zone_identifier)
    }

    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListCustomHostnameParams {
    pub id: Option<String>,
    pub direction: Option<OrderDirection>,
    pub hostname: Option<String>,
    pub order: Option<ListCustomHostnameOrder>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub ssl: Option<u8>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ListCustomHostnameOrder {
    Ssl,
    SslStatus,
}
