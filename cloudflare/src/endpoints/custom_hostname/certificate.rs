use http::Method;
use serde::{Deserialize, Serialize};
use crate::endpoints::custom_hostname::hostname::CustomHostnameDetails;
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};

#[derive(Debug)]
pub struct DeleteCustomHostnameCertificate<'a> {
    pub zone_identifier: &'a str,
    pub custom_hostname_id: &'a str,
    pub certificate_pack_id: &'a str,
    pub certificate_id: &'a str,
}
impl EndpointSpec for DeleteCustomHostnameCertificate<'_> {
    type JsonResponse = DeleteCustomHostnameCertificateResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "/zones/{}/custom_hostnames/{}/certificate_pack/{}/certificates/{}",
            self.zone_identifier, self.custom_hostname_id, self.certificate_pack_id, self.certificate_id
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteCustomHostnameCertificateResponse {
    pub id: String,
}

impl ApiResult for DeleteCustomHostnameCertificateResponse {}

#[derive(Debug)]
pub struct ReplaceCustomHostnameCertificate<'a> {
    pub zone_identifier: &'a str,
    pub custom_hostname_id: &'a str,
    pub certificate_pack_id: &'a str,
    pub certificate_id: &'a str,
    pub params: ReplaceCustomHostnameCertificateParams,
}
impl EndpointSpec for ReplaceCustomHostnameCertificate<'_> {
    type JsonResponse = CustomHostnameDetails;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "/zones/{}/custom_hostnames/{}/certificate_pack/{}/certificates/{}",
            self.zone_identifier, self.custom_hostname_id, self.certificate_pack_id, self.certificate_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ReplaceCustomHostnameCertificateParams {
    pub custom_certificate: String,
    pub custom_key: String
}