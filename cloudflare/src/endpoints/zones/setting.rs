use crate::framework::endpoint::{EndpointSpec, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use crate::framework::{BooleanValue, TlsVersion};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, strum_macros::AsRefStr)]
#[serde(tag = "id", rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ZoneSettingValue {
    AlwaysUseHttps { value: BooleanValue },
    MinTlsVersion { value: TlsVersion },
}

/// Edit Zone Setting
/// <https://developers.cloudflare.com/api/resources/zones/subresources/settings/methods/edit/>
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct EditZoneSetting<'a> {
    #[serde(skip)]
    pub zone_identifier: &'a str,
    pub enabled: Option<bool>,
    #[serde(flatten)]
    pub value: ZoneSettingValue,
}

impl EndpointSpec for EditZoneSetting<'_> {
    type JsonResponse = ZoneSettingResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/settings/{}",
            self.zone_identifier,
            self.value.as_ref()
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Get Zone Setting
/// <https://developers.cloudflare.com/api/resources/zones/subresources/settings/methods/get/>
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct GetZoneSetting<'a> {
    #[serde(skip)]
    pub zone_identifier: &'a str,
    pub setting_id: &'a str,
}

impl EndpointSpec for GetZoneSetting<'_> {
    type JsonResponse = ZoneSettingResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/settings/{}",
            self.zone_identifier, self.setting_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Deserialize, Debug)]
pub struct ZoneSettingResponse {
    pub editable: bool,
    #[serde(flatten)]
    pub value: ZoneSettingValue,
    pub modified_on: Option<chrono::DateTime<chrono::Utc>>,
    pub enabled: Option<bool>,
    pub time_remaining: Option<f64>,
}

impl ApiResult for ZoneSettingResponse {}
