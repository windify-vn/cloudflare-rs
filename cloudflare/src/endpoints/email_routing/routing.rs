use crate::framework::endpoint::EndpointSpec;
use crate::framework::response::{ApiResult, ApiSuccess};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct GetRoutingSetting<'a> {
    pub zone_identifier: &'a str,
}

impl EndpointSpec for GetRoutingSetting<'_> {
    type JsonResponse = RoutingSettings;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("zones/{}/email/routing", self.zone_identifier)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoutingSettings {
    pub id: String,
    pub enabled: bool,
    pub name: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    pub status: Option<EmailRoutingStatus>,
    pub skip_wizard: Option<bool>,
}

impl ApiResult for RoutingSettings {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EmailRoutingStatus {
    Ready,
    Unconfigured,
    Misconfigured,
    #[serde(alias = "misconfigured/locked")]
    Locked,
    Unlocked,
}
