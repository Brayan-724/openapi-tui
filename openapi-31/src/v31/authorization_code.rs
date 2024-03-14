// OpenAPI v3.1.0 Specification
//
// OpenAPI inside OpenAPI
//
// The version of the OpenAPI document: 3.1.0
//
// Generated by: https://openapi-generator.tech

use crate::v31;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationCode {
  #[serde(rename = "authorizationUrl")]
  pub authorization_url: String,
  #[serde(rename = "tokenUrl")]
  pub token_url: String,
  #[serde(rename = "refreshUrl", skip_serializing_if = "Option::is_none")]
  pub refresh_url: Option<String>,
  #[serde(rename = "scopes")]
  pub scopes: std::collections::BTreeMap<String, String>,
}

impl AuthorizationCode {
  pub fn new(
    authorization_url: String,
    token_url: String,
    scopes: std::collections::BTreeMap<String, String>,
  ) -> AuthorizationCode {
    AuthorizationCode { authorization_url, token_url, refresh_url: None, scopes }
  }
}