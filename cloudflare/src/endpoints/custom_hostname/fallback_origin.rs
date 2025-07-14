use crate::framework::endpoint::{EndpointSpec, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DeleteFallbackOrigin<'a> {
    pub zone_identifier: &'a str,
}

impl EndpointSpec for DeleteFallbackOrigin<'_> {
    type JsonResponse = FallbackOriginStatus;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/fallback_origin",
            self.zone_identifier
        )
    }
}

#[derive(Debug)]
pub struct GetFallbackOrigin<'a> {
    pub zone_identifier: &'a str,
}

impl EndpointSpec for GetFallbackOrigin<'_> {
    type JsonResponse = FallbackOriginStatus;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/fallback_origin",
            self.zone_identifier
        )
    }
}

#[derive(Debug)]
pub struct UpdateFallbackOrigin<'a> {
    pub zone_identifier: &'a str,
    pub params: UpdateFallbackOriginParams,
}

impl EndpointSpec for UpdateFallbackOrigin<'_> {
    type JsonResponse = FallbackOriginStatus;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/fallback_origin",
            self.zone_identifier
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct UpdateFallbackOriginParams {
    pub origin: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FallbackOriginStatus {
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub errors: Vec<String>,
    pub origin: Option<String>,
    pub status: Option<FallbackOriginActivationStatus>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ApiResult for FallbackOriginStatus {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FallbackOriginActivationStatus {
    Initializing,
    PendingDeployment,
    PendingDeletion,
    Active,
    DeploymentTimedOut,
    DeletionTimedOut,
}
