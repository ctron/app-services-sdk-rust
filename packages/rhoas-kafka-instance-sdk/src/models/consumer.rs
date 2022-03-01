/*
 * Kafka Admin REST API
 *
 * An API to provide REST endpoints for query Kafka for admin operations
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Consumer : A Kafka consumer is responsible for reading records from one or more topics and one or more partitions of a topic.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Consumer {
    /// Unique identifier for the consumer group to which this consumer belongs.
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// The unique topic name to which this consumer belongs
    #[serde(rename = "topic")]
    pub topic: String,
    /// The partition number to which this consumer group is assigned to.
    #[serde(rename = "partition")]
    pub partition: i32,
    /// Offset denotes the position of the consumer in a partition.
    #[serde(rename = "offset")]
    pub offset: f32,
    /// The log end offset is the offset of the last message written to a log.
    #[serde(rename = "logEndOffset", skip_serializing_if = "Option::is_none")]
    pub log_end_offset: Option<f32>,
    /// Offset Lag is the delta between the last produced message and the last consumer's committed offset.
    #[serde(rename = "lag")]
    pub lag: i32,
    /// The member ID is a unique identifier given to a consumer by the coordinator upon initially joining the group.
    #[serde(rename = "memberId", skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
}

impl Consumer {
    /// A Kafka consumer is responsible for reading records from one or more topics and one or more partitions of a topic.
    pub fn new(group_id: String, topic: String, partition: i32, offset: f32, lag: i32) -> Consumer {
        Consumer {
            group_id,
            topic,
            partition,
            offset,
            log_end_offset: None,
            lag,
            member_id: None,
        }
    }
}


