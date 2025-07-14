use crate::framework::endpoint::{serialize_query, EndpointSpec, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Drop,
    #[default]
    Forward,
    Worker,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: ActionType,
    pub value: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MatcherType {
    #[default]
    All,
    Literal,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MatcherField {
    To,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Matcher {
    #[serde(rename = "type")]
    pub matcher_type: MatcherType,
    pub field: Option<MatcherField>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoutingRule {
    pub id: Option<String>,
    #[serde(default)]
    pub actions: Vec<Action>,
    pub enabled: Option<bool>,
    #[serde(default)]
    pub matchers: Vec<Matcher>,
    pub name: Option<String>,
    pub priority: Option<u32>,
    pub tag: Option<String>,
}

impl ApiResult for RoutingRule {}
impl ApiResult for Vec<RoutingRule> {}

#[derive(Debug, Default)]
pub struct CreateRoutingRule<'a> {
    pub zone_identifier: &'a str,
    pub params: CreateRoutingRuleParams,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Default)]
pub struct CreateRoutingRuleParams {
    pub actions: Vec<Action>,
    pub matchers: Vec<Matcher>,
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub priority: Option<u32>,
}

impl EndpointSpec for CreateRoutingRule<'_> {
    type JsonResponse = RoutingRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/email/routing/rules", self.zone_identifier)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Debug, Default)]
pub struct DeleteRoutingRule<'a> {
    pub zone_identifier: &'a str,
    pub rule_identifier: &'a str,
}

impl EndpointSpec for DeleteRoutingRule<'_> {
    type JsonResponse = RoutingRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/email/routing/rules/{}",
            self.zone_identifier, self.rule_identifier
        )
    }
}

#[derive(Debug, Default)]
pub struct GetRoutingRule<'a> {
    pub zone_identifier: &'a str,
    pub rule_identifier: &'a str,
}

impl EndpointSpec for GetRoutingRule<'_> {
    type JsonResponse = RoutingRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/email/routing/rules/{}",
            self.zone_identifier, self.rule_identifier
        )
    }
}

#[derive(Debug, Default)]
pub struct ListRoutingRule<'a> {
    pub zone_identifier: &'a str,
    pub params: ListRoutingRuleParams,
}

#[derive(Debug, Default, Serialize)]
pub struct ListRoutingRuleParams {
    pub enabled: Option<bool>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl EndpointSpec for ListRoutingRule<'_> {
    type JsonResponse = Vec<RoutingRule>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("zones/{}/email/routing/rules", self.zone_identifier)
    }

    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

#[derive(Debug, Default)]
pub struct UpdateRoutingRule<'a> {
    pub zone_identifier: &'a str,
    pub rule_identifier: &'a str,
    pub params: UpdateRoutingRuleParams,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Default)]
pub struct UpdateRoutingRuleParams {
    pub actions: Vec<Action>,
    pub matchers: Vec<Matcher>,
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub priority: Option<u32>,
}

impl EndpointSpec for UpdateRoutingRule<'_> {
    type JsonResponse = RoutingRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/email/routing/rules/{}",
            self.zone_identifier, self.rule_identifier
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}


#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CatchAllActionType {
    #[default]
    All,
    Literal,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CatchAllMatcherType {
    #[default]
    To,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct CatchAllAction {
    #[serde(rename = "type")]
    pub action_type: CatchAllActionType,
    pub value: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct CatchAllMatcher {
    #[serde(rename = "type")]
    pub matcher_type: CatchAllMatcherType,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CatchAllRoutingRule {
    pub id: Option<String>,
    #[serde(default)]
    pub actions: Vec<CatchAllAction>,
    pub enabled: Option<bool>,
    #[serde(default)]
    pub matchers: Vec<CatchAllMatcher>,
    pub name: Option<String>,
    pub tag: Option<String>,
}

impl ApiResult for CatchAllRoutingRule {}

#[derive(Debug, Default)]
pub struct GetCatchAllRule<'a> {
    pub zone_identifier: &'a str,
}
impl EndpointSpec for GetCatchAllRule<'_> {
    type JsonResponse = CatchAllRoutingRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/email/routing/rules/catch_all",
            self.zone_identifier
        )
    }
}

#[derive(Debug, Default)]
pub struct UpdateCatchAllRule<'a> {
    pub zone_identifier: &'a str,
    pub params: UpdateCatchAllRuleParams
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct UpdateCatchAllRuleParams {
    pub actions: Vec<CatchAllAction>,
    pub matchers: Vec<CatchAllMatcher>,
    pub enabled: Option<bool>,
    pub name: Option<String>,
}

impl EndpointSpec for UpdateCatchAllRule<'_> {
    type JsonResponse = CatchAllRoutingRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/email/routing/rules/catch_all",
            self.zone_identifier
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}