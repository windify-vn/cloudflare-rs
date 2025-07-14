use crate::endpoints::email_routing::routing::RoutingSettings;
use crate::framework::endpoint::{serialize_query, EndpointSpec, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct EnableEmailRouting<'a> {
    pub zone_identifier: &'a str,
    pub params: EnableEmailRoutingParams,
}

impl EndpointSpec for EnableEmailRouting<'_> {
    type JsonResponse = RoutingSettings;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/email/routing/dns", self.zone_identifier)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct EnableEmailRoutingParams {
    pub domain: String,
}

#[derive(Debug)]
pub struct DisableEmailRouting<'a> {
    pub zone_identifier: &'a str,
    pub destination_address_identifier: &'a str,
}

impl EndpointSpec for DisableEmailRouting<'_> {
    type JsonResponse = Vec<RoutingDnsRecord>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!("zones/{}/email/routing/dns", self.zone_identifier)
    }
}

#[derive(Debug)]
pub struct UnlockEmailRouting<'a> {
    pub zone_identifier: &'a str,
    pub params: UnlockEmailRoutingParams,
}

impl EndpointSpec for UnlockEmailRouting<'_> {
    type JsonResponse = RoutingSettings;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }
    fn path(&self) -> String {
        format!("zones/{}/email/routing/dns", self.zone_identifier)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Debug, Default, Serialize)]
pub struct UnlockEmailRoutingParams {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct EmailRoutingSetting<'a> {
    pub zone_identifier: &'a str,
    pub params: EmailRoutingSettingParams,
}

impl EndpointSpec for EmailRoutingSetting<'_> {
    type JsonResponse = RoutingSettingUnion;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("zones/{}/email/routing/dns", self.zone_identifier)
    }

    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RoutingSettingUnion {
    Records(Vec<RoutingDnsRecord>),
    ErrorRecords {
        record: Vec<RoutingDnsRecord>,
        errors: Vec<RoutingRecordError>,
    },
}

impl ApiResult for RoutingSettingUnion {}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct RoutingRecordError {
    pub code: Option<String>,
    pub missing: Option<RoutingDnsRecord>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct EmailRoutingSettingParams {
    pub subdomain: String,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct RoutingDnsRecord {
    pub content: Option<String>,
    pub name: Option<String>,
    pub priority: Option<u32>,
    pub ttl: Option<u32>,
    #[serde(rename = "type")]
    pub record_type: Option<String>,
}

impl ApiResult for RoutingDnsRecord {}
impl ApiResult for Vec<RoutingDnsRecord> {}
