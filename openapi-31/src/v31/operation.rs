// OpenAPI v3.1.0 Specification
//
// OpenAPI inside OpenAPI
//
// The version of the OpenAPI document: 3.1.0
//
// Generated by: https://openapi-generator.tech

use crate::v31;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
  #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
  pub tags: Option<Vec<String>>,
  #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
  pub summary: Option<String>,
  #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
  pub external_docs: Option<Box<v31::ExternalDocumentation>>,
  #[serde(rename = "operationId", skip_serializing_if = "Option::is_none")]
  pub operation_id: Option<String>,
  #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
  pub parameters: Option<Vec<serde_json::Value>>,
  #[serde(rename = "requestBody", default, skip_serializing_if = "Option::is_none")]
  pub request_body: Option<serde_json::Value>,
  #[serde(rename = "responses", skip_serializing_if = "Option::is_none")]
  pub responses: Option<Box<v31::Responses>>,
  #[serde(rename = "callbacks", skip_serializing_if = "Option::is_none")]
  pub callbacks: Option<std::collections::HashMap<String, serde_json::Value>>,
  #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
  pub deprecated: Option<bool>,
  #[serde(rename = "security", skip_serializing_if = "Option::is_none")]
  pub security: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
  #[serde(rename = "servers", skip_serializing_if = "Option::is_none")]
  pub servers: Option<Vec<v31::Server>>,
}

impl Operation {
  pub fn new() -> Operation {
    Operation {
      tags: None,
      summary: None,
      description: None,
      external_docs: None,
      operation_id: None,
      parameters: None,
      request_body: None,
      responses: None,
      callbacks: None,
      deprecated: None,
      security: None,
      servers: None,
    }
  }
}
