/*
 * Kafka Admin REST API
 *
 * An API to provide REST endpoints for query Kafka for admin operations
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AclResourceType {
    #[serde(rename = "GROUP")]
    GROUP,
    #[serde(rename = "TOPIC")]
    TOPIC,
    #[serde(rename = "CLUSTER")]
    CLUSTER,
    #[serde(rename = "TRANSACTIONAL_ID")]
    TRANSACTIONALID,

}

impl ToString for AclResourceType {
    fn to_string(&self) -> String {
        match self {
            Self::GROUP => String::from("GROUP"),
            Self::TOPIC => String::from("TOPIC"),
            Self::CLUSTER => String::from("CLUSTER"),
            Self::TRANSACTIONALID => String::from("TRANSACTIONAL_ID"),
        }
    }
}

impl Default for AclResourceType {
    fn default() -> AclResourceType {
        Self::GROUP
    }
}




