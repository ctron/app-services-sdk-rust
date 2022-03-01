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
pub struct ServiceAccountListAllOf {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "items")]
    pub items: Vec<crate::models::ServiceAccountListItem>,
}

impl ServiceAccountListAllOf {
    pub fn new(kind: String, items: Vec<crate::models::ServiceAccountListItem>) -> ServiceAccountListAllOf {
        ServiceAccountListAllOf {
            kind,
            items,
        }
    }
}

