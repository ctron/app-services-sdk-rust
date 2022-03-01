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
pub struct ConsumerGroupResetOffsetParameters {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "offset")]
    pub offset: Offset,
    #[serde(rename = "topics", skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<crate::models::TopicsToResetOffset>>,
}

impl ConsumerGroupResetOffsetParameters {
    pub fn new(offset: Offset) -> ConsumerGroupResetOffsetParameters {
        ConsumerGroupResetOffsetParameters {
            value: None,
            offset,
            topics: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Offset {
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "absolute")]
    Absolute,
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "earliest")]
    Earliest,
}

impl Default for Offset {
    fn default() -> Offset {
        Self::Timestamp
    }
}

