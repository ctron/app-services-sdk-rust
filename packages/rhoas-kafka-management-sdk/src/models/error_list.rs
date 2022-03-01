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
pub struct ErrorList {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "size")]
    pub size: i32,
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "items")]
    pub items: Vec<crate::models::Error>,
}

impl ErrorList {
    pub fn new(kind: String, page: i32, size: i32, total: i32, items: Vec<crate::models::Error>) -> ErrorList {
        ErrorList {
            kind,
            page,
            size,
            total,
            items,
        }
    }
}


