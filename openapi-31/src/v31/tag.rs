// OpenAPI v3.1.0 Specification
//
// OpenAPI inside OpenAPI
//
// The version of the OpenAPI document: 3.1.0
//
// Generated by: https://openapi-generator.tech

use crate::v31;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
  pub external_docs: Option<Box<v31::ExternalDocumentation>>,
}

impl Tag {
  pub fn new(name: String) -> Tag {
    Tag { name, description: None, external_docs: None }
  }
}