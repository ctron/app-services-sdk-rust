/*
 * Kafka Service Fleet Manager
 *
 * Kafka Service Fleet Manager is a Rest API to manage Kafka instances.
 *
 * The version of the OpenAPI document: 1.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetricsRangeQueryList {
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::RangeQuery>>,
}

impl MetricsRangeQueryList {
    pub fn new() -> MetricsRangeQueryList {
        MetricsRangeQueryList {
            kind: None,
            id: None,
            items: None,
        }
    }
}


