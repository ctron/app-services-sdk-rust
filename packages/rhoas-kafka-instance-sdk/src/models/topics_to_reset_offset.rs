/*
 * Kafka Admin REST API
 *
 * An API to provide REST endpoints for query Kafka for admin operations
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TopicsToResetOffset {
    #[serde(rename = "topic")]
    pub topic: String,
    #[serde(rename = "partitions", skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<i32>>,
}

impl TopicsToResetOffset {
    pub fn new(topic: String) -> TopicsToResetOffset {
        TopicsToResetOffset {
            topic,
            partitions: None,
        }
    }
}

