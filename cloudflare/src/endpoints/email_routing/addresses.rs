use crate::framework::endpoint::{serialize_query, EndpointSpec, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use crate::framework::OrderDirection;
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Addresses {
    pub id: Option<String>,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    pub email: Option<String>,
    pub tag: Option<String>,
    pub verified: Option<chrono::DateTime<chrono::Utc>>,
}

impl ApiResult for Addresses {}
impl ApiResult for Vec<Addresses> {}
impl ApiResult for Option<Addresses> {}

#[derive(Debug)]
pub struct CreateDestinationAddress<'a> {
    pub account_identifier: &'a str,
    pub params: CreateDestinationAddressParams,
}

impl EndpointSpec for CreateDestinationAddress<'_> {
    type JsonResponse = Option<Addresses>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/email/routing/addresses",
            self.account_identifier
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateDestinationAddressParams {
    pub email: String,
}

#[derive(Debug)]
pub struct DeleteDestinationAddress<'a> {
    pub account_identifier: &'a str,
    pub destination_address_identifier: &'a str,
}

impl EndpointSpec for DeleteDestinationAddress<'_> {
    type JsonResponse = Option<Addresses>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/email/routing/addresses/{}",
            self.account_identifier, self.destination_address_identifier
        )
    }
}

#[derive(Debug)]
pub struct GetDestinationAddress<'a> {
    pub account_identifier: &'a str,
    pub destination_address_identifier: &'a str,
}

impl EndpointSpec for GetDestinationAddress<'_> {
    type JsonResponse = Option<Addresses>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/email/routing/addresses/{}",
            self.account_identifier, self.destination_address_identifier
        )
    }
}

#[derive(Debug, Default)]
pub struct ListDestinationAddress<'a> {
    pub account_identifier: &'a str,
    pub params: ListDestinationAddressParams,
}

impl EndpointSpec for ListDestinationAddress<'_> {
    type JsonResponse = Vec<Addresses>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/email/routing/addresses",
            self.account_identifier
        )
    }

    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListDestinationAddressParams {
    pub direction: Option<OrderDirection>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub verified: Option<bool>,
}
