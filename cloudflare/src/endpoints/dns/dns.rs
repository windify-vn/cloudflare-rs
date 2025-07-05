use crate::framework::endpoint::{serialize_query, EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
/// <https://api.cloudflare.com/#dns-records-for-a-zone-properties>
use crate::framework::{OrderDirection, SearchMatch};
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};

/// List DNS Records
/// <https://api.cloudflare.com/#dns-records-for-a-zone-list-dns-records>
#[derive(Debug)]
pub struct ListDnsRecords<'a> {
    pub zone_identifier: &'a str,
    pub params: ListDnsRecordsParams,
}
impl EndpointSpec for ListDnsRecords<'_> {
    type JsonResponse = Vec<DnsRecord>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("zones/{}/dns_records", self.zone_identifier)
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

/// Create DNS Record
/// <https://api.cloudflare.com/#dns-records-for-a-zone-create-dns-record>
#[derive(Debug)]
pub struct CreateDnsRecord<'a> {
    pub zone_identifier: &'a str,
    pub params: DnsRecordOperator,
}

impl EndpointSpec for CreateDnsRecord<'_> {
    type JsonResponse = DnsRecord;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/dns_records", self.zone_identifier)
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Delete DNS Record
/// <https://api.cloudflare.com/#dns-records-for-a-zone-delete-dns-record>
#[derive(Debug)]
pub struct DeleteDnsRecord<'a> {
    pub zone_identifier: &'a str,
    pub identifier: &'a str,
}
impl EndpointSpec for DeleteDnsRecord<'_> {
    type JsonResponse = DeleteDnsRecordResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/dns_records/{}",
            self.zone_identifier, self.identifier
        )
    }
}

/// Update DNS Record
/// <https://api.cloudflare.com/#dns-records-for-a-zone-update-dns-record>
#[derive(Debug)]
pub struct UpdateDnsRecord<'a> {
    pub zone_identifier: &'a str,
    pub identifier: &'a str,
    pub params: DnsRecordOperator,
}

impl EndpointSpec for UpdateDnsRecord<'_> {
    type JsonResponse = DnsRecord;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/dns_records/{}",
            self.zone_identifier, self.identifier
        )
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ListDnsRecordsOrder {
    Type,
    Name,
    Content,
    Ttl,
    Proxied,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListDnsRecordsParams {
    #[serde(flatten)]
    pub record_type: Option<DnsContent>,
    pub name: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order: Option<ListDnsRecordsOrder>,
    pub direction: Option<OrderDirection>,
    #[serde(rename = "match")]
    pub search_match: Option<SearchMatch>,
}

/// Extra Cloudflare-specific information about the record
#[derive(Deserialize, Debug)]
pub struct Meta {}

/// Type of the DNS record, along with the associated value.
/// When we add support for other types (LOC/SRV/...), the `meta` field should also probably be encoded
/// here as an associated, strongly typed value.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[allow(clippy::upper_case_acronyms)]
pub enum DnsContent {
    A { content: Ipv4Addr },
    AAAA { content: Ipv6Addr },
    CNAME { content: String },
    NS { content: String },
    MX { content: String, priority: u16 },
    TXT { content: String },
    SRV { content: String },
}

#[derive(Deserialize, Debug)]
pub struct DeleteDnsRecordResponse {
    /// DNS record identifier tag
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct DnsRecord {
    /// Extra Cloudflare-specific information about the record
    pub meta: Meta,
    /// DNS record name
    pub name: String,
    /// Time to live for DNS record. Value of 1 is 'automatic'
    pub ttl: u32,
    /// When the record was last modified
    pub modified_on: DateTime<Utc>,
    /// When the record was created
    pub created_on: DateTime<Utc>,
    /// Whether this record can be modified/deleted (true means it's managed by Cloudflare)
    pub proxiable: bool,
    /// Type of the DNS record that also holds the record value
    #[serde(flatten)]
    pub content: DnsContent,
    /// DNS record identifier tag
    pub id: String,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    pub proxied: bool,
}

impl ApiResult for DnsRecord {}
impl ApiResult for Vec<DnsRecord> {}
impl ApiResult for DeleteDnsRecordResponse {}

/// Batch DNS Records
/// <https://developers.cloudflare.com/api/resources/dns/subresources/records/methods/batch/>
#[derive(Debug)]
pub struct BatchDnsRecords<'a> {
    pub zone_identifier: &'a str,
    pub params: BatchDnsRecordsParams,
}

impl EndpointSpec for BatchDnsRecords<'_> {
    type JsonResponse = BatchDnsRecordsResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/dns_records/batch", self.zone_identifier)
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct BatchDnsRecordsParams {
    pub deletes: Option<Vec<BatchDelete>>,
    pub patches: Option<Vec<DnsRecordOperator>>,
    pub posts: Option<Vec<DnsRecordOperator>>,
    pub puts: Option<Vec<DnsRecordOperator>>,
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct BatchDelete {
    pub id: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct DnsRecordOperator {
    /// Time to live for DNS record. Value of 1 is 'automatic'
    pub ttl: Option<u32>,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    pub proxied: Option<bool>,
    /// DNS record name
    pub name: String,
    /// Type of the DNS record that also holds the record value
    #[serde(flatten)]
    pub content: DnsContent,
}

#[derive(Deserialize, Debug)]
pub struct BatchDnsRecordsResponse {
    pub deletes: Option<Vec<DnsRecord>>,
    pub patches: Option<Vec<DnsRecord>>,
    pub posts: Option<Vec<DnsRecord>>,
    pub puts: Option<Vec<DnsRecord>>,
}

impl ApiResult for BatchDnsRecordsResponse {}

/// Batch DNS Records
/// <https://developers.cloudflare.com/api/resources/dns/subresources/records/methods/get/>
#[derive(Debug)]
pub struct GetDnsRecord<'a> {
    pub zone_identifier: &'a str,
    pub record_identifier: &'a str,
}

impl EndpointSpec for GetDnsRecord<'_> {
    type JsonResponse = DnsRecord;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/dns_records/{}",
            self.zone_identifier, self.record_identifier
        )
    }
}

/// Scan DNS Records
/// <https://developers.cloudflare.com/api/resources/dns/subresources/records/methods/scan/>
#[derive(Debug)]
pub struct ScanDnsRecords<'a> {
    pub zone_identifier: &'a str,
}

impl EndpointSpec for ScanDnsRecords<'_> {
    type JsonResponse = Option<ScanDnsRecordsResponse>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/dns_records/scan", self.zone_identifier)
    }
}

#[derive(Deserialize, Debug)]
pub struct ScanDnsRecordsResponse {
    pub recs_added: Option<u32>,
    pub total_records_parsed: Option<u32>,
}

impl ApiResult for ScanDnsRecordsResponse {}
impl ApiResult for Option<ScanDnsRecordsResponse> {}

/// Overwrite DNS Records
/// <https://developers.cloudflare.com/api/resources/dns/subresources/records/methods/update/>
#[derive(Debug)]
pub struct OverwriteDnsRecord<'a> {
    pub zone_identifier: &'a str,
    pub record_identifier: &'a str,
    pub params: DnsRecordOperator,
}

impl EndpointSpec for OverwriteDnsRecord<'_> {
    type JsonResponse = DnsRecord;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/dns_records/{}",
            self.zone_identifier, self.record_identifier
        )
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}
